#!/usr/bin/env python3
"""🙈️ ignoredenv-tools: a name the witness policy declares ignored is never an input, so its reads leave no `e` row either (before, a spread of process.env recorded every ignored name and any change replanned); SEMIO_BUILD_PARALLEL (the job count, already out of the key) and SEMIO_HOST_CAPACITY_PHASE_MARKS (per-run phase marks) join the list.
Usage: python3 -I edit.py <root> apply|check|revert. Anchored, all-or-nothing; backups under ignoredenv-tools/base/<n>."""
import os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
W = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness"
EDITS = [
 (W + "/🟨️.mjs", [("const observeEnvironment = (name, target) => { if (environmentWrites.has(name) || authority.has(name)) return;", "const observeEnvironment = (name, target) => { if (environmentWrites.has(name) || ignoredForDigest.has(name)) return;")]),
 (W + "/🧬️schema/🔣️.json", [("\"SEMIO_BUILD_INVOCATION_PID\", \"SEMIO_BUILD_REUSE_GRAPH\"]", "\"SEMIO_BUILD_INVOCATION_PID\", \"SEMIO_BUILD_REUSE_GRAPH\", \"SEMIO_BUILD_PARALLEL\", \"SEMIO_HOST_CAPACITY_PHASE_MARKS\"]")]),
 (W + "/🧪️tests/🟦️.ts", [("test(\"an allowed spawn and an explicit observation are recorded,",
   "test(\"a name the policy ignores is never an input: reading it (also through a spread of the environment) leaves no row, any other read does\",()=>{\n"
   " const{base,root}=world();\n"
   " try{\n"
   "  const witness=run(base,root,`const copy={...process.env};process.env.SEMIO_BUILD_PARALLEL;process.env.SEMIO_FIXTURE_VALUE;globalThis.__semioWorldWitnessV1.finish();`,{SEMIO_BUILD_PARALLEL:\"6\",SEMIO_LEASE_PRIORITY:\"batch\",SEMIO_FIXTURE_VALUE:\"a\"});\n"
   "  const names=new Set(witness.observations.filter(row=>row[0]===\"e\").map(row=>row[1]));\n"
   "  for(const name of[\"SEMIO_BUILD_PARALLEL\",\"SEMIO_LEASE_PRIORITY\",\"PWD\",\"SHLVL\",\"_\"])expect(names.has(name),name).toBe(false);\n"
   "  expect(names.has(\"SEMIO_FIXTURE_VALUE\")).toBe(true);\n"
   " }finally{rmSync(base,{recursive:true,force:true});}\n"
   "});\n\n"
   "test(\"an allowed spawn and an explicit observation are recorded,")]),
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
        for index, rel, path, _, _ in plans:
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
