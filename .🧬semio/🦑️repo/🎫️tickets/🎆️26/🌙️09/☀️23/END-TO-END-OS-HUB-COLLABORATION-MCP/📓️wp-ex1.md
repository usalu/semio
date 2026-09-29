# WP-EX1 — Example Loaders Load For Real Or Refuse By Code

Session 15 slice EX1 (NEW, coordinator `main`). Scope: every plugin example loader that silently falls back to an empty /
genesis document (or a silent no-op) when its example payload does not decode or its id is unknown → the example loads for real,
anything else is a typed refusal (fault API of `📓️fault-localization-api.md`, lands AFTER T6 row 12), plus a language-agnostic
fixture + Rust law that every example loads non-empty and round-trips through its codec. Scripts `wp-ex1/`; expendable captures
`wp-ex1/generated/`; durable data `.🧬semio/🌐hub/s15-ex1-*`. Rules: `📓️session-15-preamble.md` (+ session-14 rules 1–28).

## Session 15

| # | Item | Status |
|---|---|---|
| 1 | Census of every example-loader fallback (all plugins, all languages) with the exact pattern list | **done**: `wp-ex1/ex1-census.py` (rules L1–L7 + P below); per-plugin table pre → post (set applied) below; no runtime loader outside Rust |
| 2 | Fix design (typed refusal by code, rebase-aware to row 12) + coordination with S20 / LB2 | **done**: ONE SDK resolver, framework codes `app.example.unknown` / `app.example.unreadable` {example} (S20 agreed); LB2 p18 folded in; order agreed via main: LB2 p20 → row 12 → EX1 (p21 after EX1) |
| 3 | Prepared set `wp-ex1/ex1-example-loaders.py` + queue row | **dry-run clean** on S20's post-row-12 overlay: 295 edited + 56 new + 12 copied + 18 deleted / 0 problems (22:1x); idempotent (a re-run on the applied tree = 0 edits), `--revert` byte-exact (scratch clone); live: 7 problems = row-12 anchors only; on LB2's p17 scratch the stdio + SDK parts anchor clean; `📓️t6-queue.md` row **T7h** |
| 4 | Law: language-agnostic fixture + Rust law + third-party oracle | **done (in the set)**: schema `semio.example-catalog.v1`, 34 catalogs (stdio 88 apps + 33 plugins / 60 editors), SDK laws `assert_editor_examples_load` + `assert_example_catalog_lists`, ajv oracle `🔌️plugin/🧪️tests/📚️example-catalog/🟦️.ts` **69/69 green** on the applied tree (red on a broken catalog) |
| 5 | Overlay proof (overlay lane) | p1 20:13 (TDD baseline), p2 20:37–21:15 (R green, C 48/90, no plugin law ran), p3 21:50–22:13 (R green, C 57/90 — every red fixed in the set since except jpg ×2 / gltf → LB2 and the flaky details-UI baseline; L: architect 2/2, block 4/4 green, 22 law files needed `crate::plugin::plugin()` → stopped for p4); **p4 QUEUED 22:14** (full set, R C L W); `verify faults` on the p3 overlay **0 violations** |
| 6 | Left | jpg (DSL) / gltf (pack) round-trip codec defects → LB2 (coordinator 22:0x); L6 missing-`exampleId` defaults (the declared-arguments law may already refuse them — to measure); full `--lib` suites of the 124 touched crates (step T, after p4); dry-run on LB2 p20's post-state once it exists (the ifc split touches `🏗️ifc/🦀️.rs`'s examples block, p20 the same file's codec rows) |

### Census — exact patterns (`wp-ex1/ex1-census.py` `PATTERNS`)

| Rule | Meaning | Regex family |
|---|---|---|
| L1 decode-fallback | an example decode whose Err arm yields a document | `DECODE(…).unwrap_or_default()`, `.unwrap_or_else(|_| empty…/default…)`, `.or_else(|_| parse_dsl…)` |
| L2 decode-noop | an example decode whose Err arm is a silent no-op | `DECODE(…).ok()`, `let Ok/Some(…) = …parse… else { return Ok(Emit::default()) }`, `Err(_) => Emit::default()` |
| L3 unknown-genesis | an unknown id loads the genesis / empty / default document | `} else { X::default() / empty_*() / default_*snapshot() }`, `_ => &EMPTY / empty_*() / X::default(),` |
| L4 unknown-noop | an unknown id is a silent no-op | `return Ok(Emit::default())`, `None => Emit::default()`, `_ => None,` |
| L5 untyped-refusal | refuses with free text | `Fault::from(format!/"…"/error)` |
| L6 missing-id | absent `exampleId` → "" / a default id | `exampleId … .unwrap_or_default() / .unwrap_or("")`, `example_id_argument(args, "")` |
| L7 own-codes | per-plugin example codes duplicating `app.example.*` | `FaultCode::new / app_fault / Fault::from("…example|template…")` |
| P law-guarded | an example decode that panics (loud, pinned by the law) | `DECODE(…).expect(` |

`DECODE` = `parse_dsl|from_json_str|decode_*_dsl|example_snapshot|example_document|example_text|fixture_dsl_for_preset|example_model`.
Totals on S20's post-row-12 overlay → the same tree with the set applied (`.🧬semio/🌐hub/s15-ex1-scratch`):
L1 92 → 2 · L2 29 → 0 · L3 69 → 1 · L4 65 → 25 · L5 0 → 0 · L6 107 → 91 · L7 159 → 132 · P 15 → 39. The residue is classified:
L1 2 = stdio txt/csv boot docs (pinned since 21:5x, not yet in p3); L3 1 + L4 3 = puzzle (row 6); the other L4 are non-example
no-ops (hover, identical edit, extent `None`); L7 132 = retained-route / capacity codes that only mention "example"
(`stdio-example-tool-mismatch` ×52, `puzzle*-set-active-example-*-malformed`, …), not duplicates of `app.example.*`; L6 = row 6.

| Plugin | L1 | L2 | L3 | L4 | L6 | L7 | P |
|---|---|---|---|---|---|---|---|
| ✒️writer | 2→0 | 0 | 1→0 | 0 | 0→1 | 0 | 0→2 |
| ➗️mathematical | 0 | 0 | 0 | 1→0 | 1 | 3→2 | 0 |
| 🀄️wfc | 0 | 1→0 | 0 | 12→7 | 3 | 5→1 | 0 |
| 🌀️procedural | 4→0 | 2→0 | 0 | 3→1 | 5→3 | 1 | 0→3 |
| 🌊️flow | 0 | 0 | 0 | 0 | 1 | 2→0 | 0 |
| 🌍️gis | 2→0 | 0 | 0 | 0 | 2 | 4→0 | 0→2 |
| 🌿️vcs | 1→0 | 0 | 0 | 0 | 0 | 0 | 0 |
| 🎞️animate | 0 | 0 | 0 | 0 | 1 | 0 | 0 |
| 🎥️shooting | 1→0 | 1→0 | 0 | 1→0 | 0 | 0 | 0→1 |
| 🎪️demonstrator | 0 | 0 | 0 | 1→0 | 1 | 1→0 | 0 |
| 🎬️sequence | 0 | 0 | 0 | 1→0 | 1 | 3→2 | 0 |
| 🏗️fem | 4→0 | 0 | 1→0 | 1→0 | 2 | 0 | 1→2 |
| 🏛️architect | 0 | 0 | 0 | 1 | 1 | 2 | 0 |
| 🏭️process | 3→0 | 0 | 0 | 0 | 0 | 0 | 0→3 |
| 💡️reasoning | 0 | 0 | 1→0 | 1 | 0 | 1→0 | 0 |
| 📋️forms | 0 | 0 | 0 | 0 | 0 | 2→0 | 3 |
| 📐️cad | 0 | 0 | 0 | 1 | 1 | 2→0 | 0 |
| 📕️norm | 0 | 13→0 | 0 | 15→0 | 0 | 1→0 | 0 |
| 📖️playbook | 0 | 0 | 1→0 | 0 | 0 | 1→0 | 0 |
| 📜️imperative | 0 | 0 | 1→0 | 0 | 0 | 1→0 | 0 |
| 📸️remodel | 2→0 | 3→0 | 0 | 2→0 | 0 | 0 | 0→2 |
| 🔋️energy | 0 | 0 | 0 | 0 | 0 | 1→0 | 0 |
| 🔱️trinity | 1→0 | 2→0 | 0 | 5→1 | 0 | 0 | 1→2 |
| 🕸️dag | 0 | 0 | 1→0 | 1 | 0 | 0 | 1 |
| 🖍️draw | 1→0 | 0 | 0 | 0 | 0 | 2→0 | 0→1 |
| 🖨️raster | 1→0 | 1→0 | 0 | 2→1 | 0 | 0 | 0→1 |
| 🗄️stdio | 64→2 | 0 | 62→0 | 8 | 77→62 | 104 | 0 |
| 🗒️note | 1→0 | 0 | 0 | 0 | 0 | 1→0 | 0→1 |
| 🧩️puzzle | 2→0 | 0 | 1 | 3 | 7 | 20 | 9→11 |
| 🧱️block | 3→0 | 6→0 | 0 | 6→0 | 3 | 0 | 0→3 |
| 🪵️sourcing | 0 | 0 | 0 | 0 | 1 | 2→0 | 0→1 |

### The set (`wp-ex1/ex1-example-loaders.py`)

- `sdk`: `example_snapshot(examples, id)` (published body decoded by the snapshot's own codec; `app.example.unknown` /
  `app.example.unreadable` {example}), `editor_example_snapshot::<E>(id)` (E's picker list: `E::examples()`, else the
  subset declaration registered at assembly; `""` = `E::initial_snapshot()`), `subset_example_snapshot::<E>(list, id)` (the
  subset's own list, resolvable before assembly — unit tests dispatch examples without assembling), catalogue dispatch on the
  same decoder, catalog text {example} en/de, schema `🧬️schema/📚️example-catalog/🔣️.json` (draft-07), laws in
  `artifact_app_laws`, dummy resolver law, ajv oracle.
- `stdio` (LB2 p18 folded): 62 loaders deleted, every `SetActiveExample` arm loads through the resolver, 10 unit-test files,
  `editor_catalog` law on 88 editors + catalog-size law.
- `laws`: 33 plugin catalogs + one law per editor (60) in the plugins' surface tests (assembled plugin first).
- `plugins-a` / `plugins-b` / `plugins-c`: every plugin handler (mathematical, sequence, flow, vcs, demonstrator, architect,
  reasoning, playbook, imperative, dag, note, draw, gis ×2, fem ×2, generation ×2, remodel, writer, raster, block ×3, wfc ×5,
  norm ×13, process3d, energy, shooting, forms, sourcing, animate, cad, trinity ×2), 27 duplicate codes + `.fault` removed,
  22 boot fallbacks pinned by `expect`, 20 subsets expose their ONE list (`pub fn examples()`); forms offering what they do
  not publish fixed by publishing: writer `dag-jack`, process3d `drilled-plate` (inline fixture → asset), forms `contact` +
  `onboarding`, cad `hexagonal-cut-concrete-forest-left` (deferred from its model JSON), trinity `branch-chain` (the nakagin
  graph with its whole-graph query as an asset, instead of a query patched over the demo); demo leaves that ARE named
  examples carry their names (process3d "Timber Beam Joinery", forms "Building Component").
- `plugins-c` also: puzzle 2d/3d/5d retained example work answers exactly the published ids and refuses any other in its
  first step with the SDK's ONE `unknown_example` (it walks pre-decoded documents, so it cannot decode per step); the
  aliases `concrete`/`nakagin`/`capsule` go (2d loaded the EMPTY document, 3d faulted untyped "exceeds capacity", 5d
  completed a "too large" notice for an unknown id).
- `examples-stdio`: 26 editors that published nothing publish their demo; the 16 semio subset editors publish their OWN
  leaves (they decoded the base scaffold and opened empty) and their forms offer them; semio base's `📃️note` (the genuine
  text demo) moves to the text subset, base's hex scaffold `🎬️demo` goes; new semio leaves drawing/image; gif89a →
  `dancing`; mp4 and epw demos derive their document from the real file through the codec (deferred body; epw's file is
  materialized from the scaffold's hex dump as `🌦️.epw`) instead of publishing the hex dump; ifc: one leaf per standard —
  the IFC4 demo (and its native `🏗️.ifc`) moves to `4️⃣4/…/📚️examples/🎬️demo` (`examples::ifc4`), the 2x3 leaf carries the
  2x3 generator's output (`examples::ifc2x3`, the test-only `🧫️fixtures/🎬️demo` goes), both honesty laws and generators
  point at their own leaf.

### Session 15 log

- ≤20:10 read preambles 15/14, `📓️fault-localization-api.md`, the audit's "63 example loaders"; GUEST FREEZE on. Census tool
  (L1–L7 + P). Sites confirmed by reading: 62 stdio loaders, vcs `snapshot()`, ~30 plugin handlers, ~20 boot documents,
  duplicate codes. Finding: remodel's picker offered `demo-session` (a `.cmd.semio` replay, not a document → silent no-op).
  Design → S20 via main; S20: row 12 catalogues both codes, EX1 updates `app.example.unreadable` to carry {example} and
  removes the duplicates + `.fault`. Coordinator: LB2 p18 handed to EX1 → folded (no stdio codes, its demo-asset law became
  the SDK laws). Finding: the 15 semio subset editors publish the BASE demo and decode it as their own subset snapshot.
- 20:13 p1 queued (`sdk stdio`); 20:22 section `laws` (TDD: the law on every editor before its fix); finding: writer's form
  offers `jack`/`dag.jack` while it publishes only `demo` → `setActiveExample("jack")` opened an EMPTY document. 20:28 p1
  requeued with `laws` (same FIFO stamp); `""` semantics settled (navbar "No example" sends `""` → initial document).
- 20:29 p2 queued (+ `plugins-a`). 20:35 p1 R: one compile error (E0507 in the handler law) + one warning → fixed.
- 20:37 ops: the overlay lane has 2 slots; p2 was granted slot 2 while p1 held slot 1 and re-cloned the ONE overlay under p1.
  Rule since: one EX1 proof in the lane at a time.
- 20:46 p1 C measured (13 passed / 77 failed of 90 tests; triage `wp-ex1/generated/p1-C-triage.txt`); 20:47 p2 R **green** (resolver + catalogue
  laws); 20:52 p1 stopped (its L/W redundant). 20:56 p2 C (**48 passed / 42 failed of 90**, triage `p2-C-triage.txt`): real defects
  the old `unwrap_or_default` hid — 16 semio subset editors (base scaffold), 4 ifc-2x3 editors (raw IFC asset), gif89a (a GIF87a
  asset), mp4 (stale DSL), jpg ×2 (DSL round trip), gltf (pack round trip); 10 editors without any example; 4 pdf 1.7 reds
  at `editor-catalog:138` are baseline (`ui.snapshot-details.*` admission), not EX1.
- 21:15 p2 L: `crate::plugin` is a MODULE in architect and lowpoly (`crate::plugin::plugin()`) → both law files failed to
  compile and cargo ran no plugin law; W reported 0 compiled units (to re-verify in p3). Fixed in the set.
- 21:2x `plugins-c` (process3d, energy, shooting, forms, sourcing, animate, cad, trinity + 22 boot pins). Found: 22 editors
  publish their examples ONLY through the subset declaration, whose list the SDK registers at plugin assembly — ~40 unit
  tests dispatch examples without assembling → `subset_example_snapshot` + `pub fn examples()` on those subsets (the ONE
  list, read directly). Found: `PluginBuilder::editor` docs claim it stamps `E::examples()`, but only `register_app_factory`
  does (for `.editor::<E>` registrations); declared subsets publish `subset.examples`.
- 21:39 p3 queued (all 7 sections, steps R C L W, one cargo per crate for L/W so a compile error never hides another crate);
  set refreshed in the run dir before the grant (21:44, 21:46). 21:4x ajv oracle 69/69 on the materialized catalogs, red on a
  broken one. 21:50 p3 granted. 21:5x set applied to `.🧬semio/🌐hub/s15-ex1-scratch` (clone of S20's overlay): census
  post-state above; leftover references fixed (energy test `example_model`, trinity test doc, txt/csv boot pins — after p3's
  snapshot, they ride the next proof); `verify faults` on the p3 overlay: **0 violations** (3847 raises, 1915 declarations).
- 22:0x p3 C: 57/90 (`wp-ex1/generated/p3-C-triage.txt`): ifc-2x3 ×4, mp4, epw (a hex dump again), 16 semio/gif89a forms
  offering `demo` while publishing their own leaves, semio base publishing a TEXT document (`note`) — all fixed in the set
  (above); jpg ×2 / gltf round trips → LB2 (coordinator); `editor-catalog:138` details-UI admission reds are a flaky
  baseline (p2: pdf17 e/h/a/vt; p3: pdf17 h/vt/ua/x/any + ply — different editors each run, independent of examples).
- 22:0x coordinator: jpg/gltf → LB2; EX1 keeps puzzle, mp4 and ifc (after p20); every editor ≥ 1 real example. Done since:
  puzzle, mp4, epw, ifc split, semio restructure; catalog rows corrected for energy (15), norm en1995 (4), generation3d (9)
  (the first cut missed `examples()` delegations); every catalog row now names ≥ 1 example.
- 22:1x p3 L: `crate::plugin` is a module in 22 plugin crates (`mod plugin;` + `plugin_exports!`) → the law assembly call
  is chosen from the crate root; architect 2/2 and block 4/4 green. p3 stopped 22:13 (cargo 45795 + body 31084, my pids;
  slot released cleanly), p4 queued 22:14 with the complete set; set idempotence fixed (`once` treats an applied insertion
  as applied), binary-safe copies added, `--revert` restores byte-exact (0 differing files on the scratch clone).
