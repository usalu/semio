#!/bin/sh
# usage: image-space-return-probe3.sh <scratch-dir> : does touching a SECOND volume of the same APFS container trim while the main volume stays mounted and busy?
D=$1; I=$D/p3.dmg; M=$D/p3m; rm -rf "$I" "$M"; mkdir -p "$M"
diskutil image create blank --format RAW --size 1800m --fs APFS --volumeName t "$I" >/dev/null || exit 1
sz(){ stat -f '%b' "$I" | awk '{print $1*512/1048576}'; }
diskutil image attach --nobrowse --mountPoint "$M" "$I" >/dev/null
VOL=$(mount | awk -v m="$M" '$3==m{print $1}'); CON=$(diskutil info "$VOL" | awk -F': *' '/APFS Container:/{print $2}'); echo "vol=$VOL container=$CON"
head -c 1073741824 /dev/urandom > "$M/blob"; sync; rm "$M/blob"; sync; sleep 2; echo "deleted, main mounted: $(sz) MiB"
cd "$M" || exit 1   # keep main volume busy: cwd inside
diskutil apfs addVolume "$CON" "Case-sensitive APFS" aux -nomount >/dev/null 2>&1; diskutil apfs list "$CON" | grep -c "Volume disk"
AUX=$(diskutil apfs list "$CON" | awk '/APFS Volume Disk/{print $4}' | grep -v "$(basename $VOL)" | head -1); echo "aux=$AUX"
diskutil mount "$AUX" >/dev/null 2>&1; sleep 2; echo "after aux mount: $(sz) MiB"
diskutil unmount "$AUX" >/dev/null 2>&1; sleep 3; echo "after aux unmount: $(sz) MiB"
diskutil apfs deleteVolume "$AUX" >/dev/null 2>&1; sleep 3; echo "after aux delete: $(sz) MiB"
echo "main still busy-mounted: $(mount | grep -c " $M ")"
cd /; diskutil unmount "$VOL" >/dev/null; sleep 2; echo "after main unmount: $(sz) MiB"
hdiutil detach "$M" -quiet 2>/dev/null; hdiutil detach "$(hdiutil info | awk -v i="$I" '$0~i{f=1} f&&/\/dev\/disk/{print $1; exit}')" -quiet 2>/dev/null
rm -rf "$I" "$M"
