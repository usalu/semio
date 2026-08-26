# Fixed Owner Cargo, Nx, and Transaction Schema

## Outcome

The bounded schema-first slice is complete and strict-green. Taxonomy SHA-256 at verification was `a0da370a95a057dc0d7fd4a2358b681226a060e48c26c7a195809cc6c5e7abea`; `loadTaxonomy()` returned schema version 7 and `validateTaxonomy()` returned zero problems.

No Git state was modified and neither Compose prefix was accessed.

## Frozen authority

### Cargo target triples

- Two exact fixed-directory contracts admit only `wasm32-unknown-unknown` and `wasm32-wasip2` beneath a parent proven as `ticket-cargo-target-evidence` and a governed canonical or embedded ticket path.
- Two exact `CACHEDIR.TAG` contracts require the corresponding winning fixed-directory contract through the tagged scope `{ kind: "fixed-directory-contract", fixedDirectoryContractId }`.
- The language-neutral golden binds all 20 current target-triple cache leaves.
- Six unowned cache leaves remain unresolved, including the three retained embedded transaction fixture leaves. Unknown triples and non-ticket lookalikes reject.

### Adjacent Nx manifests

- `nx-owned-node-package-manifest` owns only `package.json` with a sibling winning `nx-project-manifest`.
- `nx-owned-typescript-config` owns only `tsconfig.json` with that same sibling proof.
- The matcher consumes the explicit tagged scope `{ kind: "sibling-fixed-filename-contract", fixedFilenameContractId }`; it never infers sibling ownership from a basename or directory shape.
- The language-neutral golden binds 13 current `package.json` and two current `tsconfig.json` paths. The ticket `_tmp/package.json` and a counterfeit sibling identity remain blocked.

### Windows-safe transaction preparations

- `transaction-edit-preparation` is an exact child of `transaction-stage`. Its child-state union is the four exact subsets of hash-bound `<24hex>.edit` and matching `<24hex>.pre`: empty, candidate-only, candidate plus displaced preimage, or displaced preimage only.
- `transaction-restore-preparation` accepts only empty, matching `<24hex>.backup`, matching backup plus `<24hex>.post`, or post only.
- `transaction-lease-preparation` remains an exact child of `transaction-backup` with `🚧️lease-<positive-pid>-<uuidv4>-(preparing|stale)`.
- `transaction-json-write-preparation` is an exact child of either `transaction-journal-write` or `transaction-lease-preparation`. It accepts only empty, `🔣️.json`, `🔣️.json` plus `⏮️.json`, or `⏮️.json`.
- New scoped physical evidence kinds are `transaction-edit-preimage` and `transaction-json-previous`. Duplicate, mismatched, partial, extra, wrong-parent, malformed PID, malformed hash, and malformed UUID cases reject.

The normalization writer received the exact tagged scope matcher semantics and transaction child-state unions. Its duplicate schema parser must mirror these unions without a fallback; that parser is outside this slice.

## Tests and evidence

All ticket tests derive their root from `import.meta.url`, use ticket-relative production imports, and are portable across Windows, macOS, Linux, and devcontainers. Captured historical JSON paths were not rewritten.

From repository root:

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️fixed-owner-scope.test.ts'
4 pass, 0 fail, 157 expect() calls, 156 ms

bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️cli-artifact-directory-kinds.test.ts'
4 pass, 0 fail, 124 expect() calls, 74 ms

bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️ticket-cargo-fixed-authority.test.ts'
2 pass, 0 fail, 12 expect() calls, 136 ms

bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️cargo-cache-tag.test.ts'
4 pass, 0 fail, 19 expect() calls, 72 ms
```

The fixed-owner golden compares the schema matcher to `picomatch` for every governed Cargo and Nx path and uses `fast-glob` to prove each exact owner/sibling path exists. The ticket-Cargo suite additionally validates an isolated Cargo package through Cargo metadata.

Permanent focused tests from the repo-lib package directory:

```text
bun ./📜️script.ts test --test-name-pattern=triples
1 pass, 0 fail, 13 expect() calls, 3.37 s

bun ./📜️script.ts test --test-name-pattern=preparation
2 pass, 0 fail, 24 expect() calls, 3.29 s
```

Strict load from repository root:

```text
bun -e 'import {loadTaxonomy,validateTaxonomy} from "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts"; const t=loadTaxonomy(); const p=validateTaxonomy(t); console.log(`schema=${t.schemaVersion} problems=${p.length}`); if(p.length) process.exit(1);'
schema=7 problems=0
```

`git diff --check` over every touched production/test/golden path returned no output.

## Nx routing blocker

The repository-owned Nx route is not presently executable: both `bun nx run @semio-tech/repo-lib:test ...` and the installed Nx entry resolve the inferred package script `"test": "nx run @semio-tech/repo-lib:test"`, recursively launching the same target instead of the explicit `project.json` command `bun ./📜️script.ts test`. The runaway processes were interrupted; none remained. Package metadata repair is outside the authorized taxonomy/discovery/test scope. The exact permanent command behind the intended target is green as recorded above.

## Files

- Production: `🔣️taxonomy.json`, `🔍️discovery/🟦️component.ts`.
- Permanent focused test: repo-lib `🧪️index.test.ts`.
- Ticket authority: `🧪️fixed-owner-scope.test.ts`, `🧪️fixed-owner-scope/🔣️.json`, `🧪️cli-artifact-directory-kinds.test.ts`, `🧪️transaction-attempt-authority/🔣️.json`.
- Portable prior-slice tests: `🧪️ticket-cargo-fixed-authority.test.ts`, `🧪️cargo-cache-tag.test.ts`.

## Nested publication addendum

The transaction schema now owns both Windows-safe byte-publication protocols without granting their partial bytes final authority:

- `transaction-edit-write-preparation` is an exact child of `transaction-edit-preparation`; `transaction-edit-write-candidate` admits only kind-only `🚧️.edit` below it.
- `transaction-backup-write-preparation` is an exact child of `transaction-backup-preparation`; `transaction-backup-write-candidate` admits only kind-only `🚧️.backup` below it.
- Both writer directories are exactly `🚧️write-<positive-pid>-<uuidv4>`. Their state union is empty or the one kind-only regular leaf. A foreign, duplicate, wrong-parent, partial-name, or cross-protocol leaf rejects.
- The outer edit and backup preparation facts are exactly `{parentKindId,directoryName,leafNames,writePreparations}`. Direct leaves remain operation-hash-bound and authoritative; zero or one nested writer is admitted. Two writers reject before recovery.
- `taxonomyCliAttemptPreparationsProblems` consumes all sibling facts at once, byte-sorts them, rejects duplicate directory identities and duplicate target ordinals, and validates the exact tagged child union: direct `🚧️stage`, `💾️backup`, and `🔒️lease` directories plus the regular `🔣️.json` journal. A writer-looking direct child is a counterfeit. `validateTaxonomy()` now exercises this validate-all authority, rather than leaving it as an uncalled helper.
- `transaction-json-write-preparation` retains all four exchange states (empty, final only, final plus previous, previous only) and now resolves below `transaction-journal-write`, `transaction-lease-preparation`, and canonical `transaction-lease`.
- Canonical, preparing, and stale-quarantined lease child unions are validated by `taxonomyCliLeaseDirectoryProblems`. A preparing lease may still be unpublished/empty; canonical and stale leases require the complete canonical JSON or the exact no-canonical exchange state with both `🔣️.json` and `⏮️.json` inside one authorized writer. Stale quarantine may retain every otherwise-authorized JSON writer crash state without blessing foreign leaves.

The normalization writer confirmed receipt and is consuming these exact IDs through canonical directory resolution and scoped kind authority. It was also told to call the validate-all attempt-preparation preflight before any recovery mutation; normalization implementation remains outside this schema lane.

Current deterministic identities:

```text
taxonomy.json SHA-256
7bf866f53921e22ae0f514db3ba7bc19d83c2ab5e56991ccb6d5c468ee15e975

transaction-attempt-authority/🔣️.json SHA-256
2e9d0736a1a0312a6bc34200c12d32ddab41bc1dc67bcf0196fec027c6c1e305

strict taxonomy
schemaVersion=7 semanticDirectoryKinds=128 scopedFileKinds=7 problems=0
```

Final focused evidence from repository root:

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️cli-artifact-directory-kinds.test.ts'
4 pass, 0 fail, 177 expect() calls

bun nx run @semio-tech/repo-lib:test-quick -- --test-name-pattern='validates the exact transaction edit preparation|validates every attempt preparation sibling|validates restore exchange states'
3 pass, 0 fail, 240 filtered out, 37 expect() calls; Nx target succeeded

bun -e '<loadTaxonomy + validateTaxonomy strict probe>'
{"schemaVersion":7,"semanticDirectoryKinds":128,"scopedFileKinds":7,"problems":[]}

git diff --check -- <five bounded schema/test/golden paths>
no output
```

The portable golden uses `picomatch` independently for the directory grammars and both scoped writer paths. It includes complete sibling sets, reversed valid ordering, exact and ordinal duplicates, malformed preparations, foreign children, direct nested-writer counterfeits, all outer/inner edit and backup states, canonical/preparing/stale lease states, and JSON previous-only/both/final-only states.

The earlier Nx-routing note is superseded for the focused test target: the exact Nx command above is green. The Nx lint target recursively re-entered itself and was interrupted; a direct `bun x tsc` reached the compiler and reported only pre-existing cross-package `ImportMeta.env`/`ImportMeta.glob` and `rootDir` errors outside this slice, with no diagnostic in the changed taxonomy, discovery, or test regions.

## Independent Root Verification

The root lane reran the final portable artifact authority after the schema lane released it: `4 pass`, `0 fail`, `177 expect()` calls in 87 ms. A separate strict loader returned schema version 7, 128 semantic directory kinds, seven scoped file kinds, and zero validation problems.
