"""🧪 Test edits of batch `memolist` on a root, starting from the live files: python3 -I edit_tests.py <root>."""
import os, shutil, sys
root = sys.argv[1]
SUB = "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/"
G = root + SUB
live = os.path.dirname(os.path.abspath(__file__))
while not os.path.isdir(os.path.join(live, "🧰️framework")) and live != "/": live = os.path.dirname(live)
if os.path.abspath(live) != os.path.abspath(root):
    for rel in ("🧪️tests/🟦️.ts", "🧪️tests/🐍️.py"):
        shutil.copyfile(live + SUB + rel, G + rel)
def edit(rel, pairs):
    path = G + rel
    s = open(path, encoding="utf8").read()
    for old, new in pairs:
        assert s.count(old) == 1, (rel, old[:100], s.count(old))
        s = s.replace(old, new)
    open(path, "w", encoding="utf8").write(s)

edit("🧪️tests/🐍️.py", [
 ("a change is relevant when it names one of them or lies directly inside one, except that the modification of a directory matters only to the directories themselves.\"\"\"",
  "a change is relevant when it names one of them. Directories are told apart by what read them: a STAMPED directory (a frontier directory with a usable stamp that only entries and name/type listings of the policy's `stamped` kinds read) reacts to a row directly inside it that adds, removes or renames a name, and a row that only says 'modified' there is PENDING (the stamp must confirm it); a STRICT directory (named by a resolution or a derived fact, or its parent; read by a kind of the policy's `following`; listed but without a stamp) makes every row directly inside it relevant, except the modification of a directory; any other directory reacts only to rows that name an entry or an ancestor.\"\"\""),
 ("TABLE = json.load(open(SCHEMA, encoding=\"utf8\"))[\"definitions\"][\"Policy\"][\"const\"][\"relevance\"]",
  "POLICY = json.load(open(SCHEMA, encoding=\"utf8\"))[\"definitions\"][\"Policy\"][\"const\"]\nTABLE = POLICY[\"relevance\"]\nSTAMPED = set(POLICY[\"stamped\"])\nFOLLOWING = set(POLICY[\"following\"])"),
 ('''def relevant(dirs, change, insensitive):
    key = fold if insensitive else (lambda value: value)
    held = {key(item) for item in dirs}
    kind, directory, path = change
    parent = path.rsplit("/", 1)[0] if "/" in path else ""
    return key(path) in held or (not (kind == "m" and directory == 1) and key(parent) in held)
''','''def classes(kind, mentions):
    how = TABLE[kind]
    around = []
    for mention in mentions:
        around += [mention, mention.rsplit("/", 1)[0] if "/" in mention else ""]
    if how == "path" and kind in STAMPED:
        return mentions, [], mentions
    if how == "path" and kind in FOLLOWING:
        return mentions, mentions, []
    if how in ("resolve", "text"):
        return mentions, around, []
    return mentions, [], []


def decide(names, strict, stamped, change, insensitive):
    key = fold if insensitive else (lambda value: value)
    held, hard, stamps = ({key(item) for item in group} for group in (names, strict, stamped))
    kind, directory, path = change
    parent = path.rsplit("/", 1)[0] if "/" in path else ""
    if key(path) in held:
        return [True, None]
    if key(parent) in hard and not (kind == "m" and directory == 1):
        return [True, None]
    if key(parent) in stamps:
        return [True, None] if kind != "m" else [False, parent]
    return [False, None]
'''),
 ("    rows_out, vouched = [], []", "    rows_out, vouched, hard, listing = [], [], [], []"),
 ("            if result is not None and not result[\"recheck\"]:\n                vouched.extend(result[\"mentions\"])\n    dirs = directories(vouched)\n    return {\"rows\": rows_out, \"directories\": sorted(dirs), \"relevant\": [relevant(dirs, change, case[\"insensitive\"]) for change in case[\"changes\"]]}",
  "            if result is not None and not result[\"recheck\"]:\n                entries, strict, listed = classes(leaf[\"kind\"], result[\"mentions\"])\n                vouched.extend(entries)\n                hard.extend(strict)\n                listing.extend(listed)\n    dirs = directories(vouched)\n    stampable = set(case[\"stampable\"])\n    final = set(hard) | {item for item in listing if item not in stampable}\n    stamped = sorted(item for item in stampable if item not in final)\n    return {\"rows\": rows_out, \"directories\": sorted(dirs), \"strict\": sorted(final), \"stamped\": stamped, \"relevant\": [decide(dirs, final, stamped, change, case[\"insensitive\"]) for change in case[\"changes\"]]}"),
])

edit("🧪️tests/🟦️.ts", [
 ("  if(excluded(path))continue;\n  const was=before.get(path),now=after.get(path);\n  if(was===now)continue;", "  if(excluded(path))continue;\n  const was=before.get(path),now=after.get(path);\n  if(was===now)continue;\n  if(advisory&&(now??was)!.startsWith(\"d\"))continue;"),
 ("import {GRAPH_MEMO_POLICY_V1,collectGraphMemoV1,firstRelevantChangeV1,graphMemoEnabledV1,graphMemoKeyV1,memoDirectoriesV1,memoExcluderV1,memoIdentityHoldsV1,memoReachV1,openGraphMemoV1,packMemoRelevanceV1,",
  "import {GRAPH_MEMO_POLICY_V1,classifyMemoChangesV1,collectGraphMemoV1,firstRelevantChangeV1,graphMemoEnabledV1,graphMemoKeyV1,memoClassesV1,memoDirectoriesV1,memoExcluderV1,memoIdentityHoldsV1,memoReachV1,memoStampsHoldV1,openGraphMemoV1,packMemoRelevanceV1,packMemoStampsV1,"),
 ('"dist/out.js":"out\\n","dist/gen.ts":"gen\\n"};', '"dist/out.js":"out\\n","dist/gen.ts":"gen\\n","src/extra.ts":"x\\n","src/sub/extra.ts":"x\\n","src/sub/deep/extra.ts":"x\\n","lib/extra.ts":"x\\n"};'),
 ("fs.readdirSync(at(\"src\"));fs.readdirSync(at(\"src/sub\"));fs.realpathSync(at(\"src\"));", "fs.readdirSync(at(\"src\"));fs.readdirSync(at(\"src/sub\"));fs.readdirSync(at(\"src/sub/deep\"),{withFileTypes:true});fs.realpathSync(at(\"src\"));"),
 # frontier with real stamps
 (""" const directories=[".","src","src/sub","src/sub/deep","lib"].map(path=>[path,0,0] as [string,number,number]),members:[string,number][]=[];
 for(const directory of directories)for(const entry of readdirSync(join(root,directory[0]),{withFileTypes:true}))if(entry.isFile())members.push([directory[0]==="."?entry.name:directory[0]+"/"+entry.name,0]);
 return{directories,members};""",
  """ const directories=[".","src","src/sub","src/sub/deep","lib"].map(path=>{const status=lstatSync(join(root,path));return[path,status.mtimeMs,status.ino] as [string,number,number];}),members:[string,number][]=[];
 for(const directory of directories)for(const entry of readdirSync(join(root,directory[0]),{withFileTypes:true}))if(entry.isFile()&&/^(a|e)\\.ts$/u.test(entry.name))members.push([directory[0]==="."?entry.name:directory[0]+"/"+entry.name,0]);
 return{directories,members,capturedAt:Date.now()+60000};"""),
 ("function journalRows(before:Snapshot,after:Snapshot):ChangeRowV1[]{", "function journalRows(before:Snapshot,after:Snapshot,advisory?:\"m\"):ChangeRowV1[]{"),
 ("  rows.push([was===undefined?\"a\":now===undefined?\"r\":\"m\",(now??was)!.startsWith(\"d\")?1:0,path]);", "  rows.push([advisory??(was===undefined?\"a\":now===undefined?\"r\":\"m\"),(now??was)!.startsWith(\"d\")?1:0,path]);"),
 ("type Mutation={name:string;apply:(root:string,outside:string)=>void;visible:boolean};", "type Mutation={name:string;apply:(root:string,outside:string)=>void;visible:boolean;advisory?:\"m\"};"),
 (' {name:"nothing",visible:true,apply:()=>{}},\n];', ''' {name:"nothing",visible:true,apply:()=>{}},
 {name:"rewrite an unread sibling in a strict directory (a resolution names it)",visible:true,apply:root=>appendFileSync(join(root,"src/extra.ts"),"more\\n")},
 {name:"rewrite an unread sibling in a stamped directory",visible:true,apply:root=>appendFileSync(join(root,"src/sub/extra.ts"),"more\\n")},
 {name:"touch an unread sibling in a stamped directory",visible:true,apply:root=>{const now=new Date();utimesSync(join(root,"src/sub/extra.ts"),now,new Date(now.getTime()+9000));}},
 {name:"rewrite an unread sibling in a stamped directory no fact lists",visible:true,apply:root=>appendFileSync(join(root,"lib/extra.ts"),"more\\n")},
 {name:"remove and recreate an unread sibling in a stamped directory",visible:true,apply:root=>{unlinkSync(join(root,"src/sub/extra.ts"));writeFileSync(join(root,"src/sub/extra.ts"),"x\\n");}},
 {name:"replace an unread sibling by a directory (the typed listing changes)",visible:true,apply:root=>{unlinkSync(join(root,"src/sub/deep/extra.ts"));mkdirSync(join(root,"src/sub/deep/extra.ts"));}},
 {name:"a creation the journal reports only as modified (a coalesced create and remove)",visible:true,advisory:"m",apply:root=>writeFileSync(join(root,"src/sub/ghost.ts"),"g\\n")},
 {name:"a removal the journal reports only as modified",visible:true,advisory:"m",apply:root=>unlinkSync(join(root,"src/sub/extra.ts"))},
];'''),
 (" for(const how of Object.values(GRAPH_MEMO_POLICY_V1.relevance))expect([\"path\",\"analysis\",\"link\",\"resolve\",\"text\",\"none\"]).toContain(how);",
  " for(const how of Object.values(GRAPH_MEMO_POLICY_V1.relevance))expect([\"path\",\"analysis\",\"link\",\"resolve\",\"text\",\"none\"]).toContain(how);\n for(const kind of [...GRAPH_MEMO_POLICY_V1.stamped,...GRAPH_MEMO_POLICY_V1.following])expect(GRAPH_MEMO_POLICY_V1.relevance[kind],kind).toBe(\"path\");\n expect(GRAPH_MEMO_POLICY_V1.stamped.filter(kind=>GRAPH_MEMO_POLICY_V1.following.includes(kind))).toEqual([]);"),
 ("relevance:{id:\"c\".repeat(64),rows:10,rechecked:2}", "relevance:{id:\"c\".repeat(64),stamps:\"e\".repeat(64),rows:10,rechecked:2}"),
 ("directories:2,linked:0,unsealable:null}),JSON.stringify(header.errors)).toBe(true);\n expect(header({version:1,pointer:record.pointer,rows:1,rechecked:0,directories:2,linked:0})).toBe(false);",
  "directories:2,strict:1,stamped:1,linked:0,unsealable:null}),JSON.stringify(header.errors)).toBe(true);\n expect(header({version:1,pointer:record.pointer,rows:1,rechecked:0,directories:2,linked:0})).toBe(false);"),
 ("{kind:\"readdir\",path:root,fact:\"h\"},\n", "{kind:\"readdir\",path:root,fact:\"h\"},{kind:\"readdirNames\",path:root+\"/lib\",fact:\"h\"},{kind:\"directories\",path:root+\"/docs\",fact:\"h\"},{kind:\"readdir\",path:root+\"/other\",fact:\"h\"},\n"),
 ("const paths=[\"src/a.ts\",\"src\",\"\",\"src/sub/deep/x.ts\",\"docs/new.md\",\"docs\",\"lib\",\"lib/e.ts\",\"lib/other.ts\",\"other/f.ts\",\"Cargo.toml\",\"node_modules/x\",\"src/SUB\",\"LIB/e.ts\"]",
  "const paths=[\"src/a.ts\",\"src\",\"\",\"src/sub/deep/x.ts\",\"docs/new.md\",\"docs\",\"lib\",\"lib/e.ts\",\"lib/other.ts\",\"other/f.ts\",\"other/x.ts\",\"src/sub/extra.ts\",\"src/extra.ts\",\"Cargo.toml\",\"node_modules/x\",\"src/SUB\",\"LIB/e.ts\"],stampable=[\"\",\"lib\",\"docs\",\"other\",\"src/sub\",\"Cargo\"]"),
 ("const cases=[false,true].map(insensitive=>({root,excludedNames:[...NAMES],rows:[...rows,...synthetic],changes,insensitive}));", "const cases=[false,true].map(insensitive=>({root,excludedNames:[...NAMES],rows:[...rows,...synthetic],changes,insensitive,stampable}));"),
 ("model=JSON.parse(run.stdout) as {rows:({kind:string;mentions:string[];recheck:boolean}|null)[];directories:string[];relevant:boolean[]}[];", "model=JSON.parse(run.stdout) as {rows:({kind:string;mentions:string[];recheck:boolean}|null)[];directories:string[];strict:string[];stamped:string[];relevant:[boolean,string|null][]}[];"),
 ("  const vouched:string[]=[],mine:unknown[]=[];\n  for(const row of cases[index]!.rows)for(const leaf of leavesOf(row)){const reach=memoReachV1(leaf,root,excluded);mine.push(reach===undefined?null:{kind:leaf.kind,mentions:[...reach.mentions],recheck:reach.recheck});if(reach&&!reach.recheck)vouched.push(...reach.mentions);}",
  "  const vouched:string[]=[],hard=new Set<string>(),listing=new Set<string>(),mine:unknown[]=[];\n  for(const row of cases[index]!.rows)for(const leaf of leavesOf(row)){const reach=memoReachV1(leaf,root,excluded);mine.push(reach===undefined?null:{kind:leaf.kind,mentions:[...reach.mentions],recheck:reach.recheck});if(reach&&!reach.recheck){const found=memoClassesV1(leaf,reach);vouched.push(...found.names);for(const path of found.strict)hard.add(path);for(const path of found.listing)listing.add(path);}}"),
 ("  const relevance=unpackMemoRelevanceV1(packMemoRelevanceV1(directories,insensitive))!;\n  expect(changes.map(change=>firstRelevantChangeV1(relevance,[change])!==undefined)).toEqual(model[index]!.relevant);",
  "  const stampSet=new Set(stampable),final=new Set([...hard,...[...listing].filter(path=>!stampSet.has(path))]),stamped=new Set(stampable.filter(path=>!final.has(path)));\n  expect([...final].sort()).toEqual(model[index]!.strict);expect([...stamped].sort()).toEqual(model[index]!.stamped);\n  const relevance=unpackMemoRelevanceV1(packMemoRelevanceV1(directories,insensitive,final,stamped))!;\n  expect(changes.map(change=>{const found=classifyMemoChangesV1(relevance,[change]);return[found.changed!==undefined,found.pending.size?[...found.pending.keys()][0]!:null];})).toEqual(model[index]!.relevant);\n  expect(changes.map(change=>firstRelevantChangeV1(relevance,[change])!==undefined)).toEqual(model[index]!.relevant.map(([now,pending])=>now||pending!==null));"),
 ("let both=0,caught=0,lyingWrong=0,blindWrong=0,vouched=0;", "let both=0,caught=0,lyingWrong=0,blindWrong=0,vouched=0,settled=0,skipWrong=0;"),
 ("const before=snapshot(root);mutation.apply(root,outside);const after=snapshot(root),changes=journalRows(before,after);", "const before=snapshot(root);mutation.apply(root,outside);const after=snapshot(root),changes=journalRows(before,after,mutation.advisory);"),
 ("relevance=unpackMemoRelevanceV1(packMemoRelevanceV1(built.directories,process.platform===\"darwin\"||process.platform===\"win32\"))!;\n  const decide=async(rowsSeen:ChangeRowV1[],recheck:MemoRecheckV1):Promise<boolean>=>firstRelevantChangeV1(relevance,rowsSeen)===undefined&&recheck.identities.every(identity=>memoIdentityHoldsV1(root,identity))&&await hold(recheck.rows);\n  const memo=await decide(changes,built.recheck);",
  "insensitive=process.platform===\"darwin\"||process.platform===\"win32\",relevance=unpackMemoRelevanceV1(packMemoRelevanceV1(built.directories,insensitive,built.strict,built.stamped))!,stamps=packMemoStampsV1(built.stamps,insensitive);\n  expect(built.stamped.size,mutation.name).toBeGreaterThanOrEqual(3);\n  const decide=async(rowsSeen:ChangeRowV1[],recheck:MemoRecheckV1,confirm=true):Promise<boolean>=>{\n   const found=classifyMemoChangesV1(relevance,rowsSeen);if(found.changed)return false;\n   if(found.pending.size&&confirm&&!memoStampsHoldV1(root,stamps,found.pending.keys(),insensitive))return false;\n   return recheck.identities.every(identity=>memoIdentityHoldsV1(root,identity))&&await hold(recheck.rows);\n  };\n  const memo=await decide(changes,built.recheck);\n  if(classifyMemoChangesV1(relevance,changes).pending.size&&memo)settled++;\n  if(await decide(changes,built.recheck,false)&&!engine)skipWrong++;"),
 (" expect(vouched).toBeGreaterThanOrEqual(5);expect(both).toBeGreaterThanOrEqual(5);expect(caught).toBeGreaterThanOrEqual(12);", " expect(vouched).toBeGreaterThanOrEqual(8);expect(both).toBeGreaterThanOrEqual(8);expect(caught).toBeGreaterThanOrEqual(14);\n expect(settled,\"content edits of unread siblings in stamped directories are served after the stamp confirmed them\").toBeGreaterThanOrEqual(3);\n expect(skipWrong,\"a memo that trusts the advisory kind without the stamp must be wrong somewhere\").toBeGreaterThan(0);"),
 (" journal.push([\"m\",0,\"src/a.ts\"]);\n const changed=await (await open()).lookup();",
  """ const baseline=served.hit?served.rechecked:0;
 journal.push(["m",0,"src/sub/extra.ts"]);
 const settledHit=await (await open()).lookup();
 expect(settledHit.hit,"a content edit of an unread sibling in a stamped directory is settled by the stamp").toBe(true);
 if(settledHit.hit)expect(settledHit.rechecked).toBeGreaterThan(baseline);
 journal.push(["a",0,"src/sub/zzz.ts"]);
 expect(await (await open()).lookup(),"a new name in a stamped directory").toEqual({hit:false,reason:"changed",detail:"src/sub/zzz.ts"});
 journal.log.pop();journal.seq--;
 writeFileSync(join(root,"lib/zzz2.ts"),"z\\n");journal.push(["m",0,"lib/zzz2.ts"]);
 expect(await (await open()).lookup(),"an advisory 'modified' that hides a new name moved the stamp").toEqual({hit:false,reason:"changed",detail:"lib/zzz2.ts"});
 rmSync(join(root,"lib/zzz2.ts"));journal.log.pop();journal.seq--;
 journal.push(["m",0,"src/a.ts"]);
 const changed=await (await open()).lookup();"""),
 ("const planFile=join(directory,record.plan.sha256+\".plan\"),relFile=join(directory,record.relevance.id+\".rel\"),checkFile=join(directory,record.relevance.id+\".chk\");\n const saved=[planFile,relFile,checkFile].map(file=>readFileSync(file));",
  "const planFile=join(directory,record.plan.sha256+\".plan\"),relFile=join(directory,record.relevance.id+\".rel\"),checkFile=join(directory,record.relevance.id+\".chk\"),stampFile=join(directory,record.relevance.stamps+\".lst\");\n const saved=[planFile,relFile,checkFile,stampFile].map(file=>readFileSync(file));"),
 ("relevance:{id:string}};\n", "relevance:{id:string;stamps:string}};\n"),
 (" expect((await (await open()).lookup()).hit).toBe(true);\n rmSync(join(root,\"dist/out.js\"));",
  " expect((await (await open()).lookup()).hit).toBe(true);\n journal.push([\"m\",0,\"src/sub/extra.ts\"]);\n writeFileSync(stampFile,Buffer.concat([saved[3]!,Buffer.from([0])]));expect(await (await open()).lookup(),\"a tampered stamp table\").toEqual({hit:false,reason:\"generation\"});writeFileSync(stampFile,saved[3]!);\n rmSync(stampFile);expect(await (await open()).lookup(),\"a missing stamp table\").toEqual({hit:false,reason:\"generation\"});writeFileSync(stampFile,saved[3]!);\n expect((await (await open()).lookup()).hit,\"the stamp confirms again\").toBe(true);\n rmSync(join(root,\"dist/out.js\"));"),
 ("[0,1,2,3,4,5,6,7,8,9,10,12,14,15,17,18,20,21].includes(index)", "[0,1,2,3,4,5,6,7,8,9,10,12,14,15,17,18,20,21,22,23,24,25,26,27].includes(index)"),
])
print("edited tests", root[-30:])
