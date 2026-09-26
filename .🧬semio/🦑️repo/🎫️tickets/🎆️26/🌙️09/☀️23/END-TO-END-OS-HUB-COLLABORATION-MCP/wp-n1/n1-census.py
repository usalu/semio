"""N1 per-family census of the norm mutation contract layers (read-only)."""
import json, os, re, sys, unicodedata
ROOT = "/Users/ueli/Documents/semio"
NORM = ROOT + "/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
INV = ROOT + "/.🧬semio/🦑️repo/⚡️cache/tests/results/🏭️inventory"
EMOJI_RE = re.compile(r"^[^a-z0-9]+")
def kind_of(d): return EMOJI_RE.sub("", d)
rows = []
for fam in sorted(f for f in os.listdir(NORM) if os.path.isdir(f"{NORM}/{f}")):
    sub = f"{NORM}/{fam}/🏅️standards/🔖️1/🪆️subsets/✳️any"
    orc = f"{sub}/🔮️oracles/🔣️.json"
    if not os.path.exists(orc):
        print(f"== {fam}: NO oracles json; contents {os.listdir(f'{NORM}/{fam}')}"); continue
    d = json.load(open(orc))
    man = d.get("mutationManifests", [])
    M = [m["id"] for mm in man for m in mm["mutations"]]
    cats = d.get("mutationCatalogs", [])
    K = [k for c in cats for k in c.get("kinds", [])]
    V = {v["mutationId"]: v for c in cats for v in c.get("vectors", [])}
    leafroot = f"{sub}/🧬️schema/🧬️mutations"
    leaves = {}
    for e in sorted(os.listdir(leafroot)) if os.path.isdir(leafroot) else []:
        p = f"{leafroot}/{e}/🔣️.json"
        if os.path.isfile(p):
            try:
                k = json.load(open(p)).get("semanticKind")
                if k: leaves[k] = e
            except Exception as ex: leaves[f"BAD:{e}"] = e
    fx = f"{sub}/🧫️fixtures/🧬️mutations"
    fixdirs = {kind_of(e): e for e in os.listdir(fx)} if os.path.isdir(fx) else {}
    art = man[0]["artifact"] if man else "?"
    invp = f"{INV}/{art}@1@any.json"
    R = [m["id"] for m in json.load(open(invp))["mutations"]] if os.path.exists(invp) else None
    vec_missing_files = 0
    for mid, v in V.items():
        for s in v["scenarios"]:
            base = f"{fx}/{v['mutationDirectoryName']}/{s['directoryName']}"
            if not os.path.isdir(base): vec_missing_files += 1
    Ms, Ks, Ls, Vs, Fs = set(M), set(K), set(leaves), set(V), set(fixdirs)
    print(f"== {fam} {art}: manifest {len(M)} kinds {len(K)} leaves {len(leaves)} vectors {len(V)} (scenario dirs missing {vec_missing_files}) fixtureKinds {len(fixdirs)} runtime(stale) {None if R is None else len(R)}")
    print(f"   manifest-leaves: {sorted(Ms-Ls)[:8]} | leaves-manifest: {sorted(Ls-Ms)[:8]}")
    print(f"   kinds!=manifest: {sorted(Ks^Ms)[:8]}")
    print(f"   manifest w/o vector: {len(Ms-Vs)}; vectors not in manifest: {sorted(Vs-Ms)[:8]}")
    print(f"   fixture kinds not in manifest: {sorted(Fs-Ms)[:10]}")
