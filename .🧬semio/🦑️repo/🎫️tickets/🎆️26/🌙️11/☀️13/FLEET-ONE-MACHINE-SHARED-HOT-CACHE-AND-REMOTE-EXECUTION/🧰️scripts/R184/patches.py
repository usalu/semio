import os,sys,difflib
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
N=B+"/🕸️graph/📥️inference/📦️project-nodes"
O=N+"/🌐️owner-pool"
OB=B+"/👁️observation"
P={}
P[f"{OB}/🟨️.mjs"]=[
('import { sourceTreeFilterV1 } from "./📂️source-names/🟨️.mjs";','import { hollowDirectoryV1, sourceTreeFilterV1 } from "./📂️source-names/🟨️.mjs";'),
('''function directoryFactV1(entries,typed=false,directory=null){
 const kept=sourceTreeFilterV1(entries,entry=>String(typed?entry.name:entry),entry=>typed?entry.isDirectory()||entry.isSymbolicLink():directory===null||directoryLinkOrDirectoryV1(join(directory,String(entry))));''','''function directoryFactV1(entries,typed=false,directory=null){
 const hollow=directory===null?undefined:entry=>{const name=String(typed?entry.name:entry);if(!(typed?entry.isDirectory():directoryOnlyV1(join(directory,name))))return false;return hollowDirectoryV1(join(directory,name),path=>world.readdirSync(path,{withFileTypes:true}),join);};
 const kept=sourceTreeFilterV1(entries,entry=>String(typed?entry.name:entry),entry=>typed?entry.isDirectory()||entry.isSymbolicLink():directory===null||directoryLinkOrDirectoryV1(join(directory,String(entry))),hollow);'''),
('function directoryLinkOrDirectoryV1(path){try{const stat=world.lstatSync(path);return stat.isDirectory()||stat.isSymbolicLink();}catch{return false;}}','function directoryLinkOrDirectoryV1(path){try{const stat=world.lstatSync(path);return stat.isDirectory()||stat.isSymbolicLink();}catch{return false;}}\nfunction directoryOnlyV1(path){try{return world.lstatSync(path).isDirectory();}catch{return false;}}'),
('readdir: (path) => { probeCountsV1.readdir++; try { return directoryFactV1(world.readdirSync(path, { withFileTypes: true }),true); }','readdir: (path) => { probeCountsV1.readdir++; try { return directoryFactV1(world.readdirSync(path, { withFileTypes: true }),true,path); }'),
]
P[f"{OB}/📂️source-names/🟨️.mjs"]=[
('''/** 🧿️ The entries of a directory listing that belong to the source tree as the build sees it: operating-system noise never, a generated directory (the cache policy list) never where `isDirectory` says the entry is a directory or a link, everything else always. https://git-scm.com/docs/gitignore */
export function sourceTreeFilterV1(entries,nameOf,isDirectory){
 return entries.filter(entry=>{const name=nameOf(entry);return!noise.has(name)&&!suffixes.some(suffix=>name.endsWith(suffix))&&!(generated.has(name)&&isDirectory(entry));});
}''','''/** 🧿️ The entries of a directory listing that belong to the source tree as the build sees it: operating-system noise never, a generated directory (the cache policy list) never where `isDirectory` says the entry is a directory or a link, a hollow directory (`isHollow`: nothing a repository can hold lies below it, so it is what a removed file leaves behind on one replica and never on another) never, everything else always. https://git-scm.com/docs/gitignore
 * @see ./🧪️tests/🕳️hollow/🟦️.ts */
export function sourceTreeFilterV1(entries,nameOf,isDirectory,isHollow){
 return entries.filter(entry=>{const name=nameOf(entry);return!noise.has(name)&&!suffixes.some(suffix=>name.endsWith(suffix))&&!(generated.has(name)&&isDirectory(entry))&&!(isHollow!==undefined&&isHollow(entry));});
}

/** 🕳️ Whether a directory holds no source file anywhere below it: every entry it lists is noise, generated output or a directory that is hollow itself (a link, a file and any entry that cannot be read count as source, so a doubt keeps the entry). `entriesOf(path)` lists a directory with its types; `depth` bounds the walk (a deeper tree is not hollow).
 * @see ./🧪️tests/🕳️hollow/🟦️.ts */
export function hollowDirectoryV1(path,entriesOf,join,depth=32){
 if(depth<=0)return false;
 let entries;try{entries=entriesOf(path);}catch{return false;}
 for(const entry of entries){
  const name=String(entry.name),isDirectory=entry.isDirectory()||entry.isSymbolicLink();
  if(sourceTreeFilterV1([entry],()=>name,()=>isDirectory).length===0)continue;
  if(!entry.isDirectory()||!hollowDirectoryV1(join(path,name),entriesOf,join,depth-1))return false;
 }
 return true;
}'''),
]
P[f"{N}/🟨️.mjs"]=[
("""neverHere: (names, childFootprintMegabytes) => { const here = hereLimitBytesV1({ limitBytes, soloBytes: soloBytes(), planBytes: planStageLimitBytesV1(environment) }), table = recorder.table(), estimate""","""neverHere: (names, childFootprintMegabytes) => { const here = hereLimitBytesV1({ limitBytes, soloBytes: soloBytes(), planBytes: planStageLimitBytesV1(environment) }), table = ownHostTableV1(recorder.table(), context.hostMegabytes ?? host), estimate"""),
("""/** 🧱️ The most one owner may take while it runs alone on a machine that has a plan limit""","""/** 🏠️ The cost table with only the samples that this machine or a smaller one measured: what a bigger host measured (a lax heap that shares its warm-up) informs an owner nobody here ran, but it is no evidence that the owner does not fit this machine, so it never bars an owner here; a stop that this machine records after a real overrun is its own evidence.
 * @see ./🧪️tests/🌍️bigger-host/🟦️.ts */
export function ownHostTableV1(table, hostMegabytes) {
  return { ...table, owners: Object.fromEntries(Object.entries(table.owners ?? {}).map(([name, entry]) => [name, { ...entry, samples: entry.samples.filter(sample => sample.hostMegabytes <= hostMegabytes) }])) };
}

/** 🔁️ The owners a read that waited for the fleet in vain runs here once, alone, before it refuses: all the owners it waited for the first time, none the second time. A pure function.
 * @see ./🧪️tests/🔁️retry-alone/🟦️.ts */
export function heavyRetryNamesV1(left, retried) { return retried ? [] : [...left]; }

/** 🧱️ The most one owner may take while it runs alone on a machine that has a plan limit"""),
("""const HEAVY_ENDED_V1 = Object.freeze({""","""const HEAVY_RETRY_V1 = Object.freeze({ en: "the fleet delivered nothing for {count} heavy owners ({names}); this machine runs them once itself, one at a time, before it refuses", de: "Die Flotte lieferte für {count} schwere Projekte nichts ({names}); diese Maschine leitet sie vor der Ablehnung einmal selbst ab, eines nach dem anderen" });
const HEAVY_ENDED_V1 = Object.freeze({"""),
("""    if(left.length>0){takeSay(language=>HEAVY_ENDED_V1[language==="de"?"de":"en"].replace("{count}",left.length).replace("{cause}",String(ended)));throw refuseHeavy(left);}
  };""","""    const again=heavyRetryNamesV1(left,retriedAlone);
    if(again.length>0){retriedAlone=true;takeSay(language=>HEAVY_RETRY_V1[language==="de"?"de":"en"].replace("{count}",again.length).replace("{names}",again.slice(0,3).join(", ")));for(const name of again){never.delete(name);const job=byName.get(name);if(job!==undefined)jobs.push(job);}await loop(Infinity);const rest=again.filter(name=>never.has(name)&&!["present","fetched","inferred"].includes(ledger.state(name)));if(rest.length>0){takeSay(language=>HEAVY_ENDED_V1[language==="de"?"de":"en"].replace("{count}",rest.length).replace("{cause}",String(ended)));throw refuseHeavy(rest);}return;}
    if(left.length>0){takeSay(language=>HEAVY_ENDED_V1[language==="de"?"de":"en"].replace("{count}",left.length).replace("{cause}",String(ended)));throw refuseHeavy(left);}
  };"""),
("""  const waitHeavy=async()=>{""","""  let retriedAlone=false;
  const waitHeavy=async()=>{"""),
]
P[f"{O}/🟦️.ts"]=[
(' if(input.targetReason!==null)return input.targetReason;\n if(input.children>0)return null;',' if(input.targetReason!==null&&input.targetChildren<=input.children)return input.targetReason;\n if(input.children>0)return null;'),
("A target of the capacity gate that says why (waiting, yielding) is passed through;","A target of the capacity gate that says why (waiting, yielding) is passed through while it holds the pool where it is or below (a target above the children that run is a pool that is starting, whatever bound the gate names);"),
]
P[f"{N}/🧪️tests/🐘️never-here/🟦️.ts"]=[(''' const small=await enforce(root,8192);
 expect(small.neverHere!(["@semio-tech/fits"])).toEqual(["@semio-tech/fits"]);''',''' recordOwnerCostFileV1(path,{name:"@semio-tech/fits",kind:"project",megabytes:1500,seconds:90,rows:10,stopped:false,atMs:now,hostMegabytes:8192});
 const small=await enforce(root,8192);
 expect(small.neverHere!(["@semio-tech/fits"])).toEqual(["@semio-tech/fits"]);''')]
P[f"{N}/🧪️tests/🪂️set-aside/🟦️.ts"]=[("setAsideOverrunV1({error:overrun(),never:new Set(),alone:new Set(),recorder:enforcement.recorder,factor,say:()=>{}});","await withTotal(16384,async()=>setAsideOverrunV1({error:overrun(),never:new Set(),alone:new Set(),recorder:enforcement.recorder,factor,say:()=>{}}));")]
P[f"{OB}/🧪️tests/🪜️row-sequence/🔣️.json"]=[("42d7279f6250725b4c7deb18027eb13c9c60706c8216daf47c8e999989858d5b","437554b3618d49c0d6e0744d5fbb3974654243f6809cb2bba4c9e9d2ec2330a6"),("9e66c054568a1da4feba99f44982ab4cab54da810604b0011b728b98e11b7164","c1691808111b31134e2b11f083586ce2b5d9cd35ebb91fb84df99efa20485650")]
P[f"{O}/🧪️tests/🌊️early-wave/🔣️.json"]=[('''{"name": "the gate says why",''','''{"name": "the gate names a bound while the pool still grows", "waves": [{"state": "open", "backoffUntil": 0, "acceptable": true}], "open": 1, "children": 1, "targetChildren": 8, "targetReason": "yielding", "expect": null},
    {"name": "the gate says waiting with no child", "waves": [{"state": "open", "backoffUntil": 0, "acceptable": true}], "open": 1, "children": 0, "targetChildren": 0, "targetReason": "waiting", "expect": "waiting"},
    {"name": "the gate says why",''')]
P[f"{OB}/📂️source-names/🧪️tests/🟦️.ts"]=[('''&&!/^(?:function|export function) directoryFactOfV1/u.test(line));''','''&&!/^(?:function|export function) directoryFactOfV1/u.test(line)&&!/hollowDirectoryV1\\(/u.test(line));''')]
NEW=[f"{OB}/📂️source-names/🧪️tests/🕳️hollow/🟦️.ts",f"{N}/🧪️tests/🌍️bigger-host/🟦️.ts",f"{N}/🧪️tests/🔁️retry-alone/🟦️.ts"]
def patched(live_root,rel):
    t=open(f"{live_root}/{rel}",encoding="utf8").read()
    for old,new in P[rel]:
        assert t.count(old)==1,(rel,old[:70],t.count(old))
        t=t.replace(old,new,1)
    return t
if __name__=="__main__":
    L="/Users/ueli/Documents/semio";S=sys.argv[1]
    for rel in P:
        mine=open(f"{S}/mirror/{rel}",encoding="utf8").read()
        got=patched(L,rel)
        print("EQUAL" if mine==got else "DIFF",rel.rsplit("/",3)[-3:])
        if mine!=got:
            for l in list(difflib.unified_diff(mine.splitlines(),got.splitlines(),lineterm="",n=0))[:8]: print("   ",l[:200])
