#!/usr/bin/env python3
"""Wave 4 final-six: install 🧬️mutations facets for present/sequence/layout/playbook/imperative/raster."""
from __future__ import annotations

import os
import re
from pathlib import Path

REPO = Path(__file__).resolve().parents[6]


def write(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


def triad(art: Path, emoji_kebab: str, art_mod: str, prefix: str, proj: str, apply_expr: str, mut_type: str) -> None:
    base = art / "🧬️mutations" / emoji_kebab
    for leaf in ("🦠️mutation", "🔺️diff", "↩️inverse"):
        (base / leaf).mkdir(parents=True, exist_ok=True)
    art_path = f"crate::artifacts::{art_mod}"
    write(
        base / "🦠️mutation" / "🦀️component.rs",
        f"""//! {emoji_kebab} `{mut_type}` apply leaf.
use {art_path}::{proj};
use {art_path}::mutations::{mut_type};

pub fn apply(projection: &mut {proj}, mutation: &{mut_type}) {{
    {apply_expr}
}}
""",
    )
    write(
        base / "↩️inverse" / "🦀️component.rs",
        f"""//! {emoji_kebab} `{mut_type}` inverse leaf.
use {art_path}::{proj};
use {art_path}::mutations::{mut_type};
use protocol::Mutation;

pub fn inverse(base: &{proj}, mutation: &{mut_type}) -> Vec<{mut_type}> {{
    <{mut_type} as Mutation<{proj}>>::inverse(mutation, base)
}}
""",
    )
    write(base / "🔺️diff" / "🦀️component.rs", "//! stub per-mutation diff leaf\n")
    write(base / "🦠️mutation" / "🟦️component.ts", "export {};\n")


def glue_leaves(art_folder: str, variants: list[tuple[str, str]]) -> str:
    parts = []
    for emoji_kebab, rust_mod in variants:
        parts.append(
            f"""            #[path = "."]
            pub mod {rust_mod} {{
                #[path = "../../🗿️artifacts/{art_folder}/🧬️mutations/{emoji_kebab}/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/{art_folder}/🧬️mutations/{emoji_kebab}/↩️inverse/🦀️component.rs"]
                pub mod inverse;
                #[path = "../../🗿️artifacts/{art_folder}/🧬️mutations/{emoji_kebab}/🔺️diff/🦀️component.rs"]
                pub mod diff;
            }}"""
        )
    return "\n".join(parts)


def insert_mutations_glue(glue_path: Path, art_folder: str, art_mod: str, variants: list[tuple[str, str]]) -> None:
    glue = glue_path.read_text(encoding="utf-8")
    if "pub mod mutations" in glue:
        print(f"glue already has mutations: {glue_path}")
        return
    block = f"""
        #[path = "."]
        pub mod mutations {{
            #[path = "../../🗿️artifacts/{art_folder}/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;
{glue_leaves(art_folder, variants)}
        }}
"""
    # Insert after `pub mod op;` inside the artifact module
    pattern = rf'(#\[path = "[^"]*{re.escape(art_folder)}/🔧️op/🦀️component\.rs"\]\s*pub mod op;)'
    m = re.search(pattern, glue)
    if not m:
        # fallback: after first pub mod op;
        pattern2 = r"(pub mod op;)"
        m = re.search(pattern2, glue)
        if not m:
            raise SystemExit(f"cannot find op module in {glue_path}")
        glue = glue[: m.end()] + "\n" + block + glue[m.end() :]
    else:
        glue = glue[: m.end()] + "\n" + block + glue[m.end() :]
    glue_path.write_text(glue, encoding="utf-8")


def ensure_ts_mutations(index_path: Path, export_name: str, art_folder: str) -> None:
    text = index_path.read_text(encoding="utf-8")
    line = f'export * as {export_name} from "../../🗿️artifacts/{art_folder}/🧬️mutations/🟦️component.ts";\n'
    if export_name in text:
        return
    # insert before *_op export if present
    op = f"export * as {export_name.replace('_mutations', '_op')}"
    if op in text:
        text = text.replace(op, line + op, 1)
    else:
        text = text.rstrip() + "\n" + line
    index_path.write_text(text, encoding="utf-8")


def append_artifact_engine(engine_path: Path, prefix: str, proj: str, art_mod: str, diff: str, apply_body: str) -> None:
    eng = engine_path.read_text(encoding="utf-8")
    if "ArtifactEngine" in eng:
        print(f"engine already has ArtifactEngine: {engine_path}")
        return
    block = f"""

//#region 🔖️ArtifactEngine
/// 🧬️ UI-independent document engine — every transition is a `{prefix}Mutation`.
pub struct {prefix}Engine {{
    projection: {proj},
}}

impl {prefix}Engine {{
    pub fn new(projection: {proj}) -> Self {{
        Self {{ projection }}
    }}

    pub fn into_projection(self) -> {proj} {{
        self.projection
    }}
}}

impl protocol::ArtifactEngine for {prefix}Engine {{
    type Projection = {proj};
    type Mutation = crate::artifacts::{art_mod}::mutations::{prefix}Mutation;
    type Diff = crate::artifacts::{art_mod}::diff::{diff};

    fn projection(&self) -> &Self::Projection {{
        &self.projection
    }}

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {{
        {apply_body}
    }}

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {{
        <Self::Mutation as protocol::Mutation<Self::Projection>>::inverse(mutation, &self.projection)
    }}
}}
//#endregion 🔖️ArtifactEngine
"""
    # Ensure proj type is in scope — engines usually already import their projection via crate paths in apply_body
    engine_path.write_text(eng.rstrip() + block + "\n", encoding="utf-8")


def slim_op(art: Path, art_mod: str, prefix: str, reexports: str, extra: str = "") -> None:
    op_path = art / "🔧️op" / "🦀️component.rs"
    content = f"""//! 🔧 {art_mod} artifact — OpText/OpBinary bridge for `{prefix}Mutation`.

pub use crate::artifacts::{art_mod}::mutations::{{{reexports}}};

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar
{extra}
"""
    write(op_path, content)


def fix_common_renames(root: Path) -> None:
    pairs = [
        ("protocol::Operation<", "protocol::Mutation<"),
        ("protocol::OperationDiff", "protocol::MutationDiff"),
        ("use protocol::{CollectionDiff, OperationDiff", "use protocol::{CollectionDiff, MutationDiff"),
        ("use protocol::{CollectionDiff, OperationDiff,", "use protocol::{CollectionDiff, MutationDiff,"),
        ("impl OperationDiff<", "impl MutationDiff<"),
        ("impl Operation<", "impl Mutation<"),
        ("vcs::apply_operation", "vcs::apply_mutation"),
        ("store::test_support::assert_operation_round_trip", "store::test_support::assert_mutation_round_trip"),
        ("unknown operation line", "unknown mutation line"),
        ("backwards_layout_operation", "inverse_layout_mutation"),
        ("sequence_fixture_operations", "sequence_fixture_mutations"),
        ("LayoutDiff {\n    pub operations:", "LayoutDiff {\n    pub mutations:"),
        ("self.operations.extend", "self.mutations.extend"),
        ("for operation in &self.operations", "for mutation in &self.mutations"),
        ("LayoutDiff { mutations: vec![self.clone()] }", "LayoutDiff { mutations: vec![self.clone()] }"),
    ]
    for path in root.rglob("*.rs"):
        text = path.read_text(encoding="utf-8")
        orig = text
        for a, b in pairs:
            text = text.replace(a, b)
        # layout diff field rename leftovers
        if "pub struct LayoutDiff" in text:
            text = text.replace("pub operations: Vec<LayoutMutation>", "pub mutations: Vec<LayoutMutation>")
            text = text.replace("self.operations", "self.mutations")
            text = text.replace("&self.operations", "&self.mutations")
            text = text.replace("for operation in &self.mutations", "for mutation in &self.mutations")
            text = text.replace("crate::artifacts::layout::op::apply_layout_mutation(&mut next, operation)", "crate::artifacts::layout::mutations::apply_layout_mutation(&mut next, mutation)")
            text = text.replace("use crate::artifacts::layout::op::LayoutMutation", "use crate::artifacts::layout::mutations::LayoutMutation")
        if "ImperativeDiff" in text and "use crate::artifacts::imperative::op::ImperativeMutation" in text:
            text = text.replace(
                "use crate::artifacts::imperative::op::ImperativeMutation",
                "use crate::artifacts::imperative::mutations::ImperativeMutation",
            )
        if text != orig:
            path.write_text(text, encoding="utf-8")


# ───────────────────────── PRESENT ─────────────────────────
def migrate_present() -> None:
    plugin = REPO / "✏️s/🔌️plugins/🎞️animate"
    art = plugin / "🗿️artifacts/🎬️present"
    op = (art / "🔧️op/🦀️component.rs").read_text(encoding="utf-8")

    # Extract mutation enum+impl region (Operations)
    m = re.search(r"//#region 🔖️Operations\n(.*?)//#endregion 🔖️Operations\n", op, re.S)
    if not m:
        raise SystemExit("present: Operations region missing")
    ops_body = m.group(1)
    ops_body = ops_body.replace("use protocol::{collection_diff_from_mutation, inverse_collection_mutation, CollectionMutation, Operation};", "")
    mut_rs = f"""//! 🧬️ present artifact — document mutation dispatch.

use crate::artifacts::present::diff::PresentDiff;
use crate::artifacts::present::{{FigureTileDraft, FigureTileDraftPatch, FigureTileSource, PresentDeck}};
use protocol::{{collection_diff_from_mutation, inverse_collection_mutation, CollectionMutation, Mutation}};
use serde::{{Deserialize, Serialize}};

//#region 🔖️Mutations
{ops_body}
/// ▶️ Applies `mutation` onto a deck clone via its diff.
pub fn apply_present_mutation(projection: &PresentDeck, mutation: &PresentMutation) -> PresentDeck {{
    mutation.diff(projection).apply(projection)
}}

pub fn inverse_present_mutation(projection: &PresentDeck, mutation: &PresentMutation) -> Vec<PresentMutation> {{
    mutation.inverse(projection)
}}
//#endregion 🔖️Mutations
"""
    write(art / "🧬️mutations/🦀️component.rs", mut_rs)
    write(art / "🧬️mutations/🟦️component.ts", "/** 🧩 present 🧬️mutations WASM facade. */\nexport {};\n")

    variants = [
        ("🎞tiles", "tiles"),
        ("📎set-source", "set_source"),
        ("📋set-tiles", "set_tiles"),
        ("🃏set-deck", "set_deck"),
    ]
    for ek, _ in variants:
        triad(art, ek, "present", "Present", "PresentDeck", "*projection = crate::artifacts::present::mutations::apply_present_mutation(projection, mutation);", "PresentMutation")

    # Rebuild op: keep DSL + codecs + tests, re-export mutation
    # Strip Operations region; fix imports
    new_op = op
    new_op = re.sub(r"//#region 🔖️Operations\n.*?//#endregion 🔖️Operations\n", "", new_op, flags=re.S)
    new_op = new_op.replace(
        "use crate::artifacts::present::diff::PresentDiff;\nuse crate::artifacts::present::{FigureTileDraft, FigureTileDraftPatch, FigureTileSource, PresentDeck};\nuse protocol::{collection_diff_from_mutation, inverse_collection_mutation, CollectionMutation, Operation};\nuse serde::{Deserialize, Serialize};\n",
        "use crate::artifacts::present::mutations::PresentMutation;\nuse crate::artifacts::present::{FigureTileDraft, FigureTileDraftPatch, FigureTileSource, PresentDeck};\nuse protocol::CollectionMutation;\n",
    )
    new_op = new_op.replace("//! 🔧️ Animate present artifact — operation enum + laws (constitutional: op).", "//! 🔧 present artifact — OpText/OpBinary for `PresentMutation`.")
    # Ensure re-export at top after grammar
    if "pub use crate::artifacts::present::mutations::" not in new_op:
        new_op = new_op.replace(
            "//#endregion 📖️SemioGrammar\n",
            "//#endregion 📖️SemioGrammar\n\npub use crate::artifacts::present::mutations::{apply_present_mutation, inverse_present_mutation, PresentMutation};\n",
            1,
        )
    # Add missing OpText for PresentMutation if absent
    if "impl protocol::OpText for PresentMutation {" not in new_op:
        new_op = new_op.replace(
            "impl protocol::OpBinary for PresentMutation {",
            """impl protocol::OpText for PresentMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        PresentMutationDsl::parse_op(line).map(Into::into)
    }
    fn print_op(&self) -> String {
        PresentMutationDsl::from(self).print_op()
    }
}

impl protocol::OpBinary for PresentMutation {""",
        )
    new_op = new_op.replace("vcs::apply_operation", "vcs::apply_mutation")
    new_op = new_op.replace("unknown operation line", "unknown mutation line")
    write(art / "🔧️op/🦀️component.rs", new_op)

    append_artifact_engine(
        art / "⚙️engine/🦀️component.rs",
        "Present",
        "crate::artifacts::present::PresentDeck",
        "present",
        "PresentDiff",
        """let diff = <Self::Mutation as protocol::Mutation<Self::Projection>>::diff(mutation, &self.projection);
        self.projection = crate::artifacts::present::mutations::apply_present_mutation(&self.projection, mutation);
        Ok(diff)""",
    )
    # PresentEngine needs PresentDeck type path only in struct - use full path; fix type alias in engine struct
    eng = (art / "⚙️engine/🦀️component.rs").read_text(encoding="utf-8")
    eng = eng.replace("projection: crate::artifacts::present::PresentDeck,", "projection: crate::artifacts::present::PresentDeck,")
    eng = eng.replace(
        "pub struct PresentEngine {\n    projection: crate::artifacts::present::PresentDeck,\n}",
        "pub struct PresentEngine {\n    projection: crate::artifacts::present::PresentDeck,\n}",
    )
    write(art / "⚙️engine/🦀️component.rs", eng)

    insert_mutations_glue(plugin / "📦️packages/🦀️rust/📦️glue.rs", "🎬️present", "present", variants)
    ensure_ts_mutations(plugin / "📦️packages/🟦️typescript/📦️index.ts", "present_mutations", "🎬️present")
    fix_common_renames(plugin)
    print("present ok")


# ───────────────────────── SEQUENCE ─────────────────────────
def migrate_sequence() -> None:
    plugin = REPO / "✏️s/🔌️plugins/🎬️sequence"
    art = plugin / "🗿️artifacts/🎬️sequence"
    op = (art / "🔧️op/🦀️component.rs").read_text(encoding="utf-8")

    # Move almost everything except grammar into mutations; keep slim op
    # Extract from Store region through end of Operations (before tests that need Op? tests stay in mutations)
    body = op
    # Remove grammar region from body copy for mutations
    mut = re.sub(r"//#region 📖️SemioGrammar\n.*?//#endregion 📖️SemioGrammar\n+", "", body, flags=re.S)
    mut = mut.replace("//! ⚡️ Sequence artifact — the operation type (constitutional: op).", "//! 🧬️ sequence artifact — document mutation dispatch.")
    mut = mut.replace(
        "use protocol::{collection_diff_from_mutation, inverse_collection_mutation, CollectionMutation, Operation};",
        "use protocol::{collection_diff_from_mutation, inverse_collection_mutation, CollectionMutation, Mutation};",
    )
    mut = mut.replace("//#region 🔖️Operations", "//#region 🔖️Mutations")
    mut = mut.replace("//#endregion 🔖️Operations", "//#endregion 🔖️Mutations")
    mut = mut.replace("sequence_fixture_operations", "sequence_fixture_mutations")
    mut = mut.replace("let mut operations = Vec::new();", "let mut mutations = Vec::new();")
    mut = mut.replace("operations.push", "mutations.push")
    mut = mut.replace("operations", "mutations")  # careful - may over-replace in comments
    # Fix over-eager rename in comments/docs that said "operations" conceptually — acceptable
    mut = mut.replace("impl Mutation<SequenceFixture>", "impl Mutation<SequenceFixture>")
    # Ensure Mutation is used (already)
    if "pub fn apply_sequence_mutation" not in mut:
        mut += """
/// ▶️ Applies `mutation` via its diff.
pub fn apply_sequence_mutation(projection: &SequenceFixture, mutation: &SequenceMutation) -> SequenceFixture {
    mutation.diff(projection).apply(projection)
}

pub fn inverse_sequence_mutation(projection: &SequenceFixture, mutation: &SequenceMutation) -> Vec<SequenceMutation> {
    mutation.inverse(projection)
}
"""
    write(art / "🧬️mutations/🦀️component.rs", mut)
    write(art / "🧬️mutations/🟦️component.ts", "/** 🧩 sequence 🧬️mutations WASM facade. */\nexport {};\n")

    variants = [
        ("➕steps-add", "steps_add"),
        ("➖steps-remove", "steps_remove"),
        ("↔️steps-move", "steps_move"),
        ("🩹steps-patch", "steps_patch"),
        ("➕edges-add", "edges_add"),
        ("➖edges-remove", "edges_remove"),
        ("↔️edges-move", "edges_move"),
        ("🩹edges-patch", "edges_patch"),
    ]
    for ek, _ in variants:
        triad(art, ek, "sequence", "Sequence", "SequenceFixture", "*projection = crate::artifacts::sequence::mutations::apply_sequence_mutation(projection, mutation);", "SequenceMutation")

    slim_op(art, "sequence", "Sequence", "apply_sequence_mutation, inverse_sequence_mutation, sequence_fixture_mutations, SequenceMutation, SequenceEnvelope, SequenceStore")

    append_artifact_engine(
        art / "⚙️engine/🦀️component.rs",
        "Sequence",
        "crate::artifacts::sequence::SequenceFixture",
        "sequence",
        "SequenceDiff",
        """let diff = <Self::Mutation as protocol::Mutation<Self::Projection>>::diff(mutation, &self.projection);
        self.projection = crate::artifacts::sequence::mutations::apply_sequence_mutation(&self.projection, mutation);
        Ok(diff)""",
    )
    insert_mutations_glue(plugin / "📦️packages/🦀️rust/📦️glue.rs", "🎬️sequence", "sequence", variants)
    ensure_ts_mutations(plugin / "📦️packages/🟦️typescript/📦️index.ts", "sequence_mutations", "🎬️sequence")
    # Update spr imports to mutations
    for path in (plugin).rglob("*.rs"):
        t = path.read_text(encoding="utf-8")
        n = t.replace("crate::artifacts::sequence::op::SequenceMutation", "crate::artifacts::sequence::mutations::SequenceMutation")
        n = n.replace("sequence_fixture_operations", "sequence_fixture_mutations")
        n = n.replace("protocol::OperationDiff", "protocol::MutationDiff")
        n = n.replace("use protocol::{CollectionDiff, OperationDiff", "use protocol::{CollectionDiff, MutationDiff")
        if n != t:
            path.write_text(n, encoding="utf-8")
    fix_common_renames(plugin)
    print("sequence ok")


# ───────────────────────── LAYOUT ─────────────────────────
def migrate_layout() -> None:
    plugin = REPO / "✏️s/🔌️plugins/📏️layout"
    art = plugin / "🗿️artifacts/📏️layout"
    op = (art / "🔧️op/🦀️component.rs").read_text(encoding="utf-8")

    mut = re.sub(r"//#region 📖️SemioGrammar\n.*?//#endregion 📖️SemioGrammar\n+", "", op, flags=re.S)
    mut = mut.replace("//! ⚡️ Layout artifact — the operation enum + laws (constitutional: op).", "//! 🧬️ layout artifact — document mutation dispatch.")
    mut = mut.replace(
        "use protocol::{apply_collection_mutation, inverse_collection_mutation, CollectionMutation, Operation};",
        "use protocol::{apply_collection_mutation, inverse_collection_mutation, CollectionMutation, Mutation};",
    )
    mut = mut.replace("backwards_layout_operation", "inverse_layout_mutation")
    mut = mut.replace("//#region 🔖️Operation", "//#region 🔖️Mutations")
    mut = mut.replace("//#endregion 🔖️Operation", "//#endregion 🔖️Mutations")
    # make apply_layout_mutation pub
    mut = mut.replace("pub(crate) fn apply_layout_mutation", "pub fn apply_layout_mutation")
    write(art / "🧬️mutations/🦀️component.rs", mut)
    write(art / "🧬️mutations/🟦️component.ts", "/** 🧩 layout 🧬️mutations WASM facade. */\nexport {};\n")

    variants = [
        ("📄pages", "pages"),
        ("📖stories", "stories"),
        ("🔗links", "links"),
        ("➕add-frame", "add_frame"),
        ("➖remove-frame", "remove_frame"),
        ("🩹patch-frame", "patch_frame"),
        ("🧾set-data-fields", "set_data_fields"),
    ]
    for ek, _ in variants:
        triad(
            art,
            ek,
            "layout",
            "Layout",
            "LayoutDocument",
            "crate::artifacts::layout::mutations::apply_layout_mutation(projection, mutation);",
            "LayoutMutation",
        )

    slim_op(art, "layout", "Layout", "apply_frame_patch, apply_layout_mutation, inverse_layout_mutation, LayoutMutation")

    append_artifact_engine(
        art / "⚙️engine/🦀️component.rs",
        "Layout",
        "crate::artifacts::layout::LayoutDocument",
        "layout",
        "LayoutDiff",
        """let diff = <Self::Mutation as protocol::Mutation<Self::Projection>>::diff(mutation, &self.projection);
        crate::artifacts::layout::mutations::apply_layout_mutation(&mut self.projection, mutation);
        Ok(diff)""",
    )
    insert_mutations_glue(plugin / "📦️packages/🦀️rust/📦️glue.rs", "📏️layout", "layout", variants)
    ensure_ts_mutations(plugin / "📦️packages/🟦️typescript/📦️index.ts", "layout_mutations", "📏️layout")

    # Fix diff field + imports
    diff_path = art / "🔺️diff/🦀️component.rs"
    d = diff_path.read_text(encoding="utf-8")
    d = d.replace("use crate::artifacts::layout::op::LayoutMutation;", "use crate::artifacts::layout::mutations::LayoutMutation;")
    d = d.replace("pub operations: Vec<LayoutMutation>", "pub mutations: Vec<LayoutMutation>")
    d = d.replace("self.operations", "self.mutations")
    d = d.replace("for operation in &self.mutations", "for mutation in &self.mutations")
    d = d.replace(
        "crate::artifacts::layout::op::apply_layout_mutation(&mut next, operation);",
        "crate::artifacts::layout::mutations::apply_layout_mutation(&mut next, mutation);",
    )
    d = d.replace(
        "crate::artifacts::layout::op::apply_layout_mutation(&mut next, mutation);",
        "crate::artifacts::layout::mutations::apply_layout_mutation(&mut next, mutation);",
    )
    # also handle if still `operation` var
    d = d.replace(
        "crate::artifacts::layout::mutations::apply_layout_mutation(&mut next, operation);",
        "crate::artifacts::layout::mutations::apply_layout_mutation(&mut next, mutation);",
    )
    if "for mutation in &self.mutations" not in d and "for operation in &self.mutations" in d:
        d = d.replace("for operation in &self.mutations", "for mutation in &self.mutations")
    # rewrite apply loop carefully
    d = re.sub(
        r"for \w+ in &self\.mutations \{\n\s*crate::artifacts::layout::(?:op|mutations)::apply_layout_mutation\(&mut next, \w+\);",
        "for mutation in &self.mutations {\n            crate::artifacts::layout::mutations::apply_layout_mutation(&mut next, mutation);",
        d,
    )
    diff_path.write_text(d, encoding="utf-8")

    for path in plugin.rglob("*.rs"):
        t = path.read_text(encoding="utf-8")
        n = t.replace("crate::artifacts::layout::op::LayoutMutation", "crate::artifacts::layout::mutations::LayoutMutation")
        n = n.replace("crate::artifacts::layout::op::apply_layout_mutation", "crate::artifacts::layout::mutations::apply_layout_mutation")
        n = n.replace("backwards_layout_operation", "inverse_layout_mutation")
        if n != t:
            path.write_text(n, encoding="utf-8")
    fix_common_renames(plugin)
    print("layout ok")


# ───────────────────────── PLAYBOOK ─────────────────────────
def migrate_playbook() -> None:
    plugin = REPO / "✏️s/🔌️plugins/📖️playbook"
    art = plugin / "🗿️artifacts/📖️playbook"

    mut = """//! 🧬️ playbook artifact — kernel `PlaybookMutation` facet.
pub use playbook::{
    add_block_operation, add_step_operation, apply_playbook_edit_operation as apply_playbook_edit_mutation,
    move_block_operation, move_step_operation, remove_block_operation, remove_step_operation,
    update_playbook_title_operation, PlaybookMutation,
};

use playbook::PlaybookSpec;
use protocol::Mutation;

pub fn inverse_playbook_mutation(spec: &PlaybookSpec, mutation: &PlaybookMutation) -> Vec<PlaybookMutation> {
    <PlaybookMutation as Mutation<PlaybookSpec>>::inverse(mutation, spec)
}
"""
    write(art / "🧬️mutations/🦀️component.rs", mut)
    write(art / "🧬️mutations/🟦️component.ts", "/** 🧩 playbook 🧬️mutations WASM facade. */\nexport {};\n")

    variants = [
        ("➕add-step", "add_step"),
        ("➖remove-step", "remove_step"),
        ("↔️move-step", "move_step"),
        ("➕add-block", "add_block"),
        ("➖remove-block", "remove_block"),
        ("↔️move-block", "move_block"),
        ("🩹update-block", "update_block"),
        ("🩹update-step", "update_step"),
        ("📖update-playbook", "update_playbook"),
    ]
    for ek, _ in variants:
        triad(
            art,
            ek,
            "playbook",
            "Playbook",
            "PlaybookSpec",
            "*projection = crate::artifacts::playbook::mutations::apply_playbook_edit_mutation(projection, mutation);",
            "PlaybookMutation",
        )

    # PlaybookSpec comes from playbook crate — triad uses crate::artifacts::playbook::PlaybookSpec
    # Ensure artifact root re-exports PlaybookSpec
    slim_op(
        art,
        "playbook",
        "Playbook",
        "add_block_operation, add_step_operation, apply_playbook_edit_mutation, move_block_operation, move_step_operation, remove_block_operation, remove_step_operation, update_playbook_title_operation, inverse_playbook_mutation, PlaybookMutation",
        extra="""
//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::playbook::engine::empty_playbook_projection;

    #[test]
    fn update_playbook_op_sets_title() {
        let spec = empty_playbook_projection();
        let mutation = PlaybookMutation::UpdatePlaybook { title: Some("Renamed".into()) };
        let next = apply_playbook_edit_mutation(&spec, &mutation);
        assert_eq!(next.title.as_deref(), Some("Renamed"));
    }

    #[test]
    fn apply_playbook_edit_op_roundtrip() {
        use crate::artifacts::playbook::PlaybookStep;

        let spec = empty_playbook_projection();
        let step = PlaybookStep { id: "step-test".into(), title: "Review".into(), description: None, blocks: Vec::new() };
        let next = apply_playbook_edit_mutation(&spec, &PlaybookMutation::AddStep { step, index: None });
        assert_eq!(next.steps.len(), 2);
    }
}
//#endregion 🧪️Tests
""",
    )

    append_artifact_engine(
        art / "⚙️engine/🦀️component.rs",
        "Playbook",
        "playbook::PlaybookSpec",
        "playbook",
        "PlaybookDiff",
        """let diff = <Self::Mutation as protocol::Mutation<Self::Projection>>::diff(mutation, &self.projection);
        self.projection = crate::artifacts::playbook::mutations::apply_playbook_edit_mutation(&self.projection, mutation);
        Ok(diff)""",
    )
    # Fix triad PlaybookSpec path — artifact may re-export it
    # Check artifact root
    root = (art / "🦀️component.rs").read_text(encoding="utf-8")
    if "PlaybookSpec" not in root:
        # triads use crate::artifacts::playbook::PlaybookSpec — need reexport
        pass

    insert_mutations_glue(plugin / "📦️packages/🦀️rust/📦️glue.rs", "📖️playbook", "playbook", variants)
    ensure_ts_mutations(plugin / "📦️packages/🟦️typescript/📦️index.ts", "playbook_mutations", "📖️playbook")

    # Fix triad type path: use playbook::PlaybookSpec via artifacts reexport if needed
    art_root = art / "🦀️component.rs"
    ar = art_root.read_text(encoding="utf-8")
    if "pub use playbook::PlaybookSpec" not in ar and "PlaybookSpec" not in ar:
        # read what it exports
        print("playbook artifact root exports check:")
        print(ar[:800])
    fix_common_renames(plugin)
    print("playbook ok")


# ───────────────────────── IMPERATIVE ─────────────────────────
def migrate_imperative() -> None:
    plugin = REPO / "✏️s/🔌️plugins/📜️imperative"
    art = plugin / "🗿️artifacts/📜️imperative"
    op = (art / "🔧️op/🦀️component.rs").read_text(encoding="utf-8")

    mut = re.sub(r"//#region 📖️SemioGrammar\n.*?//#endregion 📖️SemioGrammar\n+", "", op, flags=re.S)
    mut = mut.replace("//! 🔧️ Imperative artifact — operation enum + laws (constitutional: op).", "//! 🧬️ imperative artifact — document mutation dispatch.")
    mut = mut.replace("//#region 🔖️Operation", "//#region 🔖️Mutations")
    mut = mut.replace("//#endregion 🔖️Operation", "//#endregion 🔖️Mutations")
    mut = mut.replace("assert_operation_round_trip", "assert_mutation_round_trip")
    write(art / "🧬️mutations/🦀️component.rs", mut)
    write(art / "🧬️mutations/🟦️component.ts", "/** 🧩 imperative 🧬️mutations WASM facade. */\nexport {};\n")

    variants = [("✂️step-collection", "step_collection")]
    triad(
        art,
        "✂️step-collection",
        "imperative",
        "Imperative",
        "ImperativeDocument",
        "*projection = <ImperativeMutation as protocol::Mutation<ImperativeDocument>>::diff(mutation, projection).apply(projection);",
        "ImperativeMutation",
    )

    slim_op(art, "imperative", "Imperative", "resolve_steps, ImperativeMutation")

    append_artifact_engine(
        art / "⚙️engine/🦀️component.rs",
        "Imperative",
        "crate::artifacts::imperative::ImperativeDocument",
        "imperative",
        "ImperativeDiff",
        """let diff = <Self::Mutation as protocol::Mutation<Self::Projection>>::diff(mutation, &self.projection);
        self.projection = diff.apply(&self.projection);
        Ok(diff)""",
    )
    insert_mutations_glue(plugin / "📦️packages/🦀️rust/📦️glue.rs", "📜️imperative", "imperative", variants)
    ensure_ts_mutations(plugin / "📦️packages/🟦️typescript/📦️index.ts", "imperative_mutations", "📜️imperative")

    for path in plugin.rglob("*.rs"):
        t = path.read_text(encoding="utf-8")
        n = t.replace("crate::artifacts::imperative::op::ImperativeMutation", "crate::artifacts::imperative::mutations::ImperativeMutation")
        if n != t:
            path.write_text(n, encoding="utf-8")
    fix_common_renames(plugin)
    print("imperative ok")


# ───────────────────────── RASTER ─────────────────────────
def migrate_raster() -> None:
    plugin = REPO / "✏️s/🔌️plugins/🖨️raster"
    art = plugin / "🗿️artifacts/🖨️raster"
    op = (art / "🔧️op/🦀️component.rs").read_text(encoding="utf-8")

    # Split: move Types (enum+Mutation impl+helpers) to mutations; keep OpText codecs in op via re-export...
    # Simpler: move entire file to mutations, then slim op with OpText like note (enum has DslEnum)
    mut = re.sub(r"//#region 📖️SemioGrammar\n.*?//#endregion 📖️SemioGrammar\n+", "", op, flags=re.S)
    # Remove HandcraftedOpCodecs from mutations (stay in op)
    mut = re.sub(r"//#region 🔖️HandcraftedOpCodecs\n.*?//#endregion 🔖️HandcraftedOpCodecs\n+", "", mut, flags=re.S)
    mut = mut.replace("//! ⚡️ Raster artifact — the operation enum + laws (constitutional: op).", "//! 🧬️ raster artifact — document mutation dispatch.")
    mut = mut.replace("//! 🔧️ Raster", "//! 🧬️ raster")
    # Fix header if different
    if not mut.startswith("//! 🧬️"):
        mut = "//! 🧬️ raster artifact — document mutation dispatch.\n\n" + mut
    mut = mut.replace("vcs::apply_operation", "vcs::apply_mutation")
    # Ensure Mutation import
    if "use protocol::Mutation" not in mut and "impl Mutation<" in mut:
        mut = mut.replace("use serde::{Deserialize, Serialize};", "use protocol::Mutation;\nuse serde::{Deserialize, Serialize};")
    # Add apply helper
    if "pub fn apply_raster_mutation" not in mut:
        mut += """
/// ▶️ Applies `mutation` via its diff.
pub fn apply_raster_mutation(projection: &RasterProjection, mutation: &RasterMutation) -> RasterProjection {
    mutation.diff(projection).apply(projection)
}

pub fn inverse_raster_mutation(projection: &RasterProjection, mutation: &RasterMutation) -> Vec<RasterMutation> {
    mutation.inverse(projection)
}
"""
    write(art / "🧬️mutations/🦀️component.rs", mut)
    write(art / "🧬️mutations/🟦️component.ts", "/** 🧩 raster 🧬️mutations WASM facade. */\nexport {};\n")

    variants = [
        ("➕add-layer", "add_layer"),
        ("➖remove-layer", "remove_layer"),
        ("🩹patch-layer", "patch_layer"),
        ("↔️move-layer", "move_layer"),
        ("📄replace-document", "replace_document"),
    ]
    for ek, _ in variants:
        triad(
            art,
            ek,
            "raster",
            "Raster",
            "RasterProjection",
            "*projection = crate::artifacts::raster::mutations::apply_raster_mutation(projection, mutation);",
            "RasterMutation",
        )

    # Slim op with OpText/OpBinary on RasterMutation (DslEnum)
    slim_op(
        art,
        "raster",
        "Raster",
        "apply_raster_mutation, inverse_raster_mutation, RasterMutation, RasterEnvelope, RasterStore",
        extra="""
//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for RasterMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline },
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for RasterMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔖️HandcraftedOpCodecs
""",
    )

    append_artifact_engine(
        art / "⚙️engine/🦀️component.rs",
        "Raster",
        "crate::artifacts::raster::RasterProjection",
        "raster",
        "RasterDiff",
        """let diff = <Self::Mutation as protocol::Mutation<Self::Projection>>::diff(mutation, &self.projection);
        self.projection = crate::artifacts::raster::mutations::apply_raster_mutation(&self.projection, mutation);
        Ok(diff)""",
    )
    insert_mutations_glue(plugin / "📦️packages/🦀️rust/📦️glue.rs", "🖨️raster", "raster", variants)
    ensure_ts_mutations(plugin / "📦️packages/🟦️typescript/📦️index.ts", "raster_mutations", "🖨️raster")
    fix_common_renames(plugin)
    print("raster ok")


def main() -> None:
    migrate_present()
    migrate_sequence()
    migrate_layout()
    migrate_playbook()
    migrate_imperative()
    migrate_raster()
    print("ALL SIX DONE")


if __name__ == "__main__":
    main()
