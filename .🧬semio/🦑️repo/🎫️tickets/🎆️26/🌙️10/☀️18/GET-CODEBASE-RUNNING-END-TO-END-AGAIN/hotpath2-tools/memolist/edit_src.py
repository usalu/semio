"""📚 Source edits of batch `memolist` (schema, memo module, relevance builder) on a root, starting from the live files: python3 -I edit_src.py <root>.
The refinement: a directory of the discovery frontier carries the entry stamp (mtime, inode) the generation captured; a content edit ('modified' row) of an unread sibling there leaves that stamp as it was, which is the planner's own proof that no name, type or absence in the directory moved (`stableDirectoriesV1`), so the memo settles such a row with one lstat instead of refusing. Rows that add, remove or rename a name there, directories a resolution, a derived fact or a symbolic-link-following listing names, directories without a stamp and racy stamps keep the strict rule."""
import json, os, shutil, sys
root = sys.argv[1]
SUB = "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/"
G = root + SUB
live = os.path.dirname(os.path.abspath(__file__))
while not os.path.isdir(os.path.join(live, "🧰️framework")) and live != "/": live = os.path.dirname(live)
if os.path.abspath(live) != os.path.abspath(root):
    for rel in ("🧬️schema/🔣️.json", "🟦️.ts", "📤️relevance/🟦️.ts"):
        os.makedirs(os.path.dirname(G + rel), exist_ok=True)
        shutil.copyfile(live + SUB + rel, G + rel)
def edit(rel, pairs):
    path = G + rel
    s = open(path, encoding="utf8").read()
    for old, new in pairs:
        assert s.count(old) == 1, (rel, old[:90], s.count(old))
        s = s.replace(old, new)
    open(path, "w", encoding="utf8").write(s)

schema_path = G + "🧬️schema/🔣️.json"
d = json.load(open(schema_path, encoding="utf8"))
c = d["definitions"]["Policy"]["const"]
out = {}
for key, value in c.items():
    out[key] = value
    if key == "recheckSuffix": out["stampSuffix"] = ".lst"
    if key == "maximumRecheckBytes": out["maximumStampBytes"] = 67108864
    if key == "argumentsDenied":
        out["stamped"] = ["readdir", "readdirNames"]
        out["following"] = ["directories"]
d["definitions"]["Policy"]["const"] = out
rec = d["definitions"]["Record"]["properties"]["relevance"]
rec["required"] = ["id", "stamps", "rows", "rechecked"]
rec["properties"] = {"id": rec["properties"]["id"], "stamps": {"$ref": "#/definitions/Sha256"}, "rows": rec["properties"]["rows"], "rechecked": rec["properties"]["rechecked"]}
side = d["definitions"]["Sidecar"]
side["required"] = ["version", "pointer", "rows", "rechecked", "directories", "strict", "stamped", "linked", "unsealable"]
props = {}
for key, value in side["properties"].items():
    props[key] = value
    if key == "directories":
        props["strict"] = {"type": "integer", "minimum": 0}
        props["stamped"] = {"type": "integer", "minimum": 0}
side["properties"] = props
d["$comment"] = d["$comment"] + " Stamp refinement: the discovery frontier of a generation holds, per directory, the entry stamp (mtime, inode) captured with it; the planner itself treats a directory with an unmoved stamp (and not touched within the racy window of the capture) as one whose names, node types and absences cannot have moved (`stableDirectoriesV1`). A directory of the frontier whose facts are only entries and listings of kinds `Policy.stamped` is STAMPED: a row that adds, removes or renames a name directly inside it is relevant, a row that only says 'modified' (a content edit of a sibling no fact read; the kind is advisory) is settled by one lstat against the stamp kept in the table `.lst`. Directories a resolution or derived fact names, directories read by a kind of `Policy.following` (it follows symbolic links), and directories without a usable stamp are STRICT: every row directly inside is relevant (except the modification of a directory)."
open(schema_path, "w", encoding="utf8").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")

edit("🟦️.ts", [
 ("import {createHash} from \"node:crypto\";\nimport {existsSync,lstatSync,", "import {createHash} from \"node:crypto\";\nimport {existsSync,lstatSync,"),
 ("relevanceSuffix:string;recheckSuffix:string;maximumRecordBytes:number;", "relevanceSuffix:string;recheckSuffix:string;stampSuffix:string;maximumRecordBytes:number;"),
 ("maximumRecheckBytes:number;maximumRows:number;", "maximumRecheckBytes:number;maximumStampBytes:number;maximumRows:number;"),
 ("argumentsDenied:readonly string[];environmentPrefixes", "argumentsDenied:readonly string[];stamped:readonly string[];following:readonly string[];environmentPrefixes"),
 ("""/** 🗺️ The directories that matter for a set of vouchable entries:""",
  """/** 🧩️ How one vouched fact reads the directories it names: `names` every entry it reads (a row naming one of them, or an ancestor, is relevant), `listing` the directories whose children it lists by name or node type (a stamp of the frontier can vouch for them), `strict` the directories where any row directly inside is relevant (resolutions and derived facts, which probe candidates the table does not enumerate, and listings that follow symbolic links). Nothing for a fact the journal cannot see. */
export function memoClassesV1(row:MemoRowV1,reach:Readonly<{mentions:readonly string[];recheck:boolean}>):Readonly<{names:readonly string[];strict:readonly string[];listing:readonly string[]}>{
 if(reach.recheck)return{names:[],strict:[],listing:[]};
 const how=GRAPH_MEMO_POLICY_V1.relevance[row.kind];
 if(how==="path"&&GRAPH_MEMO_POLICY_V1.stamped.includes(row.kind))return{names:reach.mentions,strict:[],listing:reach.mentions};
 if(how==="path"&&GRAPH_MEMO_POLICY_V1.following.includes(row.kind))return{names:reach.mentions,strict:reach.mentions,listing:[]};
 if(how==="resolve"||how==="text")return{names:reach.mentions,strict:reach.mentions.flatMap(mention=>{const cut=mention.lastIndexOf("/");return[mention,cut<0?"":mention.slice(0,cut)];}),listing:[]};
 return{names:reach.mentions,strict:[],listing:[]};
}

/** 🗺️ The directories that matter for a set of vouchable entries:"""),
])
src = open(G + "🟦️.ts", encoding="utf8").read()
a = src.index('const MAGIC=Buffer.from("SMR2");')
b = src.index("/** 🧾️ The identity check of an analysed source")
new_block = '''const MAGIC=Buffer.from("SMR3");
/** 📦️ Packs the three directory sets of a relevance (names, strict, stamped) as sorted 53-bit hashes behind a small header (magic, folding flag, three counts). `strict` defaults to every name and `stamped` to none, which is the rule without a stamp. */
export function packMemoRelevanceV1(names:ReadonlySet<string>,insensitive:boolean,strict:ReadonlySet<string>=names,stamped:ReadonlySet<string>=new Set<string>()):Buffer{
 const sets=[names,strict,stamped].map(set=>{const hashes=new Float64Array(set.size);let count=0;for(const path of set)hashes[count++]=memoHashV1(path,insensitive);return hashes.sort();});
 const out=Buffer.alloc(20+sets.reduce((sum,hashes)=>sum+hashes.length*8,0));
 MAGIC.copy(out,0);out.writeUInt32BE(insensitive?1:0,4);
 let at=20;
 for(const [index,hashes] of sets.entries()){out.writeUInt32BE(hashes.length,8+index*4);Buffer.from(hashes.buffer,hashes.byteOffset,hashes.length*8).copy(out,at);at+=hashes.length*8;}
 return out;
}
/** 🔓️ A packed relevance: `holds(path)` is true for a name (an entry some fact read, every directory above it, the frontier), `strict(path)` for a directory whose every direct child row is relevant, `stamped(path)` for a directory whose names the frontier stamp vouches for (by hash). */
export type MemoRelevanceV1=Readonly<{insensitive:boolean;count:number;holds(path:string):boolean;strict(path:string):boolean;stamped(path:string):boolean}>;
/** 📭️ Reads a packed relevance back; bytes of another shape are `undefined`. */
export function unpackMemoRelevanceV1(bytes:Uint8Array):MemoRelevanceV1|undefined{
 const buffer=Buffer.from(bytes.buffer,bytes.byteOffset,bytes.byteLength);
 if(buffer.length<20||!buffer.subarray(0,4).equals(MAGIC))return undefined;
 const insensitive=buffer.readUInt32BE(4)===1,counts=[8,12,16].map(position=>buffer.readUInt32BE(position));
 if(buffer.length!==20+counts.reduce((sum,count)=>sum+count*8,0))return undefined;
 let at=20;
 const tables=counts.map(count=>{const hashes=new Float64Array(count);Buffer.from(hashes.buffer).set(buffer.subarray(at,at+count*8));at+=count*8;return hashes;});
 const has=(hashes:Float64Array,path:string):boolean=>{
  const wanted=memoHashV1(path,insensitive);let low=0,high=hashes.length-1;
  while(low<=high){const mid=(low+high)>>>1,value=hashes[mid]!;if(value===wanted)return true;if(value<wanted)low=mid+1;else high=mid-1;}
  return false;
 };
 return{insensitive,count:counts[0]!,holds:path=>has(tables[0]!,path),strict:path=>has(tables[1]!,path),stamped:path=>has(tables[2]!,path)};
}

const STAMPS=Buffer.from("SML1");
/** 🪧️ Packs the entry stamps `[mtimeMs, inode]` of the stamped directories as records (hash, mtime, inode) sorted by hash behind a small header (magic, folding flag, count). */
export function packMemoStampsV1(stamps:ReadonlyMap<string,readonly[number,number]>,insensitive:boolean):Buffer{
 const records=[...stamps].map(([path,[mtimeMs,ino]])=>[memoHashV1(path,insensitive),mtimeMs,ino] as const).sort((a,b)=>a[0]-b[0]),out=Buffer.alloc(12+records.length*24);
 STAMPS.copy(out,0);out.writeUInt32BE(insensitive?1:0,4);out.writeUInt32BE(records.length,8);
 for(const [index,[hash,mtimeMs,ino]] of records.entries()){out.writeDoubleBE(hash,12+index*24);out.writeDoubleBE(mtimeMs,20+index*24);out.writeDoubleBE(ino,28+index*24);}
 return out;
}
/** ✅️ Whether every directory still has the stamp the frontier captured (same inode, same modification time): the proof the planner uses that no name, node type or absence below it moved. A directory without a record, or a table of another shape, does not hold. */
export function memoStampsHoldV1(root:string,table:Uint8Array,directories:Iterable<string>,insensitive:boolean):boolean{
 const buffer=Buffer.from(table.buffer,table.byteOffset,table.byteLength);
 if(buffer.length<12||!buffer.subarray(0,4).equals(STAMPS)||(buffer.readUInt32BE(4)===1)!==insensitive)return false;
 const count=buffer.readUInt32BE(8);if(buffer.length!==12+count*24)return false;
 for(const directory of directories){
  const wanted=memoHashV1(directory,insensitive);let low=0,high=count-1,found=-1;
  while(low<=high){const mid=(low+high)>>>1,value=buffer.readDoubleBE(12+mid*24);if(value===wanted){found=mid;break;}if(value<wanted)low=mid+1;else high=mid-1;}
  if(found<0)return false;
  try{const status=lstatSync(join(root,directory));if(!status.isDirectory()||status.mtimeMs!==buffer.readDoubleBE(20+found*24)||status.ino!==buffer.readDoubleBE(28+found*24))return false;}catch{return false;}
 }
 return true;
}

/** ⚖️ What a stretch of journal rows means for a relevance: the first row that is relevant by itself (it names an entry some fact read, an ancestor of one or a frontier directory; it lies directly inside a strict directory, except for the modification of a directory; it adds, removes or renames a name directly inside a stamped directory), and for every stamped directory the first row that only says 'modified' directly inside it, which the stamp of that directory must still confirm because the kind of a row is advisory. */
export function classifyMemoChangesV1(relevance:MemoRelevanceV1,rows:readonly ChangeRowV1[]):Readonly<{changed?:ChangeRowV1;pending:ReadonlyMap<string,ChangeRowV1>}>{
 const pending=new Map<string,ChangeRowV1>();
 for(const row of rows){
  const path=row[2],cut=path.lastIndexOf("/"),parent=cut<0?"":path.slice(0,cut);
  if(relevance.holds(path))return{changed:row,pending};
  if(relevance.strict(parent)&&!(row[0]==="m"&&row[1]===1))return{changed:row,pending};
  if(relevance.stamped(parent)){if(row[0]!=="m")return{changed:row,pending};if(!pending.has(parent))pending.set(parent,row);}
 }
 return{pending};
}
/** 🔬️ The first row that touches what the memo read when no stamp table is at hand: a row that is relevant by itself, else the first row that needs a stamp. */
export function firstRelevantChangeV1(relevance:MemoRelevanceV1,rows:readonly ChangeRowV1[]):ChangeRowV1|undefined{
 const found=classifyMemoChangesV1(relevance,rows);
 return found.changed??found.pending.values().next().value;
}

'''
src = src[:a] + new_block + src[b:]
open(G + "🟦️.ts", "w", encoding="utf8").write(src)

edit("🟦️.ts", [
 ("""&&isSha(value.relevance?.id)&&Number.isSafeInteger(value.relevance?.rechecked)))return undefined;""", """&&isSha(value.relevance?.id)&&isSha(value.relevance?.stamps)&&Number.isSafeInteger(value.relevance?.rechecked)))return undefined;"""),
 ("""    const changed=firstRelevantChangeV1(relevance,answer.rows);if(changed)return miss("changed",changed[2]);
    let rechecked=0;""",
  """    const classified=classifyMemoChangesV1(relevance,answer.rows);if(classified.changed)return miss("changed",classified.changed[2]);
    let rechecked=0;
    if(classified.pending.size>0){
     const stampBytes=readBounded(named(directory,record.relevance.stamps,policy.stampSuffix),policy.maximumStampBytes),first=classified.pending.values().next().value!;
     if(!stampBytes||sha256(stampBytes)!==record.relevance.stamps)return miss("generation");
     if(!memoStampsHoldV1(root,stampBytes,classified.pending.keys(),insensitive))return miss("changed",first[2]);
     rechecked+=classified.pending.size;
    }"""),
 ("""     rechecked=recheck.rows.length+recheck.identities.length;""", """     rechecked+=recheck.rows.length+recheck.identities.length;"""),
 ("""relBytes=readBounded(stem+policy.relevanceSuffix,policy.maximumRelevanceBytes),checkBytes=readBounded(stem+policy.recheckSuffix,policy.maximumRecheckBytes);
     if(pointer&&pointer.generation===sealed.pointer.generation&&pointer.manifest===manifest&&relBytes&&checkBytes&&unpackMemoRelevanceV1(relBytes)){
      const id=sha256(Buffer.concat([relBytes,Buffer.from("\\n"),checkBytes]));
      if(!existsSync(named(directory,id,policy.relevanceSuffix))){atomic(named(directory,id,policy.recheckSuffix),checkBytes);atomic(named(directory,id,policy.relevanceSuffix),relBytes);}""",
  """relBytes=readBounded(stem+policy.relevanceSuffix,policy.maximumRelevanceBytes),checkBytes=readBounded(stem+policy.recheckSuffix,policy.maximumRecheckBytes),stampBytes=readBounded(stem+policy.stampSuffix,policy.maximumStampBytes);
     if(pointer&&pointer.generation===sealed.pointer.generation&&pointer.manifest===manifest&&relBytes&&checkBytes&&stampBytes&&unpackMemoRelevanceV1(relBytes)){
      const id=sha256(Buffer.concat([relBytes,Buffer.from("\\n"),checkBytes])),stamps=sha256(stampBytes);
      if(!existsSync(named(directory,stamps,policy.stampSuffix)))atomic(named(directory,stamps,policy.stampSuffix),stampBytes);
      if(!existsSync(named(directory,id,policy.relevanceSuffix))){atomic(named(directory,id,policy.recheckSuffix),checkBytes);atomic(named(directory,id,policy.relevanceSuffix),relBytes);}"""),
 ("""kept.plan.sha256===planSha&&kept.relevance.id===id))atomic(recordFile,JSON.stringify({...base,state:"sealed",pointer:{file:sealed.pointer.file,generation:sealed.pointer.generation,manifest},relevance:{id,rows:sealed.rows,rechecked:sealed.rechecked}}));""",
  """kept.plan.sha256===planSha&&kept.relevance.id===id&&kept.relevance.stamps===stamps))atomic(recordFile,JSON.stringify({...base,state:"sealed",pointer:{file:sealed.pointer.file,generation:sealed.pointer.generation,manifest},relevance:{id,stamps,rows:sealed.rows,rechecked:sealed.rechecked}}));"""),
 ("""const value=JSON.parse(readFileSync(join(directory,name),"utf8")) as {relevance?:{id?:string};plan?:{sha256?:string}};if(value.relevance?.id){alive.add(String(value.relevance.id)+policy.relevanceSuffix);alive.add(String(value.relevance.id)+policy.recheckSuffix);}""",
  """const value=JSON.parse(readFileSync(join(directory,name),"utf8")) as {relevance?:{id?:string;stamps?:string};plan?:{sha256?:string}};if(value.relevance?.id){alive.add(String(value.relevance.id)+policy.relevanceSuffix);alive.add(String(value.relevance.id)+policy.recheckSuffix);alive.add(String(value.relevance.stamps)+policy.stampSuffix);}"""),
])

edit("📤️relevance/🟦️.ts", [
 ("import {GRAPH_MEMO_POLICY_V1,memoDirectoriesV1,memoExcluderV1,memoLeavesV1,memoReachV1,packMemoRelevanceV1,", "import {GRAPH_MEMO_POLICY_V1,memoClassesV1,memoDirectoriesV1,memoExcluderV1,memoLeavesV1,memoReachV1,packMemoRelevanceV1,packMemoStampsV1,"),
 ("import {GENERATION_POLICY_V1,compareCodePointsV1,", "import {GENERATION_POLICY_V1,compareCodePointsV1,"),
 (" root:string;rows:()=>Iterable<MemoRowV1>;frontier:Readonly<{directories:Iterable<readonly [string,...unknown[]]>;members:Iterable<readonly [string,...unknown[]]>}>;",
  " root:string;rows:()=>Iterable<MemoRowV1>;frontier:Readonly<{directories:Iterable<readonly [string,...unknown[]]>;members:Iterable<readonly [string,...unknown[]]>;capturedAt?:number}>;"),
 ("""/** 🧱️ The relevance of one generation: the directories a change must not touch, the facts the journal cannot see, and why the generation cannot be sealed when it cannot. */
export type MemoBuiltV1=Readonly<{directories:ReadonlySet<string>;recheck:MemoRecheckV1;rows:number;linked:number;unsealable:string|null}>;""",
  """/** 🧱️ The relevance of one generation: the names a change must not touch (`directories`), the directories where every direct child row is relevant (`strict`), the directories a frontier stamp vouches for (`stamped`, with their `stamps`: modification time and inode), the facts the journal cannot see, and why the generation cannot be sealed when it cannot. */
export type MemoBuiltV1=Readonly<{directories:ReadonlySet<string>;strict:ReadonlySet<string>;stamped:ReadonlySet<string>;stamps:ReadonlyMap<string,readonly[number,number]>;recheck:MemoRecheckV1;rows:number;linked:number;unsealable:string|null}>;"""),
 ("none=(reason:string):MemoBuiltV1=>({directories:new Set(),recheck:", "none=(reason:string):MemoBuiltV1=>({directories:new Set(),strict:new Set(),stamped:new Set(),stamps:new Map(),recheck:"),
 ("const frontierDirectories=new Set<string>(),frontierMembers=new Set<string>();\n for(const [path] of input.frontier.directories)frontierDirectories.add(path===\".\"?\"\":path);",
  "const frontierDirectories=new Set<string>(),frontierMembers=new Set<string>(),stampable=new Map<string,readonly[number,number]>(),capturedAt=input.frontier.capturedAt;\n for(const [path,mtimeMs,ino] of input.frontier.directories){const key=path===\".\"?\"\":path;frontierDirectories.add(key);if(typeof capturedAt===\"number\"&&typeof mtimeMs===\"number\"&&typeof ino===\"number\"&&capturedAt-mtimeMs>=GENERATION_POLICY_V1.racyMilliseconds)stampable.set(key,[mtimeMs,ino]);}"),
 ("const vouched=new Set<string>(),recheckRows=", "const vouched=new Set<string>(),strict=new Set<string>(),listing=new Set<string>(),recheckRows="),
 ("   if(reach.recheck)defer(leaf);else for(const mention of reach.mentions)vouched.add(mention);",
  "   if(reach.recheck)defer(leaf);\n   else{const classes=memoClassesV1(leaf,reach);for(const mention of classes.names)vouched.add(mention);for(const mention of classes.strict)strict.add(mention);for(const mention of classes.listing)listing.add(mention);}"),
 (" memoDirectoriesV1(frontierDirectories,directories);\n", " memoDirectoriesV1(frontierDirectories,directories);\n for(const path of input.frontier.members)directories.add(path[0]);\n for(const path of listing)if(!stampable.has(path))strict.add(path);\n for(const path of frontierDirectories)if(!stampable.has(path))strict.add(path);\n const stamped=new Set<string>(),stamps=new Map<string,readonly[number,number]>();\n for(const [path,stamp] of stampable)if(!strict.has(path)){stamped.add(path);stamps.set(path,stamp);}\n"),
 (" return{directories,recheck:{rows:[...recheckRows.values()],identities:[...identities.values()]},rows:count,linked:linked.size,unsealable:null};", " return{directories,strict,stamped,stamps,recheck:{rows:[...recheckRows.values()],identities:[...identities.values()]},rows:count,linked:linked.size,unsealable:null};"),
 ("""/** 📤️ Writes the three files of a sidecar next to the plan file (`stem.json` header, `stem.rel` relevance, `stem.chk` recheck set). */""",
  """/** 📤️ Writes the four files of a sidecar next to the plan file (`stem.json` header, `stem.rel` relevance, `stem.chk` recheck set, `stem.lst` stamps of the stamped directories). */"""),
 ("directories:built.directories.size,linked:built.linked,unsealable:built.unsealable};", "directories:built.directories.size,strict:built.strict.size,stamped:built.stamped.size,linked:built.linked,unsealable:built.unsealable};"),
 ("atomic(stem+policy.relevanceSuffix,packMemoRelevanceV1(built.directories,insensitive));}", "atomic(stem+policy.stampSuffix,packMemoStampsV1(built.stamps,insensitive));atomic(stem+policy.relevanceSuffix,packMemoRelevanceV1(built.directories,insensitive,built.strict,built.stamped));}"),
 ("writeGraphMemoSidecarFilesV1(stem,header,{directories:new Set(),recheck:{rows:[],identities:[]},rows:0,linked:0,unsealable:reason},insensitive)", "writeGraphMemoSidecarFilesV1(stem,header,{directories:new Set(),strict:new Set(),stamped:new Set(),stamps:new Map(),recheck:{rows:[],identities:[]},rows:0,linked:0,unsealable:reason},insensitive)"),
])
print("edited sources", root[-30:])
