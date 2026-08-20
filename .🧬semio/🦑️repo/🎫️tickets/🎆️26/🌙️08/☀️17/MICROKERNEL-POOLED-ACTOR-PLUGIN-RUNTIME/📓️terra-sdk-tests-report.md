# terra-sdk-tests — packet report

Executor `terra-sdk-tests`. Goal: continue restoring `semio-framework-plugin`'s test suite from the
536-error residue `test-attr-restore` left (`📓️terra-test-attr-restore-report.md`), test code only,
no production-behaviour changes. **Read this whole report before trusting any single number below —
the crate was hit by a large, unrelated, live concurrent edit partway through this packet (see
"BLOCKING cross-packet finding" below), so the honest headline is two numbers: what I fixed
(measured clean), and what broke under me afterward (not mine, still present at time of writing).**

## Bottom line

| checkpoint | `--all-targets` errors (excl. the pre-existing 🩹️patches file, see below) | E0382 / E0728 / raw parse errors |
|---|---:|---|
| Start of this packet (re-measured) | 552 total / **529** in-scope | 0 / 0 / 0 |
| After my hand fixes + `strip-repeated-await-prefix.py` (clean checkpoint, before the live-peer churn below started) | **378** | 0 / 0 / 0 |
| Current, at time of writing (after an unrelated live peer's in-flight migration started breaking the crate — see below) | **637** (651 incl. patches file) | 0 / 0 / 0 |

**My own contribution, measured cleanly: 529 → 378, a 151-error reduction (28.5%), zero new E0382 /
E0728 / parse errors / mode-2 corruption at every checkpoint.** The subsequent rise to 637 happened
in a file I do not own the whole of, from edits I did not make — detailed with evidence below.
`cargo test -p semio-framework-plugin --lib` still cannot run (test code doesn't compile) — neither at
my clean checkpoint (378 production+test errors combined were still too many to reach test execution)
nor now.

## What I fixed (test code, `#[cfg(test)]` only, one disclosed exception)

All in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` unless noted.

1. **`TxnApp` transaction-testkit cluster (8 test functions, ~5860-6660)**: `let mut app =
   new_app::<TxnApp>();` was missing `.await` (a constructor call left un-awaited by the original
   codemod). Downstream code compensated inconsistently — some sites already had `app.await.method()`
   (itself illegal to repeat, see below), others called the now-async trait methods
   (`transaction_prepare`/`transaction_commit`/`transaction_undo`/`transaction_redo`/
   `ingest_operations`/`take_pending_transaction_proposal`) with no `.await` at all. Fixed by awaiting
   the declaration once and adding `.await` at every genuine call site. One assertion
   (`assert_transaction_rollback_leaves_state_untouched(&mut app, ...)`) had **no `.await` on the whole
   statement** — a genuine R13 dropped-future bug (the assertion's own body never ran); fixed.
2. **`EditorApp`/`ViewerApp` testkit cluster (4 test functions, ~6960-7045)**: same shape —
   `new_app::<EditorApp<...>>()`/`new_viewer::<...>()` missing `.await`, `app_id()`/
   `handle_action_invocation`/`handle_action`/`import_media` (all async trait methods) called without
   `.await`.
3. **`artifact_definition_contract_tests` module (~4320-4460)** — this module contains 3 of the
   ticket's own documented "5 known failures BY NAME" (`plural_definition_carries_every_artifact_
   capability_without_a_dispatch_edit`, `registry_rejects_duplicate_schema_dialect_codec_mime_and_
   extension_claims_atomically`, `identities_and_locales_are_explicit_and_conflicts_do_not_overwrite`):
   the local `identity()`/`capability()` test helpers were called without `.await`; downstream
   `ArtifactDefinitionRegistry::new()`/`register()`/`len()` (all async) were chained through a
   repeated-`.await`-on-the-receiver pattern that only works when the call it prefixes has no `.await`
   of its own (it does, here) — fixed all three functions' `.await` placement by hand.
   `ArtifactIdentityNamespace::{schema,dialect,codec,mime,extension}()` (each async, each with a
   **distinct opaque `impl Future` type**) were collected into one array with no `.await` at all
   (`error[E0308]: expected future, found a different future` — cannot mix distinct opaque future
   types in one array); fixed.
4. **`artifact_contribution_tests` module (~4323-4376)**: `descriptor_with_mutation(...)` (async) called
   without `.await` at 3 sites feeding `std::slice::from_ref(&descriptor)`. Separately,
   `owner_roster_provider` was left `async fn` by the blind codemod even though its only consumer,
   `commit_owner_mutation_roster`, takes `&[fn() -> (&'static str, &'static [SemanticDescriptor])]` — a
   **plain sync fn-pointer slot** (E4 per R2). Reverted to sync and tagged
   `// 🚫️async: E4 fn-pointer slot ... see R2/R9`; the function body has zero suspension points
   (returns a `const` slice), so this is a clean E4 case, not an R9 shortcut.
5. **`TestApp::context_menu` fixture (~18064-18082)**: `Menu::when(condition, |m| m.command(...))` —
   `when`'s closure parameter is `impl FnOnce(Self) -> Self`, a genuinely **sync** signature (it is
   public, documented API other plugins in the fleet use — `Menu::action`/`command` are legitimately
   async because they await `AppActionRegistry::get`/`get_command`, real lookups, not I/O-free R9
   candidates). An async closure can't satisfy a sync `FnOnce`, so `.when(cond, |m| m.command(..))`
   inside `.await` chains produced `error[E0308]: expected Menu<'_>, found future` — R10 residue class 1
   ("`.await` inside a sync closure"). Rewrote the fixture to two explicit `if` blocks with real
   `.await`s instead of touching `Menu::when`'s public signature (out of my scope and used correctly
   elsewhere in the doc-comment-documented pattern).
6. **`test_app_surface_id()`** (async, 5 real call sites at lines ~18091/18110/18139/18191/19164) was
   never `.await`ed anywhere — fixed all 5 (one comment mention at 17905 left untouched, it is prose).
7. **433 structurally-scoped edits via a new tool, `strip-repeated-await-prefix.py`** (below) — the
   dominant fix by volume: 254 sites on the `app` identifier alone, plus `definition`(44)/`history`(39)/
   `body`(22)/`item`(7)/`request`(6)/`reads`(6)/`forward`(6)/`source`(5)/`reverse`(4)/`registry`(4)/
   `receiver`(4)/`store`(3)/`plugin`(3)/`output`(3)/`after_undo`(3)/`after_redo`(3)/`after`(3)/
   `restored`(2)/`read`(2)/`items`(2)/`diagnostics`(2)/`contribution`(2)/`builder`(2)/`action`(1) across
   dozens of test functions spanning the whole file (lines ~1290 through ~21270).
8. **One disclosed production-code fix**, matching the precedent already recorded in
   `📓️terra-test-attr-restore-report.md` ("Production (non-test) code ... a genuine production-code gap
   this packet's audit surfaced"): the `macro_rules!` template generating `impl ::protocol::OpText for
   $Name` (~line 9265-9420, instantiated at least twice in the file for different variant-count arms)
   had `let body = ::dsl::print(&record, &spec_fn(), ::dsl::JoinMode::Inline); if body.await.is_empty()`
   — the exact same repeated-await-prefix shape, caught by tool #7 above and fixed identically
   (`.await` at the `print(...)` call, bare `body.is_empty()` after). This macro's body is untyped
   until instantiated, so it never surfaced under `--lib`'s green baseline; it does under
   `--all-targets` because a `#[cfg(test)]` module invokes it. Disclosed explicitly per this packet's
   own rule against silently touching anything outside `#[cfg(test)]`.

## New tool: `strip-repeated-await-prefix.py` (ticket folder) — an 8th `insert-await.py`-family defect

R16 documented `fix-repeated-await.py` for exactly this residue shape, but that tool's fix (rewrite
`IDENT.await.METHOD(...)` → `IDENT.METHOD(...).await`) was built against `db-dedyn`'s case, where the
downstream methods are **sync** — appending `.await` after the call is correct there. In
`semio-framework-plugin`'s test module the downstream methods (`dispatch_typed`, `snapshot`,
`transaction_commit`, ...) are themselves **async trait methods**, and a large fraction of the
`IDENT.await.METHOD(...)` sites already carry their own correct trailing `.await`
(`app.await.dispatch_typed(...).await.expect(...)`). Running `fix-repeated-await.py` unmodified here
would silently double-await every one of those (`app.dispatch_typed(...).await.await.expect(...)`) —
a corruption class not yet named in R16/R20 because it lives in a different tool.

Verified this by isolated `rustc` reproduction before trusting either tool (`x.await` on an
un-awaited local **always** moves it — confirmed E0382 on a second `x.await` in a minimal repro — but
the reason none of this crate's ~300 affected sites showed E0382 is that MIR borrowck's "use of moved
value" diagnostic is suppressed for a function that already has a type error elsewhere, which nearly
every one of these functions did; confirmed with a second minimal repro mixing one type error with one
repeated-await. **This is a new, undocumented corollary to R17's "a red crate cannot report X"
principle, extended to borrowck diagnostics, not just dropped-future warnings — worth folding into R17
proper.**

`strip-repeated-await-prefix.py`'s fix is deliberately narrower than `fix-repeated-await.py`: it awaits
the declaration once and strips the now-redundant `IDENT.await.` prefix at every later use, but **never
inserts a new trailing `.await`** — whether the call's own result needs one is left to a subsequent
`insert-await.py` pass (clean single-candidate diagnoses once the receiver type is no longer opaque).
Same structural (brace/paren-matched, same-identifier-same-function) scoping discipline as the original
tool — per R10, this is span/structure-keyed, not name-keyed guessing.

**Safety checks run before and after `--apply`** (single file, `--file` not a directory sweep, given
this file's two prior whole-file-corruption incidents):
- `wc -l` before (21282) / after (21282) — unchanged, no line loss/duplication.
- Forced recompile: 0 `E0382`, 0 `E0728`, 0 raw/parse-level errors, both immediately after and at every
  subsequent checkpoint.
- Cross-checked all 433 edit sites against a `#[cfg(test)]`-block detector; 4 landed inside a
  `macro_rules!` template outside any `#[cfg(test)]` wrapper — investigated by hand, confirmed as
  finding #8 above (a genuine bug, not scope creep), not reverted.
- `grep -c '\.await,'`/`'\.await }'` before/after: 217→224 / 35→35 — both counts are pre-existing R18
  false-positive noise (the crate was not green before or after this step, so R18's "self-revealing
  only when compiled" caveat applies; no new hits traced to my edits by manual spot-check).

Left in the ticket folder, documents its own defect class in its own docstring per R10/R16.

## Net effect of items 1-8 (clean checkpoint, before the live-peer churn below)

`cargo check -p semio-framework-plugin --all-targets` (excluding the pre-existing, not-mine
🩹️patches file — see next section): **529 → 378** in-scope errors. Breakdown at 378:
`E0277` 158, `E0308` 68, `E0369` 60, `E0609` 55, `E0599` 38, `E0283` 18, `E0600` 13, `E0432` 6
(2 are the pre-existing `__semio_dispatch_PluginApp` residue from `sdk-final`, out of scope per that
finding; 4 are `ui_contract`/`ui_runtime` — the live peer's edit landing, see below), `E0608` 3, `E0659` 1.

## 🚨 BLOCKING cross-packet finding — a live peer is mid-migrating this crate's UI dependency, and it is currently broken

Not caused by this packet. Evidence, in order discovered:

1. **`🩹️patches/🦀️component.rs`** (a file inside my nominal `path_scope` but NOT touched by me —
   production code, and its own header comment names it as belonging to a **different** ticket,
   `26/08/20 SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY` / packet slug `sdk-flip`) carried 14
   pre-existing `E0433`/`E0432` errors (`ui_contract`/`ui_runtime` unresolved) from before I started —
   excluded from every count above, unchanged throughout.
2. Partway through this packet, the SAME migration started landing in files I *am* actively editing:
   `🔌️plugin/🦀️component.rs` gained `use semio_framework_ui_contract::*; use semio_framework_ui_contract
   as ui; use semio_framework_ui_runtime::{...};` at its top (~line 189-191), replacing `ui_wgpu`'s glob
   import — confirmed via `git diff` timestamps and by the fact these exact 3 lines were absent from my
   very first baseline scan and present in every scan afterward.
3. The crate's own `📦️packages/🦀️rust/Cargo.toml` was mid-edited: `serde = {...}` became
   `semio_framework_serde = { version = "1.0.219", features = ["derive"] }` — **missing
   `package = "serde"`** — so any `cargo check` without `--offline` tried to resolve a nonexistent
   crate `semio_framework_serde` from crates.io and failed before reaching rustc at all. Worked around
   with `--offline` for the rest of this packet (`Cargo.lock` still has the old `serde` entry).
4. The rename cascade is real and unfinished: `Label`/`LocalizedLabel`/`SurfaceKind`/`UiNode`/
   `UiPresence`/`UiControlNode`/`UiTreeItemNode`/`ActionDescriptor`/`ComponentTree: Serialize` etc. do
   not exist under their old names/shapes in `semio_framework_ui_contract`, per that module's own doc
   comment ("`Ui*Node`/`UiPresence`/`UiControlNode`/`ActionDescriptor` have no drop-in replacement of
   the same name ... variant-by-variant mechanical translation" — not yet done everywhere it's used).
   **This broke `semio-framework-plugin --lib` itself** (production code, previously the ticket's
   documented GREEN baseline) — now **175 errors** — and wasip2 `component-guest` — now **179 errors**.
   `--all-targets` climbed from my clean 378 to **651** (measured twice a few minutes apart, still
   rising in between — genuinely live, not a one-time jump).
5. **`semio-framework-plugin-host --lib`** — a *different* crate, part of this packet's required
   regression baseline — cannot currently even be *checked*: it reproducibly triggers a **rustc
   internal compiler error** (SIGABRT) while compiling `semio_framework`'s own `glue.rs` with **both**
   `ui_wgpu` and `semio_framework_ui_contract` linked as externs simultaneously:
   `panicked at rustc_metadata/.../cstore_impl.rs:222` while `computing trait definition for
   protocol::mutation::MutationDiff::apply::{opaque#0}`. Reproduced twice; survives
   `cargo clean -p semio-framework -p semio-framework-ui -p semio-framework-ui-contract`; confirmed
   as a genuine compiler crash (not sccache flakiness) by bypassing the wrapper with `RUSTC_WRAPPER=`
   and getting the identical panic directly from `rustc`. This is squarely `semio-framework`'s own
   root crate (`🧰️framework/📦️packages/🦀️rust/📦️glue.rs`), outside both my `path_scope` and
   `sdk-flip`'s apparent one — needs its own escalation.

**Regression baselines re-verified, unaffected (different dependency chain)**:
`cargo test -p semio-framework-os-kernel --lib` → **779 passed / 0 failed** (unchanged).
`cargo test -p semio-framework-os-kernel-db --lib` → **424 passed / 0 failed** (unchanged).

**Not re-verifiable right now, blocked by the above, not by this packet's own work**:
`cargo test -p semio-framework-plugin --lib` (test code has never compiled this packet, and the crate's
own production code no longer does either, mid-migration); `cargo test -p semio-framework-plugin-host
--lib` (rustc ICE); `semio-framework-plugin --lib --all-features`; wasip2 `component-guest`.

## R17 corollary discovered this packet (recommend folding into the ticket's binding rules)

**A function tainted by ANY type error suppresses its own borrowck diagnostics, including
`E0382 use of moved value`.** This means a repeated-`.await`-on-the-same-binding bug (R16 mode 1) can
be present in a function for an arbitrarily long time with **zero visible symptom of that specific
defect** as long as something else in the function fails to type-check first — the function only
"confesses" E0382 once every other error in it is independently fixed. A census for R16 mode-1 taken
while other errors remain in the same function is therefore as unreliable as R17's dropped-future
census taken on a red crate — same mechanism, one level down (function-local rather than crate-wide).
Concretely: `strip-repeated-await-prefix.py`'s 433 edits were needed **precisely because** none of them
had ever shown up as E0382 despite the pattern being present since the original codemod — every one of
those ~300 affected functions had at least one other, unrelated missing-`.await` error masking it.

## What's left for `semio-framework-plugin` test code (measured at my clean 378 checkpoint, before the churn above)

Genuinely unmeasurable right now without re-doing this whole packet's clean-checkpoint work, because
every subsequent `cargo check` reflects the live peer's still-moving migration on top of mine. At the
378 checkpoint the remaining errors were the same shape family already documented by the prior packet's
report (per-call-site missing `.await` and R9 candidates needing individual judgment, e.g. the
`7164-7296`/`13561-13566`/`9474-9555`/`5855-5858` clusters) — none investigated yet by this packet
beyond the items above. `E0283` (18, "type annotations needed") is a new-to-this-packet class not seen
in the prior report's breakdown — worth a first look by whoever picks this back up, likely a knock-on
effect of one of the `VcsArtifactApp<TestApp, _>` sites needing an explicit generic parameter once its
surrounding `.await`s are correct.

## Files touched this session

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (all of items 1-8 above; also the
  macro-template fix, item 8, which sits outside `#[cfg(test)]`)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/
  strip-repeated-await-prefix.py` (new tool, this packet)
- Scratch/logs, in the ticket folder per the standing rule (`terra-sdktests-*.txt`):
  `terra-sdktests-baseline-alltargets.txt` (start-of-packet full error list),
  `terra-sdktests-clean-checkpoint-alltargets.txt` (the 378-error clean checkpoint, full list),
  `terra-sdktests-current-churned-alltargets.txt` (current, post-live-peer-churn full list),
  `terra-sdktests-strip-repeated-await-report.txt` (all 433 edit sites from
  `strip-repeated-await-prefix.py`, by file:line), `terra-sdktests-pluginhost-ice.txt` (the reproduced
  rustc ICE, full output). Build artifacts (`CARGO_TARGET_DIR=target-host`) and intermediate
  JSON/pickle diagnostics stayed in the session scratchpad, never the ticket folder, per the separate
  `CARGO_TARGET_DIR` rule; a pre-sweep snapshot of `component.rs`
  (`component_before_strip1.rs`, for recovery) is also in the scratchpad.

## Explicit escalation asks for the coordinator

1. Whoever owns `sdk-flip` (`SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`) needs to either finish landing
   the `ui_wgpu` → `semio-framework-ui-contract`/`ui-runtime` migration in one atomic step (per rule 25:
   redirect before start or let it finish, never leave it half-landed across other packets) or pause it
   — right now it has taken the ticket's documented-GREEN `semio-framework-plugin --lib` baseline red.
2. The `Cargo.toml` typo (`semio_framework_serde` missing `package = "serde"`) should be fixed by
   whoever is landing that migration — it currently makes any non-`--offline` cargo invocation against
   this crate fail before reaching rustc.
3. The rustc ICE in `semio_framework`'s `glue.rs` (`protocol::mutation::MutationDiff::apply`'s opaque
   type, both `ui_wgpu` and `semio_framework_ui_contract` linked) blocks `semio-framework-plugin-host
   --lib` — needs its own look; it may resolve itself once (1) lands cleanly (only one of the two UI
   crates linked), but is worth confirming rather than assuming.
