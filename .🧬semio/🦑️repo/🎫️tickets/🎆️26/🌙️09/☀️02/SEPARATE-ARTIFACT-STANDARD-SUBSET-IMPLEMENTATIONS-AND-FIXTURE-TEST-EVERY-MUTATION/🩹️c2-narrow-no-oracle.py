#!/usr/bin/env python3
# 🩹️ Narrows each flagged no-oracle decision's `capabilities` to remove exactly the capability that
# a real mutation manifest already requires — the same fix A10 applied to mathematical/sequence/draw.
# Never clears a decision's whole entry, never touches rationale text beyond appending one dated note.
import json
from collections import OrderedDict

NOTE = (
    "\n\n[C2 2026-09-02] Narrowed `capabilities` to remove {cap!r}: a mutation manifest already "
    "requires an external-oracle discharge for this capability (tracked honestly as "
    "`missing-external-oracle`), so a no-oracle decision claiming the same capability made that gap "
    "invisible to `no-oracle-covers-mutation` without discharging it. The rationale above is kept "
    "verbatim as the record of what was actually investigated; only the claim is withdrawn. See "
    "$TICKET/📓️c2-native-artifact-oracles.md."
)

targets = [
    ("✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "flow-widget-graph-mutation-semantics", "flow-1-mutate"),
    ("✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "vcs-1-checkpoint-mutation-semantics", "vcs-1-mutate"),
    ("✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "present-figure-deck-mutation-semantics", "present-1-mutate"),
    ("✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "shooting-render-scene-mutation-semantics", "shooting-1-mutate"),
    ("✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "playground-mutation-semantics", "playground-1-mutate"),
    ("✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "process3d-mutation-semantics", "process3d-1-mutate"),
    ("✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "wires-1-argument-board-mutation-semantics", "wires-1-mutate"),
    ("✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "layout-mutation-semantics", "layout-1-mutate"),
    ("✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "imperative-1-nested-step-list-mutation-semantics", "imperative-1-mutate"),
    ("✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "remodel-mutation-semantics", "remodel-1-mutate"),
    ("✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "energy-model-mutation-semantics", "energy-model-1-mutate"),
    ("✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "dag-1-port-directed-graph-mutation-semantics", "dag-1-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️.json", "frozen-hound-pcm16", "wav-riff-pcm-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️baseline/🧪️oracle/🔣️.json", "jpg-jfif-1-01-baseline-conformance-class-semantics", "jpg-jfif-1-01-baseline-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1018/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "dwg-ac1018-proprietary-container", "dwg-ac1018-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔖️ac1024/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "dwg-ac1024-proprietary-container", "dwg-ac1024-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️baseline/🧪️oracle/🔣️.json", "tiff-6-0-baseline-conformance-class-semantics", "tiff-6-0-baseline-mutate"),
    ("✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "s-home-mutation-semantics", "s-home-1-mutate"),
    ("✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "s-space-index-mutation-semantics", "s-space-1-mutate"),
    ("🧰️framework/🛍️products/💻️os/🎚️config/🧪️oracle/🔣️.json", "os-config-opening-preferences-mutation-semantics", "os-config-opening-1-mutate"),
    ("🧰️framework/🛍️products/💻️os/🎚️config/🧪️oracle/🔣️.json", "os-config-merge-policy-mutation-semantics", "os-config-merge-policy-1-mutate"),
    ("🧰️framework/🛍️products/💻️os/🎚️config/🧪️oracle/🔣️.json", "os-config-identity-mutation-semantics", "os-config-identity-1-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "txt-utf-8-line-structure", "txt-utf-8-mutate"),
    ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧪️oracle/🔣️.json", "raw-buffer-no-format", "binary-raw-mutate"),
]

by_file = OrderedDict()
for f, did, cap in targets:
    by_file.setdefault(f, []).append((did, cap))

changed = []
for f, entries in by_file.items():
    with open(f, encoding="utf-8") as fh:
        text = fh.read()
    d = json.loads(text)
    decisions = d.get("noOracleDecisions", [])
    for did, cap in entries:
        found = False
        for dec in decisions:
            if dec["id"] == did:
                found = True
                caps = dec.get("capabilities", [])
                if cap not in caps:
                    print(f"SKIP (already absent) {f} {did} {cap}")
                    continue
                caps = [c for c in caps if c != cap]
                dec["capabilities"] = caps
                dec["rationale"] = dec.get("rationale", "") + NOTE.format(cap=cap)
                changed.append((f, did, cap, caps))
        if not found:
            print(f"MISSING DECISION {f} {did}")
    with open(f, "w", encoding="utf-8") as fh:
        json.dump(d, fh, ensure_ascii=False, indent=2)
        fh.write("\n")

print(f"\nChanged {len(changed)} decisions across {len(by_file)} files:")
for f, did, cap, remaining in changed:
    print(f" - {did}: removed {cap!r}, remaining capabilities={remaining}")
