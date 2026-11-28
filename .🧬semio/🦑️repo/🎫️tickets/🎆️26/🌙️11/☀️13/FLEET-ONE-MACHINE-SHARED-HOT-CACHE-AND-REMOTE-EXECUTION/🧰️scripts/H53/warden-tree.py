#!/usr/bin/env python3
"""H53: the warden state names the heaviest processes of the repository (pid, parent, age, cpu, memory, command), so a worker that hangs can be read from the fleet store.
Usage: warden-tree.py stage <outdir> | warden-tree.py land"""
import hashlib, json, os, sys
ROOT = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/"
FILES = {"ts": ROOT + "🟦️.ts", "loop": ROOT + "🔁️loop/🟦️.ts", "schema": ROOT + "🧬️schema/🔣️.json"}
TYPE_ROW = '/** 🌲️ One process of the repository tree the warden published (schema `TreeRow`): the heaviest by CPU time, then memory, among the processes whose command line names a workspace root and everything below them; `megabytes` is what the kernel charges the process alone, `command` its first 240 characters. */\nexport type HostWardenTreeRowV1=Readonly<{pid:number;parent:number;elapsedSeconds:number;cpuSeconds?:number;megabytes:number;command:string}>;\n'
FUNCTION = '''
/** 🌲️ The heaviest processes of the repository for the state file: a process counts when its command line names one of the `workspaces` or when it descends from one that does; at most `limit` rows, the most CPU time first, then the most memory, then the lowest pid, so the same table gives the same rows. Pure.
 * @see ./🧪️tests/🌲️tree/🐍️.py */
export function wardenTreeV1(rows:readonly HostWardenRowV1[],workspaces:readonly string[],limit:number=12):readonly HostWardenTreeRowV1[]{
 const byPid=new Map(rows.map(row=>[row.pid,row])),inside=new Map<number,boolean>();
 const named=(row:HostWardenRowV1)=>workspaces.some(root=>root!==""&&row.command.includes(root));
 const decide=(row:HostWardenRowV1,seen:Set<number>):boolean=>{
  const known=inside.get(row.pid);if(known!==undefined)return known;
  if(seen.has(row.pid))return false;
  seen.add(row.pid);
  const parent=byPid.get(row.parent),value=named(row)||parent!==undefined&&parent.pid!==row.pid&&decide(parent,seen);
  inside.set(row.pid,value);return value;
 };
 return rows.filter(row=>decide(row,new Set())).map(row=>({pid:row.pid,parent:row.parent,elapsedSeconds:row.elapsedSeconds,...row.cpuSeconds===undefined?{}:{cpuSeconds:row.cpuSeconds},megabytes:Math.floor(row.memoryKilobytes/1024),command:row.command.slice(0,240)}))
  .sort((left,right)=>(right.cpuSeconds??-1)-(left.cpuSeconds??-1)||right.megabytes-left.megabytes||left.pid-right.pid).slice(0,limit);
}
'''
SCHEMA_ROW = '''    "TreeRow": {
      "description": "One process of the repository tree: the heaviest by CPU time, then memory.",
      "type": "object",
      "additionalProperties": false,
      "required": ["pid", "parent", "elapsedSeconds", "megabytes", "command"],
      "properties": {"pid": {"type": "integer", "minimum": 1}, "parent": {"type": "integer", "minimum": 0}, "elapsedSeconds": {"type": "integer", "minimum": 0}, "cpuSeconds": {"type": "number", "minimum": 0}, "megabytes": {"type": "integer", "minimum": 0}, "command": {"type": "string", "maxLength": 240}}
    },
'''
EDITS = [
 ("ts", 'lastAlarms:readonly HostWardenAlarmV1[];overhead:Readonly<{', 'lastAlarms:readonly HostWardenAlarmV1[];tree?:readonly HostWardenTreeRowV1[];overhead:Readonly<{'),
 ("ts", "export type HostWardenRowV1=Readonly<{", TYPE_ROW + "export type HostWardenRowV1=Readonly<{"),
 ("loop", 'discoverWardenWorkspacesV1,parseWardenSwapV1', 'discoverWardenWorkspacesV1,wardenTreeV1,parseWardenSwapV1'),
 ("loop", ' let last:{host:HostWardenHostV1;decision:', ' let lastTree:ReturnType<typeof wardenTreeV1>=[];\n let last:{host:HostWardenHostV1;decision:'),
 ("loop", 'lastAlarms:[...lastAlarms],overhead:', 'lastAlarms:[...lastAlarms],...lastTree.length>0?{tree:lastTree}:{},overhead:'),
 ("loop", 'const[host,rows]=await Promise.all([ports.sample(),ports.table()]),now=ports.now();', 'const[host,rows]=await Promise.all([ports.sample(),ports.table()]),now=ports.now();\n    lastTree=wardenTreeV1(rows,discoverWardenWorkspacesV1(rows,workspaces));'),
 ("schema", '        "trigger": {"anyOf": [{"type": "null"}, {"$ref": "#/definitions/Trigger"}]},\n        "others": {"type": "array", "items": {"$ref": "#/definitions/Other"}},\n        "suspended"', '        "trigger": {"anyOf": [{"type": "null"}, {"$ref": "#/definitions/Trigger"}]},\n        "others": {"type": "array", "items": {"$ref": "#/definitions/Other"}},\n        "tree": {"type": "array", "maxItems": 16, "items": {"$ref": "#/definitions/TreeRow"}},\n        "suspended"'),
 ("schema", '    "KindTotal": {', SCHEMA_ROW + '    "KindTotal": {'),
]
def read(path): return open(path, encoding="utf8").read()
def build():
    texts = {key: read(path) for key, path in FILES.items()}
    for key, old, new in EDITS:
        if texts[key].count(old) != 1: sys.exit(f"anchor not unique/missing in {key}: {old[:70]!r} ({texts[key].count(old)})")
        texts[key] = texts[key].replace(old, new)
    texts["ts"] = texts["ts"].rstrip("\n") + "\n" + FUNCTION
    json.loads(texts["schema"])
    return texts
def atomic(path, text):
    temporary = path + ".h53.tmp"
    open(temporary, "w", encoding="utf8").write(text)
    os.replace(temporary, path)
if __name__ == "__main__":
    texts = build()
    if sys.argv[1] == "stage":
        os.makedirs(sys.argv[2], exist_ok=True)
        for key, text in texts.items(): open(os.path.join(sys.argv[2], key + ".out"), "w", encoding="utf8").write(text)
        print("staged", {key: hashlib.sha256(text.encode()).hexdigest()[:12] for key, text in texts.items()})
    else:
        for key, path in FILES.items(): atomic(path, texts[key])
        print("landed")
