# WP2c — root script, project.json, package.json, launch.json, nx.json

Partition: root `📜️script.ts`, root `📋️project.json`, root `package.json`, `.vscode/launch.json`, `nx.json`.
Successor to W2b (stopped mid-task); W2b's staged edits were found on disk, kept, and completed.

## 1. Per-row results

| Row | Result |
|---|---|
| 3 / 64 (oracle spec + vitest include) | **done** — verified running |
| 4 / 64 (`ajv` + `ajv-formats` devDependencies) | **done** (landed by W2b, lockfile already carries both) |
| 9 / 48-aggregate (structural aggregate identity) | **done** — already structural on arrival; verified independent of `x-semio-mutationKinds`. Unblocks row 23 |
| 20 (`📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts`) | **not mine** — library partition; exact diff in §4.1 |
| 34 (job fixture paths, `⏱️trace` contention) | **done** — verified running |
| 48 (multi-format parity on stdio) | **blocked** — gate cannot reach the parity policies; two blockers fixed by me, the third is stdio-partition data (§3) |
| 54 (`compileScopeExport` hoist) | **local helper landed** in the root script; hoist request in §4.2 |
| 59 (no test-domain path / phase vocabulary in the root script) | **done** — both assertions verified false |
| 64 (register the two cargo test binaries) | **done** — commands, nx targets, package.json scripts and launch entries added; the tests themselves cannot run today (§3.3) |
| 71 (launch.json `🪶️sqlite`) | **done**, with a naming deviation (§5.1). Taxonomy half is the library worker's (§4.3) |
| 75 (`toolJobMicrosecondBudgetSelfTests` job-budget consolidation) | **done** — verified running; target file had moved out of the root script (§5.2) |

## 2. What changed

### `📜️script.ts`

- `SCHEMA_DRAFT07_ORACLE_SPEC` → `🧰️framework/🔨️modules/🧬️schema/🧪️tests/✅️draft07-oracle/🟦️.ts`, run through a
  generated one-line vitest config whose `include` names the exact spec (W2b; vitest's default include never
  matches `🟦️.ts`, so the old invocation exited 1 with "No test files found"). `schema test` now runs both
  halves unconditionally and exits on their union.
- New `SchemaScript` subcommands **`schema compile`** and **`schema entries`**, sharing one `rustReport()`
  helper that runs a registered `semio-framework-schema` test binary with the environment variable that makes
  it write its JSON report (`--out <path>` keeps it, otherwise a scratch file):
  - `schema compile` → `cargo test -p semio-framework-schema --test schema-module-compile`
    (`SEMIO_SCHEMA_MODULE_COMPILE_OUT`)
  - `schema entries` → `cargo test -p semio-framework-schema --test schema-export-entries`
    (`SEMIO_SCHEMA_EXPORT_ENTRIES_OUT`), then feeds the dump straight into `verify --rust-entries <file>`
    (passes `--rust-entries-complete` through)
- New `compileScopeExport(root, modulePath, exportId)` — compiles one `$defs` export of a scope module.
  Vendor keywords are **discovered from the module itself** (every key matching `x-semio-*`, found by a walk)
  rather than from an enumerated list, so a new annotation never breaks Ajv `strict: true`; the six wire-width
  numeric formats are declared in `SCHEMA_NUMERIC_FORMATS`.
- `toolJobFixedOperationFixtureRun` / `toolJobSharedFrameworkActionFixtureRun`: the two deleted
  `🧵️job/🧪️fixtures/*.schema.json` reads are replaced by `compileScopeExport(…, "🧰️framework/🔨️modules/🧵️job/🧬️schema/🔣️.json", …)`
  on `FixedOperationRegistryFixture` / `SharedFrameworkActionRoutesFixture`. The old hand-rolled `$id`/`$schema`
  identity assertions are gone — the fixtures are now really **validated** against the export instead of being
  checked for a matching identity string.
- `schema test` reads the harness location from the taxonomy (`testDomainPath`) instead of spelling it out
  (row 59). Narrowed at one accessor because the library `Taxonomy` interface does not surface the key (§4.4).
- `stdioWalkFiles` skips `dist/` (`STDIO_PACKAGE_BUILD_OUTPUT_DIR`) — see §3.1.
- `stdioAssertDefinition` accepts the optional `native_factory` codec field — see §3.2.

### `📋️project.json`

`schema-compile` and `schema-entries` targets (`nx:run-commands`, `cache: false`, `forwardAllArgs`), placed
between `schema-oracle` and `schema-test`.

### `package.json`

`"schema:compile"` and `"schema:entries"` scripts calling nx, in the existing `schema:*` order.
(`ajv-formats: 3.0.1` was already added by W2b and is already in `bun.lock`'s root devDependency block — no
`bun install` was needed and none was run.)

### `.vscode/launch.json`

| name | command | group | order |
|---|---|---|---|
| `🪶️sqlite test` | `bun nx run @semio-tech/repo-sqlite:test` | `3_dev` | `-2` |
| `📦️test🧬️schema🧩️compile` | `bun nx run workspace:schema-compile` | `4_build` | `208.65` |
| `📦️verify🧬️schema📤️entries` | `bun nx run workspace:schema-entries` | `4_build` | `208.66` |

No order values had to be shifted: `🦑️mcp test` at `-3` was the last `3_dev` entry.

### `nx.json`

Untouched this pass (W2b's `production` named-input change was already staged and is kept).

### Outside the partition, applied anyway (§5.2)

- `🧰️framework/🔨️modules/🧵️job/⏱️budget/🧪️tests/🔬️tool-job-microsecond-budget/🟦️.ts` — row 75's exact diff.
- `🧰️framework/🔨️modules/⏱️trace/⏱️clock/🧪️tests/🔬️tool-job-telemetry-contention/🟦️.ts` — row 34's contention path.

## 3. Row 48 — the stdio parity gate is unreachable

`bun ./📜️script.ts stdio schema-parity` never reaches the multi-format parity policies
(`surfaceSpecs` typescript / graphql / protobuf / json-schema / text / binary). It dies in
`stdioDefinitionCatalog` / `stdioAssertDefinition`, three sequential blockers deep:

### 3.1 `dist/` in the definition walk — **fixed** (my partition)

```
error: [stdio] catalog definition paths are not the complete schema-owned artifact definition set.
```

`stdioWalkFiles` walked `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🟦️typescript/dist/🧬️schema/📜️artifact-definition.json`
— untracked TypeScript build output (`.gitignore:629` `**/📦️packages/*/dist/`). Declared 36, discovered 37.
The gate therefore failed on **build state**, not on sources, on any machine that had built that package.
Fixed by skipping `dist/` in the walk.

### 3.2 `native_factory` — **fixed** (my partition)

```
error: [stdio] s.stdio.xml.codecs[0] has unknown fields native_factory.
```

`native_factory` (`factory_id`, `artifact_kind`, `document_schema`, `extension`, `pack_schema_hash`,
`runtime_capability_id`) is present in **26 of 36** artifact definitions and has been committed since
**2026-09-05 03:53**; the root gate had never heard of it (zero occurrences in `📜️script.ts`). Taught the gate
the field as optional, with all six sub-fields required and string-typed.

### 3.3 `runtime_capabilities` id grammar — **stdio partition, not fixed**

```
error: [stdio] "s.stdio.xml.standard.1-0.codec.codec-stdio-xml-extension-xml.v1"
       must be a canonical vN leaf below "s.stdio.xml.runtime.codec.".
```

`stdioAssertDefinition` requires `<artifactId>.runtime.<category>.<leaf>.vN`. The committed data uses a
different scheme entirely (`s.stdio.xml.standard.1-0.codec.…`, `s.stdio.xml.composer.…`,
`s.stdio.xml.representation.mime-application-xml-extension-xml` with no `.vN` at all). Measured repo-wide:

```
definitions 36  runtime_capabilities 308  failing the gate grammar 308
artifacts affected: 26 of 36
```

**308 of 308.** Same commit date as `native_factory` (2026-09-05), so the whole stdio artifact-definition data
model moved and the root gate stayed behind — the stdio gate has been dead for three days, independently of
this ticket. Which side is authoritative (the gate's grammar or the committed ids) is a stdio-partition
decision in the same family as ledger rows 45/46/49/81/82/83, so I stopped here rather than rewriting
someone else's contract. **Row 48 cannot be answered until 3.3 is settled** — see §4.5.

Log: `🗑️generated/wp2c-stdio-schema-parity.txt`.

## 4. Cross-partition requests

### 4.1 → library worker (row 20) — `📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts`

Three live assertions hard-code `payloadSchema === "🧬️.schema.json"`; they must assert the taxonomy location
`🧬️schema/🔣️.json`. The coordinator dispatched row 20 to W2s, but the file is in the `📚️library` partition,
which my brief assigns to the sibling worker — I did not edit it. Either the sibling takes it or the
coordinator re-dispatches.

### 4.2 → library worker (row 54) — hoist `compileScopeExport`

`compileScopeExport(root, modulePath, exportId)` now exists once in the root `📜️script.ts` (§2). Hoist it to
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts` and I will switch the root
script to the import in the next pass, together with the gis/stdio/space/flow/block/draw/lowpoly/norm/hub/os
copies. **Take the vendor-keyword discovery with it** (walk the module for `x-semio-*` keys) rather than
`wp4-plugins §6G`'s hard-coded `addKeyword({ keyword: "x-semio-state" })`: the tree carries thirteen distinct
`x-semio-*` annotations today (`state` 4035, `mutation` 120, `derived` 118, `mutationKinds` 60, `child` 15,
`invariant` 14, `persistence` 7, `note` 5, `child-kind` 5, `link-roles` 4, `link` 2,
`retained-discriminator` 1, `binary` 1) and an enumerated list would go stale on the next annotation.

### 4.3 → library worker (rows 71, 75, and the catalog)

- Taxonomy `outputRoots` `:24661` → `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🐹️entity_kinds.g.go`
  (row 71 F; land with row 70).
- `memberNames` edit so `🔌️mcp/🔗️graphql/🔗️.graphql` can move into a `🧬️schema/` module (row 71).
- **Regenerate the catalog and the index.** `schema verify` currently reports both
  `📚️library/🔣️schema-catalog.json` and `📚️library/📓️schema-catalog.md` stale. Both files are in the library
  partition, and the tree is still moving under several wave-2 workers, so this should be the **last** action
  before ticket close, not now.

### 4.4 → library worker — `Taxonomy` interface

`readonly testDomainPath: string;` is missing from the `Taxonomy` interface in
`📚️library/🔍️discovery/🟦️.ts:884` although the key exists in `🔣️taxonomy.json` and the test domain's own
`TestTaxonomy` declares it. The root script reads it through a narrowed cast today; add the field and I will
drop the cast.

### 4.5 → coordinator / stdio partition (W8c/W8d) — new row

Settle §3.3: `runtime_capabilities` ids (308/308) do not match `stdioAssertDefinition`'s
`<artifactId>.runtime.<category>.<leaf>.vN` grammar. Either the gate adopts the committed scheme or the data
is re-keyed. Until then `stdio schema-parity`, `stdio quick`, `standards-coverage`, `codec`, `mutation-law`
and `inference` are all dead (they share `stdioArtifactLedger`), and row 48 cannot be measured.

### 4.6 → coordinator — row 23 is unblocked

Row 23 (`drop x-semio-mutationKinds from aggregates`) was `blocked(W2b)` pending a structural aggregate check.
`policyMutationAggregateMembers` is structural now (`oneOf` `$ref` → leaf `$id` or payload path; every
descriptor-bearing leaf must be referenced) and reads no `x-semio-mutationKinds`. W7c/W8c can drop the
keyword. Note the check still **tolerates** relative-path refs alongside absolute `$id`s; once row 79 lands
everywhere the path branch should be deleted so contract §B is enforced rather than merely permitted.

## 5. Deviations

### 5.1 launch.json entry name

`📓️wp4b-repo-client.md` §7 B asked for `🪶️sqlite schema test`. I registered **`🪶️sqlite test`**: its
neighbours in `3_dev` are `🦑️mcp dev` / `🦑️mcp test` (`<emoji><module> <target>`), the nx target is `test`,
and CLAUDE.md requires following the existing naming. Only `test` was registered — no repo project registers
its `test-quick` / `test-long` / `test-exhaustive` phases in launch.json (0 occurrences of either).

### 5.2 Rows 34 and 75 landed outside my partition

At 19:34 during this session a repo-wide sweep extracted `toolJobMicrosecondBudgetSelfTests` and
`toolJobTelemetryContentionSelfTests` out of the root `📜️script.ts` into
`🧵️job/⏱️budget/🧪️tests/🔬️tool-job-microsecond-budget/🟦️.ts` and
`⏱️trace/⏱️clock/🧪️tests/🔬️tool-job-telemetry-contention/🟦️.ts` (both staged-added, `A`, uncommitted; the root
script lost ~5300 lines in the same move). Rows 34 and 75 name the root script, so their target moved after
the assignment. I applied the exact diffs at the new locations rather than leaving two files reading deleted
schemas with W10b already reporting row 35 done. Framework-modules worker: these two files now carry my edits.

### 5.3 `schema verify` still exits non-zero

Only because of the stale catalog/index (§4.3), which is not my partition.

## 6. Verification (real output)

### Row 3/64 — the draft-07 oracle now actually runs inside `schema test`

```
$ bun ./📜️script.ts schema test
[test schema]   … and 3338 more
[test schema] 0/0 schema-bound fixture(s) reached their declared stage

 RUN  v4.1.10 /Users/ueli/Documents/semio

 Test Files  1 passed (1)
      Tests  17 passed (17)
   Start at  20:06:40
   Duration  4.46s (transform 364ms, setup 0ms, import 1.44s, tests 1.65s, environment 0ms)

[schema test] harness exit=1, draft-07 oracle exit=0
```

Before this pass the oracle half printed "No test files found" and exited 1. The harness `exit=1` is the
other partitions' unfinished migrations (`schema-dialect-not-draft-07`, `schema-placement-outside-module`,
`schema-placement-forbidden-filename`), none inside this partition. That the harness ran at all is also the
row-59 proof: its path came from the taxonomy.

### Row 59

```
$ bun -e '…'
root names testDomainPath literal: false
root names phase vocabulary: false
```

(First run said `true`: my own `@see` docstring had reintroduced the literal. Rephrased, re-measured.)

### Row 75 — job budget module exports

```
$ bun -e '…'
job.budget/Budget over 🧫️fixture/🔣️.json: PASS
job.budget/Clock over 🕰️clock.json: PASS
job.budget/Binding over 🪢️binding.json: PASS
Budget rejects forged unit: true rejects extra field: true
```

`toolJobMicrosecondBudgetSelfTests()` itself still throws — but **after** all three schema validations, at
line 79, on `microsecond exact worker binding: none`, which is a Rust-source law over
`🔌️plugin/🦀️.rs` + `🧵️job/🦀️.rs` + `⏱️trace/🦀️.rs`. Measured against four revisions of the plugin source:

```
[DEBUG] HEAD plugin accepted: false
[DEBUG] working-tree plugin accepted: false
[DEBUG] HEAD~5 false
[DEBUG] HEAD~20 false
[DEBUG] HEAD~60 false
```

Unmutated production sources have failed `toolJobMicrosecondWorkerExact` for at least 60 commits — pre-existing,
schema-independent, and outside this partition. Flagging it, not fixing it.

### Row 34 — telemetry contention

```
$ bun -e '…'
telemetry contention self-tests: 11
```

### Row 34 — the two job fixture readers

```
$ bun ./📜️script.ts verify interactivity tool-jobs --shared-action-fixture-only
{"schema":"semio.framework.plugin.shared-framework-action-routes.v1","routes":[…12 routes…],"descriptor":[…6…],"hostile":[…12…]}
EXIT=0

$ bun ./📜️script.ts verify interactivity tool-jobs --fixed-operation-fixture-only
{"schema":"semio.framework.job.fixed-operation-registry-law.v1","results":[…8 cases…]}
EXIT=0
```

Both now validate their fixture against `https://semio.tech/schema/framework/job/schema.json#/$defs/…`
through `compileScopeExport`.

### Row 64 — the subcommands exist and dispatch

```
$ bun ./📜️script.ts schema bogus
error: unknown schema subcommand: "bogus" (expected audit | check | compile | docs | entries | generate | oracle | test | verify).
```

### Row 64 — the cargo test binaries cannot run today

```
$ CARGO_TARGET_DIR=<scratch>/target-w3 RUSTC_WRAPPER="" SEMIO_SCHEMA_EXPORT_ENTRIES_OUT=… \
    cargo test -p semio-framework-schema --test schema-export-entries --offline
error[E0599]: no method named `ensure_durable_group_idle` found for mutable reference `&mut SpaceHost<M>` …
error[E0609]: no field `envelope` on type `&mut SpaceHost<M>`
error[E0599]: no method named `replace_backbone_retained` …
error[E0599]: no method named `pump` …        help: … self.meta.pump().await?
error[E0599]: no method named `bump` …        help: … self.meta.bump()?
error: could not compile `semio-framework-os-kernel` (lib) due to 5 previous errors
EXIT=101
```

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` is uncommitted-modified (` M`) by a live peer mid-refactor
— the compiler's own `help: self.meta.pump()` is the signature of a `SpaceHost` field extraction in progress.
`semio-framework-schema` depends on `semio-framework-os-kernel`, which is precisely what ledger row 76 decided
to break with the `semio-framework-schema-registry` leaf crate. **The commands are registered and correct;
they will be runnable once the peer's refactor lands or row 76 ships.** Not re-run, per the one-shot budget.

Log: `🗑️generated/wp2c-cargo-schema-export-entries.txt`.

### Taxonomy validation — blocked repo-wide, unrelated

```
$ bun ./📜️script.ts verify taxonomy report
error: Normalization requires an explicit repository-boundary decision before authored classification: ♻️mit-bestand/🔎️recherche
EXIT=1
```

`git ls-files -s` reports mode `160000` for that path — a committed, unmodified gitlink. Nothing to do with
schemas or this partition; it blocks every taxonomy verify run on this tree today.
Log: `🗑️generated/wp2c-taxonomy-report.txt`.

### `schema verify`

```
$ bun ./📜️script.ts schema verify
[schema verify] stale generated output: 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json, 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📓️schema-catalog.md. Run bun ./📜️script.ts schema generate && bun ./📜️script.ts schema docs.
```

Both files are the library worker's; see §4.3.

## 7. Open questions

1. **Row 20 ownership.** The ledger dispatches it to W2s (me), the brief puts the file in the sibling's
   partition. Unresolved; nobody has edited it.
2. **`compileScopeExport` and `dependsOn`.** The helper registers only the module it is handed, so a module
   with a cross-scope `$ref` will not compile through it. Once the catalog carries `dependsOn` (row 43/69) the
   hoisted version should take the catalog and `addSchema` the declared dependencies — that is also what
   `schema check` needs for "compile each module against exactly its declared `dependsOn` set".
3. **Should `schema test` also run `schema compile`?** `wp3b` §7.2 calls `schema-module-compile` "the repo-wide
   compile gate". I registered it as its own command rather than folding it into `schema test`, because
   `schema test` must stay runnable without a Rust toolchain and, today, without a working `os-kernel`. If the
   coordinator wants one gate, the union belongs in `verify-gate`, not in `schema test`.
4. **The stdio gate's authority (§3.3).** 308/308 ids disagree with the gate. Whoever settles it should also
   decide whether `native_factory`'s six fields (§3.2) are the real contract or a subset — I inferred them
   from the 26 definitions that carry it, not from a declaration.
