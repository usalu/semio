# Puzzle 3D Fill Envelope Root-RED Scout

## Verdict

**RED — the retained worker still builds a whole preview envelope in one grant.** The precise live site is `FillBuilder::publish_preview`: it serializes the complete mutable `FillBuildPreview` with `serde_json::to_vec(&self.preview)`, copies those bytes into `RetainedJobPayload`, and returns that payload. The companion final commit is already an empty retained state/output pair; it is not the current blocker.

The root verifier is correctly keyed to this distinction: [📜️script.ts](../../../../../../../../📜️script.ts:8575) requires literal empty preview publication and empty commit envelopes. Static probes found the required empty-preview token absent, the whole preview serialization present, and both empty commit envelopes present. No Cargo/Nx/Wasm/browser or production mutation was performed.

## Exact Production Trace

| Boundary | Evidence | Finding |
|---|---|---|
| UI action | [fill-build-tick command](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🪣️fill-build-tick/🦀️component.rs:18) polls then calls `enqueue_fill_job`; its two live callers are the ordinary and restored tick paths at :25/:45. | Production entry, not a test-only path. |
| Retained admission/worker | [precompute session](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️component.rs:1499) moves `engine.fill` into a measured registry owner. [worker drive](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️component.rs:493) makes one `drive_step` call at :525 with a 32-fuel, 2ms budget. | Keep this identity/admission/handback protocol; do not replace it with a new worker wrapper. |
| Fill semantic state | [FillBuilder state](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs:270) retains preview, target/candidate cursors, collision cursor, acceptance cursor and fixed-owner support. [InteractiveJob step](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs:3386) dispatches exactly one stage arm. | This is the correct unit-of-work owner. |
| **Whole worker preview** | [publish_preview](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs:3337) writes metadata then executes `serde_json::to_vec(&self.preview)` at :3350 and `payload_from_bytes` at :3351–:3354. | Root RED. This must become a retained typed/shared preview observation; the worker payload must be empty. |
| Final result | [complete](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs:3357) already returns empty `CommitState` and `CommitOutput` payloads at :3359–:3360. | Preserve this behavior. |
| Renderer output | [world_fill_preview_json](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️component.rs:404) reads the shared fill progress at :408, but serializes the whole `build` into `fillBuildPreview` at :418; [render](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️component.rs:469) publishes it. | Separate presentation blocker: even after worker payload is emptied, renderer must project only the bounded typed preview fields it needs, never serialize the aggregate `FillBuildPreview`. |

## Required One-Semantic-Unit Primitives

The existing stage machine is the correct decomposition boundary. Each listed primitive needs to retain its cursor/phase and finish after one item, pair, sample, or fixed page operation:

1. Admission: `FillBuilderOwnerCensusCursor` / `FillEnvelopeAdmissionCursor` (owner page or entry) before registry reservation; [precompute](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️component.rs:1499).
2. Preparation: fixture/catalog/mesh/entry/spatial/lookup/configuration one owner each; [prepare_one](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs:2460).
3. Target plan: blocked-vortex, object/vortex enumeration, weight-tree build and weighted pick each one cursor advance; [prepare_targets](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs:2670).
4. Candidate plan: kind/vortex enumeration, classification, map drain, weight-tree build and pick each one cursor advance; [prepare_candidates](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs:2805).
5. Placement: select one target/candidate, create one ghost, build one broad-phase query, then test one collision pair and one resumable overlap sample sequence; [construct](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs:2929), [query](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs:2974), [collision](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs:3013). Geometry’s cursored query/mutation/overlap types are owned in [geometry](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/📐️geometry/🦀️component.rs:842).
6. Acceptance: validate, each attraction/vortex, spatial mutation, lookup and commit phase each one retained step; [accept_candidate](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs:3064).
7. Publication: update only the compact observation/sequence metadata and signal `PreviewReady(Vec::new())`; renderer obtains a bounded typed read of that shared observation. Do not add a second envelope, JSON snapshot, or copy of `FillBuildPreview`.

## Non-Overlapping Packet Split

| Packet | Owns | Must not touch |
|---|---|---|
| PZL-FILL-STATE | `precompute/🪣️fill`: replace :3350–:3354 with empty preview publication and a retained typed observation/page protocol; retain all stage cursors and empty commits. | registry, renderer, geometry |
| PZL-FILL-BRIDGE | 3D `precompute`: observation read/ACK and worker-to-session publication only. Preserve fixed admission, token, cancellation, terminal handback. | FillBuilder semantic logic, renderer |
| PZL-FILL-GEOMETRY | geometry only: prove broad-phase/replacement/overlap each consumes one semantic unit under the same budget/cancel boundary. | preview transfer/UI |
| PZL-FILL-PRESENT | 3D window renderer and fill tool/action presentation: project bounded ghost/progress fields, remove aggregate `serde_json::to_value(build)`. | worker/admission and geometry |
| PZL-5D-ADAPTER | 5D adapter only after the 3D observation API is stable. | 3D state packet |
| PZL-2D-INVENTORY | 2D fill-session is a separate engine and requires its own contract before modification. | 3D packet |

## Existing Tests and Acceptance

Source contains useful but not executed coverage: `preview_payload_is_typed_revisioned_and_bounded` ([fill :3896](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🦀️component.rs:3896)), `adversarial_broad_phase_fill_is_end_to_end_resumable_below_eight_ms` (:3948), and the fixed-token/ABA/handback family beginning with [precompute :2293](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️component.rs:2293). They do **not** presently prevent the JSON whole-preview line, since the current test merely asserts the payload’s bytes after serializing it.

Static acceptance for remediation:

- `publish_preview` has no `serde_json`, `to_vec`, whole `FillBuildPreview` encode, or `payload_from_bytes` of preview; its only worker preview result is `StepOutcome::PreviewReady(Vec::new())`.
- The final `CommitState` and `CommitOutput` stay empty; worker does not create a result/state envelope by another route.
- The renderer has no `serde_json::to_value(build)` / whole `fillBuildPreview` insertion; its output is bounded and fieldwise.
- Every stage above has a durable cursor plus fuel/deadline/cancel check before another semantic unit; geometry collision remains resumable.
- Tests cover zero/max/+1 preview-page capacity, stale/ABA/cancel handback, one worker grant publishing no preview bytes, and a language-neutral fixture checked with the owned test-only third-party oracle. Run the root `verify interactivity` only after source packets are no longer active; it is intentionally not run in this scout.

## 2D/5D and Launch Scope

- **2D is separate ownership.** Its production fill is the `fill-session-*` command family, e.g. [step](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️fill-session-step/🦀️component.rs:6), plus its own Fill tool. It has no `FillBuilder`/`enqueue_fill_job` reference in the census.
- **5D shares the 3D solver but not the 3D UI boundary.** [Puzzle5dPrecomputeSession](../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧠️precompute/🦀️component.rs:19) owns an inner `Puzzle3dPrecomputeSession` at :20, and its `fill_progress` JSON wrapper is at :57–:59. Therefore the state/geometry fix is shared; its 5D JSON adapter must be a separate follow-up rather than silently changed by the 3D renderer packet.
- Launch-derived 3D concrete-forest instances all run `puzzle3d` ([.vscode/launch.json](../../../../../../../../.vscode/launch.json:1542)); 5D concrete-forest and capsule-dream run their own 5D launch commands (:1644, :1695); 2D/3D/5D each have React, WGPU-Wasm and WGPU-native launch registrations beginning at :1440/:1491/:1593. These are product variants, not additional FillBuilder implementations.

## Discovery Commands

```sh
rg -n 'interactivityPuzzleFillEnvelopeSelfTests|Puzzle FillBuilder|whole preview|FillBuilder' 📜️script.ts
rg -n 'StepOutcome|PreviewReady|RetainedJobPayload|serde_json|to_vec|to_value|FillJobStage' <3d fill/precompute/geometry/editor/renderer files>
rg --files '✏️s/🔌️plugins/🧩️puzzle' | rg '(🩻️2d|👯️5d|2d|5d).*(🪣️fill|fill|precompute)'
rg -n -i 'puzzle.*(2d|3d|5d)|puzzle(2d|3d|5d)' .vscode/launch.json
```
