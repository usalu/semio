# FND-TERMINALITY-02 Evidence

## Scope

Implemented the bounded mutation-taxonomy terminality packet in the root workflow, its repository-library TypeScript regression suite, and the language-neutral inventory fixture. No production mutation apply command was run. Every apply exercise used an isolated temporary test repository and ticket directory.

## Behavior Closed

- Mutation inventories now fingerprint a deterministic, byte-sorted, no-follow roster of the live files under each discovered mutation root as canonical `{ roots, records: [{ path, sha256(bytes) }] }` data. Opaque `compose` paths are excluded before traversal; symlinks are rejected. Start/end source snapshots must match; one optimistic retry is allowed before failing closed.
- A direct-shaped record with a live violation is represented as unresolved. Any remaining live inventory violation is also a blocking unresolved plan item.
- Apply validates the stored plan identity and baseline, performs a fresh terminal mutation inventory, requires the fresh source-byte digest to equal the planned inventory digest, rejects moves/unresolved work, requires terminal structural cleanliness, rechecks the source digest immediately before result publication, checks cancellation before and immediately before publication, and only then publishes a committed result with the verified digest and verification evidence.

## Test Evidence

The initial focused command was deliberately red because the new test imported a workflow seam that did not yet exist. A second red run exposed and corrected an implementation typo in the new source walker. A third red run proved the fixture needed a real legacy nesting violation rather than an empty aggregate. These failures were all test-first implementation feedback; no production apply occurred.

```text
bun nx run @semio-tech/repo-lib:test-quick -- -t 'requires a fresh clean terminal verification'
PASS: 1 test, 21 expectations, 0 failures
```

The final regression covers direct-but-violating unresolved planning, source-byte-only stale-plan rejection, NUL-byte content hashing, baseline mismatch rejection, cancellation with no apply artifact, forged zero-move/current-violation rejection, fresh clean terminal commit, virtual compose exclusion, symlink rejection, a deterministic mid-inventory source change that retries to a stable snapshot, and an empty mutation-root addition that invalidates the previous plan. It validates the neutral fixture with Ajv and independently checks the source roster with fast-glob.

```text
bun nx run @semio-tech/repo-lib:test-quick -- -t 'direct mutation ownership|direct mutation taxonomy'
PASS: 16 tests, 209 expectations, 0 failures
```

```text
git diff --check -- 📜️script.ts 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧫️fixtures/🔣️.json
expected: no output
```

## Remaining Foundation Work

This packet intentionally does not implement the broader enriched consumer/assignment inventory, schema-first scaffold completion, or direct descriptor/derive propagation. Executable codec ownership remains the next separately reviewed packet.

## Files

- `📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-inventory/🧫️fixtures/🔣️.json`
