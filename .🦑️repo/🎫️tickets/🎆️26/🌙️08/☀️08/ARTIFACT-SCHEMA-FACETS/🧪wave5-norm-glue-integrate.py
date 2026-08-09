#!/usr/bin/env python3
"""Wave-5 norm glue integrator."""
from __future__ import annotations
from pathlib import Path
import re, shutil

p = Path(__file__).resolve()
while p != p.parent and not ((p / "Cargo.toml").exists() and (p / "✏️s").exists()):
    p = p.parent
ROOT = p
NORM = ROOT / "✏️s/🔌️plugins/📕️norm"
GLUE = NORM / "📦️packages/🦀️rust/📦️glue.rs"
CARGO = NORM / "📦️packages/🦀️rust/Cargo.toml"
INDEX = NORM / "📦️packages/🟦️typescript/📦️index.ts"
SETUP = NORM / "🔌️plugin/🔧️setup/🦀️component.rs"
APP_SURFACE = NORM / "🖥️app-surface/🦀️component.rs"

ARTIFACTS = [
    ('din4108', '📕️din4108', 'Din4108'),
    ('din16798', '📗️din16798', 'Din16798'),
    ('din18599', '📙️din18599', 'Din18599'),
    ('en1990', '📘️en1990', 'En1990'),
    ('en1991', '📘️en1991', 'En1991'),
    ('en1992', '📘️en1992', 'En1992'),
    ('en1993', '📘️en1993', 'En1993'),
    ('en1994', '📘️en1994', 'En1994'),
    ('en1995', '📘️en1995', 'En1995'),
    ('en1996', '📘️en1996', 'En1996'),
    ('en1997', '📘️en1997', 'En1997'),
    ('en1998', '📘️en1998', 'En1998'),
    ('en1999', '📘️en1999', 'En1999'),
    ('iso16757', '📓️iso16757', 'Iso16757'),
    ('vdi3805', '📔️vdi3805', 'Vdi3805'),
]
DIN_ISO = {"din4108", "din16798", "din18599", "iso16757", "vdi3805"}
NEEDS_APP = DIN_ISO | {f"en199{i}" for i in range(5)}

def artifact_mod(key, folder):
    t = """    #[path = "."]
    pub mod {key} {
        #[path = "../../🗿️artifacts/{folder}/🦀️component.rs"]
        mod component;
        pub use component::*;

        #[path = "../../🗿️artifacts/{folder}/🧬️schema/🦀️component.rs"]
        pub mod schema;

        #[path = "."]
        pub mod snapshot {
            #[path = "../../🗿️artifacts/{folder}/📸️snapshot/🧬️schema/🦀️component.rs"]
            pub mod schema;

            #[path = "../../🗿️artifacts/{folder}/📸️snapshot/🎒️pack/🦀️component.rs"]
            pub mod pack;
        }

        #[path = "."]
        pub mod diff {
            #[path = "../../🗿️artifacts/{folder}/🔺️diff/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "../../🗿️artifacts/{folder}/🔺️diff/🧬️schema/🦀️component.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod mutations {
            #[path = "../../🗿️artifacts/{folder}/🧬️mutations/🦀️component.rs"]
            mod component;
            pub use component::*;

            #[path = "."]
            pub mod set_snapshot {
                #[path = "../../🗿️artifacts/{folder}/🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs"]
                pub mod mutation;
                #[path = "../../🗿️artifacts/{folder}/🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs"]
                pub mod diff;
                #[path = "../../🗿️artifacts/{folder}/🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs"]
                pub mod inverse;
            }
        }

        #[path = "../../🗿️artifacts/{folder}/🔧️op/🦀️component.rs"]
        pub mod op;
        #[path = "../../🗿️artifacts/{folder}/🗣️dsl/🦀️component.rs"]
        pub mod dsl;
        #[path = "../../🗿️artifacts/{folder}/📡️spr/🦀️component.rs"]
        pub mod spr;
        #[path = "../../🗿️artifacts/{folder}/⚙️engine/🦀️component.rs"]
        pub mod engine;
    }
"""
    return t.replace("{key}", key).replace("{folder}", folder)

def patch_glue():
    text = GLUE.read_text()
    if "extern crate semio_framework_schema as schema" not in text:
        text = text.replace(
            "extern crate semio_framework_os_kernel as vcs;",
            "extern crate semio_framework_os_kernel as vcs;\nextern crate semio_framework_schema as schema;",
        )
    arts = "\n".join(artifact_mod(k, f) for k, f, _ in ARTIFACTS)
    text, n = re.subn(
        r"//#region 🗿️Artifacts\n.*?//#endregion 🗿️Artifacts",
        "//#region 🗿️Artifacts\n#[path = \".\"]\npub mod artifacts {\n" + arts + "\n}\n//#endregion 🗿️Artifacts",
        text, count=1, flags=re.S,
    )
    assert n == 1, "artifacts region not replaced"
    text = text.replace(
        "🎮️commands/📤️set-document/🦀️component.rs\"]\n            pub mod set_document;",
        "🎮️commands/📤️set-snapshot/🦀️component.rs\"]\n            pub mod set_snapshot;",
    )
    text = text.replace(
        "🎮️commands/📤️set-snapshot/🦀️component.rs\"]\n            pub mod set_document;",
        "🎮️commands/📤️set-snapshot/🦀️component.rs\"]\n            pub mod set_snapshot;",
    )
    GLUE.write_text(text)
    print("patched glue.rs")

def patch_cargo():
    cargo = CARGO.read_text()
    dep = (
        'semio-framework-schema = { path = "../../../../../'
        '🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust", '
        'package = "semio-framework-schema" }'
    )
    if "semio-framework-schema" not in cargo:
        needle = (
            'semio-framework-plugin = { path = "../../../../../'
            '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust", '
            'features = ["component-guest"], package = "semio-framework-plugin" }'
        )
        assert needle in cargo, "plugin dep needle missing"
        cargo = cargo.replace(needle, needle + "\n" + dep)
        CARGO.write_text(cargo)
    schema_path = (CARGO.parent / "../../../../../🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust").resolve()
    assert schema_path.exists(), schema_path
    print("patched Cargo.toml", schema_path)

def patch_index():
    lines = ["/** norm facet WASM facades */"]
    for key, folder, _ in ARTIFACTS:
        lines.extend([
            f'export * as {key}_schema from "../../🗿️artifacts/{folder}/🧬️schema/🟦️component.ts";',
            f'export * as {key}_snapshot_schema from "../../🗿️artifacts/{folder}/📸️snapshot/🧬️schema/🟦️component.ts";',
            f'export * as {key}_diff from "../../🗿️artifacts/{folder}/🔺️diff/🟦️component.ts";',
            f'export * as {key}_diff_schema from "../../🗿️artifacts/{folder}/🔺️diff/🧬️schema/🟦️component.ts";',
            f'export * as {key}_dsl from "../../🗿️artifacts/{folder}/🗣️dsl/🟦️component.ts";',
            f'export * as {key}_pack from "../../🗿️artifacts/{folder}/📸️snapshot/🎒️pack/🟦️component.ts";',
            f'export * as {key}_op from "../../🗿️artifacts/{folder}/🔧️op/🟦️component.ts";',
            f'export * as {key}_mutations from "../../🗿️artifacts/{folder}/🧬️mutations/🟦️component.ts";',
            f'export * as {key}_spr from "../../🗿️artifacts/{folder}/📡️spr/🟦️component.ts";',
        ])
    INDEX.write_text("\n".join(lines) + "\n")
    print("patched index.ts")

def patch_setup():
    langs = "\n".join(
        f"    crate::artifacts::{k}::engine::register_pilot_languages();" for k, _, _ in ARTIFACTS
    )
    regs = "\n".join(
        f"    crate::artifacts::{k}::engine::register_artifact_schema();" for k, _, _ in ARTIFACTS
    )
    SETUP.write_text(
        "//! 🔧️ Setup facet for `📕️norm` — codec/language/schema registration.\n\n"
        "/// 🔌️ Registers every norm artifact language + schema descriptor.\n"
        "pub fn register_norm_exports() {\n"
        + langs + "\n" + regs + "\n}\n"
    )
    print("patched setup")

def rewrite_text(path: Path, *repls):
    text = path.read_text()
    orig = text
    for a, b in repls:
        text = text.replace(a, b)
    if text != orig:
        path.write_text(text)
        return True
    return False

def ensure_set_snapshot_mutation_files(key, folder, prefix):
    base = NORM / f"🗿️artifacts/{folder}/🧬️mutations"
    old = base / "📤️set-document"
    new = base / "📄set-snapshot"
    if old.exists() and not new.exists():
        shutil.move(str(old), str(new))
    elif old.exists() and new.exists():
        shutil.rmtree(old)
    # mutation leaf
    mut = new / "🦠️mutation/🦀️component.rs"
    mut.parent.mkdir(parents=True, exist_ok=True)
    mut.write_text(f"""//! 📸️ {prefix} mutation — SetSnapshot payload + builder + apply.
use crate::artifacts::{key}::{prefix}Snapshot;
use crate::artifacts::{key}::mutations::{prefix}Mutation;

pub fn set_snapshot(snapshot: {prefix}Snapshot) -> {prefix}Mutation {{
    {prefix}Mutation::SetSnapshot {{ snapshot }}
}}

pub fn apply(base: &mut {prefix}Snapshot, replacement: &{prefix}Snapshot) {{
    *base = replacement.clone();
}}
""")
    diff = new / "🔺️diff/🦀️component.rs"
    diff.parent.mkdir(parents=True, exist_ok=True)
    diff.write_text(f"""//! 🔺️ Diff fragment for SetSnapshot on {prefix}.
pub type Diff = crate::artifacts::{key}::diff::{prefix}Diff;
""")
    inv = new / "↩️inverse/🦀️component.rs"
    inv.parent.mkdir(parents=True, exist_ok=True)
    inv.write_text(f"""//! ↩️ Inverse for SetSnapshot on {prefix}.
use crate::artifacts::{key}::mutations::{prefix}Mutation;
use crate::artifacts::{key}::{prefix}Snapshot;

pub fn inverse(base: &{prefix}Snapshot) -> Vec<{prefix}Mutation> {{
    vec![{prefix}Mutation::SetSnapshot {{ snapshot: base.clone() }}]
}}
""")
    # mutations root
    (base / "🦀️component.rs").write_text(f"""//! 🧬️ {prefix} artifact — document mutation dispatch.

use crate::artifacts::{key}::diff::{{diff_set_snapshot, {prefix}Diff}};
use crate::artifacts::{key}::{prefix}Snapshot;
use protocol::Mutation;
use serde::{{Deserialize, Serialize}};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslEnum)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum {prefix}Mutation {{
    SetSnapshot {{
        #[dsl(block)]
        snapshot: {prefix}Snapshot,
    }},
}}

impl Mutation<{prefix}Snapshot> for {prefix}Mutation {{
    type Diff = {prefix}Diff;

    fn diff(&self, _snapshot: &{prefix}Snapshot) -> {prefix}Diff {{
        match self {{
            {prefix}Mutation::SetSnapshot {{ snapshot }} => diff_set_snapshot(snapshot),
        }}
    }}

    fn inverse(&self, snapshot: &{prefix}Snapshot) -> Vec<Self> {{
        match self {{
            {prefix}Mutation::SetSnapshot {{ .. }} => vec![{prefix}Mutation::SetSnapshot {{ snapshot: snapshot.clone() }}],
        }}
    }}
}}
""")

ENGINE_IMPL = """
//#region 🔖️ArtifactEngine
/// ⚙️ UI-independent {prefix} artifact engine — owns the full artifact; `snapshot()` is persisted only.
pub struct {prefix}Engine {
    artifact: crate::artifacts::{key}::schema::{prefix}Artifact,
    snapshot: crate::artifacts::{key}::{prefix}Snapshot,
}

impl {prefix}Engine {
    pub fn new(snapshot: crate::artifacts::{key}::{prefix}Snapshot) -> Self {
        let artifact = crate::artifacts::{key}::schema::{prefix}Artifact::from_snapshot(snapshot.clone());
        Self { artifact, snapshot }
    }

    pub fn into_snapshot(self) -> crate::artifacts::{key}::{prefix}Snapshot {
        self.snapshot
    }
}

impl protocol::ArtifactEngine for {prefix}Engine {
    type Artifact = crate::artifacts::{key}::schema::{prefix}Artifact;
    type Snapshot = crate::artifacts::{key}::{prefix}Snapshot;
    type Mutation = crate::artifacts::{key}::mutations::{prefix}Mutation;
    type Diff = crate::artifacts::{key}::diff::{prefix}Diff;

    fn artifact(&self) -> &Self::Artifact {
        &self.artifact
    }

    fn snapshot(&self) -> &Self::Snapshot {
        &self.snapshot
    }

    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot);
        self.snapshot = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot);
        self.artifact.set_snapshot(self.snapshot.clone());
        Ok(diff)
    }

    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot)
    }
}
//#endregion 🔖️ArtifactEngine
"""

def patch_engine(key, folder, prefix):
    path = NORM / f"🗿️artifacts/{folder}/⚙️engine/🦀️component.rs"
    text = path.read_text()
    # Replace Document type usages with Snapshot for this artifact
    text = text.replace(f"use crate::artifacts::{key}::Document;", f"use crate::artifacts::{key}::{prefix}Snapshot;")
    text = text.replace("use crate::artifacts::din4108::Document;", f"use crate::artifacts::{key}::{prefix}Snapshot;")
    # NormFamily Document associated type
    text = re.sub(r"type Document = Document;", f"type Document = {prefix}Snapshot;", text)
    text = re.sub(r"type Document = crate::artifacts::" + key + r"::Document;", f"type Document = {prefix}Snapshot;", text)
    text = text.replace("fn evaluate(document: &Document)", f"fn evaluate(document: &{prefix}Snapshot)")
    text = text.replace("fn evaluate(document: &Self::Document)", "fn evaluate(document: &Self::Document)")
    # Replace old ArtifactEngine block
    new_impl = ENGINE_IMPL.replace("{prefix}", prefix).replace("{key}", key)
    text2, n = re.subn(
        r"//#region 🔖️ArtifactEngine\n.*?//#endregion 🔖️ArtifactEngine",
        new_impl,
        text, count=1, flags=re.S,
    )
    if n == 0:
        # try without region markers
        text2, n = re.subn(
            r"/// @emoji ⚙️ UI-independent.*?fn inverse\(&self, mutation: &Self::Mutation\) -> Vec<Self::Mutation> \{[\s\S]*?\}\n\}",
            new_impl,
            text, count=1,
        )
    if n == 0:
        text2, n = re.subn(
            r"pub struct " + prefix + r"Engine \{[\s\S]*?impl protocol::ArtifactEngine for " + prefix + r"Engine \{[\s\S]*?\n\}",
            new_impl,
            text, count=1,
        )
    assert n == 1, f"engine ArtifactEngine replace failed for {key} n={n}"
    # into_projection leftovers
    text2 = text2.replace("into_projection", "into_snapshot")
    text2 = text2.replace("self.projection", "self.snapshot")
    text2 = text2.replace("projection:", "snapshot:")
    # register_artifact_schema helper
    if "fn register_artifact_schema" not in text2:
        text2 += f"""

//#region 🔖️SchemaRegistry
use std::sync::{{Mutex, OnceLock}};

static SCHEMA_REGISTRY: OnceLock<Mutex<schema::ArtifactSchemaRegistry>> = OnceLock::new();

/// 📌️ Registers the fifteen handcrafted schema leaves for `s.norm.{key}`.
pub fn register_artifact_schema() {{
    let registry = SCHEMA_REGISTRY.get_or_init(|| Mutex::new(schema::ArtifactSchemaRegistry::new()));
    registry
        .lock()
        .expect("schema registry")
        .register(crate::artifacts::{key}::schema::{key}_artifact_schema_descriptor());
}}
//#endregion 🔖️SchemaRegistry
"""
    # Also ensure register_pilot_languages calls it
    if "register_artifact_schema();" not in text2:
        text2 = text2.replace(
            "pub fn register_pilot_languages() {",
            "pub fn register_pilot_languages() {\n    register_artifact_schema();",
        )
    path.write_text(text2)
    print("patched engine", key)

def mechanical_rename_rs(path: Path, key: str, prefix: str):
    if not path.exists():
        return
    text = path.read_text()
    orig = text
    # Type renames — order matters
    text = text.replace("SetDocumentMutation<", f"UNUSED_SetDocumentMutation<")  # avoid accidental
    text = text.replace(f"use crate::artifacts::{key}::Document;", f"use crate::artifacts::{key}::{prefix}Snapshot;")
    text = text.replace(f"crate::artifacts::{key}::Document", f"crate::artifacts::{key}::{prefix}Snapshot")
    # Bare Document type when clearly the snapshot (common patterns)
    text = re.sub(r"\bDocument::default\(\)", f"{prefix}Snapshot::default()", text)
    text = re.sub(r"\bDocument \{", f"{prefix}Snapshot {{", text)
    text = re.sub(r"&Document\b", f"&{prefix}Snapshot", text)
    text = re.sub(r": Document\b", f": {prefix}Snapshot", text)
    text = re.sub(r"<Document>", f"<{prefix}Snapshot>", text)
    text = re.sub(r"<Document,", f"<{prefix}Snapshot,", text)
    text = re.sub(r", Document>", f", {prefix}Snapshot>", text)
    text = re.sub(r"\bDocument,", f"{prefix}Snapshot,", text)
    text = re.sub(r"\(Document\)", f"({prefix}Snapshot)", text)
    # Mutation renames
    text = text.replace("SetDocument { document:", "SetSnapshot { snapshot:")
    text = text.replace("SetDocument { document }", "SetSnapshot { snapshot }")
    text = text.replace("::SetDocument {", "::SetSnapshot {")
    text = text.replace("SetDocumentMutation::SetDocument", f"{prefix}Mutation::SetSnapshot")
    text = text.replace("Din4108Mutation::SetDocument", "Din4108Mutation::SetSnapshot")
    text = text.replace(f"{prefix}Mutation::SetDocument", f"{prefix}Mutation::SetSnapshot")
    text = text.replace("document: Document", f"snapshot: {prefix}Snapshot")
    text = text.replace("document: next", "snapshot: next")
    text = text.replace("payload.document", "payload.snapshot")
    text = text.replace("pub document:", "pub snapshot:")
    text = text.replace("set_document(", "set_snapshot(")
    text = text.replace("fn set_document", "fn set_snapshot")
    text = text.replace('tag = "operation"', 'tag = "mutation"')
    text = text.replace("doc.projection", "doc.snapshot")
    text = text.replace("cfg.projection", "cfg.snapshot")
    text = text.replace("type Projection =", "type Snapshot =")
    text = text.replace("fn initial_projection", "fn initial_snapshot")
    text = text.replace("initial_projection()", "initial_snapshot()")
    # undo unused marker
    text = text.replace("UNUSED_SetDocumentMutation<", "SetDocumentMutation<")
    if text != orig:
        path.write_text(text)

def patch_op(key, folder, prefix):
    path = NORM / f"🗿️artifacts/{folder}/🔧️op/🦀️component.rs"
    text = path.read_text()
    # Ensure re-export of mutation enum
    if f"pub use crate::artifacts::{key}::mutations::{prefix}Mutation;" not in text:
        # replace SetDocumentMutation reexport patterns
        text = re.sub(
            r"pub use crate::document::SetDocumentMutation;\s*\n(?:pub type \w+Mutation = SetDocumentMutation<[^>]+>;\s*)?",
            f"pub use crate::artifacts::{key}::mutations::{prefix}Mutation;\n",
            text,
        )
        if f"{prefix}Mutation" not in text.split("pub use")[0] if False else True:
            pass
    text = text.replace("SetDocumentMutation<", f"{prefix}Mutation_UNUSED<")
    path.write_text(text)
    mechanical_rename_rs(path, key, prefix)
    text = path.read_text()
    # Fix op tests to use SetSnapshot
    text = text.replace(f"{prefix}Mutation_UNUSED<", "SetDocumentMutation<")
    if f"pub use crate::artifacts::{key}::mutations::{prefix}Mutation;" not in text:
        # prepend use after grammar region
        text = text.replace(
            "//#endregion 📖️SemioGrammar\n",
            f"//#endregion 📖️SemioGrammar\n\nuse crate::artifacts::{key}::{prefix}Snapshot;\npub use crate::artifacts::{key}::mutations::{prefix}Mutation;\n",
            1,
        )
    path.write_text(text)
    mechanical_rename_rs(path, key, prefix)
    print("patched op", key)


def ensure_artifact_set_snapshot(key, folder, prefix):
    path = NORM / ('🗿️artifacts/%s/🧬️schema/🦀️component.rs' % folder)
    text = path.read_text()
    if 'fn set_snapshot' in text:
        return
    method = (
        '\n    /// 🔄 Overwrite persistent fields from a snapshot; leave shared-ui untouched.\n'
        '    pub fn set_snapshot(&mut self, snapshot: crate::artifacts::%s::%sSnapshot) {\n' % (key, prefix)
        + '        let selected = self.selected_check_index;\n'
        + '        *self = Self::from_snapshot(snapshot);\n'
        + '        self.selected_check_index = selected;\n'
        + '    }\n'
    )
    if '//#endregion 🔖️Conversions' in text:
        text = text.replace('//#endregion 🔖️Conversions', method + '//#endregion 🔖️Conversions', 1)
    else:
        text += '\n' + method
    path.write_text(text)
    print('added set_snapshot', key)

def fix_din4108_layer_list():
    folder = '📕️din4108'
    rust = NORM / ('🗿️artifacts/%s/🔺️diff/🧬️schema/🦀️component.rs' % folder)
    text = rust.read_text()
    text = text.replace('pub layers: Option<Din4108StringList>', 'pub layers: Option<Din4108LayerList>')
    if 'struct Din4108LayerList' not in text:
        text = text.replace(
            'pub struct Din4108StringList { pub values: Vec<String> }',
            'pub struct Din4108StringList { pub values: Vec<String> }\n\n'
            '#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]\n'
            '#[serde(rename_all = "camelCase", default)]\n'
            'pub struct Din4108LayerList { pub values: Vec<crate::artifacts::din4108::LayerDocument> }',
        )
    rust.write_text(text)
    for name in ['🟦️component.ts', '🔣️component.json', '🔗️component.graphql', '🛰️component.proto']:
        p = NORM / ('🗿️artifacts/%s/🔺️diff/🧬️schema' % folder) / name
        if p.exists():
            p.write_text(p.read_text().replace('Din4108StringList', 'Din4108LayerList'))
    snap = NORM / ('🗿️artifacts/%s/📸️snapshot/🧬️schema/🦀️component.rs' % folder)
    st = snap.read_text()
    if 'LayerDocument {' in st and 'use crate::artifacts::din4108::LayerDocument' not in st:
        st = st.replace(
            'use crate::document::ClimateZoneDe;',
            'use crate::artifacts::din4108::LayerDocument;\nuse crate::document::ClimateZoneDe;',
        )
        snap.write_text(st)
    print('fixed din4108 layer list')

def migrate_app_command(key, folder, prefix):
    app = NORM / ('🎛️apps/%s' % folder)
    old = app / '🎮️commands/📤️set-document'
    new = app / '🎮️commands/📤️set-snapshot'
    if old.exists() and not new.exists():
        shutil.move(str(old), str(new))
    elif old.exists() and new.exists():
        shutil.rmtree(old)
    cmd = new / '🦀️component.rs'
    cmd.parent.mkdir(parents=True, exist_ok=True)
    parts = []
    parts.append('//! 📤️ %s play app command — replace the whole compliance document.\n\n' % prefix)
    parts.append('use crate::artifacts::%s::op::%sMutation;\n' % (key, prefix))
    parts.append('use crate::artifacts::%s::%sSnapshot;\n' % (key, prefix))
    parts.append('use crate::config::{NormConfig, NormConfigMutation};\n')
    parts.append('use semio_framework_plugin::{ConfigView, DocumentView, Emit, Fault};\n')
    parts.append('use serde::{Deserialize, Serialize};\n\n')
    parts.append('//#region 🔖️Payload\n')
    parts.append('#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]\n')
    parts.append('#[dsl(keyword = "set-snapshot")]\n')
    parts.append('pub struct SetSnapshot {\n')
    parts.append('    #[dsl(block)]\n')
    parts.append('    pub snapshot: %sSnapshot,\n' % prefix)
    parts.append('}\n')
    parts.append('//#endregion 🔖️Payload\n\n')
    parts.append('//#region 🔖️Handler\n')
    parts.append('pub fn handle(payload: &SetSnapshot, _doc: &DocumentView<\'_, %sSnapshot>, _cfg: &ConfigView<\'_, NormConfig>) -> Result<Emit<%sMutation, NormConfigMutation>, Fault> {\n' % (prefix, prefix))
    parts.append('    crate::app_surface::commit_snapshot(%sMutation::SetSnapshot { snapshot: payload.snapshot.clone() }, "setSnapshot")\n' % prefix)
    parts.append('}\n')
    parts.append('//#endregion 🔖️Handler\n\n')
    parts.append('//#region 🧪️Tests\n')
    parts.append('#[cfg(test)]\n')
    parts.append('mod tests {\n')
    parts.append('    use super::*;\n')
    parts.append('    use crate::artifacts::%s::op::%sMutation;\n' % (key, prefix))
    parts.append('    use semio_framework_plugin::HistoryView;\n\n')
    parts.append('    #[test]\n')
    parts.append('    fn handle_commits_the_payload_document_under_its_action_id() {\n')
    parts.append('        let projection = %sSnapshot::default();\n' % prefix)
    parts.append('        let config = NormConfig::default();\n')
    parts.append('        let emit = handle(\n')
    parts.append('            &SetSnapshot { snapshot: %sSnapshot::default() },\n' % prefix)
    parts.append('            &DocumentView { snapshot: &projection, history: &HistoryView::empty() },\n')
    parts.append('            &ConfigView { snapshot: &config },\n')
    parts.append('        )\n')
    parts.append('        .expect("handle");\n')
    parts.append('        assert_eq!(emit.document_mutations, vec![%sMutation::SetSnapshot { snapshot: %sSnapshot::default() }]);\n' % (prefix, prefix))
    parts.append('        assert_eq!(emit.description.as_deref(), Some("setSnapshot"));\n')
    parts.append('        assert!(emit.config_mutations.is_empty());\n')
    parts.append('    }\n')
    parts.append('}\n')
    parts.append('//#endregion 🧪️Tests\n')
    cmd.write_text(''.join(parts))

def patch_app_root(key, folder, prefix):
    path = NORM / ('🎛️apps/%s/🦀️component.rs' % folder)
    text = path.read_text()
    text = text.replace('set_document', 'set_snapshot')
    text = text.replace('"setDocument" as "set-document" => set_snapshot::SetDocument', '"setSnapshot" as "set-snapshot" => set_snapshot::SetSnapshot')
    text = text.replace('set_snapshot::SetDocument', 'set_snapshot::SetSnapshot')
    text = text.replace('SetDocument(', 'SetSnapshot(')
    text = text.replace('SetDocument { document:', 'SetSnapshot { snapshot:')
    text = text.replace('use crate::artifacts::%s::Document;' % key, 'use crate::artifacts::%s::%sSnapshot;' % (key, prefix))
    text = text.replace('for Document,', 'for %sSnapshot,' % prefix)
    text = text.replace('type Projection =', 'type Snapshot =')
    text = text.replace('fn initial_projection', 'fn initial_snapshot')
    text = text.replace('Document::default()', '%sSnapshot::default()' % prefix)
    text = text.replace('DocumentView<\'_, Document>', 'DocumentView<\'_, %sSnapshot>' % prefix)
    text = text.replace('.mutation("setDocument"', '.mutation("setSnapshot"')
    text = text.replace('"Set Document"', '"Set Snapshot"')
    text = text.replace('vec!["setDocument"', 'vec!["setSnapshot"')
    text = text.replace('doc.projection', 'doc.snapshot')
    text = text.replace('cfg.projection', 'cfg.snapshot')
    text = text.replace('app.projection()', 'app.snapshot()')
    # Replace remaining bare Document type refs cautiously
    text = re.sub(r'(?<![A-Za-z])Document(?![A-Za-z])', '%sSnapshot' % prefix, text)
    text = text.replace('%sSnapshotApp' % prefix, 'DocumentApp')
    text = text.replace('Vcs%sSnapshotApp' % prefix, 'VcsDocumentApp')
    text = text.replace('%sSnapshotView' % prefix, 'DocumentView')
    text = text.replace('create_%sSnapshot_envelope' % prefix, 'create_document_envelope')
    text = text.replace('%sSnapshotStore' % prefix, 'DocumentStore')
    text = text.replace('%sSnapshotCommand' % prefix, 'DocumentCommand') if False else text
    # Fix over-replacement of Document in comments/strings carefully later if needed
    path.write_text(text)
    for sub in ['🎮️commands/🧮️evaluate/🦀️component.rs', '🎮️commands/☑️selected-check/🦀️component.rs']:
        p = NORM / ('🎛️apps/%s' % folder) / sub
        if not p.exists():
            continue
        mechanical_rename_rs(p, key, prefix)
        t = p.read_text()
        t = t.replace('commit_document(', 'commit_snapshot(')
        if 'evaluate' in sub:
            t = re.sub(
                r'commit_snapshot\((doc\.snapshot\.clone\(\)), "evaluate"\)',
                r'commit_snapshot(%sMutation::SetSnapshot { snapshot: \1 }, "evaluate")' % prefix,
                t,
            )
        p.write_text(t)
    print('patched app', key)

def patch_app_surface():
    path = APP_SURFACE
    text = path.read_text()
    text = text.replace('cfg.projection.', 'cfg.snapshot.')
    text = text.replace('doc.projection', 'doc.snapshot')
    if 'fn commit_snapshot' not in text:
        helper = (
            '\n/// 📤️ Commit a typed document mutation (typically `XMutation::SetSnapshot { snapshot }`).\n'
            'pub fn commit_snapshot<M>(mutation: M, description: &str) -> Result<Emit<M, crate::config::NormConfigMutation>, Fault> {\n'
            '    Ok(Emit::commit(vec![mutation], description))\n'
            '}\n\n'
        )
        needle = (
            'pub fn commit_document<D>(document: D, description: &str) -> Result<Emit<crate::document::SetDocumentMutation<D>, crate::config::NormConfigMutation>, Fault> {\n'
            '    Ok(Emit::commit(vec![crate::document::SetDocumentMutation::SetDocument { document }], description))\n'
            '}'
        )
        if needle in text:
            text = text.replace(needle, helper + needle)
        else:
            text += helper
    text = text.replace(
        'pub fn projection<\'a, D>(doc: &\'a DocumentView<\'_, D>) -> &\'a D {\n    doc.projection\n}',
        'pub fn projection<\'a, D>(doc: &\'a DocumentView<\'_, D>) -> &\'a D {\n    doc.snapshot\n}',
    )
    # also handle already partially updated
    text = text.replace('doc.projection', 'doc.snapshot')
    path.write_text(text)
    print('patched app_surface')

def patch_leaf_files(key, folder, prefix):
    art = NORM / ('🗿️artifacts/%s' % folder)
    for path in art.rglob('🦀️component.rs'):
        mechanical_rename_rs(path, key, prefix)
    patch_op(key, folder, prefix)

def ensure_register_schema_all():
    for key, folder, prefix in ARTIFACTS:
        path = NORM / ('🗿️artifacts/%s/⚙️engine/🦀️component.rs' % folder)
        text = path.read_text()
        if 'fn register_artifact_schema' in text:
            if 'register_artifact_schema();' not in text.split('register_pilot_languages',1)[-1][:200]:
                text = text.replace(
                    'pub fn register_pilot_languages() {',
                    'pub fn register_pilot_languages() {\n    register_artifact_schema();',
                    1,
                )
                path.write_text(text)
            continue
        block = (
            '\n\n//#region 🔖️SchemaRegistry\n'
            'use std::sync::{Mutex, OnceLock};\n\n'
            'static SCHEMA_REGISTRY: OnceLock<Mutex<schema::ArtifactSchemaRegistry>> = OnceLock::new();\n\n'
            '/// 📌️ Registers the fifteen handcrafted schema leaves for `s.norm.%s`.\n' % key
            + 'pub fn register_artifact_schema() {\n'
            '    let registry = SCHEMA_REGISTRY.get_or_init(|| Mutex::new(schema::ArtifactSchemaRegistry::new()));\n'
            '    registry\n'
            '        .lock()\n'
            '        .expect("schema registry")\n'
            '        .register(crate::artifacts::%s::schema::%s_artifact_schema_descriptor());\n' % (key, key) +
            '}\n'
            '//#endregion 🔖️SchemaRegistry\n'
        )
        text += block
        text = text.replace(
            'pub fn register_pilot_languages() {',
            'pub fn register_pilot_languages() {\n    register_artifact_schema();',
            1,
        )
        path.write_text(text)
        print('added schema registry', key)

def migrate_din_iso(key, folder, prefix):
    ensure_set_snapshot_mutation_files(key, folder, prefix)
    ensure_artifact_set_snapshot(key, folder, prefix)
    patch_engine(key, folder, prefix)
    patch_leaf_files(key, folder, prefix)
    migrate_app_command(key, folder, prefix)
    patch_app_root(key, folder, prefix)
    print('migrated din-iso', key)

def migrate_partial_apps():
    for key, folder, prefix in ARTIFACTS:
        if key not in NEEDS_APP or key in DIN_ISO:
            continue
        migrate_app_command(key, folder, prefix)
        patch_app_root(key, folder, prefix)
        print('migrated partial app', key)

def main():
    patch_glue()
    patch_cargo()
    patch_index()
    patch_setup()
    patch_app_surface()
    fix_din4108_layer_list()
    for key, folder, prefix in ARTIFACTS:
        if key in DIN_ISO:
            migrate_din_iso(key, folder, prefix)
        else:
            ensure_artifact_set_snapshot(key, folder, prefix)
    migrate_partial_apps()
    ensure_register_schema_all()
    print('DONE')

if __name__ == '__main__':
    main()
