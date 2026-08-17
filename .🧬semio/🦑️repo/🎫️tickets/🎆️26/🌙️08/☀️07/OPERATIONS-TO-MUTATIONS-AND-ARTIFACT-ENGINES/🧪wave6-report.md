# Wave 6 Report — Operations → Mutations Legacy Sweep

Ticket: `26/08/07/OPERATIONS-TO-MUTATIONS-AND-ARTIFACT-ENGINES`  
Date: 2026-08-07  
MCP: unavailable (ticket closed via file write; see `mcp-unavailable-wave6.txt`)

## Verdict

Wave 6 legacy sweep complete for **framework + plugins + hub**. All zero-expectation document-mutation identifiers are **0** outside `compose/` (separate technology, not mixed).

Kernel `cargo check` **green**. Lowpoly `--lib` **138/138 green**. Plugin registry **generate** refreshed `launch.json`.  
`verify-gate` / `policy` / `plugin-registry:check` still **exit 1** on residual structural taxonomy breaches (missing TS stubs / undeclared `#[path]`), documented below — not leftover `Operation` types.

## Gates

| Gate | Command | Exit | Log | Notes |
|------|---------|------|-----|-------|
| Policy | `DEVELOPER_DIR=/Library/Developer/CommandLineTools bun ./📜️script.ts policy` | **1** | `🧪wave6-policy.txt` | Same as Wave 2b: `runPolicyExit` exits on any high breach and prints no summary (only DEBUG lines). See `🧪wave6-policy-note.md`. |
| Verify-gate | `bun nx run workspace:verify-gate` | **1** | `🧪wave6-verify-gate.txt` | Failed on `@semio-tech/plugin-registry:check` (513 residual structural items). |
| Registry check | `bun nx run @semio-tech/plugin-registry:check` | **1** | `🧪wave6-registry-check.txt` | Missing `⚙️engine/🟦️component.ts`, mutation TS leaves, undeclared glue `#[path]` — structural completeness residuals. |
| Registry generate | `bun nx run @semio-tech/plugin-registry:generate` | **0** | `🧪wave6-registry-generate.txt` | Catalog + `.vscode/launch.json` regenerated. |
| Cargo kernel | `cargo check -p semio-framework-os-kernel` | **0** | `🧪wave6-cargo-kernel.txt` | Green (warnings only; pre-existing unused imports / dead_code). |
| Cargo lowpoly | `cargo test -p semio-s-plugin-lowpoly --lib` | **0** | `🧪wave6-lowpoly-test.txt` | **138 passed**; 0 failed. |
| Renderer test (optional) | `bun nx run @semio-tech/framework-renderer-react:test` | budget kill | `🧪wave6-renderer-test.txt` | Hit 15s budget (`program worker mock unresponsive`); not re-run under exhaustive. Wave 5 already covered vitest. |

### Registry residual breach summary

- **513** checklist items in `🧪wave6-registry-check.txt`
- Top plugins by count: 🔋️energy=85, 🗒️note=41, 🎪️demonstrator=35, 📋️forms=32, 🌿️vcs=28, 🌊️flow=26, 📖️playbook=24, 🎬️sequence=21, 🕸️dag=20, 📏️layout=19
- Nature: missing TS stubs under `🧬️mutations/**/{🔺️diff,↩️inverse}` and `⚙️engine/🟦️component.ts`; some Rust leaves not wired in `📦️glue.rs` `#[path]`
- **Not** document-mutation `Operation` identifier leftovers

## Fixed (document-mutation leftovers)

No legacy aliases retained.

| Old | New | Scope |
|-----|-----|-------|
| `ActionKind::Operation` | `ActionKind::Mutation` | plugins (puzzle/trinity/space/writer/fem/gis), Shell mapping |
| `kind: "Operation"` | `kind: "Mutation"` | wgpu shell menu fixtures |
| `Apply { operations:` | `Apply { mutations:` | process/cad/draw/remodel |
| `result.operations` / `inverse_group.operations` / `diff.operations` | `.mutations` | InvocationResult / UndoGroup / LayoutDiff / Shell |
| `OperationEnvelope` | `MutationEnvelope` | `🌎️hub` |
| `operation_id` field on envelope | `mutation_id` | hub sample envelope |
| `InverseOperation` | `InverseMutation` | hub |
| `OperationId` type alias + uses | `MutationId` (alias **removed**) | kernel + glue |
| `target_operation` | `target_mutation` | kernel InverseMutation + plugin |
| `default_operation_id` / `undo_operation_id` | `*_mutation_id` | db document |
| `operation_envelope_from_edit` / `operation_ids_for_edit` | `mutation_envelope_from_edit` / `mutation_ids_for_edit` | spr/causal + store |
| `OperationDiff` imports/docs | `MutationDiff` | forms + plugin docs |
| `Operation::diff` / `Operation::backwards` / `.backwards(` | `Mutation::diff` / `Mutation::inverse` / `.inverse(` | math/norm configs + call sites |
| `DerivedOperation` (dsl derive test) | `DerivedMutation` | os dsl |
| `KernelOperation` in comments | `KernelMutation` | manifest |
| Docs `Projection`/`Operation` | `Projection`/`Mutation` | engines/glue comments |

Also: duplicate glue export `MutationId, MutationId` cleaned.

## Zero-expectation identifier counts (excl. compose / mit-bestand / tickets)

All **0**:

- `ActionKind::Operation`
- `Apply { operations`
- `OperationEnvelope`
- `OperationId`
- `KernelOperation`
- `OperationDiff` (bare)
- `CollectionOperation`
- `document_operations`
- `applyOperations`
- `fn backwards` (trait method)
- `result.operations`
- `operation_envelope_from_edit`

## Leftover Operation-pattern hits (sweep regex) — classification

Full dump: `🧪wave6-operation-hits.txt` (**81** lines after Wave 6 fixes).

Zero **BUG** leftovers for document-mutation protocol/API identifiers outside `compose/`.

### compose technology — 5 — intentional untouched (out of scope)

- `compose/client/lib/js/index.ts:890:/** @emoji 📡️ GraphQL {@code Operation} {@code __typename} → {@link EventBus} {@code kind} (react hooks rely on these strings). */`
- `compose/client/lib/rs/lib.rs:5813:        /// @emoji 📦️ Single mutation entry: walks canonical [`crate::operation::CanonicalKitDiff`] from [`crate::operation::Operation::to_diff`].`
- `compose/server/hub/rs/bin.rs:1629:        Commands { envelope: Box<protocol::OperationEnvelope>, frontier: db::Frontier },`
- `compose/server/hub/rs/bin.rs:1684:            let op_envelope = protocol::OperationEnvelope {`
- `compose/server/hub/rs/bin.rs:2486:        let envelope = protocol::OperationEnvelope {`

### 2D boolean / BREP / DrawingError::Operation — 70 — intentional untouched

Compute/geometry error variants and boolean CAD ops (normative “boolean ops” / different concept).

- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🧬️mutations/🔀set-boolean-operation/🔺️diff/🦀️component.rs:11:pub struct SetBooleanOperationDiff {`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🧬️mutations/🔀set-boolean-operation/🔺️diff/🦀️component.rs:15:impl SetBooleanOperationDiff {`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️draw/🧬️mutations/🔀set-boolean-operation/🔺️diff/🦀️component.rs:21:impl MutationDiff<DrawDocument> for SetBooleanOperationDiff {`
- `✏️s/🔌️plugins/🏗️fem/🎛️apps/◻2d/🎮️commands/📚️example/🦀️component.rs:51:    /// Operation, so the registry's View/Shell kind discipline must let a whole-document reset through.`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🧬️mutations/🦀️component.rs:330:/// encoding respectively), same local-bridge shape as `semio_compose_rs`'s `KitSnapshot`. `Operation`/`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🦀️component.rs:354:            // ✏️ Operation actions — flow through the document store with true inverses. `setActiveExample``
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎮️commands/🎨️example/🦀️component.rs:12:/// ✏️ Replaces document content via a `SetDocument` operation, so this is an Operation action (not a`
- `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🎮️commands/🎨️example/🦀️component.rs:58:    /// an Operation. Under the real registry the View/Shell → emits-operations guard rejects a mis-declaration;`
- `✏️s/🔨️modules/◻2d/⚙️engine/🦀️component.rs:227:    Operation(String),`
- `✏️s/🔨️modules/◻2d/🔀️booleans/🦀️component.rs:84:        return Err(DrawingError::Operation("boolean produced empty path".into()));`
- `✏️s/🔨️modules/◻2d/🔀️booleans/🦀️component.rs:109:        return Err(DrawingError::Operation("boolean produced empty path".into()));`
- `✏️s/🔨️modules/◻2d/🔀️booleans/🦀️component.rs:153:        assert!(matches!(err, DrawingError::Operation(message) if message.contains("empty path")));`
- `✏️s/🔨️modules/◻2d/🗄️store/🦀️component.rs:26:        EngineFault::Compute(message) => DrawingError::Operation(message),`
- `✏️s/🔨️modules/◻2d/🗄️store/🦀️component.rs:27:        EngineFault::UnknownEngine(message) => DrawingError::Operation(message),`
- `✏️s/🔨️modules/◻2d/🗄️store/🦀️component.rs:652:            Err(DrawingError::Operation("boolean operations require booleans feature".into()))`
- `✏️s/🔨️modules/◻2d/🗄️store/🦀️component.rs:664:            Err(DrawingError::Operation("boolean operations require booleans feature".into()))`
- `✏️s/🔨️modules/◻2d/🗄️store/🦀️component.rs:677:            Err(DrawingError::Operation("trace requires trace feature".into()))`
- `✏️s/🔨️modules/◻2d/🗄️store/🦀️component.rs:735:            Err(DrawingError::Operation("boolean operations require booleans feature".into()))`
- `✏️s/🔨️modules/◻2d/🔍️trace/🦀️component.rs:159:        return Err(DrawingError::Operation("trace produced no contours".into()));`
- `✏️s/🔨️modules/◻2d/🔍️trace/🦀️component.rs:167:        return Err(DrawingError::Operation("trace produced no segments".into()));`
- … +50 more in dump

### GraphQL OperationDefinition — 0 — intentional untouched

- (none in this sweep slice; GraphQL hits live under repo vscode generated docs)

### wgpu::Operations — 0 — intentional untouched

- (none matched this sweep regex; present in draw path as GPU API)

### no-operation label — 1 — intentional untouched

- `✏️s/🔌️plugins/🎞️animate/🎛️apps/🎬️present/🦀️component.rs:238:            .view_action("noMutation", LocalizedLabel::native("No Operation", "Keine Aktion"))`

### renderer test fixture display names — 2 — intentional untouched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts:3657:      name: `Operation ${index}`,`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️index.test.ts:3658:      abbreviation: `Operation${index}`,`

### NodeGraph / ink `operations_json` — 0 — intentional untouched

- (field name `operations`/`operations_json` for ink events; few matched this particular regex)

### comments / docs / unrelated Operation word — 3 — intentional / residual prose

Mostly historical comments, Op-brand discussions, or domain wording. Not protocol type leftovers.

- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🔺️diff/🦀️component.rs:17:/// 📦️ Operation-list diff: layout operations fold sequentially over a cloned projection. `absorb``
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📓️iso16757/🦀️component.rs:429:        Operation,`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️imperative/📡️spr/🦀️component.rs:29:/// default `Shape::Text` behavior — no per-field opt-in needed) since a `store::Operation` grammar is`

### BUG leftover document-mutation identifier — 0

- **none**

## AGENTS.md files that still say Operation (list only; not edited)

- `🧰️framework/🛍️products/💻️os/AGENTS.md:61:# Operation`
- `🧰️framework/🛍️products/💻️os/🖥️host/AGENTS.md:61:# Operation`
- `✏️s/🔌️plugins/🌿️vcs/AGENTS.md:12:- **Operation** — stored semantic mutation; defines `diff(pre)` and `backwards(pre)``
- `✏️s/🔌️plugins/📖️playbook/AGENTS.md:15:- **ProtocolOp**/**ProtocolDiff** — `vcs::Operation`/`OperationDiff` implementations for add/remove/move step and block, and title updates`

## Cargo green/red crates (Wave 6 verified)

| Crate | Status |
|-------|--------|
| `semio-framework-os-kernel` | **GREEN** (`cargo check`) |
| `semio-s-plugin-lowpoly` | **GREEN** (`cargo test --lib`, 138 passed) |
| `semio-hub` | Not cargo-checked this wave; sources updated to `MutationEnvelope`/`MutationId`/`InverseMutation` to match protocol |
| Other plugin crates | Not re-checked individually (Wave 4 already green); identifier renames are mechanical |

## compose/ note (out of scope)

`compose/server/hub/rs/bin.rs` still constructs `protocol::OperationEnvelope` / `protocol::OperationId`. Protocol types were renamed without legacy aliases — compose will not compile against current kernel until a **compose-scoped** follow-up (must not mix technologies in this ticket).

## Files worked on (Wave 6)

- Framework: kernel, glue, manifest, store, spr/causal, plugin InvocationResult tests, Shell, wgpu menu, db document
- Hub: `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`
- Plugins: ActionKind, Apply fields, result.mutations, docs across puzzle/trinity/space/writer/fem/gis/process/cad/draw/remodel/forms/layout/math/norm/… 
- Ticket artifacts: this report + gate logs

## Closure

Status set to **closed** in `🎫️ticket.json` (MCP `ticket_close` unavailable).


## Second-pass API leftovers (after initial sweep)

Additional document-mutation identifiers fixed after the first sweep classification:

| Old | New |
|-----|-----|
| `BackboneMessage::Operations` | `BackboneMessage::Mutations` (cad, puzzle2d) |
| `DocumentEvent::RemoteOperations` / `DocumentActorMsg::LocalOperations` | `RemoteMutations` / `LocalMutations` (Shell) |
| `apply_operations` | `apply_mutations` (Shell / ProgramBridge) |
| `AppBuilder::operation(...)` | `AppBuilder::mutation(...)` (~361 call sites) |
| diagnostic `"remoteOperations"` | `"remoteMutations"` |

Kernel recheck **green**. Lowpoly recheck logged in `🧪wave6-lowpoly-test-recheck.txt`.
