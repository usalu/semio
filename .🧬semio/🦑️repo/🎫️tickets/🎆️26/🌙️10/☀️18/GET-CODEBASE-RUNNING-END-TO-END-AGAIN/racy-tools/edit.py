#!/usr/bin/env python3
"""🕰️ racy-tools: the world witness decides "racy" only for files it records (not for skipped infrastructure, owned output or probe files) and names the first such file.
Usage: python3 -I edit.py <root> apply|check|revert. Anchored edits, all-or-nothing; backups under racy-tools/base/<n> (revert restores them)."""
import os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
W, R = B + "/🔭️witness", B + "/🔧️tools/🏗️bazel/♻️plan-reuse"
EDITS = [
 (W + "/🟨️.mjs", [
  ("let racy = false, sealed", "let racy = null, sealed"),
  ("const observeResult = (kind, subject, outcome) => { const target = absolute(subject); if (target === undefined || skipped(target)) return; put(kind, target, outcome); };",
   "const observeResult = (kind, subject, outcome) => { const target = absolute(subject); if (target === undefined || skipped(target)) return; put(kind, target, typeof outcome === \"function\" ? outcome(target) : outcome); };"),
  ("observeResult(kind, target, error !== undefined ? failure(error) : value === undefined ? \"!ENOENT\" : identity(value)); }",
   "observeResult(kind, target, at => error !== undefined ? failure(error) : value === undefined ? \"!ENOENT\" : identity(value, at)); }"),
  ("const identity = (facts) => {", "const identity = (facts, target) => {"),
  ("if (Math.floor(latest) > Date.now() - config.racyMilliseconds) racy = true;", "if (racy === null && !sealed && Math.floor(latest) > Date.now() - config.racyMilliseconds) racy = target;"),
  ("try { put(kind, target, identity(original.statSync(target))); }", "try { put(kind, target, identity(original.statSync(target), target)); }"),
  ("typeof value.racy !== \"boolean\")", "(value.racy !== null && (typeof value.racy !== \"string\" || value.racy === \"\")))"),
 ]),
 (W + "/🟨️.d.mts", [("readonly racy: boolean;", "readonly racy: string | null;")]),
 (W + "/🧬️schema/🔣️.json", [
  ("\"racy\": { \"type\": \"boolean\" },", "\"racy\": { \"type\": [\"string\", \"null\"], \"minLength\": 1 },"),
  ("\"racy\": \"a file read by the planner was modified moments before\",", "\"racy\": \"{detail} was modified moments before the planner read it\","),
  ("\"racy\": \"eine vom Planer gelesene Datei wurde kurz davor geaendert\",", "\"racy\": \"{detail} wurde kurz vor dem Lesen durch den Planer geaendert\","),
 ]),
 (R + "/🟦️.ts", [("if(witness.racy)return{recorded:false,reason:\"racy\",detail:\"\"};", "if(witness.racy!==null)return{recorded:false,reason:\"racy\",detail:witness.racy};")]),
 (R + "/🧪️tests/🟦️.ts", [("expect(witness.racy).toBe(false);", "expect(witness.racy).toBeNull();")]),
 (W + "/🧪️tests/🟦️.ts", [
  ("expect(witness.tainted).toEqual([]);expect(witness.racy).toBe(false);", "expect(witness.tainted).toEqual([]);expect(witness.racy).toBeNull();"),
  ("  expect(racy.racy).toBe(true);\n", "  expect(racy.racy).toEndWith(\"/src/fresh.txt\");\n"),
  ("test(\"an allowed spawn and an explicit observation are recorded,",
   "test(\"a file the witness skips (infrastructure, the computation's own output) never makes it racy, however fresh\",()=>{\n"
   " const{base,root}=world();\n"
   " try{\n"
   "  const own=run(base,root,`import {writeFileSync,statSync,lstatSync,mkdirSync} from \"node:fs\";import {stat} from \"node:fs/promises\";const root=process.argv[2];writeFileSync(root+\"/.cache/fresh.txt\",\"x\");statSync(root+\"/.cache/fresh.txt\");lstatSync(root+\"/.cache/fresh.txt\");await stat(root+\"/.cache/fresh.txt\");mkdirSync(root+\"/../ws\",{recursive:true});writeFileSync(root+\"/../ws/made.txt\",\"y\");statSync(root+\"/../ws/made.txt\");globalThis.__semioWorldWitnessV1.finish();`,{},3000);\n"
   "  expect(own.tainted).toEqual([]);expect(own.racy).toBeNull();\n"
   " }finally{rmSync(base,{recursive:true,force:true});}\n"
   "});\n\n"
   "test(\"an allowed spawn and an explicit observation are recorded,"),
 ]),
]


def main():
    root, mode = sys.argv[1], sys.argv[2]
    plans, bad = [], []
    for index, (rel, pairs) in enumerate(EDITS):
        path = os.path.join(root, rel)
        text = open(path, encoding="utf8").read()
        new = text
        for old, replacement in pairs:
            if mode == "revert":
                continue
            count = new.count(old)
            if replacement in new:
                continue
            if count == 1:
                new = new.replace(old, replacement)
            else:
                bad.append("%s: anchor %r found %d times" % (rel, old[:60], count))
        plans.append((index, rel, path, text, new))
    if mode == "revert":
        for index, rel, path, text, _ in plans:
            backup = os.path.join(HERE, "base", str(index))
            if os.path.exists(backup):
                shutil.copy2(backup, path)
                print("restored", rel)
        return
    for line in bad:
        print("DRIFT", line)
    if bad:
        sys.exit(2)
    for index, rel, path, text, new in plans:
        print(("pending " if new != text else "applied ") + rel)
        if mode == "apply" and new != text:
            os.makedirs(os.path.join(HERE, "base"), exist_ok=True)
            shutil.copy2(path, os.path.join(HERE, "base", str(index)))
            open(path, "w", encoding="utf8").write(new)


main()
