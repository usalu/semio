# 📓️ Verify: were laws (A) and (B) already failing before this ticket's changes?

Read-only verification. Method: real builds of the pre-change source tree, not just log inspection.

## 0. Commit identification

- Session-start HEAD: `f7fef5746d` (2026-09-16 21:35:27 +0200).
- The commit that first added `pub granularity`/`TreeWindow` to the contract
  (`git log -S'pub granularity' -- '🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs'`) is
  `a4cda597ea` (2026-09-17 01:10:14 +0200) — its parent-range `git log f7fef5746d..a4cda597ea -- '🧰️framework/🔨️modules/🖱️ui/🧬️contract' '🧰️framework/🔨️modules/🖱️ui/🧠️runtime'` shows **no other commit** touches either crate in between. So `f7fef5746d` is exactly the last commit before the change, confirmed by `git diff f7fef5746d a4cda597ea -- 🧩️component/🦀️.rs`: the whole `TreeWindow`/`granularity`/`UI_BUILT_CHILDREN_MAX`/`UI_VALUE_PAGE_ROWS` diff (contract + P5's new reconcile-unit law, 12 files, 295+/33-) landed in this **one squashed auto-commit**.
- "Current tree" below means the live working tree (same as `a4cda597ea`, HEAD unchanged since).

## 1. Method actually used: real builds, not just evidence-gathering

Extracted the pre-change source (`git archive f7fef5746d -- <paths> | tar -x`) for
`semio-framework-ui-contract`, `semio-framework-ui-runtime`, and their full path-dependency closure
(`ui-styling`, `value-derive`, `schema-registry`, `replication`, `hash`, `io-base64`, `job`, `async`,
`trace`, `deflate`, plus the source-only sibling modules `⚠️diagnostic` and `🌱️value` that
`replication`/`value-derive` `pub mod` in) into
`TICKET/🗑️generated/verify-retirement/pre/`, with a **new standalone workspace root `Cargo.toml`**
(`[workspace] members = [ui-contract, ui-runtime]` only — every other crate is a plain path dependency,
not a member, precisely so Cargo does not need to resolve their `[dev-dependencies]`) and a local
`.cargo/config.toml` pointing `target-dir`/`build-dir` at a `target/` subdir under the scratch path
(never touching the shared `⚡️cache/cargo` build dir). `rust-toolchain.toml` at the repo root was
inherited automatically (nightly-2026-07-07, unchanged).

Three classes of surgery were needed on the **scratch copy only** (never on tracked files):
1. **Stripped `[dev-dependencies]`** (and orphaned `[[test]]` entries) from the 9 leaf/support crates
   that are path-deps but not workspace members — Cargo parses a path dependency's full manifest
   including dev-deps even when it isn't a member, and `semio-framework-value-derive`'s real
   dev-dependency on `semio-framework-os-kernel` would have pulled in the entire `os` product.
   `semio-framework-ui-contract` and `semio-framework-ui-runtime` themselves keep their real
   dev-dependencies (plain `serde`/`serde_json` versions, nothing exotic) since we run their tests.
2. **Two gitignored generated files** don't exist in git history at any commit (`**/🤖️generated/` and
   the styling tokens file are build outputs, not source) — copied the current working-tree copies in
   (`🎨️styling/🔤️tokens/🦀️.rs`, a plain data-token file unrelated to this ticket; and an **empty
   placeholder** for `🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts`, which only matters to the
   unrelated `typegen_export` test — its content-mismatch failure is expected and irrelevant to (A)/(B)).
3. One missing plain fixture (`🧫️fixtures/📃️document-lease-owner-move/🔣️.json`) — this one **is**
   tracked at `f7fef5746d`, just under a directory this task's minimal path list initially missed;
   pulled via `git archive f7fef5746d`, confirmed byte-identical to the current working copy.

No tracked repo file was modified. Everything above lives under
`TICKET/🗑️generated/verify-retirement/pre/`.

Both crates built and ran clean (`cargo build -p semio-framework-ui-contract -p semio-framework-ui-runtime`
— 0 errors) once the closure above was complete.

## 2. (A) `semio-framework-ui-runtime` — Law B

**Test**: `reconcile::tree_retirement::tests::runtime_tree_retirement_preserves_occupied_sources_and_closes_exact_payloads`
(`🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs:38`), identified from
`📓️p5-wgpu-and-reconcile-law.md` §3.2 ("NOT a grant law; handed back") as the surviving failure next to
the fixed §3.1 law. Its flapping sibling per the same section:
`reconcile::tree_retirement::tests::runtime_tree_retirement_handback_preserves_partial_owner_until_full_readmission`.

| | Pre-change (`f7fef5746d`) | Current tree (`a4cda597ea`) |
|---|---|---|
| `runtime_tree_retirement_preserves_occupied_sources_and_closes_exact_payloads` | **FAILS**, isolated (`--exact`) and inside the full `-p semio-framework-ui-runtime` run alike | **FAILS**, identically |
| Panic text | `runtime exact tree owner did not reach terminal` at `♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs:38:5` | same text, same file:line, byte-identical |
| `runtime_tree_retirement_handback_preserves_partial_owner_until_full_readmission` (sibling) | **passes** in this run | reported as flapping in `📓️p5-wgpu-and-reconcile-law.md` — consistent with a flake, not a regression |
| Full suite | `121 passed; 2 failed; 2 ignored` — the two failures are this test **and**
`reconcile::tests::canonical_document_tests::surface_canonical_document_completion_transfers_do_not_borrow_the_child_grant` (the frozen-`4096`-literal Law A, still unfixed in this pre-change tree since P5's range-based fix hadn't landed yet) | `122 passed; 2 failed` reported by P5 §4 before its own fix, `123` after — Law A now green, tree-retirement still red |

Raw failure text (pre-change, `--exact`, deterministic):
```
thread 'reconcile::tree_retirement::tests::runtime_tree_retirement_preserves_occupied_sources_and_closes_exact_payloads' panicked at
🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/../../♻️reconcile/../♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs:38:5:
runtime exact tree owner did not reach terminal
```
Identical text/location reproduced live in the real workspace at HEAD.

`git log --follow` on the test file (`♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs`) shows its last edit
before this ticket was `6ad7b0e7bc` (2026-09-10 01:31:40); the retirement implementation it exercises
(`🌲️tree/🦀️.rs` region) is likewise untouched by `a4cda597ea`. Both files predate the ticket by six days.

**Verdict: pre-existing.** The exact same test, same assertion text, same source line fails identically
at the commit immediately before this ticket's `TreeWindow`/`granularity`/`UI_BUILT_CHILDREN_MAX`/
`UI_VALUE_PAGE_ROWS` changes and at the current tree. `size_of::<Component>()` at `f7fef5746d` is the
old (2568 B) value — verified by grepping the extracted `🧩️component/🦀️.rs` for `granularity`/
`TreeWindow` (zero hits) — so this failure cannot be a consequence of the size growth P1/P5 attribute it
to; it was already broken with the small `Component`. P1's own hypothesis ("a consequence of this
dependency change, not a peer edit," §5.1) is not supported by a real pre-change build — this
handoff to the coordinator (§3.2 in `📓️p5-wgpu-and-reconcile-law.md`) already treats it as a defect to
fix elsewhere rather than a P5 regression, which is the correct framing.

## 3. (B) `semio-framework-ui-contract --all-features` — the SIGABRT law

**Test**: `action::binding_copy_tests::retained_binding_copy_cancel_and_arena_contention_keep_exact_aliases`
(`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🔗️bindings/📋️copy/🧪️tests/📋️copy/🦀️.rs:26`), identified from
`📓️p1-contract.md` §5.2 as the test whose failure triggers a `Drop`-time abort that then takes ~11 other
`FAILED` names down with it in the same process.

With default (threaded) test execution the specific test that happens to hit the poisoned
global retirement arena and abort the process **varies by run** — confirmed in the pre-change build,
where a *different* test aborted first
(`document::document_component_compare_tests::retained_document_component_compare_final_reads_transfer_exact_root_to_one_retirement_owner`,
same cascade: `"UI value retirement arena is poisoned"` → `UiValueRetirement requires exact terminal
closure` → SIGABRT) — this is expected: `UiValueRetirement`'s arena is process-global, so whichever
test's `Drop` runs while it is already poisoned aborts next. Using `--test-threads=1` makes the target
test itself reach the same fault deterministically, in both trees:

| | Pre-change (`f7fef5746d`, `--test-threads=1`) | Current tree (`a4cda597ea`/HEAD, `--test-threads=1 --exact`) |
|---|---|---|
| `retained_binding_copy_cancel_and_arena_contention_keep_exact_aliases` | **Panics → SIGABRT**, deterministic | **Panics → SIGABRT**, deterministic, byte-identical |
| Panic 1 | `binding copy did not retire exact owners` at `🔗️bindings/📋️copy/🧪️tests/📋️copy/🦀️.rs:26:5` | identical text/line |
| Panic 2 (inside `Drop`) | `UiValueRetirement requires exact terminal closure` at `♻️retirement/🦀️.rs:158:9` | identical text/line |
| Panic 3 | `panic in a destructor during cleanup` → `signal: 6, SIGABRT` | identical |
| Tests completed before this one | 8 `ok`, 0 `FAILED` | (not re-measured cold; P1 §4 reports `64 ok, 12 FAILED of 180, then SIGABRT` under default threaded execution — consistent with the same underlying defect, different thread interleaving) |

Raw failure text (pre-change, deterministic):
```
thread 'action::binding_copy_tests::retained_binding_copy_cancel_and_arena_contention_keep_exact_aliases' panicked at
🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/../../🎬️action/../🔗️bindings/📋️copy/🧪️tests/📋️copy/🦀️.rs:26:5:
binding copy did not retire exact owners
thread '...' panicked at .../♻️retirement/🦀️.rs:158:9:
UiValueRetirement requires exact terminal closure
thread '...' panicked at .../panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
```
Re-run at the current tree (`cargo test -p semio-framework-ui-contract --all-features -- --test-threads=1
action::binding_copy_tests::retained_binding_copy_cancel_and_arena_contention_keep_exact_aliases --exact`)
reproduces this exact three-line cascade and SIGABRT, with matching file:line for all three panics.

P1 itself already bisected part of this (§5.2): forcing `UI_VALUE_PAGE_ROWS`/`UI_BUILT_CHILDREN_MAX`
back to their pre-change values on the **post-change** tree still failed, meaning the constants are not
the cause. This report adds the missing half of that bisection — the fully pre-change tree, constants
*and* struct fields both reverted, on the actual pre-change dependency graph — and gets the identical
failure, closing the open question P1 left ("a second bisect... never got a turn").

`git log --follow` shows the test file (`🔗️bindings/📋️copy/🧪️tests/📋️copy/🦀️.rs`) last touched at
`9b605a4550` (2026-09-09 12:10:01) and the contract's `♻️retirement/🦀️.rs` last touched at `8add1df147`
(2026-09-12 21:17:59) — both four to eight days before `a4cda597ea`.

**Verdict: pre-existing.** `UiBindingsCopy`/`UiValueRetirement`'s exact-terminal-closure invariant was
already violated by this test before `Component` grew from 2568 to 3096 bytes, before `TreeWindow`/
`granularity` existed at all, and before either capacity constant changed. This is not a P1/P5
regression; it is a live defect in the contract crate's binding-copy retirement path (P1's own §5.2
recommendation — investigate `🔗️bindings/📋️copy/🦀️.rs` and `♻️retirement/🦀️.rs`'s `Drop` — remains the
right next step, unrelated to this ticket).

## 4. Summary

Both observed test failures are **pre-existing**, reproduced identically (same panic text, same
file:line, same abort signature) by real builds of the source tree at `f7fef5746d` — the commit
immediately before this ticket's `TreeWindow`/`granularity`/`UI_BUILT_CHILDREN_MAX`/`UI_VALUE_PAGE_ROWS`
changes landed in `a4cda597ea`. Neither the widened `Component` (2568 → 3096 B), the new fields, nor the
raised capacity constants are implicated: the pre-change tree has none of them and fails the same way.
This directly answers (and slightly corrects) P1's own hedge in §5.1 ("a consequence of this dependency
change, not a peer edit") for law (A) — the real pre-change build shows it is neither: it is an
independent, older defect that the concurrent Component-growth work merely surfaced by running the
existing test suite at all.

## 5. Artifacts

- Scratch standalone workspace: `TICKET/🗑️generated/verify-retirement/pre/` (root `Cargo.toml`,
  `.cargo/config.toml`, and the extracted crate closure — kept for inspection, not cleaned up).
- Full threaded contract run log: `TICKET/🗑️generated/verify-retirement/pre-contract-run1.log`.
- Full `--test-threads=1` contract run log:
  `TICKET/🗑️generated/verify-retirement/pre-contract-full.log`.
