# 📓️ P1 — Contract + ViewModel (design §3, §4, §7)

Wave 1, packet P1. All paths relative to the repo root. Measured 2026-09-17 under a saturated build
fleet (47–57 concurrent `cargo` processes; several runs queued minutes on the shared artifact lock).

## 1. What changed

### 1.1 `semio_framework_ui_contract`

| File | Change |
|---|---|
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs` | New `pub struct TreeWindow { total: u32, offset: u32 }` with exactly the design §3 derives (`Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue`, both `rename_all = "camelCase"`, `#[value(crate = "::protocol::value")]`). `TreeSectionProps.window: Option<TreeWindow>`, `TreeItemProps.window: Option<TreeWindow>`, `TreeItemProps.granularity: Option<UiText>` — all `#[serde(default, skip_serializing_if = "Option::is_none")]` (plus the matching `#[value(..)]` on `TreeSectionProps`, which is the only one of the two that derives `ToValue`/`FromValue`). `TreeItemProps::credited_clone` copies both new fields. |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🦀️.rs` | Untouched — `pub use component::*` already exports `TreeWindow` from the crate root, like every other props type. |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs` | `TreeSectionBuilder::window(TreeWindow)`, `TreeItemBuilder::window(TreeWindow)`, `TreeItemBuilder::granularity(UiText)`, all assembled into the props in `From<…> for BuiltNode`. `UI_BUILT_CHILDREN_MAX` re-based (below) with a real docstring; `UI_BUILT_CHILD_RETIRE_SLOTS` given a docstring stating why it does **not** move. The stray `BuiltNode` docstring that sat on `UI_BUILT_CHILDREN_MAX` was moved onto `BuiltNode`, where it belongs. |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs` | `UI_VALUE_PAGE_ROWS` re-based (below); its docstring rewritten per §3/§7 — the "four resident surfaces" claim is gone, the aggregate is named as `UI_RESIDENT_SLOTS × UI_RESIDENT_DOCUMENT_BYTES`, and the continuation-row justification is replaced by the window law. `UI_VALUE_LIVE_PAGES` unchanged at 1. |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs` | New `TYPES` entry `TreeWindow` (version 1, alphabetically between `TreeSectionProps` and `Trigger`); `window?`/`granularity?` added to the `TreeSectionProps`/`TreeItemProps` strings in their struct field order, with the same inline JSDoc style the neighbours use. No version bumps — every one of the 81 entries is still `version: 1` (greenfield repo, no legacy projection). |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧬️typegen-export/🦀️.rs` | `TYPES.len()` 80 → 81. |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️component-unit/🦀️.rs` | `component_round_trips` now exercises the new fields (`TreeWindow { total: 512, offset: 128 }`, `granularity: "piece"`). |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/⚖️compare/🧫️fixtures/🔣️.json` + `⚖️compare/🧪️tests/⚖️compare/🦀️.rs` | `pageCount` 434 → 1792, `littleEndian` `b101000000040000` → `ff06000000040000`, and the hard-coded `ValueFrame::checked_page(UI_VALUE_AGGREGATE_ITEMS - 1) == 433` → `1791` (0x06FF). Driven by the `UI_VALUE_PAGE_ROWS` change. |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧵️retained/📦️wire/🧫️fixtures/🧾️typed/🔣️.json` | `expected` rows for `treeSection`/`treeItem` extended with `window: null` (+ `granularity: null`), keeping the fixture's "normalized = every field explicit" convention and matching the TS twin's `fixed({…})` key set. |

Not touched by me but landed in the same commit by a concurrent peer (they are the typed-projection
plumbing for `TreeWindow`, and they are correct): `🧾️typed/🦀️.rs` (visitor rows for `TreeWindow`,
`TreeSectionProps`, `TreeItemProps`), `🪞️copy/🦀️.rs` and `⚖️compare/🦀️.rs` (`u32` added to `scalar!`),
`♻️retirement/🌳️typed/🧱️component/🦀️.rs` (`u32` added to `typed_scalar!`), and the TS twin
`🧵️retained/📦️wire/🧾️typed/🟦️.ts`.

### 1.2 `semio-framework` (the crate that `#[path]`-mounts `🛂️manifest/🦀️.rs`; there is no `semio-framework-manifest`)

| File | Change |
|---|---|
| `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` | New `pub struct TreeWindowRequest { body_key, node_key, open: Option<bool>, offset: u32, rows: u32 }` exactly as design §4 (`Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue`, `rename_all = "camelCase"` on both, `skip_serializing_if` on `open` only). `ViewModel.tree_windows: Vec<TreeWindowRequest>` (`default` + `skip_serializing_if = "Vec::is_empty"`) and `ViewModel.tree_viewport_rows: Option<u32>` (`skip_serializing_if = "Option::is_none"`). `for_panel()`/`for_window_instance()` were already `..self.clone()`, so both survive — pinned by the tests below. New capacity constants `VIEW_CONTEXT_TREE_WINDOWS = 128`, `VIEW_CONTEXT_TREE_WINDOW_FIELDS = 5`, `VIEW_CONTEXT_TREE_WINDOW_NUMERIC_FIELDS = 2`, folded into `VIEW_CONTEXT_ENCODED_VALUES` and `MAX_SURFACE_VIEW_CONTEXT_BYTES`. |
| `🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧬️schema/🔣️.json` | New `treeWindows` (array, `maxItems: 128`, `$ref` `TreeWindowRequest`) and `treeViewportRows` (integer 0…2³²−1) properties, plus the `TreeWindowRequest` `$def`. **Required**: the schema is `additionalProperties: false`, so without this the host's own view state would be refused. |
| `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️view-context-capacity/🦀️.rs` | Pins `treeWindows.maxItems` and the `TreeWindowRequest` field count against the neutral schema; the capacity-filled `ViewModel` literal now fills 128 requests + `tree_viewport_rows` and still fits `MAX_SURFACE_VIEW_CONTEXT_BYTES`. |
| `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts` | `PluginViewState.treeWindows?`/`treeViewportRows?` + `ViewTreeWindowRequest` type; `parseResolvedPluginViewState` admits and validates them (capacity 128, `Identifier` keys, `u32` bounds, optional boolean `open`). **`viewContextWithIntegerCarriers` now mints `treeViewportRows` and each request's `offset`/`rows`** — without this the React door would widen them onto IEEE doubles and the guest's `FromValue` would refuse the whole view state (the exact failure mode recorded for `toolRunTraceCursorByWindowId.run` in 26/09/09/PROCEDURAL-3D-END-TO-END). |
| `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs` | New `TYPES` entry `TreeWindowRequest` (between `ToolRunVerdict` and `TutorialArtifactEvent`); `treeWindows`/`treeViewportRows` added to the `ViewModel` entry. |
| `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs` | `TYPES.len()` 196 → 197. |

## 2. Constants, before → after

| Constant | Before | After |
|---|---|---|
| `UI_BUILT_CHILDREN_MAX` (`🏗️builder`) | `32` | `crate::UI_DOCUMENT_NODES` = **128** |
| `UI_BUILT_CHILD_RETIRE_SLOTS` (`🏗️builder`) | `384` | `384` (unchanged, docstring added) |
| `UI_VALUE_PAGE_ROWS` (`🎬️action`) | `UI_BUILT_CHILDREN_MAX - 1` = `31` | `crate::UI_BUILT_CHILDREN_MAX` = **128** |
| `UI_VALUE_LIVE_PAGES` | `1` | `1` (unchanged, as design says) |
| derived `UI_VALUE_ADMISSION_SLOTS` | `155` | `640` |
| derived `UI_VALUE_AGGREGATE_ITEMS` | `434` | `1792` |
| derived `UI_VALUE_HANDBACK_WORDS` | `3` | `10` |
| `VIEW_CONTEXT_TREE_WINDOWS` | — | `128` (new) |
| `TYPES.len()` ui-contract | `80` | `81` |
| `TYPES.len()` framework | `196` | `197` |

### 2.1 Retirement paging at the wider fan-out — no law breaks

`UI_BUILT_CHILD_RETIRE_SLOTS` counts **pages in flight** (live + handed back), not children per page:
`BuiltChildRetireAuthority::reserve` takes one slot per `BuiltChildren` that has allocated backing,
regardless of how wide that backing is. `close_built_node_page_one` →
`BuiltChildRetireAuthority::take_close_page` still retires **exactly one child per call** and only
frees the slot once that owner's cursor reaches its `len`. So widening the page costs more *calls*,
never a bigger frame, and the U1 run-to-completion budget is unaffected. Constant left at 384; the
reasoning is now written down in its docstring so the next reader does not have to re-derive it.

## 3. Regenerated files

- `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` — carries `TreeWindow`,
  `TreeSectionProps.window`, `TreeItemProps.window`/`granularity`. Proven current by the typegen test
  (which byte-compares `render_typescript()` against the file) passing with no `SEMIO_TYPEGEN_OUT`.
- `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts` — line 977 `TreeWindowRequest`,
  line 1453 `treeWindows: Array<TreeWindowRequest>`, line 1458 `treeViewportRows?: number`.
  Regenerated with `bun nx run @semio-tech/framework-rs:generate` — **note the project is
  `@semio-tech/framework-rs`, not `@semio-tech/framework`**; the latter has no `generate` target
  (`NX Cannot find configuration for task @semio-tech/framework:generate`).

## 4. Commands run, with results

All run in the foreground (long ones detached with `nohup` and polled to completion inside this turn,
because the shared-artifact-dir lock made single calls exceed the 600 s tool cap). Logs are under
`🗑️generated/p1/`.

| Command | Result |
|---|---|
| `cargo check -p semio-framework-ui-contract --all-features` | **ok** |
| `cargo test -p semio-framework-ui-contract --features typegen --test typegen_export` | **ok — 1 passed, 0 failed** (`TYPES.len() == 81`, rendered output byte-equal to the generated mirror) |
| `bun nx run @semio-tech/ui-contract-rs:check` | **ok (rc 0)** — `wasm32-wasip2`, `wasm32-unknown-unknown`, and `wasm32-wasip2 --features typegen` |
| `cargo check -p semio-framework-ui-contract --target wasm32-wasip2` | **ok (rc 0)** |
| `cargo check -p semio-framework-plugin` | **ok (rc 0)** — P3's crate compiles against the new contract; I did not touch `🔖️PanelPaging` |
| `bun nx run @semio-tech/framework-rs:generate` | **ok (rc 0)** |
| `cargo test -p semio-framework --lib` | **FAILED — 258 passed, 11 failed** (an earlier `--all` run: 255 passed / 14 failed; the set is not stable between runs) |
| `cargo test -p semio-framework-ui-runtime --lib` | **FAILED — 122 passed, 2 failed** (identical with `--test-threads=1`: 122 / 2) |
| `cargo test -p semio-framework-ui-contract --all-features` | **FAILED — 64 ok, 12 FAILED of 180, then SIGABRT** (a panic inside a `Drop` aborts the process before the harness prints the failure section) |
| `bun nx run @semio-tech/ui-contract-rs:generate` | **rc 1** — the cargo child was `killed by signal SIGKILL` under fleet memory pressure. Not a code failure: the mirror it would write is already byte-current, proven by the typegen test above. |

### 4.1 Tests that pass and prove this packet

From `cargo test -p semio-framework --lib` (all **ok**):

- `manifest::view_context_capacity_tests::capacities_match_the_neutral_schema` — `treeWindows.maxItems`
  and the `TreeWindowRequest` field count agree with `🪟️view-context/🧬️schema/🔣️.json`.
- `manifest::view_context_capacity_tests::a_capacity_filled_context_fits_the_bound` — a `ViewModel`
  filled to every schema capacity, **including 128 `TreeWindowRequest`s and `tree_viewport_rows`**,
  still encodes inside `MAX_SURFACE_VIEW_CONTEXT_BYTES`.
- `manifest::view_context_capacity_tests::the_admission_bound_covers_every_schema_valid_context`,
  `…::contributions_are_not_a_view_context_field`.
- `manifest::view_context_integer_carrier_tests::{both_doors_carry_the_view_context_as_the_same_exact_pack_bytes, the_guest_decodes_every_integer_field_exactly, a_widened_float_is_refused_and_never_rounded}`
  — the pinned `packHex` is unchanged, which is the proof that both new fields serialize away at
  their defaults and no existing crossing moved a byte.
- `manifest::window_view_context_tests::*` and `manifest::resolved_host_context_tests::*` — `for_panel`
  / `for_window_instance` keep the new fields.

The `⚖️compare` law (`retained_component_compare_frame_storage_matches_exact_bounded_domains`) failed
before the fixture fix in §1.1 and passes now.

## 5. Failures I could NOT fix, and why

Three crates carry byte-exact / exact-retirement law failures. **None of them is in a file this
packet owns, and none is a `+N`/paging/window law.** Two distinct causes, both documented here so the
coordinator can dispatch rather than re-investigate.

### 5.1 `size_of::<Component>()` grew 2568 → 3096 bytes — measured, and it breaks page-exact laws

The design's §3 field set costs real inline bytes: `TreeItemProps` is the widest `Component` variant,
and `granularity: Option<UiText>` adds ~516 bytes (a `UiText` is an inline `[u8; 512] + u16`, never
boxed) while `window: Option<TreeWindow>` adds 12. Measured live in the runtime crate with a
temporary `[DEBUG]` print (since removed):

```
[DEBUG] sizes component=3096 patch-op=6416 comparison=2256 tree-node=6456
```

3096 − 516 − 12 = 2568, the pre-change size.

`semio-framework-ui-runtime`, 2 failures, **reproduced identically with `--test-threads=1`**, in files
nobody has touched since 2026-09-15 — so they are a consequence of this dependency change, not a peer
edit:

- `reconcile::tests::canonical_document_tests::surface_canonical_document_completion_transfers_do_not_borrow_the_child_grant`
  (`🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📃️document/🧪️tests/📃️document/🦀️.rs:161`). I instrumented
  it: of the five completion flags only `full_close_grant` is false, and the reason is that the
  comparison-lease close turns now charge **7568 bytes** where the law expects exactly one 4096-byte
  page (`[DEBUG] closing-turn bytes=` … `7568, 7568, 7568, 7568, 2496`). The fixture's `workBytes` is
  4096 and the test's own `[DEBUG]` banner calls this "lease-close=4096".
- `reconcile::tree_retirement::tests::runtime_tree_retirement_preserves_occupied_sources_and_closes_exact_payloads`
  (`♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs:38`) — "runtime exact tree owner did not reach terminal"
  after 1 000 000 `close_step(1, grant)` calls.

**Why I stopped here:** these are the reconcile crate's page-exactness laws, and design §7 already
assigns the reconcile byte laws to **P5** ("a new law in the reconcile tests prints
`size_of::<FlatPresentedNode>()` and asserts a 128-row window × 4 sections reconciles under
`SURFACE_RECONCILE_SURFACE_BYTES`"). Re-deriving "one close turn = one 4 KiB page" for a 3096-byte
`Component` / 6456-byte `TreeNode` is that same re-derivation and should be done once, by the packet
that owns it, not bumped to 7568 here. **P5 must be told `size_of::<Component>()` is now 3096 and
`size_of::<crate::TreeNode>()` 6456.**

### 5.2 Pre-existing / peer-owned failures — NOT caused by this packet

- `semio-framework --lib`: 11–14 failures, varying run to run, all in `action_bus::tests`,
  `manifest::app_label_tests::tutorial_artifact_event_kind_round_trips_tagged_camel_case`,
  `manifest::kernel::extension_activation_tests`, `manifest::kernel::ui_turn_patch_tests`. Proof that
  at least part of this set is independent of my change: one of them fails on
  `assertion failed: !std::mem::needs_drop::<FixedCommandPage>()` — a pure type property of a kernel
  type this packet never names, and neither `Option<UiText>` nor `Option<TreeWindow>` introduces a
  destructor. `🧰️framework/🔨️modules/🎠️kernel/` was last written at 2026-09-16 23:33 by a peer.
- `semio-framework-ui-contract --all-features`: the process aborts on
  `action::binding_copy_tests::retained_binding_copy_cancel_and_arena_contention_keep_exact_aliases`
  ("binding copy did not retire exact owners" at `🔗️bindings/📋️copy/🧪️tests/📋️copy/🦀️.rs:26`, then
  `UiValueRetirement requires exact terminal closure` in a `Drop`, then
  `panic in a destructor during cleanup` → SIGABRT), which takes the other 11 reported `FAILED` names
  with it (`builder::tests::*` and `builder::built_retirement::tests::*` each pass in isolation).
  **I bisected this**: with `UI_VALUE_PAGE_ROWS` forced back to `31` and `UI_BUILT_CHILDREN_MAX` at
  128 the test still fails, so the arena re-sizing is not the cause. `UiBindingsCopy` retires
  `UiNodeBindings = UiFixedList<ActionBinding, UI_NODE_BINDINGS(32)>`, which is structurally
  independent of both `BuiltChildren` and `Component`. A second bisect with both constants at their
  pre-change values never got a turn on the shared artifact lock (≥ 40 min queued behind 47–57 peer
  `cargo` processes) and is the one experiment still outstanding.

### 5.3 What is still unrun

- The `♻️retirement/🌳️typed/…` byte fixtures (`🧩️components.json`) were inspected but not re-generated:
  their `bytes` counts only charge fields that are `Some`, and no tree row in them sets `window` or
  `granularity`, so no count should move. This could not be confirmed because the contract test
  binary aborts (§5.2) before reaching those laws.
- `🧫️fixtures/🧪️conformance/…/🌳️tree*` needed no edit: neither `window` nor `granularity` is
  serialized when absent, and the conformance cases set neither.

## 6. Handover notes for the other wave-1 packets

- **P3 (SDK)**: `cargo check -p semio-framework-plugin` is green against this contract.
  `TreeWindow { total, offset }` and `TreeItemBuilder::{window, granularity}` /
  `TreeSectionBuilder::window` are in place with exactly the design's names.
  `UI_BUILT_CHILDREN_MAX` is 128, so `slice.len = min(req.rows, UI_BUILT_CHILDREN_MAX, total − offset)`
  now really admits a full viewport.
- **P4b (Interpreter + ShellHost)**: put the flattened requests on the refresh view state as
  `treeWindows: ViewTreeWindowRequest[]` and `treeViewportRows: number`; both are admitted by
  `parseResolvedPluginViewState` and minted as exact integer carriers by
  `viewContextWithIntegerCarriers` (`🛂️manifest/🟦️.ts`). The cap is **128 requests**, `bodyKey`/
  `nodeKey` are `Identifier`-shaped (1–256 chars, no control characters).
- **P5 (wgpu + reconcile size law)**: see §5.1. `size_of::<Component>() = 3096`,
  `size_of::<UiPatchOp>() = 6416`, `size_of::<ExistingComponentComparison>() = 2256`,
  `size_of::<crate::TreeNode>() = 6456`.
