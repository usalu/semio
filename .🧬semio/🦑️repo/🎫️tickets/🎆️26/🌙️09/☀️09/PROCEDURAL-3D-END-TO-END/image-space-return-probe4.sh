#!/bin/sh
# usage: image-space-return-probe4.sh <scratch-dir> <writer|nowriter> : isolates the trigger (addVolume vs mount of the sibling volume) and checks data safety with an open file in the busy main volume
D=$1; MODE=$2; I=$D/p4.dmg; M=$D/p4m; rm -rf "$I" "$M"; mkdir -p "$M"
diskutil image create blank --format RAW --size 1800m --fs APFS --volumeName t "$I" >/dev/null || exit 1
sz(){ stat -f '%b' "$I" | awk '{print $1*512/1048576}'; }
diskutil image attach --nobrowse --mountPoint "$M" "$I" >/dev/null
VOL=$(mount | awk -v m="$M" '$3==m{print $1}'); CON=$(diskutil info "$VOL" | awk -F': *' '/APFS Container:/{print $2}')
head -c 4194304 /dev/urandom > "$M/keep"; KEEP=$(shasum "$M/keep" | cut -c1-40)
W=; if [ $MODE = writer ]; then sh -c 'exec 3>>"$0"; exec sleep 600' "$M/open-writer" & W=$!; fi
churn(){ head -c 1073741824 /dev/urandom > "$M/blob"; sync; echo "   blob written: $(sz) MiB"; rm "$M/blob"; sync; sleep 2; }
churn; echo "A deleted: $(sz) MiB"
diskutil apfs addVolume "$CON" "Case-sensitive APFS" aux -nomount >/dev/null 2>&1; sleep 3; echo "B after addVolume -nomount: $(sz) MiB"
AUX=$(diskutil list "$CON" | awk '/ aux /{print $NF}' | head -1); echo "aux=$AUX"
diskutil mount "$AUX" >/dev/null 2>&1; sleep 2; echo "C after aux mount: $(sz) MiB"
diskutil unmount "$AUX" >/dev/null 2>&1; sleep 1; echo "C2 after aux unmount: $(sz) MiB"
for n in 1 2; do churn; echo "D$n churned: $(sz) MiB"; diskutil mount "$AUX" >/dev/null 2>&1; sleep 2; echo "D$n after aux mount: $(sz) MiB"; diskutil unmount "$AUX" >/dev/null 2>&1; sleep 1; echo "D$n after aux unmount: $(sz) MiB"; done
[ -n "$W" ] && kill $W
echo "keep intact: $([ "$(shasum "$M/keep" | cut -c1-40)" = "$KEEP" ] && echo yes || echo NO)  writer file bytes: $(wc -c < "$M/open-writer" 2>/dev/null)"
sleep 1; diskutil unmount "$VOL" >/dev/null; hdiutil detach "$(hdiutil info | awk -v i="$I" '$0~i{f=1} f&&/\/dev\/disk/{print $1; exit}')" -quiet 2>/dev/null
rm -rf "$I" "$M"
