# 📓️ terra-io-thunks report

Packet: **io-thunks** — io module erasure tables (`ComposerEntry`/`AsyncComposeFn`/`IoEntry`/
`SubsetValidatorEntry`/`resolve_ready`), plus the 2 stray missing-`.await` errors in
`🖱️ui/🎨️styling` and the `compose-thunk-rewrite.py` deliverable.

Re-verified fresh from disk immediately before writing this report, per the coordinator's incident
notice (bulk revert of a staged async conversion in two OTHER files today, 14:41:32 — **not**
`🚪️io/🦀️component.rs`, confirmed both by the coordinator's own measurement and by my own re-read:
192 `async fn`, 801/801 balanced braces).

## What was fixed

### 1. `🖱️ui/🎨️styling/📦️packages/🦀️rust/📦️glue.rs` — 2 missing `.await`
`rgba8_to_linear`/`linear_to_rgba8` call the local `async fn ch` three times each but only the
compiler-suggested single unambiguous site got flagged per pass. Used the shared tool as instructed:
```
python3 insert-await.py --crate semio-framework --dry-run
python3 insert-await.py --crate semio-framework --apply --scope '🧰️framework' --max-passes 12
```
First pass applied both `.await`s in this file (`[bs:be] -> '.await'` at lines 18/28). Verified:
```
CARGO_TARGET_DIR=.../target-io cargo check -p semio-framework-ui-styling --lib
    Finished `dev` profile [unoptimized] target(s) in 0.24s   — EXIT 0
```

**IMPORTANT — read before trusting the tool's own blast radius**: the same `--apply --scope
'🧰️framework'` run initially touched **314 files / 19618+/19406- lines**, because rustc's
diagnostics for `-p semio-framework` cover its entire dependency graph and the `🧰️framework`
substring matches `🧰️framework/🛍️products/**` and `🦑️repo/**` too — nothing in `📌️important.md`'s
rule 3 authorizes those. I restored the 203 out-of-scope files to `HEAD` (via `git show HEAD:<path>`
→ `Write`, never `git checkout`) and kept only the 111 files legitimately under
`🧰️framework/🔨️modules/**` — exactly what rule 3 pre-authorizes ("any other file inside
`🧰️framework/🔨️modules/` that the await tool must touch to make `semio-framework` compile"). Those
111 are pure `.await` insertions, rustc-verified unambiguous, nothing else.

### 2. The io erasure tables (`🧰️framework/🔨️modules/🚪️io/🦀️component.rs`)
The described defect was real and exactly as briefed: `AsyncComposeFn`/`ComposeFuture` and
`IoEntry.run`/`.sniff` are bare-`fn`-pointer-typed vtable slots; the blind `async fn` codemod broke
every construction site that assigned an `async fn` item into one, plus `resolve_ready`'s own
`RawWakerVTable` helpers (turned `async fn`, which can never satisfy `RawWakerVTable::new`'s bare
`unsafe fn` pointer parameters at all).

**`compose_thunk!` macro** (new, `#[macro_export]`, next to `AsyncComposeFn`): wraps an
`async fn(&[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError>` in a macro-generated
`fn __thunk<'a>(s) -> ComposeFuture<'a> { Box::pin($f(s)) }` — the exact recipe in the brief. Applied
to this file's own `compose_hop1`/`compose_hop2` test doubles (which had ALSO been double-broken:
`async fn compose_hop1(..) -> ComposeFuture<'_>` is the "double future" R1 explicitly bans — fixed by
making the leaf return `Result<ComposedArtifact, ComposeError>` directly and wrapping the
construction site instead).

**`io_run_thunk!`/`io_sniff_thunk!` macros** (new, same file): the `IoEntry`-shaped twins.
`IoEntry.run`/`.sniff` are genuinely non-future-returning (`fn(&IoPayload) -> IoResult<IoPayload>` /
`Option<fn(&IoPayload) -> Confidence>` — never touched by the ticket's `ComposeFuture` design at all,
confirmed by reading the struct definition), so these two macros resolve the wrapped `async fn`
synchronously via `resolve_ready` rather than boxing a future. For this file's own five
`IoEntry`-constructor call sites (`serializer_entry`/`serializer_entry_text`/`deserializer_entry`/
`deserializer_entry_text`/`deserializer_sniff`) I hand-wrote the equivalent sync `fn` + `resolve_ready`
inline rather than invoking the macro, because the `Deserializer::CONFORMANCE` threading needs
`T::deserialize(payload)`'s `?` folded in first — same net effect, tagged `// 🚫️async: E4 fn-pointer
slot` at each site.

**`SubsetValidatorEntry.validate`** (top-level `io` module, NOT `io_mechanism` — a second, separate
erasure table the brief didn't literally name but is the identical class of bug): `subset_validator_entry_of`
assigned `V::validate` (an `async fn` trait method) straight into `validate: fn(&IoPayload) ->
Vec<Diagnostic>`. Fixed with the same inline sync-thunk-via-`resolve_ready` pattern, tagged E4.

**`resolve_ready`**: was `pub async fn` — which defeats its entire purpose (a bare `fn`-pointer
thunk can never itself be `async`, so an async `resolve_ready` can never be called from inside one).
Its two `RawWakerVTable` helpers (`noop`/`clone_raw`) had also been turned `async fn`, which cannot
satisfy `RawWakerVTable::new`'s bare `unsafe fn` pointer parameters at all — not merely "needs
`.await`", genuinely unfixable in place. Replaced the whole hand-rolled raw-waker with
`std::task::Waker::noop()`, matching `poll_ready` in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:550` exactly (confirmed by
reading it). `resolve_ready` is now `pub fn`, tagged `// 🚫️async: E5 executor bridge` (one per
crate, per R2 — verified no other `E5` tag exists in this file). **Verified this is NOT a breaking
signature change**: grepped every existing call site of `resolve_ready` repo-wide — all of them
(`🧰️framework/🔨️modules/🚪️io/🦀️component.rs` itself at 3 sites, and the guest SDK
`🔌️plugin/🦀️component.rs` at 5 sites) already call it **without** `.await`, i.e. they already assumed
the sync signature — my fix makes them correct, not broken.

**`same_io_entry` (bare-path fn-pointer equality) and the whole io_mechanism registry/route/dispatch
mechanics**: these are regular (non-fn-pointer-slot) helpers, so per O1 they keep the literal `async`
keyword — but the codemod left ~30 call sites un-awaited (`E0308`/`E0600`/`E0728`-shaped: calling an
async fn without `.await`, `!future` unary-not, `await` inside a still-sync fn). Fixed by hand
throughout `io_register`/`build_proposed`/`validate_against`/`walk_routes` (also had to
`Box::pin(..).await` its own recursive call — `E0733`, self-recursive `async fn`)/`resolve_route`
(had to restructure `sort_by`/`.map()` off inline `async` calls into a precompute-then-sort-sync
shape, since `Vec::sort_by`'s comparator is std's fixed sync `FnMut` and can never itself await)/
`io_route`/`resolve_run`/`io_run`/`resolve_identify`/`io_identify`/`io_entries`, and the mirror-image
OLD `ComposerEntry` registry (`composer_entries_by_key`/`preflight_composer_entry_refs*`/
`register_composer_entry_refs*`/`resolve`/`dialects_for`/`io_keys_for`/`list_composer_entries`/
`io_dispatch`/`commit_artifact_assembly_registry_plan`), the `SubsetValidator` registry mirror, the
`Wire` region (`validate_wire_*`/`intern_dialect`/`wire_decode_composed_artifact`/
`wire_list_composer_entries`/`wire_artifact_compose`), the `FormatCatalog` region
(`register_format_descriptors*`/`preflight_format_descriptors*`/`index_format_descriptors`/
`validate_format_descriptors`/`formats_csv`/`normalize_format_kind`), `IoFidelityClass`, and every
test in `io_mechanism::laws`, `io_fidelity_tests`, and the top-level `tests` module that exercises
any of the above (the `key`/`passthrough`/`always_high`/`flag_non_object` test doubles: the two
fn-pointer ones (`passthrough`, `always_high`) went back to plain sync `fn` + E4 tag since they never
suspend; `key`/`format_descriptor_fixture` stayed `async fn` + `.await` at every call site since
they're regular helpers, not fn-pointer values).

**`ArtifactDialect::to_coordinate`/`parse_coordinate`, `ArtifactKindId::parse`/`as_str`/`plugin`/
`artifact`, `is_canonical_artifact_kind`** (in the SIBLING file `🚪️io/🧬️schema/🦀️component.rs` —
also inside my owned `🚪️io/**` path per rule 1, not the guest SDK): a **different, concurrent**
session's asyncify sweep (same second, 13:21:49, as the earlier one that hit `component.rs` itself —
both landed before I read this region) turned these from plain to `async fn`. They're used
pervasively throughout `🚪️io/🦀️component.rs` in `sort_by_key`/`.map()`/`.sort_by()`/`ok_or_else`
closures and in TWO `Display::fmt` impls (E1, fixed sync signature) — none of which can `.await`.
Fixed every site: plain-async-fn-body call sites got `.await`; every closure/`Display::fmt` site got
wrapped in `resolve_ready` (these never truly suspend — trivial `format!`/string-split bodies).
Found this by re-reading fresh from disk per the coordinator's own advice, NOT by trusting an earlier
transcript read — it would otherwise have silently reopened errors in the exact functions
(`route_rank`, `resolve_run`, `IoRegistryError::fmt`, `dialects_for`) I'd just finished fixing.

**E4/E5 tagging — done per the coordinator's mid-task directive**: every sync `fn` this packet left
behind (5 inline `IoEntry`-constructor thunks, `subset_validator_entry_of`'s inner `validate`, the 3
new macros' `fn __thunk` bodies — tagged even though macro-generated instances don't strictly need
it per R2, in case a text-based codemod re-scans macro *definitions* too — `passthrough`,
`always_high`, `flag_non_object`, `resolve_ready`) now carries `// 🚫️async: E4 fn-pointer slot` or
`// 🚫️async: E5 executor bridge` immediately above it. Grepped for the tag string to confirm: 10
occurrences, one per site, `resolve_ready`'s is the sole `E5`.

## What was NOT fixed, and why (scope boundary, not oversight)

`🚪️io/🦀️component.rs` turned out to carry a SECOND, unrelated wave of damage: a self-contained
`SourceSpan`/`CodecFailure`/`CancellationToken`/`CodecBudget`/`DecodeContext`/`EncodeContext`/
`PayloadSource`/`RandomAccessPayload`/`ResourceResolver` subsystem (roughly lines 49–700, plus its
own tests ~2140–2260) that a THIRD concurrent session's asyncify sweep also touched (184 fn→async-fn
conversions, confirmed via `git diff HEAD` before I started editing — not the codemod baseline, not
me, not the two sweeps above). This is plain "regular `async fn`, call site needs `.await`" damage —
structurally nothing like the erasure-table problem this packet owns — but it is NOT reachable by
`insert-await.py` right now for the same reason nothing else is (see below), so it sits unfixed. It
does not block anything this packet is responsible for; I did not touch it so as not to burn the rest
of this packet's budget on a subsystem with its own name and its own likely owner.

## Acceptance — blocked upstream, not by this packet

```
CARGO_TARGET_DIR=/private/tmp/claude-501/.../scratchpad/target-io cargo check -p semio-framework --lib
    error[E0308] × 2 in 🖱️ui/🎨️styling/📦️glue.rs   (baseline, BEFORE my fix)
```
→ fixed, verified green in isolation (`-p semio-framework-ui-styling --lib`, exit 0).

`semio-framework` itself has never been reachable by `cargo check` this whole session:
`semio-framework` → `semio-framework-os-kernel` → `semio-framework-replication` (hard, non-optional
`[dependencies]` entry, no feature gate) is broken with real compile errors (52+19+17+11+9+5+3+3+2+2
= across `🌱️value/🔀️serde`, `📡️wire`, `🔗️causal`, `⚠️diagnostic`, `🧾️wire`, `🔢️scalar`, `🌱️value`,
`📐️format`, `⚙️codec`, `🆔️ids`, `🎮️mutation` — none of it under `🚪️io/**`, none of it mine). Confirmed
repeatedly across the session as OTHER sessions made progress on it: 209 → 125 → 124 → 77 errors, all
still exclusively in `📡️replication`'s tree, **zero attributed to `🚪️io/🦀️component.rs` at any point**
(`-p semio-framework --lib` and `--all-targets` message-format=json, grepped every error's primary
span file). Also ran `-p semio-framework-os-kernel --lib` directly — same 209/… errors, confirming the
block is at that layer, not something `-p semio-framework` adds on top.

**I cannot get a real compiler verdict on my own `🚪️io/🦀️component.rs` changes.** Every check I could
run (both `-p semio-framework` variants, `-p semio-framework-os-kernel`) fails before rustc ever
reaches this file, through no fault of this packet. What I *can* report: `-p semio-framework-ui-styling
--lib` is green (exit 0, my 2-line fix), brace/paren counts are internally consistent with the
pre-codemod baseline's own pre-existing off-by-one (not a regression), and every fix above is backed
by a specific, cited reason (field type read directly from the struct definition, trait signature read
from `RawWakerVTable::new`, existing call-site convention grepped repo-wide). This is not a substitute
for `cargo check` and I am not claiming it as one.

**`--all-targets` test-module breakage**: unreachable for the same reason — cannot even ask the
question yet. Once `📡️replication` is fixed, expect `#[test] async fn` under `#[cfg(test)] mod tests`
(top-level, `io_mechanism::laws`, `io_fidelity_tests`) to need whatever the sibling `#[async_test]`
packet lands, per `📌️important.md`'s "do not fix those by hand" instruction — not attempted here.

## `compose-thunk-rewrite.py`

Written to `<ticket>/compose-thunk-rewrite.py`. Finds `ComposerEntry { .. }` / `IoEntry { .. }`
literals whose `compose:`/`run:`/`sniff: Some(..)` value is a bare path (identifier, optional `::`
qualification, optional single `::<...>` turbofish — no `!`, no `(...)` call, no closure) and wraps it
in `compose_thunk!`/`io_run_thunk!`/`io_sniff_thunk!`. Idempotent (a wrapped value's macro-call syntax
never matches "bare path" again). `--scan` (JSON report, no writes), `--apply`, `--root` to restrict.

Iterated the brace/turbofish-scanning logic twice against real false positives it found in the live
repo before trusting it: (1) `Name {` also matches the struct *definition* and a fn's `-> Name {`
return-type-then-body-open — both fixed by checking the preceding token. (2) a field regex `run\s*:`
was matching the SECOND `:` inside `run::<S, T>`'s OWN captured value as a spurious nested field —
fixed with a `(?!:)` guard. Verified idempotency and correctness on a synthetic fixture in the
scratchpad (`compose_leaf`/`run_leaf`/`sniff_leaf`): pass 1 applies 4 edits, pass 2 reports
`bare: 0, wrapped: 4` (no further edits) — confirmed by rerunning, not asserted.

**`--scan` over the whole repo:**
```json
{
  "files_scanned": 205,
  "files_with_sites": 32,
  "bare": 184,
  "wrapped": 4,
  "other": 1,
  "none": 20
}
```
(`wrapped` = 4 = this file's own `compose_thunk!` call sites, already correct. `other` = 1 =
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🦀️component.rs:87`, `compose: e.compose` — a field-access
expression, correctly left alone, not a bare path.)

**`--apply` was deliberately NOT run**, including not over `🧰️framework` as literally instructed —
running it would have been actively harmful, not a no-op. Breaking that down: of the 184 "bare" sites,
every single one under `🧰️framework` (28 in this file's own already-hand-fixed constructors — a bare
`run: run::<S, T>`/`run: passthrough`/`sniff: Some(always_high)` is now CORRECT sync code, not a
codemod artifact — and 3 in the guest SDK's `composer_entry_of`/`deserializer_entry_of`/
`serializer_entry_of`, which ALREADY hand-implement the exact `fn erased_compose(..) -> ComposeFuture
{ Box::pin(async move {..}) }` pattern `compose_thunk!` generates) would be **double-wrapped** by a
naive text-pattern scan that can't tell "still-async-fn bare path" from "already-fixed-sync-fn bare
path". For the guest SDK specifically that's also flatly forbidden (rule: "NOT yours: the guest SDK
🔌️plugin/🦀️component.rs"). The remaining ~153 genuinely-bare sites are exactly what they're
described as: the 163-site fleet population under `✏️s/**`, confirmed untouched (`✏️s/` appears
nowhere in this packet's `git diff`). The script is verified and ready for `fleet-codemods` to point
at `--root '✏️s'`.

## Owned-path compliance

- Edited: `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`,
  `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📦️glue.rs` (only the 2 named lines),
  plus 111 files under `🧰️framework/🔨️modules/**` via the sanctioned `insert-await.py` run (rule 3),
  excluding `⏳️async/**` and `🎒️pack/**` throughout (grepped both exclusions against the final diff:
  zero hits).
- Read-only: `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️component.rs` (grepped/read to discover the
  `to_coordinate`-family breakage — fixed at CALL SITES in `component.rs` instead of editing this
  file, since the async conversion there is another session's live, uncommitted, in-progress edit I
  should not fight — the fix is entirely call-site `resolve_ready`/`.await`, no change to the schema
  file itself was needed), `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (read-only,
  confirmed its `composer_entry_of` family and `resolve_ready` usage are already correct, no edit).
- Restored to `HEAD`: 203 files the shared `insert-await.py` tool over-reached into
  (`🧰️framework/🛍️products/**`, `🦑️repo/**`) — via `git show HEAD:<path>` + `Write`, no
  git-modifying command used.
- Not touched: `🎒️pack/**`, `⏳️async/**`, `🏪️store/**`, root `Cargo.toml`, anything under `✏️s/`.

## `lease-request`

One public signature in my owned file changed from sync to async and has exactly one external
caller, in the guest SDK (`🔌️plugin/🦀️component.rs`, not mine to edit — ATOMIC packet owns it):

```lease-request
File: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs
Line: 3739 (inside `pub(crate) fn commit_artifact_registration_plan`, 🔖️ArtifactDeclaration region)

Reason: `semio_framework::io::commit_artifact_assembly_registry_plan` changed from `pub fn` to
`pub async fn` (terra-io-thunks, 🧰️framework/🔨️modules/🚪️io/🦀️component.rs) — it now calls several
regular helpers (io_registry/subset_validator_registry/composer_entries_by_key/
validate_composer_entries/validate_subset_validators/index_format_descriptors/
validate_format_descriptors) that are themselves correctly `async fn` per the universal-async decree,
and could not stay sync without reintroducing the exact "async fn called from sync context" bug this
packet exists to remove.

Current text:
    pub(crate) fn commit_artifact_registration_plan(assembly: &store::ArtifactAssemblyTransaction, plan: semio_framework::io::ArtifactAssemblyRegistryPlan) -> Result<(), PluginAssemblyError> {
        semio_framework::io::commit_artifact_assembly_registry_plan(assembly, plan).map_err(|error| PluginAssemblyError::new("plugin-assembly.registry", error.to_string()))
    }

Suggested replacement (matches this file's own existing `resolve_ready` convention for exactly this
shape — see e.g. line 16336's `resolve_ready(instance.app.media_fingerprint(&port))` — keeps this
function's own signature, and every call site of IT, unchanged):
    pub(crate) fn commit_artifact_registration_plan(assembly: &store::ArtifactAssemblyTransaction, plan: semio_framework::io::ArtifactAssemblyRegistryPlan) -> Result<(), PluginAssemblyError> {
        semio_framework::resolve_ready(semio_framework::io::commit_artifact_assembly_registry_plan(assembly, plan)).map_err(|error| PluginAssemblyError::new("plugin-assembly.registry", error.to_string()))
    }

Alternative (if the SDK's own convention there is trending toward async instead): make
`commit_artifact_registration_plan` itself `async fn` and add `.await` before `.map_err(..)`; then its
own callers need one more hop of `.await`/`resolve_ready` — I did not chase that chain since it's
outside my owned path and I have no visibility into what calls THIS function.
```

## Files touched (for `ticket_close`, when sol runs it)

- `🧰️framework/🔨️modules/🚪️io/🦀️component.rs` (extensive — erasure tables + cascading `.await`/
  `resolve_ready` repairs)
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📦️glue.rs` (2 lines)
- 111 files under `🧰️framework/🔨️modules/**` (mechanical `.await` insertion via `insert-await.py`,
  rule-3-sanctioned)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/compose-thunk-rewrite.py` (new)
- Ticket-folder scratch (`.txt`/`.json`, this report) — all under `/private/tmp/.../scratchpad/` per
  rule 24, nothing left in the ticket folder itself except this report and the script.
