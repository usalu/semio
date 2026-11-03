"""v13 step 1: adds the stiffness basis next to every stored stiffness ratio in the en1998 JSON fixtures (idempotent, run with python -I).
A ratio of 0 was the old sentinel for the simplified method: it becomes basis `simplified` with ratio 1.0; any other number becomes basis `declared`.
Inside the diff files of the two ratio kinds the basis is a patch field (null)."""
import os, re

ART = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998"
KEYS = {"storeyStiffnessRatio": "storeyStiffnessBasis", "effectiveStiffnessRatio": "effectiveStiffnessBasis"}
PAT = re.compile(r'(?P<ws>[ \t]*|\n[ \t]*)"(?P<key>storeyStiffnessRatio|effectiveStiffnessRatio)"(?P<c>\s*:\s*)(?P<v>null|-?[0-9][0-9.eE+-]*)')
n_files = n_hits = 0
for dp, dn, fs in os.walk(ART):
    dn[:] = [d for d in dn if d not in ("target", "node_modules")]
    for f in fs:
        if not f.endswith(".json"):
            continue
        p = os.path.join(dp, f)
        t = open(p, encoding="utf-8").read()
        if "StiffnessRatio" not in t:
            continue
        patch_file = f.endswith("diff.json") and ("stiffness-ratio" in dp)

        def sub(m):
            global n_hits
            key, v, ws, c = m.group("key"), m.group("v"), m.group("ws"), m.group("c")
            before = t[max(0, m.start() - 80):m.start()]
            if KEYS[key] in before.replace(" ", "").replace("\n", "") and False:
                return m.group(0)
            n_hits += 1
            if v == "null" or patch_file:
                return '%s"%s"%snull,%s"%s"%s%s' % (ws, KEYS[key], c, ws, key, c, v)
            if float(v) == 0.0:
                return '%s"%s"%s"simplified",%s"%s"%s1.0' % (ws, KEYS[key], c, ws, key, c)
            return '%s"%s"%s"declared",%s"%s"%s%s' % (ws, KEYS[key], c, ws, key, c, v)

        if re.search(r'"(storeyStiffnessBasis|effectiveStiffnessBasis)"', t):
            continue
        new = PAT.sub(sub, t)
        if new != t:
            open(p, "w", encoding="utf-8").write(new)
            n_files += 1
print("files", n_files, "hits", n_hits)
