#!/usr/bin/env python3
"""Rewrites io mutation leaves to name the schema leaf module directly: schema::mutations::Name -> schema::mutations::snake::Name."""
import re, sys, os, glob
VERSIONS = [("7️⃣1.7", "v1_7"), ("4️⃣1.4", "v1_4")]
SUBS = {"🧱️base": "base", "🖨️x": "x", "📐️e": "e", "♿️ua": "ua", "🧾️vt": "vt", "⚕️h": "h"}
apply = len(sys.argv) > 1 and sys.argv[1] == "apply"
changed = []
for vdir, vname in VERSIONS:
    ROOT = f"/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/{vdir}/🪆️subsets"
    for folder, sid in SUBS.items():
        aggp = f"{ROOT}/{folder}/🧬️schema/🧬️mutations/🦀️.rs"
        if not os.path.exists(aggp): continue
        mapping = {}
        for m in re.finditer(r"^pub use (\w+)::(\w+);", open(aggp).read(), re.M):
            mapping[m.group(2)] = m.group(1)
        pat = re.compile(r"(crate::standards::%s::subsets::%s::schema::mutations::)(\w+)\b(?!::)" % (vname, sid))
        for kind in ("💾️binary", "📝️text"):
            for f in sorted(glob.glob(f"{ROOT}/{folder}/🚪️io/{kind}/🧬️mutations/*/🦀️.rs")):
                text = open(f).read()
                new = pat.sub(lambda m: m.group(1) + mapping[m.group(2)] + "::" + m.group(2) if m.group(2) in mapping else m.group(0), text)
                if new != text:
                    changed.append(f)
                    if apply:
                        orig = f"{os.environ['F']}/r5b-tools/orig/" + f.split("standards/")[1].replace("/", "__")
                        if not os.path.exists(orig): open(orig, "w").write(text)
                        open(f, "w").write(new)
print(len(changed), "files", "applied" if apply else "dry-run")
