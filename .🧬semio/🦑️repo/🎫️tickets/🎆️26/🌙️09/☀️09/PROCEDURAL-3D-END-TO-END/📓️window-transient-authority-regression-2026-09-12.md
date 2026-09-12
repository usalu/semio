# Window-transient publication authority — native regression cover (2026-09-12)

Closes the coverage gap named in `📓️audit-invoke-extension-authority-2026-09-12.md` **Q5 item 2**:
commit `de93f84300` removed the `_ if failed => Err("… is retiring a rejected authority")` arm from
all seven publication lanes and added a `refresh()` before the window-transient `begin()`, but **no
test pinned either behaviour**. It is pinned now, fixture-first, across all seven lanes, with a
TypeScript twin.

## What shipped

| Artefact | Path |
| --- | --- |
| Language-agnostic fixture | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/♻️publication-retirement-authority/🔣️.json` |
| Rust law (2 tests) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🦀️.rs` |
| TypeScript twin (Ajv + `node:assert`) | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🟦️.ts` |
| Production consolidation | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` |
| Runner registration | `📜️script.ts` (`verify publication-retirement-authority [oracle]`), `📋️project.json`, `.vscode/launch.json` |
| Anchor repair | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs` |

Placement follows the neighbouring triple `🪟️window/🫧️transient/{🧫️fixtures,🧪️tests}/🪟️retained-window-input/{🔣️.json,🦀️.rs,🟦️.ts}`.
The plugin crate declares **no `[[test]]` targets** — every one of its suites is a `#[cfg(test)]`
`#[path]` module inside `🦀️.rs` (see `mutation_fixture`, `publication_fixture`,
`retained_window_input_tests`), and `PendingArtifactStorePublication` is private to `pub mod app`,
so an integration-test crate could not reach it. The new module is registered the same way and is
run with `--lib <filter>`, exactly as the audit's own proof line proposed
(`cargo test -p semio-framework-plugin <new_test_name>`).

## Production change: one shared retirement law instead of seven copies

The seven `Closing`-phase arms inside `publish_mounted_typed_operation_unit` were seven verbatim
copies of the same ten lines, differing only in a store label and in window-transient's leniency —
the exact shape that let ONE lane keep the `_ if failed` bug while the others were fixed. They now
collapse to one call, and the whole decision lives in one testable method on
`PendingArtifactStorePublication<A>`:

```rust
fn lane(&self) -> (TypedOperationResultLane, &'static str)
fn is_closing(&self) -> bool
fn fault(&self) -> Option<&str>
fn retirement_turn(&mut self, maximum_items: usize, maximum_bytes: usize)
    -> Result<PendingArtifactStorePublicationRetirement, Fault>   // Retiring | Retired | Rejected(Fault)
```

Behaviour is preserved exactly, including the asymmetry the audit flagged: window-transient returns
`Retired` (no host fault) even when it was rejected, the other six return `Rejected(<label> publication
rejected stale or cancelled authority)`; a `Complete` step without terminal emptiness is still
`Err(<label> publication closed without terminal emptiness)`. Net: 208 lines changed, three hunks,
no other edit to the file.

## The law the fixture declares

`begin → superseding mutation rejects it → several INCOMPLETE close turns must be Ok → terminal turn
→ a new begin() for the same window succeeds only against a refreshed generation.`

Per-lane rows carry `storeLabel`, the store's own `supersededFault`, `rejectionIsFatal`,
`terminalOutcome`, `rejectedFault`, `falseTerminalFault`. Both terminal messages are derived from the
lane's own store label, so no lane can drift into another lane's wording — the Rust test asserts the
derivation against the live `lane()` accessor and the TS twin asserts the same derivation from the
JSON alone.

### Rust test 1 — all seven lanes

`every_publication_lane_retires_a_rejected_authority_without_faulting_each_turn` parametrises over the
seven fixture rows against a new in-crate `RetirementApp` that owns a **real** store on every lane at
once (document/config/draft from `test_app_mutation_fixture`, presence/transient from
`publication_fixture`, plus a registered `WindowConfigOwner` and `WindowTransientOwner`). Per row it
begins a real publication, supersedes the store (a dispatch for the batch lanes, a second publication
driven to `Published` for the ephemeral and window lanes), takes the store's real rejection, then
drives `retirement_turn` to its terminal turn.

Observed (`-- --nocapture`):

```
[DEBUG] "artifact"        retired a rejected authority over 50 Ok turns then "rejected"
[DEBUG] "config"          retired a rejected authority over 50 Ok turns then "rejected"
[DEBUG] "draft"           retired a rejected authority over 50 Ok turns then "rejected"
[DEBUG] "presence"        retired a rejected authority over 55 Ok turns then "rejected"
[DEBUG] "transient"       retired a rejected authority over 58 Ok turns then "rejected"
[DEBUG] "windowConfig"    retired a rejected authority over 50 Ok turns then "rejected"
[DEBUG] "windowTransient" retired a rejected authority over 60 Ok turns then "retired"
```

Every one of those 50–60 turns per lane is a turn the pre-fix code answered with
`typed-operation failed: … is retiring a rejected authority`. That is the live fault's mechanism made
visible: a rejected publication drains its reason **one scalar per bounded turn**, so the old arm
re-raised the same fault ~50 times per lane and the operation behind it never retired.

### Rust test 2 — the `refresh()` half

`window_transient_re_begin_needs_the_refreshed_live_generation` drives the full lifecycle on one
window: capture authority → begin → supersede → rejected → **1081 Ok retirement turns** → terminal →
`begin()` with the authority captured at admission is **refused** → `refresh(&mut authority)` →
generation advanced → `begin()` **accepted** and published. That is precisely the two-line sequence
the window-transient emission branch performs (`window_transient_store.refresh(authority)?` then
`.begin(...)`), so removing the `refresh()` makes the re-begin unreachable in the same way the test
asserts.

### TypeScript twin

`🟦️.ts` parses the same fixture with `JSON.parse`, validates the lane table with a third-party
`Ajv` schema (7 items, closed properties, enum terminal outcomes), asserts distinct ids and store
labels, the exact lane order, both message derivations, that window-transient is the one lenient lane,
and the ordered re-begin revisions.

## Proof the tests catch the regression

The `_ if failed => Err(format!("{label} publication is retiring a rejected authority"))` arm was
temporarily reinstated in `retirement_turn` and the suite re-run — both tests fail immediately with
the original live fault text:

```
WindowTransient answered an incomplete retirement turn with a fault:
Fault { origin: Plugin, code: FaultCode("plugin.internal"),
        message: "window-transient publication is retiring a rejected authority", … }
```

The arm was then removed again; `grep -n "is retiring a rejected authority" 🔌️plugin/🦀️.rs` now
returns one hit only, inside the `retirement_turn` docstring that explains the defect.

## Commands run (foreground) and results

```
$ RUST_MIN_STACK=33554432 cargo test -p semio-framework-plugin --lib publication_retirement -- --nocapture
test component::app::publication_retirement_authority::window_transient_re_begin_needs_the_refreshed_live_generation ... ok
test component::app::publication_retirement_authority::every_publication_lane_retires_a_rejected_authority_without_faulting_each_turn ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 665 filtered out; finished in 0.04s

$ bun 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🟦️.ts
[DEBUG] publication retirement authority: 7 lanes, 6 fatal on rejection, refreshed re-begin accepted=true

$ bun nx run workspace:test-publication-retirement-authority          → Successfully ran (oracle + cargo)
$ bun nx run workspace:test-publication-retirement-authority-oracle   → Successfully ran (oracle only)
```

No wasm build, no dev server, no restage was started. Raw logs:
`🗑️generated/authority-regression/{final-run,cargo-build,cargo-regression-proof,cargo-full-lib,cargo-related,cargo-anchor,cargo-dummy,cargo-unrelated,nx-verify,warnings}.txt`.

**`verify-taxonomy-enforce` was NOT completed.** It was started, ran 34 min at 20 % CPU while a peer
held a concurrent `verify taxonomy report` walk, and was then stopped (only my own pid) rather than
left orphaned. Targeted check instead: `semanticDirectoryKinds` matches directory kinds by
emoji + slug, and the immediate neighbours of the new directories carry no registered kind either —
`📢️publication-fixtures`, `🔬️tick-addressing` and the new `♻️publication-retirement-authority` all
resolve to zero kinds, while the file kinds used (`🦀️.rs`, `🟦️.ts`, `🔣️.json`) and the parent kinds
(`🧪️tests`, `🧫️fixtures`) are the registered ones. The new directories are therefore in exactly the
same taxonomy position as their siblings; re-run the gate when the repo walk is uncontended.

## Pre-existing failures observed in this crate (NOT caused by this change)

`cargo test -p semio-framework-plugin --lib` (whole suite) aborts on unrelated, already-broken tests.
Each was checked against an untouched surface:

1. `app_builder_tests::*` — `app-definition.interactive-job-classification: unclassified interactive
   command …`. A peer's in-flight job-classification work; `🦀️.rs:5662` is outside all three of my hunks.
2. `mutation_fixture::dummy::*` / `::transaction::*` — `app id testkit-dummy must be a canonical
   surface id: … missing '#'` and `interactive-job.catalog-authority … generated_migrated=false`.
   Same peer lane; no publication involvement.
3. `typed_command_full_operation_tests::full_operation_source_rejects_generic_reducers_and_old_monolithic_shells`
   and `::host_configuration_uses_one_bounded_event_sourced_lane_before_the_generic_gate` — stale
   **source-text anchors**: `grep -cF "async fn dispatch_typed_command(" 🔌️plugin/🦀️.rs` = 0, so the
   anchors point at routes a peer already renamed.
4. `typed_command_full_operation_tests::fixture_contract_is_anchored_to_the_production_retained_factory_publisher_and_host_receivers`
   — this one **did** break on my refactor: it anchors the publisher body on the literal
   `"publication.close_step(grant)"`, which the consolidation moved into `retirement_turn`. I updated
   that single anchor entry to `"pending.retirement_turn(grant.maximum_items, grant.maximum_bytes)"`
   with a comment naming this ticket. The test now advances past my line and fails on a **different,
   pre-existing** stale anchor: `reactor.contains("output.typed_operation_result.as_ref()")`, and
   `grep -cF` for that literal in the unmodified `⚛️reactor/🦀️.rs` = 0. Left for its owner.
5. `window_transient::retained_window_input_tests::retained_window_input_replacement_rejects_old_authority_and_publication`
   — `assert!(pending.terminal_is_empty())` at `🪟️retained-window-input/🦀️.rs:88`. **Proved
   pre-existing**: the new test module was temporarily `cfg`-disabled and the test still failed
   identically. It lives in `window_transient`, a module this change does not touch; the closing
   publication stays `Blocked` because the captured authority still holds its read lease at that point
   in the test. Landed already-red in `de93f84300` alongside the fix; worth a separate ticket lane.

Suites that pass clean after the change: `publication_fixture` (3/3), `retained_window_input` (5/6 —
the one failure is item 5 above, including
`retained_window_input_refresh_admits_live_generation_after_a_committed_write` which passes),
`typed_command_full_operation` 11/14 (the 3 are items 3 and 4).

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (three hunks: module registration, the
  shared `retirement_turn` law, the collapsed lane dispatch)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/♻️publication-retirement-authority/🔣️.json` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🦀️.rs` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/♻️publication-retirement-authority/🟦️.ts` (new)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs` (one anchor entry)
- `📜️script.ts`, `📋️project.json`, `.vscode/launch.json` (command registration)
