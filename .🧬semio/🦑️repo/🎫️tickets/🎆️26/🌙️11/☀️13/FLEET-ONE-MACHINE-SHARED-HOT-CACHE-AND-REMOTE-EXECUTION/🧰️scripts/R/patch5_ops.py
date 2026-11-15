import json,sys
H="/Users/Shared/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/"
B="/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
p=H+"🧬️schema/🔣️.json";s=json.load(open(p,encoding='utf8'))
if 'minimumMegabytes' in s['definitions']['Operation']['properties']: sys.exit("patched")
s['definitions']['Operation']['properties']['minimumMegabytes']={"type":"integer","minimum":0,"maximum":1048576}
open(p,'w',encoding='utf8').write(json.dumps(s,indent=2,ensure_ascii=False)+"\n")
p=B+"🔣️policy.json";s=json.load(open(p,encoding='utf8'))
for row in s['hostCapacity']['operations']:
    if row['name'] in ("show","graph","dependencies"): row['minimumMegabytes']=3072
open(p,'w',encoding='utf8').write(json.dumps(s,indent=2,ensure_ascii=False)+"\n")
p=H+"📏️decision/🟦️.ts";t=open(p,encoding='utf8').read()
a='fixedMegabytes:number;cpu:number;priority:"batch"|"interactive";forecast?:boolean}>;'
assert t.count(a)==1
t=t.replace(a,'fixedMegabytes:number;cpu:number;priority:"batch"|"interactive";forecast?:boolean;minimumMegabytes?:number}>;')
a='fixedMegabytes:operation.fixedMegabytes,perJobMegabytes:0,cpuPerJob:operation.cpu}));'
assert t.count(a)==1
t=t.replace(a,'fixedMegabytes:operation.fixedMegabytes,perJobMegabytes:0,cpuPerJob:operation.cpu,...operation.minimumMegabytes===undefined?{}:{minimumMegabytes:operation.minimumMegabytes}}));')
a='`forecast` lets a run of the operation hold only a small quiet reservation'
assert t.count(a)==1
t=t.replace(a,'`minimumMegabytes` (fixed shape) is the least memory the operation starts with when its fixed block does not fit in full (see `HostRequestV1`); `forecast` lets a run of the operation hold only a small quiet reservation')
open(p,'w',encoding='utf8').write(t);print("ok")
