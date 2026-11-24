#!/bin/sh
# usage: image-space-return-experiment.sh <kind> <attacher> <scratch-dir>
# kind: raw|asif|udsb|hdsparse|hdsb   attacher: hdiutil|diskutil
K=$1; A=$2; D=$3; M=$D/m-$K-$A; I=$D/img-$K-$A
case $K in raw) I=$I.dmg;; asif) I=$I.asif;; udsb|hdsb) I=$I.sparsebundle;; hdsparse) I=$I.sparseimage;; esac
rm -rf "$I" "$M"; mkdir -p "$M"
case $K in
 raw|asif|udsb) F=$(echo $K | tr a-z A-Z); diskutil image create blank --format $F --size 1800m --fs APFS --volumeName t "$I" >/dev/null || exit 1;;
 hdsparse) hdiutil create -size 1800m -type SPARSE -fs APFS -volname t -quiet "$I" ;;
 hdsb) hdiutil create -size 1800m -type SPARSEBUNDLE -fs APFS -volname t -quiet "$I" ;;
esac
[ $K = hdsparse ] && I=$I
sz(){ if [ -d "$I" ]; then du -sk "$I" | awk '{print $1*1024}'; else stat -f '%b' "$I" | awk '{print $1*512}'; fi; }
mb(){ echo $(( $1 / 1048576 )); }
if [ $A = hdiutil ]; then hdiutil attach "$I" -nobrowse -noautoopen -mountpoint "$M" -quiet; else diskutil image attach --nobrowse --mountPoint "$M" "$I" >/dev/null; fi || exit 1
echo "$K/$A created+attached: $(mb $(sz)) MiB"
head -c 1073741824 /dev/urandom > "$M/blob"; sync; sleep 2
echo "$K/$A after write 1GiB:  $(mb $(sz)) MiB"
rm "$M/blob"; sync; sleep 5
echo "$K/$A after delete(+5s): $(mb $(sz)) MiB"
sleep 10; echo "$K/$A after +10s:        $(mb $(sz)) MiB"
diskutil apfs list >/dev/null 2>&1
if [ $K = hdsparse -o $K = hdsb ]; then c=$(hdiutil compact "$I" 2>&1 | tail -1); echo "  compact while attached: $c => $(mb $(sz)) MiB"; fi
DEV=$(mount | awk -v m="$M" '$3==m{print $1}'); 
echo "  mount: $(mount | grep " $M " | cut -d'(' -f2)"
hdiutil detach "$M" -quiet; sleep 2
echo "$K/$A after detach:      $(mb $(sz)) MiB"
if [ $K = hdsparse -o $K = hdsb ]; then c=$(hdiutil compact "$I" 2>&1 | tail -1); echo "  compact detached: $c => $(mb $(sz)) MiB"; fi
if [ $A = hdiutil ]; then hdiutil attach "$I" -nobrowse -noautoopen -mountpoint "$M" -quiet; else diskutil image attach --nobrowse --mountPoint "$M" "$I" >/dev/null; fi; sleep 3; hdiutil detach "$M" -quiet; sleep 2
echo "$K/$A after reattach+detach cycle: $(mb $(sz)) MiB"
rm -rf "$I" "$M"
