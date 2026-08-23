# Sol Independent P8 Draw Retained-Load Cohort Audit — 2026-08-23

## Verdict

**REJECT — Draw retained-load source cohort.** The cohort has substantial fixed-owner machinery, but its live store-initialization route still performs a whole recursive snapshot clone and synchronous JSON reconstruction while applying a forward mutation. It also materializes a complete encoded operation before checking the field-byte limit. Those live transitive calls invalidate the claimed one-field/one-child retained construction and exact pre-admission boundary.

This is an independent Sol High source audit. I did not author this cohort and made no production change. Terra admission for this cohort was scheduler-blocked, so no Terra verdict exists to inherit or displace.

The verdict is deliberately narrow. Phase 8 remains **RED at 0/884 admitted commands and 18 failure classes**. Native, Cargo, Wasm, browser, runtime timing, and hostile-payload timing remain **RED/unverified**.

## Blocking Findings

### 1. Live mutation replay escapes the retained clone authority

`DrawStoreInitializationAuthority::ApplyForward` invokes `operation.diff(current)` and then `diff.apply(current)` in one initializer step (`owned/component.rs:1604-1635`). The reached `MutationDiff<DrawSnapshot> for DrawDiff::apply` starts with `let mut next = snapshot.clone()` (`diff/component.rs:364-390`). That recursively clones the complete layer tree, assets, strings, fills, strokes, gradient stops, dashes, paths, points, and other snapshot ownership outside `DrawLayerCloneAuthority`.

The same live diff application reaches synchronous JSON reconstruction for layer replacement, transform, fill, stroke, and trace parameters (`diff/component.rs:271-315`). `apply_assets_delta` additionally clones the complete asset map before applying its delta (`diff/component.rs:343-361`). Therefore the retained initial-snapshot clone does not establish a retained end-to-end mutation replay route.

This is hidden from the permanent verifier because its no-clone assertion checks only that the owned source text lacks the literal `source.clone()` (`📜️script.ts:1703-1750`). It does not inspect the called Draw diff implementation or reject `snapshot.clone()` there.

### 2. Mutation byte admission happens after whole-operation materialization

Forward, inverse, redo-forward, and redo-inverse phases call `operation.encode_op()` and only then compare the completed `Vec<u8>` length to `DRAW_OWNED_FIELD_BYTES` (`owned/component.rs:1604-1617`, `1638-1651`, `1693-1706`, and `1709-1722`). The operation is thus re-encoded as a whole before the byte boundary can reject it. This is not schema-first item/byte reservation and does not advance one mutation field per grant.

There is also an edit-ID validation hole: `ValidateEditPair` checks the left edit ID only while a right-hand comparison exists (`owned/component.rs:1509-1519`). A single edit and the final edit advance without their ID length being checked, yet later phases clone IDs, actors, and timestamps while seeding or committing history (`owned/component.rs:1543-1577`, `1654-1668`, `1725-1735`). Consequently the current source does not prove exact aggregate item/byte preflight for the whole live envelope.

### 3. The adversarial evidence does not exercise the retained authorities

The owned authority file contains zero Rust test functions. The editor contains two retained-load route fixtures:

- success through a Group containing one Path plus one image asset, generation swap, first ACK, and duplicate ACK (`editor/component.rs:667-747`);
- partial-page cancellation without publication (`editor/component.rs:749-764`).

Those fixtures do not cover all seven layer variants, both gradient forms/stops, stroke dash/cap/join, polygon points, Boolean operands, Trace fields, Text/Image fields, all fourteen mutation applications, depth 64 and depth +1, item/byte capacity and +1, zero grant, stale/superseded generation, false terminal, displaced-store retirement before ACK, or cancellation in each clone/apply/retirement phase.

The 174 permanent verifier self-tests pass, but the eleven Draw mutations are source-string predicate mutations (`📜️script.ts:3022-3091`). They prove that selected names and snippets remain present; they do not execute the authority or catch the transitive whole-tree clone and serde route above. This is not sufficient adversarial evidence for the requested source acceptance.

## Positive Source Evidence

These seams are present and were not the reason for rejection:

| Requirement | Source result | Evidence |
| --- | --- | --- |
| Fixed recursive authority | PASS | `DRAW_MAXIMUM_NESTED_ITEMS = 4_096`, fixed envelope byte cap, `DRAW_MAXIMUM_LAYER_DEPTH = 64`, and fixed `[usize; 64]`/frame arrays (`owned/component.rs:655-684`). |
| Initial snapshot preflight | PASS for the initial snapshot only | `DrawSnapshotBoundsAuthority` advances the seven typed layer variants through fixed path/frame state and faults on item, byte, or depth capacity (`owned/component.rs:690-807`). |
| Retained typed clone | PASS for the initial snapshot only | `DrawLayerCloneAuthority` uses typed skeletons and advances base strings, fills, gradient stops, stroke strings/dashes, points, segments, Boolean operands, and Group children across retained phases (`owned/component.rs:820-1129`). No whole-tree `source.clone()` or `serde_json::to_vec` appears in this authority. |
| Seven layer variants | PASS in clone/retirement taxonomy | Shape, Path, Text, Image, Group, Boolean, and Trace are explicit (`owned/component.rs:107-154`, `710-750`, `858-1073`). |
| Nested owners | PASS in retirement taxonomy | Base/attributes, fills, strokes, stops, dashes, segments, points, assets, strings, Group children, and Boolean operand identifiers are detached through explicit owners (`owned/component.rs:10-359`). |
| Fourteen mutation variants | PASS in retirement taxonomy | Visible, locked, opacity, blend mode, rename, transform, fill, stroke, Boolean operation, trace parameters, create, duplicate, delete, and reorder are exhaustive (`owned/component.rs:259-341`). This does not cure the unbounded semantic apply route. |
| Terminal ownership | PASS structurally for the local authorities | `ManuallyDrop`, retained active retirement, terminal witnesses, and Drop assertions exist for snapshot roots, decode values, layer clones, snapshot clones, and initializer state (`owned/component.rs:47-620`, `820-1322`, `1360-1797`). |
| Shared retained decoder | PASS structurally | `DrawEnvelopeOwnedFieldCatalog` uses `artifact_owned_spr_edit_history_decoder` and supplies snapshot/mutation retirement factories (`owned/component.rs:625-650`). |

## Live Editor and Wasm Route

The browser-facing source route is materially present:

1. `DrawSnapshotVcs` owns `VcsArtifactApp<EditorApp<DrawPlayApp>>`.
2. `beginEnvelopeLoad` bounds requested pages and bytes before opening ingress.
3. `admitEnvelopePage` rejects `Uint8Array` length before copying into the fixed 4,096-byte Rust page.
4. `sealEnvelopeLoad` transfers the ingress owner.
5. `pollEnvelopeLoad` advances one `maintenance_step(1, pageBytes)` and one operation poll.
6. Ready performs generation-matched acknowledgement; duplicate ACK is idempotent in the inspected fixture. Cancelled/faulted polls also acknowledge their terminal operation.
7. `cancelEnvelopeLoad` and one-grant `closeStep` are exposed.

The route is at `editor/component.rs:494-616`. Its source-level ingress/generation/ACK/cancel/close shape is accepted as present, but the reached initializer defect prevents cohort acceptance. No Wasm or browser execution was performed.

## Zero-Placeholder Census

The exact current repository census is **14** `reject_whole_buffer_artifact_envelope_ingress` occurrences: **one shared fail-closed definition plus 13 live structural callers**. The Draw subtree has **zero** occurrences. This specific decrement is source-valid; the remaining 13 callers and the global roster stay RED.

## Executed Gates

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check --config skip_children=true` on Draw owned/editor/glue | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | PASS: **174** self-tests clean |
| `bun ./📜️script.ts verify interactivity --self-test --format json` | PASS: DENY clean; one recorded test-only blocking bridge and two predeclared future entries |
| `bun ./📜️script.ts verify interactivity --format json` | PASS: same DENY result |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json` | Expected global RED: exit 1, **0/884**, **18** failure classes; no Draw-named verifier failure |
| Draw independent ledger vs both p8yt ledgers | PASS: byte-identical, SHA-256 `59dfda87dd276d9b1ecd9d7e12782108838e983da977f327254b30f2d2e47b07` |
| Ledger counts | 50 macro hosts, 50 invocations, 775 rows, 773 unique rows, 0 bounded rows, 884 remaining, 18 failures, 174 self-tests |
| Scoped and whole working/staged/HEAD `git diff --check` | PASS |
| Direct placeholder census | PASS for Draw only: 1 shared definition + 13 live callers; Draw zero |
| Cargo, Nx, native, Wasm, browser, network, root lint, runtime timing | Not run by instruction; **RED/unverified** |

The independent audit ledger is `sol-independent-p8-draw-tool-jobs-2026-08-23.json`; it is byte-identical to `p8yt-draw-tool-jobs.json` and `p8yt-draw-tool-jobs-repeat.json`.

## Required Repair Before Re-Audit

1. Replace `operation.diff(current)`/`DrawDiff::apply` replay with a retained typed mutation-apply authority that reserves exact field/item/byte ownership before construction, applies one bounded field/child per grant, and publishes atomically after generation revalidation.
2. Remove live whole-snapshot/asset-map clone and JSON field reconstruction from retained replay; retain JSON/legacy diff behavior only as a test oracle if needed.
3. Replace whole `encode_op()` hashing with a retained field/page encoder or digest cursor whose credits are established before owned output construction.
4. Validate every edit ID and all cloned history strings with exact aggregate credits, including single/final edits.
5. Add executable Rust fixtures for all seven layer variants and fourteen mutation variants, depth/item/byte boundaries and +1, zero grant, stale/superseded generation, false terminal, saturation, cancellation in every phase, displaced-store close before ACK, duplicate ACK, and exact terminal-empty retirement.

Until those source gaps close and the serialized build/runtime gates execute, the Draw source cohort and Phase 8 remain **REJECTED**.
