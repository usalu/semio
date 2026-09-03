#!/usr/bin/env python3
"""🔨️ F4 — duplicate fem2d/fem3d's per-subset cross-language mutation cases up into ✳️any, reusing
the SHARED capability (fem2d-1-mutate / fem3d-1-mutate) every subset already carries. Same
mechanism E3 proved on `sequence`. Drops the extra `@id-spec-vector` Outline (its fixtures
only resolve from the real owning subset — the escape guard blocks a sideways reach from
✳️any) since the coverage gate only requires mutate-/inverse- scenarios; that replay evidence stays
intact, undiminished, at the original subset-owned case. Ticket 26/09/02/SEPARATE-ARTIFACT-STANDARD-
SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION."""
import re
import shutil
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")


def duplicate(subsets_dir: Path, artifact_slug: str, subset: str, fixture_name: str):
    src_dir = subsets_dir / f"✳️{subset}" / "🧪️tests" / f"mutate-{artifact_slug}-1-{subset}"
    dst_dir = subsets_dir / "✳️any" / "🧪️tests" / f"mutate-{artifact_slug}-1-any-{subset}"
    dst_dir.mkdir(parents=True, exist_ok=True)
    note = (
        f"Duplicated (relative paths adjusted, the extra spec-vector-replay Outline dropped — its "
        f"committed-fixture references only resolve from the real owning subset, which the escape "
        f"guard blocks a ✳️any-owned case from reaching sideways into) from "
        f"`../../../✳️{subset}/🧪️tests/mutate-{artifact_slug}-1-{subset}/` by shard F4 (this ticket) "
        f"to close `unregistered-mutation-vocabulary` at the `✳️any/🧬️schema/🧬️mutations` owner — "
        f"same mechanism E3 already proved on `sequence`: reuse the already-manifested "
        f"`{artifact_slug}-1-mutate` capability, no new v2 manifest entry or runtime-inventory "
        f"coordinate. The dropped Outline's own replay evidence stays intact, undiminished, at the "
        f"original subset-owned case above — this duplicate only needs to satisfy the coverage "
        f"gate's mutate-<kind>/inverse-<kind> requirement."
    )

    # --- feature: strip the @id-spec-vector Outline (from that tag to EOF), adjust tags/paths ---
    feature_src = (src_dir / "🥒️.feature").read_text(encoding="utf-8")
    cut = feature_src.index("\n  @id-spec-vector")
    feature_dst = feature_src[:cut].rstrip("\n") + "\n"
    feature_dst = feature_dst.replace(
        f"@mutations-{artifact_slug}-1-{subset}", f"@mutations-{artifact_slug}-1-any-{subset}"
    )
    lines = feature_dst.split("\n")
    for i, line in enumerate(lines):
        if line.startswith("Feature:"):
            lines.insert(i + 1, f"  🧩️ {note}")
            lines.insert(i + 2, "")
            break
    feature_dst = "\n".join(lines)
    (dst_dir / "🥒️.feature").write_text(feature_dst, encoding="utf-8")

    # --- rust: adjust include_str! paths, drop the spec-vector registration line ---
    rust_src = (src_dir / "🦀️.rs").read_text(encoding="utf-8")
    rust_dst = rust_src.replace(
        "../../🧬️schema/🧬️mutations", f"../../../✳️{subset}/🧬️schema/🧬️mutations"
    )
    rust_dst = re.sub(
        r'\n\s*built = built\.subject\(&format!\("spec-vector-\{kind\}"\), subject::spec_vector\(kind\)\);',
        "",
        rust_dst,
    )
    rust_lines = rust_dst.split("\n")
    rust_lines.insert(1, f"//! {note}")
    rust_dst = "\n".join(rust_lines)
    (dst_dir / "🦀️.rs").write_text(rust_dst, encoding="utf-8")

    # --- python: copied verbatim, no path adjustments needed (fixture URIs come from the runner) ---
    shutil.copy2(src_dir / "🐍️.py", dst_dir / "🐍️.py")

    # --- fixture: local:// copy, verbatim bytes ---
    (dst_dir / "🧫️fixtures").mkdir(parents=True, exist_ok=True)
    shutil.copy2(src_dir / "🧫️fixtures" / fixture_name, dst_dir / "🧫️fixtures" / fixture_name)

    print(f"wrote {dst_dir}")


if __name__ == "__main__":
    FEM2D = ROOT / "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets"
    FEM3D = ROOT / "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets"
    for subset in ["mesh", "material", "boundary", "load", "analysis"]:
        duplicate(FEM2D, "fem2d", subset, "🏗️timber-portal-frame.snapshot.json")
    for subset in ["mesh", "material", "boundary", "load", "analysis"]:
        duplicate(FEM3D, "fem3d", subset, "🧊️steel-frame.snapshot.json")
