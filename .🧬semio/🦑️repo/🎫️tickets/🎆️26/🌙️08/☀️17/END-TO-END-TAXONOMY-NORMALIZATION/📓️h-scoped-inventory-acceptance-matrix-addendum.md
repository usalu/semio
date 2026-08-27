# Scoped Inventory Acceptance Matrix Addendum

## Scope

This test-only packet edits only repo-lib `🧪️index.test.ts`. Every exercised repository is created below this ticket and owns its own `.git` directory. No production normalizer, root script, taxonomy, real opaque tree, or repository Git state was changed.

## Active results

Command:

```text
bun test --timeout 60000 <repo-lib 🧪️index.test.ts> -t 'exact symlink leaf|standalone ignored-generator admission|standalone explicit-ticket admission|inventory cancellation|creation order'
```

Result on Bun 1.3.14: 4 passed, 1 failed, 262 filtered, 18 assertions, 9.25 seconds.

- Passed: ignored generator output is admitted independently of ordinary ignored-untracked enumeration and matches the unscoped physical-leaf census after scope filtering.
- Passed: explicit ticket evidence is admitted independently and matches the unscoped physical-leaf census after scope filtering; removing ticket authority excludes it.
- Passed: cancellation is observed from the `0/*` boundary of every nonterminal frozen phase: `setup`, `tracked-enumeration`, `untracked-enumeration`, `ignored-generator-admission`, `explicit-ticket-admission`, `directories`, `files`, and `references`.
- Passed: reversing untracked leaf creation order preserves exact canonical inventory bytes, event bytes, digests, and phase sequence.
- Blocked before production assertion: the symlink fixture is an `lstat`-confirmed symlink, but the fixture's post-commit `git ls-files --stage -- .` assertion returned no row for it. The test therefore did not establish a production exact-leaf failure and must not be reported as one. The focused rerun reproduced this same fixture/index failure with 0 passed, 1 failed, 266 filtered, and 2 assertions in 13.63 seconds.

The previously present language-neutral pathspec golden, literal-metacharacter/NFC-NFD scope, third-party no-follow physical census, and closed phase-order test also passed in the first matrix run.

## Explicitly deferred tests

Four acceptance rows remain visibly `test.skip`, with these exact reasons:

1. Symlink-ancestor fallback: the Git TRACE2 capture helper produced no fixture-local trace file, so fallback argv was not proved.
2. Mode-160000 gitlink-ancestor fallback: the same missing TRACE2 evidence prevents an exact fallback-argv claim; nested content was not accepted as proven unread.
3. Exact unscoped argv/canonical regression: the same missing TRACE2 evidence prevents the exact argv assertion, although ordinary canonical repeatability already passes elsewhere.
4. Root CLI stream purity: the isolated `CleanScript` child exited zero but emitted empty stdout, so the harness did not prove canonical JSON stdout; this is not classified as a production CLI defect without a real dispatched-root reproduction.

Per the frozen instruction to stop at the first exposed blocker rather than edit production, no production code was changed and the deferred rows were not weakened into indirect assertions.

## Root closure

The root coordinator removed the fixture-only blockers without changing production behavior:

- force-admitted the isolated symlink index row and retained the no-follow content-hash proof;
- replaced unavailable TRACE2 side-channel assertions with semantic fallback proofs: a below-symlink scope returns only the indexed symlink ancestor, and a below-gitlink scope returns the mode-`160000` ancestor without nested content;
- verified exact unscoped pathspec rendering through the language-neutral pure builder and canonical bytes through two inventories;
- invoked the CLI harness through its actual `taxonomy inventory` route and proved canonical JSON-only stdout plus phase-only stderr.

Final focused command:

```text
bun test --timeout 120000 './🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts' --test-name-pattern 'exact symlink leaf|scope below a symlink ancestor|mode-160000 ancestor|standalone ignored-generator|standalone explicit-ticket|inventory cancellation|unscoped inventory retains|reversed creation order|root taxonomy inventory CLI'
9 pass, 0 fail, 33 expect() calls, 14.06 s
```

No row remains skipped. The acceptance matrix now proves all four formerly deferred behaviors in disposable repositories.
