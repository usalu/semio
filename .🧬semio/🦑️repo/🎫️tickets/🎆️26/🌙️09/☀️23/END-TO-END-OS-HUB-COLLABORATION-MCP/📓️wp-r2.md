# WP-R2: Kernel Edit-Text Law, semio-base Oracle Fixture, OS Suite Rerun

Slice: R2 (session 10). Captures: `.tmp-ticket/wp-r2/generated/`. Private cargo: `.tmp-ticket/wp-r2/target`.
Inherits: R1 "Still open" (kernel edit-text law, semio-base 36/43).

## Status

| Item | State | Evidence |
|------|-------|----------|
| 1. kernel `document_codec_of_round_trips_dsl_and_pack_and_edit_text` | PASS: the product is correct, the law was stale and is rewritten | `kernel-codec-law-1.txt` |
| 1. kernel lib count (`--features sync,ureq`) | nextest **1189/1189**. libtest (`cargo test`, in-process parallel) gave 1188, 1187 and 1188 of 1189 in three runs. Every failure is the sync actor flake below. Handed to h4 | `kernel-lib-nextest-1.txt`, `kernel-lib-{1,2,3}.txt` |
| 2. semio-base `✉️mutate-semio-base` exhaustive | PASS **43/43** (was 36/43). `parity=0/0` by the spec's recorded no-oracle decision (see §2) | `semiobase-parity-exhaustive-1.txt` |
| 3. framework-os `nx test` | PASS **373/373** (7 files), after one peer-break fix | `framework-os-test.txt` (1 fail) → `framework-os-test-2.txt` |
| 3. os-mcp `nx test` | PASS **52/52** (7 files) | `os-mcp-test.txt` |
| 3. host-rs vitest (os `📦️packages/🟦️typescript`, `vitest run --config ../../🧪️tests/🎚️config/🟦️.ts`, same as R1) | PASS **373/373** (7 files) | `host-rs-vitest.txt` |
| 3. os-dev `nx test-quick` | PASS **163 passed + 28 skipped** (191, 3 files) | `os-dev-test.txt` |

## 1. Kernel edit-text law

- **Which side is correct:** the product. `ArtifactCodec::edit_text_from_envelope` → `print_edit_lines` has emitted the complete
  append unit since C2b: `edit` header, the indented op line, the `inverse` record, and one `metadata` record per forward. Its only
  consumer is `FolderTextStorage::append_ops`, and that function's own contract (`🔄️sync/🦀️.rs`) is "one complete print_edit_lines
  unit: edit + inverse + metadata". The strict `.ops` grammar (`replay_ops`) *refuses* an edit that has no inverse record, or
  whose metadata does not exactly cover its forwards. So the old "one header line + one op line" text could not be appended to a
  document and replayed. The stale parts were the law and the field docstring.
- The law (`🏪️store/🧪️tests/🔬️unit/🦀️.rs`) now asserts the exact 4-line unit: `edit …`, `  set-n n=9`, `inverse …set-n n=4…`, `metadata …`.
  It also asserts the round trip the unit exists for: `parse_document_text(dsl, ops + edit_text)` succeeds under the strict parser
  and replays to `n = 9`.
- Updated the `edit_text_from_envelope` field docstring in `🏪️store/🦀️.rs`. h4 was messaged about both narrow edits.
- **Found but not fixed (handed to h4, whose files these are):** `cargo test … --lib --features sync,ureq -- sync` fails 4 of 6
  libtest runs.
  - Failing laws: `actor_tests::fixtures_replay_matches_expected_events` ("seed snapshot for fixture-backlog|basic on disk not
    satisfied before 5s deadline"), and sometimes `detach_drains_pending_outbound_operations`.
  - Both pass serially (`--test-threads=1`: 61/61) and isolated (3/3, 0.07 s). Nextest also passes, full lib 1189/1189.
  - A `sample` of a stuck run shows every `semio-pool-worker` idle in `Condvar::wait_timeout`, with the test parked in its
    current-thread `block_on`. So the pool is not saturated; the actor's first turn is never scheduled (lost wake).
  - A `[DEBUG]` probe on `ActorRunner::submit_exact`'s Pool-terminal branch never fired, so the 8-retry saturation path is ruled
    out. The probe was removed.
  - Captures: `actor-debug-2.txt`, `actor-sample.txt`, `actor-run.txt`, `kernel-sync-serial.txt`.

## 2. semio-base fixture location

- **Verdict: the spec is authoritative and matches the taxonomy. The adapter was wrong.**
  - The feature declares `shared://🧬️mutations/📸️set-snapshot/✉️replaces-the-envelope-wrapping-a-value-subset/…`.
  - The test platform resolves `shared://` to `<owner>/${testFixturesDirName}` and `asset://` to `<owner>/${exampleAssetsDirName}`
    (`🧪️test/🟦️.ts` `resolveFixtures`). In `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` those are
    `🧫️fixtures` and `🖼️assets`.
  - The vector really lives at `✉️base/🧫️fixtures/🧬️mutations/📸️set-snapshot/✉️…/{📸️snapshot/⬅️before,➡️after,🦠️mutation,🔺️diff,🎯️outcome}/🔣️.json`.
    The production unit test (`🧬️schema/🧬️mutations/📸️set-snapshot/🧪️tests/✉️…/🦀️.rs`) `include_str!`s it from the same place.
  - The adapter read `asset://🏅️standards/…/🧬️schema/🧬️mutations/📸️set-snapshot/🧪️tests/✉️…/…`, a pre-rename unit-test path that is
    not a fixture location and is not declared in the feature.
- Fix: in the adapter, `LEAF_DIR` is now `shared://🧬️mutations/📸️set-snapshot/✉️replaces-the-envelope-wrapping-a-value-subset`, and
  `leaf_before_uri`/`leaf_mutation_uri` compose from it (no aliases, no fallback). Two stale prose references now name the
  `🧫️fixtures/…` leaf and `shared://`: the feature narrative and the `🔮️oracles/🔣️.json` no-oracle rationale. No file moved; every
  file was already where the spec says.
- Result: `bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts parity exhaustive --owner 🗄️stdio --case ✉️mutate-semio-base`
  gives **43/43** passed, 0 failed, 0 errored.
- **Third-party oracle: not run, and it is not added. This contradicts the brief.** The case carries
  `@no-oracle-semio-envelope-routing`, and `✉️base/🔮️oracles/🔣️.json` has `oracles: []` plus that recorded decision. The rationale:
  no third party reads `.dsl.semio`/`.pack.semio`, and a routing oracle over eighteen opaque arms would be "our own answer on both
  sides". So the runner executes the subject phase only, and `parity=0/0` is the correct result, not a gap in the run.
  - The only honest third-party route: a serde_json carrier reader like the `📐️cad`/`📑️document`/`🔺️mesh`/`🖊️drawing`/`🧊️brep`
    `serde-json-semio-*-carrier-reader`s. It would cover only `set-snapshot`/`no-mutation` over the committed JSON vector, as its
    own case. It would first need a JSON bridge for `SemioSnapshot`/`SemioMutation`, which the rationale names as the blocker.
  - Recommended follow-up: the subset's contract debt is 26 rows (`semiobase-contract-case.txt`). It includes
    `Unknown mutation catalog @mutations-semio-v1-base`, the validator's "subsetDirectoryName must start with ✳️" (left over from the
    `✳️any`→`✉️base` rename), 19× "requires a third-party-library … none is registered", and 19 kinds without a wire record. No
    `missing-fixture` breach remains for this case.

## 3. OS suites

All four suites are green (table above).
- framework-os was 372/373 at first. A peer's in-progress change widened `OperationCompleted.revision` to `bigint` (source
  `💻️os/🟦️.ts` and the codec-vector law were already updated). The unsolicited-completion law still asserted `toBe(4)` and cast
  `revision: number`. I made the one-line obvious fix (`bigint` / `4n`) in `💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts`. Rerun: 373/373.

## Other

- Removed two committed `[DEBUG]` console logs from the test platform's host preparation
  (`🧪️test/🏃️execution/🎬️scenario/🟦️.ts`, "Preparing/Prepared … host"). They printed on every parity/subject run.

## Processes (pids)

None left running. Every run was in the foreground. The copied kernel test binary used for `sample` was deleted.

## Files changed (R2)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs`: codec edit-text law
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`: `edit_text_from_envelope` docstring
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧪️tests/✉️mutate-semio-base/🦀️.rs`: `shared://` leaf URIs
- `…/✉️base/🧪️tests/✉️mutate-semio-base/🥒️.feature`: narrative fixture reference
- `…/✉️base/🔮️oracles/🔣️.json`: rationale fixture path
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏃️execution/🎬️scenario/🟦️.ts`: `[DEBUG]` logs removed
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts`: bigint revision assertion (peer-break fix)
