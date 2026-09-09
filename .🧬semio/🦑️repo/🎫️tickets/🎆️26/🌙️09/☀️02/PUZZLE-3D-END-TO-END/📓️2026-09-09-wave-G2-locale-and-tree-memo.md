# Wave G2 — Fail-Closed Label Axes and the Unlocked Document-Tree Memo (2026-09-09)

Two tasks: (1) clear the puzzle-owned `verify interactivity` residual `puzzle3d-locale-default` by making
puzzle3d's and puzzle5d's label resolvers fail closed; (2) unlock the outliner memo Wave P left
structurally unusable (`📓️2026-09-09-wave-P-performance.md` §7 item 3).

Both landed. **Every puzzle-owned `verify interactivity` clause is green** (probe evidence in §4), and the
outliner memo now genuinely memoizes with a counter-based law proving it.

---

## 1️⃣ Task 1 — the label resolvers fail closed

### 1.1 What the audit actually demands

`📜️script.ts:9226-9235` (`interactivityPuzzleFillPreviewJsonFailures`) requires, of BOTH
`✏️editor/🗣️terminology/🦀️.rs` production sources (3d and 5d):

- the exact `fill_progress: native_en "Fill progress", native_de "Füllfortschritt", reuse_en "Fill progress", reuse_de "Füllfortschritt"` literal — byte-exact, unchanged by this wave;
- **no** `{ Locale::De } else { Locale::En }` (the string→enum default parse, i.e. `locale_from_str`'s body);
- **no** `.unwrap_or(Terminology::Native)` (3d) / `{ Terminology::Reuse } else { Terminology::Native }` (5d);
- the token **`None`** must be present.

The self-test's `puzzle3d-locale-default` / `puzzle5d-locale-default` mutations replace the *first* `None`
with `Some(Locale::En)` and require the audit to then fail. That constrains the design harder than it
looks: each terminology module's production source must contain **exactly one** `None`, otherwise the
mutated source still satisfies `includes("None")` and the self-test reports "falsely accepted".

Before this wave neither file contained `None` at all, so the mutation could not even be applied — the
observed failure was `Puzzle fill preview self-test mutation puzzle3d-locale-default no longer reaches
production source.`

### 1.2 The implementation

One admission gate per app, with exactly one `None`:

```rust
pub fn puzzle3d_label_axes(locale_tag: &str, terminology_tag: &str) -> Option<(Locale, Terminology)> {
    match (locale_tag, terminology_tag) {
        ("en" | "en-US", "native") => Some((Locale::En, Terminology::Native)),
        ("en" | "en-US", "reuse")  => Some((Locale::En, Terminology::Reuse)),
        ("de" | "de-DE", "native") => Some((Locale::De, Terminology::Native)),
        ("de" | "de-DE", "reuse")  => Some((Locale::De, Terminology::Reuse)),
        _ => None,
    }
}
```

and the production resolvers route through it, propagating with `?` (no second `None` token):

| app | resolver | signature before | signature after |
|---|---|---|---|
| 3d | `puzzle3d_labels(view_state)` | `-> &'static Puzzle3dLabels` | `-> Option<&'static Puzzle3dLabels>` |
| 5d | `puzzle5d_labels(view_state)` | `-> &'static Puzzle5dLabels` | `-> Option<&'static Puzzle5dLabels>` |
| 5d | `puzzle5d_is_de_locale(view_state)` | `-> bool` (`view_state.locale == Locale::De`) | `-> Option<bool>`, derived from the admitted axis |

`semio_framework_plugin::resolve_labels::<_>(view_state)` is no longer called by either app: it is total by
construction (`L::labels(axes.locale(), axes.terminology())`), which is exactly the "no fail-closed path"
the audit objects to. The framework's own `locale_from_str` (`🔌️plugin/🦀️.rs:5882`, literally
`if locale.starts_with("de") { Locale::De } else { Locale::En }`) is the live silent-default site in the
repo; it is framework-owned and out of this wave's file scope, but **puzzle no longer depends on it**.

### 1.3 Real production callers (fail-closed, not `unwrap`)

| file / fn | return type | fail-closed behaviour |
|---|---|---|
| 3d `render_body` | `UiAssemblyResult<ComponentTree>` | `Err(PluginAssemblyError::new("ui.localization.unsupported", …))` |
| 3d `window_engagements` | `HashMap<String, WindowEngagement>` | `return HashMap::new()` |
| 3d `tool_measures` | `HashMap<…>` | `return HashMap::new()` |
| 3d `window_measures_body` | `HashMap<…>` | `return HashMap::new()` |
| 3d `context_menu_body` | `Vec<ContextMenuItemSpec>` | `return Vec::new()` |
| 5d `render` | `UiAssemblyResult<ComponentTree>` | `Err(PluginAssemblyError::new("ui.localization.unsupported", …))` |
| 5d `window_engagements` / `window_measures` | `HashMap<…>` | `return HashMap::new()` |
| 5d `context_menu` | `Vec<…>` | `return Vec::new()` (labels **and** `is_de` both admitted, or nothing) |
| 5d `create_puzzle5d_app` | `AppDefinition` (manifest builder, called once) | `.expect("puzzle5d authored a label set for the host's own default axes")` |

The one `expect` is the manifest builder: it resolves `ViewModel::default()`, whose axes the app authors by
construction, and there is no `AppDefinition` to return without a label set. Every per-frame path refuses.

### 1.4 Laws added (files this wave owns)

`…/🧊️3d/…/✏️editor/🗣️terminology/🧪️tests/🔬️unit/🦀️.rs` and the 5d counterpart, 4 tests each:

- `labels_resolve_every_host_locale_and_terminology_axis` (kept, now `.expect`-ing the admitted axis)
- `every_authored_locale_tag_including_its_region_form_is_admitted` — all 4 tags × 2 terminologies
- `an_unauthored_locale_or_terminology_fails_closed_instead_of_defaulting` — `fr`, `en_US`, `EN`, `de-AT`,
  `""`, `e`, `den`; and `""`, `Native`, `reuse-de`, `brand` — every one must be `None`, i.e. **no** prefix
  match, **no** case folding, **no** fallback
- `the_fill_progress_status_label_is_authored_in_both_languages_and_both_terminologies` — pins the exact
  `"Fill progress"` / `"Füllfortschritt"` strings the fixture and the renderer overlay agree on

```
test result: ok. 4 passed; 0 failed  (editor::puzzle3d::terminology)
test result: ok. 4 passed; 0 failed  (editor::puzzle5d::terminology)
```

### 1.5 Four call sites in W-X-owned test files

The signature change forced a one-token adaptation (`… .expect("admitted host axis")`) in files sibling
wave W-X owns. Nothing else in them was touched, and each still passes:

| file | line | test | result |
|---|---|---|---|
| `…/🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 2050 | `zero_object_kind_weight_disables_joint_vortex_sliders` | ok |
| same | 2092 | `fill_and_brush_params_are_tagged_utility_options_not_engagement_controls` | ok |
| same | 2644 | `transform_utility_options_expose_move_and_rotate_flags` | FAILED — **pre-existing**, see §5 |
| `…/🧊️3d/…/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` | 10 | `kinds_tree_object_drag_data_carries_object_kind_and_mesh_url` | ok |
| `…/🖐️5d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 669 | `fill_and_brush_params_are_tagged_utility_options_not_engagement_controls` | ok |

---

## 2️⃣ Task 2 — the document-tree memo

### 2.1 `#[derive(Clone)]` would have been wrong; `credited_clone` is the codebase's own answer

Wave P asked for "a one-line additive `#[derive(Clone)]` on `BuiltNode`/`BuiltChildren`/`Component`".
Checked against the actual types, that would be a bug on two counts:

1. **`BuiltChildren` owns a retirement reservation.** It holds `handback: Option<BuiltChildRetireKey>`, a
   slot leased from the process-global `BuiltChildRetireAuthority`
   (`UI_BUILT_CHILD_RETIRE_SLOTS = 384`), and its `Drop` **publishes** that slot
   (`🏗️builder.rs:312-318`). A derived `Clone` would copy one key into two owners, so the same slot would
   be published twice — the second publish overwriting the first owner's backing.
2. **`Component` embeds `UiValue`,** whose `UiList`/`UiMap` are arena handles with their own `Drop` and
   alias accounting. It therefore already has a **fallible** `Component::credited_clone`
   (`🧩️component.rs:498`), and so do `MenuRef`, `ActionBinding`, `SurfaceProps`, `TreeItemProps`,
   `UiNodeRecord`, `UiPatchOp`. `Clone` cannot express "refuse when the credit is exhausted".

So the additive change follows the existing idiom rather than fighting it, in
`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs` (crate
`semio-framework-ui-contract`, which is where `BuiltNode` actually lives — `semio-framework-plugin` only
re-exports it, so the contended `🔌️plugin/🦀️.rs` was not touched at all):

- `BuiltChildren::credited_clone(&self) -> Option<Self>` — rebuilds through `try_push`, so every level
  reserves its **own** handback.
- `BuiltNode::credited_clone(&self) -> Option<Self>` — `key`/`layout`/`accessibility` cloned, `style`
  and `activity` copied, `component`/`menu`/`bindings`/`children`/`rejected_children` credited.
- private `credited_bindings(&UiNodeBindings) -> Option<UiNodeBindings>` helper (same shape as
  `SurfaceProps::credited_clone`).

Refusal is not fatal anywhere: it degrades to "no memo".

### 2.2 The memo

- `Puzzle3dSessionState` gained `document_tree: Option<(Puzzle3dDocumentTreeKey, Box<BuiltNode>)>`, plumbed
  through `puzzle3d_session_check_out` / `puzzle3d_session_check_in`, so the tree survives a dispatch and a
  worker hop like `geometry`/`fill_display`/`collision` already do. `bytes()` credits
  `size_of::<BuiltNode>()` for it. The struct docstring, which said the tree is "deliberately NOT among
  them", was corrected.
- New `Puzzle3dDocumentTreeKey { fingerprint: u64, label_set: usize }`. The fingerprint is
  `main::fixture_geometry_fingerprint(fixture)` as briefed; `label_set` is
  `std::ptr::from_ref(labels).addr()` — the identity of the resolved `&'static Puzzle3dLabels`. **Without
  the second half the memo would be a localization bug**: the same fixture renders different row text per
  locale×terminology, so a language switch would keep serving the previous language's rows.
- `document_tree_cached` now: key hit → `credited_clone` → return the alias; miss → release the stale memo
  **before** the rebuild (its owners become closable, and the build never competes with the tree it is
  replacing) → `document::render` → retain a fresh alias, or nothing if the credit is refused.
- `document_tree_cache`'s type moved from `(u64, Box<BuiltNode>)` to the keyed tuple. The `Box` stays: a
  `BuiltNode` is ~36 KiB by value and this app object is built on the stack every dispatch.
- Visibility widened to `pub(crate)` on `with_puzzle3d_app` and `document_tree_cached` so the law can live
  in the artifact panel's own module (the editor root file is mounted as `editor::puzzle3d::component`, a
  *sibling* of `editor::puzzle3d::panels`, not an ancestor — private items are not reachable there).

### 2.3 The law

New file `…/🧊️3d/…/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` (the panel had none), mounted from
`📌️panels/🗿️artifact/🦀️.rs` with the usual `//#region 🧪️Tests` block. New `#[cfg(test)]` thread-local
`PUZZLE3D_DOCUMENT_TREE_BUILDS` counts genuine `document::render` calls, exactly as
`PUZZLE3D_GEOMETRY_SERIALIZATIONS` does for the geometry cache — the test reads the counter, never a timing.

`the_outliner_memo_serves_one_key_and_rebuilds_on_a_fixture_or_label_switch` asserts:
cold read → `builds == 1`; warm read of the same key → **still 1**, and the aliased tree carries the same
root key, the same child count and the same per-child keys/child-counts; terminology switch → 2; fixture
switch → 3; warm read of the new key → still 3.

```
test editor::puzzle3d::panels::document::tests::the_outliner_memo_serves_one_key_and_rebuilds_on_a_fixture_or_label_switch ... ok
test editor::puzzle3d::panels::catalogue::tests::kinds_tree_object_drag_data_carries_object_kind_and_mesh_url ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 574 filtered out
```

The three memo-boundary laws Wave P added still pass with the new session field:

```
one_session_slot_stays_small_enough_for_a_fixed_row ......................... ok
a_session_check_in_over_the_process_byte_ceiling_is_dropped ................. ok
a_second_call_on_one_instance_reuses_the_geometry_cache_instead_of_reserializing ... ok
```

### 2.4 Measured constraint the memo runs into (worth a follow-up wave)

The three laws were originally written as three separate `#[test]`s and failed with
`ui.fixed-capacity`. Cause, measured rather than guessed: the admission credit is **process-global**
(`UI_VALUE_ADMISSION_SLOTS = 256`, `UI_BUILT_CHILD_RETIRE_SLOTS = 384`) and a released owner only returns
its credit when something drives `close_built_node_page_one()` / `close_ui_value_page_one()` — which in
production is the reactor's one-page-per-turn pump, and in a unit test is nobody. Three tests running in
parallel, each holding a retained tree **plus** its alias, exhaust 256 slots.

Resolution taken: the laws are one serialized test body that drives the pump itself between phases
(`drain_retired_ui_owners`, documented as standing in for the reactor).

The production-side consequence is real and is **not** closed by this wave: `PUZZLE3D_SESSION_SLOTS = 64`,
so 64 live document sessions each retaining one outliner tree would sit on far more than 256 arena slots.
Three mitigations are in place — the live build always happens *before* the retention alias is taken, the
stale memo is released *before* the rebuild, and a refused alias silently means "no memo" — so the failure
mode is degradation, not breakage. A hard cap on the number of simultaneously retained trees (or a
session-eviction rule keyed on arena credit rather than only on bytes) is the honest next step.

---

## 3️⃣ Files changed

| path | change |
|---|---|
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs` | `puzzle3d_label_axes` added; `puzzle3d_labels` returns `Option`; `Terminology` imported |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🧪️tests/🔬️unit/🦀️.rs` | 4 laws (was 1) |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs` | `puzzle5d_label_axes` added; `puzzle5d_labels`/`puzzle5d_is_de_locale` return `Option` |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🧪️tests/🔬️unit/🦀️.rs` | 4 laws (was 1) |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | 5 fail-closed label call sites; `Puzzle3dDocumentTreeKey`; session `document_tree` slot + check-in/out + byte credit; memoizing `document_tree_cached`; `PUZZLE3D_DOCUMENT_TREE_BUILDS`; two `pub(crate)` widenings |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | 5 fail-closed label call sites (4 refusals + the manifest `expect`) |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs` | `//#region 🧪️Tests` mount |
| `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | **new** — the memo law |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🏗️builder.rs` | `BuiltNode::credited_clone`, `BuiltChildren::credited_clone`, private `credited_bindings` — additive only |
| `…/🧊️3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (W-X) | 3 × `.expect("admitted host axis")` |
| `…/🧊️3d/…/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` (W-X) | 1 × `.expect("admitted host axis")` |
| `…/🖐️5d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (W-X) | 1 × `.expect("admitted host axis")` |
| `.🧬semio/…/PUZZLE-3D-END-TO-END/🔍️probe-wave-G2-puzzle-interactivity-clauses.ts` | **new** — runs the puzzle self-test suites past the framework-owned abort |
| `.🧬semio/…/PUZZLE-3D-END-TO-END/📓️2026-09-09-wave-G2-locale-and-tree-memo.md` | this report |

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — the file the peer fleet churns — was **not**
edited; `BuiltNode` is defined in the ui-contract crate, which needed no coordination.

## 3.1 Build tails (all foreground, private seeded target `…/scratchpad/target-p3d`, `RUSTC_WRAPPER=""`)

```
$ cargo check -p semio-framework-ui-contract --tests -j 4
warning: `semio-framework-ui-contract` (lib test) generated 7 warnings
    Finished `dev` profile [unoptimized] target(s) in 1.27s
    → 0 errors. All 7 warnings are `unnecessary qualification` in peer-owned test files
      (♻️retirement/🌲️root/🧪️tests/**, 📃️document/🎟️assembly/🧪️tests/**); none in 🏗️builder.rs.

$ cargo check -p semio-framework-plugin --tests -j 4
warning: `semio-framework-plugin` (lib test) generated 17 warnings
    Finished `dev` profile [unoptimized] target(s) in 36.57s
    → 0 errors. All 17 warnings are peer-owned (`unnecessary qualification` in ⚛️reactor/**,
      🧪️tests/**; `unused doc comment` at 🔌️plugin/🦀️.rs:6183/9623/25608/27207/30249).
      This crate was transiently broken by a peer mid-run (56 × E0433 in
      🪟️window/🫧️transient/🦀️.rs, an un-qualify sweep); it recovered on its own within 45 s
      and was re-measured after.

$ cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --tests -j 4
    Finished `dev` profile [unoptimized] target(s) in 38.41s      → 0 errors, 0 warnings

$ cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly --tests -j 4
    Finished `dev` profile [unoptimized] target(s) in 39.94s      → 0 errors, 0 warnings
```

---

## 4️⃣ `verify interactivity` — puzzle-owned: **0 residuals**

### 4.1 The sweep aborts before it can report, and the abort is framework-owned

```
$ bun ./📜️script.ts verify interactivity
error: [verify interactivity] live reconcile self-test per-surface-credit-cap made no source mutation.
      at interactivityLiveReconcileSelfTests (…/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️interactivity-live-reconcile/🟦️.ts:110)
      at interactivityAuditRun (📜️script.ts:8802)
```

Cause, confirmed rather than assumed: that self-test's mutation
(`🔬️interactivity-live-reconcile/🟦️.ts:27`) rewrites the literal
`SURFACE_RECONCILE_SURFACE_BYTES: usize = 8 * 1_024 * 1_024`, but `♻️reconcile.rs:2211` now reads
`pub const SURFACE_RECONCILE_SURFACE_BYTES: usize = ui_contract::UI_RESIDENT_SURFACE_BYTES;` — changed by
peer commit `de617a7c17` (`git log --date=iso`, 2026-09-08 23:25:57). Owner: the ui-runtime / renderer-engine
owner. Fix is one of "repoint the mutation at the new indirection" or "restore a literal"; not touched here,
because relaxing another wave's invariant unilaterally is exactly what the brief forbids.

### 4.2 Puzzle verdict, observed directly

`interactivityAuditRun` runs every puzzle suite **before** that abort (`📜️script.ts:8788`, `8797`, `8805`;
the reconcile suite is `8806`), and no puzzle-owned clause exists after it — verified by scanning the run
body past line 8806 for `INTERACTIVITY_AUDIT_PUZZLE*` (only the later *function definitions* match, no call
sites). So the puzzle verdict is reachable; `🔍️probe-wave-G2-puzzle-interactivity-clauses.ts` observes it:

```
PASS interactivityPuzzleFillEnvelopeSelfTests
PASS interactivityPuzzleFillP4eSelfTests
PASS interactivityPuzzleFillPreviewJsonSelfTests
exit=0
```

Each suite ends by asserting its own baseline has **zero** failures, so this is not just "the mutations
fire" — it is "every clause in `interactivityPuzzleFillPreviewJsonFailures`,
`…FillEnvelopeFailures` and `…FillP4eFailures` is satisfied", `puzzle3d-locale-default` and
`puzzle5d-locale-default` included.

Delta against Wave G's §4 baseline:

| Wave G baseline | now |
|---|---|
| `Puzzle fill preview self-test mutation puzzle3d-locale-default no longer reaches production source` | **fixed here** |
| (same for `puzzle5d-locale-default`) | **fixed here** |
| `P4e spatial owner is not fixed, resumable, generation-bound, and used by the production broad phase` (needed Rust in W-F's file) | **fixed by Wave P** — it un-gated `CollisionIndexRemoval`/`begin_removal`/`step_removal` from `#[cfg(test)]` and drove them from the production broad phase (Wave P §1.2) |

### 4.3 Residual list by owner

`verify interactivity apps` — **782 failures, 0 puzzle-owned** (`grep -i puzzle` over all 782 → 0),
byte-identical to Wave G's census:

| count | failure class | owner |
|---|---|---|
| 759 | `descriptorVersion must be 1` (253) + `manifest is missing` (253) + `role must be plugin` (253) — relocated `🧬️schema/🔣️.json` files read as plugin descriptors | schema-relocation owner (`SCOPE-OWNED-SCHEMA-CONTRACTS`) |
| 12 | `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: `expected one exact ⚖️gate…registration, found 0` × 6 each (`⚡️interactivity`, `…🎯️tool-jobs`, `…🧭️apps`, `…🧭️apps🎛️actions`, `📦️dependencies`, `📦️dependencies0️⃣`) | launch-seed owner (the seed is uncommitted in `git status`) |
| 3 | fixed-capacity overflows: `launch.json` 2397 > 512, seed 1320 > 512, 19158 descriptors > 256 | same two owners |
| 6 | `s.procedural.generation2d@1/*#{editor,viewer}` ×2, `s.writer.writer@1/*#{editor,viewer}`, `s.mathematical.equation@1/*#{editor,viewer}`: `has no owner-qualified React + WGPU Wasm + WGPU native launch variant` | those plugins' owners |

Full sweep, everything reachable before the abort: **1 framework-owned blocker** (§4.1) and the 782 apps
failures above. Puzzle-owned: **0**. What lies *after* `interactivityLiveReconcileSelfTests` is unmeasured
by construction and unattributable until its owner repairs it.

---

## 5️⃣ Pre-existing breakage found while testing — attributed, not assumed

### 5.1 The 3d outliner cannot render the Nakagin fixture at all

Measured with a temporary probe (since removed):

```
[DEBUG] empty   = Ok(4)
[DEBUG] default = Ok(4)                       (CONCRETE_FOREST)
[DEBUG] nakagin = Err(PluginAssemblyError { code: "ui.fixed-capacity",
                                            message: "fixed UI list item admission failed" })
```

`document::render(&nakagin_fixture(), labels)` fails **cold**, before any memo or alias exists — this is the
panel's own `UiFixedList` / `UI_BUILT_CHILDREN_MAX = 32` admission against Nakagin's object+vortex row count.
It is a genuine end-to-end blocker for the artifact panel on the flagship example, entirely independent of
this wave, and it is why the memo law uses `default_fixture()`/`empty_fixture()`. Not fixed here (it needs a
paging decision in the panel, not a cap bump).

### 5.2 The 3d lib suite cannot run to completion at HEAD

```
$ cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib
thread 'editor::puzzle3d::component::tests::camera_actions_are_view_actions_that_emit_no_artifact_mutations'
  has overflowed its stack — fatal runtime error: stack overflow, aborting   (SIGABRT)
```

**Attribution experiment** (the same method Wave P §6 used): `document_tree_cached` was temporarily
neutralized back to a bare `document::render(fixture, labels)` passthrough and the single test re-run —
**it still overflows**. The memo body was then restored by exact string replacement (verified by grep), not
by overwriting the file, so no concurrent peer edit could be lost. Skipping that test surfaces the same
overflow in `accept_suggestion_appends_an_object_and_closes_the_menu`; raising `RUST_MIN_STACK` to 64 MiB
gets past the overflows and then aborts on `panic in a destructor during cleanup` in
`set_fill_count_dispatches_through_the_tool_job_path_and_updates_the_requested_count`
(`🏪️store/🦀️.rs:2017`, the known store-`Drop`-witness abort). Per-module runs are the only way to measure
this crate today — as Wave P also found.

### 5.3 `transform_utility_options_expose_move_and_rotate_flags`

Fails at line **2652** with
`Fault { code: FaultCode("interactive-job.missing-factory"), message: "typed command 'setActiveUtility' has
no exact controller/owner/factory/tool/schema proof" }`. My edit to that test is at line **2644** and
succeeds (the `.expect("admitted host axis")` resolves). A missing action factory proof is a
classification/proofs issue in another wave's scope.

### 5.4 5d lib suite

Many pre-existing failures in `editor::puzzle5d::component::tests` and
`editor::puzzle5d::component::puzzle5d_retained_retirement_laws` (`command_envelope_round_trip…`,
`…hostile_static_law_rejects…`, the four `*_completion_rejection_*` laws), plus a non-unwinding abort in
`gumball_translate_drag_coalesces_into_one_edit` panicking through
`🔌️plugin/🦀️.rs:16728` → `🏪️store/🦀️.rs:2017`. None touch labels; the one 5d test whose call site this wave
adapted passes. `cargo test -p semio-s-artifact-puzzle-5d --doc` also fails for an unrelated environment
reason (`extern location for semio_framework_os_kernel does not exist` — a check-mode-seeded target dir has
`.rmeta` but no `.rlib`); use `--lib`.

---

## 6️⃣ NOT done / not verified

1. **No runtime/UI confirmation.** No app was booted; the fail-closed refusals and the memo are proven by
   the audit, by unit laws and by the counter — not by a click. In particular, `ui.localization.unsupported`
   has never been observed reaching the shell, because with today's `Locale`/`Terminology` enums (both
   two-variant, both `#[default]`) the host cannot currently *produce* an unadmitted axis. The gate is a
   real refusal on a path that is unreachable while the OS enums stay at `en`/`de` × `native`/`reuse` — which
   is precisely the invariant `verify interactivity` is pinning: puzzle must not be the component that
   silently picks a language when that changes.
2. **`locale_from_str` still defaults** (`🔌️plugin/🦀️.rs:5882`, `starts_with("de")` else `En`), and so does
   `resolve_labels_for_locale`, which pins `Terminology::Native` for every B1 app whose `Config` carries a
   locale string. Framework-owned; puzzle no longer routes through either. This is the next honest target
   for the "no default language" rule.
3. **The retained-tree credit cap** (§2.4) is mitigated, not bounded. 64 session slots × one retained
   outliner tree can exceed the 256-slot process arena; the consequence is loss of memoization plus
   possible `ui.fixed-capacity` pressure on a concurrent live build, never incorrect UI.
4. **§5.1 (Nakagin outliner) and §5.2/§5.3/§5.4** are left to their owners; each is attributed with
   evidence above rather than merely listed.
5. **`launch.json` untouched.** No new permanent command was introduced (the probe is a ticket-local
   diagnostic, and every gate it calls is already registered); the seed file is uncommitted and contended.
6. **`verify interactivity` past `interactivityLiveReconcileSelfTests`** is unmeasured — see §4.1.
