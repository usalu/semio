#!/bin/sh
# usage: image-space-return-probe.sh <RAW|ASIF|UDSB> <scratch-dir> : which in-place action returns blocks while the image stays attached
F=$1; D=$2; I=$D/probe.$F; M=$D/pm; rm -rf "$I" "$M"; mkdir -p "$M"
diskutil image create blank --format $F --size 1800m --fs APFS --volumeName t "$I" >/dev/null || exit 1
sz(){ if [ -d "$I" ]; then du -sk "$I" | awk '{print $1/1024}'; else stat -f '%b' "$I" | awk '{print $1*512/1048576}'; fi; }
P=$(diskutil image attach --nobrowse --mountPoint "$M" "$I" 2>&1); DEV=$(echo "$P" | awk '/disk[0-9]+s?[0-9]*/{print $1}' | head -3 | tr '\n' ' '); echo "attached: $DEV"
head -c 1073741824 /dev/urandom > "$M/blob"; sync; sleep 2; echo "written: $(sz) MiB"
rm "$M/blob"; sync; sleep 3; echo "deleted: $(sz) MiB"
VOL=$(mount | awk -v m="$M" '$3==m{print $1}'); echo "volume $VOL"
for step in "diskutil apfs list -plist" "diskutil unmount $VOL" ; do :; done
diskutil unmount "$VOL" >/dev/null; sleep 3; echo "after volume unmount (image stays attached): $(sz) MiB"
diskutil mount -mountPoint "$M" "$VOL" >/dev/null; sleep 1; echo "after remount: $(sz) MiB  $(mount | grep " $M " | cut -d'(' -f2 | cut -c1-40)"
head -c 536870912 /dev/urandom > "$M/b2"; sync; echo "b2 written: $(sz) MiB"; rm "$M/b2"; sync; sleep 2
diskutil apfs list >/dev/null; mount -u -o ro "$M" 2>/dev/null; sleep 2; echo "after remount ro (mount -u): $(sz) MiB"; mount -u -o rw "$M" 2>/dev/null
hdiutil detach "$M" -quiet; sleep 2; echo "after detach: $(sz) MiB"
rm -rf "$I" "$M"
