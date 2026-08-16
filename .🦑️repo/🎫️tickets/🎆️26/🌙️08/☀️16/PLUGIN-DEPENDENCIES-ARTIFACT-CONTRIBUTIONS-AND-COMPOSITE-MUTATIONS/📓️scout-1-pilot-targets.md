# Scout 1 — Pilot targets, composition reality, TS pack storage

Read-only survey (Haiku), 2026-08-16. Answers the open questions the master plan listed for W3.

## 1. Demonstrator playground holds NO artifact references

`✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/…/🧬️schema/📸️snapshot/🦀️component.rs`:

```rust
#[artifact_schema(id = "s.demonstrator.playground")]
pub struct PlaygroundSnapshot { #[state(artifact)] pub schema: String }
```

`🛂️manifest/🎪️demonstrator/🦀️component.rs` registers six FOREIGN app types as statically linked panes of the same plugin (`.register_document_app::<CadPlayApp>(…)`, `Puzzle3dPlayApp`, `Procedural3dPlayApp`, `SourcingCurateApp`, `Process3dPlayApp`, `Gis2dPlayApp`) plus `register_document_codec_for_app::<…>` for each.

**Consequence:** the plan's *preferred* P3 (playground composite with foreign steps into pane documents) is **not viable** — the playground snapshot has no `ArtifactRef`/`ArtifactChild`/link slots to step into. It does, however, run **six app instances of different document schemas inside one plugin**, which is exactly the cross-artifact / cross-instance shape a transaction needs.

## 2. Owned-child composition is real but in-guest

`store::ArtifactChild<…>` is used by three real plugins: `🔱️trinity` (`JackContentChild = ArtifactChild<SemioGraphSnapshot>`, `🔌️jack/🦀️component.rs:91`), `📸️remodel` (`RemodelAssetChild = ArtifactChild<SemioImageSnapshot>`, `RemodelMeshChild = ArtifactChild<SemioMeshSnapshot>`), `🖨️raster` (`RasterAssetChild = ArtifactChild<SemioImageSnapshot>`), each constructing `store::os_io::ArtifactRef { artifact_id, dialect }`. No `Emit::child_emits` / `ChildEmit::of` call site was found in any plugin — the seam exists in the framework and is not yet driven from plugin code.

**Consequence:** children live inside the owning guest instance (`ChildStoreFactory`, `AppCommand::LoadChildren`), so child edits are the **in-guest** composition path, not host transaction members. The host transaction protocol is for separate *instances*.

## 3. Flow mutation vocabulary

Nine kinds: `➕️create-widget`, `🔗️connect-widgets`, `✂️disconnect-widgets`, `📍️move-widgets`, `🔀️reorder-synapses`, `🔀️🪟️reorder-widgets`, `🔁️replace-widget`, `🔄️update-synapse-endpoints`, `🗑️delete-widget`.

`ReplaceWidget { id, widget }` is a single whole-value swap — **not** a hidden composite, so there is nothing to refactor. Real multi-mutation gestures are produced dynamically by `host_operations` → `flow_fixture_operations` snapshot diffing in `🗂️delete-selection` and `🔄️reorganize`.

**Consequence:** P1 stays as planned — add a genuine `duplicate-widget` composite planning `create-widget` + `connect-widgets`.

## 4. Browser host keeps no document pack

`AppChannelClient` (`🧰️framework/🛍️products/💻️os/🟦️component.ts:2143-2256`) passes packs straight through `loadDocument(pack, spr)` / `readDocument()` with no retained field; `adaptPluginHandle` (`…/🧱️elements/PluginRuntime/🟦️component.tsx:305-345`) keeps only `channels: Map<number, AppChannelClient>`; `ShellState` has no pack field; `ShellHost` drops pack bytes after `loadAppDocumentPack`.

**Consequence (binding on W2-B):** the browser transaction coordinator must add a per-instance pack cache on `AppChannelClient` (populated from `AppFrame::Document` and from `loadDocument` arguments) plus an accessor surfaced through `PluginWasmHandle`, before a contributor can be asked to plan against a target's current snapshot.

## Decisions taken by the coordinator

- **P3 primary (Rust host e2e):** two separately loaded plugin components, one app instance each, transaction initiated in plugin A with a foreign step into plugin B's artifact instance. This is the deterministic proof and does not depend on any UI arrangement.
- **P3 browser proof:** demonstrator's cad pane instance foreign-steps into the puzzle pane instance (two instances, two document schemas, one plugin) — proves cross-artifact/cross-instance in the browser host including group undo.
- **P2 unchanged:** `cad` ← `aec-building` extension contributes one composite mutation and one inference.
- **P1 unchanged:** flow `duplicate-widget` composite.
