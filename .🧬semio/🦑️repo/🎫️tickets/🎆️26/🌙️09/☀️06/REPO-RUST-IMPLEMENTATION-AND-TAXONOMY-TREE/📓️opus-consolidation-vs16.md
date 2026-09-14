# 📓️ Consolidation — VS16 region markers in `collect_go_tests_in_section`

## Divergence

The Rust `collect_go_tests_in_section` stripped **one leading rune**, not one grapheme cluster:

```rust
let mut region = trimmed[index + "#region".len()..].trim().to_string();
for rune in region.clone().chars() {
    if (rune as u32) > 0x7F {
        region = region[rune.len_utf8()..].trim().to_string();
        break;
    }
}
```

For `// 🧪️#region 📷️Alpha` — emoji + `U+FE0F`, the spelling every real region marker in this repo
uses — the orphan `U+FE0F` survived. `flat()` keeps non-ASCII runes, so the section never matched
and `plan_section` returned an empty run pattern (and therefore a refusal). The loop also sliced from
the *start* of `region` rather than from the offset of the non-ASCII rune, so a marker with any
leading ASCII would have panicked or cut the wrong bytes.

Go is the decided contract: `StripLeadingGrapheme` drops the leading non-ASCII rune together with
the variation selectors (`U+FE00..U+FE0F`), skin-tone modifiers (`U+1F3FB..U+1F3FF`), combining
marks (`U+0300..U+036F`), the keycap `U+20E3`, and the runes joined to it by ZWJ (`U+200D`).

## Fix

`📦️packages/🦀️rust/🦀️.rs`

- Added `pub fn strip_leading_grapheme(value: &str) -> String`, a rune-for-rune port of Go
  `StripLeadingGrapheme`: same leading-ASCII skip, same `if index >= len { return value }` escape,
  same modifier classes in the same order, same ZWJ double-advance, same trailing `trim`.
- `collect_go_tests_in_section` now calls it, replacing the one-rune loop.

`🧫️fixtures/🗺️planning-vectors.json`

- New snapshot entry `/repo/bundles/go-core/pkg/decorated_test.go` whose region header is
  `// #region 🚀️Gamma` (`U+1F680` + `U+FE0F`); verified on disk as `['0x1f680', '0xfe0f']`.
- New vector `section-name-drops-a-leading-emoji-with-a-variation-selector` (scope `section`,
  section `Gamma`), expecting exactly the bare-emoji vector's plan:
  `go test -v -run ^(TestGammaOne)$ ./...` in `/repo/bundles/go-core`.

No new scenario was needed: `🧪️tests/🗺️invocation-planning` is the case that drives
`plan_section` → `collect_go_tests_in_section` (`📊️result-parsing` does not), and both its
adapters iterate `fixture.vectors` automatically in `scope-plans-match-the-frozen-argv`
(`🐹️.go` `for _, vector := range fixture.Vectors`, `🦀️.rs` `for vector in &fixture.vectors`).
No test-case directory was created or renamed.

## Verification

### Go

```
$ cd 📦️packages/🐹️go && gofmt -l . && go build ./... && go vet ./... && go test -count=1 ./...
ok  	github.com/usalu/semio/repo/testrunner	0.451s
```

### Rust

```
$ cargo test -p semio-framework-repo-test-runner
   Compiling semio-framework-repo-test-runner v0.1.0
    Finished `test` profile [unoptimized] target(s) in 1.73s
     Running unittests 🦀️.rs
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
   Doc-tests semio_framework_repo_test_runner
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

```
$ cargo clippy -p semio-framework-repo-test-runner --all-targets
warning: this `if` can be collapsed into the outer `match`
warning: `semio-framework-repo-workspace` (lib) generated 1 warning
```

The single finding is `clippy::collapsible_match` in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🦀️rust/🦀️.rs` — a dependency
crate outside this task's edit scope, currently being edited by a concurrent agent (its line number
moved from 557 to 673 between two runs, and one run caught it mid-edit with an unbalanced brace).
`semio-framework-repo-test-runner` itself produces zero clippy findings.

### Parity

```
$ SEMIO_TEST_BUDGET_MS=600000 bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts parity fundamental --owner 🏃️test-runner
[test] level=fundamental cases=5 executed=27 passed=27 failed=0 errored=0 parity=18/18
```

(`--owner ./🧰️framework/.../🏃️test-runner` selects nothing — `--owner` matches path *segments*, so
the bare segment `🏃️test-runner` is the correct spelling. An earlier run reported
`executed=24 … parity=15/15` only because the concurrent workspace-crate edit left that crate
uncompilable and the `🛑️cancellation` Rust host could not build; the run above is clean.)

The scenario count is unchanged by design — the vector table is iterated, so a new vector adds a row
to `scope-plans-match-the-frozen-argv` rather than a scenario. Both implementations emit the new row
and agree on it:

`…/invocation-planning-subject-rust/scope-plans-match-the-frozen-argv.subject.projection.json`

```json
{"id": "section-name-drops-one-leading-emoji", "plan": {"invocations": [{"runner": "go", "argv": ["go","test","-v","-run","^(TestGammaOne)$","./..."], "cwd": "/repo/bundles/go-core", "env": {}, "filter": "^(TestGammaOne)$"}], "problems": []}}
{"id": "section-name-drops-a-leading-emoji-with-a-variation-selector", "plan": {"invocations": [{"runner": "go", "argv": ["go","test","-v","-run","^(TestGammaOne)$","./..."], "cwd": "/repo/bundles/go-core", "env": {}, "filter": "^(TestGammaOne)$"}], "problems": []}}
```

The same two rows appear in the `-subject-go` projection, and `parity=18/18` confirms the Go and
Rust projections are byte-identical.

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🏃️test-runner/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🏃️test-runner/🧫️fixtures/🗺️planning-vectors.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/📓️opus-consolidation-vs16.md`
