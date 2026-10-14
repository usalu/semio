#!/usr/bin/env python3
"""🏭️ Writes the standard cut config of one stdio format artifact. Every owning subset (one with its own schema
snapshot) becomes a family: `descriptor`, `inferences` and `mutations` (each with its text/binary codecs), `model`
(schema, snapshot, diff with their text, binary, SQLite and representation codecs) and the subset crate (io root,
import/export conversions, editor, viewer, examples). An extending subset (no snapshot of its own) is one crate.
Standard-level folders join the first owning subset's model; the artifact root stays the aggregator.

usage: 🏭️format-config.py <artifact dir (repo-rel)> <out dir> [layer=dir|regex ...]
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
artifact, out = sys.argv[1], sys.argv[2]
A = os.path.join(REPO, artifact)
fmt = re.sub(r"^[^a-z0-9]+", "", os.path.basename(artifact))
P = f"semio-s-artifact-stdio-{fmt}"
slug = lambda d: re.sub(r"^[^a-z0-9]+", "", d).replace(".", "-")
standards = sorted(d for d in os.listdir(os.path.join(A, "🏅️standards")) if os.path.isdir(os.path.join(A, "🏅️standards", d)))
crates, rules = {}, []
C = "(📝️text|💾️binary)"
for arg in sys.argv[3:]:
    key, rest = arg.split("=", 1)
    directory, pattern = rest.split("|", 1)
    crates[key] = {"package": f"{P}-{key}", "dir": f"{directory}/📦️packages/🦀️rust", "doc": f"//! 🧩️ The {fmt} {key} crate."}
    rules.append([pattern, key])
for std in standards:
    sdir = f"🏅️standards/{std}"
    subsets_dir = os.path.join(A, sdir, "🪆️subsets")
    subsets = sorted(d for d in os.listdir(subsets_dir) if os.path.isdir(os.path.join(subsets_dir, d)))
    owning = [s for s in subsets if os.path.exists(os.path.join(subsets_dir, s, "🧬️schema", "📸️snapshot"))]
    first_model = None
    for sub in sorted(subsets, key=lambda s: (s not in owning, s)):
        root = f"{sdir}/🪆️subsets/{sub}"
        parts = ([slug(std)] if len(standards) > 1 else []) + ([slug(sub)] if len(subsets) > 1 else [])
        base = "-".join(parts)
        name = lambda layer: "-".join([P] + parts + ([layer] if layer else [])) if parts or layer else P + "-subset"
        key = lambda layer: "-".join(parts + ([layer] if layer else [])) if parts else layer
        label = f"{fmt} {std.split(chr(0xfe0f))[-1]} {sub.split(chr(0xfe0f))[-1]}"
        extending_mutations = sub not in owning and os.path.isdir(os.path.join(A, root, "🧬️schema", "🧬️mutations"))
        if extending_mutations:
            crates[key("mutations")] = {"package": name("mutations"), "dir": f"{root}/🧬️schema/🧬️mutations/📦️packages/🦀️rust", "doc": f"//! 🧬️ The {label} mutation vocabulary with its text and binary codecs."}
            rules += [[f"^{root}/🧬️schema/", key("mutations")], [f"^{root}/🚪️io/{C}/🧬️mutations/", key("mutations")]]
        if sub in owning:
            for layer, d, doc, rs in [
                    ("descriptor", f"{root}/🧬️schema/📇️descriptor", f"//! 📇️ The {label} schema descriptor: every schema facet leaf, for the schema registry.", [f"^{root}/🧬️schema/📇️descriptor/"]),
                    ("inferences", f"{root}/🧬️schema/💡️inferences", f"//! 💡️ The {label} inferences with their text and binary codecs.", [f"^{root}/🧬️schema/💡️inferences/", f"^{root}/🚪️io/{C}/💡️inferences/"]),
                    ("mutations", f"{root}/🧬️schema/🧬️mutations", f"//! 🧬️ The {label} mutation vocabulary with its text and binary codecs.", [f"^{root}/🧬️schema/🧬️mutations/", f"^{root}/🚪️io/{C}/🧬️mutations/"]),
                    ("model", f"{root}/🧬️schema", f"//! 🧬️ The {label} model: schema, snapshot and diff with their text, binary and SQLite codecs.", [f"^{root}/🧬️schema/", f"^{root}/🚪️io/(📝️text|💾️binary|🪶️sqlite)/"])]:
                if os.path.isdir(os.path.join(A, d)):
                    crates[key(layer)] = {"package": name(layer), "dir": f"{d}/📦️packages/🦀️rust", "doc": doc}
                    rules += [[r, key(layer)] for r in rs]
            first_model = first_model or key("model")
        k = key("") if parts else "subset"
        crates[k] = {"package": name(""), "dir": f"{root}/📦️packages/🦀️rust", "doc": f"//! 🧩️ The {label} subset: composer, builder and analyzer, conversions, editor, viewer and examples."}
        rules.append([f"^{root}/", k])
    for extra in sorted(d for d in os.listdir(os.path.join(A, sdir)) if d != "🪆️subsets" and os.path.isdir(os.path.join(A, sdir, d))):
        if first_model:
            rules.insert(0, [f"^{sdir}/{re.escape(extra)}/", first_model])
rules.append([".", fmt])
too_long = [c["package"] for c in crates.values() if len(c["package"]) > 48]
if too_long:
    print("TOO LONG", too_long)
json.dump({"artifact": artifact, "manifest": artifact + "/📦️packages/🦀️rust/Cargo.toml", "layers": f"{fmt}-layers.json", "crate": P.replace("-", "_"), "aggregator": fmt, "crates": crates, "consumers": ["✏️s", "🌎️hub", "🧰️framework"]}, open(os.path.join(out, f"{fmt}-cut.json"), "w"), ensure_ascii=False, indent=1)
json.dump({"order": list(crates) + [fmt], "allow": {}, "rules": rules}, open(os.path.join(out, f"{fmt}-layers.json"), "w"), ensure_ascii=False, indent=1)
print(fmt, list(crates))
