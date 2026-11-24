#!/bin/sh
# usage: image-space-return-probe2.sh <scratch-dir> : tries non-unmounting triggers on an attached RAW image
D=$1; I=$D/p2.dmg; M=$D/p2m; rm -rf "$I" "$M"; mkdir -p "$M"
diskutil image create blank --format RAW --size 1800m --fs APFS --volumeName t "$I" >/dev/null || exit 1
sz(){ stat -f '%b' "$I" | awk '{print $1*512/1048576}'; }
diskutil image attach --nobrowse --mountPoint "$M" "$I" >/dev/null
VOL=$(mount | awk -v m="$M" '$3==m{print $1}')
churn(){ head -c 1073741824 /dev/urandom > "$M/blob"; sync; rm "$M/blob"; sync; }
try(){ churn; echo "[$1] before: $(sz) MiB"; sh -c "$2" >/dev/null 2>&1; echo "[$1] rc=$? after: $(sz) MiB"; }
try "mount -u -o ro" "mount -u -o ro $M; sleep 3; mount -u -o rw $M"
try "mount -u -o update,rdonly via diskutil" "diskutil mount readOnly $VOL"
mount -u -o rw "$M" 2>/dev/null
try "purge" "purge"
try "apfs list/defrag" "diskutil apfs list; diskutil apfs defragment $VOL status"
try "wait 90s" "sleep 90"
try "fsck_apfs -n (live)" "fsck_apfs -n $VOL"
try "unmount+mount volume" "diskutil unmount $VOL; diskutil mount -mountPoint $M $VOL"
hdiutil detach "$M" -quiet; sleep 2; echo "detach: $(sz) MiB"; rm -rf "$I" "$M"
