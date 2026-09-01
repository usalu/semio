"""🫥️ Reclassify every oracle the `reimplementation-registered-as-third-party` gate flags.

The crate these entries name does real work — it parses and writes the format — but what a MUTATION
SHOULD PRODUCE is computed by the owner's own `🧪️oracle/🦀️component.rs`, which several of them state
outright ("a fresh, independent implementation", "read out of the BYTES by the independent
implementation", "deliberately mirrors" the production diff). Both halves of the comparison then descend
from one reading of the specification, so a misreading of it yields two agreeing wrong answers — the one
failure a differential test exists to prevent.

The honest split, and the reason this is a reclassification rather than a deletion: the crate discharges
"the result is a well-formed file of this format"; it does NOT discharge "the mutation computed the right
answer". Only the second is what a mutation oracle is for. Fixtures registered alongside are untouched —
they are real bytes, reproducible, and exactly what a qualifying reader gets pointed at when one exists.
"""
import json, os, re, subprocess, sys, io, collections

QUAL = {"third-party-library", "third-party-cli", "standards-reference-tool"}
SKIP = sys.argv[1:]  # owners to leave alone (an agent may be mid-write)

flagged = json.loads(subprocess.run(
    ["bun", "-e", """
const T=await import("./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/📦️index.ts");
const reg=T.loadOracleRegistry(process.cwd());
console.log(JSON.stringify((T.reimplementationOracleBreaches(process.cwd(),reg)).map(b=>b.scope.replace("/🧪️oracle/🦀️component.rs",""))));
"""], capture_output=True, text=True).stdout.strip().splitlines()[-1])

done = kinds = 0
for owner in sorted(set(flagged)):
    if any(s in owner for s in SKIP):
        print(f"  SKIPPED (agent live): {owner.split('/🗿️artifacts/')[-1][:40]}")
        continue
    path = os.path.join(owner, "🧪️oracle", "🔣️.json")
    if not os.path.exists(path):
        continue
    d = json.load(open(path), object_pairs_hook=collections.OrderedDict)
    caps = {c for m in d.get("mutationManifests", []) for mu in m["mutations"] for c in (r.get("capability") for r in mu.get("oracleRequirements", []))}
    if not caps:
        continue
    changed = 0
    for o in d.get("oracles", []):
        if o.get("kind") in QUAL and (set(o.get("capabilities") or []) & caps):
            hit = sorted(set(o["capabilities"]) & caps)
            o["kind"] = "cross-semio-implementation"
            o["capabilities"] = [c for c in o["capabilities"] if c not in hit] + [c + "-second-implementation" for c in hit]
            o["rationale"] = (
                "🫥️ RECLASSIFIED by the `reimplementation-registered-as-third-party` gate. The crate named here does "
                "real work — it parses and writes this format — but what a MUTATION SHOULD PRODUCE is computed by this "
                "owner's own `🧪️oracle/🦀️component.rs`. Both halves of the comparison then descend from ONE reading of "
                "the specification, so a misreading of it yields two agreeing wrong answers: the single failure a "
                "differential test exists to prevent.\n\n"
                "The honest split is that the crate discharges \"the result is a well-formed file of this format\" and "
                "NOT \"the mutation computed the right answer\". Only the second is what a mutation oracle is for, so "
                "this is a `cross-semio-implementation` — a required SUPPLEMENT, never a substitute, and a qualifying "
                "third-party reference is still owed.\n\n"
                "Any fixtures registered alongside are untouched and remain valuable: real bytes of the real format, "
                "byte-reproducible, and exactly what a qualifying reader gets pointed at the day one is registered."
            )
            changed += 1
    if not changed:
        continue
    for m in d.get("mutationManifests", []):
        kinds += len(m["mutations"])
    json.dump(d, open(path, "w"), indent=2, ensure_ascii=False)
    io.open(path, "a", encoding="utf-8").write("\n")
    done += 1
    print(f"  {owner.split('/🗿️artifacts/')[-1].replace('🏅️standards/','').replace('🪆️subsets/','')[:38]}")
print(f"reclassified {done} owner(s), {kinds} mutation kinds now report as owed a qualifying reference")
