#!/usr/bin/env python3
"""🔨️ F4 — generic duplicator: copy a real subset's own no-oracle mutation case up into ✳️any,
reusing its already-manifested capability. Same mechanism E3 proved on `sequence`, reused here for
drawing/equation. Ticket 26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-
TEST-EVERY-MUTATION."""
import sys
from pathlib import Path

def duplicate(artifact_subsets: Path, prefix: str, subset: str, ticket_note: str):
    """prefix: the case's own <prefix>-1-<subset> stem, e.g. 'mutate-equation'."""
    src_dir = artifact_subsets / f"✳️{subset}" / "🧪️tests" / f"{prefix}-1-{subset}"
    dst_dir = artifact_subsets / "✳️any" / "🧪️tests" / f"{prefix}-1-any-{subset}"
    dst_dir.mkdir(parents=True, exist_ok=True)

    feature_src = (src_dir / "🥒️.feature").read_text(encoding="utf-8")
    feature_dst = feature_src.replace(
        f"@mutations-{prefix.split('mutate-')[-1]}-1-{subset}", f"@mutations-{prefix.split('mutate-')[-1]}-1-any-{subset}"
    ).replace(
        "../../../✳️any/🧪️oracle/🔣️.json", "../../🧪️oracle/🔣️.json"
    )
    lines = feature_dst.split("\n")
    for i, line in enumerate(lines):
        if line.startswith("Feature:"):
            lines.insert(i + 1, f"  🧩️ {ticket_note}")
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
    rust_lines = rust_dst.split("\n")
    rust_lines.insert(1, f"//! {ticket_note}")
    rust_dst = "\n".join(rust_lines)
    (dst_dir / "🦀️.rs").write_text(rust_dst, encoding="utf-8")

    print(f"wrote {dst_dir}")

if __name__ == "__main__":
    ROOT = Path("/Users/ueli/Documents/semio")
    EQUATION = ROOT / "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets"
    for subset in ["graph", "geometry", "equation"]:
        note = (
            f"Duplicated verbatim (only relative paths adjusted) from "
            f"`../../../✳️{subset}/🧪️tests/mutate-equation-1-{subset}/` by shard F4 (this ticket) to close "
            f"`unregistered-mutation-vocabulary` at the `✳️any/🧬️schema/🧬️mutations` + `✳️any/🚪️io/🧬️mutations` "
            f"owner — same mechanism E3 already proved on `sequence`: reuse the already-manifested "
            f"`equation-1-{subset}-mutate` capability, no new v2 manifest entry or runtime-inventory coordinate."
        )
        duplicate(EQUATION, "mutate-equation", subset, note)
