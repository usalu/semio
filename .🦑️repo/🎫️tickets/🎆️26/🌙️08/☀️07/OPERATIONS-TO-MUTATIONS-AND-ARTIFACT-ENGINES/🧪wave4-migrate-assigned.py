#!/usr/bin/env python3
"""Wave 4: mutations facet + rename for assigned plugin artifacts."""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")

# (plugin emoji dir, artifact emoji dir, rust mod name, TypePrefix, Projection, apply_fn stem)
ARTIFACTS: list[tuple[str, str, str, str, str, str]] = [
    ("🧩️puzzle", "◻2d", "puzzle2d", "Puzzle2d", "Puzzle2dProjection", "puzzle2d"),
    ("🧩️puzzle", "🧊️3d", "puzzle3d", "Puzzle3d", "Puzzle3dProjection", "puzzle3d"),
    ("🧩️puzzle", "🖐️5d", "puzzle5d", "Puzzle5d", "Puzzle5dPlayProjection", "puzzle5d"),
    ("🧱️block", "◻2d", "block2d", "Block2d", "Block2dDefinition", "block2d"),
    ("🧱️block", "🖐️5d", "block5d", "Block5d", "Block5dDefinition", "block5d"),
    ("🧱️block", "🧊️3d", "block3d", "Block3d", "Block3dDefinition", "block3d"),
    ("🏗️fem", "◻2d", "fem2d", "Fem2d", "Fem2dDocument", "fem2d"),
    ("🏗️fem", "🧊️3d", "fem3d", "Fem3d", "Fem3dDocument", "fem3d"),
    ("🌍️gis", "🗺️gismap", "gismap", "GisMap", "GisMapDocument", "gis_map"),
    ("🌍️gis", "🏔️gisterrain", "gisterrain", "Gis3dTerrain", "Gis3dTerrainDocument", "gis_3d_terrain"),
    ("🌀️procedural", "🌀️procedural2d", "procedural2d", "Procedural2d", "Procedural2dDocument", "procedural2d"),
    ("🌀️procedural", "🧊️procedural3d", "procedural3d", "Procedural3d", "Procedural3dDocument", "procedural3d"),
    ("🔱️trinity", "♻️rewrite", "rewrite", "RewriteRule", "RewriteRuleDocument", "rewrite_rule"),
    ("🔱️trinity", "🔌️jack", "jack", "TrinityGraph", "TrinityGraphDocument", "trinity_graph"),
    ("💡️reasoning", "🔌️wires", "wires", "MindmapWires", "MindmapWiresDocument", "mindmap_wires"),
    ("🪵️sourcing", "🗂️curate", "curate", "Sourcing", "SourcingDocument", "sourcing"),
    ("🪐️space", "🏠️home", "home", "SHome", "SHomeDocument", "shome"),
]

EMOJI_FOR = {
    "SetDocument": "📄",
    "NoOperation": "🫙",
    "SetNode": "📍",
    "RemoveNode": "➖",
    "SetEdge": "🔗",
    "RemoveEdge": "✂️",
    "SetMeta": "🏷",
    "Generation": "🧬",
}


def kebab(name: str) -> str:
    s = re.sub(r"([a-z0-9])([A-Z])", r"\1-\2", name)
    return s.replace("_", "-").lower()


def variant_emoji(name: str) -> str:
    if name in EMOJI_FOR:
        return EMOJI_FOR[name]
    if name.startswith("Set"):
        return "🎛"
    if name.startswith("Add"):
        return "➕"
    if name.startswith("Remove"):
        return "➖"
    if name.startswith("Patch"):
        return "🩹"
    if name.startswith("Move"):
        return "↔"
    return "📌"


def rename_text(text: str) -> str:
    shields: dict[str, str] = {}

    def shield(pat: str, key: str) -> None:
        nonlocal text
        out: list[str] = []
        last = 0
        for i, m in enumerate(re.finditer(pat, text)):
            ph = f"__SH_{key}_{i}__"
            shields[ph] = m.group(0)
            out.append(text[last : m.start()])
            out.append(ph)
            last = m.end()
        out.append(text[last:])
        text = "".join(out)

    for pat, key in [
        (r"\bOpText\b", "a"),
        (r"\bOpBinary\b", "b"),
        (r"\bprint_op\b", "c"),
        (r"\bparse_op\b", "d"),
        (r"\bencode_op\b", "e"),
        (r"\bdecode_op\b", "f"),
        (r"\bLanguageRole::Ops\b", "g"),
        (r"boolean_operation", "h"),
        (r"ActionKind::Operation", "i"),
        (r"NoOperation", "j"),
        (r"no-operation", "k"),
        (r"noOperation", "l"),
    ]:
        shield(pat, key)

    reps = [
        ("CollectionOperation", "CollectionMutation"),
        ("apply_collection_operation", "apply_collection_mutation"),
        ("invert_collection_operation", "inverse_collection_mutation"),
        ("collection_diff_from_operation", "collection_diff_from_mutation"),
        ("OperationDiff", "MutationDiff"),
        ("apply_operation", "apply_mutation"),
        ("DocumentApp::Operation", "DocumentApp::Mutation"),
        ("type Operation =", "type Mutation ="),
        ("type ConfigOperation =", "type ConfigMutation ="),
        ("type DraftOperation =", "type DraftMutation ="),
        ("NoDraftOperation", "NoDraftMutation"),
        ("NoConfigOperation", "NoConfigMutation"),
        ("document_operations", "document_mutations"),
        ("config_operations", "config_mutations"),
        ("draft_operations", "draft_mutations"),
        ("Emit::operations", "Emit::mutations"),
        ("Emit::commit(", "Emit::commit("),  # noop anchor
        ("operations: vec", "mutations: vec"),
        ("Apply { operations", "Apply { mutations"),
        ("AmendLast { operations", "AmendLast { mutations"),
        (".backwards(", ".inverse("),
        ("fn backwards(", "fn inverse("),
        ("impl Operation<", "impl Mutation<"),
        ("use protocol::Operation;", "use protocol::Mutation;"),
        ('serde(tag = "operation"', 'serde(tag = "mutation"'),
        ('tag = "operation"', 'tag = "mutation"'),
    ]
    for old, new in reps:
        text = text.replace(old, new)
    text = re.sub(r"(schema\s+\S+)\.operation\b", r"\1.mutation", text)
    text = text.replace("start operation", "start mutation")
    text = re.sub(r"(?m)^operation\s*=", "mutation =", text)
    text = text.replace("unknown operation line", "unknown mutation line")

    for ph, orig in shields.items():
        text = text.replace(ph, orig)
    return text


def parse_variants(op_text: str, prefix: str) -> list[str]:
    for en in (f"{prefix}Operation", f"{prefix}Mutation"):
        m = re.search(rf"pub enum {en} \{{(?P<body>.*?)\n\}}", op_text, re.S)
        if m:
            body = m.group("body")
            names: list[str] = []
            for line in body.splitlines():
                line = line.strip()
                if not line or line.startswith("#") or line.startswith("//") or line.startswith("@"):
                    continue
                if line.startswith("#["):
                    continue
                vm = re.match(r"(\w+)(\s*\{|\s*,|\s*$)", line)
                if vm:
                    names.append(vm.group(1))
            return names
    return []


def ts_stub(label: str) -> str:
    return f"/** 🧩 {label} facade stub. */\nexport {{}};\n"


def write(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if not content.endswith("\n"):
        content += "\n"
    path.write_text(content)
    print("W", path.relative_to(ROOT))


def migrate_artifact(plugin_dir: str, art_dir: str, mod: str, prefix: str, projection: str, stem: str) -> None:
    plugin = ROOT / "✏️s/🔌️plugins" / plugin_dir
    art = plugin / "🗿️artifacts" / art_dir
    op_path = art / "🔧️op" / "🦀️component.rs"
    if not op_path.exists():
        print("SKIP no op", art)
        return
    op_text = op_path.read_text()
    if (art / "🧬️mutations" / "🦀️component.rs").exists() and f"pub enum {prefix}Mutation" in (
        art / "🧬️mutations" / "🦀️component.rs"
    ).read_text():
        print("SKIP already migrated", art)
        return
    if f"pub enum {prefix}Operation" not in op_text and f"pub enum {prefix}Mutation" not in op_text:
        if "pub use" in op_text and "Mutation" in op_text:
            print("SKIP kernel re-export op", art)
            migrate_kernel_reexport(plugin, art, mod, prefix, projection, stem)
            return
        print("SKIP no enum", art)
        return

    op_renamed = rename_text(op_text)
    op_renamed = op_renamed.replace(f"{prefix}Operation", f"{prefix}Mutation")
    op_renamed = op_renamed.replace(f"puzzle2d_operation_diff", f"puzzle2d_mutation_diff")
    op_renamed = op_renamed.replace(f"_operation_diff", f"_mutation_diff")

    variants = parse_variants(op_renamed, prefix)
    mut_root = art / "🧬️mutations"
    art_rs = f"crate::artifacts::{mod}"

    for v in variants:
        if v == "NoOperation":
            continue
        emoji = variant_emoji(v)
        kb = kebab(v)
        base = mut_root / f"{emoji}{kb}"
        mod_snake = kb.replace("-", "_")
        write(
            base / "🦠️mutation" / "🦀️component.rs",
            f"""//! {emoji} {prefix} mutation — `{v}` apply delegate.
use {art_rs}::{projection};
use {art_rs}::mutations::{prefix}Mutation;

pub fn apply(projection: &mut {projection}, mutation: &{prefix}Mutation) {{
    {art_rs}::mutations::apply_{stem}_mutation(projection, mutation);
}}
""",
        )
        write(base / "🔺️diff" / "🦀️component.rs", f"""use {art_rs}::diff::{prefix}Diff;
use {art_rs}::{projection};
use {art_rs}::mutations::{prefix}Mutation;
use protocol::MutationDiff;

pub fn diff_for(mutation: &{prefix}Mutation, base: &{projection}) -> {prefix}Diff {{
    <{prefix}Mutation as protocol::Mutation<{projection}>>::diff(mutation, base)
}}
""")
        write(
            base / "↩️inverse" / "🦀️component.rs",
            f"""use {art_rs}::{projection};
use {art_rs}::mutations::{prefix}Mutation;

pub fn inverse(base: &{projection}, mutation: &{prefix}Mutation) -> Vec<{prefix}Mutation> {{
    <{prefix}Mutation as protocol::Mutation<{projection}>>::inverse(mutation, base)
}}
""",
        )
        write(base / "🦠️mutation" / "🟦️component.ts", ts_stub(f"{mod} {emoji}{kb}/🦠️mutation"))
        write(base / "🔺️diff" / "🟦️component.ts", ts_stub(f"{mod} {emoji}{kb}/🔺️diff"))
        write(base / "↩️inverse" / "🟦️component.ts", ts_stub(f"{mod} {emoji}{kb}/↩️inverse"))

    # Build mutations root: enum + impl from op (strip OpText/Binary to op file later)
    mut_body = op_renamed
    # Remove OpText/Binary impl blocks from mutations copy
    mut_body = re.sub(
        r"//#region 🔖️HandcraftedOpCodecs.*?//#endregion 🔖️HandcraftedOpCodecs",
        "",
        mut_body,
        flags=re.S,
    )
    mut_body = mut_body.replace("//! 🔧", "//! 🧬")
    if f"pub fn apply_{stem}_mutation" not in mut_body:
        mut_body += f"""

pub fn apply_{stem}_mutation(projection: &mut {projection}, mutation: &{prefix}Mutation) {{
    *projection = vcs::apply_mutation(projection, mutation);
}}

pub fn inverse_{stem}_mutation(projection: &{projection}, mutation: &{prefix}Mutation) -> Vec<{prefix}Mutation> {{
    mutation.inverse(projection)
}}
"""

    write(mut_root / "🦀️component.rs", mut_body)
    write(mut_root / "🟦️component.ts", ts_stub(f"{mod} 🧬️mutations WASM"))

    # Slim op: keep grammar + OpText on Mutation + ValueBridge regions
    slim_parts = [
        f"//! ⚡️ {prefix} artifact — OpText/OpBinary codecs + grammar for `{prefix}Mutation`.\n",
        f"pub use {art_rs}::mutations::{{apply_{stem}_mutation, inverse_{stem}_mutation, {prefix}Mutation}};\n",
    ]
    if "//#region 📖️SemioGrammar" in op_renamed:
        slim_parts.append(re.search(r"(//#region 📖️SemioGrammar.*?//#endregion 📖️SemioGrammar)", op_renamed, re.S).group(1))
    slim_parts.append(
        f"""
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
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }}
}}

impl protocol::OpBinary for {prefix}Mutation {{
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {{
        dsl::variants_binary::encode_op(self)
    }}
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {{
        dsl::variants_binary::decode_op(bytes)
    }}
}}
//#endregion 🔖️HandcraftedOpCodecs
"""
    )
    for region in ("//#region 🔖️ValueBridge", "//#region 🔖️PlayProjection"):
        if region in op_renamed:
            idx = op_renamed.index(region)
            rest = op_renamed[idx:]
            slim_parts.append(rest)

    write(op_path, "\n".join(slim_parts))

    gram = art / "🔧️op" / "📖️component.grammar.semio"
    if gram.exists():
        g = rename_text(gram.read_text())
        write(gram, g)

    diff_path = art / "🔺️diff" / "🦀️component.rs"
    if diff_path.exists():
        write(diff_path, rename_text(diff_path.read_text()))

    engine_path = art / "⚙️engine" / "🦀️component.rs"
    if not engine_path.exists():
        engine_path.parent.mkdir(parents=True, exist_ok=True)
        write(
            engine_path,
            f"""//! ⚙️ {prefix} artifact — headless compute (constitutional: engine).

//#region 🔖️ArtifactEngine
pub struct {prefix}Engine {{
    projection: {art_rs}::{projection},
}}

impl {prefix}Engine {{
    pub fn new(projection: {art_rs}::{projection}) -> Self {{
        Self {{ projection }}
    }}
}}

impl protocol::ArtifactEngine for {prefix}Engine {{
    type Projection = {art_rs}::{projection};
    type Mutation = {art_rs}::mutations::{prefix}Mutation;
    type Diff = {art_rs}::diff::{prefix}Diff;

    fn projection(&self) -> &Self::Projection {{
        &self.projection
    }}

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {{
        let diff = <Self::Mutation as protocol::Mutation<Self::Projection>>::diff(mutation, &self.projection);
        {art_rs}::mutations::apply_{stem}_mutation(&mut self.projection, mutation);
        Ok(diff)
    }}

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {{
        <Self::Mutation as protocol::Mutation<Self::Projection>>::inverse(mutation, &self.projection)
    }}
}}
//#endregion 🔖️ArtifactEngine
""",
        )
    elif "ArtifactEngine" not in engine_path.read_text():
        eng = engine_path.read_text()
        eng += f"""

//#region 🔖️ArtifactEngine
pub struct {prefix}Engine {{
    projection: {art_rs}::{projection},
}}

impl {prefix}Engine {{
    pub fn new(projection: {art_rs}::{projection}) -> Self {{
        Self {{ projection }}
    }}
}}

impl protocol::ArtifactEngine for {prefix}Engine {{
    type Projection = {art_rs}::{projection};
    type Mutation = {art_rs}::mutations::{prefix}Mutation;
    type Diff = {art_rs}::diff::{prefix}Diff;

    fn projection(&self) -> &Self::Projection {{
        &self.projection
    }}

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {{
        let diff = <Self::Mutation as protocol::Mutation<Self::Projection>>::diff(mutation, &self.projection);
        {art_rs}::mutations::apply_{stem}_mutation(&mut self.projection, mutation);
        Ok(diff)
    }}

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {{
        <Self::Mutation as protocol::Mutation<Self::Projection>>::inverse(mutation, &self.projection)
    }}
}}
//#endregion 🔖️ArtifactEngine
"""
        write(engine_path, eng)

    inject_glue(plugin, art_dir, mod, variants)


def migrate_kernel_reexport(plugin: Path, art: Path, mod: str, prefix: str, projection: str, stem: str) -> None:
    """Playbook-style: enum lives in kernel; facet wraps PlaybookMutation."""
    pass


def inject_glue(plugin: Path, art_dir: str, mod: str, variants: list[str]) -> None:
    glue_path = plugin / "📦️packages" / "🦀️rust" / "📦️glue.rs"
    if not glue_path.exists():
        return
    glue = glue_path.read_text()
    if f"pub mod {mod}" not in glue and f"mod {mod}" not in glue:
        return
    if f"pub mod mutations" in glue and f"artifacts::{mod}::mutations" in glue:
        return
    block = [
        '\n        #[path = "."]\n        pub mod mutations {\n',
        f'            #[path = "../../🗿️artifacts/{art_dir}/🧬️mutations/🦀️component.rs"]\n',
        "            mod component;\n",
        "            pub use component::*;\n",
    ]
    for v in variants:
        if v == "NoOperation":
            continue
        emoji = variant_emoji(v)
        kb = kebab(v)
        mod_snake = kb.replace("-", "_")
        dirname = f"{emoji}{kb}"
        block.append(f"""
            #[path = "."]
            pub mod {mod_snake} {{
                #[path = "../../🗿️artifacts/{art_dir}/🧬️mutations/{dirname}/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/{art_dir}/🧬️mutations/{dirname}/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/{art_dir}/🧬️mutations/{dirname}/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }}
""")
    block.append("        }\n")
    block_s = "".join(block)
    needle = f"pub mod {mod.split('_')[0]}"  # fragile
    # insert after `pub mod op;` inside artifact mod
    art_folder = art_dir
    pattern = rf"(pub mod {re.escape(mod.split('2d')[0] if '2d' in mod else mod)}.*?pub mod op;)"
    # simpler: after first `pub mod op;` following artifact path comment
    marker = f"🗿️artifacts/{art_dir}"
    if marker not in glue:
        # try mod name from path
        for line in glue.splitlines():
            if art_dir in line and "🗿️artifacts" in line:
                marker = art_dir
                break
    idx = glue.find(f"../../🗿️artifacts/{art_dir}/🔧️op/")
    if idx == -1:
        idx = glue.find("pub mod op;")
    if idx != -1:
        insert_at = glue.find("pub mod op;", idx)
        if insert_at != -1:
            end = insert_at + len("pub mod op;")
            glue = glue[:end] + block_s + glue[end:]
            write(glue_path, glue)


def rename_plugin(plugin_dir: str) -> None:
    plugin = ROOT / "✏️s/🔌️plugins" / plugin_dir
    for p in plugin.rglob("*"):
        if not p.is_file() or p.suffix not in {".rs", ".ts", ".semio"}:
            continue
        try:
            t = p.read_text()
        except Exception:
            continue
        n = rename_text(t)
        # targeted Operation renames per plugin artifact prefix — avoid breaking unrelated *Operation
        n = re.sub(r"\b(\w+)Operation\b", lambda m: f"{m.group(1)}Mutation" if m.group(1)[0].isupper() else m.group(0), n)
        n = n.replace("PlaybookOperation", "PlaybookMutation")
        n = n.replace("apply_playbook_edit_operation", "apply_playbook_edit_mutation")
        if n != t:
            p.write_text(n)
            print("R", p.relative_to(ROOT))


def stub_artifact(plugin_dir: str, art_dir: str, mod: str, prefix: str, schema: str) -> None:
    art = ROOT / "✏️s/🔌️plugins" / plugin_dir / "🗿️artifacts" / art_dir
    mut_root = art / "🧬️mutations"
    projection = f"{prefix}Document"
    write(
        mut_root / "🦀️component.rs",
        f"""//! 🧬️ {prefix} artifact — minimal mutation dispatch.
use serde::{{Deserialize, Serialize}};
use protocol::Mutation;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum {prefix}Mutation {{
    #[default]
    NoMutation,
    SetDocument {{
        #[dsl(block)]
        document: {prefix}Document,
    }},
}}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct {prefix}Document {{
    pub schema: String,
}}

pub fn apply_{mod}_mutation(projection: &mut {prefix}Document, mutation: &{prefix}Mutation) {{
    match mutation {{
        {prefix}Mutation::NoMutation => {{}}
        {prefix}Mutation::SetDocument {{ document }} => *projection = document.clone(),
    }}
}}

impl Mutation<{prefix}Document> for {prefix}Mutation {{
    type Diff = {prefix}Mutation;
    fn diff(&self, _p: &{prefix}Document) -> Self::Diff {{ self.clone() }}
    fn inverse(&self, p: &{prefix}Document) -> Vec<Self> {{
        vec![{prefix}Mutation::SetDocument {{ document: p.clone() }}]
    }}
}}
""",
    )
    write(mut_root / "🟦️component.ts", ts_stub(f"{mod} mutations"))
    for emoji, kb in [("📄", "set-document"), ("🫙", "no-mutation")]:
        base = mut_root / f"{emoji}{kb}"
        write(base / "🦠️mutation" / "🦀️component.rs", "// stub\n")
        write(base / "🔺️diff" / "🦀️component.rs", "// stub\n")
        write(base / "↩️inverse" / "🦀️component.rs", "// stub\n")
    op = art / "🔧️op"
    op.mkdir(parents=True, exist_ok=True)
    write(op / "🦀️component.rs", f"pub use crate::artifacts::{mod}::mutations::{prefix}Mutation;\n")
    eng = art / "⚙️engine"
    eng.mkdir(parents=True, exist_ok=True)
    write(
        eng / "🦀️component.rs",
        f"""pub struct {prefix}Engine {{ projection: crate::artifacts::{mod}::mutations::{prefix}Document }}
impl protocol::ArtifactEngine for {prefix}Engine {{
    type Projection = crate::artifacts::{mod}::mutations::{prefix}Document;
    type Mutation = crate::artifacts::{mod}::mutations::{prefix}Mutation;
    type Diff = crate::artifacts::{mod}::mutations::{prefix}Mutation;
    fn projection(&self) -> &Self::Projection {{ &self.projection }}
    fn apply(&mut self, m: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {{
        crate::artifacts::{mod}::mutations::apply_{mod}_mutation(&mut self.projection, m);
        Ok(m.diff(&self.projection))
    }}
    fn inverse(&self, m: &Self::Mutation) -> Vec<Self::Mutation> {{ m.inverse(&self.projection) }}
}}
""",
    )


def main() -> None:
    for row in ARTIFACTS:
        migrate_artifact(*row)
    stub_artifact("🎪️demonstrator", "🎪️playground", "playground", "Playground", "demonstrator.playground")
    stub_artifact("🔋️energy", "🔋️model", "model", "EnergyModel", "energy.model")
    plugins = sorted({a[0] for a in ARTIFACTS} | {"🎪️demonstrator", "🔋️energy", "📖️playbook"})
    for p in plugins:
        rename_plugin(p)
    print("DONE")


if __name__ == "__main__":
    main()
