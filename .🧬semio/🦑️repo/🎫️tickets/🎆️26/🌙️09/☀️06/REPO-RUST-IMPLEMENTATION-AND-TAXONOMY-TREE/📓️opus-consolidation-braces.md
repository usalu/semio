# 🃏️ Consolidation — brace alternation in the owned glob matcher

Divergence item (c): `workspace.Match` had no brace alternation in either implementation, so
`**/*.{ts,tsx,py,cs,go,rs}` matched nothing. `📜️statutes.matchesScope` routes every file-scoped
policy through that matcher, so `CheckPolicies` with no explicit policy id selected zero policies for
any file scope — the `analyze` verb over a single file was silently empty.

## Contract now implemented, identically, in Go and Rust

- A brace group expands to its alternatives; the pattern matches when **any** expansion matches.
- Nesting works, and a comma binds to its **innermost** enclosing group (`{a,{b,c}}` → `a`, `b`, `c`).
- A group **without a top-level comma** is literal (`{a}.go` matches `{a}.go`, not `a.go`).
- An **unpaired** `{` or `}` is literal (`a{b.go` matches `a{b.go`; `{a.go` never matches `a.go`).
- An **empty alternative** stands for the empty string (`a{,-b}.go` matches both `a.go` and `a-b.go`).
- A comma inside a character class belongs to the class, not to the group.
- Expansion is capped at **1024** patterns (`maxBraceExpansions` / `MAX_BRACE_EXPANSIONS`); a pattern
  that would expand past it falls back to the original pattern with its braces **literal**. `2^10`
  groups expand; `2^11` fall back. Both implementations agree on the boundary.
- **Escaping is not a portable spelling.** Both implementations normalise separators before reading a
  pattern (`filepath.ToSlash` in Go, `to_slash` in Rust), and that rewrite turns `\` into `/`. The
  brace scanners are escape-aware and run on the raw pattern, so `\{` survives wherever the
  normalisation is a no-op (Go on POSIX) and nowhere else. No escaping was invented; the docstrings
  on `Match` and `Pattern::compile` state the limitation.

Every rule above was checked against `micromatch` (the case's declared oracle) before it was pinned.
The one micromatch quirk deliberately **not** pinned is picomatch returning `true` when the input
string equals the pattern verbatim (`isMatch("{a,b}", "{a,b}")`), which is an oracle artefact rather
than brace semantics.

## Changes

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🐹️go/🐹️.go`
  `Match` now iterates `expandBraces`; new `expandBraces`, `braceGroup`, `braceEnd`,
  `braceAlternatives`, `maxBraceExpansions`. `compile` is untouched, so the compiled-pattern cache
  still memoises one brace-free pattern per entry.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🦀️rust/🦀️.rs`
  `Pattern` now holds `alternatives: Vec<Vec<Token>>`; the old inline compile body was extracted to
  `compile_tokens`. New `expand_braces`, `brace_group`, `brace_end`, `brace_alternatives`,
  `MAX_BRACE_EXPANSIONS`. Because `Pattern` is what the ignore reader compiles, both implementations
  gain braces on the same surfaces (`glob_match`, ignore rules, traversal) rather than only on
  `Match`. Also collapsed a pre-existing `collapsible_match` clippy error in the `📋️config.toml`
  reader (`"detail" if !value.is_empty() =>`) that was blocking `-D warnings`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/🧫️fixtures/📡️glob-vectors.json`
  New `braceVectors` array, 21 vectors, all agreeing with micromatch.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/🧪️tests/🃏️glob-matching/🥒️.feature`
  New scenario `@id-brace-alternation-expands-the-same-way` `@level-fundamental` `@mode-differential`
  `@seed-1`, alongside the existing one (existing case directory extended, none created).
- The same case's `🐹️.go`, `🦀️.rs` and `🟦️.ts` adapters register the new scenario; the TypeScript
  oracle's verdict rendering was factored into one `globVerdicts(ctx, set)` helper shared by both
  scenarios.

## Verification (real output)

```
$ cd .../🏠️workspace/📦️packages/🐹️go && go build ./... && go vet ./... && go test -count=1 ./...
ok  	github.com/usalu/semio/repo/workspace	40.770s
```

```
$ cargo test -p semio-framework-repo-workspace
running 4 tests
test tests::missing_config_yields_defaults ... ok
test tests::ignore_precedence_is_last_rule_wins ... ok
test tests::logging_section_overrides_defaults ... ok
test tests::double_star_spans_directories ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
   Doc-tests semio_framework_repo_workspace
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```
$ cargo clippy -p semio-framework-repo-workspace --all-targets -- -D warnings
    Checking semio-framework-repo-workspace v0.1.0 (...)
    Finished `dev` profile [unoptimized] target(s) in 1.28s
```

```
$ SEMIO_TEST_BUDGET_MS=600000 bun .../🧪️test/📜️script.ts parity fundamental --owner <owner>
🏠️workspace  [test] level=fundamental cases=5 executed=17 passed=17 failed=0 errored=0 parity=13/13
📜️statutes   [test] level=fundamental cases=5 executed=18 passed=18 failed=0 errored=0 parity=9/9
🗂️codebase   [test] level=fundamental cases=4 executed=16 passed=16 failed=0 errored=0 parity=8/8
```

`--owner ./🧰️framework/…/🏠️workspace` selects **zero** cases: `📜️script.ts` matches `--owner` against
owner path *segments*, so the bare segment name (`🏠️workspace`) is the spelling that works. The first
run with the full relative path reported `cases=0 executed=0` and is not a result.

Nothing in `📜️statutes` or `🗂️codebase` relied on the broken matcher: both were green before and after,
and no file outside `🏠️workspace` was edited.

## End-to-end consequence, confirmed at runtime

A throwaway `[DEBUG]` probe against the exact strings `matchesScope` feeds the matcher
(`workspace.NormalizePath` on both sides), removed afterwards:

```
[DEBUG] scope "js/sketchpad/src/app.ts" matched=true err=<nil>
[DEBUG] scope "a/b/m.rs" matched=true err=<nil>
[DEBUG] scope "main.go" matched=true err=<nil>
[DEBUG] scope "x/y.py" matched=true err=<nil>
[DEBUG] scope "x/y.cs" matched=true err=<nil>
[DEBUG] scope "x/y.tsx" matched=true err=<nil>
[DEBUG] scope README.md matched=false
```

The default statute scope now selects source files and still rejects everything else, so
`CheckPolicies` over a single file is no longer empty.

## Observed, not touched

`🐹️.go` is not `gofmt`-clean at line ~1373 (a missing blank line before `GetGitIgnoredSet`), far from
this change and inside another agent's in-flight edit. Left alone to avoid clobbering concurrent work.
The Rust file was already not `rustfmt --check`-clean before this change (the crate uses long lines);
new code follows the surrounding style and `clippy -D warnings` is clean.
