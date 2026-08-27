# Space Retained Action Cohort

## Outcome

The canonical Space command surface has 40 routes. Four fixed host-effect reducers now have a concrete owner-local retained factory, exact bounded-first-step proofs, and an exact `HostOnly` publication contract per tool. The other 36 routes remain intentionally fail-closed because they need document/config/presence publication authority, registry ownership, graph cursors, or retained codec work that this packet does not have.

The honest split is therefore 4 source-migrated / 36 fail-closed. The official verifier recognizes all four as owner-local retained routes with zero forged-factory failures, but still reports all 40 Space rows as remaining because its coordinator-owned full prepare/job/commit admission gate is not bounded yet. This report does not claim runtime admission.

## Changed Sources

- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🦀️component.rs`
  - Added `SpaceCommandJobFactory` with direct `type Owner = SpaceApp`.
  - Added exact factory registration, owner builder, bounded reducer/extent, wire-size rejection, and four proof rows.
  - Added a one-to-one `HostOnly` publication contract for each retained tool id.
  - Annotated only the four retained host-effect actions in the manifest.
  - Removed the Space-engine process-global presence peer registry. Remote peers now fail closed as an empty host-owned view until `ArtifactHost` supplies typed instance presence.
  - Added a test-only `SpaceRetainedCatalogOracle` implemented by the existing third-party `serde_json` dependency.
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🧪️fixtures/🎯️retained-command-limits.json`
  - Language-neutral 40-route catalog, exact execution/status classification, limits, and blockers.
- `✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🧪️fixtures/🎯️retained-command-limits.schema.json`
  - Draft 2020-12 schema for the catalog and oracle expectations.
- `✏️s/🔌️plugins/🪐️space/🦀️component.rs`
  - Marks the remaining `shared_studio_ports` process-global mutable payload registry as an explicit blocker; no closure claim is made.

## Retained Bounded Routes

All four reducers emit exactly one host effect, traverse no document or global registry, invoke no codec, and contain no loop:

- `setActiveExample`: one capped navigation effect.
- `importSpacePack`: one fixed file-selection request; no payload decoding occurs.
- `goHome`: one fixed navigation effect.
- `navigateVirtualFileSystemNode`: one capped navigation effect.

Their shared contract is `maxRawBytes=65,536`, `maxDecodedItems=64`, `maxWorkUnitsPerStep=1`, `maxOutputBytes=262,144`, `maxStepMicros=7,500`. The factory rejects checkpoints and oversized raw input.

## Fail-Closed Routes

The exact 36-route disposition and per-route blocker are schema-first in `🎯️retained-command-limits.json`. The main blocker groups are:

- Document/config publication: fixed reducer output is insufficient because `SpaceApp` installs neither artifact nor config one-item preparation factories. This includes apparently small mutations such as `addParameter`, `moveMediaNode`, `setActivePanelTab`, and `closeFocusedInstance`.
- Presence: `presenceHeartbeat` remains unannotated because the central publisher rejects presence/transient output without the retained ephemeral domain seam.
- Graph/selection: selection expansion, node/port lookup, graph edits, layout, copy/duplicate/paste, and open-instance resolution require persistent document/interaction cursors.
- Registry/global work: `spawnApp` and `workflowEngagementSubmit` resolve and clone app definitions; `importMedia` clones format descriptors and traverses registered extensions. None is claimed O(1).
- Media/pack/import/export: payload decode, encode, serialization, and document replacement need operation-owned retained byte/item cursors with progress, cancellation, freshness, ACK, incremental close, and terminal-empty witnesses.
- `compiledDagEngagementSubmit`: currently a no-op mutation route, so it remains fail-closed instead of receiving a meaningless migrated claim.

## Global Payload Ownership Blocker

`✏️s/🔌️plugins/🪐️space/🦀️component.rs` still owns `shared_studio_ports` through `OnceLock<Arc<Mutex<HashMap<String, Arc<dyn OsBackbonePort>>>>>`. The map is process-global mutable payload state and violates the instance-ownership rule. Its two producers receive already-erased host ports from `open_folder_space_backbone` and `open_file_space_backbone`; the correct closure requires a host-created, instance-scoped port-catalog service threaded into both Home and Studio operation context. Moving the map to another static is not a fix. Routes traversing it remain fail-closed.

## Validation Evidence

- `🧪️codex-space-retained-catalog-source-audit-2026-08-26.txt`: AJV validation, source parity, bounded-body checks, compiler status, and blocker.
- The strict schema rejects hostile missing, extra, wrong-tool, and wrong-lane publication-contract fixtures.
- `📊️codex-official-tool-jobs-shooting-source-2026-08-27.json`: latest cross-cohort official verifier output after mandatory publication contracts. It recognizes the direct-owner factory and four owner-local routes with zero Space-scoped forged/publication failures; the repository-global run remains red at the central full-operation gate.

## Pending Runtime Tests

No compiler, Cargo, Nx, or rustfmt task was run because the Store cohort holds the exclusive compiler lease. Once granted, run the focused Space Rust unit covering `retained_command_catalog_matches_the_serde_json_oracle`, then the official verifier again after the central full-operation gate changes. Runtime evidence must cover progress, cancellation, replay, ACK, close, and terminal emptiness for the four retained jobs before they can be called admitted end to end.
