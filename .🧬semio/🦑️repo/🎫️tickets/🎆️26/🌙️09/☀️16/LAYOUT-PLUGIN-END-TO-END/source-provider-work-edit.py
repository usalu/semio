from pathlib import Path
import hashlib,json,sys
ROOT=Path.cwd()
TICKET=Path(__file__).resolve().parent
TARGET=ROOT/"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts"
def sha(value):
 return hashlib.sha256(value).hexdigest()
raw=TARGET.read_bytes()
text=raw.decode("utf-8")
changes=[
 ('manifests=new Map<string,Readonly<{debits:number;profile:ReturnType<typeof readJavascriptDependencySourceOwnerManifestProfileV1>}>>()','manifests=new Map<string,ReturnType<typeof readJavascriptDependencySourceOwnerManifestProfileV1>>()'),
 ('const physical=(input:string):Readonly<{path:string;facts:BigIntStats}>|null=>','const physicalOnce=(input:string):Readonly<{path:string;facts:BigIntStats}>|null=>'),
 ('if(hit){for(let debit=0;debit<hit.debits;debit++)charge();return hit.profile;}','if(hit){charge();return hit;}'),
 ('let debits=0;const profile=readJavascriptDependencySourceOwnerManifestProfileV1(bytes.toString(),()=>{debits++;charge();});manifests.set(key,Object.freeze({debits,profile}));return profile;','const profile=readJavascriptDependencySourceOwnerManifestProfileV1(bytes.toString(),()=>charge());manifests.set(key,profile);return profile;'),
]
for before,after in changes:
 if text.count(before)!=1:raise RuntimeError("Exact unique guarded fragment required")
 text=text.replace(before,after,1)
lines=text.splitlines(keepends=True)
index=next(i for i,line in enumerate(lines) if line.startswith(" const physicalOnce="))
lines.insert(index+1," const physicalMemo=new Map<string,ReturnType<typeof physicalOnce>>(),physical=(input:string)=>{charge();const path=requireMaterializedPathV1(resolve(input));if(!physicalMemo.has(path))physicalMemo.set(path,physicalOnce(path));return physicalMemo.get(path)??null;};\n")
output="".join(lines).encode("utf-8")
packet={"version":1,"target":str(TARGET.relative_to(ROOT)),"beforeSha256":sha(raw),"afterSha256":sha(output),"limitsChanged":False,"sourceApplied":False}
packetPath=TICKET/"🗑️generated/source-provider-work-patch.json"
if sys.argv[1:]==["--prepare"]:
 packetPath.write_text(json.dumps(packet,ensure_ascii=False,indent=2)+"\n",encoding="utf-8")
 print(json.dumps(packet,ensure_ascii=False))
elif sys.argv[1:]==["--apply"]:
 prior=json.loads(packetPath.read_text())
 if prior["beforeSha256"]!=sha(raw) or prior["afterSha256"]!=sha(output):raise RuntimeError("Prepared exact Source bytes changed")
 if TARGET.read_bytes()!=raw:raise RuntimeError("Source changed before write")
 TARGET.write_bytes(output)
 print(json.dumps({**packet,"sourceApplied":True},ensure_ascii=False))
else:raise RuntimeError("Use --prepare or --apply")
