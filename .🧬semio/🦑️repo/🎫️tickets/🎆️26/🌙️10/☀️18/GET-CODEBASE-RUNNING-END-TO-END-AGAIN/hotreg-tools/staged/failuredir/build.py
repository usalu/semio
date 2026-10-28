"""🏗️ Rebuilds the staged batch `failuredir` (lane hotreg) from the CURRENT live files: `python3 -I build.py [--root DIR]` re-bases `base/` and `files/` and rewrites `manifest.json`; every edit is an exact one-time replacement and the build refuses a live file that no longer holds the expected text."""
import hashlib, json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else os.path.abspath(os.path.join(HERE, *([".."] * 10)))
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
REUSE = B + "/🔧️tools/🏗️bazel/♻️plan-reuse/🟦️.ts"
WITNESS = B + "/🔭️witness/🧬️schema/🔣️.json"
GUIDE = "🧰️framework/🛍️products/🦑️repo/📓️build-guide.md"
LAW = B + "/🔧️tools/🏗️bazel/♻️plan-reuse/🧪️tests/🫥️executor-owned/🟦️.ts"

EDITS = {
    REUSE: [
        (
            'import schema from "./🧬️schema/🔣️.json" with {type:"json"};\n',
            'import schema from "./🧬️schema/🔣️.json" with {type:"json"};\nimport failureTailSchema from "../🪵️failure-tail/🧬️schema/🔣️.json" with {type:"json"};\n',
        ),
        (
            'const policy=schema.definitions.Policy.const;\nconst cursorFile="plan.cursor.json";\n',
            'const policy=schema.definitions.Policy.const;\nconst cursorFile="plan.cursor.json";\nconst executorOwned:ReadonlySet<string>=new Set([failureTailSchema.definitions.Policy.const.directory]);\n',
        ),
        (
            '/** 🔍️ Names the entries the planner owns in the root of a workspace; everything the executor, Bazel or the record itself writes there is left out. */',
            '/** 🔍️ Names the entries the planner owns in the root of a workspace; everything the executor, Bazel or the record itself writes there is left out, among it the failure folder of the output tails (`failureTailSchema` directory), which every build empties before it starts and a failed action refills, so a plan recorded beside one must not call its removal a change of the plan. */',
        ),
        (
            ' return !policy.excluded.includes(name)&&!ignore.has(name)&&',
            ' return !policy.excluded.includes(name)&&!executorOwned.has(name)&&!ignore.has(name)&&',
        ),
        (
            '  try{facts=await lstat(join(input.workspace,row.name),{bigint:true});}catch{return{reuse:false,reason:"outputs"};}\n  if(identityOf(facts)!==row.identity)return{reuse:false,reason:"outputs"};',
            '  try{facts=await lstat(join(input.workspace,row.name),{bigint:true});}catch{return{reuse:false,reason:"outputs",subject:row.name};}\n  if(identityOf(facts)!==row.identity)return{reuse:false,reason:"outputs",subject:row.name};',
        ),
    ],
    WITNESS: [
        ('"outputs": "the generated workspace changed",', '"outputs": "the generated workspace changed: {subject}",'),
        ('"outputs": "der erzeugte Arbeitsbereich wurde geändert",', '"outputs": "der erzeugte Arbeitsbereich wurde geändert: {subject}",'),
    ],
    GUIDE: [
        (
            "`the generated workspace changed`, `the earlier plan record is unreadable`: the record of the slot was damaged.",
            "`the generated workspace changed: <file>`: a file the planner wrote into the package of the slot was replaced or removed since the record (a build never touches them; the folder `failures` of the output tails is no part of the record, a build empties it before it starts). `the earlier plan record is unreadable`: the record of the slot was damaged.",
        ),
        (
            "`the generated workspace changed`, `the earlier plan record is unreadable`: der Eintrag des Slots war beschädigt.",
            "`the generated workspace changed: <Datei>`: eine Datei, die der Planer in das Paket des Slots schrieb, wurde seit dem Eintrag ersetzt oder entfernt (ein Build berührt sie nie; der Ordner `failures` der Ausgabe-Enden gehört nicht zum Eintrag, jeder Build leert ihn vor dem Start). `the earlier plan record is unreadable`: der Eintrag des Slots war beschädigt.",
        ),
    ],
}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    for folder in ("base", "files"):
        shutil.rmtree(os.path.join(HERE, folder), ignore_errors=True)
        os.makedirs(os.path.join(HERE, folder))
    manifest, count = {}, 0
    for rel, edits in EDITS.items():
        live = open(os.path.join(ROOT, rel), "rb").read()
        text = live.decode("utf8")
        for old, new in edits:
            if text.count(old) != 1:
                sys.exit("expected exactly one occurrence in %s: %s" % (rel[-60:], old[:70]))
            text = text.replace(old, new)
        name = "f%03d" % count
        count += 1
        open(os.path.join(HERE, "base", name), "wb").write(live)
        open(os.path.join(HERE, "files", name), "wb").write(text.encode("utf8"))
        manifest[rel] = {"file": name, "base": sha(live), "new": sha(text.encode("utf8"))}
    law = open(os.path.join(HERE, "law.ts"), "rb").read()
    name = "f%03d" % count
    open(os.path.join(HERE, "files", name), "wb").write(law)
    manifest[LAW] = {"file": name, "base": None, "new": sha(law)}
    json.dump(manifest, open(os.path.join(HERE, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print("staged", len(manifest), "files")


main()
