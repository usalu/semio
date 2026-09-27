#!/usr/bin/env python3
"""🛡️ LB-P4 (prepared; lands after W3 announces 7800 on B3 — it edits stdio's Cargo.toml, which the running chain
builds): (1) guard law `🧪️tests/🚢️shipped-fleet`: the app set the SHIPPED component assembles (default features,
`plugin-root`) is exactly the declared one — every editor app pinned by a `[[package.metadata.semio.playground]]`
row (the declared shipped list, "ONE row per app of the shipped document fleet") plus its dialect's viewer — and
stays under the per-component app ceiling, so the 176-app library fleet can never silently become the component
again; (2) `editor_catalog` iterates all 88 editors → `required-features = ["full-app-catalog"]`; (3) stdio's
dev-dependency enables the SDK's `artifact-app-testing` (LB-P2: `editor_catalog` imports `artifact_app_laws`);
(4) the Codex test's `println!("[DEBUG] …")` leftover is removed.
Usage: lb-p4-stdio-shipped-fleet-law.py --dry-run | --write"""
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio")
CARGO = ROOT / "📦️packages/🦀️rust/Cargo.toml"
CATALOG = ROOT / "🧪️tests/✏️editor-catalog/🦀️.rs"
LAW = ROOT / "🧪️tests/🚢️shipped-fleet/🦀️.rs"
write = "--write" in sys.argv
if not write and "--dry-run" not in sys.argv:
    sys.exit(__doc__)
problems, edits = [], {}


def replace(path, old, new):
    """✂️ One exact replacement in the pending text of `path`, or a recorded problem."""
    text = edits.get(path) or path.read_text(encoding="utf-8")
    if text.count(old) != 1:
        problems.append(f"{path.name}: expected 1× {old[:70]!r}, found {text.count(old)}")
        return
    edits[path] = text.replace(old, new)


replace(CARGO, '[[test]]\nname = "editor_catalog"\npath = "../../🧪️tests/✏️editor-catalog/🦀️.rs"\nrequired-features = ["component-app-assembly"]\n',
        '[[test]]\nname = "editor_catalog"\npath = "../../🧪️tests/✏️editor-catalog/🦀️.rs"\nrequired-features = ["full-app-catalog"]\n\n'
        '[[test]]\nname = "shipped_fleet"\npath = "../../🧪️tests/🚢️shipped-fleet/🦀️.rs"\n')
replace(CARGO, "[dev-dependencies]\nsemio-framework-async-macros = { workspace = true }\n",
        "[dev-dependencies]\nsemio-framework-async-macros = { workspace = true }\nsemio-framework-plugin = { workspace = true, features = [\"artifact-app-testing\"] }\n")
replace(CATALOG, '    println!("[DEBUG] stdio plugin assembles all {} editable artifact subsets", fixture["editorCount"]);\n', "")
if LAW.exists():
    problems.append(f"{LAW} already exists")
else:
    edits[LAW] = '''//! 🛡️ The stdio component ships a bounded, declared app fleet — never the 88-subset library fleet.

use std::collections::BTreeSet;

/// 📏️ Apps one stdio component may assemble. Every registered app monomorphises the whole app runtime
/// (`VcsArtifactApp<EditorApp<E>/ViewerApp<V>, Members>`, its retained tool-job factories and snapshot-edit
/// factory) and is live code inside the component: the 18-app document fleet compiles in ~1.8 GB, while the
/// 176-app library fleet drove the wasm-dev rustc to 85 GB in one codegen unit and `wasm-component-ld` refused it
/// ("functions count exceeds limit of 1000000", chain b3 run 2, 2026-09-27). A family beyond this bound ships as
/// its own component.
const SHIPPED_APP_CEILING: usize = 24;

/// 📋️ The declared shipped editors: the `app` of every `[[package.metadata.semio.playground]]` row, which
/// declares ONE row per shipped editor (see the comment above those rows in `Cargo.toml`).
fn declared_shipped_editors() -> BTreeSet<String> {
    let manifest = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"));
    manifest
        .split("[[package.metadata.semio.playground]]")
        .skip(1)
        .filter_map(|row| row.lines().find_map(|line| line.strip_prefix("app = \\"").and_then(|rest| rest.strip_suffix('"')).map(str::to_string)))
        .collect()
}

/// 🛡️ LAW: the default (`plugin-root`) assembly registers exactly the declared shipped editors plus one viewer per
/// shipped dialect, and stays under [`SHIPPED_APP_CEILING`] — so widening the component is a declared, reviewed
/// change of the playground rows, never a feature edit that silently ships the library fleet.
#[cfg(not(feature = "full-app-catalog"))]
#[test]
fn the_shipped_component_assembles_exactly_the_declared_bounded_fleet() {
    let plugin = semio_s_plugin_stdio::plugin().expect("shipped stdio assembly");
    let editors = plugin.manifest.apps.iter().filter(|app| app.id.ends_with("#editor")).map(|app| app.id.clone()).collect::<BTreeSet<_>>();
    let viewers = plugin.manifest.apps.iter().filter(|app| app.id.ends_with("#viewer")).map(|app| app.id.trim_end_matches("#viewer").to_string()).collect::<BTreeSet<_>>();
    let declared = declared_shipped_editors();
    assert_eq!(editors, declared, "the shipped editors are exactly the playground-declared ones");
    assert_eq!(viewers, declared.iter().map(|id| id.trim_end_matches("#editor").to_string()).collect::<BTreeSet<_>>(), "every shipped dialect ships its viewer, and only those");
    assert!(plugin.manifest.apps.len() <= SHIPPED_APP_CEILING, "{} apps exceed the per-component ceiling of {SHIPPED_APP_CEILING}", plugin.manifest.apps.len());
}
'''
print(f"files={len(edits)} problems={len(problems)} write={write}")
for problem in problems:
    print("PROBLEM", problem)
if write and not problems:
    for path, text in edits.items():
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
