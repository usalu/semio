# oskernel-alltargets — `semio-framework-os-kernel --all-targets` to green + `cargo test --lib`

**Packet**: `oskernel-alltargets` (terra, autonomous, no interactive dev in either session half)
**Scope**: `🧰️framework/🛍️products/💻️os/🔨️modules/**` (excl. `🔌️plugin/**`, `🤖️generated/**`), plus `🔨️modules/🚪️io/**` / `🔨️modules/🕹️interaction/**` only where the compiler attributed an error there (it never did — untouched).

## Final state (all four acceptance gates, re-verified from a clean run immediately before writing this report)

```
CARGO_TARGET_DIR=.../scratchpad/target-oskat cargo check -p semio-framework-os-kernel --all-targets   → EXIT 0
CARGO_TARGET_DIR=.../scratchpad/target-oskat cargo check -p semio-framework-os-kernel --lib            → EXIT 0
CARGO_TARGET_DIR=.../scratchpad/target-oskat cargo check -p semio-framework --lib                      → EXIT 0
CARGO_TARGET_DIR=.../scratchpad/target-oskat cargo test  -p semio-framework-os-kernel --lib
  test result: ok. 779 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.72s
```

**779 passed, 0 failed — exact match to the historical baseline (779), by name (full list via `cargo test --lib -- --list`: 779 tests, 0 benchmarks) and by outcome (zero failures, zero newly-added `#[ignore]`s — none were added at any point).** Full raw run saved at `terra-oskat-lib-check.txt` / the last `cargo test` capture is `/private/tmp/.../scratchpad/testrun-final.txt` (session-local scratch, not copied into the ticket folder since it is pure console text with no diagnostic value beyond what's summarized here).

## Progress arc

- Session start (pre-compaction): `--lib` already EXIT 0 (1000+ → 0 errors, prior packet work). `--all-targets`: **1553 errors**.
- Pre-compaction end: driven down to **44 errors** (22×E0308, 22×E0277) across ~54 diagnostic rounds, using the pre-existing shared `insert-await.py` (fixpoint fast, ~25 edits total) plus two new diagnostic-driven tools built this packet (`terra-oskat-future-fixer.py`, `terra-oskat-collapse-repeated-await.py`) plus extensive hand-fixing (E4/E1 trait-tag reverts, E0733 recursion boxing, E0502/E0505/E0716 borrows, E0283 ambiguity, derive-macro fixes in `dsl_derive`'s `glue.rs`).
- This continuation: **44 → 0** over 7 more rounds (55–60 below), then a targeted sweep of **292 latent "unused implementer of `Future`" warnings** (see "the big one" below) that `--all-targets` finally being green made visible for the very first time all session.

### Rounds 55–60 (44 → 0 compile errors)

1. **22×E0308 (round 54→55)**: mechanical missing-`.await` at argument/binding positions (`pack/⌨️cli`, `pack/🔢️value`, `💡️inference`, `🏪️store`), a `bool::then(|| async_call())` restructured to `if/else` with `.await` inside, and the `dsl_derive::CompositeMutation` proc-macro (`🗣️dsl/✨️derive/.../📦️glue.rs`) — its 5 generated `MutationKind` delegate methods (`diff`/`inverse`/`label`/`target`/`foreign_steps`) called the now-async `fold_plan_diff`/`fold_plan_inverse`/`CompositeMutationKind::label`/`::target`/`plan_foreign_steps` helpers **without `.await`**, a compile-time-caught instance of the same defect class as the runtime ones below.
2. **20×E0277/E0382/E0716/E0502 (round 55→57)**: repeated-`.await`-on-one-binding reuse (hoisted to the `let`, same pattern as the pre-compaction `collapse-repeated-await` tool), a `store.generation()` future left un-awaited holding an immutable borrow across a later `&mut store` call (E0502), and — the one **genuinely architectural** fix — `🏪️store/🦀️component.rs`'s `mod tests` defines a **local newtype `ArtifactStore<P,Mutation>(super::ArtifactStore<P,Mutation>)`** wrapping the real store, because only *this wrapper* implements `MemberFactory` (needed by `CompositionCoordinator::dispatch_group<M: SpaceMember + MemberFactory>`'s children). Four tests built `parent_store` from the **real** type (`super::ArtifactStore::new(...)`) per an explicit doc comment warning about the wrapper's `SpaceMember` impl not overriding `merge_policy`, which no longer type-checked once `dispatch_group` requires the *same* `M` for parent and children. Switched all four to the wrapper.
3. **6×E0382 (round 57)**: same repeated-await-reuse pattern in `spr/⌨️cli/component.rs`'s file-roundtrip test (`ops_path`/`spr_path`/`decompiled_path`), hoisted to the `let`.
4. **Round 57 → EXIT 0.**
5. **Regression caught by `cargo test`, not `cargo check`**: switching `parent_store` to the test wrapper (step 2) compiled clean but **silently broke the merge-policy tests** — `dispatch_group_phase1_accepts_the_same_error_scenario_under_laissez_faire` and `..._rejects_under_vigilant_on_a_members_warning` failed with the dispatch always seeing `MergePolicy::Normal` regardless of `set_merge_policy`. Root cause: `SpaceMember::merge_policy` has a trait-level default (`MergePolicy::default()` = `Normal`); the **real** type's own `impl SpaceMember` overrides it to delegate to the landed inherent method (§C6), but the **test wrapper's** `impl SpaceMember` (in `mod tests`) never got the same override — so any call to `.merge_policy()` on the wrapper, whether direct or through `dispatch_group`'s generic `M: SpaceMember` body, silently used the trait default instead of the real value. This is exactly the failure mode the original doc comment was warning about, just reached by a different path (the wrapper's *trait* impl, not a `&mut dyn SpaceMember` coercion). Fixed by adding the missing override to the wrapper's `impl SpaceMember` block (`SpaceMember::merge_policy(&self.0).await`), mirroring every other method in that block. Re-ran `cargo test --lib`: 779/779 green.

### The big one — 292 silently-dropped futures, only now visible

The moment `--all-targets` first reached EXIT 0 (round 57), `cargo check --all-targets`'s own warning stream (previously never reached, because compilation always aborted on an error first) surfaced **292 instances of `warning: unused implementer of \`Future\` that must be used`** across 11 files — every one a bare expression-statement calling a now-async helper (`assert_*_round_trip(...)`, `write_varint_u64(...)`, `hex::encode(...)`, byte-writer/parser calls, etc.) whose returned future was simply discarded. **This is precisely the "silent no-op" defect class the ticket brief names as the single highest-value bug category in the whole program** — a call that visually looks correct, compiles clean, and never runs.

Distribution (by file, warning count):
`🏪️store` 77 · `📡️spr/🧵️channel` 63 · `📡️spr/🧪️testkit` 57 · `🗣️dsl/🧬️schema` 29 · `🗣️dsl/` (root) 14 · `📡️spr/🎮️command` 13 · `📇️directory/🔌️client` 12 · `🗣️dsl/🧪️fixture-sweep` 9 · `📇️directory/🪪️identity` 9 · `🌿️vcs` 8 · `💡️inference` 1.

**Two of these were confirmed to have real behavioral consequences, not just "assertion never ran":** `🗣️dsl/🧪️fixture-sweep/🦀️component.rs` has **three** self-recursive directory-walk functions (`collect_files` ×2 in different scopes, `walk`) whose recursive call (`collect_files(&path, out)` / `walk(&path, hits)`) was the dropped future — meaning **these walkers never descended into subdirectories at all**, silently limiting fixture discovery to each directory's top level for however long this went unnoticed. Fixing this (adding `.await`) turned it into an E0733 "recursion in an async fn requires boxing" compile error, confirming the recursion was real and previously dead code from the compiler's perspective; wrapped each in `Box::pin(...)`.

Given the scale (292 sites, 11 files) a **new diagnostic-driven tool** was built rather than hand-fixing each one — `terra-oskat-unused-future-fixer.py`, saved in this ticket folder per R10. It is strictly span-keyed off rustc's own `unused_must_use` diagnostic spans (never name/regex-matched): for each warning's primary span it inserts `.await` immediately before the span's trailing `;` if present, else appends it directly at the span end; spans are processed back-to-front by byte offset per file so earlier insertions never invalidate later offsets; a span whose text already ends in `.await`/`.await;` is skipped (idempotent against being re-run). Applied once, cleanly, across all 11 files; re-running `cargo check --all-targets` afterward showed **zero** remaining "unused implementer of `Future`" warnings and (after the 3-site `Box::pin` fix above) EXIT 0 again.

### Tooling-corruption note (residual from the pre-compaction incident, found and fixed this half)

While auditing the fixer's output, one of the 3 post-fix compile-error sites led to inspecting `🗣️dsl/📖️grammar/🦀️component.rs` line 894, which read `cursor.expect_ident("arm.await").await?;` — a `.await` **inside a string literal** (`"arm"` → `"arm.await"`), which is syntactically valid Rust (so `cargo check` never caught it) but semantically wrong (the parser was looking for the literal token `arm.await` instead of `arm`), and it broke exactly one grammar test at runtime (`repeat_block_two_level_nested_dispatch_gif89a_shaped`) the first time `cargo test` actually got far enough to run it. `🗣️dsl/📖️grammar/🦀️component.rs` is one of the 8 files the **pre-compaction** session had to hand-repair after `terra-oskat-collapse-repeated-await.py`'s original double-delete bug (see prior summary) — that repair used `re.sub(r'(\.await){2,}', '', text)` plus targeted hand fixes, which would not have caught a single stray `.await` suffix like this one. Fixed the one line by hand, then swept the **entire `🧰️framework` tree** for the same shape (`"[A-Za-z0-9_]+(\.await)+"` — a whole-token string literal ending in one or more `.await` suffixes) — zero further hits. This was **not** introduced by anything in this continuation (the new `terra-oskat-unused-future-fixer.py` never touched this file — it's not in its 11-file output list); it's a previously-undetected residue of the earlier incident, now closed out.

## Silent no-op / dropped-future bugs found and fixed this ticket (cumulative, both session halves)

1. *(pre-compaction)* `bad_record.await.body_hash = [0xFFu8; 32];` and a similar `record.await.fields.insert(...)` — mutating a **dropped temporary** produced by an inline `.await`, never persisting.
2. *(pre-compaction)* `register_schema_spec(...)` called without `.await` — registration silently never happened.
3. *(this half)* `dsl_derive::CompositeMutation`'s 5 generated delegate methods missing `.await` on their inner calls (compile-time error, not runtime-silent, but same "async call whose result is discarded/mistyped" root cause).
4. *(this half)* **292** unused-`Future` warnings, the large majority of which are dropped `assert_*_round_trip`/encode/decode helper calls — the test bodies that called them looked complete but the assertion helper's body (including its own nested `assert_eq!`s) never executed. Three of them (the fixture-sweep directory walkers) had an additional behavioral effect beyond "assertion didn't run": recursive directory descent was dead code.
5. *(this half, found while investigating the assert_eq! macro-expansion E0277 residue)* `📡️spr/🧪️testkit/🦀️component.rs`'s `golden_hash_hex_is_deterministic_and_hex_encoded` test had `assert_eq!(a, 64)` comparing a `String` (a hex hash) against the integer `64` — not an async-migration artifact, a pre-existing test bug (evidently the intent, per the adjacent `a.chars().all(is_ascii_hexdigit)` check, was `assert_eq!(a.len(), 64)`) that simply never compiled under `--all-targets` before now. Fixed to `a.len()`.

## New tools saved to this ticket folder (R10 — span-keyed, diagnostic-driven, never name/regex-matched against source identifiers)

- `terra-oskat-future-fixer.py` *(pre-compaction)* — inserts `.await` at rustc-diagnosed E0308/E0369/E0599/E0600/E0608/E0277 spans where no `suggested_replacement` exists.
- `terra-oskat-collapse-repeated-await.py` *(pre-compaction, bug-fixed mid-session)* — hoists a single `.await` to a `let` binding and strips it from repeated reuses (E0382), with dedupe + overlap guards added after the corruption incident above.
- `terra-oskat-unused-future-fixer.py` *(this half, new)* — inserts `.await` at every `unused_must_use` "unused implementer of `Future`" warning span; the tool that closed out the 292-warning sweep described above.

All three remain in this ticket folder per R10/the ticket rules; none were deleted.

## Files touched this half (continuation only; the pre-compaction file list is in the prior session's summary, not repeated here)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🔢️value/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` (incl. the `MemberFactory`/`merge_policy` wrapper fix)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs` (incl. the `a.len()` bug fix)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🪪️identity/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs` (the dead-recursion `Box::pin` fix)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/📖️grammar/🦀️component.rs` (the residual `"arm.await"` string-literal corruption fix)

## Not touched / explicitly out of scope

- `🎭️actor/**`, `🔌️plugin/**`, root `Cargo.toml` — not owned this packet; a peer session is live there (confirmed via `git diff --stat` showing large concurrent churn in `🔌️plugin/component.rs`, `🛎️services/component.rs`, etc. that this packet never edited).
- `🔨️modules/🚪️io/**` / `🔨️modules/🕹️interaction/**` — never attributed a compiler error, so left untouched per the packet's conditional-ownership clause.
- The 48 "`async fn` in public traits is discouraged" and 20 "unexpected `cfg` condition value: `typegen`" warnings visible in `cargo test --lib` output are pre-existing, unrelated to this packet's acceptance criteria (all-targets EXIT 0 + test baseline), and were not touched.

## Ticket-folder scratch artifacts (not deleted, per ticket rules)

`terra-oskat-round1.json` … `terra-oskat-round60.json` (progressive diagnostic captures, 1553→0 errors), `terra-oskat-diag*.jsonl`/`terra-oskat-diagloop-*.jsonl`/`terra-oskat-loop2-*.json` (intermediate fixpoint-loop captures), `terra-oskat-*-apply*.txt`/`terra-oskat-*-dryrun*.txt` (tool-run logs), `terra-oskat-all-baseline.{json,txt}` and `terra-oskat-lib-baseline.txt` (session-start baselines), `terra-oskat-asynctest-scan.json`, plus the three `.py` tools listed above.
