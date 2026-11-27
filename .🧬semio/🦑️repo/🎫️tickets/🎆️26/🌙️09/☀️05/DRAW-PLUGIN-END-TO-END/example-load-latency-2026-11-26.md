# Example switch latency (Draw), 2026-11-26

## Verdict so far
The latency is NOT host poll cadence and NOT (mainly) waiting for read leases. It is guest work: a Blank/boot-Demo archive load executes ~149,000 guest maintenance steps (512 per guest turn, 289 turns) costing ~12.9 s of `worker.guest` wall time. Native per-phase attribution is NOT done (see Blockers).

## Evidence (browser, headless, dev server :6064, `probe-example-latency.mjs`, hop-trace measures)
- `nextArchivePollTurnV1` is a 32 ms timer, but only 2-3 polls are ever sent per load: the first `PollDocumentArchiveLoad` blocks 9-15 s because the host settle loop keeps draining `more-work` guest turns inside that single call; the next poll returns `ready` in ~150 ms.
- Boot Demo load: poll 11-15 s. Demo -> Blank: poll 9-13.5 s. Blank -> Demo: poll ~1 s (UI shows 1 layer ~1.5 s after the pick). So the cost follows the displaced document (Demo, with the 29 KB base64 PNG), not the loaded one.
- Demo -> Blank breakdown: `worker.guest` 289 turns / 12.9 s (max single turn 147 ms in the latest runs, 3.2 s once on a loaded host); `worker.reply` 0.1 s; `turn.accept` 0.02 s; React/refresh negligible. Host overhead is not the cost.
- Steps per 2 s bucket: 18.9k, 7.7k, 8.7k, 10.8k, 9.7k, 12.8k (about 0.15-0.25 ms each, ~68k real steps in ~10 s), then 80k in 0.85 s (cheap `Blocked`/empty spins while the displaced store waits in RetiringCommittedStore). Progress counter moves only 1136-2177 units over ~149k steps (~70-130 steps per unit).
- Slow-step TRACE (runtime diagnostics armed) shows only 33 steps >= 2 ms (stages 8, 14 and the idle sentinel), so no single monolithic step; it is step count.

## Not proven
Which ladder unit/stage owns the ~68k real steps (candidate: displaced-store retirement walking the Demo snapshot one retained item per step; hydration paging is ruled out as the dominant part because Blank -> Demo hydrates the same-size archive in ~1 s).

## Blockers
The native measurement (`repo capacity run --memory=4608 ... cargo test -p semio-s-artifact-draw-drawing-editor demo_example_load -- --nocapture`) queued behind other agents' reservations for 48+ minutes (spare memory negative; 2.7-3 GB reserved-not-resident). Cancelled. No source fix was made.

## Files changed
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️archive-load/🦀️.rs`: `settle_archive` returns `(status, rounds)` and prints temporary `[DEBUG] round N completed a/b` and `[DEBUG] archive rounds N` lines (remove once the bound assertion is calibrated). Not compiled yet.
- `probe-example-latency.mjs` (ticket): browser probe (PROBE_DIAG=1 arms guest diagnostics).

## Update (code reading, no build)
- The displaced store retires through `ArtifactStoreCursorDisposerPhase` (ReturnedReads, Displaced, RuntimeOwners, PrefixSnapshots, TailSnapshot, CurrentSnapshot, Backbone, Causal, Envelope; `⚙️engine/🧯️close/📥️lifted/🏪️store/🦀️.rs`). Every snapshot root (prefix ring entry, tail, current) goes through the snapshot retirement factory; Draw's owner is the derived `DrawingRetirementOwner` (`🧰️owned/♻️retirement/🦀️.rs`), so per-field/per-element cursors are generic `PagedListRetirement` (`🌱️value/♻️retirement/🦀️.rs`), which releases plain `Copy` elements per page but steps every `needs_drop` element (each String, each nested list) one item per step, and string pages are 4096 bytes (`PAGED_BYTES_CHUNK_BYTES`).
- Leading hypothesis: the Demo store carries many retained prefix snapshots (history replay at hydration) that each own a copy of the 29 KB base64 asset plus layer lists; retirement of N such roots is N x (pages + elements) steps. Unverified: needs the native round log (`archive-load` law prints rounds per progress change).
- Runtime guest diagnostics (`SEMIO_RUNTIME_DIAGNOSTICS=1`) only trace maintenance steps >= 2 ms (33 in a Demo->Blank switch, stages 8/14/idle), so they cannot attribute the ~68k fast steps.
- Native run queued: `repo capacity run --memory=4608 ... cargo test -p semio-s-artifact-draw-drawing-editor demo_example_load -- --nocapture`, output to `🗑️generated/rounds-4.txt`; after 1 h still waiting on others' reservations. No fix landed; the law file is edited but not yet compiled.

## Update 3 (value crate layer)
- Draw has no custom per-item retirement: `DrawingRetirementOwner` is a derive (`RetireOwned`) over framework cursors; no `release_string` exists in `🧰️owned/`. `DrawingImageAsset.samples` is `PagedList<[u8;4]>` (decoded pixels, plain => page-wise) and layer paths are `PagedList<PathSegment>` (all-plain enum => page-wise). Plain data is already paged by the framework (`plain-elements` laws); owning elements (String, nested lists, structs with owners) are element-wise BY DESIGN and asserted so (`a_vector_with_an_owning_field_stays_element_wise`): each costs a pop step plus its own cursor steps in `PagedListRetirement::close_step`.
- A draft law `paged_lists_of_samples_strings_and_nested_lists_retire_in_few_turns` (16,384 `[u8;4]` samples, 1,000 strings, 100 nested lists of 64 `[f64;2]`, grant 4096 bytes / 64 items) is saved as `value-retirement-law-draft-plain-elements.rs` in the ticket folder (it generalises `drain_turns` with an items parameter). It was never executed: `semio-framework-value-retirement` fails the pre-compiler source boundary ("23 of 32 files, 266240 of 262144 bytes"; `🎮️controlled/🦀️.rs` alone is 58 KB, the main file 84 KB), with or without the draft; a peer is editing `🎮️controlled` and `📋️queue` (they were removing `[DEBUG]` prints). I reverted the draft out of the crate so it does not add bytes.
- A peer's temporary prints in those files ("controlled step grant ... does not admit ...", "reserve_step early") suggest the same hunt: steps returning empty progress because the grant does not admit the quoted demand, which would also spin a guest step without progress (cheap, matching the ~80k fast steps).
