#!/usr/bin/env python3
"""🤝 [farmid] Builds the staged batch `authority` from the CURRENT files of a root (default: live, walked up from here): a graph farm worker serves under the sealed record authority its parent sends (gate g44: `identity differs: records`). Anchored edits (each anchor exactly once) plus the new files in new/; writes files/, base/, manifest.json beside this script. Rebase = run it again on the moved tree.
usage: python3 -I make.py [--root DIR]"""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
F = B + "🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/"
O = B + "👁️observation/"
NEW = {
    F + "🧪️tests/🪆️authority/🟦️.ts": "test.ts",
    F + "🧪️tests/🧫️fixtures/🪆️parent-history/🟨️.mjs": "fixture-parent.mjs",
}
EDITS = {
    O + "🟨️.mjs": [
        (" return authorityDigest??=sha256HexV1(JSON.stringify([...loadObservations.values()].filter(row=>!(row.kind===\"file\"&&stampedFragments.some(fragment=>row.path.includes(fragment)))).map(row=>JSON.stringify([observationIdentityV1(row),observationMeaningV1(row)])).sort()));\n}\n",
         " return authorityDigest??=sha256HexV1(JSON.stringify([...loadObservations.values()].filter(row=>!(row.kind===\"file\"&&stampedFragments.some(fragment=>row.path.includes(fragment)))).map(row=>JSON.stringify([observationIdentityV1(row),observationMeaningV1(row)])).sort()));\n}\n\n"
         "/** 🫱️ Seals the module-load authority of this process and answers its rows, so a farm worker can serve under exactly this authority. */\n"
         "export function inferenceAuthorityRowsV1(){inferenceAuthorityDigestV1();return [...loadObservations.values()];}\n\n"
         "/** 🎁️ Serves under the sealed authority of a parent process: the parent's rows become the authority of this process (its records are then named as the parent's own records are, whatever the parent's command read before its first project), unless a row this process recorded at its own load says something else about the same fact; those conflicts are answered and nothing is adopted. */\n"
         "export function adoptInferenceAuthorityV1(rows){\n"
         " const parent=new Map(rows.map(row=>[JSON.stringify(observationIdentityV1(row)),observationMeaningV1(row)])),conflicts=[...loadObservations.values()].filter(row=>{const meaning=parent.get(JSON.stringify(observationIdentityV1(row)));return meaning!==undefined&&meaning!==observationMeaningV1(row);}).map(row=>row.kind+\":\"+row.path);\n"
         " if(conflicts.length)return conflicts;\n"
         " loadObservations.clear();retainObservationsV1(loadObservations,rows);loadSealed=true;authorityDigest=undefined;return [];\n"
         "}\n"),
    ],
    O + "🟨️.d.mts": [
        ("export function inferenceRecordIdentityV1():string;\n",
         "export function inferenceRecordIdentityV1():string;\n/** 🫱️ Seals the module-load authority of this process and answers its rows. */\nexport function inferenceAuthorityRowsV1():readonly {readonly kind:string;readonly path:string;readonly fact:unknown}[];\n/** 🎁️ Serves under a parent's sealed authority unless an own load row conflicts; answers the conflicts (`kind:path`), empty when adopted. */\nexport function adoptInferenceAuthorityV1(rows:readonly {readonly kind:string;readonly path:string;readonly fact:unknown}[]):string[];\n"),
    ],
    F + "🟨️.mjs": [
        ('import { inferenceRecordIdentityV1 } from "../../../../../🏗️build/👁️observation/🟨️.mjs";', 'import { inferenceAuthorityRowsV1, inferenceRecordIdentityV1 } from "../../../../../🏗️build/👁️observation/🟨️.mjs";'),
        ("componentRoots, stagingBase: base },", "componentRoots, stagingBase: base, authority: inferenceAuthorityRowsV1() },"),
        ('refusal = difference || undefined; return difference === ""; },', 'refusal = difference === "" ? undefined : difference + (Array.isArray(identity?.conflicts) && identity.conflicts.length ? " (" + identity.conflicts.join(", ") + ")" : ""); return difference === ""; },'),
        ("and serves only when it reports the captured implementation and the record identity of this process;",
         "and serves only when it reports the captured implementation and the record identity of this process, under the sealed load authority this process sends it (a worker never repeats what this command read before its first project);"),
    ],
    F + "👷️service/🟨️.mjs": [
        ('import { inferenceRecordIdentityV1, observeInferenceSnapshotV1 } from "../../../../../../🏗️build/👁️observation/🟨️.mjs";', 'import { adoptInferenceAuthorityV1, inferenceRecordIdentityV1, observeInferenceSnapshotV1 } from "../../../../../../🏗️build/👁️observation/🟨️.mjs";'),
        ("; answers the farm identity it serves under: the captured implementation it runs from and the identity its records are written under. */",
         "; serves under the sealed load authority its parent sent unless one of its own load rows conflicts with it, and answers the farm identity it serves under: the captured implementation it runs from, the identity its records are written under and the first conflicting rows. */"),
        ("  return graphFarmIdentityV1(inferenceRecordIdentityV1());\n}",
         "  const conflicts = Array.isArray(session.authority) ? adoptInferenceAuthorityV1(session.authority) : [];\n  return { ...graphFarmIdentityV1(inferenceRecordIdentityV1()), conflicts: conflicts.slice(0, 3) };\n}"),
    ],
}
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here: here = os.path.dirname(here)
    return here
def sha(data): return hashlib.sha256(data).hexdigest()
def main():
    root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(HERE)
    for name in ("files", "base"): shutil.rmtree(os.path.join(HERE, name), ignore_errors=True); os.makedirs(os.path.join(HERE, name))
    manifest, failed = {}, False
    def put(rel, base, new):
        flat = "%02d" % len(manifest) + "-" + os.path.basename(os.path.dirname(rel)) + "-" + os.path.basename(rel)
        open(os.path.join(HERE, "files", flat), "wb").write(new)
        if base is not None: open(os.path.join(HERE, "base", flat), "wb").write(base)
        manifest[rel] = {"file": flat, "base": None if base is None else sha(base), "new": sha(new)}
    for rel, edits in EDITS.items():
        base = open(os.path.join(root, rel), "rb").read(); text = base.decode("utf8")
        for old, new in edits:
            if text.count(old) != 1: print("ANCHOR", text.count(old), rel[-60:], repr(old[:80])); failed = True; continue
            text = text.replace(old, new)
        put(rel, base, text.encode("utf8"))
    for rel, source in NEW.items():
        if os.path.exists(os.path.join(root, rel)): print("EXISTS", rel); failed = True; continue
        put(rel, None, open(os.path.join(HERE, "new", source), "rb").read())
    if failed: sys.exit(2)
    json.dump(manifest, open(os.path.join(HERE, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    print("staged", len(manifest), "files from", root)
main()
