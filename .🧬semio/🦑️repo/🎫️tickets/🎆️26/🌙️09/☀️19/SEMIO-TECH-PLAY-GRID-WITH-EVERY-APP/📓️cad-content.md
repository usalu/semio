# 📐️ cad-content — report (topic owner, sessions 5 + 6)

Ticket `26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP`. Session 6 (successor) ran 2026-09-22 01:28–02:45.
Everything below that is labelled "ran" was executed in the session named; nothing is inferred from a
prior agent's claim. Logs: `🗑️generated/cad-content/run*.txt`, probe evidence `…/probe-0922/`.

## 1. Crates in scope (6)
`semio-s-artifact-cad-cad`, `semio-s-plugin-cad`, `semio-s-plugin-cad-aec-building`,
`semio-s-plugin-cad-aec-building-energy`, `semio-s-plugin-cad-aec-building-structure`,
`semio-s-plugin-cad-spatial-shape`. None declares `component-app-assembly`.

## 2. Native test numbers

| run | when | what | result |
|---|---|---|---|
| `run1.txt` | 09-21 14:07 (s5) | all 6 crates | **13 failed / 417 passed**, all 13 in `semio-s-artifact-cad-cad --lib`; other 5 green |
| `run4.txt` | 09-21 16:00 (s5) | all 6 crates | **3 failed / 427 passed / 1 ignored** in `artifact-cad-cad`; plugin-cad 5✅, aec-building 8✅, aec-building-energy 2✅, aec-building-structure 2✅, spatial-shape 2✅ |
| `run6/7` | 09-21 17:38 (s5) | all 6 | no result — peer break in `semio-framework-plugin` (`send_member_mutations`/`take_member_inbound` gone) |
| `run9` | 09-21 22:14 (s5) | all 6 | no result — **infrastructure, not cad** (see §4) |
| `run11.txt` | 09-22 01:33 (s6, mine) | all 6 | **no result** — 8 framework units in 52 min, then killed silently at 01:41 with no error line. Not the test watchdog (its log's last kill is 21:50). Machine was in swap exhaustion: `vm.swapusage used = 12 314 MB / 13 312 MB`, free 998 MB; free disk fell 113 GB → 74 GB (92 % full) during the run |
| `run12.txt` | 09-22 02:26 (s6, mine) | `artifact-cad-cad --lib`, `CARGO_BUILD_JOBS=1` | **still blocked on an artifact-directory lock at hand-off — no result** |
| `run14.txt` | 09-22 03:28 (s6, mine, **via `📜️native-test-mutex.sh`**) | all 6 crates | **2 failed / 428 passed / 1 ignored** in `artifact-cad-cad --lib`; plugin-cad 5✅, aec-building 8✅, aec-building-energy 2✅, aec-building-structure 2✅, spatial-shape 2✅ — **27 units, whole run under 20 min** |

**✅ CURRENT NUMBERS (run14, 2026-09-22 03:28, mine): `semio-s-artifact-cad-cad --lib` 428 passed /
2 failed / 1 ignored; the other five cad crates fully green (5, 8, 2, 2, 2). Total 2 red across all six.**
That is 3 red → 2 red versus run4, and the crate went 427 → 428 passing: the extra pass is the
`forest_transformation_uses_live_shape_pane` fix in §3, confirmed `... ok` in run14.

`XCUT-TOOLPROOF is clear for cad` — zero `tool factory proof rejected tool` failures in run1 or run14.

**What finally made a run possible:** the fleet's new `📜️native-test-mutex.sh` (one native cargo at a
time). Runs 11–13 were all lost to contention — 52 min for 8 framework units, a silent swap kill, an
artifact-directory lease block, then 52 min parked on peer unit locks. Serialised through the mutex the
same 6-crate batch compiled 27 units and ran to completion in under 20 min. A private `CARGO_TARGET_DIR`
alone does not fix this: it clears the artifact-directory lease but not the shared build-dir unit locks.

## 3. Fix landed this session — VERIFIED GREEN in run14
`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `forest_transformation_uses_live_shape_pane` (run4's 3rd red).
Root cause, read from source, **not** the Brep handle collision session 5 suspected:
- `run_derive_from_geometry` is `#[cfg(test)]` scaffolding (`…/🧬️schema/💡️inferences/🦀️.rs:303`) → no production impact.
- `FROM_GEOMETRY_CLASSIFY_RULES` (same file, 116–122) keys **only** on a face's normal and z-band.
  There is no extent, aspect-ratio or area term, so a 1×1×1 cube and a 4×0.2×3 wall panel (the extents
  `make_object_for_typology` assigns, `✏️editor/🦀️.rs:829-834`) *necessarily* yield the identical
  typology multiset `[hull, externalwall×4, roof, baseplate, windows]`. The old `assert_ne!` on the
  typology list could never hold on shape grounds.
- It passed in run1/run2 and failed in run4 because the derive groups faces through a **`HashMap`**
  (`for (_key, faces) in grouped`, 362–374): iteration order is randomised per process, so the test was
  comparing two randomly-ordered lists and passed or failed by luck.
- Fix: assert the order-insensitive typology multisets are **equal** (the real normal-only contract) and
  that the classified faces' **centroids** (`origin: face.centroid`, line 380) **differ** — extent is the
  one input that reaches the output honestly, via `solid_for_object`'s `box_prim(extent)` fallback.
  Nothing weakened, nothing ignored, no fixture touched.
  **run14 verdict: `forest_transformation_uses_live_shape_pane ... ok`.**

Session 5's other fixes are in the tree and auto-committed (`git status` for `✏️s/🔌️plugins/📐️cad` was
empty at session-6 start; last commit `ef2210a418`): `deny_unknown_fields` on `CadArtifact`/`CadSnapshot`/
`CadDiff`, `new_app_with_registry_and_members::<_, SemioMembers>` at 4 call sites, the `detach_backbone`
pair before close, and `settle_registered_typed_operation(app, INSTANCE)` replacing the wrong-receiver
hand-rolled drain (`…/🪟️windows/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs:65`). **These were the edit
session 5 died mid-way through; it is complete and coherent, not half-applied.**

## 3b. The 2 tests still red in run14, with their exact run14 diagnostics

**(a) `cad_document_contract_world_window_runtime_isolates_commands_and_restores_exact_owner`**
Session 5's extended assertion message finally fired and names the offenders:
```
CAD exact-window pack ownership changed, expected exactly cad-shape-left/cad-play-shape:
[("cad-building", "cad-play-building"), ("cad-shape-left", "cad-play-shape"), ("cad-shape-right", "cad-play-shape")]
```
`window_config_packs()` returns **three** packs where the law demands exactly one. Two things to separate
before fixing (I did not get to run this down, so I am not guessing which):
- `cad-shape-right` — `assert_exact_state` renders BOTH `left` and `right` immediately before the check,
  so this pack may simply be lazy default materialisation on render, i.e. the law's "exactly one" is
  stale w.r.t. the current framework and should assert "exactly one pack *that a command wrote*".
- `cad-building` — harder to explain benignly and the more interesting one: every command was dispatched
  with `view = left` (a `cad-play-shape` window), but three of them carry a foreign `pane`
  (`SetCamera pane=cad.play.scene3d/building`, `SetDislocateOption pane=building`, …). If a command
  dispatched in the shape window can write the *building* window's config, that is exactly the
  cross-window leak this law exists to catch, and the defect is production-side, not test-side.
Note each `dispatch()` already asserted it published exactly ONE `WindowConfig` result lane and that
passed, so the leak (if it is one) is in which window the single result is keyed to.

**(b) `two_instances_converge_disjoint_edits_via_backbone`**
Session 5's `detach_backbone` + settle got it past convergence; it now dies later, in framework code:
```
🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2839
artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority
detached every nested owner
```
This is the store-drop-witness bucket (v2 bucket 2): a nested owner is still attached when the envelope
drops. Next step is to find which nested owner the two instances leave attached and retire it before
drop — the panic is in the framework's witness, but the missing detach is almost certainly test-side.

## 4. run9 was not a cad defect
run9's 25 errors are one cascade from `error[E0463]: can't find crate for serde which
semio_framework_os_kernel depends on`: the private `CARGO_TARGET_DIR` never received the uplifted
dependency rlibs (both `🗑️generated/cad-content/target` and `target2` are 4.0 K — only `CACHEDIR.TAG` and
an empty `debug/`). The 8 × `unresolved import crate::editor::cad::terminology::CadLabels` are downstream:
`CadLabels` **is** generated by `semio_framework_plugin::app_labels!` (`…/🗣️terminology/🦀️.rs:11`) and the
macro cannot expand while `semio_framework_plugin` fails to resolve. **No rename happened; no source fix
is needed.** Session 6 therefore reverted to the shared target dir.

## 5. What `#cad` renders on `:6033` — ANSWERED: all four panes, correct content, zero errors

Two read-only playwright probes, `--use-angle=metal`, one page at a time, both run by session 6.

**Probe B — 2026-09-22 03:12, AFTER the coordinator's activation finished (03:04:52, rc=0, 218 nx tasks,
all lanes incl. `activate-demonstrator-react-dev` which covers cad). This is the authoritative answer.**
Evidence: `🗑️generated/cad-content/probe-0922b/{cad.png, cad-console.txt, cad-report.json}`.
- Host element ends with `data-shell-ready="cad"` and **no** `data-shell-error`.
- **4 `<canvas>` elements**, each 788×437 CSS 789×438 — one per pane.
- 115 SVGs, 47 text rows naming all four panes with full per-window chrome:
  `Shape`, `Building`, `Energy`, `Structure Classic`, each with `Projection` / `Window Options` /
  `Actions` / `Search` / `Utilities`.
- **0 console errors, 0 page errors, 0 refusals, 0 intake rejections.**
- The 466 936-byte screenshot shows the `Demo` example rendered and VISIBLY DISTINCT in all four panes:
  the coloured site plan (the concrete-forest reference strips) under a grey canopy — Shape and Building
  show the full slab-on-columns assembly, Energy shows the derived roof hull, Structure Classic shows the
  roof with its structural members. Each pane carries its axis gizmo.
- Note: the probe's pixel census reports `0,0,0,0 100 %` for the canvas readback. That is a
  `drawImage`-of-a-WebGL-canvas artifact (no `preserveDrawingBuffer`), **not** an empty pane — the
  screenshot is the reliable evidence and it is full of content.

**Probe C — 2026-09-22 03:32, re-probe requested by the coordinator. Identical to probe B, so the
result is stable, not a one-off.** Evidence: `…/probe-0922c/`.
- `data-shell-ready="cad"`, no `data-shell-error`; **4 canvases**; 115 SVGs; 47 text rows naming
  `Shape`, `Building`, `Energy`, `Structure Classic`; screenshot byte-identical to probe B (466 936 B).
- **0 errors, 0 refusals, 0 admission rejections.**
- The page-wide `[stale]` banner is down to **2 entries** (it was 32 at probe A) and `cad` is not among
  them — independent confirmation that the 03:04 activation landed and the serve is on the new union.

**Probe A — 2026-09-22 02:27, DURING the activation (it was still running; it finished 03:04:52).**
Evidence: `…/probe-0922/`. Recorded here because it is a real failure mode worth recognising, not a
standing defect:
- shell reached ready then faulted to `data-shell-error="cad"`; **zero canvases**; 32 KB screenshot;
  the only body content was `plugin-ui.intake-rejected:intake:Owned UI patch admission rejected`;
  console: `PluginRuntime: actor cad#1 stopped without publishing requested UI surfaces
  (missing=[artifact/catalogue/inspection/history panels + engagements/measures/tools/catalogue
  sections], status=idle, faults=[])` and `setActiveExample refused: dispatch-failed`.
- `cad` was absent from that page's 32-entry `[stale] source-newer` banner, which initially looked like
  "current guest, live defect". It was in fact a **mid-activation host/guest mismatch**: the serve was
  still running off the pre-activation union receipt (last serve start 01:43:52) while the lanes were
  being restaged underneath it. Probe B, on the same unrestarted serve after the activation completed,
  is clean — so nothing in the cad guest needs fixing for this.
- Lesson for the fleet: a pane probed while `describe-all-then-activate` is running can present as a
  total UI-admission failure (`Owned UI patch admission rejected`, actor idle, all surfaces missing)
  with no staleness warning for that plugin. Always check
  `🗑️generated/activation/describe-all-then-activate-*.txt` has logged its `rc=0` before judging a pane.

## 6. `:6033` availability (fleet-wide, not cad)
At 01:27–01:43 the server was **down, crash-looping every ~75 s**. Cause, from
`🗑️generated/serve-6033-supervised.txt`: `mergePlayActivationReceipts`
(`🎡️play/🔨️modules/🧩️runtime/♻️activation/🟦️.ts:82`) threw
`Stale play activation lane: flow is 2ccbc653… in demonstrator but 847c2359… in flow`, so
`readPlayActivation` aborted before the listener ever bound. Per fleet-brief v4 I ran no
`activate-*-react-dev`; I touched `🗑️generated/activate.request/{flow,demonstrator}` at 01:32.
The server was serving 200 again by 02:26 and the probe above succeeded.

## 7. Machine conditions during session 6 (context for runs 11–13)
16–36 concurrent peer cargos on 10 cores; my invocation held all six cad build locks but took 52 min to
clear 8 framework units, blocked behind `semio-framework-ui-contract (a84da951e9595d8e)` whose holder
(pid 1818, `cargo test --features component-app-assembly -p semio-s-artifact-fem-3d`) ran for ~40 min.
Swap was exhausted (998 MB free of 13 312 MB) and free disk fell 113 GB → 62 GB across the session; run11
died with no error line and no watchdog entry. A private `CARGO_TARGET_DIR` does **not** solve this — it
clears the artifact-directory lease but not the shared build-dir unit locks, and it is what broke run9.
**The fleet's `📜️native-test-mutex.sh` did solve it** (run14: 27 units, complete, under 20 min).

## Session 7 — 2026-09-22 11:00–17:00

Durable working home after the 16:0x sweep: `.🧬semio/🦑️repo/⚡️cache/play-fleet/cad-content`
(`STATUS.md`, `run18.txt`, `run19.txt`, `target/`). **At ~16:05–16:25 the repo's workspace-cleanup
deleted most of `$T/🗑️generated`** — my session-7 run logs (`run16.txt`, `run17.txt`), the probe
evidence folders `probe-0922{,b,c,d}/`, the shared `activation/` logs and the first `STATUS.md` went
with it. Every number and quotation below was read out of those logs before they were deleted; the
source edits live outside `🗑️generated` and are intact.

### 9. Numbers

| run | when | result |
|---|---|---|
| `run16` | 12:05–12:40 | `semio-s-artifact-cad-cad --lib` **428 passed / 2 failed / 1 ignored**; plugin-cad 5✅ · aec-building 8✅ · aec-building-energy 2✅ · aec-building-structure 2✅ · spatial-shape 2✅ |
| `run17` | 13:10–13:29 | `semio-s-artifact-cad-cad --lib` **430 passed / 2 failed / 1 ignored**; other five green (5, 8, 2, 2, 2) |
| `run19` | launched 16:48 | the verification run for §11; see the addendum at the end of this section |

`world_scene_render_cost_probe` (`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:2344-2365`) failed in run16 with
`assertion failed: warm < cold` printing `cold 1.025042ms warm 1.415666ms`, at load 75 with 26 peer
cargos on 10 cores, and **passed again in run17**. It is a bare wall-clock probe (one cold render vs the
mean of ten warm ones) — a load flake under fleet rule 5. Re-measured, nothing loosened.

### 10. The three window config packs are NOT a cross-window leak (the open item from brief v5)
Every CAD window command — `SetCamera`, `SetProjection`, `SetProjectionParam`, `ToggleSun`,
`SetSunAzimuth`, `SetDislocateOption` — ends in `window_config::addressed_from_context(ctx, config)`
(`🎮️commands/🎥️camera/🦀️.rs:32,71,110`, `🌞️sun/🦀️.rs:26,48,69,90`, `🧰️utility/🦀️.rs:34`).
`addressed_from_context` (`🪟️windows/🎚️config/🦀️.rs:166` → `addressed`, `:153`) resolves the target from
`ctx.view_state.window_id` ALONE; the payload's `pane` is deliberately dead (`let _surface = …`,
`🎥️camera/🦀️.rs:29`). A dispatch made in a shape window structurally cannot write the building window's
config, whatever `pane` the payload carries.
The building partition exists for an unrelated reason: `VcsArtifactApp::window_measures` and
`window_engagements` LOOP over `view_state.window_instances` and call `window_config_store.capture(…)`
for every one (`🧰️framework/…/🔌️plugin/🦀️.rs:31708` and `:31674`), and `capture` lazily creates a
partition (`🪟️window/🎚️config/🦀️.rs:399-409`). `assert_exact_state` calls `window_measures`, so all three
declared WORLD windows materialise; `cad-panel` is kind `cad-document-panel`, has no registered owner,
`capture` answers `Ok(None)`, and it correctly gets no pack.
**So the predecessor's restatement to the exact owner SET of three is right**, and the old
`packs.len() == 1` confused "only one window was written" with "only one window exists". Isolation is
still proven twice over: each `dispatch()` asserts exactly ONE `WindowConfig` result lane, and
`assert_exact_state` requires the same-kind sibling `right` to still hold its defaults for camera,
projection, sun and utility.

### 11. The real defect: a CAD window config pack could not be reloaded at all
**What the law reported.** After the restatement the law got far enough to die with
`artifact store reached Drop without its exact terminal-empty shallow-shell witness`
(`🧰️framework/…/🏪️store/🦀️.rs:18927`). `RUST_BACKTRACE=1` (run16) named it exactly:
```
2: <ArtifactStore<CadSnapshot, CadMutation> as Drop>::drop
4: drop_glue::<VcsArtifactApp<EditorApp<CadPlayApp>, SemioMembers>>
5: drop_glue::<Box<VcsArtifactApp<…>>>
6: …window-ownership…{closure#0}{closure#0}{closure#0}
```
— the DOCUMENT store of the law's SECOND app (`reopened`), dropped at the end of the inner async block.
That panic was always latent: before the restatement the law returned `Err` at the packs assertion, so
`outcome.expect(...)` panicked first and every later store `Drop` ran under `std::thread::panicking()`,
which the witness explicitly suppresses.

**Bisect.** No other CAD test renders a mounted app and then closes it, and none reloads a window config
pack. Two new laws in `…/🪟️windows/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs` separate the two:
- `cad_rendered_world_window_app_reaches_its_exact_terminal_close_witness` — render → close → drop:
  **green in run17**, so the close ladder is fine and rendering is not the cause;
- `cad_reloaded_window_config_app_reaches_its_exact_terminal_close_witness` — **fails at the LOAD**:
  `Fault { origin: Framework, code: FaultCode("window-config.typed-state"),
  message: "retained exact window config Pack load was rejected" }`.
With a scaffolding guard added to the ownership law (its `for pack in packs { …? }` used to `?` out of
the async block leaving a LIVE app that had never run the close ladder — the Drop witness then panicked
over the real refusal), the ownership law now prints that same fault instead of the Drop panic.
**The Drop witness was a mask. The defect is that CAD cannot reopen its persisted per-window state.**

**Root cause, read from source.** `CadCamera::position` and `::target` are `#[dsl(coord)] [f64; 3]`
(`📐️cad/🗿️artifacts/📐️cad/🦀️.rs:365-370`) → `Shape::Coord(3)`, whose value is a `FieldValue::Tuple`
(`🗣️dsl/🧬️schema/🦀️.rs:88-92`). A homogeneous float sequence is written in the pack's packed wire form
`TAG_PACKED_F64` (`🎒️pack/🌱️value/🦀️.rs:466`, `encode_seq`), **which carries no tuple-vs-list marker**.
The whole-pack decoder knows this and spells the whole set — `is_tuple_shape` =
`Tuple | Coord | Dir | Dim | Range` (`🎒️pack/🌱️value/🦀️.rs:100-102`) — and has a regression test for
precisely this hazard (`🎒️pack/🌱️value/🧪️tests/🔬️unit/🦀️.rs:287-299`: a `List` "fails `[T; N]`'s
`DslField::from_value` downstream"). The RETAINED window-config decoder duplicated that predicate and
knew only `Shape::Tuple`, so `position` was rebuilt as a `FieldValue::List`, `CadWorldWindowConfig`'s
`DslField::from_value` refused it, and the load answered `TypedState`.
**Every plugin whose window config state carries a coord/dir/dim/range field is affected**; CAD is the
one in the play fleet that has one.

**Fix.** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs`
(framework-general, not on the peer no-touch list; `git status` for `🔌️plugin/🪟️window/` was clean when
I took it):
1. new `shape_is_tuple_valued()` mirroring the whole-pack decoder's `is_tuple_shape`, with the
   provenance and the failure it fixes in its doc comment;
2. `end_container` uses it for the tuple-vs-list decision (was `matches!(…, Shape::Tuple(_, _))`);
3. `child_element` gives a coord/dir/dim/range's elements `Shape::Float` instead of no shape at all.
Test-side, the reload law also asserts the reopened packs are **byte-exact** with the ones they were
printed from, so the coordinate round-trip is pinned by a law and not only by the close witness.

### 12. Describe — SUCCEEDED
The coordinator's serial chain logged `describe cad ok` at 14:32–14:33, `describe phase done; failed:
none`, and `activate-dev rc=0` at 15:47:19. cad's `🛂️.descriptor.semio` and `🔣️.json` are updated on
disk (`git diff HEAD --stat` shows both). **cad never had a cad-side describe blocker.** The three
earlier failures were all foreign:
- 09-21 17:24 chain: `✔ nx run @semio-tech/cad-plugin:describe` — it already worked then;
- 09-21 21:30 chain: `error[E0063]: missing field accessibility_label in initializer of UiInputNode`
  at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:2639` (a peer's in-flight wgpu edit);
- 09-22 01:30 chain: `cargo build -p semio-framework-plugin-describe killed by signal SIGTERM`;
- 09-22 11:06 chain: a peer's `TaskSlot.reserved` edit in `🔌️plugin/⚛️reactor/🧵️executor/🦀️.rs`
  (`E0027`, 4× `E0609`, `E0560` → `could not compile semio-framework-plugin`) killed the queue at
  `describe animate FAILED` 11:19:44 and `describe writer FAILED` 11:25:35, before cad's turn.
I touched `🗑️generated/describe.request/cad` at 12:25; the 14:32 describe is the answer.

### 13. Live pane — GREEN on the new activation
`:6033` was relaunched 15:57:03 onto the 15:47 activation (HTTP 200). Probe D, 16:10, read-only
playwright with `--use-angle=metal`, one page (evidence `probe-0922d/` was deleted by the sweep minutes
later; the figures are as read at 16:10):
- `data-shell-ready="cad"`, **no** `data-shell-error`;
- **4 `<canvas>` elements**, each 788×437 (CSS 789×438) — one per pane;
- 115 SVGs and 47 text rows: `Example / Demo`, `Editor`, `Viewer`, and `Shape` / `Building` / `Energy` /
  `Structure Classic`, each with `Projection` / `Window Options` / `Actions` / `Search` / `Utilities`;
- **0 console errors, 0 page errors, 0 refusals, 0 admission rejections** (grep `refus|reject|error`
  over the console log: 0 hits);
- the 466 936-byte screenshot was **byte-identical (md5 `e012d9af79cea45fcfe6d239454a9f13`) to probe C**
  of 03:32 — the same correct `Demo` geometry, unchanged across an activation.
Caveat for the record: that serve's `[stale] source-newer` banner still listed ~33 plugins including
`cad` (`staged 2026-09-22T12:2x < 📐️cad/🔣️.json 12:33`), so the demonstrator lane the pane runs from was
staged at ~12:2x rather than by the 15:42 `activate-dev`. The pane is correct on what is served; a
descriptor-fresh restage of `demonstrator` is a coordinator matter, not a cad defect.

### 14. Verification — `run20`, 2026-09-22 21:38: ALL SIX CAD CRATES GREEN
Log `.🧬semio/🦑️repo/⚡️cache/play-fleet/cad-content/run20.txt`, through
`📜️native-test-mutex.sh cad-content` with `CARGO_TARGET_DIR=$G/target`, `CARGO_INCREMENTAL=0`,
`CARGO_BUILD_JOBS=2`, `RUST_MIN_STACK=33554432`, `RUST_BACKTRACE=1`, `--lib`, `--no-fail-fast` BEFORE
the `--`, `-- --test-threads=4`. 111 compiler warnings in the log, so the tree really was type-checked.

| crate | result |
|---|---|
| `semio-s-artifact-cad-cad --lib` | **432 passed / 0 failed / 1 ignored** (34.54 s) |
| `semio-s-plugin-cad` | 5 passed / 0 failed |
| `semio-s-plugin-cad-aec-building` | 8 passed / 0 failed |
| `semio-s-plugin-cad-aec-building-energy` | 2 passed / 0 failed |
| `semio-s-plugin-cad-aec-building-structure` | 2 passed / 0 failed |
| `semio-s-plugin-cad-spatial-shape` | 2 passed / 0 failed |

**0 red across all six crates.** The one `ignored` is the pre-existing documented skip
`default_example_dsl_round_trips` ("fixture predates the model/drawing composition rewrite; regenerate
via print_dsl before re-enabling") — not introduced or touched this session.
Every law named in §11 reports `... ok`, including the two new bisecting close laws, the ownership law
itself, `two_instances_converge_disjoint_edits_via_backbone`, `forest_transformation_uses_live_shape_pane`
and `world_scene_render_cost_probe` (confirming run16's `warm < cold` red was a load flake, rule 5).

`run19` (17:53) produced no result and is not a cad defect: it died on a peer's mid-edit
`🧰️framework/…/🔌️plugin/🦀️.rs` — `error[E0425] cannot find value TYPED_OPERATION_STALL_PROGRESS_MIX`
(:27892) and `error[E0599] no method named note_publication_checkpoint` (:28626), i.e.
`could not compile semio-framework-plugin`. Both symbols existed again by 21:16; run20 is the rerun.

### 15. Live pane re-probed with durable evidence
`probe-cad.mjs` was deleted with the rest of `🗑️generated`, so it was rewritten into
`⚡️cache/play-fleet/cad-content/` and run once (21:41) against the 15:47 activation, `:6033` HTTP 200.
Evidence: `⚡️cache/play-fleet/cad-content/probe-2200/{cad.png, cad-console.txt, cad-report.json}`.
`data-shell-ready="cad"`, no `data-shell-error`; **4 canvases** 788×437 (CSS 789×438); 115 SVGs;
47 text rows; **0 console errors, 0 page errors, 0 refusals, 0 admission rejections**; screenshot md5
`e012d9af79cea45fcfe6d239454a9f13` — byte-identical to probe C (03:32) and probe D (16:10), i.e. the
same correct `Demo` geometry, stable across the activation. This matches the coordinator's strict
acceptance run (70/70 on the 15:47 activation, cad passing).

### 16. What remains
1. **The retained-decoder fix has not reached the browser yet.**
   `🔌️plugin/🪟️window/🎚️config/📥️retained/🦀️.rs` is framework code compiled into every plugin's wasm
   component, so a re-describe/activate is needed before a served guest can reopen a window config pack.
   I touched `🗑️generated/activate.request/demonstrator` at 21:41 (that lane also clears the outstanding
   `[stale] cad: source-newer` banner entry). I did not run any wasm32/describe/activate target myself.
2. **The same gap is worth a fleet-wide check.** The retained decoder refused ANY window config state
   with a `#[dsl(coord)]` / `dir` / `dim` / `range` field, so every such plugin silently lost its
   per-window persistence on reopen. A repo-wide grep for `dsl(coord|dir|dim|range)` inside a
   `WindowConfigOwner::State` would say who else was affected; CAD is the only one in the play fleet
   whose own law caught it, because the other window-ownership laws' states are plain scalars.
3. `🗑️generated` was wiped at ~16:0x and again risks being swept; the durable home for this topic is
   `.🧬semio/🦑️repo/⚡️cache/play-fleet/cad-content` (`STATUS.md`, `run18/19/20.txt`, `probe-cad.mjs`,
   `probe-2200/`, `target/`).
