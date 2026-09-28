#!/usr/bin/env python3
"""🧬️ LB2 prepared patch p6 — `.artifact(…)` declarations publish their schema and inference descriptor catalogs.

Measured 2026-09-28 (S18 served matrix: json/i-json `set-node` refused `snapshot-edit.schema-unregistered native document
schema 's.stdio.json' … not registered`; native probe `generated/c10-probe-1.txt`: after `semio_s_plugin_stdio::plugin()`
NONE of `s.stdio.{json,xml,csv,tsv,txt,md,html}` is in the OS-wide artifact schema catalog). Cause: `PluginBuilder::artifact`
(the ArtifactDeclaration path stdio, trinity, remodel, dag, writer, forms, mathematical, puzzle, fem, draw, playbook use)
collects `schemas`/`inferences`/`languages` into `PluginRuntimeRegistry` and never publishes them — the registry itself
carries `#[allow(dead_code)]` + "captured but unwired — follow-up needed". Only `declare_artifact` (the declaration tree)
publishes them (`commit_artifact_declarations`). The snapshot-edit route (`apply_snapshot_patch_for_dialect`, every stdio
editor's `setSnapshotValue`/details panel and json's `set-node`) resolves the editor's document schema in that catalog.

Hunks: SDK `PluginRuntimeRegistry::publish_declared_catalogs` (schema + inference descriptors, each preflight-then-commit,
exact-duplicate tolerant, conflict-fatal — the semantics `commit_artifact_declarations` already has); `try_build` calls it
right after `commit_artifact_declarations`; those two fields lose their dead-code allowance. Languages stay unwired on
purpose (51 single `register_language` overwrite sites at plugin init + fn-address comparison in the batch registry = a
false-conflict risk for every plugin's describe; needs its own census), and app schemas have no catalog yet. Law: `🧪️tests/🚢️shipped-fleet` — after `plugin()` every shipped editor's document schema is
registered, and a revision-bound json `set-node` through the shipped editor lands (serde_json oracle).
Window 3: native `shipped_fleet` + SDK lib; wasm32 proof = the chain's describe of every `.artifact(…)` plugin (a conflicting
intra-plugin descriptor would now fail its assembly — the census list above is the set to watch).

Usage: lb2-p6-declared-catalogs.py [--dry-run | --write | --revert] [--root <repo-or-overlay root>]"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in sys.argv
BACKUP = Path(__file__).resolve().parent / "generated" / "p6-backup"
PAYLOAD = Path(__file__).resolve().parent / "payload" / "p6"
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
BUILDER = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs"
FLEET = "✏️s/🔌️plugins/🗄️stdio/🧪️tests/🚢️shipped-fleet/🦀️.rs"

FIELDS_OLD = """        definitions: ArtifactDefinitionRegistry,
        // 🕳️ Unlike every other field here, these four have no reader anywhere in this crate —
        // `inference_services`/`host_media_handlers`/`flow_extensions`/the two mutation maps all
        // flow out through a `self.runtime.*` accessor (see the `impl` below); these were collected
        // at `into_runtime` time and then never consumed. Z1 (26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME)
        // found this while chasing a `dead_code` warning: a real gap (some global artifact
        // schema/inference/language catalog this crate should be publishing into, analogous to how
        // `definitions()` already exposes `ArtifactDefinitionRegistry`), not dead data — flagged for
        // follow-up wiring rather than fixed here, since inventing the consumer without knowing the
        // intended catalog risks wiring it to the wrong place.
        #[allow(dead_code, reason = "captured but unwired — see comment above, follow-up needed")]
        schemas: Vec<::semio_framework_schema::ArtifactSchemaDescriptor>,
        #[allow(dead_code, reason = "captured but unwired — see comment above, follow-up needed")]
        inferences: Vec<::semio_framework_schema::ArtifactInferenceDescriptor>,
        inference_services: ArtifactInferenceServiceRegistry,
        routed_inferences: Vec<ArtifactInferenceServiceMetadata>,
        #[allow(dead_code, reason = "captured but unwired — see comment above, follow-up needed")]
        languages: Vec<dsl::LanguageSpec>,
        #[allow(dead_code, reason = "captured but unwired — see comment above, follow-up needed")]
        app_schemas: Vec<::semio_framework_schema::AppSchemaDescriptor>,
"""
FIELDS_NEW = """        definitions: ArtifactDefinitionRegistry,
        schemas: Vec<::semio_framework_schema::ArtifactSchemaDescriptor>,
        inferences: Vec<::semio_framework_schema::ArtifactInferenceDescriptor>,
        inference_services: ArtifactInferenceServiceRegistry,
        routed_inferences: Vec<ArtifactInferenceServiceMetadata>,
        // 🕳️ Collected at `into_runtime` time and never consumed. Languages: plugin inits also register grammars one by one
        // (`dsl::register_language`, overwrite semantics, 51 sites) and the batch registry compares hook fn addresses, so
        // publishing the declared set needs its own census first. App schemas: no OS-wide app-schema catalog exists yet.
        #[allow(dead_code, reason = "captured but unwired — see comment above")]
        languages: Vec<dsl::LanguageSpec>,
        #[allow(dead_code, reason = "captured but unwired — see comment above")]
        app_schemas: Vec<::semio_framework_schema::AppSchemaDescriptor>,
"""
PUBLISH_ANCHOR = """        /// 🧾️ Reads the immutable definitions published by this plugin assembly.
        pub fn definitions(&self) -> &ArtifactDefinitionRegistry {
            &self.definitions
        }
"""
PUBLISH_NEW = PUBLISH_ANCHOR + """
        /// 📌️ Publishes the schema and inference descriptors the `.artifact(…)` declarations contributed into the OS-wide
        /// catalogs — the same two descriptor channels `commit_artifact_declarations` publishes for the declaration tree, each
        /// preflighted then committed, exact duplicates tolerated, a conflicting descriptor fatal. The snapshot-edit route
        /// resolves an editor's document schema there; unpublished, every stdio editor edit was refused
        /// `snapshot-edit.schema-unregistered`.
        pub(crate) fn publish_declared_catalogs(&self) -> Result<(), PluginAssemblyError> {
            ::semio_framework_schema::register_artifact_schema_descriptors(self.schemas.clone()).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-schema", error.to_string()))?;
            ::semio_framework_schema::register_artifact_inference_descriptors(self.inferences.clone()).map_err(|error| PluginAssemblyError::new("plugin-assembly.declaration-inference", error.to_string()))
        }
"""
COMMIT_OLD = """        crate::app::declarations::commit_artifact_declarations(&plugin_id, &declared_artifacts)?;
"""
COMMIT_NEW = """        crate::app::declarations::commit_artifact_declarations(&plugin_id, &declared_artifacts)?;
        runtime.publish_declared_catalogs()?;
"""

problems, changed = [], {}
if "--revert" in sys.argv:
    for backup in sorted(path for path in BACKUP.rglob("*") if path.is_file()):
        rel = backup.relative_to(BACKUP)
        if backup.name.endswith(".absent"):
            target = ROOT / str(rel)[: -len(".absent")]
            if target.exists():
                target.unlink()
                print("removed", target.relative_to(ROOT))
                parent = target.parent
                while parent != ROOT and not any(parent.iterdir()):
                    parent.rmdir()
                    parent = parent.parent
        else:
            (ROOT / rel).write_bytes(backup.read_bytes())
            print("restored", rel)
    sys.exit(0)


def text(rel):
    return changed[rel] if rel in changed else (ROOT / rel).read_text(encoding="utf-8")


def replace_once(rel, old, new, done_marker):
    current = text(rel)
    if done_marker in current:
        print(f"already applied: {rel} ({done_marker[:60]!r})")
        return
    if current.count(old) != 1:
        problems.append(f"{rel}: expected 1x {old[:70]!r}, found {current.count(old)}")
        return
    changed[rel] = current.replace(old, new, 1)


replace_once(SDK, FIELDS_OLD, FIELDS_NEW, "publishing the declared set needs its own census first")
replace_once(SDK, PUBLISH_ANCHOR, PUBLISH_NEW, "pub(crate) fn publish_declared_catalogs(&self)")
replace_once(BUILDER, COMMIT_OLD, COMMIT_NEW, "runtime.publish_declared_catalogs()?;")
law = (PAYLOAD / "law.rs").read_text(encoding="utf-8")
fleet = text(FLEET)
if "fn the_shipped_assembly_publishes_every_editor_document_schema_and_a_json_node_edit_lands" in fleet:
    print(f"already applied: {FLEET}")
else:
    changed[FLEET] = fleet.rstrip("\n") + "\n" + law
for problem in problems:
    print("PROBLEM", problem)
print(f"root={ROOT} files={len(changed)} problems={len(problems)} write={WRITE}")
for rel in changed:
    print("  ", rel)
if WRITE and not problems:
    for rel, content in changed.items():
        target = ROOT / rel
        backup = BACKUP / (rel if target.exists() else rel + ".absent")
        backup.parent.mkdir(parents=True, exist_ok=True)
        backup.write_bytes(target.read_bytes() if target.exists() else b"")
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(content, encoding="utf-8")
    print("written")
sys.exit(1 if problems else 0)
