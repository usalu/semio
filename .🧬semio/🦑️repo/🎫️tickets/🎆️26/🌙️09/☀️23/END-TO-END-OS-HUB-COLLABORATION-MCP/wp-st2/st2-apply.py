#!/usr/bin/env python3
"""🚢️ ST1 apply: per-family stdio components (Option A) — new family packages + anchored edits of the tree.

usage: python3 st2-apply.py [--dry-run | --write] [--root <repo root>] [--part code|r10|all]
  --part code (ST2, compile-atomic): family packages (Rust + Cargo + 📜️script.ts), stdio assembly/laws/scripts, root Cargo
  workspace rows, deployment catalog, play panes. --part r10 (R10's serialized window-3 pass, after `code`): taxonomy,
  workspace-contract counts, root 📜️script.ts policy row, every 📋️project.json (the nine new ones + the composition move with
  its referrers) and the launch seed/json gate rows. --part all = both (overlay proofs).
  --dry-run (default) re-reads every target, proves every anchor matches exactly once, and writes the unified diffs to
  `generated/st1-dry.diff`; nothing in the tree changes. --write applies the same plan (all-or-nothing: every edit is
  computed in memory first). Inputs: `plan.json` + `payload/` from `st1-gen.py` (run it first on the same tree).

After --write (window 3): `bun nx run @semio-tech/plugin-registry:generate` (registry + `.vscode/launch.json` dev
launchers of the 79 new playground rows), then the native checks listed in `📓️wp-st1.md`.
"""
import argparse
import difflib
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
STDIO = "✏️s/🔌️plugins/🗄️stdio"
TAXONOMY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
WORKSPACE_CONTRACT = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts"
CATALOG = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json"
PLAY_RUNTIME = "🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json"
PLAY_COVERAGE = "🏢️semio-tech/🎡️play/🧪️tests/🧪️playpanecoverage/🟦️.ts"
JCO_DIST = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules"


class Refused(Exception):
    pass


def once(text, old, label):
    count = text.count(old)
    if count != 1:
        raise Refused(f"{label}: anchor found {count}× (expected exactly once)")
    return text.index(old)


def replace_once(text, old, new, label):
    once(text, old, label)
    return text.replace(old, new, 1)


def remove_span(text, start_anchor, end_marker, label, keep_end=False):
    start = once(text, start_anchor, label)
    end = text.index(end_marker, start + len(start_anchor))
    end = end if keep_end else end + len(end_marker)
    return text[:start] + text[end:]


def insert_after(text, anchor, insertion, label):
    index = once(text, anchor, label) + len(anchor)
    return text[:index] + insertion + text[index:]


def block_end(text, start, label):
    """Index just past the `\n    },\n` that closes the 4-space-indented JSON object starting at `start`."""
    end = text.find("\n    },\n", start)
    if end < 0:
        raise Refused(f"{label}: object end not found")
    return end + len("\n    },\n")


R10_PATHS = {TAXONOMY, WORKSPACE_CONTRACT, "📜️script.ts", ".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"}


def r10_owned(rel):
    """🗂️ Whether R10's serialized window-3 pass lands this path: taxonomy + its count laws, the root policy row, every
    `📋️project.json`, the launch rows and the `🧩️composition` → `🏘️composition` move with all its referrers."""
    return rel in R10_PATHS or rel.endswith("📋️project.json") or rel in COMPOSITION_REFERRERS or rel.startswith(f"{STDIO}/🧩️composition")


class Plan:
    def __init__(self, root, part):
        self.root = root
        self.part = part
        self.edits = {}
        self.news = {}
        self.renames = []
        self.problems = []

    def wants(self, rel):
        return self.part == "all" or (self.part == "r10") == r10_owned(rel)

    def read(self, rel):
        if rel in self.edits:
            return self.edits[rel][1]
        with open(os.path.join(self.root, rel), encoding="utf-8") as handle:
            return handle.read()

    def edit(self, rel, transform):
        if not self.wants(rel):
            return
        before = self.edits[rel][0] if rel in self.edits else self.read(rel)
        try:
            after = transform(self.read(rel))
        except Refused as error:
            self.problems.append(str(error))
            return
        if after == self.read(rel):
            self.problems.append(f"{rel}: edit changes nothing (already applied?)")
            return
        self.edits[rel] = (before, after)

    def new(self, rel, content):
        if not self.wants(rel):
            return
        if os.path.lexists(os.path.join(self.root, rel)):
            raise Refused(f"{rel}: new file already exists")
        self.news[rel] = content

    def rename_dir(self, source, target, expected_files):
        if not self.wants(source):
            return
        if not os.path.isdir(os.path.join(self.root, source)):
            raise Refused(f"{source}: rename source missing")
        if os.path.lexists(os.path.join(self.root, target)):
            raise Refused(f"{target}: rename target exists")
        files = sorted(os.path.relpath(os.path.join(dirpath, name), os.path.join(self.root, source)) for dirpath, _dirs, names in os.walk(os.path.join(self.root, source)) for name in names if name != ".DS_Store")
        if files != sorted(expected_files):
            raise Refused(f"{source}: holds {files}, expected {sorted(expected_files)}")
        self.renames.append((source, target, files))


#region stdio plugin assembly
def stdio_plugin_rs(text, plan_data):
    text = remove_span(text, "// 🗃️ Closed runtime app fleet for every stdio editor and viewer surface — the library fleet native hosts\n// and tests assemble (`full-app-catalog`).\n#[cfg(feature = \"full-app-catalog\")]\ndyn_enum_close! {\n", "\n    }\n}\n\n", "stdio plugin: library enum")
    text = replace_once(
        text,
        "// 🗃️ Closed runtime app fleet the SHIPPED component assembles: the nine text/data document subsets, 18 apps.\n// Every app monomorphises the whole app machinery and every registered app is live code in the component, so\n// the 176-app library fleet above cannot be the component (single-CGU rustc 85 GB, `wasm-component-ld`'s\n// 1 000 000-function ceiling).\n#[cfg(not(feature = \"full-app-catalog\"))]\ndyn_enum_close! {\n    pub enum StdioApps: PluginApp {\n",
        "dyn_enum_close! {\n    /// 🗃️ Closed runtime app fleet of the stdio component: the nine text/data document subsets, 18 apps. Every app\n    /// monomorphises the whole app runtime and is live code inside its component, so the other 79 subsets ship in their\n    /// family components (`🧩️extensions/*`, whole artifact kinds each) — see `🧪️tests/🚢️shipped-fleet`.\n    pub enum StdioApps: PluginApp {\n",
        "stdio plugin: shipped enum",
    )
    text = replace_once(
        text,
        "/// 🧾️ Builds all stdio definitions before the typed library assembly boundary. `.activation(…)`/\n/// `.execution(…)`/`.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M0,\n/// `📓️design-abi.md` §3/§6, following the `✏️s/🔌️plugins/🗒️note` E2 proof migration's shape): stdio\n/// owns 36 well-known file-format artifact kinds (see `//#region 🔖️Descriptor` below), so the host\n/// activates one `stdio` actor instance whenever any one of them is opened; the actor runs\n/// `Isolated` (no publisher trust assumed beyond the sandbox default, same as every other\n/// migrated plugin so far); and it asks the broker for document write access, because every one\n/// of its ~90 registered editors persists mutations back to whichever of these formats is open.\n",
        "/// 🧾️ Builds all stdio definitions before the typed library assembly boundary: stdio declares and codec-owns all 36\n/// well-known file-format artifact kinds, but activates only on the seven kinds its own apps open (csv, tsv, txt, json,\n/// xml, md, html) — every other kind activates the family package that ships its editors and viewers and depends on\n/// `stdio`, so exactly one catalog row claims each kind (`📓️design-abi.md` §2/§3). The actor runs `Isolated` and asks\n/// the broker for document write access, because its editors persist mutations back to the open document.\n",
        "stdio plugin: plugin() docstring",
    )
    text = replace_once(
        text,
        "    // 🚀 Ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M0 (`📓️design-abi.md` §3/§6): one\n    // `on-artifact-kind:` activation event per artifact kind this crate genuinely owns — every\n    // package-local `semio_s_artifact_stdio_<fmt>::artifact_kind()` function (36 formats: image/\n    // audio/video/text/data/document/geometry), each read via its own function rather than a\n    // hardcoded string so this list can never silently drift from the real declarations above.\n",
        "",
        "stdio plugin: activation comment",
    )
    family_formats = [fmt for family in plan_data["families"] for fmt in family["formats"]]
    for fmt in family_formats:
        text = replace_once(text, f"    builder = builder.activation(ActivationEvent::OnArtifactKind {{ kind: semio_s_artifact_stdio_{fmt}::artifact_kind().id }});\n", "", f"stdio plugin: activation {fmt}")
    for fmt in plan_data["baseFormats"]:
        once(text, f"    builder = builder.activation(ActivationEvent::OnArtifactKind {{ kind: semio_s_artifact_stdio_{fmt}::artifact_kind().id }});\n", f"stdio plugin: base activation {fmt}")
    text = replace_once(
        text,
        "        reason: \"persist editor mutations back to whichever of stdio's 36 owned file-format artifacts (image/audio/video/text/data/document/geometry) is currently open\".into(),\n",
        "        reason: \"persist csv/tsv/txt/json/xml/md/html editor edits back to the open stdio document\".into(),\n",
        "stdio plugin: capability reason",
    )
    text = remove_span(text, "/// 🗃️ Registers all 88 editor/viewer subsets — the library fleet (`full-app-catalog`).\n#[cfg(feature = \"full-app-catalog\")]\nfn register_apps(", "\n    builder\n}\n\n", "stdio plugin: library register_apps")
    text = replace_once(
        text,
        "/// 📄️ Registers the shipped component fleet: csv/tsv/txt/json(any, i-json)/xml(any, valid)/md/html — the nine\n/// text and data document subsets, 18 apps; the plugin still declares and codec-owns all 36 stdio artifact kinds.\n#[cfg(not(feature = \"full-app-catalog\"))]\nfn register_apps(",
        "/// 📄️ Registers the stdio component fleet: csv/tsv/txt/json(any, i-json)/xml(any, valid)/md/html — the nine text and\n/// data document subsets, 18 apps; the plugin still declares and codec-owns all 36 stdio artifact kinds.\nfn register_apps(",
        "stdio plugin: shipped register_apps",
    )
    if "full-app-catalog" in text:
        raise Refused("stdio plugin: full-app-catalog still referenced after the edit")
    return text
#endregion


#region stdio manifest
def stdio_cargo(text, plan_data):
    text = replace_once(
        text,
        "# 🧩️ What the SHIPPED wasm component assembles: all 36 artifact kinds and codecs, plus the nine\n# text/data document app subsets. Inside its own component every registered app is live code, so the\n# full fleet blows `wasm-component-ld`'s 1 000 000-function ceiling — see `full-app-catalog`.\n",
        "# 🧩️ What the stdio wasm component assembles: all 36 artifact kinds and codecs, plus the nine text/data document app\n# subsets. Every other subset ships in its family component (`🧩️extensions/*`): inside one component every registered\n# app is live code, and the 176-app fleet blows `wasm-component-ld`'s 1 000 000-function ceiling.\n",
        "stdio Cargo: component-app-assembly comment",
    )
    start = once(text, "# 🗃️ Every editor/viewer subset stdio owns (88 apps). Library-only: it cannot be linked into a component.\nfull-app-catalog = [", "stdio Cargo: full-app-catalog")
    end = text.index("\n", text.index("full-app-catalog = [", start)) + 1
    text = text[:start] + text[end:]
    deps = "".join(f"{family['crate']} = {{ workspace = true }}\n" for family in plan_data["families"])
    text = insert_after(
        text,
        "[dev-dependencies]\nsemio-framework-async-macros = { workspace = true }\nsemio-framework-plugin = { workspace = true, features = [\"artifact-app-testing\"] }\n",
        "# 🧾️ The family components, linked as libraries (their workspace rows switch `plugin-root` off) so the shipped-fleet\n# census law can describe every stdio package and the editor catalogue reaches every editor.\n" + deps,
        "stdio Cargo: dev-dependencies",
    )
    text = replace_once(text, "[[test]]\nname = \"editor_catalog\"\npath = \"../../🧪️tests/✏️editor-catalog/🦀️.rs\"\nrequired-features = [\"full-app-catalog\"]\n", "[[test]]\nname = \"editor_catalog\"\npath = \"../../🧪️tests/✏️editor-catalog/🦀️.rs\"\n", "stdio Cargo: editor_catalog test")
    if "full-app-catalog" in text:
        raise Refused("stdio Cargo: full-app-catalog still referenced after the edit")
    return text
#endregion


#region stdio laws
def shipped_fleet_rs(plan_data):
    """🛡️ The census laws for the ten stdio packages plus LB2 p6's schema-publication law widened to every package's editors."""
    return shipped_fleet_census_rs(plan_data).rstrip("\n") + "\n\n" + open(os.path.join(HERE, "st2-laws", "shipped-fleet-schema-law.rs"), encoding="utf-8").read()


def shipped_fleet_census_rs(plan_data):
    rows = ["        ShippedPackage { id: \"stdio\", manifest: include_str!(\"../../📦️packages/🦀️rust/Cargo.toml\"), descriptor: describe(semio_s_plugin_stdio::plugin()) },"]
    for family in plan_data["families"]:
        rows.append(f"        ShippedPackage {{ id: \"{family['id']}\", manifest: include_str!(\"../../🧩️extensions/{family['dir']}/📦️packages/🦀️rust/Cargo.toml\"), descriptor: describe({family['lib']}::plugin()) }},")
    return f"""//! 🛡️ Every stdio package ships a bounded, declared app fleet, and together the stdio packages ship every stdio app
//! and open every stdio artifact kind exactly once.

use semio_framework::{{AppRole, PackageDescriptor}};
use semio_framework_plugin::kernel::ActivationEvent;
use semio_framework_plugin::plugin_runtime::{{install_plugin_bundle_result, PluginRuntime}};
use semio_framework_plugin::{{Plugin, PluginApp, PluginAssemblyError}};
use semio_s_artifact_stdio_contract::editing::SNAPSHOT_EDIT_ACTION_IDS;
use std::collections::{{BTreeMap, BTreeSet}};
use std::sync::OnceLock;

/// 📏️ Apps one stdio component may assemble. Every registered app monomorphises the whole app runtime
/// (`VcsArtifactApp<EditorApp<E>/ViewerApp<V>, Members>`, its retained tool-job factories and snapshot-edit factory) and
/// is live code inside its component: all 176 stdio apps as ONE component drove the wasm-dev rustc to 85 GB in one codegen
/// unit and `wasm-component-ld` refused it ("functions count exceeds limit of 1000000", chain b3 run 2, 2026-09-27). A
/// package ships whole artifact kinds, because the host resolves who opens a kind per kind (`on-artifact-kind:`), never per
/// subset — so the largest kind, `s.stdio.semio` (19 subsets, 38 apps), sets the bound; ST1's wasm32 measurement of
/// `stdio-semio` is recorded in ticket 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP `📓️wp-st1.md`.
const SHIPPED_APP_CEILING: usize = {plan_data['ceiling']};

/// 🧾️ One stdio package as it ships: the Cargo manifest whose playground rows declare its shipped editors, and the
/// descriptor its own component describes.
struct ShippedPackage {{
    id: &'static str,
    manifest: &'static str,
    descriptor: PackageDescriptor,
}}

/// 🛂️ Describes one assembled bundle through the runtime path its component's `describe` export takes.
fn describe<PA: PluginApp>(bundle: Result<Plugin<PA>, PluginAssemblyError>) -> PackageDescriptor {{
    let runtime = PluginRuntime::<PA>::new();
    install_plugin_bundle_result(&runtime, bundle);
    let bytes = semio_framework_plugin::app::resolve_ready(semio_framework_plugin::describe::describe_plugin(&runtime));
    semio_framework::from_dsl_value(semio_framework_os_kernel::pack_rt::decode_wire_value(&bytes).expect("descriptor bytes")).expect("strict descriptor")
}}

/// 📦️ The stdio component and its nine family components, each described once.
fn packages() -> &'static [ShippedPackage] {{
    static PACKAGES: OnceLock<Vec<ShippedPackage>> = OnceLock::new();
    PACKAGES.get_or_init(|| {{
        vec![
{chr(10).join(rows)}
        ]
    }})
}}

/// 📋️ The `app` of every `[[package.metadata.semio.playground]]` row of one manifest — one row per shipped editor.
fn declared_editors(manifest: &str) -> BTreeSet<String> {{
    manifest
        .split("[[package.metadata.semio.playground]]")
        .skip(1)
        .filter_map(|row| row.lines().find_map(|line| line.strip_prefix("app = \\"").and_then(|rest| rest.strip_suffix('"')).map(str::to_string)))
        .collect()
}}

/// 🧫️ The neutral editor catalogue: every stdio editor app and the number of stdio formats.
fn fixture() -> serde_json::Value {{
    serde_json::from_str(include_str!("../../🧫️fixtures/✏️editor-catalog/🔣️.json")).expect("neutral editor catalogue fixture")
}}

/// 🗂️ The app ids one descriptor ships in one role.
fn app_ids(descriptor: &PackageDescriptor, role: AppRole) -> BTreeSet<String> {{
    descriptor.manifest.apps.iter().filter(|app| app.role == role).map(|app| app.id.clone()).collect()
}}

/// 🎬️ The artifact kinds one descriptor activates on.
fn activated_kinds(descriptor: &PackageDescriptor) -> BTreeSet<String> {{
    descriptor
        .activation_events
        .iter()
        .map(|event| match event {{
            ActivationEvent::OnArtifactKind {{ kind }} => kind.clone(),
            other => panic!("{{}} activates on {{other:?}}, a stdio package activates on artifact kinds only", descriptor.manifest.plugin_id),
        }})
        .collect()
}}

/// 🛡️ LAW: each package registers exactly its playground-declared editors plus one viewer per shipped dialect, stays
/// under [`SHIPPED_APP_CEILING`], and every family depends on exactly `stdio` — so widening a component is a declared,
/// reviewed change of its playground rows, never a feature edit that silently ships another package's fleet.
#[test]
fn every_stdio_package_ships_exactly_its_declared_bounded_fleet() {{
    for package in packages() {{
        let descriptor = &package.descriptor;
        assert_eq!(descriptor.package_id, format!("semio:{{}}", package.id), "{{}} package identity", package.id);
        assert_eq!(descriptor.manifest.plugin_id, package.id, "{{}} plugin identity", package.id);
        let declared = declared_editors(package.manifest);
        assert_eq!(app_ids(descriptor, AppRole::Editor), declared, "{{}} ships exactly its playground-declared editors", package.id);
        let viewers = app_ids(descriptor, AppRole::Viewer).iter().map(|id| id.trim_end_matches("#viewer").to_string()).collect::<BTreeSet<_>>();
        assert_eq!(viewers, declared.iter().map(|id| id.trim_end_matches("#editor").to_string()).collect::<BTreeSet<_>>(), "{{}} ships the viewer of every shipped dialect, and only those", package.id);
        assert!(descriptor.manifest.apps.len() <= SHIPPED_APP_CEILING, "{{}} ships {{}} apps over the per-component ceiling of {{SHIPPED_APP_CEILING}}", package.id, descriptor.manifest.apps.len());
        let dependencies = descriptor.manifest.dependencies.iter().map(|dependency| dependency.plugin_id.as_str()).collect::<Vec<_>>();
        assert_eq!(dependencies, if package.id == "stdio" {{ Vec::<&str>::new() }} else {{ vec!["stdio"] }}, "{{}} runtime dependencies", package.id);
    }}
}}

/// 🧮️ LAW: the stdio packages together ship every editor of the neutral catalogue and its viewer, each in exactly one
/// package, and every shipped editor exposes the complete details window and every snapshot edit action.
#[test]
fn the_stdio_packages_ship_every_stdio_app_exactly_once() {{
    let fixture = fixture();
    let mut owners: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for package in packages() {{
        for app in &package.descriptor.manifest.apps {{
            owners.entry(app.id.as_str()).or_default().push(package.id);
        }}
    }}
    let repeated = owners.iter().filter(|(_, packages)| packages.len() > 1).collect::<Vec<_>>();
    assert!(repeated.is_empty(), "apps shipped by more than one package: {{repeated:?}}");
    let expected = fixture["editorApps"].as_array().unwrap().iter().map(|app| app.as_str().unwrap().to_string()).collect::<BTreeSet<_>>();
    assert_eq!(expected.len(), fixture["editorCount"].as_u64().unwrap() as usize);
    assert_eq!(owners.keys().filter(|id| id.ends_with("#editor")).map(|id| id.to_string()).collect::<BTreeSet<_>>(), expected, "every catalogue editor ships");
    assert_eq!(owners.keys().filter(|id| id.ends_with("#viewer")).map(|id| id.trim_end_matches("#viewer").to_string()).collect::<BTreeSet<_>>(), expected.iter().map(|id| id.trim_end_matches("#editor").to_string()).collect::<BTreeSet<_>>(), "every catalogue dialect ships its viewer");
    for package in packages() {{
        for app in package.descriptor.manifest.apps.iter().filter(|app| app.role == AppRole::Editor) {{
            let details = app.window_kinds.iter().find(|window| window.id == fixture["detailsWindow"].as_str().unwrap()).unwrap_or_else(|| panic!("{{}} has no complete details window", app.id));
            for id in SNAPSHOT_EDIT_ACTION_IDS {{
                assert!(details.actions.iter().any(|action| action.id == *id), "{{}} details window does not expose {{id}}", app.id);
            }}
        }}
    }}
}}

/// 🎬️ LAW: every stdio artifact kind is opened by exactly one package — the one shipping its apps activates on it, and
/// no other package does — so the catalog's `on-artifact-kind:` rows resolve every stdio document to the package that
/// can open every one of its subsets.
#[test]
fn every_stdio_kind_is_opened_by_exactly_one_package() {{
    let fixture = fixture();
    let mut openers: BTreeMap<String, Vec<&str>> = BTreeMap::new();
    for package in packages() {{
        let activated = activated_kinds(&package.descriptor);
        let opened = package.descriptor.manifest.apps.iter().map(|app| app.dialect.artifact_kind.clone()).collect::<BTreeSet<_>>();
        assert_eq!(activated, opened, "{{}} activates on exactly the kinds its apps open", package.id);
        for kind in activated {{
            openers.entry(kind).or_default().push(package.id);
        }}
    }}
    let repeated = openers.iter().filter(|(_, packages)| packages.len() > 1).collect::<Vec<_>>();
    assert!(repeated.is_empty(), "kinds opened by more than one package: {{repeated:?}}");
    let expected = fixture["editorApps"].as_array().unwrap().iter().map(|app| app.as_str().unwrap().split('@').next().unwrap().to_string()).collect::<BTreeSet<_>>();
    assert_eq!(expected.len(), fixture["formatCount"].as_u64().unwrap() as usize);
    assert_eq!(openers.keys().cloned().collect::<BTreeSet<_>>(), expected, "every stdio kind is opened");
}}
"""


def editor_catalog_rs(text):
    text = replace_once(text, "    println!(\"[DEBUG] editor={} native replay, retained edit/undo/redo, and artifact/source reopen passed\", definition.id);\n", "", "editor catalog: DEBUG line")
    text = remove_span(editor_catalog_details_rs(text), "#[test]\nfn assembled_plugin_exposes_every_editable_artifact() {\n", "\n}\n\n", "editor catalog: assembled plugin law")
    text = replace_once(text, "async fn assert_editor<E: ArtifactEditor + SnapshotEditingEditor>(definition: AppDefinition) {\n    let fixture = fixture();\n", "async fn assert_editor<E: ArtifactEditor + SnapshotEditingEditor>(definition: AppDefinition) {\n    stdio_packages_assembled();\n    let fixture = fixture();\n", "editor catalog: assemble before driving an editor")
    return replace_once(text, "fn fixture() -> serde_json::Value {\n", open(os.path.join(HERE, "st2-laws", "editor-catalog-assembly.rs"), encoding="utf-8").read() + "fn fixture() -> serde_json::Value {\n", "editor catalog: package assembly helper")


def editor_catalog_details_rs(text):
    """✏️ The SDK keeps actions a window kind explicitly owns out of the app roster, and the snapshot details window owns the six
    snapshot edit actions, so the law reads them where they are declared."""
    return replace_once(
        text,
        """    assert!(definition.window_kinds.iter().any(|kind| kind.id == details), "{} needs editable details", definition.id);
    for row in fixture["actions"].as_array().unwrap() {
        let id = row["id"].as_str().unwrap();
        let action = definition.actions.iter().find(|action| action.id == id).unwrap_or_else(|| panic!("{} is missing {id}", definition.id));
""",
        """    let details_kind = definition.window_kinds.iter().find(|kind| kind.id == details).unwrap_or_else(|| panic!("{} needs editable details", definition.id));
    for row in fixture["actions"].as_array().unwrap() {
        let id = row["id"].as_str().unwrap();
        let action = details_kind.actions.iter().find(|action| action.id == id).unwrap_or_else(|| panic!("{} details window is missing {id}", definition.id));
""",
        "editor catalog: the details window owns the snapshot edit actions",
    )


MIGRATED = "semio_framework_plugin::InteractiveJobClassification::Migrated"


def wav_extra_actions_rs(text):
    """🔊️ wav's appended Main-window table verbs are retained routes (bounded tool publication contracts for exactly these ids)
    declared after the kit scaffold stamped its own rows, so they are stamped `Migrated` where they are declared."""
    text = replace_once(text, "pub fn extra_actions() -> Vec<ActionDefinition> {\n    vec![\n", "pub fn extra_actions() -> Vec<ActionDefinition> {\n    [\n", "wav extra actions: head")
    return replace_once(
        text,
        "            .in_palette(false),\n    ]\n}\n\nfn data_payload_len",
        f"            .in_palette(false),\n    ]\n    .map(|mut action| {{\n        action.semantics.execution.interactive_job = {MIGRATED};\n        action\n    }})\n    .into()\n}}\n\nfn data_payload_len",
        "wav extra actions: tail",
    )


def set_vertex_action_rs(text, label, tail):
    """🔺️ semio mesh/brep `set-vertex` is a retained route (its own bounded tool work) declared both on the Main window and on
    the app, so the one declaration is stamped `Migrated`."""
    text = replace_once(text, "pub fn set_vertex_action() -> ActionDefinition {\n    ActionDefinition::bounded_catalog(\"set-vertex\"", "pub fn set_vertex_action() -> ActionDefinition {\n    let mut action = ActionDefinition::bounded_catalog(\"set-vertex\"", f"{label}: set_vertex_action head")
    return replace_once(text, tail, tail[:-len("\n}\n")] + f";\n    action.semantics.execution.interactive_job = {MIGRATED};\n    action\n}}\n", f"{label}: set_vertex_action tail")


def stdio_script_ts(text):
    """🧾️ The Codex peer's editor-catalog gate (36/36 formats + one playground row per editor, 19:11 shape with the
    `shipping` flag and the full-catalog component check) becomes one union contract over the ten stdio packages, and the
    component check links and validates every stdio package's own component."""
    start = once(text, "async function testEditorCatalogContract(packageRoot: string, shipping = false): Promise<void> {\n", "stdio script: editor catalogue contract")
    head_end = text.index("  const manifest = Bun.TOML.parse(readFileSync(join(packageRoot, \"Cargo.toml\"), \"utf8\"))", start)
    body_end = text.index("\n}\n", head_end) + len("\n}\n")
    head = text[start:head_end].replace("async function testEditorCatalogContract(packageRoot: string, shipping = false): Promise<void> {\n", "async function testEditorCatalogContract(packageRoot: string): Promise<void> {\n", 1)
    text = replace_once(text, "/** 🧪️ Checks the neutral editor acceptance fixture with an independent JSON Schema validator. */\nasync function testEditorCatalogContract(", "/** 🧪️ Checks the neutral editor acceptance fixture with an independent JSON Schema validator: every stdio editor ships in\n * exactly one stdio package (the stdio component or one `🧩️extensions` family), the packages together ship every catalogue\n * format, and every editor owns exactly one launchable playground row across them. */\nasync function testEditorCatalogContract(", "stdio script: contract docstring")
    start = once(text, "async function testEditorCatalogContract(packageRoot: string, shipping = false): Promise<void> {\n", "stdio script: editor catalogue contract")
    head_end = text.index("  const manifest = Bun.TOML.parse(readFileSync(join(packageRoot, \"Cargo.toml\"), \"utf8\"))", start)
    body_end = text.index("\n}\n", head_end) + len("\n}\n")
    tail = """  const formats = new Map<string, string>();
  const apps = new Map<string, string>();
  for (const { id, manifest } of stdioPackageManifests(root)) {
    for (const format of stdioPackageEditorFormats(manifest)) {
      if (formats.has(format)) throw new Error(`${format} editors ship in both ${formats.get(format)} and ${id}`);
      formats.set(format, id);
    }
    for (const row of manifest.package.metadata.semio.playground ?? []) {
      if (apps.has(row.app)) throw new Error(`${row.app} has playground rows in both ${apps.get(row.app)} and ${id}`);
      apps.set(row.app, id);
    }
  }
  if (formats.size !== fixture.formatCount) throw new Error(`the stdio packages ship editors for ${formats.size}/${fixture.formatCount} formats`);
  if (fixture.editorApps.length !== fixture.editorCount) throw new Error("editor fixture count differs from its app identities");
  if (!isDeepStrictEqual([...apps.keys()].sort(), [...fixture.editorApps].sort())) throw new Error("each editor needs exactly one launchable playground across the stdio packages");
  console.log(`🧾️ editor catalogue fixture validated: ${roots} editors in ${new Set(apps.values()).size} packages, ${fixture.actions.length} edit operations`);
}

/** 📦️ Every stdio package manifest, parsed by Bun's TOML reader independently of the Rust census law's row scan: the
 * stdio component's own and one per family component under `🧩️extensions`. */
function stdioPackageManifests(root: string): { readonly id: string; readonly manifest: StdioPackageManifest }[] {
  const families = readdirSync(join(root, "🧩️extensions"), { withFileTypes: true }).filter((entry) => entry.isDirectory()).map((entry) => join(root, "🧩️extensions", entry.name, "📦️packages/🦀️rust/Cargo.toml"));
  return [join(root, "📦️packages/🦀️rust/Cargo.toml"), ...families.sort()].map((path) => {
    const manifest = Bun.TOML.parse(readFileSync(path, "utf8")) as StdioPackageManifest;
    return { id: manifest.package.metadata.component.package.slice("semio:".length), manifest };
  });
}

/** 🧩️ The stdio formats whose editors one package's default build assembles: `component-app-assembly` reached through its
 * default feature closure (the stdio component) or requested on the artifact dependency itself (a family component). */
function stdioPackageEditorFormats(manifest: StdioPackageManifest): readonly string[] {
  const selected = new Set<string>();
  const pending = ["default"];
  while (pending.length) {
    const feature = pending.pop()!;
    if (selected.has(feature)) continue;
    selected.add(feature);
    pending.push(...(manifest.features?.[feature] ?? []));
  }
  const prefix = "semio-s-artifact-stdio-";
  const viaFeatures = [...selected].filter((feature) => feature.startsWith(prefix) && feature.endsWith("/component-app-assembly")).map((feature) => feature.slice(prefix.length, -"/component-app-assembly".length));
  const viaDependencies = Object.entries(manifest.dependencies ?? {}).filter(([name, spec]) => name.startsWith(prefix) && typeof spec === "object" && (spec.features ?? []).includes("component-app-assembly")).map(([name]) => name.slice(prefix.length));
  return [...new Set([...viaFeatures, ...viaDependencies])];
}

/** 📜️ The Cargo manifest fields the editor catalogue contract reads from a stdio package. */
type StdioPackageManifest = {
  readonly features?: Record<string, string[]>;
  readonly dependencies?: Record<string, string | { readonly features?: readonly string[] }>;
  readonly package: { readonly name: string; readonly metadata: { readonly component: { readonly package: string }; readonly semio: { readonly playground?: readonly { readonly app: string }[] } } };
};
"""
    text = text[:start] + head + tail + text[body_end:]
    text = replace_once(text, """    await testEditorCatalogContract(this.root, rest[0] === "editor-shipping-contract");
    if (rest[0] === "editor-catalog-contract" || rest[0] === "editor-shipping-contract") return;
""", """    await testEditorCatalogContract(this.root);
    if (rest[0] === "editor-catalog-contract") return;
""", "stdio script: test router")
    check_start = once(text, "/** 🧩️ Verifies the complete editor component against the native WebAssembly parser. */\nclass EditorComponentCheckScript extends BundleScript {\n", "stdio script: component check")
    check_end = text.index("\n}\n", check_start) + len("\n}\n")
    text = text[:check_start] + """/** 🧩️ Links every stdio package's own component (stdio and each `🧩️extensions` family) with the release component
 * profile, extracts its core module with JCO and validates it against the native WebAssembly parser and the component
 * function ceiling — the measured bound each family's bounded fleet must stay under. */
class EditorComponentCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await testEditorCatalogContract(this.root);
    const outputRoot = segments[0] ?? join(cargoTargetDirectory(this.repoRoot), "stdio-editor-components");
    if (segments.length > 1 || !isAbsolute(outputRoot)) throw new Error("editor-component-check accepts one absolute output directory");
    mkdirSync(outputRoot, { recursive: true });
    const started = Date.now();
    let interrupted = false;
    const interrupt = (): void => { interrupted = true; };
    process.on("SIGINT", interrupt);
    process.on("SIGTERM", interrupt);
    const control: CatalogControl = { cancelled: () => interrupted, remainingMs: () => Math.max(0, (buildBudgetMs() || CATALOG_DEADLINE_MS) - (Date.now() - started)) };
    const env = devToolingEnv({});
    const jco = resolveWorkspaceBin("@bytecodealliance/jco", this.repoRoot);
    if (!jco) throw new Error("missing workspace component tooling");
    try {
      const packages = stdioPackageManifests(resolve(this.root, "../.."));
      for (const [index, { id, manifest }] of packages.entries()) {
        const lib = manifest.package.name.replaceAll("-", "_");
        await runControlled("cargo", ["rustc", "-p", manifest.package.name, "--profile", COMPONENT_PROFILE, "--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2"], this.repoRoot, env, control);
        const out = join(outputRoot, id);
        mkdirSync(out, { recursive: true });
        await runControlled("node", [jco, "transpile", join(cargoTargetDirectory(this.repoRoot, env), "wasm32-wasip2", COMPONENT_PROFILE, `${lib}.wasm`), "-o", out, "--name", lib, "--map", "semio:framework/pure=./pure.js", "--map", "semio:framework/host-async=./host-async.js"], this.repoRoot, env, control);
        const core = readFileSync(join(out, `${lib}.core.wasm`));
        const structure = assertComponentizableCore(core);
        console.log(`🧩️ ${index + 1}/${packages.length} ${id} component validated: ${JSON.stringify({ ...structure, coreBytes: core.byteLength })}`);
      }
    } finally {
      process.off("SIGINT", interrupt);
      process.off("SIGTERM", interrupt);
    }
  }
}
""" + text[check_end:]
    for leftover in ("shipping", "full-app-catalog", "fullCatalog", "[DEBUG] complete editor component", "[DEBUG] ${shipping"):
        if leftover in text[start:text.index("/** 📈️", start)]:
            raise Refused(f"stdio script: {leftover!r} survives the rewrite")
    return text
#endregion


#region workspace
def root_cargo(text, plan_data):
    members = "".join(f"    \"{family['owner']}/📦️packages/🦀️rust\",\n" for family in plan_data["families"])
    text = insert_after(text, "    \"✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust\",\n", members, "root Cargo: members")
    deps = "".join(f"{family['crate']} = {{ path = \"{family['owner']}/📦️packages/🦀️rust\", default-features = false }}\n" for family in plan_data["families"])
    return insert_after(text, "semio-s-plugin-space = { path = \"✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust\" }\n", deps, "root Cargo: workspace dependencies")


def catalog_json(text, plan_data):
    rows = "".join(f"    {{ \"pluginId\": \"{family['id']}\", \"directoryName\": \"{family['directoryName']}\" }},\n" for family in sorted(plan_data["families"], key=lambda family: family["id"]))
    return insert_after(text, "    { \"pluginId\": \"stdio\", \"directoryName\": \"🗄️stdio\" },\n", rows, "deployment catalog")
#endregion


#region hub publication
HUB_SCRIPT = "🌎️hub/📦️packages/🦀️rust/📜️script.ts"


def hub_script_ts(text, plan_data):
    """🌎️ The trusted-catalog publisher selects every family component like any other document-opening package: a guest
    codec closure of its own (no hub-linked registry — `stdio` keeps the linked native codecs), published right after the
    `stdio` package its exact pin names, so `--packages all` puts every stdio subset on the hub."""
    families = plan_data["families"]
    text = replace_once(
        text,
        "/** 🌎️ Every selectable top-level `s` plugin package, in publication order. */\nconst TRUSTED_BOOTSTRAP_ALL_PACKAGES = \"stdio,gis,",
        "/** 🌎️ Every selectable `s` plugin package — the top-level plugins and the stdio family components right after the `stdio`\n * package they depend on — in publication order. */\nconst TRUSTED_BOOTSTRAP_ALL_PACKAGES = \"stdio," + ",".join(family["id"] for family in families) + ",gis,",
        "hub: all packages",
    )
    rows = "".join(f'  Object.freeze({{ pluginId: "{family["id"]}", cargoPackage: "{family["crate"]}", componentPackageId: "semio:{family["id"]}", outputName: "{family["lib"]}.wasm", linkedCodecRegistry: null, opensDocuments: true }}),\n' for family in families)
    anchor = '  Object.freeze({ pluginId: "stdio", cargoPackage: "semio-s-plugin-stdio", componentPackageId: "semio:stdio", outputName: "semio_s_plugin_stdio.wasm", linkedCodecRegistry: "✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json", opensDocuments: true }),\n'
    return insert_after(text, anchor, rows, "hub: package specs")
#endregion


#region taxonomy
def taxonomy_json(text, plan_data):
    families = plan_data["families"]
    start = once(text, "    \"deployed-module-members\": {\n", "taxonomy: deployed-module-members")
    anchor = text.index("        \"🗄️stdio\",\n", start) + len("        \"🗄️stdio\",\n")
    text = text[:anchor] + "".join(f"        \"{family['directoryName']}\",\n" for family in sorted(families, key=lambda family: family["id"])) + text[anchor:]

    start = once(text, "    \"members-of-plan\": {\n", "taxonomy: members-of-plan")
    list_start = text.index("      \"memberNames\": [\n", start) + len("      \"memberNames\": [\n")
    list_end = text.index("\n      ]", list_start)
    names = [line.strip().rstrip(",").strip('"') for line in text[list_start:list_end].split("\n")]
    merged = sorted(set(names) | {family["dir"] for family in families})
    if sorted(names) != names:
        merged = names + [family["dir"] for family in families]
    text = text[:list_start] + ",\n".join(f"        \"{name}\"" for name in merged) + text[list_end:]

    for set_id in ("dev-jco-all-interfaces", "dev-jco-random-interfaces"):
        start = once(text, f"    \"{set_id}\": [\n", f"taxonomy: {set_id}")
        close = text.index("\n    ]", start)
        text = text[:close] + "".join(f",\n      \"dev-plugin-interfaces-{family['id']}\"" for family in families) + text[close:]

    def filename_contracts(family):
        base = f"{JCO_DIST}/{family['directoryName']}/{family['lib']}_component"
        label = family["label"]
        rows = [("js", ".js", f"Exact emitted {label} JavaScript companion"), ("declaration", ".d.ts", f"Exact emitted {label} TypeScript declaration"), ("wasm", ".core.wasm", f"Exact {label} core module URL emitted by JCO")]
        return "".join(f"""    "dev-plugin-component-{family['id']}-{suffix}": {{
      "pathPattern": "{base}{extension}",
      "authority": "Bytecode Alliance JCO",
      "reason": "{reason}",
      "configurability": "unconfigurable",
      "scope": {{
        "kind": "path-pattern"
      }},
      "verification": "materialized JCO component pairing",
      "expires": null
    }},
""" for suffix, extension, reason in rows)

    start = once(text, "    \"dev-plugin-component-stdio-wasm\": {\n", "taxonomy: stdio wasm filename contract")
    end = block_end(text, start, "taxonomy: stdio wasm filename contract")
    text = text[:end] + "".join(filename_contracts(family) for family in families) + text[end:]

    def directory_contract(family):
        path = f"{JCO_DIST}/{family['directoryName']}/interfaces"
        return f"""    "dev-plugin-interfaces-{family['id']}": {{
      "pathPattern": "{path}",
      "authority": "Bytecode Alliance JCO",
      "reason": "Exact emitted {family['label']} component interface owner",
      "configurability": "unconfigurable",
      "scope": {{
        "kind": "exact-path",
        "path": "{path}"
      }},
      "verification": "materialized JCO output boundaries: exact interface roster",
      "expires": null
    }},
"""

    start = once(text, "    \"dev-plugin-interfaces-stdio\": {\n", "taxonomy: stdio interfaces contract")
    end = block_end(text, start, "taxonomy: stdio interfaces contract")
    text = text[:end] + "".join(directory_contract(family) for family in families) + text[end:]

    def dispositions(family):
        return "".join(f"""    "dev-plugin-component-{family['id']}-{suffix}": {{
      "contractKind": "fixed",
      "disposition": "adapter-source",
      "validator": "package-glue",
      "authority": "{authority}",
      "verification": "materialized JCO component pairing"
    }},
""" for suffix, authority in (("js", f"JCO {family['label']} companion without purity exemption"), ("declaration", f"JCO {family['label']} declaration pairing")))

    section = once(text, "  \"packageSourceDispositions\": {\n", "taxonomy: packageSourceDispositions")
    start = text.index("    \"dev-plugin-component-stdio-declaration\": {\n", section)
    end = block_end(text, start, "taxonomy: stdio declaration disposition")
    text = text[:end] + "".join(dispositions(family) for family in families) + text[end:]
    json.loads(text)
    return text


def workspace_contract_ts(text, plan_data):
    count = len(plan_data["families"])
    text = replace_once(text, "    expect(parentIds).toHaveLength(60);\n", f"    expect(parentIds).toHaveLength({60 + count});\n", "workspace contract: interface parents")
    text = replace_once(text, "  test(\"preserves only the 60 exact compiler-linked triples\", async () => {\n", f"  test(\"preserves only the {60 + count} exact compiler-linked triples\", async () => {{\n", "workspace contract: triples title")
    return replace_once(text, "    expect(contracts).toHaveLength(180);\n", f"    expect(contracts).toHaveLength({180 + 3 * count});\n", "workspace contract: companion contracts")
#endregion


#region play
def play_runtime_json(text, plan_data):
    groups = []
    for family in plan_data["families"]:
        panes = []
        for subset in family["subsets"]:
            pane = subset["pane"]
            body = ", ".join(f"{json.dumps(key)}: {json.dumps(value, ensure_ascii=False)}" for key, value in pane.items())
            panes.append(f"        {{ {body} }}")
        groups.append(f"""    {{
      "id": "{family['group']['id']}",
      "label": "{family['group']['label']}",
      "panes": [
{(',' + chr(10)).join(panes)}
      ]
    }},
""")
    anchor_start = once(text, "      \"id\": \"documents\",\n", "play runtime: documents group")
    end = text.index("      ]\n    },\n", anchor_start) + len("      ]\n    },\n")
    text = text[:end] + "".join(groups) + text[end:]
    document = json.loads(text)
    variants = [pane["variant"] for group in document["groups"] for pane in group["panes"]]
    if len(variants) != len(set(variants)):
        raise Refused("play runtime: a pane variant repeats")
    return text


def play_coverage_ts(text):
    text = replace_once(
        text,
        """/** @emoji 🗄️ What stdio's SHIPPED component assembles: every artifact crate whose apps
 * `component-app-assembly` turns on. Its `full-app-catalog` sibling closes the same `StdioApps` enum
 * over all 88 subsets and costs ≈600 000 wasm functions, which `wasm-component-ld` refuses over
 * wasmparser's 1 000 000-function ceiling — so the shipped fleet must stay this bounded set, and
 * `default`/`plugin-root` must never reach `full-app-catalog`. */
""",
        """/** @emoji 🗄️ What stdio's OWN component assembles: every artifact crate whose apps its `component-app-assembly` turns
 * on — the nine text/data subsets. Every other stdio subset ships in its family component under `🗄️stdio/🧩️extensions`,
 * because one component closing all 176 stdio apps exceeds wasmparser's 1 000 000-function ceiling
 * (`🗄️stdio/🧪️tests/🚢️shipped-fleet`). */
""",
        "play coverage: stdio docstring",
    )
    text = replace_once(text, "      expect([...closure].filter(name => name === \"full-app-catalog\")).toEqual([]);\n", "", "play coverage: full-app-catalog expectation")
    text = replace_once(
        text,
        """/** @emoji 🗂️ The apps every plugin descriptor declares, BOTH roles, keyed by the plugin DIRECTORY that
""",
        """/** @emoji 🗂️ The apps every registry plugin's descriptor declares, BOTH roles, keyed by the crate's OWNER root
 * (`<owner>/📦️packages/🦀️rust` → `<owner>`): a family component nested under its plugin directory (`🗄️stdio/🧩️extensions/*`)
 * commits its own descriptor there, and its apps must be reachable like every other plugin's. `undefined` for an owner
 * that commits no descriptor yet. */
function descriptorAppsByOwner(repoRoot: string, rows: readonly { readonly cratePath: string }[]): ReadonlyMap<string, { readonly pluginId: string; readonly apps: readonly any[] } | undefined> {
  const byOwner = new Map<string, { readonly pluginId: string; readonly apps: readonly any[] } | undefined>();
  for (const owner of rows.map(row => dirname(dirname(row.cratePath)))) {
    const file = join(repoRoot, owner, "🔣️.json");
    if (!existsSync(file)) { byOwner.set(owner, undefined); continue; }
    const manifest = JSON.parse(readFileSync(file, "utf8")).manifest;
    byOwner.set(owner, { pluginId: manifest.pluginId, apps: manifest.apps ?? [] });
  }
  return byOwner;
}

/** @emoji 🗂️ The apps every plugin descriptor declares, BOTH roles, keyed by the plugin DIRECTORY that
""",
        "play coverage: owner descriptor reader",
    )
    text = replace_once(text, "import { join } from \"node:path\";\n", "import { dirname, join } from \"node:path\";\n", "play coverage: dirname import")
    text = replace_once(
        text,
        """  const editorAppIds = (directory: string | undefined): readonly string[] => (appsByDirectory.get(directory!)?.apps ?? []).filter((app: any) => app.role === "editor").map((app: any) => app.id as string);
  const paneDirectory = (pluginId: string): string => pluginDirectoryOfCratePath(PLUGIN_BUILD_TARGETS.find(entry => entry.pluginId === pluginId)!.cratePath)!;
""",
        """  const appsByOwner = descriptorAppsByOwner(repoRoot, PLUGIN_BUILD_TARGETS);
  const editorAppIds = (owner: string): readonly string[] => (appsByOwner.get(owner)?.apps ?? []).filter((app: any) => app.role === "editor").map((app: any) => app.id as string);
  const paneOwner = (pluginId: string): string => dirname(dirname(PLUGIN_BUILD_TARGETS.find(entry => entry.pluginId === pluginId)!.cratePath));
""",
        "play coverage: pane owner",
    )
    text = replace_once(text, "        else for (const id of editorAppIds(paneDirectory(row.pluginId))) reachable.add(id);\n", "        else for (const id of editorAppIds(paneOwner(row.pluginId))) reachable.add(id);\n", "play coverage: reachable editors")
    text = replace_once(text, "      const unreachable = [...appsByDirectory.values()].flatMap(", "      const unreachable = [...appsByOwner.values()].flatMap(", "play coverage: unreachable editors")
    text = replace_once(text, "        const apps = appsByDirectory.get(paneDirectory(row.pluginId))?.apps;\n", "        const apps = appsByOwner.get(paneOwner(row.pluginId))?.apps;\n", "play coverage: viewer apps")
    text = replace_once(text, "      const declared = PLAY_RUNTIME_TARGETS.flatMap((row: any) => (appsByDirectory.get(paneDirectory(row.pluginId))?.apps ?? [])", "      const declared = PLAY_RUNTIME_TARGETS.flatMap((row: any) => (appsByOwner.get(paneOwner(row.pluginId))?.apps ?? [])", "play coverage: declared viewers")
    if "paneDirectory" in text:
        raise Refused("play coverage: paneDirectory still referenced")
    return text
#endregion


#region composition rename and launch rows
COMPOSITION_REFERRERS = [
    f"{STDIO}/📦️packages/🟦️typescript/📋️project.json",
    f"{STDIO}/📦️packages/🟦️typescript/📜️script.ts",
    f"{STDIO}/🧪️tests/🧩️composition-consumption/🟦️.ts",
    f"{STDIO}/🧫️fixtures/🏃️command-ownership/🔣️.json",
]


def composition_referrer(text, rel):
    count = text.count("🧩️composition/") - text.count("🧪️tests/🧩️composition-consumption")
    if count < 1:
        raise Refused(f"{rel}: no 🧩️composition/ reference")
    return re.sub(r"(?<![\w-])🧩️composition/(?=🏃️commands|🏗️build)", "🏘️composition/", text)


LAUNCH_CATALOG_OLD = """      "name": "⚖️gate🗄️stdio✏️catalog🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:test -- --features full-app-catalog --test editor_catalog",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.91
      }
    },
"""
LAUNCH_CATALOG_NEW = """      "name": "⚖️gate🗄️stdio✏️catalog🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:test -- --test editor_catalog",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.91
      }
    },
    {
      "name": "⚖️gate🗄️stdio🚢️shipped-fleet🦀️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:test -- --test shipped_fleet",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.915
      }
    },
"""
LAUNCH_RETIRED = [
    """    {
      "name": "⚖️gate🗄️stdio✏️shipping🟦️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:test -- editor-shipping-contract",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.99
      }
    },
""",
    """    {
      "name": "⚖️gate🗄️stdio✏️catalog🌐️",
      "type": "node-terminal",
      "request": "launch",
      "command": "bun nx run @semio-tech/stdio-plugin:editor-component-check -- --full-catalog",
      "cwd": "${workspaceFolder}",
      "presentation": {
        "group": "4_gate",
        "order": 407.995
      }
    },
""",
]


def launch_rows(text, rel):
    text = replace_once(text, LAUNCH_CATALOG_OLD, LAUNCH_CATALOG_NEW, f"{rel}: editor catalog gate row")
    for row in LAUNCH_RETIRED:
        text = replace_once(text, row, "", f"{rel}: retired full-catalog row")
    return text


POLICY_ANCHOR = '  "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions": "Extension-crate axis (role=extension, extends=sourcing, 3 crates) — pending the §6 ruling in 📓️w0-census.md.",\n'
POLICY_ROW = '  "✏️s/🔌️plugins/🗄️stdio/🧩️extensions": "Family-component axis (role=plugin, depends-on=stdio, 9 crates): each ships the apps of whole stdio artifact kinds as its own component (ST1, one stdio component cannot link all 176 apps).",\n'
#endregion


def build(root, part):
    plan_data = json.load(open(os.path.join(HERE, "plan.json"), encoding="utf-8"))
    plan = Plan(root, part)
    payload = os.path.join(HERE, "payload")
    for dirpath, _dirs, names in os.walk(payload):
        for name in names:
            source = os.path.join(dirpath, name)
            with open(source, encoding="utf-8") as handle:
                plan.new(os.path.relpath(source, payload), handle.read())
    plan.edit(f"{STDIO}/🔌️plugin/🦀️.rs", lambda text: stdio_plugin_rs(text, plan_data))
    plan.edit(f"{STDIO}/📦️packages/🦀️rust/Cargo.toml", lambda text: stdio_cargo(text, plan_data))
    shipped = f"{STDIO}/🧪️tests/🚢️shipped-fleet/🦀️.rs"
    current = plan.read(shipped)
    if plan.wants(shipped) and ("const SHIPPED_APP_CEILING: usize = 24;" not in current or "fn the_shipped_component_assembles_exactly_the_declared_bounded_fleet()" not in current):
        raise Refused(f"{shipped}: not the lb-p4 guard this patch supersedes")
    plan.edit(shipped, lambda _text: shipped_fleet_rs(plan_data))
    plan.edit(f"{STDIO}/🧪️tests/✏️editor-catalog/🦀️.rs", editor_catalog_rs)
    plan.edit(f"{STDIO}/📦️packages/🦀️rust/📜️script.ts", stdio_script_ts)
    plan.edit("Cargo.toml", lambda text: root_cargo(text, plan_data))
    plan.edit(CATALOG, lambda text: catalog_json(text, plan_data))
    plan.edit(HUB_SCRIPT, lambda text: hub_script_ts(text, plan_data))
    plan.edit(TAXONOMY, lambda text: taxonomy_json(text, plan_data))
    plan.edit(WORKSPACE_CONTRACT, lambda text: workspace_contract_ts(text, plan_data))
    plan.edit(PLAY_RUNTIME, lambda text: play_runtime_json(text, plan_data))
    plan.edit(PLAY_COVERAGE, play_coverage_ts)
    plan.edit(f"{STDIO}/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/✏️editor/🎮️commands/🔊️edit-audio/🦀️.rs", wav_extra_actions_rs)
    plan.edit(f"{STDIO}/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/✏️editor/🦀️.rs", lambda text: set_vertex_action_rs(text, "semio mesh", "        ActionArgDef::vec3(\"point\", LocalizedLabel::native(\"Target Point\", \"Zielpunkt\")).required(),\n    ])\n}\n"))
    plan.edit(f"{STDIO}/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/✏️editor/🦀️.rs", lambda text: set_vertex_action_rs(text, "semio brep", "ActionArgDef::vec3(\"point\", LocalizedLabel::native(\"Target Point\", \"Zielpunkt\")).required()])\n}\n"))
    plan.rename_dir(f"{STDIO}/🧩️composition", f"{STDIO}/🏘️composition", ["🏃️commands/🟦️.ts", "🏗️build/🟦️.ts"])
    for rel in COMPOSITION_REFERRERS:
        plan.edit(rel, lambda text, rel=rel: composition_referrer(text, rel))
    for rel in (".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"):
        plan.edit(rel, lambda text, rel=rel: launch_rows(text, rel))
    plan.edit("📜️script.ts", lambda text: insert_after(text, POLICY_ANCHOR, POLICY_ROW, "root script: closed-shape destination"))
    return plan


def main():
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--dry-run", action="store_true")
    mode.add_argument("--write", action="store_true")
    parser.add_argument("--root", default="/Users/ueli/Documents/semio")
    parser.add_argument("--part", choices=("code", "r10", "all"), default="all")
    args = parser.parse_args()
    try:
        plan = build(args.root, args.part)
    except (Refused, ValueError, OSError) as error:
        print(f"st2-apply: REFUSED — {error}")
        sys.exit(1)
    if plan.problems:
        for problem in plan.problems:
            print(f"st2-apply: REFUSED — {problem}")
        sys.exit(1)
    diffs = []
    for rel, (before, after) in sorted(plan.edits.items()):
        diff = list(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"a/{rel}", f"b/{rel}", n=2))
        added = sum(1 for line in diff if line.startswith("+") and not line.startswith("+++"))
        removed = sum(1 for line in diff if line.startswith("-") and not line.startswith("---"))
        diffs.extend(diff)
        print(f"edit  {rel}  +{added} -{removed}")
    for rel, content in sorted(plan.news.items()):
        print(f"new   {rel}  {content.count(chr(10))} lines")
    for source, target, files in plan.renames:
        print(f"move  {source} → {target}  ({len(files)} files)")
    print(f"st2-apply: part {args.part}: {len(plan.edits)} edits, {len(plan.news)} new files, {len(plan.renames)} directory move — root {args.root}")
    os.makedirs(os.path.join(HERE, "generated"), exist_ok=True)
    with open(os.path.join(HERE, "generated", f"st2-{args.part}-{'write' if args.write else 'dry'}.diff"), "w", encoding="utf-8") as handle:
        handle.writelines(diffs)
    if not args.write:
        print("st2-apply: dry run — nothing written")
        return
    for source, target, files in plan.renames:
        os.rename(os.path.join(args.root, source), os.path.join(args.root, target))
    for rel, (_before, after) in plan.edits.items():
        with open(os.path.join(args.root, rel), "w", encoding="utf-8") as handle:
            handle.write(after)
    for rel, content in plan.news.items():
        path = os.path.join(args.root, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(content)
    print("st2-apply: WRITTEN")


if __name__ == "__main__":
    main()
