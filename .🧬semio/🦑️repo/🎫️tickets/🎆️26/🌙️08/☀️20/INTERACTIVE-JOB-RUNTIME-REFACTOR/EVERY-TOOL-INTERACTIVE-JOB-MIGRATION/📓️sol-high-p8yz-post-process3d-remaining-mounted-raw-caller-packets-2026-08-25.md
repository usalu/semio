# P8yz Remaining Mounted Raw-Caller Packets After Process3d

Date: 2026-08-25  
Scope: source-static packet map only; no build, test, Wasm, browser, Nx, or runtime command was run.

## Decision

`P8yx` (Process3d) is the next prerequisite, not an accepted decrement.  The current production
scan of `reject_whole_buffer_artifact_envelope_ingress` has **12 occurrences**: **one** shared
fail-closed definition and **eleven** live callers.  If and only if P8yx independently passes, the
remaining raw-caller queue is **10 live callers + the unchanged shared definition = 11**.

This is deliberately a narrower census than the general P8 command problem.  `p8t-independent-
remaining-tools-global-audit.md` still rejects the generic terminal command adapter and the wider
catalog; completing this queue does not credit those routes.

## Exact Current Census And Post-P8yx Queue

| Caller | Current raw boundary and bridge symbol | Direct bridge work that must disappear from the interactive route | Packet | Prerequisite |
| --- | --- | --- | --- | --- |
| Process3d | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:36`; `Process3dSnapshotVcs` | whole `Option<String>`, `ArtifactStore::new`, direct dispatch/output | P8yx (already specified) | P2a1 and mounted page/session mechanics |
| Procedural2d | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️procedural2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:23`; `Procedural2dSnapshotVcs` | `new`, `dispatch_text`, `dispatch_binary`, `snapshot_json`, `envelope_json`, `generation` | P8yz-a | accepted P8yx |
| Procedural3d | `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:23`; `Procedural3dSnapshotVcs` | same five bridge routes | P8yz-b | accepted P8yx |
| Puzzle3d | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:27`; `Puzzle3dArtifactVcs` | direct store bridge plus `puzzle3d_parse_dsl_json` | P8yz-c | P8yx; independently GREEN P4d; no active P4/P7 collision |
| Puzzle5d | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:27`; `Puzzle5dArtifactVcs` | direct store bridge plus `puzzle5d_parse_dsl_json` | P8yz-d | P8yx; P4d/P7 stable; no active P4/P7 collision |
| FEM2d | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:22`; `Fem2dSnapshotVcs` | `create`, direct dispatch/output/generation | P8yz-e | P8yx; independently GREEN P6g |
| FEM3d | `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:22`; `Fem3dSnapshotVcs` | same bridge routes | P8yz-f | P8yx; independently GREEN P6g |
| Flow | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs:976`; `FlowArtifactVcs` | whole `ArtifactDsl`/`ArtifactPack`, direct construction/dispatch, `snapshot_json`, host undo/redo | P8yz-g | P8yx; P3/P5 surface lifetime and renderer close contracts accepted |
| Dag | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs:9134`; `DagSnapshotVcs` | direct construction/dispatch and snapshot/envelope serialization | P8yz-h | P8yx; P3n populated-surface disposer accepted |
| Shooting | `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:31`; `ShootingArtifactVcs` | direct construction/dispatch and `projection_json` | P8yz-i | P8yx; P3/P5 geometry/renderer close contracts accepted |
| CAD | `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️component.rs:24`; `CadArtifactVcs` | direct construction/dispatch/output through `resolve_ready` | P8yz-j | P8yx; P3/P5 geometry/GPU retirement accepted |
| Shared guard | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs:8109`; `reject_whole_buffer_artifact_envelope_ingress` | unchanged until all ten caller predicates pass | P8yz-k | P8yz-a through -j accepted |

The ten post-P8yx caller files are distinct.  The two framework files are large and high-churn
(`Flow`: 1,374 lines; `Dag`: 9,517 lines), so their packets must be isolated even though they can
be executed in the same wave.

## Collision-Safe Waves

1. **Gate P8yx alone.** Independent source acceptance changes the measured census `12 -> 11`.
2. **Wave A, four independent writers:** P8yz-a Procedural2d, P8yz-b Procedural3d, P8yz-e FEM2d,
   P8yz-f FEM3d.  The FEM pair waits for P6g; the procedural pair needs only P8yx.  Each owns its
   domain-local field catalog, lifecycle adapter, fixtures, and its one Wasm bridge.
3. **Wave B, two conditionally independent writers:** P8yz-c Puzzle3d and P8yz-d Puzzle5d, only
   after a fresh edit census says that neither conflicts with active P4/P7 work.  Otherwise run
   P8yz-c then P8yz-d; the code must not borrow the P4 fill authority by alias.
4. **Wave C, four independent writers:** P8yz-g Flow, P8yz-h Dag, P8yz-i Shooting, P8yz-j CAD.
   They are source-file disjoint, but all require the stated renderer/surface retirement gates.
5. **Wave D, one serialized verifier/guard writer:** P8yz-k owns only the permanent aggregate
   predicate registration and, after a fresh zero-caller scan, deletes the shared guard.  It is
   the only remaining P8 packet allowed to edit `📜️script.ts` or the shared store definition.

At the end of Waves A--C the expected count is `11 -> 1`; only P8yz-k may claim `1 -> 0`.
Each decrement is accepted individually, never inferred from planned work.

## Contract Shared By Every Domain Packet

Each packet replaces its `RefCell<ArtifactStore<…>>` bridge with a mounted worker-owned session:
fixed operation/page/item/byte/output/control admission before producer copy; exact-page preflight,
seal, bounded poll, generation/base-revision/parent validation immediately before atomic publication,
ACK only after publication, and `take/resume/close_step` registry rediscovery.  Its source-local
catalog covers every snapshot field, mutation variant, history/edit/conflict root, child/control
owner, string/collection backing, and output cursor.  Construction, decode, history replay,
publication, and close advance one retained semantic unit per grant.  Interactive paths cannot
reach `serde_json::{from_str,to_string}`, `to_vec`, `collect`, `join`, whole `ArtifactDsl`/
`ArtifactPack`, snapshot clone, generic diff/apply, `resolve_ready`, or a direct store call.

Common source-static gate for **each** packet:

- its raw caller is absent while all out-of-packet raw callers remain;
- a domain-specific static predicate and fixture self-test prove mounted admission, retained
  construction/replay, paged output, freshness publication, progress/checkpoint, and terminal
  disposal; no renamed whole-buffer substitute qualifies;
- deterministic credit/progress/checkpoint ledgers are byte-identical; every page/item/byte/output/
  control counter is zero at terminal empty; and
- scoped `rustfmt --check`, scoped/whole `git diff --check`, and an independent source audit pass.

Cargo, Nx, native/Wasm/browser, replay, cancellation, memory, and 8 ms timing gates stay out of
the concurrent packets and belong to the final serialized matrix.

## Packet-Specific Ownership And Hostile Mutations

| Packet | Exclusively writable source scope | Required domain discriminator | Faithful hostile mutations (the local permanent test must fail) |
| --- | --- | --- | --- |
| P8yz-a | Procedural2d Wasm bridge plus its 2d schema/mutation/history cursor and local fixture region | 2d schema entry cannot be satisfied by 3d data | remove preflight; construct `String` before admission; omit one 2d field/mutation owner; replace candidate swap with direct store mutation; remove close owner; re-enable whole JSON/DSL output |
| P8yz-b | Procedural3d equivalents only | 3d schema entry cannot be satisfied by 2d data | same mutations, but target a 3d-only field/mutation and 3d combined-depth route |
| P8yz-c | Puzzle3d bridge plus 3d envelope/DSL cursor, document-to-FillBuilder binding, local fixtures | replacement during every FillBuilder phase retires the exact old fill/session/document authority while last valid preview remains | remove generation binding; let a stale fill publish/commit; retain `puzzle3d_parse_dsl_json`; omit completed-before-ACK disposal; remove one fill/document close grant |
| P8yz-d | Puzzle5d bridge plus 5d envelope/DSL cursor and local fixtures | 5d parse/output path is independently bounded | retain `puzzle5d_parse_dsl_json`; remove fixed page handback; erase a 5d mutation/history owner; weaken max/+1 output handling; remove stale-generation check |
| P8yz-e | FEM2d bridge plus 2d snapshot/mutation cursor and solver-session join | document replacement/validation fault moves the exact 2d mesh/assembly/PCG/visual owners into P6 disposal | omit a 2d solver owner; admit a solver after close; remove page handback; publish a stale result; bypass one close grant |
| P8yz-f | FEM3d equivalents only | 3d node/element/DOF/material/load/result caps and close depth do not inherit 2d limits | substitute the 2d cap/stack; omit a 3d solver owner; remove document-generation validation; accept maximum+1 combined document/solver credits |
| P8yz-g | Flow retained VCS subregion only; no unrelated forms/host rewrite | document generation invalidates exact Flow surface; widget/synapse/preview/expanded/history/host owners close incrementally | re-enable `ArtifactDsl`/`ArtifactPack` or `serde_json::Value` edit helper; leave a `FlowHost` undo/redo root; omit surface invalidation; publish stale generation; remove a control-backing close step |
| P8yz-h | narrow Dag VCS subregion only; no unrelated board formatting | document replacement retires GraphHost/sync-cache/scene/layout/preview through accepted close paths | re-enable direct `DagStore::new`; ordinary-drop a populated host; omit one node/edge/mutation owner; allow stale layout/preview; remove quiet-wake or surface invalidation; weaken combined-depth stack |
| P8yz-i | Shooting bridge plus shooting snapshot/mutation/history/output cursor and local fixtures | camera/shot/sequence/media reference closure and paged projection are retained | retain `projection_json`; omit one media/history owner; allow output maximum+1 allocation; remove cancel after complete; publish before fresh revision check |
| P8yz-j | CAD bridge plus CAD model/geometry/topology/property/reference/mutation/history cursors and local fixtures | decode/publication never invokes whole boolean/tessellation/serialization/render work | retain `resolve_ready`; invoke whole geometry work in decode; omit geometry/GPU control backing; remove preflight or generation check; ordinary-drop populated geometry after cancel |
| P8yz-k | `📜️script.ts` P8 predicate/mutation registration and, last, the shared guard definition | a reintroduced raw constructor or weakened domain predicate fails the aggregate verifier | delete one caller predicate; make a predicate accept `Option<String>`/direct `ArtifactStore::new`; mutate the final raw scan expectation; delete the guard while one caller remains |

Every packet additionally needs zero/max/+1 pages, items, bytes, strings, collections, outputs,
operations, and control owners; zero/insufficient fuel and expired deadline; cancellation/fault/
panic/handle loss before and after each transfer; wrong/stale/duplicate/ABA/exhausted handles;
interrupted close; terminal-empty idempotence; and deterministic native/Wasm ledger identity.  Its
single combined-depth fixture includes all legal domain nesting plus history/conflict frames; it is
not a sum of independently capped subtrees.

## Shared-File Rules

- Do not edit `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` in P8yz-a through
  -j.  Its guard remains fail-closed until P8yz-k.
- Do not edit `📜️script.ts` in P8yz-a through -j.  Put per-domain fixture mechanics next to the
  domain; P8yz-k serializes aggregate verifier wiring and avoids an eleven-writer collision.
- Reuse, do not fork, the mounted page/session APIs in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`.  Any required change there
  is a separate prerequisite packet, not a hidden edit in a domain migration.
- Puzzle packets must not edit the accepted P4 FillBuilder authority; FEM packets must not edit P6
  solver/disposer authority.  Bind to those established interfaces and prove the join locally.
- Flow and Dag are source-disjoint but reside in shared framework/product territory; narrow edits
  to the named VCS/Wasm regions and avoid formatting the surrounding 1,374/9,517-line files.

## Evidence

- Current raw scan: `rg -n "reject_whole_buffer_artifact_envelope_ingress" --glob '*.rs' --glob '!target/**' --glob '!**/🧪️tests/**'` returned 12 lines on 2026-08-25.
- Governing packet order and mandatory lifecycle: `📓️p8-whole-buffer-ingress-wave-order-2026-08-24.md`.
- Process3d exact owner/lifecycle census: `📓️p8yx-fresh-process3d-retained-envelope-ingress-census-2026-08-25.md`.
- Wider uncredited P8 command evidence: `📓️p8t-independent-remaining-tools-global-audit.md`.
