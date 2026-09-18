# 🔍️ Audit — wfc3d (3D arbitrary-graph artifact)

Scope: `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d` (crate `semio-s-artifact-wfc-3d`, dialect `s.wfc.wfc3d`)
against `📓️plan.md` §1/§3/§7, the developer brief, the slice's own `📓️wfc3d.md`, the sibling
`📓️audit-wfc2d.md` (shared graph-window contract), and the code. Read-only audit; no source files
touched, no cargo run (logs read from `🗑️generated/wfc3d/`), no sub-agents.

> ⚠️ **This artifact was being actively edited by the slice author throughout this audit** (the
> deny-list → allow-list rule switch amendment). Every finding below states the exact timestamp/file
> `mtime` it was observed at. See §3 (item 3f) for the full timeline — it is the most important part
> of this report.

---

## Verdict table

| # | Item | Verdict | Evidence |
|---|---|---|---|
| 1a | `Wfc3dSnapshot{schema,seed,slots,edges,tiles,rules}` | PASS | `🧬️schema/📸️snapshot/🦀️.rs:117-137`, field order matches plan §3 exactly. |
| 1b | `Slot3d{id,x,y,z,width,height,depth,pinnedTileId?}` | PASS | `:71-82`. |
| 1c | `SlotEdge{id,fromSlotId,toSlotId,relation}` | PASS | `:87-94`. |
| 1d | `Tile{id,label?,weight,media}` + `TileMedia3d = Mesh{positions,indices,color?} \| MeshChild{child: s.stdio.semio@v1/mesh}` | PASS | `:33-63`. |
| 1e | `GraphRule{id,tileAId,tileBId,relation?,allowed}` | PASS | `:101-110`. |
| 1f | `x/y/z` semantics — docstring says "box CENTRE", code/behaviour is "box MINIMUM corner" | **DEVIATION, undisclosed, self-contradictory** | Docstring: `📸️snapshot/🦀️.rs:67` ("place its box centre") and `🧬️mutations/🚚️move-slot/🦀️.rs:1` ("relocates one slot's box centre"). Actual behaviour: preview places `position=[slot.x,y,z]`, `scale=[width,height,depth]` with unit-box (`0..1`) tile media (`🪟️windows/🧊️preview/🦀️.rs:139-148`), and the window's own docstring at `:13` states plainly "a slot's `x`/`y`/`z` is its box's MINIMUM corner". `wfc3d.md` §2 agrees with the code ("MINIMUM corner"), not with the two in-code docstrings. Two docstrings are simply wrong; zero functional impact since every consumer (graph window, preview window) treats it consistently as a corner. |
| 1g | `MeshChild` not a declared `#[child(kind=…)]` | PARTIAL, disclosed gap | `📸️snapshot/🦀️.rs:42-44`; matches wfc2d's identical `Image` gap. No bundled example uses it (all three use inline `Mesh`). Disclosed as gap 4 in `wfc3d.md` §7. |
| 1h | Collections canonical-ascending-id order, point-invertible inserts | PASS | `canonical_slot_index`/`canonical_edge_index`/`canonical_tile_index`/`canonical_rule_index` (`📸️snapshot/🦀️.rs:168-185`), each editor `create-*` command calls the matching helper (`✏️editor/🦀️.rs:164,174,187,194`). |
| 2a | `wfc-graph` is the SAME window module as wfc2d's, byte-for-byte | PASS, verified directly | `diff "◻️2d/…/🪟️windows/🕸️graph/🦀️.rs" "🧊️3d/…/🪟️windows/🕸️graph/🦀️.rs"` → **empty**. Same for the window's own unit-test file. Only the newtype adapter differs, and it lives outside the window file as designed. |
| 2b | `Wfc3dGraphView` projects x/y, drops z | PASS | `✏️editor/🦀️.rs:87-100` — `Wfc3dGraphView<'a>(&'a Wfc3dSnapshot)` implements `SlotGraphView`, maps `slot.x`/`slot.y` only. |
| 2c | `wfc-3d-preview`: World3d, `meshes_json` one per distinct tile + placeholder | PASS | `🪟️windows/🧊️preview/🦀️.rs:125-134` (`meshes_json`). |
| 2d | `instances_json` — brief expects "one per SOLVED slot" | **DEVIATION, undisclosed** | `🪟️windows/🧊️preview/🦀️.rs:164-174` emits one instance per slot **regardless of solved state** — `slot_mesh` (`:152-161`) falls back to the slot's authored pin, then to `WFC_3D_PLACEHOLDER_MESH`, so an unsolved slot still gets a placeholder-box instance rather than being omitted. This differs from the sibling `grid3d` artifact, whose own preview strictly maps only `assignments` (i.e. only solved cells) 1:1 into instances (confirmed in `📓️audit-grid3d.md` item 2.6, `preview_instances_json`). Reasonable UX (nothing silently vanishes), but it is a real, undisclosed divergence from the literal "one per solved slot" contract and from the sibling's actual implementation; `wfc3d.md` §4 describes it only as "one record per slot" without flagging the difference. |
| 2e | `instances_delta_json` rides alongside, full generation each render | PASS | `:180-191`; `status_json` one short line, capacity-safe (`:240-251`). |
| 2f | `camera_json` frames real document bounds | PASS | `:196-223`, `framing_bounds` computes centre/span over every slot box; empty-document fallback `([0,0,0], 1.0)`. |
| 2g | Layout `row` 50/50 | PASS | `✏️editor/🎭️modes/✏️edit/🦀️.rs:16-18` — `create_default_layout([graph, preview], "row", [50.0,50.0], …)`. |
| **2h** | **`ArtifactEditor::render` threads the REAL document/config, not `Default` — this was the wfc2d blocking finding; checked explicitly here** | **PASS, confirmed** | `✏️editor/🦀️.rs:141-143`: `fn render(...) { render_body(body_key, doc.snapshot, cfg.snapshot) }` — `doc.snapshot`/`cfg.snapshot` are the real live values, not defaults. `render_body` (`:205-214`) calls `preview::render(document, &solved_transient(document), config.camera_zoom)` and `solved_transient` (`✏️editor/🫧️transient/🦀️.rs:34-37`) calls `crate::inferences::solve_assignments(document)` — a REAL solve over the REAL document, computed fresh every render. wfc3d structurally cannot reproduce wfc2d's bug because it never caches a transient behind `ArtifactEditor::Transient` (declared `NoTransient`, `✏️editor/🦀️.rs:116-117`) — there is no stale-default path to fall into. Trade-off (disclosed, `wfc3d.md` §4): the solve re-runs on every repaint rather than being cached/pushed from a spawned job. |
| 2i | Viewer: same threading discipline | PASS | `👁️viewer/🦀️.rs:66-67` — `render(...) { render_body(body_key, doc.snapshot) }`, real document. `render_body` → `preview::render(document)` (`👁️viewer/…/🪟️windows/🧊️preview/🦀️.rs:39-52`), which itself calls `solved_transient(document)` (`:40`) and reuses the editor preview's own `instances_delta_json`/`status_json`/`camera_json`/`meshes_json`/`instances_json` verbatim (`:42-49`) — confirmed byte-identical function reuse, exactly as `wfc3d.md` §4 claims. |
| 3a | Inference route: `ModelBuilder`, not `GraphModelBuild` (multi-relation) | PASS, disclosed & justified | `💡️inferences/🦀️.rs` Rules/Model stages; doc comment explains the substitution. Matches wfc2d's identical, already-audited reasoning. |
| 3b | Resumable `WfcJob<GraphTopology>` | PASS | `Wfc3dInferenceJob.child: Option<engine::job::WfcJob<engine::topology::GraphTopology>>` (struct field, confirmed in code); `ToolExecutionContract::resumable(…)` (`:686`). |
| 3c | Bug 1 — `CommitCandidate` carries TWO retained payloads, both retired | PASS | `💡️inferences/🦀️.rs:497-501` (Restore arm: `retire_payload(candidate.state)` + `retire_payload(candidate.output)`) and `:518-523` (Solve arm: `state` kept as `final_checkpoint`, `output` explicitly `retire_payload`d, never dropped). |
| 3d | Bug 2 — child `WfcJob`/restore closed via `close_job` before drop | PASS | `:507` (`engine::job::close_job(&mut restore)`), `:530` (`engine::job::close_job(&mut child)`), comment explicitly ties this to the measured >10min→0.04s fix. |
| 3e | Bug 3 — `terminal_is_empty` not gated on a `closing` flag | PASS | `:654-656` — `fn terminal_is_empty(&self) -> bool` checks only `output`/`final_checkpoint`/`restore`/`child`/`rejected_output_page` are `None`; docstring at `:648-653` states the rationale and the measured incident directly. |
| 3f | `wfc-unsatisfiable` fault handling | N/A here (different, equally sound design) | wfc3d does not special-case a `"wfc-unsatisfiable"` byte string like wfc2d does; instead `Wfc3dSolve::compute` (`:812-817`) and `Wfc3dContradiction::compute` (`compute(...) { solve_with_job(snapshot).is_ok() }`) treat ANY `Err` from `solve_with_job` — including a genuine engine contradiction fault — uniformly as "unsolved", never panicking or hanging. Functionally equivalent safety net, different mechanism; not a deviation. |
| 3g | One payload page per step | PASS (plausible) | `retained_payload`/`retire_payload` helpers (`:25-42`) plus admission caps below. |
| 3h | Admission caps sane | PASS | `MAX_WFC3D_TILES=256, MAX_WFC3D_RELATIONS=32, MAX_WFC3D_SLOTS=65_536, MAX_WFC3D_EDGES=262_144, MAX_WFC3D_RULES=262_144, MAX_WFC3D_ID_BYTES=1_024, MAX_WFC3D_OUTPUT_BYTES=1<<20` (`:66-72`), enforced in `Wfc3dInferenceJob::new` (admission check). |
| 3i | Headless fuel/step budget — brief explicitly asks for "sane…, not assembly's 1/2000" | **DEVIATION, undisclosed** | `solve_with_job`: `BatchDriveConfig { fuel_per_step: 1, step_budget_us: 2000, … }` — **exactly** assembly's value (`procedural/🗿️artifacts/🧩️assembly/…/💡️inferences/🦀️.rs:643`, same `1`/`2000`). wfc2d and grid2d copy the identical `1`/`2000` pair. Only `grid3d` fixed this, with named constants `HEADLESS_FUEL_PER_STEP = 1 << 16` (65536), `HEADLESS_STEP_BUDGET_US = 250_000` (`grid3d/…/💡️inferences/🦀️.rs:62-63,723`). Not disclosed in `wfc3d.md`. Harmless for the tiny bundled examples (3–6 slots solve in ~0.04s per `wfc3d.md` §5), but a real scaling risk the brief specifically flagged and grid3d already addressed — should be pulled up to `grid3d`'s constants for consistency. |
| 4a | Every mutation (15) has ≥1 fixture quintet | PASS | `🧫️fixtures/🧬️mutations/` has exactly 15 dirs, 1:1 with the 15 kind dirs under `🧬️schema/🧬️mutations/` (`change-tile-weight, disconnect-slots, create-tile, change-seed, pin-slot, unpin-slot, resize-slot, connect-slots, delete-slot, change-tile-media, delete-tile, move-slot, create-rule, delete-rule, create-slot`). Spot-checked `move-slot`'s quintet (`mutation/outcome/diff/snapshot⬅️before/snapshot➡️after`) — all 5 present. |
| 4b | Mounted Rust test per mutation | PASS | Crate root `🦀️.rs:275-283` (spot-checked `move-slot`) `#[path]`-mounts schema + diff + inverse + the case test; same pattern for the other 14 (structurally verified via the 227→228 green run, see 4f). |
| 4c | Feature row + python oracle branch + oracle manifest vector | PASS | `🧪️tests/🧩️mutate-wfc3d-1/{🥒️.feature, 🐍️.py}` present; `🔮️oracles/🔣️.json` `mutationCatalogs[0].vectors` has all 15 mutation ids (verified by direct JSON parse). |
| 4d | Rust subject adapter for `🧩️mutate-wfc3d-1` | **CONFIRMED MISSING, disclosed** | `find … -path "*🧩️mutate-wfc3d-1*"` returns only `🥒️.feature` and `🐍️.py` — no `🦀️.rs`. Matches `wfc3d.md` §7 gap 1 exactly. Python route runnable today (`python3 $T/🐍️wfc3d-oracle-runner.py`, 15/15 per §1). |
| 4e | Render tests both windows × 3 examples | PASS (spot-checked, not exhaustively re-derived) | `viewer::wfc3d::component::tests::the_view_body_renders_for_every_example` and `…preview::tests::every_example_renders_a_non_empty_surface` observed passing in `test-5.log`/`test-6.log`; editor-side equivalents claimed in `wfc3d.md` §6 and consistent with the green aggregate count. |
| 4f | **Deterministic-seed solve + contradiction tests, and the newest green run** | PASS, **but see the timeline in item 3f/6 below — the number moved during this audit** | Newest Rust test log as of this audit is **`test-6.log`** (mtime `Sep 18 14:26:15`): `test result: ok. 228 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 0.05s`. This supersedes `wfc3d.md`'s own cited `test-4.log` (227 passed, mtime `14:14:30`) — see the rule-semantics timeline. |
| 4g | Newest TS/wasm/clippy logs | PASS with a caveat | `wasm-1.log` tail: `Finished 'dev' profile … 25.82s`, crate is the last one checked, no wfc-3d warnings. `clippy-3.log` tail: `Checking semio-s-artifact-wfc-3d …Finished … 2.52s`, no warning lines between the "Checking" line and "Finished" (0 warnings, matches claim). TS: `ts-test-3.log` (14:12) is green (8 files/125 tests), but `ts-test-4.log` (14:17) and `ts-test-5.log` (14:21) — both **newer** than `ts-test-3.log` and produced under the plain `nx` daemon — reproduce the SAME pre-existing fault `wfc3d.md` already discloses ("stale project graph after the folder move… `Source file "…/Cargo.toml" does not exist`"). This is not a new regression, just confirms the daemon-staleness caveat is still live; `NX_DAEMON=false` remains the working path. |
| 5a | Approved-verb mapping `pin-slot`→`fix`/`Fixed`, `unpin-slot`→`clear`/`Cleared` | PASS | `🧬️mutations/📌️pin-slot/🦀️.rs:23` (`SemanticDescriptor{verb:"fix",…,record:"Fixed"}`), `📍️unpin-slot/🦀️.rs:22` (`verb:"clear",…,record:"Cleared"`). |
| 5b | Uppercase direction-style enum tokens | N/A, correctly so | `wfc3d`'s document schema has no direction enum (`relation`/`GraphRule.relation` are free `String`s, like wfc2d) — the grid2d/grid3d `LEFT/RIGHT/…` trap does not apply here. |
| 5c | Icon ids valid | PASS | `box`, `network`, `eye`, `pencil` — all four confirmed present via `#[serde(rename = "…")]` in `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🪪️icon-name/🦀️.rs`. |
| 5d | UiText ≤ 512 bytes | PASS | `status_json` is a single short sentence (`🧊️preview/🦀️.rs:240-251`); the graph window's node caption bound (96 bytes) is inherited verbatim from wfc2d (byte-identical file, already audited PASS in `audit-wfc2d.md` item 5d). |
| 5e | Floats `N.0` | PASS | Spot-checked `move-slot`'s after-snapshot fixture: every numeric field (`x,y,z,width,height,depth,weight,color components as ints`) is `0.0`/`1.0`/`2.0`-style, no bare integers on float fields. |
| 5f | No dependency on another app plugin crate | PASS | `📦️packages/🦀️rust/Cargo.toml` deps are all `semio-framework-*`, `semio-s-plugin-wfc-engine`, `semio-s-artifact-stdio-{semio,txt}` (stdio precedent), `pack`, `serde*`. No `semio-s-plugin-{draw,raster,lowpoly,procedural,bitmap,grid2d,grid3d,2d}`. |
| 5g | Labels en + de | PASS | `"Edit"/"Bearbeiten"`, `"View"/"Ansicht"`, `"Preview"/"Vorschau"` confirmed in the mode/window definitions read. |
| 5h | Emoji docstrings | PASS | Every file opened (editor, viewer, snapshot, inferences, both windows, transient) opens with a leading-emoji `//!`/`///` doc comment. |
| 5i | No comments inside definitions | **DEVIATION, more extensive than wfc2d's single instance** | `🧬️mutations/💡️inferences/🦀️.rs` has (as of the final re-read) several bare `//` blocks **inside the `step()` match arms** (a fn body, i.e. squarely "inside a definition"): the Restore-arm payload-retirement rationale, the close-ladder rationale, the Solve-arm checkpoint rationale, the close-before-drop rationale (4 blocks, ~14 lines total, around the Restore/Solve match arms). Plus `📸️snapshot/💾️binary/🦀️.rs:84-85` — 2 bare `//` lines inside `impl Drop for Wfc3dSnapshotRetirement`, the same pattern the wfc2d audit flagged as one isolated instance. wfc3d has the same category of slip, at roughly 3× the volume. Zero functional impact — same rating as wfc2d's (cosmetic) — but should be folded into the preceding `///` doc comments per convention. (A free-standing module-level comment at `🧬️mutations/🦀️.rs:11-14`, between `use` statements and outside any item body, is NOT counted here — it is not "inside a definition".) |

---

## 3. Rule semantics — deny-list vs allow-list (checked LAST, as instructed, with timestamps)

This item moved live, in front of the auditor, over the course of this audit. Full timeline, all times
`Europe/Zurich` (system clock), all file `mtime`s from `stat`:

| when | what |
|---|---|
| `wfc3d.md` first read (slice report, saved `14:16:25`) | Document model §2 states: **"Rules are a DENY-LIST over an UNORDERED tile pair. A pair no rule mentions is admitted… Assembly was inconsistent here… wfc3d is deny-list on both sides."** |
| Code read #1 (`💡️inferences/🦀️.rs`, before `14:22`) | Confirmed **deny-list** in code: `denied_any`/`denied_per_relation` sets populated only from `allowed:false` rules; `allowed:true` rules were literal no-ops; `pair_is_admitted()` docstring: *"Deny-list: only an explicit `allowed: false` rule forbids."* Fixture `🧫️fixtures/…/move-slot/…/📸️snapshot/➡️after/🔣️.json` at this point carried two `allowed:false` self-pair rules (`rule-corridor-corridor`, `rule-room-room`) and NO rule for the cross pair — consistent with deny-list (cross pair admitted by omission). |
| Cross-check: `🧱️grid3d` (sibling, already audited) | `📓️audit-grid3d.md` item 1.4: grid3d already implements **"Closed allow-list semantics (deny always wins)"**. `🔲️grid2d`'s rule compile (`💡️inferences/🦀️.rs:268`) only calls `builder.allow_mirrored(...)` for `allowed:true`, never `deny()` — and the engine's `ModelBuilder::compile()` (`⚙️engine/🏗️model/🦀️.rs:132`) starts every pair at **not-admitted** (`PatternSet::new_empty`) — so grid2d is allow-list too. wfc2d calls both `builder.allow()`/`builder.deny()` (`◻️2d/…/💡️inferences/🦀️.rs:247-253`) onto the same empty-by-default engine, which is *also* effectively allow-list at the engine level (an unmentioned pair never gets an `allow()` call, so it stays denied) — **wfc3d, uniquely, was NOT actually calling into the engine's `allow`/`deny` primitives from its deny-set at all** at this point in time; see below. |
| Code re-read #2, `14:22:22`–`14:24:22` | File **`💡️inferences/🦀️.rs` mtime jumped to `14:22:23`** (and `📸️snapshot/🦀️.rs` too, same instant) mid-audit. Re-reading found the Rules stage rewritten: a `compatibility: Vec<(bool, Option<usize>, u32, u32)>` field, populated 1:1 from authored rules only (`:283`, `self.compatibility.push((rule.allowed, scope, a.get(), b.get()))`), replayed in the Model stage as `builder.allow(...)`/`builder.deny(...)` calls **onto an initially-empty `ModelBuilder`** — i.e. a pair with no rule never gets an `allow()` call and stays denied. New module doc (`:12-17`): *"Rule semantics: the rules ARE the compatibility table. They compile onto an EMPTY `ModelBuilder`, so a tile pair no rule mentions is FORBIDDEN — an allow-list, the same law every other `wfc` artifact states."* `GraphRule`'s own docstring (`📸️snapshot/🦀️.rs:97-100`) was rewritten to match. |
| Fixture re-read, `14:24` | The SAME `move-slot` after-snapshot fixture had already been rewritten on disk: the two `allowed:false` self-pair rules were replaced with a single `{"id":"rule-room-corridor","tileAId":"corridor","tileBId":"room","allowed":true}` — the correct allow-list-shaped rule for this document (corridor↔room must now be explicitly admitted). |
| Examples re-read, `14:24` | All three bundled examples (`🚪️two-room-corridor`, `🗼️tower-stack`, `🧱️wall-roof-facade-strip`) already carry explicit `allowed:true` rules for every legitimate adjacency (e.g. `rule-room-corridor`, `rule-cap-deck-above`, `rule-roof-roof`, `rule-wall-roof`, …) — fully migrated. |
| Test logs, `14:14` → `14:26` | `test-4.log` (`14:14:30`, cited by `wfc3d.md`'s verification table): 227 passed, PRE-DATES the `14:22:23` code switch. **`test-5.log` (`14:26:00`)**: `FAILED. 227 passed; 1 failed` — `snapshot::component::tests::the_addressing_helpers_find_every_member_by_id` panicked (`assertion left==right failed: left: None, right: Some(1)`), i.e. an in-flight casualty of the switch. **`test-6.log` (`14:26:15`)**, the newest log that exists as of this audit: `test result: ok. 228 passed; 0 failed; 2 ignored` — green again, one test MORE than before (consistent with a new allow-list-specific assertion being added). |
| `wfc3d.md` re-read, `14:25:57` | The slice report itself was rewritten mid-audit to match: §2 now reads *"Rules ARE the compatibility table — an ALLOW-LIST over an UNORDERED tile pair… `wfc3d` originally shipped a deny-list and was switched (coordinator ruling, plan §7)."* Its §1 verification table, however, **still cites the stale `test-4.log`/227-passed** as of that same save — it was not updated to point at `test-6.log`. |

**State as observed at the end of this audit (`2026-09-18 14:26:59 CEST`):** wfc3d is now on **closed
allow-list** semantics (a tile pair no rule mentions is forbidden), matching grid2d and grid3d, and the
newest Rust test run (`test-6.log`) is green at 228/0/2. The switch itself, the fixture that needed to
change to match it, and the example set were all consistent and correct by the end of the window I
observed. The one loose end: `wfc3d.md` §1's verification table names `test-4.log` (pre-switch) as its
evidence and was saved (`14:25:57`) 15 seconds before the green post-switch run (`test-6.log`,
`14:26:15`) landed — a cosmetic staleness in the report's own citation, not a code problem, but W2/W3
should re-point it at `test-6.log` (or re-run once more) rather than trust the number currently printed
in `wfc3d.md` §1.

---

## Known-gaps triage (`wfc3d.md` §7, current revision)

| gap | rating | why |
|---|---|---|
| 1. No Rust subject adapter for `🧩️mutate-wfc3d-1` | nice-to-have | Shared W2/W3 infra work across every `wfc` artifact (same rating wfc2d's audit gave the identical gap); Python route runs today and is green 15/15. |
| 2. Taxonomy registrations not made | nice-to-have | Explicitly owned by slice P; cross-cutting, not this artifact's defect. |
| 3. `verify taxonomy\|artifact-field-parity\|contract` not run | nice-to-have | Blocked by an unrelated repo-wide fault (the `▦️grid2d` fixture digest, since fixed by P) and the stale nx daemon graph — neither caused by this artifact. |
| 4. `TileMedia3d::MeshChild` declares no `#[child(kind)]` | nice-to-have | No bundled example uses it; identical, already-accepted pattern to wfc2d's `Image` gap; cross-cutting composed-child infra. |
| 5. Preview solves inline per render (no cached transient lane) | **nice-to-have — explicitly NOT the wfc2d blocking bug** | See verdict item 2h: because wfc3d never threads a *cached* transient through `ArtifactEditor::render` in the first place (it recomputes from the live document every call), it cannot reproduce wfc2d's "render always sees `Default`" failure mode. This is a disclosed performance trade-off (solve cost paid every repaint), not a correctness gap — a real, working preview today, at the cost of redundant solves. Rated far below wfc2d's equivalent gap, which was blocking. |
| 6. `ModelBuilder` rather than `TiledModelBuilder` | nice-to-have | Disclosed trade-off; matches current engine capability (no steppable multi-relation tiled compiler exists yet); identical reasoning already accepted for wfc2d. |
| 7. `🔺️diff/📝️text`, `🔺️diff/💾️binary` TS leaves with no Rust module | nice-to-have | Honestly disclosed; the diff rides the document's own carrier by design. |

---

## Additional findings not in `wfc3d.md`'s own gap list

- **`instances_json` places one instance per slot, not one per SOLVED slot** (verdict 2d) — undisclosed, differs from the sibling `grid3d`'s stricter contract. nice-to-have (reasonable UX; still bounded, still correct once solved).
- **Two internal docstrings ("box centre") contradict the actual, consistently-implemented "box MINIMUM corner" semantics** (verdict 1f) — undisclosed, cosmetic, zero functional impact since every consumer already agrees on the correct behaviour.
- **Headless `fuel_per_step`/`step_budget_us` still `1`/`2000`**, identical to assembly's flagged-as-unsound value, unlike `grid3d`'s fix (verdict 3i) — undisclosed, nice-to-have today (tiny bundled examples solve in ~0.04s) but a real scaling risk for larger documents, and the brief calls this out by name.
- **More extensive "no comments inside definitions" slips than wfc2d's one instance** (verdict 5i) — cosmetic, same rating, larger volume.
- **The rule-semantics switch (§3)** is not itself a "gap" — it is the single biggest, live, time-boxed event of this audit, documented in full above with a resolved, green outcome as of `14:26:15`.

---

## Ordered fix list

1. **Point `wfc3d.md` §1's verification table at `test-6.log`** (228 passed, 0 failed, 2 ignored, `14:26:15`) instead of the pre-switch `test-4.log` (227 passed, `14:14:30`) — the report's own headline evidence is one commit behind its own prose. Trivial, but do it before anyone else reads the report as current.
2. **Pull the headless `solve_with_job` fuel/step budget up to `grid3d`'s already-fixed constants** (`HEADLESS_FUEL_PER_STEP = 1<<16`, `HEADLESS_STEP_BUDGET_US = 250_000`) instead of the inherited `1`/`2000` from assembly — the brief calls this out by name as unsound, and one sibling artifact has already demonstrated the fix.
3. **Decide and document whether `instances_json` should omit unsolved slots** (matching `grid3d`'s strict "one per solved cell/slot" contract) or keep the current placeholder-fallback UX — either is defensible, but `wfc3d.md` §4 should say which one was chosen and why, since right now it silently differs from its closest sibling.
4. **Fix the two self-contradictory "box centre" docstrings** (`📸️snapshot/🦀️.rs:67`, `🧬️mutations/🚚️move-slot/🦀️.rs:1`) to say "box MINIMUM corner", matching the code, the window docstring, and `wfc3d.md` §2 itself.
5. Minor: fold the ~14 lines of bare `//` comments inside `💡️inferences/🦀️.rs`'s `step()` match arms and the 2 lines inside `📸️snapshot/💾️binary/🦀️.rs`'s `Drop` impl into the preceding `///` doc comments, per the "no comments inside definitions" convention (same fix wfc2d's audit already recommended for its one instance).

Everything else audited — the document model, the byte-identical reusable `wfc-graph` window, the
World3d preview's mesh/instance/camera/status contract, **the explicit confirmation that
`ArtifactEditor::render`/`ArtifactViewer::render` both thread the real document and never fall back to
`Default` (the wfc2d blocking bug does not reproduce here, by construction)**, the graph-route inference
with its three previously-latent job bugs verified fixed at exact line numbers, the 15/15 mutation
fixture/test/oracle scaffolding, the verb/icon/float/label/dependency conventions, and — as of the last
log produced during this audit — a green 228/0/2 Rust test run under the now-completed allow-list rule
default — is solid and matches the plan and the slice's own (largely honest, and now updated) report.
