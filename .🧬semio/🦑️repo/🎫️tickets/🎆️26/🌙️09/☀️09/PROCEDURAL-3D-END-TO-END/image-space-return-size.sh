#!/bin/sh
S=$1; G=$2; mkdir -p $S/m $S/q; I=$S/big.dmg; sz(){ echo $(( $(stat -f %b $I) * 512 / 1048576 )); }
diskutil image create blank --format RAW --size ${G} --fs "Case-sensitive APFS" --volumeName t $I >/dev/null 2>&1
diskutil image attach --nobrowse --mountPoint $S/m $I >/dev/null; CON=$(diskutil info -plist $S/m | plutil -extract APFSContainerReference raw -); diskutil apfs addVolume $CON "Case-sensitive APFS" semio-trim -nomount >/dev/null
head -c 1073741824 /dev/urandom > $S/m/blob; sync; rm $S/m/blob; sync; sleep 2; A=$(sz)
diskutil mount nobrowse -mountPoint $S/q ${CON}s2 >/dev/null; diskutil unmount ${CON}s2 >/dev/null; sleep 4; echo "${G}: deleted $A -> after kick $(sz) (host free $(df -g $S | tail -1 | awk '{print $4}') GiB)"
diskutil unmount $S/m >/dev/null; hdiutil detach $(hdiutil info | awk '/big.dmg/{f=1} f&&/\/dev\/disk/{print $1; exit}') -quiet; rm -rf $I $S/m $S/q
