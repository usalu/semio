#!/usr/bin/env python3
"""🏭️ Writes the standard cut config of one semio subset. Trait impls stay with the types they implement for, so the
layers follow type ownership: one crate per `*-internals` schema folder; `descriptor` (the schema facet descriptor);
`model` (schema root, snapshot, diff and their text, binary and SQLite codecs); `inferences` and `mutations` (each with
its text and binary codecs); the subset aggregator keeps the io root (composer, derived construction), the editor,
viewer and examples.

usage: 🏭️subset-config.py <subset dir name, e.g. 🔢️value> [layer=path-regex ...]
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 9))
subset = sys.argv[1]
slug = re.sub(r"^[^a-z]+", "", subset)
A = f"✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/{subset}"
P = f"semio-s-artifact-stdio-semio-{slug}"
schema = os.path.join(REPO, A, "🧬️schema")
crates, rules = {}, []
for d in sorted(d for d in os.listdir(schema) if d.endswith("-internals")):
    key = re.sub(r"^[^a-z]+", "", d).removesuffix("-internals")
    crates[key] = {"package": f"{P}-{key}", "dir": f"🧬️schema/{d}/📦️packages/🦀️rust", "doc": f"//! {d[:d.index(chr(0xfe0f)) + 1] if chr(0xfe0f) in d else '🧮️'} The semio {slug} {key} internals."}
    rules.append([f"^🧬️schema/{re.escape(d)}/", key])
for arg in sys.argv[2:]:
    key, rest = arg.split("=", 1)
    directory, pattern = rest.split("|", 1)
    crates[key] = {"package": f"{P}-{key}", "dir": f"{directory}/📦️packages/🦀️rust", "doc": f"//! 🧩️ The semio {slug} {key} crate."}
    rules.append([pattern, key])
CODEC = "^🚪️io/(📝️text|💾️binary)/"
for key, d, doc, layer_rules in [
        ("descriptor", "🧬️schema/📇️descriptor", f"//! 📇️ The semio {slug} schema descriptor: every schema facet leaf of the subset, for the schema registry.", ["^🧬️schema/📇️descriptor/"]),
        ("inferences", "🧬️schema/💡️inferences", f"//! 💡️ The semio {slug} inferences derived from a snapshot, with their text and binary codecs.", ["^🧬️schema/💡️inferences/", CODEC + "💡️inferences/"]),
        ("mutations", "🧬️schema/🧬️mutations", f"//! 🧬️ The semio {slug} mutation vocabulary: every mutation with its diff and inverse, their text and binary codecs, and the member store owner.", ["^🧬️schema/🧬️mutations/", CODEC + "🧬️mutations/"]),
        ("model", "🧬️schema", f"//! 🧬️ The semio {slug} model: schema root, snapshot and diff with their text, binary and SQLite codecs, and the snapshot retirement implementations.", ["^🧬️schema/", "^🚪️io/(📝️text|💾️binary|🪶️sqlite)/"])]:
    if os.path.isdir(os.path.join(REPO, A, d)):
        crates[key] = {"package": f"{P}-{key}", "dir": f"{d}/📦️packages/🦀️rust", "doc": doc}
        rules += [[r, key] for r in layer_rules]
rules.append([".", slug])
config = {"artifact": A, "manifest": A + "/📦️packages/🦀️rust/Cargo.toml", "layers": f"{slug}-layers.json", "crate": P.replace("-", "_"), "aggregator": slug, "crates": crates,
          "move": [{"match": r"^semio_framework_value::artifact_retire_\w+!\([^;]*\);\n", "to": "model"}, {"match": r"^semio_s_artifact_stdio_semio_base::semio_snapshot_retirement!\([^;]*\);\n", "to": "model"}, {"match": r"^semio_s_artifact_stdio_semio_base::semio_member_owner!\([^;]*\);\n", "to": "mutations"}],
          "consumers": ["✏️s", "🌎️hub", "🧰️framework"]}
json.dump(config, open(os.path.join(HERE, f"{slug}-cut.json"), "w"), ensure_ascii=False, indent=1)
json.dump({"order": list(crates) + [slug], "allow": {}, "rules": rules}, open(os.path.join(HERE, f"{slug}-layers.json"), "w"), ensure_ascii=False, indent=1)
print(f"{slug}: {list(crates)}")
