# rust-path-join / rust-path-join-unproven — root cause and fixes

## Method
Instrumented `rustFiniteManifestTargets` in `🧹️normalization/🟦️.ts` (temporary `[DEBUG]` guards,
env-gated, removed after diagnosis) and ran a scoped `clean taxonomy plan` for `🌉️abi` with debug
targeting the pdf `✳️a` mutations file, per the "instrument the actual guard" method. Debug run
showed the file passing every whole-file guard until: `guard=macro-trust source=…/🧬️schema/🦀️component.rs
hasHashBang=true`. That ancestor file (which does `#[path="🧬️mutations/🦀️.rs"] pub mod mutations;`)
contains two untrusted constructs whose `!`/`#` survive `rustCodeOnlyTextForMacroTrust`'s scrub and
trip the raw `/[#!]/` disqualifier:

1. `semio_framework_plugin::derive_artifact_facets!(...)` — a macro invocation not in
   `RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS`. Read its full `macro_rules!` body
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:37190-37277`, both the primary
   arm and the two `@children_ty` dispatch arms): zero `mod` tokens anywhere — only `struct`/`impl`
   items via `$crate`-qualified paths. Called 145x repo-wide; grep confirms 29 subset directories
   have both a `derive_artifact_facets!` caller and a descendant file needing `CARGO_MANIFEST_DIR`
   proof, so this alone was likely blocking dozens of rows repo-wide, not just the 2 pdf files.
2. `assert_eq!`/`assert!` inside the same ancestor's `#[cfg(test)] mod tests { ... }` block — not in
   `RUST_MODULE_STRUCTURE_TRANSPARENT_STD_EXPRESSION_MACROS` (only `format`/`unreachable` were).
   These are expression-position-only by language guarantee (same class already trusted for
   `format!`), and appear in nearly every ancestor file with tests, so this was disqualifying far
   more broadly than just this file.

Separately, `🧰️framework/🔨️modules/⚠️diagnostic/🦀️component.rs` and `✍️editor/🦀️component.rs`'s
4 rows each turned out to be a different gap: `rustStringCollectionJoinArguments`
(`🔍️discovery/🟦️component.ts`) only recognized `.join()` on a named `let`-bound `Vec<String>`
receiver. It never recognized the far more common idiom `<iter>.collect::<Vec<_>>().join(literal)`
chained inline with no intermediate binding (editor's exact construct, 4/4 rows). This is
structurally provable without binding-tracking: `.collect::<Vec<...>>()` can never produce a
`Path`/`PathBuf`, so a `.join()` chained directly onto it can never be `Path::join`.

The `w18-…/🏗️vector-converter/src/main.rs` file (7 rows, `🌉️abi` scope) is a **correct refusal, not
a bug** — it has no `CARGO_MANIFEST_DIR` anywhere; every `.join()` is off a `&Path` function
parameter (`target`, from a runtime directory walk), never a manifest-relative literal. No static
proof is possible. Left unchanged.

## Changes landed (`🧹️normalization/🟦️.ts`)
- `RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS`: added
  `"semio_framework_plugin :: derive_artifact_facets !"` (verified zero `mod` tokens in its body).
- `RUST_MODULE_STRUCTURE_TRANSPARENT_STD_EXPRESSION_MACROS`: added `panic`, `assert`, `assert_eq`,
  `assert_ne`, `debug_assert`, `debug_assert_eq`, `debug_assert_ne` (all expression/statement-position
  only by language guarantee).
- Split the conflated `targets?.length !== 1` diagnostic (line ~4677, was line 4298 pre-shift) into
  two messages: `"...was never admitted into a proven physical source chain"` (targets undefined —
  never reached the finite map) vs `"...resolved to N distinct physical targets, not exactly one"`
  (targets defined, genuine 0-or-many ambiguity). This is the fix the ticket explicitly wanted
  regardless of row count — the conflation caused all three prior wrong diagnoses in §8.

## Changes landed (`🔍️discovery/🟦️component.ts`)
- `rustStringCollectionJoinArguments`: added a second, binding-free structural pass detecting
  `<expr>.collect::<Vec<...>>().join("literal")` chained inline. Backward-scans the turbofish via
  bracket-depth counting (handles the `>>` single-token lexing of nested `Vec<_>>` correctly),
  requires the outer turbofish type identifier to resolve to `Vec`/`std::vec::Vec` via the existing
  `standard()` shadow/wildcard-aware check. Purely additive — unions into the existing result Set,
  never removes an existing detection.

## What I deliberately did NOT change
- The `visit()`/`bindings` machinery in `rustStringCollectionJoinArguments` (struct-field access,
  e.g. `self.keywords.join("|")` in `⚠️diagnostic`, 2 of its 4 rows) — no local-variable type
  information exists for `self.<field>`; extending this needs cross-referencing the enclosing
  `impl`'s target struct definition, which is real added scope/risk (false-positive risk against a
  `PathBuf`-typed field elsewhere) I did not want to rush. Left as a correct refusal.
- `w18-…/main.rs` (7 rows) — genuinely unprovable, see above. Not touched.
- The regression canary `independent-map-in-for-of-source-conservatively-suppressed` — not touched,
  not exercised by anything in this diff.
- Did not add a new fixture case to
  `📦️packages/🟦️typescript/🧫️fixtures/🧪️rust-physical-reference-context/🔣️.json` for the
  collect-chain construct: that fixture is jointly validated against a `syn`-based Rust oracle binary
  (`golden.oracle.sourceInput`) via the `"independent syn parsing..."` test, and adding a case
  correctly requires updating that oracle's Rust source too — under this turn's time budget I chose
  not to touch a shared multi-oracle fixture file blind. Verified the actual construct behavior
  directly instead (see below). This is a real coverage gap, flagged for follow-up.

## Verification methodology correction
My first comparison used `git show HEAD:<path>` as "baseline" — wrong, HEAD is stale relative to
already-landed concurrent work in the working tree (confirmed via diff: HEAD lacks an already-fixed
`impl X for Y` for-loop misparse and a typed-`Vec<String>`-binding fix). Redid it correctly by
diffing the CURRENT working tree against a copy with ONLY my own hunks removed.

## Test output (`bun test`, this file only, ~13-14s)
Corrected baseline (current tree, my 3 hunks removed): **9 pass / 23 fail**.
Mine (current tree, as landed): **9 pass / 23 fail** — identical.
All 23 failures are pre-existing and environment-caused (rustc/cargo subprocess spawns returning
empty stdout in this sandbox — reproduces identically with or without my change; unrelated to this
diff). Direct unit-level check (`inspectRustJoinArgumentSpans` called directly, bypassing the
subprocess-heavy suite) confirms the actual fix:
```
source: fn f() { let lines: String = (0..20).map(|i| format!("line {i}")).collect::<Vec<_>>().join("\n"); }
without my change: ["\\n"]   (incorrectly flagged as unproven path-join)
with my change:    []        (correctly excluded as a string-collection join)
```
Typecheck: `bun build --no-bundle --target=node` transpiles both edited files with zero errors.

## Ranking / expected impact
`derive_artifact_facets!` trust fix: touches 29+ subset directories repo-wide (not just the 2 pdf
`✳️a`/`✳️x` rows named in the brief) — highest-value fix in this slice. `assert*!` trust fix: likely
broader still (near-universal `#[cfg(test)]` pattern) but unquantified — no full-tree scan was run
per the new no-plan-runs contract. Collect-chain fix: clears `✍️editor`'s 4/4 rust-path-join-unproven
rows and 1 of `⚠️diagnostic`'s 4 (3 remain, correctly refused). Message split: zero row impact by
design, landed because the ticket asked for it regardless.
