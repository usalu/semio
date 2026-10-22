# Authenticated Closure Recovery Regression Proposal

Read-only static inventory; no Source functions or tests executed and no cleanup performed.

## Existing owners and test selection

Paths below are repository relative. Owner prefix: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs`.

- `💾️storage/🧬️schema/🔣️.json`: closed storage namespaces and OptionalQuota, ScopedOptionalRecoveryCase/NativeCase/AncestorCase contracts.
- `💾️storage/🧪️tests/♻️scoped/🧫️cases.json` and `💾️storage/🧪️tests/♻️scoped/🟦️.ts`: language-neutral recovery rows, independent Ajv/SQLite expectations, physical canonical image plus actual busy namespace lease test.
- `💾️storage/🧪️tests/💧️headroom/🔣️.json` and `💾️storage/🧪️tests/💧️headroom/🟦️.ts`: closed reclamation pressure, allocated counts, cancellation and queued-wrapper vectors. These isolate actual private function bodies through AST ports; they do not prove production enumeration includes closures.
- `🪶️lean-inventory/🧪️tests/🔣️.json`, its `🧬️schema/🔣️.json`, and `🟦️.ts`: authenticated hash-named inventory leaves, Python SHA oracle, physical identity refusal/replacement coverage.
- Graph headroom owner `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom` has schema plus `🧪️tests/🔣️.json`/`🟦️.ts` pressure and actual image tests.

Actual `💾️storage/📜️script.ts` TestScript accepts no arguments and runs storage main tests, storage ledger tests and graph headroom tests via runBudgetedTestCommand. Storage main imports scoped tests, so scoped physical recovery is registered on that route. Do not assume every sibling subdirectory is automatically selected by the script.

## Production construction and gap

`inputs/🟨️.mjs` cargoLockInput (around79) captures actual Cargo inventory, hashes exact contract text and publishes hash-named JSON in build-lock-closures. Source closure construction (around139–145) captures exact file/helper roster, hashes contract text and publishes in build-input-closures. `🧮️digest/📥️read/🟨️.mjs` managed Source batches retain graph consumer custody and read only closed namespaces/key identities. These are production constructors/readers to exercise, rather than writing arbitrary valid-looking closure JSON alone.

Existing global optional inventory walks only inference. Its optional filename grammar does not admit either top-level closure namespace. Namespace acceptance and ledger classification vectors alone therefore cannot prove closure retirement coverage or safe reference lifetime.

## Minimal schema-first addition

Add one closed closure-recovery corpus to this storage owner, then test the actual production selector and retirement callback under existing issued canonical image custody. Corpus must distinguish authenticated unreachable closure from reachable closure (including transitive/shared reference), active producer/writer, retained reader, malformed/hash-mismatched/unknown leaf, symbolic/hard link, ancestor or leaf replacement, cancellation and insufficient eligible space. Reference roots must represent the real production graph/selection lifetime established by the owner, not merely filenames or age. Failure to authenticate a root/reference must retain affected data, not declare it unreachable.

Use actual closure constructors for representative Source and Cargo-lock cases, retaining controlled ports explicitly where construction needs source fixtures. Independent SQLite tables of roots/edges plus recursive CTE compute expected reachable keys and deletion complement. Independent SHA256 over exact contract bytes checks leaf identity; real lstat blocks and actual statfs before/after check allocated accounting/headroom. A separate process SQLite lock probe already used in scoped native tests proves actual busy writer/image-reader preservation. Native physical test must exercise the genuine exported callback, reject cloned/forged custody, and show retained contents and valid reads after attempted recovery. No blind unlink or test-only selector can substitute.

## Neutral phase fit

Current combined.cases.json has19 phases; none selects storage or graph-headroom tests. combined.ts fixes one path per Source phase, exact anchored title pattern, one pass count equal to selected titles. Consequently appending a storage title to an existing phase cannot collect this test. Clean direct coverage needs an explicitly reviewed storage phase and corresponding schema/receipt/collection/Source witness updates. Keeping19 phases is technically possible only by deliberately adding a real invocation/assertion inside an already selected test file/title (for example existing Cargo inference constructor witness), with the additional actual dependencies pinned; this is a test change, not automatic sibling discovery. I recommend the owner-local regression and explicit phase rather than hiding a distinct maintenance contract inside the Cargo inference title.

No tests passed or were run by this audit.
