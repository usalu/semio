#!/bin/sh
S=$1; M=$2; mkdir -p $S/m $S/q $S/seed; echo x > $S/seed/x; I=$S/ud.dmg; sz(){ echo $(( $(stat -f %b $I) * 512 / 1048576 )); }
hdiutil create -size 64m -format UDIF -nospotlight -fs "Case-sensitive APFS" -volname semio -srcfolder $S/seed -quiet $I; hdiutil resize -size ${M}m $I >/dev/null 2>&1
diskutil image attach --nobrowse --mountOptions owners --mountPoint $S/m $I >/dev/null; CON=$(diskutil info -plist $S/m | plutil -extract APFSContainerReference raw -); diskutil apfs addVolume $CON "Case-sensitive APFS" semio-trim -nomount >/dev/null
head -c 1073741824 /dev/urandom > $S/m/blob; sync; rm $S/m/blob; sync; sleep 2; A=$(sz)
diskutil mount nobrowse -mountPoint $S/q ${CON}s2 >/dev/null; diskutil unmount ${CON}s2 >/dev/null; sleep 4; echo "udif+resize ${M}m: deleted $A -> after kick $(sz)"
diskutil unmount $S/m >/dev/null; hdiutil detach $(hdiutil info | awk '/ud.dmg/{f=1} f&&/\/dev\/disk/{print $1; exit}') -quiet; rm -rf $I $S/m $S/q $S/seed
