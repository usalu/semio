# 📓️ Divergence item (g): metrics numstat + statutes TrimLeft

## (g1) 📊️metrics numstat parsing — already converged, no change needed

`📦️packages/🐹️go/🐹️.go` already carries the same contract as `📦️packages/🦀️rust/🦀️.rs`:

| Concern | Rust | Go | Same? |
| --- | --- | --- | --- |
| Octal unquoting | `unquote_git_path` (l. 273) | `UnquoteGitPath` (l. 318) | ✅ identical escape table (`n t r a b f v " \`, 1–3 octal digits, byte-wise then `from_utf8_lossy` / `string([]byte)`) |
| Rename `old => new` | `resolve_numstat_path` (l. 328) | `ResolveNumstatPath` (l. 373) | ✅ |
| Rename brace form `pre/{old => new}/tail` | handled, then `collapse_slashes` | handled, then `collapseSlashes` | ✅ |
| Binary `-`/`-` rows | record kept, `binary: true`, counts zeroed | same | ✅ |
| Merge commit | `COMMIT` line with no file rows ⇒ record with empty `files` and empty `delta` | same | ✅ |
| Quoting approach | **no** `-c core.quotepath=off`; the parser decodes instead | `NumstatLogArgs` is likewise `log --no-merges --numstat --first-parent --reverse --pretty=…` with no `-c` | ✅ same approach |
| `COMMIT` field indices | `strip_prefix("COMMIT")` + `splitn(5,'\t')` ⇒ `[ "", sha, name, mail, at ]` | `SplitN(line,"\t",5)` ⇒ `[ "COMMIT", sha, name, mail, at ]` | ✅ same indices 1..4 |

### Vectors
`🧫️fixtures/🌱️repository-recipe.json` + the recorded `🎞️git-transcript.json` already pin both defects — the
recorded `--no-merges` stream contains, verbatim:

```
2	1	"\360\237\247\254\357\270\217alpha.ts" => "src/\360\237\247\254\357\270\217beta.ts"
```

which is **one row that is both a cross-directory rename and two octal-quoted emoji paths**, plus
`1	0	".\360\237\247\254semio/\360\237\246\221\357\270\217repo/note.md"` and a binary row `-	-	assets/logo.png`.
`🥒️.feature` scenario `resolves-renames-and-quoted-paths` (`@level-fundamental @mode-differential`) is
registered in all three adapters — `🐹️.go:113`, `🦀️.rs:66`, `🟦️.ts:231` — so both implementations
exercise them. No fixture or adapter change was required.

## (g2) 📜️statutes `requiresDefinitionRequirements` TrimLeft cutset bug — fixed in both

Go had `strings.TrimLeft(noExport, "async abstract declare default ")`, a **cutset** of
`{a,b,c,d,e,f,l,n,r,s,t,u,␠}` that eats `funct` out of `function ` and `class ` entirely, so
`code/definition/missing-requirements` could never fire for TypeScript. Rust replicated the bug with
`trim_start_matches(['a','s','y','n','c',' ','b','t','r','d','e','l','f','u'])`.

Both now strip the modifier words as whole space-separated **prefix** words, repeatedly:

- Go: `definitionModifierWords` + `stripDefinitionModifierWords` in `📦️packages/🐹️go/🐹️.go`
- Rust: `DEFINITION_MODIFIER_WORDS` + `strip_definition_modifier_words` in `📦️packages/🦀️rust/🦀️.rs`

### Golden regenerated: 69 → 71
`🧪️tests/🔍️analyze-breaches/🧫️fixtures/🔣️breaches.json` regenerated with the ticket's
`🏗️statutes-golden` helper. Exactly two entries added, both in `🧫️fixtures/📁️some/📁️folder/🧪️file/🟦️.tsx`:

- `…🧪️file/🟦️.tsx::TestComponent#31 | code/definition/missing-requirements | line 31 | autofixable`
- `…🧪️file/🟦️.tsx::TestClass#35 | code/definition/missing-requirements | line 35 | autofixable`

Nothing removed. No scenario/feature/adapter text states the count 69 (`🥒️.feature` describes the list,
the Go adapter `🐹️.go:97` compares `len(golden.Breachs)` against the fixture), so no count literal
needed updating. The `the-clean-sources-are-clean` scenario still yields an empty list.

### Extra: clippy
- `📜️statutes/📦️packages/🦀️rust/🦀️.rs` had a pre-existing `needless_lifetimes` on `find_header` blocking
  `-D warnings`; elided.
- `📊️metrics/📦️packages/🦀️rust/🦀️.rs` had 10 pre-existing lints (`map(..).unwrap_or(..)` ×9, one
  `redundant_clone`); applied `cargo clippy --fix --lib`.
- **Out of scope, still failing:** `🏠️workspace/📦️packages/🦀️rust/🦀️.rs:674` raises
  `clippy::collapsible_if` ("this `if` can be collapsed into the outer `match`"). It is a dependency of
  `semio-framework-repo-metrics`, so `cargo clippy -p semio-framework-repo-metrics -- -D warnings`
  cannot go green until the `🏠️workspace` crate is fixed by whoever owns it.

## ✅️ Verification output

```
$ cd 📊️metrics/📦️packages/🐹️go && gofmt -l . && go build ./... && go vet ./... && go test -count=1 ./...
?   	github.com/usalu/semio/repo/metrics	[no test files]

$ cd 📜️statutes/📦️packages/🐹️go && gofmt -l . && go build ./... && go vet ./... && go test -count=1 ./...
ok  	github.com/usalu/semio/repo/statutes	0.605s

$ cargo test -p semio-framework-repo-metrics
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test -p semio-framework-repo-statutes
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo clippy -p semio-framework-repo-statutes --all-targets -- -D warnings
    Checking semio-framework-repo-statutes v0.1.0 (…📜️statutes\📦️packages\🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 1.13s

$ cargo clippy -p semio-framework-repo-metrics --all-targets   # metrics crate itself clean
warning: this `if` can be collapsed into the outer `match`     # ← 🏠️workspace crate only, out of scope

$ SEMIO_TEST_BUDGET_MS=600000 bun …/🧪️test/📜️script.ts parity fundamental --owner "📊️metrics"
[test] level=fundamental cases=4 executed=23 passed=23 failed=0 errored=0 parity=19/19

$ SEMIO_TEST_BUDGET_MS=600000 bun …/🧪️test/📜️script.ts parity fundamental --owner "📜️statutes"
[test] level=fundamental cases=5 executed=18 passed=18 failed=0 errored=0 parity=9/9
```

Note: `--owner ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics` selects **0 cases** — the flag
matches owner path segments, so the leading `./` makes it match nothing. `--owner "📊️metrics"` is the
form that works.

## 🗂️ Files touched
- `📊️metrics/📦️packages/🦀️rust/🦀️.rs` (clippy only)
- `📜️statutes/📦️packages/🐹️go/🐹️.go`
- `📜️statutes/📦️packages/🦀️rust/🦀️.rs`
- `📜️statutes/🧪️tests/🔍️analyze-breaches/🧫️fixtures/🔣️breaches.json`
