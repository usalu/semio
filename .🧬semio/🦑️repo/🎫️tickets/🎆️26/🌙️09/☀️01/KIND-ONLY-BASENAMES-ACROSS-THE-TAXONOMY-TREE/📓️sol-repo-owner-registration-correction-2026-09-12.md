# Repository Library Owner Registration Correction

Date: 2026-09-12

## Outcome

The two repository-library source owners from the completed 22-source extraction now have complete, contextual semantic-directory ancestry. The registry admits only the three observed owner-local names:

| Parent kind | Member name | Resolved kind |
| --- | --- | --- |
| `modules` | `📚️library` | `members-of-modules` |
| `members-of-modules` | `🕸️dependencies` | `members-of-members-of-modules` |
| `members-of-members-of-modules` | `🧩️runtime` | `members-of-members-of-members-of-modules` |

The portable ownership fixture now contains the two actual owner paths and their complete ordered member chains:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟦️.d.mts`

The focused test verifies that each owner exists, resolves each member against the preceding parent kind, and retains its anonymous language-kind leaf. No global `library`, `dependencies`, `runtime`, source, or declaration exemption was added.

## Changed Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`: added the three exact member-name admissions in byte order.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🦑️repo-source-ownership/🔣️.json`: added the two real owner directory chains.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🦑️repo-source-ownership/🔣️.json`: made the two-chain portable contract required and closed.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦑️repo-source-ownership/🟦️.ts`: added contextual chain resolution against the live catalog taxonomy.
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE/📓️sol-repo-owner-registration-correction-2026-09-12.md`: retained this evidence.

## Verification

- Direct strict taxonomy probe: `loadTaxonomy()` succeeded and `validateTaxonomy()` returned zero problems. All three contextual `semanticDirectoryKindId()` calls returned the kinds in the table. `implementationLeafBasenameFinding()` returned `null` for both owner paths.
- Direct portable/native test: `bun test ./…/🧪️tests/🦑️repo-source-ownership/🟦️.ts` passed **7 tests, 219 assertions, 0 failures** in 1.56 seconds.
- Registered Nx route with isolated workspace data and cache disabled: `bun nx run @semio-tech/repo-lib:test-repo-source-ownership --skip-nx-cache` passed **7 tests, 219 assertions, 0 failures**. Nx reported 2.9 seconds total and 2.7 seconds critical path.

This correction does not address unrelated ambient directory names or the coordinator `@/lib` referent identified by the independent audit.
