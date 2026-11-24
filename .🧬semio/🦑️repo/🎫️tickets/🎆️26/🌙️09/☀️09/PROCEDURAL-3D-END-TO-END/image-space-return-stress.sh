#!/bin/sh
# usage: image-space-return-stress.sh <scratch-dir> : sibling-volume kick under concurrent writes, open handles and a final fsck_apfs
D=$1; I=$D/p5.dmg; M=$D/p5m; rm -rf "$I" "$M"; mkdir -p "$M"
diskutil image create blank --format RAW --size 1800m --fs APFS --volumeName t "$I" >/dev/null || exit 1
sz(){ stat -f '%b' "$I" | awk '{print $1*512/1048576}'; }
diskutil image attach --nobrowse --mountPoint "$M" "$I" >/dev/null
VOL=$(mount | awk -v m="$M" '$3==m{print $1}'); CON=$(diskutil info "$VOL" | awk -F': *' '/APFS Container:/{print $2}')
diskutil apfs addVolume "$CON" "Case-sensitive APFS" aux -nomount >/dev/null; AUX=$(diskutil list "$CON" | awk '/ aux /{print $NF}' | head -1)
mkdir "$M/live"; : > "$D/p5.sums"
( i=0; while [ -e "$M/live" ]; do i=$((i+1)); head -c 4194304 /dev/urandom > "$M/live/f$i"; shasum "$M/live/f$i" | cut -c1-40 >> "$D/p5.sums"; [ $i -gt 40 ] && rm "$M/live/f$((i-40))" && sed -i '' "1d" "$D/p5.sums"; done ) & L=$!
sh -c 'exec 3>>"$0"; exec sleep 600' "$M/open-writer" & W=$!
for n in 1 2 3 4; do
 head -c 536870912 /dev/urandom > "$M/blob$n"; sync; rm "$M/blob$n"; sync; sleep 1
 B=$(sz); diskutil mount "$AUX" >/dev/null 2>&1; sleep 1; diskutil unmount "$AUX" >/dev/null 2>&1; sleep 2; echo "round $n: $B -> $(sz) MiB"
done
rm -rf "$M/live-stop" ; mv "$M/live" "$M/live-x"; wait $L 2>/dev/null; kill $W
BAD=0; for f in "$M"/live-x/f*; do :; done
ls "$M/live-x" | wc -l | xargs echo "files kept by the writer loop:"
diskutil unmount "$VOL" >/dev/null; sleep 1
DEVN=$(echo $VOL | sed 's#/dev/##'); fsck_apfs -n "/dev/$DEVN" 2>&1 | tail -3
hdiutil detach "$(hdiutil info | awk -v i="$I" '$0~i{f=1} f&&/\/dev\/disk/{print $1; exit}')" -quiet 2>/dev/null
rm -rf "$I" "$M" "$D/p5.sums"
