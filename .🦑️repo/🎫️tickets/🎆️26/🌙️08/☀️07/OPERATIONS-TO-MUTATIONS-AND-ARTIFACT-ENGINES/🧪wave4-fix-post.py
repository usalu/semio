#!/usr/bin/env python3
"""Repair wave4 migrate-assigned fallout: glue doc order, grammar copy, slim op, imports."""
from __future__ import annotations

import re
import shutil
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
PLUGINS = ROOT / "✏️s/🔌️plugins"

GLUE_REORDER = [
    PLUGINS / "🪐️space/📦️packages/🦀️rust/📦️glue.rs",
    PLUGINS / "🔱️trinity/📦️packages/🦀️rust/📦️glue.rs",
    PLUGINS / "💡️reasoning/📦️packages/🦀️rust/📦️glue.rs",
]

OP_REEXPORT: dict[str, str] = {
    "🧩️puzzle/🗿️artifacts/◻2d/🔧️op/🦀️component.rs": """pub use crate::artifacts::puzzle2d::mutations::{
    apply_puzzle2d_mutation, inverse_puzzle2d_mutation, puzzle2d_document_delta_operations, Puzzle2dMutation,
    Puzzle2dPlayProjection,
};""",
    "🧩️puzzle/🗿️artifacts/🧊️3d/🔧️op/🦀️component.rs": """pub use crate::artifacts::puzzle3d::mutations::{
    apply_puzzle3d_mutation, inverse_puzzle3d_mutation, puzzle3d_document_delta_operations, Puzzle3dMutation,
    Puzzle3dPlayProjection,
};""",
    "🧩️puzzle/🗿️artifacts/🖐️5d/🔧️op/🦀️component.rs": """pub use crate::artifacts::puzzle5d::mutations::{
    apply_puzzle5d_mutation, inverse_puzzle5d_mutation, puzzle5d_document_delta_operations, Puzzle5dMutation,
    Puzzle5dPlayProjection,
};""",
    "🌀️procedural/🗿️artifacts/🌀️procedural2d/🔧️op/🦀️component.rs": """pub use crate::artifacts::procedural2d::mutations::{
    apply_procedural2d_mutation, inverse_procedural2d_mutation, procedural2d_fixture_operations, Procedural2dMutation,
};""",
    "🌀️procedural/🗿️artifacts/🧊️procedural3d/🔧️op/🦀️component.rs": """pub use crate::artifacts::procedural3d::mutations::{
    apply_procedural3d_mutation, inverse_procedural3d_mutation, procedural3d_fixture_operations, Procedural3dMutation,
};""",
}

MARKER = "//#endregion 🔖️HandcraftedOpCodecs"


def reorder_glue(path: Path) -> None:
    text = path.read_text(encoding="utf-8")
    if text.startswith("//!"):
        return
    m = re.match(r"((?:extern crate[^\n]+\n)+)", text)
    if not m:
        return
    prefix = m.group(1)
    rest = text[len(prefix) :]
    if not rest.startswith("//!"):
        return
    doc_end = rest.find("\n\n", rest.find("//!"))
    if doc_end == -1:
        return
    doc = rest[: doc_end + 2]
    body = rest[doc_end + 2 :]
    path.write_text(doc + prefix + body, encoding="utf-8")
    print("glue reorder", path.relative_to(ROOT))


def slim_op(rel: str, reexport: str) -> None:
    path = PLUGINS / rel
    if not path.is_file():
        return
    text = path.read_text(encoding="utf-8")
    idx = text.find(MARKER)
    if idx == -1:
        return
    head = text[: idx + len(MARKER)] + "\n\n" + reexport.strip() + "\n"
    path.write_text(head, encoding="utf-8")
    print("slim op", rel)


def copy_grammars() -> None:
    for mut_rs in PLUGINS.rglob("🧬️mutations/🦀️component.rs"):
        mut_dir = mut_rs.parent
        gram_mut = mut_dir / "📖️component.grammar.semio"
        if gram_mut.is_file():
            continue
        art = mut_dir.parent
        gram_op = art / "🔧️op/📖️component.grammar.semio"
        if gram_op.is_file():
            shutil.copy2(gram_op, gram_mut)
            print("grammar", gram_mut.relative_to(ROOT))


def fix_mutation_imports() -> None:
    for rs in PLUGINS.rglob("🧬️mutations/**/*.rs"):
        if "puzzle" not in str(rs) and "block" not in str(rs) and "procedural" not in str(rs):
            continue
        text = rs.read_text(encoding="utf-8")
        new = text.replace("protocol::Operation", "protocol::Mutation")
        new = new.replace("use protocol::{Operation,", "use protocol::{Mutation,")
        new = new.replace("{Operation, MutationDiff}", "{Mutation, MutationDiff}")
        if new != text:
            rs.write_text(new, encoding="utf-8")


def main() -> None:
    for p in GLUE_REORDER:
        reorder_glue(p)
    for rel, rex in OP_REEXPORT.items():
        slim_op(rel, rex)
    copy_grammars()
    fix_mutation_imports()
    block_glue = PLUGINS / "🧱️block/📦️packages/🦀️rust/📦️glue.rs"
    bg = block_glue.read_text(encoding="utf-8")
    if "extern crate semio_framework_os_kernel as vcs" not in bg:
        bg = bg.replace(
            "extern crate semio_framework_os_kernel as pack;",
            "extern crate semio_framework_os_kernel as pack;\nextern crate semio_framework_os_kernel as vcs;",
        )
    if "register_block_exports" not in bg:
        bg = bg.replace(
            "semio_framework_plugin::plugin_exports!(plugin::plugin);",
            "/// 🔌️ Registers block artifact codecs and pilot languages.\n"
            "pub fn register_block_exports() {\n"
            "    crate::artifacts::block2d::engine::register();\n"
            "    crate::artifacts::block3d::engine::register();\n"
            "    crate::artifacts::block5d::engine::register();\n"
            "}\n\n"
            "semio_framework_plugin::plugin_exports!(plugin::plugin);",
        )
        block_glue.write_text(bg, encoding="utf-8")
    sourcing_glue = PLUGINS / "🪵️sourcing/📦️packages/🦀️rust/📦️glue.rs"
    sg = sourcing_glue.read_text(encoding="utf-8")
    if "extern crate semio_framework_os_kernel as vcs" not in sg:
        sg = sg.replace(
            "extern crate semio_framework_os_kernel as pack;",
            "extern crate semio_framework_os_kernel as pack;\nextern crate semio_framework_os_kernel as vcs;",
        )
        sourcing_glue.write_text(sg, encoding="utf-8")
    curate = PLUGINS / "🪵️sourcing/🗿️artifacts/🗂️curate/🦀️component.rs"
    ct = curate.read_text(encoding="utf-8")
    if "pub type SourcingDocument" not in ct and "pub struct CurateDocument" in ct:
        ct = ct.replace(
            "pub struct CurateDocument {",
            "pub type SourcingDocument = CurateDocument;\n\npub struct CurateDocument {",
        )
        curate.write_text(ct, encoding="utf-8")
    print("DONE")


if __name__ == "__main__":
    main()
