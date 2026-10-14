#!/usr/bin/env python3
"""🧬️ Moves the 297 flat energy mutation leaves into the registered domain-operation layout (lane P1, slice S2a).

`🧬️mutations/<emoji><kind>` becomes `🧬️mutations/<domain>/<emoji><operation>` per `🧬️mutation-domains.json`, and the
fixture bundles follow (`🧪️tests/🧫️fixtures/🧬️mutations/<domain>/<operation>`). Updated with the move: each leaf
descriptor's `owner`, the leaf tests' fixture paths, the artifact root's mounts, the aggregate's `DIRECTORIES`, the
oracle catalog's directory names, the exhaustive feature's fixture directories, the Rust adapter's fixture paths and the
taxonomy's `mutationDomainOwners` registration. Plain filesystem moves (no git command).
usage: 🧬️mutation-domains.py plan|apply
"""
import json, os, re, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
ANY_REL = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
ANY = f"{REPO}/{ANY_REL}"
MUT = f"{ANY}/🧬️schema/🧬️mutations"; FIX = f"{ANY}/🧪️tests/🧫️fixtures/🧬️mutations"
ROOT_RS = f"{REPO}/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs"
TAX = f"{REPO}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
mode = sys.argv[1]
cfg = json.load(open(os.path.join(HERE, "🧬️mutation-domains.json"), encoding="utf-8"))
agg = open(f"{MUT}/🦀️.rs", encoding="utf-8").read()
old_dirs = dict(re.findall(r'\("([a-z0-9-]+)", "([^"]+)"\)', agg[agg.index("pub const DIRECTORIES"):]))
plan = []
for emoji, slug, nouns, kinds in cfg["domains"]:
    for k in kinds:
        if k in cfg["ops"]: op = cfg["ops"][k]
        else:
            verb, rest = k.split("-", 1); op = k
            for n in nouns:
                if rest == n: op = verb; break
                if rest.startswith(n + "-"): op = verb + "-" + rest[len(n) + 1:]; break
        em = cfg["emoji"].get(k) or re.match(r"^(.*?️)", old_dirs[k]).group(1)
        plan.append((k, old_dirs[k], emoji + slug, em + op))
assert len(plan) == 297 and len({p[0] for p in plan}) == 297
if mode == "plan":
    for row in plan: print(*row)
    sys.exit(0)
if not os.path.isdir(f"{MUT}/{plan[0][1]}"): sys.exit("already applied")
new_of = {old: f"{dom}/{op}" for k, old, dom, op in plan}
kind_of = {old: k for k, old, dom, op in plan}
for k, old, dom, op in plan:
    os.makedirs(f"{MUT}/{dom}", exist_ok=True); os.makedirs(f"{FIX}/{dom}", exist_ok=True)
    shutil.move(f"{MUT}/{old}", f"{MUT}/{dom}/{op}")
    if os.path.isdir(f"{FIX}/{old}"): shutil.move(f"{FIX}/{old}", f"{FIX}/{dom}/{op}")
    d = f"{MUT}/{dom}/{op}/🔣️.json"; t = open(d, encoding="utf-8").read()
    t2 = t.replace(f'"owner": "{ANY_REL}/🧬️schema/🧬️mutations/{old}"', f'"owner": "{ANY_REL}/🧬️schema/🧬️mutations/{dom}/{op}"')
    assert t2 != t, d; open(d, "w", encoding="utf-8").write(t2)
    for root, _, files in os.walk(f"{MUT}/{dom}/{op}/🧪️tests"):
        for f in files:
            if f.endswith(".rs"):
                p = os.path.join(root, f); s = open(p, encoding="utf-8").read()
                s2 = s.replace(f'"../../../../../🧪️tests/🧫️fixtures/🧬️mutations/{old}/', f'"../../../../../../🧪️tests/🧫️fixtures/🧬️mutations/{dom}/{op}/')
                open(p, "w", encoding="utf-8").write(s2)
s = open(ROOT_RS, encoding="utf-8").read()
for old, new in new_of.items():
    s = s.replace(f'"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{old}/', f'"🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/{new}/')
open(ROOT_RS, "w", encoding="utf-8").write(s)
for old, new in new_of.items():
    agg = agg.replace(f'("{kind_of[old]}", "{old}")', f'("{kind_of[old]}", "{new}")')
agg = agg.replace("/// 🗂️ `(semanticKind, leaf directory name)` for every declared kind", "/// 🗂️ `(semanticKind, domain/operation owner directory)` for every declared kind")
open(f"{MUT}/🦀️.rs", "w", encoding="utf-8").write(agg)
cat_p = f"{ANY}/🔮️oracles/🔣️.json"; cat = open(cat_p, encoding="utf-8").read()
for old, new in new_of.items():
    op = new.split("/")[1]
    cat = cat.replace(f'"sourceMutationDirectoryName": "{old}"', f'"sourceMutationDirectoryName": "{op}"').replace(f'"mutationDirectoryName": "{old}"', f'"mutationDirectoryName": "{op}"')
open(cat_p, "w", encoding="utf-8").write(cat)
feat_p = f"{ANY}/🧪️tests/🏛️mutate-energy-model-1/🥒️.feature"; feat = open(feat_p, encoding="utf-8").read()
for old, new in new_of.items():
    feat = feat.replace(f"| {old} |", f"| {new} |")
open(feat_p, "w", encoding="utf-8").write(feat)
for name in ("🦀️.rs", "🐍️.py"):
    p = f"{ANY}/🧪️tests/🏛️mutate-energy-model-1/{name}"; t = open(p, encoding="utf-8").read()
    for old, new in new_of.items():
        t = t.replace(f"🧫️fixtures/🧬️mutations/{old}/", f"🧫️fixtures/🧬️mutations/{new}/").replace(f"shared://🧬️mutations/{old}/", f"shared://🧬️mutations/{new}/")
    open(p, "w", encoding="utf-8").write(t)
tax = open(TAX, encoding="utf-8").read()
root_key = f"{ANY_REL}/🧬️schema/🧬️mutations"
assert f'"{root_key}"' not in tax
owners = {}
for k, old, dom, op in plan: owners.setdefault(dom, {})[op] = k
body = json.dumps(owners, ensure_ascii=False, indent=2).split("\n")
entry = f'    "{root_key}": ' + body[0] + "\n" + "\n".join("    " + line for line in body[1:]) + ",\n"
anchor = '  "mutationDomainOwners": {\n'; assert tax.count(anchor) == 1
tax = tax.replace(anchor, anchor + entry, 1)
json.loads(tax)
open(TAX, "w", encoding="utf-8").write(tax)
print("moved", len(plan))
