# S-PATH-PROJECTION-TEST-DOMAIN

## Outcome

Implemented the schema-first test-domain contract for artifact mutation-vector projection without changing runtime mutation semantics, fixture trees, generator-owner roots, Git state, Compose, or temp Compose.

The completed registry census is:

| Record | Count |
|---|---:|
| Non-Compose governed mutation catalogs | 144 |
| Existing independent runtime capability kinds | 2,173 |
| Registered physical mutation vectors | 1,555 |
| Registered physical scenarios | 1,555 |
| Catalogs explicitly declaring `vectors: []` | 30 |
| Physical source/projected bundles missing, duplicated, unregistered, or malformed | 0 |
| Intentional canonical/source identity mismatches pending transactional apply | 3 |

Every physical mutation directory has exactly one scenario in the current census. Each of the 1,555 physical 13-node bundles is registered once. Runtime `kinds` remains an independent capability vocabulary: for example, glTF retains its seven dispatchable kinds while registering all 120 checked-in physical vectors.

## Frozen contract correction

The implementation census disproved the design report's proposed `vectors[].kind ↔ kinds[]` bijection. Expanding runtime kinds or inventing directory names would falsify current semantics.

The strict shape implemented instead is:

```ts
type MutationVectorScenario = Readonly<{
  id: string;
  directoryName: string;
}>;

type MutationVector = Readonly<{
  mutationId: string;
  mutationDirectoryName: string;
  scenarios: readonly MutationVectorScenario[];
}>;

type MutationCatalog = Readonly<{
  id: string;
  capability: string;
  standardDirectoryName: string;
  subsetDirectoryName: string;
  kinds: readonly string[];
  vectors: readonly MutationVector[];
  deferredKinds?: readonly string[];
}>;
```

`vectors` is required in every catalog and may be empty only when no physical vector exists. Each nonempty vector has a unique exact NFC mutation identity/directory and one or more unique canonical scenario identities. A scenario directory must be exactly `🧪️<id>`. There is no optional old form, alias, runtime-kind inference, or invented emoji.

The correction is also appended to `📓️s-path-projection-design.md`.

## Implementation

### Test-domain schema and loader

- Extended `🧪️test/🧬️schema/🔣️component.json` with strict `MutationVectorScenario`, `MutationVector`, and required catalog profile/vector fields.
- Extended `🧪️test/📦️packages/🟦️typescript/📦️index.ts` with:
  - exported strict record types;
  - `mutationCatalogProblems`, including unknown-field, NFC, profile-owner, unique mutation identity/directory, kebab-case scenario, and exact semantic-directory rendering validation;
  - fail-closed contribution parsing through `strictMutationCatalogs`;
  - `mutationVectorRegistryBreaches`, which proves exact source-or-projected ownership and closed 13-node bundle shape with one exclusive diff alternative;
  - explicit mixed-state, missing, unregistered, invalid-bundle, and canonical-source-mismatch findings.

The physical registry validator deliberately never consults `kinds`.

### Governed catalogs

Updated every non-Compose plugin-artifact `🧪️oracle/🔣️.json` containing `mutationCatalogs`:

- 144 manifests updated;
- profile fields captured from their exact `🏅️standards/<standard>/🪆️subsets/<subset>` owner;
- 1,555 vectors captured from exact existing `🧬️schema/🧬️mutations/<mutation>/🧪️tests/<scenario>` directories;
- mutation/scenario records sorted deterministically;
- 30 catalogs explicitly use `vectors: []`;
- all existing runtime kinds, oracle data, comments, and declaration order preserved.

The three DIN 16798 canonical scenario IDs are stored:

1. `raises-required-humidification-to-3-point-5-kg-per-hour`
2. `drops-provided-humidification-to-1-point-25-kg-per-hour`
3. `raises-infiltration-allowance-to-52-point-5-m3-per-hour`

Their physical source directories and references were not manually moved. The validator reports exactly three `mutation-vector-source-id-mismatch` findings until transactional apply.

### Tests and golden contract

- Added strict shape/independence and duplicate/profile rejection unit tests.
- Added exact source/projected 13-node bundle validation with malformed-bundle failure.
- Added projected `asset://🧪️tests/🪆️...` extraction/resolution coverage.
- Added Nx regression proving `🧪️tests/🪆️<profile>/<mutation>/<scenario>` storage never becomes an executable test project.
- Added language-neutral JSON golden:
  - `📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️mutation-path-projection/🔣️.json`
  - semantic fixture/test-case directories and physical `🔣️.json` leaf;
  - all six file mappings;
  - ordinary vector plus all three canonical DIN identities.
- Added an independent renderer/census assertion whose source membership matches test-only `fast-glob` exactly.
- Added the precise in-flight normalization assertion for all 24 golden file moves and rationale `artifact-mutation-test-projection-v1`; it is not skipped or weakened.

## Exact verification evidence

### Passing

```text
bun test <repo-test index.test.ts> --test-name-pattern='physical vectors are strict|13-node bundle|projected asset URI|profile storage never'
4 pass, 69 filtered, 0 fail, 9 expect(), 33.40s
```

```text
bun test <repo-test index.test.ts> --test-name-pattern='mutation completeness gate'
7 pass, 66 filtered, 0 fail, 8 expect(), 34.96s
```

```text
bun test <repo-lib index.test.ts> --test-name-pattern='language-agnostic mutation projection golden'
1 pass, 212 filtered, 0 fail, 2 expect(), 3.57s
```

```text
bun nx run @semio-tech/repo-test:test-long '--test-name-pattern=profile.*storage'
1 pass, 72 filtered, 0 fail, 1 expect(), 34.70s
NX Successfully ran target test-long for project @semio-tech/repo-test
```

```text
bun nx run @semio-tech/repo-lib:test-long '--test-name-pattern=language-agnostic.*golden'
1 pass, 212 filtered, 0 fail, 2 expect(), 3.17s
NX Successfully ran target test-long for project @semio-tech/repo-lib
```

The independent catalog/filesystem audit returned:

```json
{
  "catalogs": 144,
  "vectors": 1555,
  "scenarios": 1555,
  "problems": [],
  "breaches": [
    "mutation-vector-source-id-mismatch",
    "mutation-vector-source-id-mismatch",
    "mutation-vector-source-id-mismatch"
  ]
}
```

There were no missing, duplicate, unregistered, mixed-state, or invalid-bundle findings.

### Exact residual failures and atomic dependencies

1. The post-transaction whole-repository loader regression currently receives zero catalogs:

```text
bun test <repo-test index.test.ts> --test-name-pattern='all governed catalogs'
Expected length: 144
Received length: 0
0 pass, 73 filtered, 1 fail, 75.60s
```

Taxonomy v7 discovery resolves the test-contribution JSON kind to `🧪️oracle/🔣️.json`, while this lane was expressly limited to the existing governed `🧪️oracle/🔣️.json` files and forbidden from moving production trees. The failing regression is intentionally precise: transactional physical-leaf apply must move the manifests before the strict whole-repository loader can observe the 144 catalogs. No alternate filename, alias, or legacy fallback was added.

2. The precise normalization projection test initially reached the engine but failed before planning at `normalization/🟦️.ts:1638` because `OLD_MUTATION_STRUCTURE_SOURCE` produced an invalid Unicode regular expression. The engine owner reported that regex corrected after the run; root then requested immediate finalization, so this packet does not claim an unperformed rerun. Projection behavior remains owned by the in-flight engine lane.

3. The three canonical DIN catalog identities intentionally differ from current physical source basenames. They must be moved with their Rust/Python/Gherkin/JSON/prose references in the same normalization transaction. No production runtime was pointed at the canonical path early.

4. Test-package TypeScript lint was invoked and failed only in an unrelated shared UI dependency:

```text
🖱️ui/🎨️styling/📦️packages/🟦️typescript/📦️index.ts:594:54
TS2339 Property 'env' does not exist on type 'ImportMeta'.
🖱️ui/🎨️styling/📦️packages/🟦️typescript/📦️index.ts:916:33
TS2339 Property 'glob' does not exist on type 'ImportMeta'.
```

No diagnostic referenced this lane's files.

An initial fundamental Nx selector also demonstrated two command-boundary facts: shell alternation must not be passed through `nx:run-commands` unescaped, and the repository-test module's approximately 34-second initialization exceeds the 15-second fundamental budget. The final evidence uses one equal-form selector per `test-long` target.

## Acceptance status

| Check | Status |
|---|---|
| Required strict schema/profile/vector fields | Complete |
| No optional legacy catalog form or aliases | Complete |
| Runtime kinds unchanged and independent | Complete |
| All 144 catalogs updated | Complete |
| Exact 1,555 physical vectors/scenarios registered once | Complete |
| 30 vectorless catalogs explicit | Complete |
| Exact source/projected 13-node validation | Complete |
| Three canonical DIN identities stored | Complete |
| Projected URI test | Complete |
| Nx profile storage non-project regression | Complete and Nx-passing |
| Language-neutral JSON + fast-glob parity | Complete and Nx-passing |
| Transactional manifest physical-leaf move | Pending normalization apply |
| Transactional DIN moves/reference edits | Pending normalization apply |
| Engine projection assertion | Present; in-flight engine rerun required |

## Safety

- No modifying Git command was run.
- No actual mutation scenario tree was edited or moved.
- No generator-owner root, root manifest/script, taxonomy, discovery, or normalization production file was edited.
- No Compose or temp Compose path was traversed, read, or changed.
- No compatibility behavior was introduced.
