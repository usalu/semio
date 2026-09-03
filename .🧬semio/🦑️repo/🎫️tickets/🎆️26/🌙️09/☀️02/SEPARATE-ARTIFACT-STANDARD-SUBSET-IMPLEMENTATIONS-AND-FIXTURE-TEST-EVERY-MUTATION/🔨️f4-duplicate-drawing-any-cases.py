#!/usr/bin/env python3
"""🔨️ F4 — duplicate drawing's 4 per-subset no-oracle mutation cases up into ✳️any, reusing each
subset's own already-manifested capability, mirroring E3's sequence mechanism exactly. Ticket
26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION."""
import re
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
ARTIFACT = ROOT / "✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets"

SUBSETS = ["metadata", "structure", "style", "transform"]

for subset in SUBSETS:
    src_dir = ARTIFACT / f"✳️{subset}" / "🧪️tests" / f"mutate-drawing-1-{subset}"
    dst_dir = ARTIFACT / "✳️any" / "🧪️tests" / f"mutate-drawing-1-any-{subset}"
    dst_dir.mkdir(parents=True, exist_ok=True)

    feature_src = (src_dir / "🥒️.feature").read_text(encoding="utf-8")
    feature_dst = feature_src.replace(
        f"@mutations-drawing-1-{subset}", f"@mutations-drawing-1-any-{subset}"
    ).replace(
        "../../../✳️any/🧪️oracle/🔣️.json", "../../🧪️oracle/🔣️.json"
    )
    # Insert a provenance line right after the Feature: title line.
    lines = feature_dst.split("\n")
    for i, line in enumerate(lines):
        if line.startswith("Feature:"):
            lines.insert(i + 1, f"  🧩️ Duplicated from `../../../✳️{subset}/🧪️tests/mutate-drawing-1-{subset}/` (shard F4, this ticket) to close `unregistered-mutation-vocabulary` at the `✳️any/🧬️schema/🧬️mutations` + `✳️any/🚪️io/🧬️mutations` owner — same mechanism E3 already proved on `sequence`. Reuses the ALREADY-manifested `drawing-1-{subset}-mutate` capability, so no new v2 manifest entry or runtime-inventory coordinate is created.")
            lines.insert(i + 2, "")
            break
    feature_dst = "\n".join(lines)
    (dst_dir / "🥒️.feature").write_text(feature_dst, encoding="utf-8")

    rust_src = (src_dir / "🦀️.rs").read_text(encoding="utf-8")
    rust_dst = rust_src.replace(
        "../../🧬️schema/🧬️mutations", f"../../../✳️{subset}/🧬️schema/🧬️mutations"
    ).replace(
        "../../../✳️any/🧪️oracle/🔣️.json", "../../🧪️oracle/🔣️.json"
    )
    # Prepend a provenance doc-comment line right after the existing top-of-file //! block's first line.
    rust_lines = rust_dst.split("\n")
    insert_at = 1
    rust_lines.insert(insert_at, f"//! Duplicated verbatim (only relative paths adjusted) from `../../../✳️{subset}/🧪️tests/mutate-drawing-1-{subset}/🦀️.rs` by shard F4 (this ticket) to close `unregistered-mutation-vocabulary` at the `✳️any/🧬️schema/🧬️mutations` + `✳️any/🚪️io/🧬️mutations` owner — same mechanism E3 already proved on `sequence`: reuse the already-manifested capability, no new v2 manifest entry.")
    rust_dst = "\n".join(rust_lines)
    (dst_dir / "🦀️.rs").write_text(rust_dst, encoding="utf-8")

    print(f"wrote {dst_dir}")
