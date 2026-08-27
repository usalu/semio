# S-Independent Mutation Verification

## Live corpus census

An independent read-only Bun census of every physical `🧪️oracle/🔣️.json` below `✏️s/🔌️plugins` found:

- 166 oracle manifests;
- 144 mutation catalogs;
- 1,555 vectors and 1,555 scenarios;
- zero vectors missing `sourceMutationDirectoryName`;
- 126 rows where the source and canonical mutation directory differ;
- 125 unique changed source names and 125 unique canonical destinations;
- zero duplicate source tuples and zero duplicate canonical tuples inside any catalog.

The apparent 17 duplicates found by an intentionally under-scoped repository-global tuple key disappear when catalog ownership is retained. This confirms that uniqueness belongs to the registered catalog/profile owner, not to a global mutation-name namespace.

## Focused checks

```text
bun ./📜️script.ts test --test-name-pattern='parses 🔣️taxonomy.json'
1 pass, 0 fail, 30 assertions

bun ./📜️script.ts test --test-name-pattern=language-agnostic
2 pass, 0 fail, 6 assertions
```

The first independent source-to-canonical registry run exposed a test-only matcher error: Bun's expect implementation has no `toHaveSize` matcher. The implementation owner was notified to assert `.size` explicitly and rerun the strict corpus test. This is not being waived and must be green before the planner consumes the catalog.

The correction landed and the independent rerun is green:

```text
bun ./📜️script.ts test --test-name-pattern=source-to-canonical
1 pass, 0 fail, 6 assertions
```

The independent full-bundle planner check then found a projection-ownership defect: a correctly addressed descendant leaf was still attributed to the generic `semantic-stem-resolution` rule instead of `artifact-mutation-test-projection-v1`. The engine owner was notified to make the explicit projection contract own every descendant node and rerun the test; this must also be green before apply.

Compose and `temp/compose` were excluded and not read. No Git state or workspace content was modified by the census or focused tests.
