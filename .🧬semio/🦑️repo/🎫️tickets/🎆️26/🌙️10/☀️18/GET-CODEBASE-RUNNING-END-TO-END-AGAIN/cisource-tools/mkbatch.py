#!/usr/bin/env python3
"""🧷 Builds the staged batch `cisrc` of lane cisource from the CURRENT live files: `python3 -I mkbatch.py [--root DIR] [--batch NAME]`.
Every edit is anchored (an anchor that is not found exactly once stops the build with exit 2), so the batch can be rebuilt on top of cicover's batch `ci1`, which edits other lines of the same fixture and schema."""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here: here = os.path.dirname(here)
    return here
ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(HERE)
BATCH = sys.argv[sys.argv.index("--batch") + 1] if "--batch" in sys.argv else "cisrc"
OUT = os.path.join(HERE, "staged", BATCH)
DASH = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🏗️ci/"
MANIFEST = "{projectRoot}/📋️project.json"
def sha(data): return hashlib.sha256(data).hexdigest()
def read(rel): return open(os.path.join(ROOT, rel), encoding="utf8").read()
def sub(text, old, new):
    found = text.count(old)
    if found != 1: sys.exit(f"anchor found {found} times, expected 1: {old[:100]!r}")
    return text.replace(old, new)
LAW = '''
test("dashboard CI Source gate declares every workflow and manifest file its test reads",()=>{
 const gate=cases.sourceGate,source=readFileSync(import.meta.path,"utf8"),target=JSON.parse(readFileSync(join(root(),gate.owner,"📋️project.json"),"utf8")).targets[gate.target];
 const reads=[...new Set([...[...source.matchAll(/"(\\.github\\/[\\w./-]+\\.ya?ml)"/gu)].map(match=>"{workspaceRoot}/"+match[1]),...[...source.matchAll(/gate\\.owner,"([^"/]+\\.json)"/gu)].map(match=>"{projectRoot}/"+match[1])])];
 expect(reads).toContain("{projectRoot}/📋️project.json");expect(reads.length).toBeGreaterThanOrEqual(5);
 for(const read of reads)expect(target.inputs,read).toContain(read);
 for(const input of target.inputs as string[])expect(existsSync(join(root(),input.replace("{workspaceRoot}/","").replace("{projectRoot}",gate.owner))),input).toBe(true);
});
'''
def fixture(t):
    return sub(t, '      "{workspaceRoot}/.github/workflows/build-system.yml"\n    ]', '      "{workspaceRoot}/.github/workflows/build-system.yml",\n      "' + MANIFEST + '"\n    ]')
def schema(t):
    t = sub(t, '''        "inputs": {
          "type": "array",
          "minItems": 9,
          "maxItems": 9,
          "uniqueItems": true,
          "items": {
            "type": "string",
            "pattern": "^\\\\{workspaceRoot\\\\}/",''', '''        "inputs": {
          "type": "array",
          "minItems": 10,
          "maxItems": 10,
          "uniqueItems": true,
          "contains": {
            "const": "''' + MANIFEST + '''"
          },
          "items": {
            "type": "string",
            "pattern": "^\\\\{(?:workspaceRoot|projectRoot)\\\\}/",''')
    return t
def law(t):
    t = sub(t, 'expect(validate({...cases,windowGate:{...cases.windowGate,command:"repo cargo test"}})).toBe(false);', 'expect(validate({...cases,windowGate:{...cases.windowGate,command:"repo cargo test"}})).toBe(false);\n expect(validate({...cases,sourceGate:{...cases.sourceGate,inputs:cases.sourceGate.inputs.slice(0,9)}})).toBe(false);')
    t = sub(t, ".toEqual({early:1,inputs:9});", ".toEqual({early:1,inputs:target.inputs.length});")
    if not t.endswith("\n"): t += "\n"
    return t + LAW
EDITS = [(DASH + "🔣️.json", fixture), (DASH + "🧬️schema/🔣️.json", schema), (DASH + "🟦️.ts", law)]
def main():
    shutil.rmtree(OUT, ignore_errors=True)
    os.makedirs(os.path.join(OUT, "files")); os.makedirs(os.path.join(OUT, "base"))
    manifest = {}
    for index, (rel, edit) in enumerate(EDITS):
        old = read(rel); new = edit(old); name = f"{index:02d}-" + os.path.basename(rel)
        open(os.path.join(OUT, "base", name), "w", encoding="utf8", newline="").write(old)
        open(os.path.join(OUT, "files", name), "w", encoding="utf8", newline="").write(new)
        manifest[rel] = {"file": name, "base": sha(old.encode()), "new": sha(new.encode())}
    json.dump(manifest, open(os.path.join(OUT, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    template = open(os.path.join(HERE, "..", "xplat5-tools", "stage.template.py"), encoding="utf8").read()
    head, _doc, tail = template.split('"""', 2)
    doc = '''🧷 Staged batch `cisrc` (lane cisource): the dashboard CI Source gate follows its own manifest input and gains a law that every workflow and manifest file the test reads is a declared input.
usage: python3 -I stage.py check|apply|revert [--root DIR]   (default root: walk up from this file to the directory holding 🧰️framework)
check : exit 0 when every file is `base` or `applied`, 1 otherwise (prints each state).
apply : every live file must hold its base hash or already the new one, else exit 2 (drift); atomic replace that keeps the mode of the file it replaces.
revert: every file must hold the new hash (or already base), else exit 2; restores base bytes.'''
    open(os.path.join(OUT, "stage.py"), "w", encoding="utf8").write(head + '"""' + doc + '"""' + tail)
    os.chmod(os.path.join(OUT, "stage.py"), 0o755)
    shutil.copy(os.path.join(HERE, "verify.sh"), os.path.join(OUT, "verify.sh"))
    print("built", OUT, len(manifest), "files")
main()
