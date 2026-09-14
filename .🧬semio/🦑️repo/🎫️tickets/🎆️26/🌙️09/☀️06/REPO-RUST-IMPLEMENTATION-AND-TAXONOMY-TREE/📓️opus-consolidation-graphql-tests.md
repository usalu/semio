# 📓️ Opus — `🔗️graphql` Go test consolidation

Scope: the five failing tests in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🐹️go/🔬️_test.go`.
Environment: `GOWORK=C:/git/semio/go.work`, `RUSTC_WRAPPER=""`, Windows 11, go 1.25, bun.

**Only `🔬️_test.go` was edited.** No product code, no Rust crate, no fixture, no feature file and
no schema was touched. See §4 for the one product-level loose end I found and deliberately left.

## 1. Result

```
$ cd 🔗️graphql/📦️packages/🐹️go
$ gofmt -l .
$ go build ./...
build-ok
$ go vet ./...
vet-ok
$ go test -count=1 ./...
ok  	github.com/usalu/semio/repo/graphql	2.148s
```

No other Go module regressed:

```
$ cd ⌨️cli/📦️packages/🐹️go && go build ./... && go test -count=1 ./...
ok  	github.com/usalu/semio/repo/cli	7.797s
$ cd 🔌️mcp/📦️packages/🐹️go && go build ./... && go test -count=1 ./...
ok  	github.com/usalu/semio/repo/mcp	0.570s
```

Harness:

```
$ SEMIO_TEST_BUDGET_MS=600000 bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts parity fundamental --owner 🔗️graphql
[test] level=fundamental cases=8 executed=21 passed=21 failed=0 errored=0 parity=18/18
```

Baseline before this job (same command set): five failures —
`TestFixApplyAutofixes`, `TestFixViaRepoContext`, `TestGraphQLBundlesQuery`,
`TestGraphQLFixMutation`, `TestFixHeaderWithShebang`.

## 2. Per-test decisions

### 2.1 `TestFixApplyAutofixes` — repointed onto the surviving fixture, run in a temp copy

It read `repo/asset/fixture/some/folder/🧪️file-fixable/🟦️.tsx`, a path in the deleted legacy tree.
The surviving pair lives at
`🔨️modules/📜️statutes/🧫️fixtures/📁️some/📁️folder/{🧪️file-fixable,🧪️file-fixable-expected}/🟦️.tsx`
— the same golden the `📜️statutes` executor repointed its own tests to
(`📓️opus-go-align-tree-codebase-statutes.md` §4.2).

Two changes beyond the path:

1. **The policy id is narrowed to `code`.** Unnarrowed, `CheckPoliciesWithContext(ctx, nil)` runs
   the repository-walking policies too and returns nothing useful for a single-file scope; §5.3 of
   the align report is the precedent. Narrowed, the check yields the autofixable breachs the golden
   was recorded against.
2. **The fixture is copied into `t.TempDir()` and fixed there, not in place.** The old test mutated
   the checked-in fixture and restored it with a `defer`, which is a data race against every other
   agent working in this checkout and leaves the fixture corrupted if the process dies. The copy is
   written at the *same relative path* under the temp root, so language detection (`.tsx`), the
   emoji-prefixed basename and the scope string are all identical to the real tree. A side benefit:
   no `package.json`/`.prettierrc.json` exists under the temp root, so `applyAutofixes` never shells
   out to prettier and the comparison is deterministic.

The assertion is unchanged and non-vacuous: at least one autofixable breach must exist, at least one
fix must be applied, and the fixed bytes must equal `🧪️file-fixable-expected/🟦️.tsx` (the golden
differs from the input by exactly the `code/section/wrong-format/newline-after-region` repair, so a
no-op autofix fails the test).

New helpers: `statutesFixtureDir` (the fixture path constant) and `readStatutesFixture(t, folder)`,
which resolves the repo root from `runtime.Caller` rather than the process cwd.

### 2.2 `TestFixViaRepoContext` — **deleted**

It asserted `RepoContext.Fix(&scope)` returns a `*model.FixResult` whose `Remaining` equals
`len(Breachs)`. `RepoContext.Fix` is now a one-line stub
(`🐹️.go:1959`) that unconditionally returns
`fix was removed; handle autofix inside script.ts policy export`. The whole subject of the test is
the removed surface — there is no surviving path it could be routed onto, because the result type it
inspects is only ever produced by the removed function. This is the same call
`📓️opus-go-tests-triage.md` §3 made for the sibling `TestToolFixScope` ("the behaviour was removed
on purpose"), and a greenfield repo with no legacy support keeps no test for a deleted verb.
Its scope string `repo/go/main_test.go` was doubly dead — that path no longer exists either.

### 2.3 `TestGraphQLFixMutation` — **deleted**

Same reason. `mutation { fix(scope: …) { fixed remaining } }` routes straight into the same stub, so
the executor returns `graphql errors: [fix: fix was removed; …]`. Rewriting it to assert *that error*
would be a test of a placeholder string, not of behaviour. See §4 for the schema loose end this
exposes.

### 2.4 `TestFixHeaderWithShebang` — **rewritten onto the surviving path**

This one does assert something that still exists: that the autofixer does not displace a shebang
from line 1 of a Python file whose header region is malformed. It merely *routed* through
`RepoContext.Fix`. Rewritten onto `statutes.CheckPolicies` + `applyAutofixes`, which is exactly what
the removed `Fix` used to do internally and what the file's other twelve autofix tests already use.

It was also the weakest of the five: every original assertion was a `t.Logf` or a
`t.Log("No fixes applied (unexpected).")` — the test could not fail except through the `Fix` error.
It now genuinely asserts:

- at least one autofixable breach is detected in the fixture;
- `applyAutofixes` reports `fixed > 0`;
- the fixed content still *starts with* `#!/usr/bin/env python3\n` (the old check was a
  `strings.Contains`, which a shebang pushed to line 5 would have satisfied);
- the body `print("hello")` survives;
- re-running the check yields strictly fewer autofixable breachs than before the fix.

### 2.5 `TestGraphQLBundlesQuery` — moved off the developer's checkout onto a fixture monorepo

It expected the live repository walk to contain the bundle `compose/js`. That bundle is gone; the
current tree has `compose/client`, `compose/dev`, `coda/client` and the `🦑️repo` modules, and the
walk returned `"bundles": []` outright. A test that walks whoever's checkout it happens to run in is
the defect, not the expectation — it asserts against a moving target and cannot be made stable by
patching the expected string.

Rewritten onto a throwaway monorepo built in `t.TempDir()` by a new local
`withMonorepoFixture(t)` — the equivalent of `⌨️cli/📦️packages/🐹️go/🔬️_test.go`'s helper of the same
name (`📓️opus-go-tests-triage.md` §3), **reimplemented locally rather than imported**, since Go test
helpers do not cross module boundaries and the repo rule forbids exporting one. It writes three
technologies (`compose` user, `repo` infrastructure, `coda` research) and three bundles carrying
explicit `AGENTS.md` emojis, sets `workspace.RootDir`, invalidates the technology cache and restores
both in `t.Cleanup`. The two sectioned sources (`fixtureSectionedTypeScript`, `fixtureSectionedGo`)
are local copies of the cli fixture constants.

The test then builds its **own** executor with `NewExecutorWithContext(root, NewRepoContext(root))`
instead of using the package-level `executor`, which `TestMain` binds once to whatever
`workspace.RootDir` was at process start and which would therefore ignore the fixture root.

The assertion was strengthened from a `strings.Contains` on the raw JSON to a decode of the payload:
every bundle must carry a non-empty `id`, the three fixture bundles `compose/js`, `compose/go` and
`repo/client` must all be present, and the list must contain *exactly* three entries — so a walk
that leaks the surrounding filesystem fails.

The neighbouring `TestGraphQLRepoQuery`, `TestGraphQLPoliciesQuery` and
`TestGraphQLContributorsQuery` still use the live-checkout executor. They were passing and are out
of scope; `TestGraphQLRepoQuery`'s `strings.Contains(result, "compose")` is the same class of
checkout-dependent assertion and is worth the same treatment by whoever owns it next.

## 3. Files changed

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🐹️go/🔬️_test.go` — the only edit.

## 4. Product-level loose end (not fixed — outside the stated edit surface)

**The GraphQL schema still advertises a `fix` mutation whose resolver can only fail.**
`🐹️.go:4594` declares

```go
ResolvedField("fix", NN(Ty("FixResult")), "mutation.fix").Arg("scope", Ty("String")),
```

and `🐹️.go:6684` still carries `Fix *model.FixResult` in the mutation response struct, while
`RepoContext.Fix` (`🐹️.go:1959`) is a stub that always errors. Any client that introspects the
schema is told `fix` exists and gets an error on every call. The `FixResult` type, its `fixed` /
`remaining` fields and `model.FixResult` are likewise orphaned by the removal.

I did **not** remove them: the field lives in the schema builder, and the brief forbids schema edits.
This is a genuine inconsistency left behind by the `fix` removal and it belongs to whoever owns the
GraphQL schema surface — the correct end state is that `fix`, `FixResult`, `RepoContext.Fix` and the
mutation-response field all disappear together, since `applyAutofixes` / `applySystemAutofixes` are
now internal helpers with no public verb in front of them.

Two smaller observations, left alone:

- `findTestRepoRoot` (`🔬️_test.go:31`) still probes for the deleted marker `repo/client/main.go`
  before falling back to `.git`. The fallback is what fires today, so it is harmless, but the first
  probe is dead — and it is the same stale marker `📓️opus-go-tests-triage.md` §3 recorded for
  `🏠️workspace`.
- `applyAutofixes` sorts its `breachs` slice in place (`sort.Slice` by descending line) — it mutates
  the caller's slice. Not a bug for any current caller, just surprising.

## 5. Cleanup

Working drafts lived under
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE/🗑️generated/consolidation/`
and were deleted at the end of the job.
