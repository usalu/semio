# 🚦 terra / framework-last-gate — report

## Headline result

**`cargo check -p semio-framework --lib` → EXIT 0.** The 63-fleet-crate blocker is cleared.

```
$ CARGO_TARGET_DIR=.../scratchpad/target-lastgate cargo check -p semio-framework --lib
   ...
warning: `semio-framework` (lib) generated 28 warnings
    Finished `dev` profile [unoptimized] target(s) in 1.93s
EXIT: 0
```
Full log: `terra-lastgate-FINAL-lib-check.txt`.

**`cargo check -p semio-framework --all-targets` → EXIT 101, but 0 of the 29 remaining errors trace to
any file I own.** All 29 are in `🚪️io/🦀️component.rs` (24, explicitly excluded — "NOT yours") and
`🔨️modules/🕹️interaction/🦀️component.rs` (5, not in my owned-paths list either — a different packet's
crate). Verified by parsing every diagnostic's own `--> file:line:col` arrow line (see methodology
note below), not by naive per-diagnostic-object primary-span grouping, which undercounts when the
primary span is attributed to a std macro definition (`assert_eq!`) rather than the call site.
Full log: `terra-lastgate-alltargets-final.txt` / `.json`.

**`cargo check -p semio-framework-os-kernel --lib` → EXIT 0, unchanged.** Re-verified last, after every
other change in this session:
```
EXIT: 0
warning: `semio-framework-os-kernel` (lib) generated 417 warnings (...)
    Finished `dev` profile [unoptimized] target(s) in 0.21s
```
Full log: `terra-lastgate-FINAL-oskernel-check.txt`. No regression.

**`cargo check -p semio-framework-plugin --lib` (the headline downstream gate) → EXIT 101, 719 errors,
NOT yet green.** Verified **zero** of the 719 errors have their `-->` arrow pointing into any file I
own (manifest/workflow/platform/action-bus/kernel) — every single one is in `🔌️plugin`, `🏪️store`,
`🗣️dsl`, `📇️directory`, `🎒️pack`, `📡️spr`, `🖱️ui/wgpu`, all explicitly outside my ownership. This is a
large, separate crate with its own independent backlog (was 890 errors at the start of this session,
719 now — other packets are actively working it down concurrently). Since it is not EXIT 0, per the
packet's own acceptance conditional I did **not** run `cargo test -p semio-framework-plugin --lib` or
`cargo check -p semio-s-plugin-note --lib` (both would trivially fail to build, telling us nothing new).
Full log: `terra-lastgate-FINAL-plugin-check.txt` / `.json`.

## What I actually own and touched

1. `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` — **0 errors, `--lib` and `--all-targets`.**
2. `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs` — **0 errors, both gates.**
3. `🧰️framework/🔨️modules/🖥️platform/🦀️component.rs` — **0 errors, both gates.**
4. `🧰️framework/🔨️modules/🎯️action-bus/🦀️component.rs` — **0 errors, both gates.**
5. `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` — one surgical E4 fix (see below); compiler
   attributed the original error here for this crate, so it was in scope under "be surgical."

## Starting state

`cargo check -p semio-framework --lib` measured at hand-off: **exit 101, 199 errors**
(E0599 59 · E0308 45 · E0277 34 · E0600 2 · E0053 2, plus 60 diagnostics the shared tool declined).
Also found, before any of my edits: `🖥️platform` E0107 (`ActionBus` missing generic), and two live
struct-literal-shorthand `.await` corruptions in `🏪️store` that **self-healed while I was inspecting
them** (a concurrent session fixed them between two of my reads — confirmed the file changed under me,
documented, never touched by me; see "Concurrent Workspace Churn" below).

## Fix categories (R2/R9 judgement calls)

**Reverted to sync + tagged (E1/E4 transitive, R9)** — pure computations whose only real consumers are
language-barred (external traits, or `Iterator`/`Option` combinators needing sync closures):
- `manifest`: `Version::{new,parse}`, `VersionReq::{parse,matches,matches_raw}` (feed `TryFrom`/
  `FromStr`/`Deserialize`), `NonEmptyVec<T>`'s whole inherent impl (feeds `Iterator` combinators and
  `TryFrom<Vec<T>>`/`Into<Vec<T>>`), `ActionRef::as_str`, `ToolRef::as_str`, `action_is_panel_eligible`
  (all feed `Iterator::find`/`filter` closures), `ActionBus::new` (feeds `Default::default`).
- `🎠️kernel`: `history_entry_count` (E4, serde `default = "…"` fn-pointer slot).
- `manifest`: six serde `default`/`skip_serializing_if` fn-pointer targets
  (`introduction_pointer_button_{left,right}`, `introduction_orbit_default_modifiers`,
  `tutorial_narration_default_rate`, `tutorial_rate_is_default`, `tutorial_camera_up_z`) plus four
  `Shape::Record(fn() -> RecordSpec)` targets in `workflow` (`media_contract_spec`,
  `workflow_media_port_spec`, `workflow_input_spec`, `run_trigger_spec`) and their three pure
  variant-table helpers (E4, same reasoning as `DslField::shape`'s own tag).

**Made async where the trait is first-party and was already so (shape 1 from the brief)**:
`workflow`'s `impl protocol::OpText`/`OpBinary for {WorkflowMutationDsl,WorkflowMutation,RunMutationDsl,
RunMutation}` — the trait (`🔨️modules/📡️replication`, already green) declares async; all four impls were
stale-sync. Ran `asyncify-universal.py --apply` once over the whole `workflow` file (scan showed 0
external-trait hits, so no misfire risk), then iterated `insert-await.py` to fixpoint.

**Real bugs the conversion exposed, fixed by hand** (R10-safe, byte-offset diagnostic-driven, never
name/regex-keyed across the file):
- **Repeated `.await` on an already-moved future** (`X.await` used more than once) — dozens of
  instances across both files' test modules; canonical fix is `let x = expr.await;` once, then plain
  `x` everywhere after.
- **Struct-literal-shorthand `.await` corruption** (`Field { name.await, ... }`, unparseable) — one
  live instance in `workflow` (`graph.await,` inside a `WorkflowSnapshot` literal), fixed the same way
  the packet's own notes described: hoist the `.await` to the constructor, plain `graph,` in the
  literal.
- **Recursive `async fn`** (E0733) — `arg_schema_json_schema`'s `Array` branch and the mutual edge via
  `ActionArgDef::json_schema`, in `manifest`; both fixed with `Box::pin(...).await` at exactly the
  recursive call sites, nothing else.
- **`E0107` `ActionBus` missing generic** — `Platform.action_bus` in `🖥️platform` now
  `ActionBus<NoActionHandlers>`, matching the sibling packet's own documented convention (`action-bus`'s
  doc comment names this exact type for "production callers that never register anything").
- **Dropped futures never polled** (`cargo`'s "unused implementer of `Future`" warning, a real
  functional bug, not cosmetic) — `Platform::notify`/`notify_chrome` calls in `add_app`/
  `set_active_app_id`/`set_panel_visibility`, `apply_arg_format`/`apply_tutorial_ui_change` in
  `manifest`, and the two recursive local `dfs` helpers in `workflow` (`validate_workflow`'s
  cycle-detector and `workflow_topological_node_order`) — the last two are the most serious: without the
  fix, the graph traversal **never actually ran** (the future was constructed and dropped, never
  polled), so validation and topological ordering were silent no-ops.

## Tools

- `insert-await.py` (shared) had a **real bug**, fixed in place: its `AWAIT_CODES` filter checked
  `".await" in repl`, but this nightly's rustc frequently emits the insertion as bare `await.` (dot
  *after*, none before — inserted mid-chain right after an existing `.`), which never matched. Effect:
  **the tool had been silently discarding every candidate of that shape crate-wide** (measured: 0 edits
  applied despite 100+ eligible diagnostics, before the fix; 91+ edits in the very next pass after).
  Fixed with a token regex (`\.?await\.?`) instead of a fixed-position substring check.
- `terra-lastgate-await-binding-fixer.py` (new) — companion to `insert-await.py` for the shape it
  can't cover: rustc gives no `suggested_replacement` at all when the fix is "await the *binding*, not
  the use site" (E0308/E0369/E0600/E0382/E0716 on a variable used many lines after an unawaited call).
  Diagnostic-driven: reads the exact bytes at the error's own span, and only acts when that span is a
  bare `IDENT`/`&IDENT`; then bracket-matches forward from that identifier's nearest preceding
  `let [mut] IDENT = …` to find the statement's own terminating `;` (handles multi-line initializers),
  bounded by the nearest preceding `async fn`/test attribute so it can never cross into another test
  function. First version only handled single-line lets; upgraded mid-session.
- `terra-lastgate-chain-await-fixer.py` (new, then largely superseded) — E0271 chain-mismatch fixer.
  **Caution for future readers**: its first version blindly trusted the "note: calling an async
  function returns a future" span for *every* E0271 diagnostic in one pass, and multiple diagnostics
  from the same root cause produced *stacked* `.await.await.await` corruption on ~9 lines in `manifest`
  before I caught it via a second independent read and hand-repaired every instance (documented inline
  in the file). Left in the ticket folder as a cautionary/reference tool, not for further blind use.
- `terra-lastgate-await-call-sites-whitelist.py` (new) — for the common "same async helper called N
  times across a test module, always missing `.await`" pattern once N got past hand-editing size:
  bracket-matched (not regex) append of `.await` after every *call* of a named function (never a `fn
  NAME(` definition, checked by prefix; never a same-name-suffix collision, checked by word boundary).
  Used only after manually confirming both the function is genuinely async and every occurrence in the
  file is a call site, never a definition.
- `async-test-attr.py` (shared, unmodified) — rewrote 132 `#[test]` → `#[semio_framework_async_macros::
  async_test]` sites across manifest/workflow/platform/action-bus (bare `#[test]` on `async fn` is a
  hard compile error, not something `--lib` ever exercises). Also added the missing
  `semio-framework-async-macros` dev-dependency to `🧰️framework/📦️packages/🦀️rust/Cargo.toml` (the tool
  flagged it as needed; os-kernel's `Cargo.toml` already had it).
- `remove-bad-await.py` (shared, unmodified) — used repeatedly for the inverse case (E0277 "`X` is not
  a future") that shows up right after fixing a binding: once `X` is awaited at its `let`, every later
  `X.await` becomes illegal and needs the `.await` removed, not added.

## A live-corruption near-miss that was not mine to fix

Early in the session `cargo check -p semio-framework --lib` failed on two `.await`-appended-to-
struct-literal-shorthand parse errors in `🏪️store/🦀️component.rs` — the exact shape the packet's brief
warned about. `🏪️store` is explicitly "NOT yours" and its `git status` showed live uncommitted changes
(`MM`, large diff). Rather than touch it, I re-read the same two spans a few minutes later: **both had
already been fixed by the owning session** (confirmed via a second, independently-timed read — the file
changed under me between reads). Documented, not touched, matches the ticket's own "Concurrent
Workspace Churn" guidance.

## Methodology note (for whoever reads the intermediate logs)

Several intermediate `terra-lastgate-*-check*.json` files in this folder were analyzed with a script
that grouped errors by each diagnostic's `is_primary` span. For `assert_eq!`/`matches!`-macro-expansion
diagnostics, rustc's own `is_primary` span often lands inside the **std macro definition**
(`core/src/macros/mod.rs`), not the call site — while the human-rendered `--> file:line:col` arrow
(which is what a person actually reads) correctly points at the call site. This caused several
intermediate "manifest/workflow is now clean" readings to be **wrong** (111 real manifest errors were
hiding under a `core/src/macros/mod.rs` bucket for a while). Caught and corrected by switching to
parsing the rendered arrow line directly; the final counts in this report use that method throughout.
If you resume this ticket, do the same — don't trust `is_primary`-based file grouping against
`--all-targets` output.

## Files touched (all within owned paths, plus the one surgical kernel exception)

- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖥️platform/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎯️action-bus/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs` (one fn, E4 tag)
- `/Users/ueli/Documents/semio/🧰️framework/📦️packages/🦀️rust/Cargo.toml` (added
  `semio-framework-async-macros` dev-dependency)
- `.🧬semio/🦑️repo/🎫️tickets/…/insert-await.py` (shared tool bugfix — token-match instead of fixed
  substring; backward-compatible superset, safe for other packets already relying on it)
- New ticket-folder tools: `terra-lastgate-await-binding-fixer.py`,
  `terra-lastgate-chain-await-fixer.py`, `terra-lastgate-await-call-sites-whitelist.py`
- Scratch/log files: `terra-lastgate-*.txt`, `terra-lastgate-*.json` (all in ticket folder, none
  `.log`)

## Out of scope, flagged for other packets (not touched)

- `🔌️plugin/**` and everything else behind `semio-framework-plugin`'s remaining 719 errors — separate,
  actively-worked crate.
- `🚪️io/🦀️component.rs` — 24 errors, explicitly excluded from my ownership.
- `🔨️modules/🕹️interaction/🦀️component.rs` — 5 errors (`#[test]` on `async fn`, one moved-value
  struct-field pattern identical to what I fixed dozens of times in my own files), never in my owned
  paths or the exclusion list — a different packet's file.

## Acceptance checklist

| # | Check | Result |
|---|---|---|
| 1 | `cargo check -p semio-framework --lib` | **EXIT 0** ✅ (the bar) |
| 2 | `cargo check -p semio-framework-plugin --lib` | EXIT 101, 719 errors, **0 mine** — not green yet |
| 2b | `cargo test`/fleet crate check | skipped per conditional (plugin not green) |
| 3 | `cargo check -p semio-framework --all-targets` | EXIT 101, 29 errors, **0 mine** |
| 4 | `cargo check -p semio-framework-os-kernel --lib` | **EXIT 0**, unchanged, re-verified last |
