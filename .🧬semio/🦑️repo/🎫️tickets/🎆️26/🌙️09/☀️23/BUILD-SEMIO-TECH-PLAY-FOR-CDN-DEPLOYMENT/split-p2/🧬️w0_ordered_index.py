#!/usr/bin/env python3
"""🔢️ Wave 0: moves `ordered_index` (the canonical key order every keyed snapshot collection keeps) from the mutation
dispatch module into the snapshot facet, so mutation leaves no longer reach up into the dispatch module (lane P2)."""
import os
REPO = os.path.abspath(os.path.join(os.path.dirname(os.path.abspath(__file__)), *[".."] * 8))
S = os.path.join(REPO, "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema")
MUT, SNAP = os.path.join(S, "🧬️mutations/🦀️.rs"), os.path.join(S, "📸️snapshot/🦀️.rs")
mut = open(MUT, encoding="utf-8").read()
start = mut.index("//#region 🔖️CanonicalOrder\n")
end = mut.index("//#endregion 🔖️CanonicalOrder\n") + len("//#endregion 🔖️CanonicalOrder\n")
block = mut[start:end]
mut = mut[:start] + mut[end:]
snap = open(SNAP, encoding="utf-8").read().rstrip("\n") + "\n\n" + block
open(MUT, "w", encoding="utf-8").write(mut)
open(SNAP, "w", encoding="utf-8").write(snap)
print("moved", block.count("\n"), "lines")
