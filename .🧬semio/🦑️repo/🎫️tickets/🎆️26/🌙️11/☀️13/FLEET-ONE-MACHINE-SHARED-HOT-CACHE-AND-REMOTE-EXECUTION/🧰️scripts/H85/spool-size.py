import os,re,sys
ROOT="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
OUT=sys.argv[1] if len(sys.argv)>1 else ROOT
def p(rel):return os.path.join(OUT,rel)
FLEET="⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet"
UP=FLEET+"/📤️uploader/🟦️.ts"
NEW=FLEET+"/📏️spool-size/🟦️.ts"
def rd(rel):
    src=os.path.join(ROOT,rel);return open(src,encoding="utf8").read()
writes={}
writes[NEW]='''import {readdirSync,statfsSync} from "node:fs";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const fleet=schema.definitions.Policy.const;

/** 📮️ The directory of the upload spool of a cache root. */
export const spoolDirectoryV1=(root:string)=>join(root,fleet.layout.state,fleet.layout.spool);

const bounds=new Map<string,{at:number;entries:number}>();

/** 📏️ How many units the spool of this cache root may hold: `spool.freeSharePercent` of the free room of its disk, one entry per `spool.entryBytes`, between `spool.minimumEntries` and `spool.maximumEntries` (a machine with a small disk still holds a build's worth, a big one never fills its disk with queue files). Re-measured every `spool.boundTtlMilliseconds`. This module reads only the policy and the file system, so a status reader never loads the uploader (and with it the revision and cache modules). */
export function spoolBoundV1(root:string):number{
 const policy=fleet.spool,known=bounds.get(root),now=Date.now();
 if(known!==undefined&&now-known.at<policy.boundTtlMilliseconds)return known.entries;
 let entries=policy.minimumEntries;
 try{const stats=statfsSync(root);entries=Math.floor(Number(stats.bavail)*Number(stats.bsize)*policy.freeSharePercent/100/policy.entryBytes);}catch{}
 entries=Math.min(policy.maximumEntries,Math.max(policy.minimumEntries,entries));
 bounds.set(root,{at:now,entries});return entries;
}

/** 🔢️ How many units wait in the spool. */
export function spoolLengthV1(root:string):number{try{return readdirSync(spoolDirectoryV1(root)).filter(name=>name.endsWith(".json")).length;}catch{return 0;}}
'''
# uploader
t=rd(UP)
def cut(t,a,b,note):
    i=t.index(a);j=t.index(b,i);return t[:i]+t[j:]
t=t.replace("const spoolDirectory=(root:string)=>join(root,fleet.layout.state,fleet.layout.spool);\n","")
i=t.index("const bounds=new Map<string,{at:number;entries:number}>();");j=t.index("/** 📮️ Queues a unit for upload")
t=t[:i]+t[j:]
i=t.index("/** 🔢️ How many units wait in the spool. */\nexport function spoolLengthV1");j=t.index("\n",t.index("export function spoolLengthV1"))+1
t=t[:i]+t[j:]
assert "spoolDirectory(" in t
t=t.replace("spoolDirectory(","spoolDirectoryV1(")
anchor='import {registerSpoolV1} from "../📮️registry/🟦️.ts";\n'
assert anchor in t
t=t.replace(anchor,anchor+'import {spoolBoundV1,spoolDirectoryV1,spoolLengthV1} from "../📏️spool-size/🟦️.ts";\n')
for name in ["statfsSync"]:
    body=t.replace('import {existsSync,mkdirSync,readFileSync,readdirSync,renameSync,rmSync,statSync,statfsSync,writeFileSync} from "node:fs";','')
    if len(re.findall(r"\bstatfsSync\b",body))==0:t=t.replace("statSync,statfsSync,","statSync,")
writes[UP]=t
# importers
rx=re.compile(r'^(import \{([^}]*)\} from "([^"]*📤️uploader/🟦️\.ts|\.\./🟦️\.ts|\./📤️uploader/🟦️\.ts)";)$',re.M)
def fix(rel,expectRel):
    text=writes.get(rel) or rd(rel)
    d=os.path.dirname(rel)
    def sub(m):
        names=[n.strip() for n in m.group(2).split(",") if n.strip()]
        spool=[n for n in names if n.split()[-1] in("spoolBoundV1","spoolLengthV1")]
        if not spool:return m.group(0)
        rest=[n for n in names if n not in spool]
        target=os.path.relpath(os.path.join(ROOT,NEW),os.path.join(ROOT,d))
        if not target.startswith('.'):target='./'+target
        out=[]
        if rest:out.append("import {"+",".join(rest)+"} from \""+m.group(3)+"\";")
        out.append("import {"+",".join(spool)+"} from \""+target+"\";")
        return "\n".join(out)
    new=rx.sub(sub,text)
    assert new!=text,rel
    writes[rel]=new
FILES=[
 "⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/📋️manifest/🟦️.ts",
 "⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/🕵️leaks/🟦️.ts",
 FLEET+"/📮️registry/🧪️tests/🟦️.ts",
 FLEET+"/🧪️tests/🌳️rooted/🟦️.ts",
 FLEET+"/🧪️tests/🧷️session/🟦️.ts",
 FLEET+"/🧪️tests/🟦️.ts",
 FLEET+"/🟦️.ts",
 FLEET+"/📤️uploader/🧪️tests/🟦️.ts",
 "🏗️build/🌐️remote-execution/🛰️sidecar/📮️spool/🟦️.ts",
]
for f in FILES:fix(f,None)
# reader
R="🏗️build/🚢️fleet-status/📥️readers/🟦️.ts"
t=rd(R)
assert t.count("spawnSync")==2
t=t.replace('import {spawnSync} from "node:child_process";','import {execFile} from "node:child_process";')
a='const listed=spawnSync("ps",["-o","pid=,ppid=","-p",pids.join(",")],{encoding:"utf8",timeout:2000});for(const line of String(listed.stdout??"").split("\\n")){'
assert a in t
t=t.replace(a,'const listed=await new Promise<string>(done=>{try{execFile("ps",["-o","pid=,ppid=","-p",pids.join(",")],{encoding:"utf8",timeout:2000},(_failure,stdout)=>done(String(stdout??"")));}catch{done("");}});for(const line of listed.split("\\n")){')
b='await import("../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📤️uploader/🟦️.ts")'
assert t.count(b)==1
t=t.replace(b,'await import("../../../⚡️caching/🦀️cargo/🗃️unit-cache/🛰️fleet/📏️spool-size/🟦️.ts")')
writes[R]=t
for rel,content in writes.items():
    path=p(rel);os.makedirs(os.path.dirname(path),exist_ok=True)
    open(path+".h85.tmp","w",encoding="utf8").write(content);os.replace(path+".h85.tmp",path)
print("wrote",len(writes))
