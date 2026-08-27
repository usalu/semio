# Scoped Ticket and Generated-Output Admission

## Decision

Git enumeration already pushed the requested scope into the candidate command, but explicit ticket evidence and ignored generator outputs were walked in full before the final scope filter. This let a narrow production transaction encounter concurrently removed test fixtures in its otherwise unrelated active ticket. It also performed unnecessary reads under unrelated generated-output owners.

Both physical walkers now accept the same scope predicate used by inventory. Disjoint roots are rejected before any filesystem probe, and disjoint children are rejected before stat, enumeration, or content reads. Ancestors needed for semantic ownership remain admitted. Full inventory retains its existing behavior. Verification passes the plan scope into explicit ticket admission as well; unscoped lexical-reference calls retain their previous authority.

## Test-Driven Evidence

The permanent language-neutral fixture covers disjoint authorities, a descendant of an ignored ticket root, and a descendant of an ignored generated-output root. An isolated child wraps the filesystem probes and throws on every attempted read, stat, existence check, or directory enumeration under forbidden siblings. The expected leaf census is independently produced by fast-glob with symlink following disabled.

The initial test fixture was rejected for unordered/duplicate generator declarations and was corrected without changing production. The intended preimplementation run then failed all three cases at prohibited generated-output stat calls: zero passing tests, three failures, nine assertions, 5.11 seconds.

After the scope pushdown:

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️scoped-admission-walk.test.ts'
3 pass, 0 fail, 12 assertions, 6.84 seconds
```

The guarded and unguarded inventories have the same expected leaf census. Fixtures and their temporary Git repositories are ticket-local. No real Git state or actual Compose tree is used by this packet.

## Files

- Normalizer `🧹️normalization/🟦️.ts`: walker signatures, pre-probe filtering, inventory and verification call sites.
- Permanent fixture `🧫️fixtures/🧪️scoped-admission-walk/🔣️.json`.
- Ticket test `🧪️scoped-admission-walk.test.ts`.

The existing scoped/unscoped admission and ordering matrix subsequently passed: 4 tests, zero failures, 10 assertions, 19.40 seconds. The selected cases were standalone ignored-generator admission, standalone explicit-ticket admission, frozen unscoped inventory, and reversed creation order. The transaction regression remains a separate integration check; this packet alone is not a claim of complete monorepo convergence.
