import sys
sys.path.insert(0,sys.argv[2])
from edit import *
f=sys.argv[1]
s=open(f,encoding="utf-8").read()
lines=s.split("\n")
def idx(prefix,start=0):
    for i in range(start,len(lines)):
        if lines[i].startswith(prefix): return i
    raise SystemExit("missing line "+prefix[:60])
def drop(prefix,count=1,start=0):
    i=idx(prefix,start);del lines[i:i+count]
# 1 imports and helpers
drop('import{nativeFixedPreparationMetadataScriptV1')
drop('import{nativeFixedAllocationScriptV1')
drop('import{nativeFixedWindowsSystemV1,requireNativeFixedWindowsHostV1')
drop('/** 💾️ Flushes only the captured actual Windows image volume',3)
drop('/** 🪟️ Admits only a checksummed fixed VHD footer',6)
drop('/** 🔎️ Reads only the bounded fixed VHD footer',2)
drop('/** 🏠️ Dispatches only the actual private initial-host creator',4)
drop('const literal=(value:string)=>')
i=idx('const windows:NativeFixedBackendV1=');j=i
while not lines[j].endswith('captureFixedReadV1(options,"deactivate");}};'): j+=1
del lines[i:j+1]
i=idx('/** 📂️ Retains each physical same-volume directory identity');j=idx('export function syncNativeFixedImageMetadataV1',i);del lines[i:j+1]
i=idx('import{readNativeFixedWindowsImageReceiptV1');del lines[i:]
drop('export{captureFixedWindowsCurrentV1 as nativeFixedWindowsCurrentV1')
s="\n".join(lines).rstrip("\n")+"\n"
open(f,"w",encoding="utf-8").write(s)
