# P8yx Fresh Process3d Retained-Envelope Ingress Census

Date: 2026-08-25  
Scope: read-only Phase 8 scout; no source, build, Cargo, Nx, Wasm, browser, or runtime command was run.  
Verdict: **RED — Wave 1 has not started.** Process3d remains a live whole-buffer caller and the full retained lifecycle/domain authority is absent.

## Evidence Read

- Master plan: `/Users/ueli/.codex/attachments/2225dd4d-c3b6-4564-b4b1-f552928e8ff3/pasted-text.txt:99`.
- Wave contract: `📓️p8-whole-buffer-ingress-wave-order-2026-08-24.md`.
- Prior Process3d contract: `📓️p8yx-process3d-retained-envelope-ingress-census-2026-08-23.md`.
- Live bridge: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs`.
- Live persisted schema/whole codecs: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`.
- Shared mounted ingress lifecycle: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:16483-16757`.
- Permanent verifier: `📜️script.ts:1413-1422`.

## Exact Live Raw-Caller Census

The production-only command was run:

```text
rg -n 'reject_whole_buffer_artifact_envelope_ingress' --glob '*.rs' --glob '!target/**' --glob '!**/🧪️tests/**'
```

It returns exactly **12 occurrences: one shared fail-closed definition and eleven live callers**.
This agrees with the 2026-08-24 wave-order baseline; Raster, Writer, Trinity Jack, Trinity Rewrite,
GIS Map, and Draw are not callers in this live census and must not be counted as remaining raw callers.

| # | Kind | Owner | Current source location |
| ---: | --- | --- | --- |
| 1 | Shared definition | OS Store | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:8109` |
| 2 | Live caller | CAD | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:24` |
| 3 | Live caller | Procedural3d | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:23` |
| 4 | Live caller | Procedural2d | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:23` |
| 5 | Live caller | Shooting | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:31` |
| 6 | Live caller | Dag | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs:9021` |
| 7 | Live caller | Flow | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs:976` |
| 8 | Live caller | FEM2d | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:22` |
| 9 | Live caller | FEM3d | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:22` |
| 10 | Live caller | **Process3d** | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:36` |
| 11 | Live caller | Puzzle5d | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:27` |
| 12 | Live caller | Puzzle3d | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:27` |

Thus accepted Process3d cutover alone changes **12 → 11** (one shared definition plus ten live callers). No count decrement is presently earned.

## Current Process3d Seam

`Process3dSnapshotVcs` is a Wasm-only `RefCell<Process3dStore>` wrapper
(`.../🌉️wasm/🦀️component.rs:25-28`). Its constructor accepts `Option<String>`
(`:33`), feeds the owned entire string to the rejecting placeholder (`:34-37`), then immediately
constructs a store (`:37`); the no-input branch immediately creates the default envelope/store
(`:39`). This is a direct whole-document constructor, not a mounted scheduler session.

The externally exposed bridge also calls synchronous `resolve_ready` store work behind direct Wasm
methods:

- whole command text: `dispatch_text` (`:44-47`);
- whole command byte slice: `dispatch_binary` (`:49-52`);
- whole JSON projection: `snapshot_json` (`:54-57`);
- whole JSON envelope: `envelope_json` (`:59-62`); and
- scalar generation read (`:64-67`).

The persisted snapshot has ten top-level fields (`.../📸️snapshot/🦀️component.rs:25-52`): workshop;
stock identity/label/pose/payload; two child handles; step payload vector; tool-child vector; and
cursor. The nested live taxonomy includes machine/capability/parameter/rule records
(`.../🧊️process3d/🦀️component.rs:60-215`), pose/origin/working-solid/process-measure/step owners
(`:329-469`), three composed child families, and all **16** `Process3dMutation` variants
(`.../🧬️mutations/🦀️component.rs:62-79`).

The current handcrafted snapshot route is deliberately whole-buffer: `enc_child_list` uses
`collect::<Vec<_>>().join` (`.../📸️snapshot/🦀️component.rs:119-124`), structured fields use
`serde_json::to_string/from_str` (`:127-136`), printed body builds one `String` (`:140-153`),
and binary decoding calls `to_vec`/allocates a collection from untrusted count (`:196-253`). These
may remain only for explicitly UI-forbidden batch/offline routes; the mounted Process3d ingress,
history, and output paths may not reach them.

## Exact Gaps Against the Shared Lifecycle

The shared app mechanism already offers an appropriate ownership boundary: pre-admitted fixed page
and byte credits before producer construction (`plugin/component.rs:16483-16495`), stale/closed/full
preflight (`:16497-16512`), untouched-page handback (`:16526-16542`), sealed transfer/retry
(`:16544-16578`), bounded decode/replacement polling (`:16580-16603`), cancellation routing
(`:16605-16616`), live-generation publication (`:16646-16675`), and exact replacement ACK
(`:16735-16757`). Process3d does not currently bind to any of these APIs.

| Contract | Current Process3d state | Required Wave-1 result |
| --- | --- | --- |
| Admission | Entire `String` exists before placeholder/store call; no operation/page/item/byte/output reservation. | Fixed operation/session registry; begin reserves exact maximum pages/bytes/items/output before a JS byte page is constructed; max+1 returns the exact page unchanged. |
| Decode/construction | Direct constructor plus whole text/binary/schema codecs. | Process3d-owned fixed field catalog and retained parser/candidate cursors; one token/field/collection/control root per grant; no whole `serde_json`, `to_vec`, clone, diff/apply, collect, or join on the interactive route. |
| Domain ownership | No explicit retained authorities for snapshot, all mutation variants, history, child references, conflicts, or control backing. | Explicit credits and one-grant retirement for strings, `Vec` backing, child refs, nested workshop/capability/rule, step/measure/solid, history and all 16 mutations; observed allocation capacity rather than requested capacity; fixed combined-depth stack. |
| Freshness/publication | `Process3dStore::new` completes synchronously; no operation/generation/parent revalidation or atomic candidate swap. | Checked nonzero operation generation and base revision/edit-parent check immediately before atomic swap; stale/ABA candidate retired while last-valid store/snapshot stays visible. |
| Progress/output | No Process3d ingress progress/checkpoint/preview or bounded paged projection/envelope API. | Bounded latest-wins progress/checkpoint/preview and lossless terminal result; paged projection/envelope output, or a separately enforced UI-forbidden batch-only classification. |
| Cancellation/fault | No mounted cancel method, no deadline/fuel phase checks, no retained fault/producer handback. | Cancel, deadline, stale, reject, panic, saturation, and handle-loss all enter the same retained terminal disposer; completed-but-unclaimed candidate remains owned. |
| Close/Drop | `RefCell<Process3dStore>` relies on ordinary destructor; no terminal-empty witness or incremental close. | `take/resume/close_step` and registry rediscovery; each close grant retires one exact owner/page/control root; ordinary populated Drop fails closed until terminal empty; cancellation after complete remains a close path. |
| Verifier | Generic verifier detects any raw caller (`📜️script.ts:1413-1422`), but no Process3d-specific retained-route predicate or mutation set exists. | Add a Process3d predicate plus hostile mutations that remove each boundary below; self-test and live predicate must distinguish a genuine lifecycle from renamed whole-buffer code. |

## Bounded Implementation Packets

1. **P8yx-a — Process3d owner catalog and initializer.** Touch only Process3d owned schema/codec
   surfaces. Add fixed catalog/constants, fixed combined-depth retirement stack, snapshot/mutation/history
   cursors, and an `ArtifactStoreInitializationAuthority<Process3dSnapshot, Process3dMutation>`.
   Keep the old codecs unreachable from interactive ingress; do not modify shared Store.
2. **P8yx-b — mounted Process3d lifecycle and Wasm bridge.** Replace `Option<String>` construction
   with begin/preflight/admit/seal/poll/ack/cancel/close APIs that use the existing mounted app
   lifecycle. Remove direct `ArtifactStore::new`, direct dispatch, whole JSON exports, and all
   synchronous complete-to-publish work from the interactive bridge. Add bounded paged output or
   explicitly route batch-only exports through a UI-forbidden gate.
3. **P8yx-c — source fixtures and permanent verifier.** Add Process3d-specific static predicate and
   hostile mutations for preflight, page handback, page/item/byte/output maximum+1, every snapshot
   field and mutation variant, nested combined depth, history/revision/ABA, cancellation/fault/panic
   at each transfer, complete-before-ACK, interrupted close, terminal-empty, last-valid publication,
   and deterministic progress/checkpoint digest. This packet owns no unrelated raw caller.
4. **P8yx-d — serialized acceptance.** After source packets are quiescent: scoped rustfmt, exact
   census **12 → 11**, verifier self-test/live predicate, deterministic ledgers, scoped and whole
   diff checks, then independent Terra audit. Cargo/native/Wasm/browser/replay/timing remain final
   serialized-matrix work and must not overlap active Rust packets.

## Preservation Gates

- Preserve the shared rejecting definition and every non-Process3d caller; this packet may change
  only the Process3d caller before independent acceptance.
- Preserve accepted P8 retained cohorts (Writer, Trinity Jack/Rewrite, GIS, Draw, Raster) and their
  fixed ingress contracts; do not copy a permissive whole-buffer compatibility path from them.
- Preserve Process3d's schema-first public taxonomy and child-handle semantics; move ownership via
  domain authorities rather than erasing the composed BRep/Flow children.
- Preserve the Phase 1 single-pool scheduler boundary: no plugin worker, thread, channel, nested
  pool, `resolve_ready`, or run-to-completion loop in the interactive route.
- Keep whole-codec helpers only when the enclosing call path is explicitly batch-only and UI-forbidden;
  no retained Wasm/history/output path may reach them.

## Blockers

1. **Implementation blocker:** no Process3d retained field catalog, initializer, mounted application
   seam, or Wasm lifecycle exists; this is the first Wave-1 implementation packet, not a narrow
   bridge-only replacement.
2. **Verification blocker:** no Process3d-specific permanent predicate/mutation suite currently
   proves owner/admission/cancellation/progress/close behavior. The generic raw-caller scan alone
   cannot accept a replacement.
3. **Serialization blocker:** Cargo/Nx/Wasm/browser/runtime checks are intentionally deferred while
   overlapping Rust source packets are active. This scout ran none and claims no runtime result.

