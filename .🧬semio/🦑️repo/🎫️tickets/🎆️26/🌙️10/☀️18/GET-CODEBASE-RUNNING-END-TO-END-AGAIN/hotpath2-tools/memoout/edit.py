"""📦 Edits of batch `memoout` on a root, starting from the live files: python3 -I edit.py <root>.
A listing fact of a directory a build writes its own results into (`Policy.outputs`, e.g. the git-ignored `.🧾️store`) is checked by the engine on every lookup (the recheck set) instead of being vouched for by the journal: the build's temp file, lock directory and rename leave the listing as it was, so the memo keeps serving; a real new or removed name makes the recorded listing stale and the memo refuses."""
import json, os, shutil, sys
root = sys.argv[1]
SUB = "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/"
G = root + SUB
live = os.path.dirname(os.path.abspath(__file__))
while not os.path.isdir(os.path.join(live, "🧰️framework")) and live != "/": live = os.path.dirname(live)
FILES = ("🧬️schema/🔣️.json", "🟦️.ts", "📤️relevance/🟦️.ts", "🧪️tests/🟦️.ts")
if os.path.abspath(live) != os.path.abspath(root):
    for rel in FILES:
        os.makedirs(os.path.dirname(G + rel), exist_ok=True)
        shutil.copyfile(live + SUB + rel, G + rel)
def edit(rel, pairs):
    path = G + rel
    s = open(path, encoding="utf8").read()
    for old, new in pairs:
        assert s.count(old) == 1, (rel, old[:90], s.count(old))
        s = s.replace(old, new)
    open(path, "w", encoding="utf8").write(s)

sp = G + "🧬️schema/🔣️.json"
d = json.load(open(sp, encoding="utf8"))
c = d["definitions"]["Policy"]["const"]
out = {}
for key, value in c.items():
    out[key] = value
    if key == "following": out["outputs"] = [".🧾️store"]
d["definitions"]["Policy"]["const"] = out
d["$comment"] += " Outputs: a directory named in `Policy.outputs` is one a build writes its own results into (the git-ignored `.🧾️store`: temp file, lock directory, rename on every build). Its listing facts are not vouched for by the journal but re-checked by the engine on every lookup (they join the recheck set), and the directory is neither strict nor stamped; a row that names an entry or an ancestor is still relevant, a row directly inside is settled by the recorded listing. Assumption declared with the list: the graph takes no project, member or entry from such a directory."
open(sp, "w", encoding="utf8").write(json.dumps(d, ensure_ascii=False, indent=2) + "\n")

edit("🟦️.ts", [("following:readonly string[];environmentPrefixes", "following:readonly string[];outputs:readonly string[];environmentPrefixes")])

edit("📤️relevance/🟦️.ts", [
 ("const vouched=new Set<string>(),strict=new Set<string>(),listing=new Set<string>(),recheckRows=", "const vouched=new Set<string>(),strict=new Set<string>(),listing=new Set<string>(),confirmed=new Set<string>(),recheckRows="),
 ("   if(reach.recheck)defer(leaf);\n   else{const classes=",
  "   if(reach.recheck)defer(leaf);\n   else if(policy.stamped.includes(leaf.kind)&&reach.mentions.length===1&&policy.outputs.includes(reach.mentions[0]!.slice(reach.mentions[0]!.lastIndexOf(\"/\")+1))){defer(leaf);confirmed.add(reach.mentions[0]!);}\n   else{const classes="),
 (" for(const path of input.frontier.members)directories.add(path[0]);\n", " for(const path of input.frontier.members)directories.add(path[0]);\n for(const path of confirmed)if(!vouched.has(path))directories.delete(path);\n"),
 (" for(const path of frontierDirectories)if(!stampable.has(path))strict.add(path);", " for(const path of frontierDirectories)if(!stampable.has(path)&&!confirmed.has(path))strict.add(path);"),
 (" for(const [path,stamp] of stampable)if(!strict.has(path)){stamped.add(path);stamps.set(path,stamp);}", " for(const [path,stamp] of stampable)if(!strict.has(path)&&!confirmed.has(path)){stamped.add(path);stamps.set(path,stamp);}"),
])

edit("🧪️tests/🟦️.ts", [
 ('"lib/extra.ts":"x\\n"};', '"lib/extra.ts":"x\\n","lib/.🧾️store/build.json":"{}\\n"};'),
 ('for(const directory of ["src/sub/deep","lib","docs","other","node_modules/pkg","dist"])', 'for(const directory of ["src/sub/deep","lib/.🧾️store","docs","other","node_modules/pkg","dist"])'),
 ("fs.readdirSync(at(\"src/sub/deep\"),{withFileTypes:true});", "fs.readdirSync(at(\"src/sub/deep\"),{withFileTypes:true});fs.readdirSync(at(\"lib/.🧾️store\"));"),
 ('const directories=[".","src","src/sub","src/sub/deep","lib"].map(', 'const directories=[".","src","src/sub","src/sub/deep","lib","lib/.🧾️store"].map('),
 (' {name:"a removal the journal reports only as modified",visible:true,advisory:"m",apply:root=>unlinkSync(join(root,"src/sub/extra.ts"))},\n];',
  ''' {name:"a removal the journal reports only as modified",visible:true,advisory:"m",apply:root=>unlinkSync(join(root,"src/sub/extra.ts"))},
 {name:"a build rewrites its output the way the store does: lock directory, temp file, rename over the old one",visible:true,apply:root=>{const store=join(root,"lib/.🧾️store");mkdirSync(join(store,"build.json.lock"));writeFileSync(join(store,"build.json.tmp"),"{\\"n\\":2}\\n");renameSync(join(store,"build.json.tmp"),join(store,"build.json"));rmSync(join(store,"build.json.lock"),{recursive:true});}},
 {name:"a build leaves a second output file in its output directory",visible:true,apply:root=>writeFileSync(join(root,"lib/.🧾️store/other.json"),"{}\\n")},
 {name:"a build removes its output file",visible:true,apply:root=>unlinkSync(join(root,"lib/.🧾️store/build.json"))},
];'''),
 ("  expect(built.stamped.size,mutation.name).toBeGreaterThanOrEqual(3);", "  expect(built.stamped.size,mutation.name).toBeGreaterThanOrEqual(3);\n  expect(built.recheck.rows.some(row=>row.kind===\"readdirNames\"&&row.path.endsWith(\"lib/.🧾️store\")),mutation.name+\": the listing of an output directory is re-checked by the engine\").toBe(true);\n  expect(built.stamped.has(\"lib/.🧾️store\")||built.strict.has(\"lib/.🧾️store\"),mutation.name+\": an output directory is neither stamped nor strict\").toBe(false);\n  expect(built.edits.inside.paths.includes(\"lib/.🧾️store\")||built.edits.named.paths.includes(\"lib/.🧾️store\"),mutation.name+\": the edit index holds nothing for an output directory (its listing is the engine's, every lookup)\").toBe(false);"),
 ("[0,1,2,3,4,5,6,7,8,9,10,12,14,15,17,18,20,21,22,23,24,25,26,27].includes(index)", "[0,1,2,3,4,5,6,7,8,9,10,12,14,15,17,18,20,21,22,23,24,25,26,27,30,31,32].includes(index)"),
 (" expect(vouched).toBeGreaterThanOrEqual(8);expect(both).toBeGreaterThanOrEqual(8);", " expect(vouched).toBeGreaterThanOrEqual(9);expect(both).toBeGreaterThanOrEqual(9);"),
 ("expect(settled,\"content edits", "expect(outputServed,\"a build rewriting its own output directory is served: the engine confirms the listing\").toBe(1);\n expect(settled,\"content edits"),
 ("let both=0,caught=0,lyingWrong=0,blindWrong=0,vouched=0,settled=0,skipWrong=0;", "let both=0,caught=0,lyingWrong=0,blindWrong=0,vouched=0,settled=0,skipWrong=0,outputServed=0;"),
 ("  if(memo)vouched++;", "  if(memo)vouched++;\n  if(memo&&mutation.name.startsWith(\"a build rewrites its output\"))outputServed++;"),
])
print("edited memoout", root[-30:])
