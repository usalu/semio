#!/usr/bin/env python3
"""H53 round 2 of the warden tree: the subjects the warden classified (cargo, rustc, local units: they may be orphans of a dead command and name no workspace) are roots of the tree too, and the command is shortened to what tells commands apart (workspace root and bun runtime path collapsed)."""
import os, sys
ROOT = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/"
TS, LOOP = ROOT + "🟦️.ts", ROOT + "🔁️loop/🟦️.ts"
EDITS = {TS: [
 ('export function wardenTreeV1(rows:readonly HostWardenRowV1[],workspaces:readonly string[],limit:number=12):readonly HostWardenTreeRowV1[]{',
  'export function wardenTreeV1(rows:readonly HostWardenRowV1[],workspaces:readonly string[],limit:number=12,roots:readonly number[]=[]):readonly HostWardenTreeRowV1[]{'),
 ('const named=(row:HostWardenRowV1)=>workspaces.some(root=>root!==""&&row.command.includes(root));',
  'const rooted=new Set(roots),named=(row:HostWardenRowV1)=>rooted.has(row.pid)||workspaces.some(root=>root!==""&&row.command.includes(root));\n const short=(command:string)=>workspaces.reduce((text,root)=>root===""?text:text.split(root).join("~"),command).replace(/\\S*\\/runtimes\\/bun\\/[0-9a-f]+\\/bun /gu,"bun ");'),
 ('command:row.command.slice(0,240)}))', 'command:short(row.command).slice(0,240)}))'),
 ('a process counts when its command line names one of the `workspaces` or when it descends from one that does;', 'a process counts when its command line names one of the `workspaces`, when it is one of the `roots` (the subjects the warden classified, which may be orphans of a dead command) or when it descends from one that does; the workspace root shows as `~` and the bun runtime path as `bun` in the command;'),
], LOOP: [
 ('\n    lastTree=wardenTreeV1(rows,discoverWardenWorkspacesV1(rows,workspaces));', ''),
 ('    const stopped=new Map(rows.filter(row=>row.stopped).map(row=>[row.pid,row]));', '    lastTree=wardenTreeV1(rows,discoverWardenWorkspacesV1(rows,workspaces),12,found.map(subject=>subject.pid));\n    const stopped=new Map(rows.filter(row=>row.stopped).map(row=>[row.pid,row]));'),
]}
def build():
    out = {}
    for path, edits in EDITS.items():
        text = open(path, encoding="utf8").read()
        for old, new in edits:
            if text.count(old) != 1: sys.exit(f"anchor not unique/missing: {old[:80]!r} ({text.count(old)})")
            text = text.replace(old, new)
        out[path] = text
    return out
if __name__ == "__main__":
    out = build()
    for path, text in out.items():
        temporary = path + ".h53.tmp"; open(temporary, "w", encoding="utf8").write(text); os.replace(temporary, path)
    print("landed")
