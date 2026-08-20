# R10 — Test-Target Repair: `semio-framework-os-kernel` + `semio-framework-ui`

## Scope

Repair the two broken TEST targets (lib compiles clean on both already):
1. `semio-framework-os-kernel` (lib test) — reported 16 errors.
2. `semio-framework-ui` (lib test) — reported 84 errors.

Ownership boundary: test targets/fixtures of these two crates only. Did not touch
`semio-framework-machine` (R8) or the render-backend platform gates (R9/R5/R6), both concurrent
sibling packets, nor `semio-framework-ui`'s lib sources that R1 already repaired.

## `semio-framework-os-kernel` — attribution verified

**Reproduction**: default features never unify `sync`/`ureq`/`typegen` (nobody else in the
workspace was checked with those on when I first ran `cargo check --lib --tests` — 0 errors).
Grepping every workspace `Cargo.toml` for `semio-framework-os-kernel = {...features...}` found the
real unification set: `sync` (from `os/🔨️modules/🌉️mcp`), `sync,ureq` (from
`os/🔨️modules/📺️renderer/…/🎯️targets/🧊️wgpu`), and `typegen` (from `🧰️framework/📦️packages/🦀️rust`'s
own `typegen` feature). Exact reproduction:

```
cargo check -p semio-framework-os-kernel --all-targets --features sync,ureq,typegen
```

This reproduces **16 previous errors** verbatim.

**Bug class confirmed**: 100% async bug class (a) — stale `.await` on now-sync macro-generated /
already-de-asynced callees, all 16 in `#[cfg(test)] mod tests` of
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs` (12 call sites; the `.await`
on `crate::os_dsl::print(...)` at line 2304 alone produced 5 cascaded diagnostics). Every callee
(`crate::os_dsl::parse`, `crate::os_dsl::print`, the `dsl_derive`-generated `__dsl_from_record`/
`__dsl_to_record`, and the `DslVariants` trait's `to_named_record`/`from_named_record`) is `pub fn`,
not `pub async fn`, in current production source — verified by reading
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️component.rs` and
`🗣️dsl/✨️derive/🦀️component.rs` directly, and by confirming every other production call site of
`DslVariants::to_named_record`/`from_named_record` elsewhere in the crate already calls them without
`.await`. So P1d's "pre-existing" note was accurate (present verbatim at baseline, untouched by any
other packet) but its "unrelated" framing undersold it — this is the exact same async-fn-called-
without-`.await` class R1 fixed in `semio-framework-ui`'s lib, just left over in this crate's test
fixtures. Decision-rule check: none of the 12 call sites needed a genuine suspension (`.await`)
re-added — all were "callee never suspends → drop the stale `.await`".

**Fix**: removed the 12 stale `.await`s (kept `.await` where the callee genuinely is async, e.g.
`pack_rt::encode_document(...).await?`, `ByteReader::read_u8().await`, `envelope_id().await`).
Result: **16 → 0** errors, confirmed with `--all-targets` and with `--all-targets --features
sync,ureq,typegen,worker` (0).

**Additional (non-blocking) feature-gated path found, left untouched**: `dsl-fixture-sweep-full` is
a crate-local feature declared but never referenced by any other workspace `Cargo.toml`, so
`cargo check --workspace --all-targets` never turns it on and it does not block the exit gate. With
it explicitly enabled the crate still shows 72 errors in
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs`: 15×`E0433` "cannot
find module/crate X" (plugin-name lookups, likely expecting on-disk example-plugin dirs not present
in this checkout), plus a real async-class handful (`impl Future<...> is not an iterator`, `no
method named len/iter found for opaque type impl Future<...>` — `collect_files`/`repo_root`/
`find_example_asset` etc. are still `pub async fn` with genuinely zero suspension). Out of scope for
this packet (not part of the reported 16, not reachable from the exit-gate command) — flagged for a
follow-up ticket rather than fixed speculatively.

## `semio-framework-ui` — attribution verified, split as requested

**Reproduction** (R1's own note applies — `--lib` alone undershoots):
```
cargo check -p semio-framework-ui --features wgpu,wgpu-engine,tui,tui-terminal --all-targets
```
Reproduces **84 previous errors** verbatim, matching R1's own probe.

**Split, verified by grouping every error's primary message**:
- **39 / 84** — `label_impl::Label: From<&str>` unsatisfied (`E0277`) — a raw `&str`/`String` literal
  passed where `impl Into<Label>` is required.
- **45 / 84** — `mismatched types` (`E0308`), 100% `String` vs `Label` / `Option<String>` vs
  `Option<Label>` — i.e. the *other face* of the same design gate: test-only `UiNode` → `WidgetNode`
  equivalence-harness conversion functions (`to_widget_node`, `control_to_widget`,
  `control_to_widget_node`, `tree_item_to_widget`, `tree_section_to_widget`,
  `tree_action_to_widget` in `engine.rs`) whose `WidgetNode`-family target types still use plain
  `String` for label fields while the `UiNode`-family source types were migrated to `Label`.
- **0 / 84** mention `Future`/`await` anywhere in the diagnostic text (checked explicitly, per the
  measurement caveat about confirming the bug class before assuming). **Not** the async-fn class.

So both halves of the 84 are the SAME deliberate `Label: From<&str>` gate (ticket
`26/08/03/COMPILE-TIME-CHECKED-UI-LABELS-ACROSS-LOCALE-TERMINOLOGY-AND-BRAND`,
`🎯️targets/🧊️wgpu/🦀️label.rs`'s own "No `From<&str>`/`From<String>` on purpose" doc comment) —
R1's characterization was correct; 84 is simply the transitive fallout of that one gate across 9
test-fixture files, not one design gate producing one error each.

**Fix (all in `#[cfg(test)] mod tests` blocks, no lib/production files touched)**: updated the test
fixtures to satisfy the gate rather than weakening it —
- Raw string literal → `Label::data("…")` (test data is exactly the gate's sanctioned "genuine
  runtime data" case) at every `UiNode`/`UiTextNode`/`UiButtonNode`/`UiSectionNode`/`UiToggleNode`/
  `UiSelectItem`/`UiTreeItemNode::base` construction site that had one.
- `Label` value flowing into a `String`-typed `WidgetNode`-family field → `.to_string()` (via
  `Label`'s `Display` impl) at every one of the 45 mismatch sites, applying rustc's own
  `help: try using a conversion method` suggested diff verbatim in the 44 cases it offered one, and
  the same pattern (`.clone().map(|l| l.to_string())` for `Option<Label>` → `Option<String>` fields)
  in the ~13 cases rustc's conversion-method suggester doesn't reach through `Option`.
- One comparison fixed to compare data, not types: `reconcile.rs`'s
  `assert_eq!(button.label, "Alpha")` → `assert_eq!(button.label.as_str(), "Alpha")`.
- Added the missing `use crate::wgpu::Label;` to 5 files (`tree.rs`, `cursor.rs`, `flex.rs`,
  `events.rs`, `scene_slots.rs`) whose test modules never needed the type in scope before (`engine.rs`/
  `paint.rs`/`reconcile.rs` already had it).

Files touched (all under `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/`):
`🦀️engine.rs`, `🦀️paint.rs`, `🦀️reconcile.rs`, `🦀️events.rs`, `🦀️flex.rs`, `🦀️scene_slots.rs`,
`🦀️cursor.rs`, `🦀️tree.rs`.

Result: **84 → 0** errors (`cargo check -p semio-framework-ui --features
wgpu,wgpu-engine,tui,tui-terminal --all-targets`).

**Deliberate gate NOT weakened**: `label_impl::Label`'s missing `From<&str>`/`From<String>` impls
were left exactly as-is — every fix went into the test-side call, never into `🦀️label.rs`. No
gate-widening `impl From<&str> for Label` was added anywhere.

## Verification actually run

```
cargo check -p semio-framework-os-kernel --all-targets --features sync,ureq,typegen        # 16 -> 0
cargo check -p semio-framework-os-kernel --all-targets --features sync,ureq,typegen,worker  # 0
cargo check -p semio-framework-os-kernel --lib --tests                                      # 0 (default features, unchanged)
cargo clippy -p semio-framework-os-kernel --all-targets --features sync,ureq,typegen         # 0 errors, pre-existing style warnings only
cargo test -p semio-framework-os-kernel --lib --features sync,ureq,typegen                   # 796 passed, 4 failed (see below)
cargo test -p semio-framework-os-kernel --lib --features sync,ureq,typegen --release         # 796 passed, 4 failed (same 4)

cargo check -p semio-framework-ui --features wgpu,wgpu-engine,tui,tui-terminal --all-targets  # 84 -> 0
cargo check -p semio-framework-ui --features wgpu,wgpu-engine,tui,tui-terminal --lib          # 0 (unchanged, R1's target)
cargo clippy -p semio-framework-ui --features wgpu,wgpu-engine,tui,tui-terminal --all-targets # 0 errors, pre-existing style warnings only
cargo test -p semio-framework-ui --features wgpu,wgpu-engine,tui,tui-terminal --lib            # 344 passed, 1 failed (see below)
cargo test -p semio-framework-ui --features wgpu,wgpu-engine,tui,tui-terminal --lib --release  # 344 passed, 1 failed (same)

cargo check -p semio-framework-os-kernel --all-targets                     # 0 (default features)
cargo check -p semio-framework-ui --all-targets                            # 0 (default features)
bun ./📜️script.ts verify dependencies                                     # clean, 238 = 238
rustfmt --check --config-path ./rustfmt.toml <each file I edited>          # only pre-existing drift remains (see below)
```

## Test failures found — pre-existing runtime bugs, NOT the async/Label classes, left unfixed

Both targets now compile; running the suites for the first time (they never compiled with these
features before) surfaced 5 runtime failures, none touched by my edits and none matching bug class
(a) or (b):

**`semio-framework-os-kernel`, 4 failures (both debug and release)**:
- `os_store::sync::tests::folder_text_storage_round_trips_dsl_and_appends_ops` and
  `...::folder_text_storage_round_trips_pack` — both panic "ops text has no inverse record for edit
  X". Root cause: the test calls `storage.append_ops(id, id, &print_edit_lines(edit).await?)`, but
  `print_edit_lines`'s own doc comment says it prints ONLY the edit header + forward-op lines — "Its
  matching inverse and authoritative metadata records are emitted by `print_ops_log` immediately
  after it" (a separate, non-exposed helper). The parser (`parse_document_text`,
  `🏪️store/🦀️component.rs:3416`) unconditionally requires an inverse record per edit. This is a
  pre-existing contract mismatch between the `.ops`-text-mirror writer and reader, orthogonal to
  async/Label — it needs a domain decision about the ops-text hot-append format, not a mechanical
  fix, so left unfixed and flagged.
- `os_store::sync::tests::actor_tests::fixtures_replay_matches_expected_events` — panics "expected
  fixtures" at a path built as `env!("CARGO_MANIFEST_DIR").join("🧫️fixtures")`
  (`…/📦️packages/🦀️rust/🧫️fixtures`, which does not exist). The real fixtures directory is two
  levels up at `🧰️framework/🛍️products/💻️os/🧫️fixtures`. Pre-existing path-construction bug,
  unrelated to async/Label.
- `os_store::sync::tests::actor_tests::folder_external_edit_delivers_remote_operations` — asserts
  the post-ingest snapshot `n == 42` (the forward mutation's value) but gets `1` (the inverse's
  value). Encode/decode round-trip for `DemoMutation` (which I touched) is internally symmetric and
  unrelated to which of forward/inverse gets applied — that selection happens in production
  actor/ingest code I did not touch. Pre-existing logic bug, orthogonal to async/Label.

**`semio-framework-ui`, 1 failure (both debug and release)**:
- `wgpu::component::layout::layout_wire_format_tests::action_descriptor_and_style_spec_serialize_to_golden_json`
  — golden JSON mismatch: `"args":42.0` vs expected `"args":42` (a numeric-value JSON float-vs-int
  serialization drift in `serde_json`'s float formatting for `ActionDescriptor.args`). Nothing to do
  with `Label`; that field is unrelated. Pre-existing, unrelated.

All 5 are newly-visible (never ran before — the crates didn't compile with these feature sets until
this packet), not newly-introduced: none touch code paths my edits changed (my changes were either
inert `.await`-token removal on already-sync callees, or `Label::data(...)`/`.to_string()` wrapping
in test literals that never reach the failing assertions above).

## Formatting

`rustfmt --check --config-path ./rustfmt.toml` on each edited file, individually (not the whole
tree, per R1's precedent — `cargo fmt --check --` on explicit paths still sweeps unrelated files on
this tree). Two of my own edits (adding `use crate::wgpu::Label;` to `events.rs` and
`scene_slots.rs`) landed in the wrong sorted position and were re-ordered to match rustfmt's import
sort. All other diffs `rustfmt --check` still reports on these files are pre-existing drift I did
not introduce (struct-literal/line-wrap formatting in code I never touched) — left as-is, matching
R1's same finding and same choice not to take a wholesale-reformat diff on a live, concurrently
edited tree.

## Cross-boundary observations (not touched)

- `git status` showed `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs` and
  `…/🛢️db/🔒️security/🦀️component.rs` already staged as modified by something other than this
  session — not touched here, flagging per the "expect concurrent sibling churn" guidance rather
  than assuming it's mine.
- `semio-framework-os-kernel`'s `dsl-fixture-sweep-full` feature (72 errors when force-enabled, see
  above) and the ops-text-mirror inverse/metadata contract mismatch (2 of the 4 kernel test
  failures) are both real, both outside R10's async-fn/Label-gate remit, and both worth their own
  follow-up tickets.
- Did not touch `semio-framework-machine` (R8) or the render-backend platform-gate crates (R5/R6/R9),
  per the stated ownership boundary.
