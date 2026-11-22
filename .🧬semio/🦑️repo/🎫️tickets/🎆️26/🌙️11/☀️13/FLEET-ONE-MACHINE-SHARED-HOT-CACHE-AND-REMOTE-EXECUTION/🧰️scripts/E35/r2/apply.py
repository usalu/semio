import sys,os,shutil,hashlib
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
G=L+"⚡️caching/🚀️bootstrap/📦️dependencies/🧾️generation/"
BUN=L+"🗂️workspaces/🟦️bun/"
R=os.path.dirname(os.path.abspath(__file__))+"/profile/"
CHECK=os.environ.get('CHECK')=='1'
backup=sys.argv[1]
os.makedirs(backup,exist_ok=True)
def edit(path,pairs):
    t=open(path,encoding="utf8").read()
    shutil.copy(path,os.path.join(backup,hashlib.sha1(path.encode()).hexdigest()+".orig"))
    for a,b in pairs:
        assert t.count(a)==1,(path,a[:80])
        t=t.replace(a,b)
    if not CHECK:open(path,"w",encoding="utf8").write(t)
def new(src,dst):
    if not CHECK:os.makedirs(os.path.dirname(dst),exist_ok=True)
    assert not os.path.exists(dst),dst
    if not CHECK:shutil.copy(R+src,dst)
P=G+"🪪️profile/"
new("schema/schema.json",P+"🧬️schema/🔣️.json");new("profile.ts",P+"🟦️.ts")
if not CHECK:os.makedirs(P+"🧪️tests",exist_ok=True)
if not CHECK:shutil.copy(R+"tests/test.ts",P+"🧪️tests/🟦️.ts");shutil.copy(R+"tests/cases.json",P+"🧪️tests/🔣️.json");shutil.copy(R+"tests/oracle.py",P+"🧪️tests/🐍️.py")
edit(G+"💾️storage/🟦️.ts",[
 ('import {readBunWorkspaceListingProjectionV1','import {manifestProfileSha256V1} from "../🪪️profile/🟦️.ts";\nimport {readBunWorkspaceListingProjectionV1'),
 ('/** 🗺️ Maps one completed walk onto the neutral capture input;','/** 🪪️ Derives the receipt profile of one retained manifest text from the file as it stands, only while the file still holds exactly the retained text. */\nconst manifestTextProfileV1=(root:string,row:Readonly<{path:string;sha256:string}>):string|null=>{try{const bytes=readFileSync(join(root,row.path));return createHash("sha256").update(bytes).digest("hex")===row.sha256?manifestProfileSha256V1(row.path,bytes):null;}catch{return null;}};\n/** 🗺️ Maps one completed walk onto the neutral capture input;'),
 ('  membership:observed,\n','  membership:{...observed,texts:observed.texts.map(row=>({...row,profileSha256:manifestTextProfileV1(root,row)}))},\n'),
])
edit(G+"🟦️.ts",[
 ('import {javascriptGenerationSchemaV1 as schema}','import {manifestProfileSha256V1} from "./🪪️profile/🟦️.ts";\nimport {javascriptGenerationSchemaV1 as schema}'),
 ('mode:number;sha256:string|null}>[]}>[];','mode:number;sha256:string|null;profileSha256:string|null}>[]}>[];'),
 ('texts:readonly Readonly<{path:string;sha256:string}>[]}>;','texts:readonly Readonly<{path:string;sha256:string;profileSha256:string|null}>[]}>;'),
 ('texts:readonly Readonly<{path:string;sha256:string}>[]}\n','texts:readonly Readonly<{path:string;sha256:string;profileSha256:string|null}>[]}\n'),
 ('sha256:source.kind==="file"?generationSha256V1(source.bytes):null})','sha256:source.kind==="file"?generationSha256V1(source.bytes):null,profileSha256:source.kind==="file"?manifestProfileSha256V1(source.path,source.bytes):null})'),
 ('texts:membership.texts.map(row=>({path:row.path,sha256:row.sha256}))','texts:membership.texts.map(row=>({path:row.path,sha256:row.sha256,profileSha256:row.profileSha256}))'),
 ('if(!facts||facts.isSymbolicLink()||(source.kind==="file"?!facts.isFile():!facts.isDirectory())||String(facts.dev)!==source.dev||String(facts.ino)!==source.ino||Number(facts.mode)!==source.mode)return mismatch("input-changed",source.path,"identity");\n   if(source.kind==="file"){if(facts.size>BigInt(protocol.maximumSourceFileBytes)||generationSha256V1(readGenerationData(path,protocol.maximumSourceFileBytes))!==source.sha256)return mismatch("input-changed",source.path,"bytes");}',
  'if(!facts||facts.isSymbolicLink()||(source.kind==="file"?!facts.isFile():!facts.isDirectory())||(String(facts.dev)!==source.dev||String(facts.ino)!==source.ino||Number(facts.mode)!==source.mode)&&source.profileSha256===null)return mismatch("input-changed",source.path,"identity");\n   if(source.kind==="file"){if(facts.size>BigInt(protocol.maximumSourceFileBytes))return mismatch("input-changed",source.path,"bytes");const data=readGenerationData(path,protocol.maximumSourceFileBytes);if(generationSha256V1(data)!==source.sha256&&(source.profileSha256===null||manifestProfileSha256V1(path,data)!==source.profileSha256))return mismatch("input-changed",source.path,"bytes");}'),
 ('if(generationSha256V1(text)!==row.sha256)return mismatch("manifest-changed",row.path,"bytes");','if(generationSha256V1(text)!==row.sha256&&(row.profileSha256===null||manifestProfileSha256V1(row.path,text)!==row.profileSha256))return mismatch("manifest-changed",row.path,"bytes");'),
])
import json
p=G+"🧬️schema/🔣️.json"
raw=open(p,encoding="utf8").read();shutil.copy(p,os.path.join(backup,"schema.orig"))
s=json.loads(raw);assert json.dumps(s,indent=2,ensure_ascii=False)+"\n"==raw
d=s["$defs"];profile={"anyOf":[{"type":"string","pattern":"^[a-f0-9]{64}$"},{"type":"null"}]}
assert d["Protocol"]["const"]["walkStamp"]=="semio.dependency-generation-walk/2"
d["Protocol"]["const"]["walkStamp"]="semio.dependency-generation-walk/3"
for name in ("Source","Text"):
    assert "profileSha256" not in d[name]["properties"]
    d[name]["required"].append("profileSha256");d[name]["properties"]["profileSha256"]=profile
if not CHECK:open(p,"w",encoding="utf8").write(json.dumps(s,indent=2,ensure_ascii=False)+"\n")
print("applied")
