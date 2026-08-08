#!/usr/bin/env python3
"""Wave 4: add 🧬️mutations triads + split 🔧️op for owned enums. Run from repo root."""
from __future__ import annotations
import os
import re
import textwrap

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))))

def write(path: str, content: str) -> None:
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)

def triad(artifact_abs: str, emoji: str, kebab: str, mod: str, art_mod: str, prefix: str, proj: str, diff: str) -> None:
    base = os.path.join(artifact_abs, "🧬️mutations", f"{emoji}{kebab}")
    for leaf in ("🦠️mutation", "🔺️diff", "↩️inverse"):
        os.makedirs(os.path.join(base, leaf), exist_ok=True)
    art = f"crate::artifacts::{art_mod}"
    write(
        os.path.join(base, "🦠️mutation", "🦀️component.rs"),
        f"""//! {emoji} {prefix} mutation — `{kebab}` apply.
use {art}::{proj};
use {art}::mutations::{prefix}Mutation;

pub fn apply(projection: &mut {proj}, mutation: &{prefix}Mutation) {{
    {art}::mutations::apply_{art_mod}_mutation(projection, mutation);
}}
""",
    )
    write(
        os.path.join(base, "🔺️diff", "🦀️component.rs"),
        f"""use {art}::diff::{diff};
use {art}::{proj};
use {art}::mutations::{prefix}Mutation;
use protocol::MutationDiff;

pub fn into_diff(mutation: &{prefix}Mutation, base: &{proj}) -> {diff} {{
    mutation.diff(base)
}}
""",
    )
    write(
        os.path.join(base, "↩️inverse", "🦀️component.rs"),
        f"""use {art}::{proj};
use {art}::mutations::{prefix}Mutation;

pub fn inverse(base: &{proj}, mutation: &{prefix}Mutation) -> Vec<{prefix}Mutation> {{
    mutation.inverse(base)
}}
""",
    )
    write(os.path.join(base, "🦠️mutation", "🟦️component.ts"), "export {};\n")

def glue_mod(mod: str, artifact_glue_path: str, emoji_kebab: str) -> str:
    art_folder = artifact_glue_path.split("🗿️artifacts/")[1].split("/")[0]
    return textwrap.dedent(
        f"""
            #[path = "."]
            pub mod {mod} {{
                #[path = "../../🗿️artifacts/{art_folder}/🧬️mutations/{emoji_kebab}/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/{art_folder}/🧬️mutations/{emoji_kebab}/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/{art_folder}/🧬️mutations/{emoji_kebab}/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }}"""
    )

def install_owned(
    plugin_rel: str,
    art_folder: str,
    art_mod: str,
    prefix: str,
    proj: str,
    diff: str,
    apply_old: str,
    apply_new: str,
    variants: list[tuple[str, str, str]],
) -> None:
    artifact_abs = os.path.join(REPO, plugin_rel, "🗿️artifacts", art_folder)
    op_path = os.path.join(artifact_abs, "🔧️op", "🦀️component.rs")
    op_text = open(op_path, encoding="utf-8").read()
    op_text = op_text.replace(apply_old, apply_new)
    mut_root = os.path.join(artifact_abs, "🧬️mutations")
    write(os.path.join(mut_root, "🟦️component.ts"), f'/** 🧩 {art_mod} 🧬️mutations WASM facade. */\nexport {{}};\n')

    for emoji, kebab, mod in variants:
        triad(artifact_abs, emoji, kebab, mod, art_mod, prefix, proj, diff)

    mut_rs = op_text
    mut_rs = re.sub(r"//#region 🔖️HandcraftedOpCodecs.*?//#endregion 🔖️HandcraftedOpCodecs\n", "", mut_rs, flags=re.S)
    mut_rs = mut_rs.replace("//! ⚡️ VCS artifact", f"//! 🧬️ {art_mod} artifact — document mutation dispatch.")
    mut_rs = mut_rs.replace("operation enum + laws", "mutations facet")
    mut_rs = mut_rs.replace("//#region 🔖️Types", "//#region 🔖️Mutations")
    mut_rs = mut_rs.replace("//#endregion 🔖️Types", "//#endregion 🔖️Mutations")
    if f"pub fn {apply_new}" not in mut_rs and f"pub fn {apply_old}" in mut_rs:
        pass
    elif apply_old in mut_rs:
        mut_rs = mut_rs.replace(f"pub fn {apply_old}", f"pub fn {apply_new}")

    dispatch = "\n".join(
        f"        {prefix}Mutation::{v[0].split('_')[-1].title()} {{ .. }} => super::{v[2]}::mutation::apply(projection, mutation),"
        for v in variants
    )
    write(os.path.join(mut_root, "🦀️component.rs"), mut_rs)

    op_new = f"""//! 🔧 {art_mod} artifact — OpText/OpBinary for `{prefix}Mutation`.

pub use crate::artifacts::{art_mod}::mutations::{{{prefix}Mutation, {apply_new}, inverse_{art_mod}_mutation}};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️HandcraftedOpCodecs
impl protocol::OpText for {prefix}Mutation {{
    fn parse_op(line: &str) -> Result<Self, store::TextError> {{
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {{
            let probe = format!("{{}} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {{
                let record = dsl::parse(
                    line,
                    &spec_fn(),
                    &dsl::ParseOptions {{ limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline }},
                )?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }}
        }}
        Err(dsl::__rt::field_error(format!("unknown mutation line '{{line}}'")))
    }}
    fn print_op(&self) -> String {{
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }}
}}

impl protocol::OpBinary for {prefix}Mutation {{
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {{ dsl::variants_binary::encode_op(self) }}
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {{ dsl::variants_binary::decode_op(bytes) }}
}}
//#endregion 🔖️HandcraftedOpCodecs
"""
    write(op_path, op_new)

    engine_path = os.path.join(artifact_abs, "⚙️engine", "🦀️component.rs")
    if os.path.isfile(engine_path):
        eng = open(engine_path, encoding="utf-8").read()
        if "ArtifactEngine" not in eng:
            block = f"""

//#region 🔖️ArtifactEngine
pub struct {prefix}Engine {{
    projection: {proj},
}}

impl {prefix}Engine {{
    pub fn new(projection: {proj}) -> Self {{ Self {{ projection }} }}
    pub fn into_projection(self) -> {proj} {{ self.projection }}
}}

impl protocol::ArtifactEngine for {prefix}Engine {{
    type Projection = {proj};
    type Mutation = crate::artifacts::{art_mod}::mutations::{prefix}Mutation;
    type Diff = crate::artifacts::{art_mod}::diff::{diff};

    fn projection(&self) -> &Self::Projection {{ &self.projection }}

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {{
        let diff = mutation.diff(&self.projection);
        self.projection = {apply_new}(&self.projection, mutation);
        Ok(diff)
    }}

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {{
        mutation.inverse(&self.projection)
    }}
}}
//#endregion 🔖️ArtifactEngine
"""
            write(engine_path, eng.rstrip() + block)

    glue_path = os.path.join(REPO, plugin_rel, "📦️packages/🦀️rust/📦️glue.rs")
    glue = open(glue_path, encoding="utf-8").read()
    if "pub mod mutations" not in glue:
        insert = textwrap.dedent(
            f"""
        #[path = "."]
        pub mod mutations {{
            #[path = "../../🗿️artifacts/{art_folder}/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;
"""
        )
        for emoji, kebab, mod in variants:
            insert += glue_mod(mod, f"🗿️artifacts/{art_folder}", f"{emoji}{kebab}")
        insert += "        }\n"
        glue = glue.replace(
            f'        #[path = "../../🗿️artifacts/{art_folder}/🔧️op/🦀️component.rs"]\n        pub mod op;',
            f'        #[path = "../../🗿️artifacts/{art_folder}/🔧️op/🦀️component.rs"]\n        pub mod op;\n{insert}',
        )
        open(glue_path, "w", encoding="utf-8").write(glue)

    ts_index = os.path.join(REPO, plugin_rel, "📦️packages/🟦️typescript/📦️index.ts")
    if os.path.isfile(ts_index):
        ts = open(ts_index, encoding="utf-8").read()
        export = f'export * as {art_mod}_mutations from "../../🗿️artifacts/{art_folder}/🧬️mutations/🟦️component.ts";'
        if f"{art_mod}_mutations" not in ts:
            ts = ts.replace(f"export * as {art_mod}_op", f"export * as {art_mod}_op\n{export}")
            open(ts_index, "w", encoding="utf-8").write(ts)

    print("installed", plugin_rel, art_mod)


def install_kernel_reexport(
    plugin_rel: str,
    art_folder: str,
    art_mod: str,
    prefix: str,
    proj: str,
    diff: str,
    kernel_path: str,
    kernel_type: str,
    apply_fn: str | None,
    variants: list[tuple[str, str, str]],
) -> None:
    artifact_abs = os.path.join(REPO, plugin_rel, "🗿️artifacts", art_folder)
    mut_root = os.path.join(artifact_abs, "🧬️mutations")
    write(
        os.path.join(mut_root, "🦀️component.rs"),
        f"""//! 🧬️ {art_mod} — kernel `{kernel_type}` re-export + apply helper.
pub use {kernel_path}::{kernel_type} as {prefix}Mutation;
pub use {kernel_path}::{kernel_type};

pub fn apply_{art_mod}_mutation(projection: &mut {proj}, mutation: &{prefix}Mutation) -> {proj} {{
    {"apply_fn(projection, mutation)" if apply_fn else "mutation.diff(projection).apply(projection)"}
}}

pub fn inverse_{art_mod}_mutation(projection: &{proj}, mutation: &{prefix}Mutation) -> Vec<{prefix}Mutation> {{
    mutation.inverse(projection)
}}
""",
    )
    write(os.path.join(mut_root, "🟦️component.ts"), f'export {{}};\n')
    for emoji, kebab, mod in variants:
        triad(artifact_abs, emoji, kebab, mod, art_mod, prefix, proj, diff)

    write(
        os.path.join(artifact_abs, "🔧️op", "🦀️component.rs"),
        f"""//! 🔧 {art_mod} — Op facet re-exports `{prefix}Mutation`.
pub use crate::artifacts::{art_mod}::mutations::{{{prefix}Mutation, apply_{art_mod}_mutation, inverse_{art_mod}_mutation}};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar
""",
    )
    print("kernel reexport", plugin_rel)


if __name__ == "__main__":
    install_owned(
        "✏️s/🔌️plugins/🌿️vcs",
        "🌿️vcs",
        "vcs",
        "VcsDemo",
        "VcsDemoProjection",
        "VcsDemoDiff",
        "apply_vcs_demo_operation",
        "apply_vcs_demo_mutation",
        [
            ("🔢", "set-counter", "set_counter"),
            ("📛", "set-title", "set_title"),
            ("📝", "set-notes", "set_notes"),
            ("🚦", "set-status", "set_status"),
            ("🏷️", "add-tag", "add_tag"),
            ("🗑️", "remove-tag", "remove_tag"),
        ],
    )
