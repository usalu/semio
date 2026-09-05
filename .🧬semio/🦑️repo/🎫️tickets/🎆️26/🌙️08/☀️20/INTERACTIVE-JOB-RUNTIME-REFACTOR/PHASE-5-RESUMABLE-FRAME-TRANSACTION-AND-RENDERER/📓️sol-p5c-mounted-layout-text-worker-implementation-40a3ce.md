# P5c Mounted Layout and Text-worker Implementation — 2026-08-24

## Verdict

**SOURCE-READY for independent P5c audit.** The isolated P5c verifier accepts the live sources and
kills all 28 structural mutations plus all 15 law-body mutations. Scoped Rust formatting and diff
whitespace gates are clean. Cargo, Nx, Wasm, browser, and broad runtime gates were deliberately not
run under the packet boundary.

The full shared `verify interactivity` command reaches and accepts the P5c baseline, then remains RED
on eight concurrent findings outside P5c: four DB/P1 findings and four P5b live-reconcile findings.
Those failures were not modified or hidden by this packet.

## Counterexample-to-fix Map

| Contract counterexample | Bounded production repair | Source evidence |
| --- | --- | --- |
| `LayoutJob` retained dynamic vectors/maps after tree mutation | `MountedLayoutJob` preowns fixed node, walk, run, glyph, line, preview, result, and four 16-KiB atlas-page authorities | `mounted_layout.rs` lines 185–292 |
| Whole-tree recursion, stage fallthrough, and whole-string shaping | Persistent admission/node/text/unwind, run, glyph, measure, arrange, preview, publication, and close cursors advance one unit per call | `mounted_layout.rs` lines 294–604 |
| Direct UI-thread `FontAtlas::measure`/opaque text work | `OwnedTextWorker` accepts one retained scalar; its generation-tagged atlas reference is written into fixed pages and invoked only by `InteractiveJob::step` | `mounted_layout.rs` lines 91–172 and 410–491 |
| Worker ran on its caller and had no take/resume/close owner | `Ui::step_layouts` mounts `MountedWorkerJobSession`, submits only through the shared `WorkerPool` typed lane, separates take from publish, propagates cancellation, and incrementally closes checked-out/rejected/session owners | `engine.rs` lines 563–735 |
| Supersede/cancel/completion deep-dropped a job | Exact surface/layout/tree/theme/viewport identity mismatch begins retained close; no supersede path assigns away a live owner before terminal-empty | `engine.rs` lines 576–705 |
| Theme equality reset every job and generations could alias | Layout-relevant theme equality gates invalidation; tree/theme/viewport/layout revisions use checked preflight and reject exhaustion | `engine.rs` lines 334–526 |
| Dynamic string surface queues and whole-queue lane scans | Fixed 64-slot generational surface registry and fixed per-lane rings retain `SurfaceId`/`UiSurfaceToken`; lane changes are lazily reinserted in one opportunity | `engine.rs` lines 82–286 and 725–755 |
| Partial live-node publication exposed mixed geometry | Results write the inactive per-node slot; completeness and full identity are rechecked; one `mounted_layout_active ^= 1` commits the generation exactly once | `mounted_layout.rs` lines 605–636; `tree.rs` lines 272–375 |
| Paint and hit testing could disagree with publication | Paint, event hit testing, absolute scene commands, and scene-slot collection resolve `UiTree::accepted_layout` | `paint.rs`, `events.rs`, and `scene_slots.rs` accepted-layout call sites |
| No progressive result while pending | One geometry preview and one generation/revision-checked glyph preview remain retained on `UiWindow` and are exposed read-only | `engine.rs` mounted outcome and introspection regions |
| Renderer never drove the retained layout worker | Both retained node and production retained-document routes drive one fuel/deadline opportunity before input dispatch/frame through `renderer_worker_pool` | `Interpreter/🧊️component.rs` lines 1137–1157, 1197, and 1283 |
| Production legacy fallback retained dynamic Taffy maps/vectors | Legacy `LayoutJob` is test-only; production imports and `UiWindow` no longer own `LayoutEngine`; `mounted_layout` is mounted by target glue | `flex.rs`, `engine.rs`, and UI-wgpu `📦️glue.rs` |

## Exact Hostile-law Evidence

The laws execute the production types and paths, not decorative capacity containers alone:

- actual glyph MAX+1 retains the rejected scalar and closes to terminal-empty;
- actual wide-tree node MAX+1 retains the rejected tree owner identity;
- 16,384 four-byte Unicode scalars cross all four fixed atlas pages with at most one glyph or page
  boundary per turn;
- depth MAX+1 retains the rejected walk authority and releases exactly one owner per close call;
- a mounted session submits on `Lane::UserVisible` and observes the named shared-pool worker thread;
- cancellation is asserted both before and after the owned text call;
- an expired deadline advances zero glyphs and partial close releases exactly one owner;
- stale surface identity faults, inactive writes leave the accepted generation unchanged, completion
  swaps once, and repeated Ready does not swap again;
- surface MAX+1 returns the exact `SurfaceId` without registry mutation;
- equal theme preserves generation/revision/queue state;
- pending resize preserves the old accepted geometry, then changes generation once and retains both
  progressive layout and glyph previews;
- tree/theme/viewport `u64::MAX` cases refuse without alias;
- resize supersede/replay returns identical accepted geometry;
- weighted scheduling services background within one wheel and the hostile 1,025-node/text workload
  asserts more than 10,000 turns with every observed caller slice below 8 ms.

Production law bodies are in `mounted_layout.rs` lines 819–1005 and `engine.rs` lines 1038–1214.

## Permanent Verifier

The exact `P5cMountedLayoutText` region in root `📜️script.ts`:

- filters test-only legacy source before structural production checks;
- rejects dynamic working sets, whole-stage loops, whole-string measure, UI-thread stepping, second
  scheduler creation, wrapping identities, dynamic surfaces/lanes, partial publication, bulk close,
  missing cancellation/freshness/completeness, missing accepted-snapshot consumers, and a missing or
  reordered renderer driver;
- inspects each hostile function body for its substantive assertion evidence;
- proves the baseline only after 28 structural mutations and 15 individual law-body evidence
  mutations are rejected.

Evidence: `📜️script.ts` lines 6612–6620 and 7707–7889.

## Gates

### Passed

```text
bun -e 'import { interactivityMountedLayoutTextSelfTests } from "./📜️script.ts"; interactivityMountedLayoutTextSelfTests(process.cwd()); ...'
[verify interactivity p5c] live-source clean; structural-mutations=28; law-body-mutations=15
```

```text
rustfmt --edition 2021 --check <11 exact P5c Rust files>
exit 0
```

```text
git diff --check -- <12 exact P5c source files>
exit 0
```

### Shared full verifier residual

```text
bun ./📜️script.ts verify interactivity --severity error
P5c baseline and mutation self-tests: accepted
aggregate result: RED on 8 unrelated concurrent DB/P1 and live-reconcile/P5b findings
```

### Deliberately deferred

- Cargo/test/build
- Nx
- Wasm compilation/runtime
- browser/runtime timing harnesses beyond the retained hostile law source

These were prohibited by the isolated packet instructions and are not reported as passing.
