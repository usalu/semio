# 🚀️ Development Speed: Status and Long-Term Recommendations — 2026-11-28

## Draw status (00:00)
| Area | State |
|---|---|
| TypeScript (91 files, strict tsc incl. test files) | ✅ green |
| Component build + descriptor + Draw registry | ✅ `d6d1e5e2…` built, staged, served |
| Viewer journey en + de (render, zoom, back to Editor, zero faults) | ✅ pass |
| Native: host 37, style 47, structure 34, editor-interaction 42, scene 49, scene-mounted 15 | ✅ green |
| Native: viewer (14/15 → fixes pending verify), model, transform, editor ×3, workspace check | ⏳ queued / running |
| Editor journeys en + de | ❌ Demo→Blank: second store replacement's refused worker session never re-admits (framework) — agent investigating |

## Fixed in this ticket
- Draw: viewer dropped canvas actions; scene read held until ~6,800 private owners drained (bounded early hand-back, PNG export kept strict); host test-kit pool budget; viewer `SetCamera` command/argument shape; window-config receiver/encoding mismatch (viewer, editor, plus Layout); missing `replay_input` on Draw's `GeometryJob`; viewer test actor model and key-order expectation.
- Repo-wide: missing `maintenance_starved_mark`; over-boundary crate from a stray debug line; 20 GB stale ticket build output; post-reboot detached cache images (`repo doctor --fix --only=storage`) and empty Cargo registry; stale isolated stores; warden admission deadlock from idle compressed pages (noted in the OOM-pause ticket).

## Where the time went
About 80 % of wall time was waiting, not working: host memory (16 GB, leaking `fseventsd`, warden holds), peers' in-flight framework edits breaking shared crates, size-boundary refusals, capacity-queue starvation, reboot fallout, and 30-minute component rebuilds per verification loop. Draw-specific defects took a small share.

## Long-term recommendations (highest leverage first)
1. **Never break the shared tree.** Peer edits broke builds about 10 times (`job-context` carve, `replay_input`, capacity policy, cache policy, debug lines over the size boundary). Gate replica propagation of `🧰️framework` Rust/policy edits on a fast `cargo check` of the edited crate and its direct dependents (plus schema validation for policy JSON), or stage multi-file carves atomically (write all files, then publish). A broken shared crate costs every agent on 50 machines.
2. **Warden and capacity: liveness before caution.** Idle pages of processes waiting for admission must not count toward the line that holds them; a hold triggered only by compression under normal kernel pressure needs a bounded escape (admit the oldest waiter alone). Reserve sizes per lane must fit the image (unit-cache budget 25 GB inside a 26 GB Rust image). Publish one `repo doctor` finding when every grant is held.
3. **Fix the host before scaling agents.** `fseventsd` grew to 15 GB from replica churn; ignore build caches and generated folders in FSEvents watchers or move them off the watched tree, and keep ticket `🗑️generated` output out of replication (this ticket alone carried 22 GB). Add a doctor check for `fseventsd` footprint and swap state.
4. **Cheap verification loops.** A Draw change needs a 30–60 min, ~6 GB wasm component link before any browser check. Provide a native host harness that runs the same browser journeys against the native guest (or a dev component split so only Draw crates relink), and make the fleet CAS usable (its 77 GB reserve exceeds free disk, so every offload falls back to local).
5. **Make long-running agents restartable and non-blocking.** Standardise one serial "build queue" script per ticket (gate on memory, peer builds, retry only on infrastructure, count only real verdicts) as a repo command instead of every agent re-inventing retry/gate loops and spawning dozens of waiter shells.
6. **Prefer contract laws at the boundary.** The worst Draw bugs were contract mismatches (receiver vs encoder, command vs action args, trait method added without implementors). Derive both sides from one declaration (macros already exist: `field_set_mutations_binary!`, `original_pack_value_operation_receiving!`) and add a workspace law that every `BoundedJob`/window-config owner pairs its encoder and receiver, so the compiler or one fast test catches it, not a 30-minute browser loop.
7. **Ticket hygiene.** Debug lines must never be committed (several `[DEBUG]` prints are in HEAD); add a pre-replication check rejecting new `[DEBUG]` eprintlns in framework crates outside tests.
