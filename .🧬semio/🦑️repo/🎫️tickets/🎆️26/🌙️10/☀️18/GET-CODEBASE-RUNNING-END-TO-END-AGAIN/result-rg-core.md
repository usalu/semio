# Result rg-core: framework crates split under the source gate (4,000 active lines / 32 files / 262,144 bytes)

Executor `rg-core` (Sonnet 5.5, foreground, no sub-agents, no modifying git command). Tools in `rg-core-tools/` (`closure.ts` = canonical `readRustSourceClosureV1` closure of one crate, prod and test cfg; `rgl.py`, `deps.py`, `repoint.py`, `fixturize.py`, `widen.py` = registration, importer re-pointing, fixtures to run time; `*carve*.py` = the per-crate carves; `fwcheck.sh` = lock refresh plus `bun repo cargo check`). Logs under `🗑️generated/rg-core-*`.

## 1. Gate measurements (canonical reader, working tree; ratio = max(bytes, files, lines) / gate; prod / test cfg)

| crate | before | after | binding |
| --- | --- | --- | --- |
| `semio-framework-course` | 1.004 / 1.004 | 0.505 / 0.505 | bytes |
| `semio-framework-value-derive` | 1.104 / 1.140 | 0.334 / 0.345 | bytes |
| `semio-framework-pack-json` | 0.965 / 1.025 | 0.874 / 0.935 | bytes |
| `semio-framework-tool-run` | 1.169 / 1.262 | 0.640 / 0.689 | bytes |
| `semio-framework-pets-staging` | 1.168 / 1.438 | 0.504 / 0.776 | lines |
| `semio-framework-replication-mutation-messages` | 1.089 / 3.399 | 0.561 / 0.887 | bytes |
| `semio-framework-job` | 1.088 / 1.250 | 0.882 / 0.981 | bytes, files 0.875 |
| `semio-framework-value` | 1.795 / 3.470 | 0.835 / 0.985 | bytes |
| `semio-framework-value-native-decoding` (coordinator request) | test 1.275 | 0.255 / 0.289 | bytes |
| `semio-framework-value-retained-clone` (coordinator request) | test 1.010 | 0.570 / 0.968 | bytes |
| `semio-framework-os-store-testing-fixtures` (ceiling restored to 1700) | 1730 lines | 1565 lines | lines |

New crates (all registered: `🧰️framework/Cargo.toml` or root `Cargo.toml` members, `📋️build-owners.json` row, `📋️project.json`; framework value family + job + pack-json + tool-run + replication also have a corpus row in the crate-boundaries law): `course-presence`, `value-derive-expansion`, `pack-json-float`, `pack-json-source-sink`, `tool-run-tick-wire`, `pets-staging-draft`, `replication-mutation-retained`, `job-payload`, `value-registry`, `value-schema`, `value-persistent`, `value-edit`, `value-native-owned`, `value-native-decoding-tests`, `os-store-testing-prelude` (all `semio-framework-…`).

## 2. Carves

- course: `👥️presence` (24 KB + the 106 KB schema it embeds) is its own crate; its unit laws mount in it, the oversized presentation fixture is read at run time (`fixture_text!`). The proctor base crate and the shared-presence subject adapter import it; the course oracle contribution gained `subjectHostPackages`.
- value-derive: the whole expansion tree (ToValue/FromValue, canonical tree, native receive) is the plain library `value-derive-expansion`; the proc-macro crate keeps entry points, retained-clone, factory, projection, equality.
- pack-json: float writer (no importers but 3) and the borrowed source sink (3 importers, re-pointed).
- tool-run: tick encoder and funded receiving (`tool-run-tick-wire`), 5 importers re-pointed; the companion tests re-point to it.
- pets-staging: draft, spacing, schedule, attention, locomotion form `pets-staging-draft`; their unit laws (which drive the stage) mount in the upper crate; the source-law test reads the sources at run time.
- replication messages: retained mutation family (field, patch, diff, key, span, diagnostic) is `replication-mutation-retained`; fixtures run time.
- job: the payload custody subsystem (pages and ledger, writer and grant, staged writer, in-place builder) is `job-payload`; it runs against the `JobPayloadStep` trait that `StepContext` implements (dependency inversion), the builder's last-page supply became the extension trait `RetainedPayloadBuilderSupply` in the job core. 214 importer files (83 manifests) re-pointed to `semio_framework_job_payload`.
- value facade: `value-registry` (slots, request owners, exact key lookup), `value-persistent` (persistent lists, concat, slice equality, tree equality, and the `ToValue` impl of `PersistentList` moved out of intrinsic), `value-edit` (indexed/named edit, commit, heap sort), `value-schema` (schema export, intrinsic JSON, GraphQL Int64). The facade `semio-framework-value` re-exports these modules explicitly (`pub use semio_framework_value_persistent::{list, retained_equality}` etc.): the facade is the public API of the value family (7,000 importers); this is the existing pattern of that file, not a glob, and is a decision flagged for review.
- value native decoding (broken dependency cycle found on the way): the `owned` receiver subtree needed `DslValue`/`Number` from intrinsic, which depends on the decoding core, so the core never compiled. `value-native-owned` now holds the `NativeReceive`/`NativeReceiver` traits and all receivers and composes the `native_decoding` module (core plus `owned`) that the facade, the edit crate and the derive output use. `retained_clone_grant_admits` moved down into `value-control-contract`.
- Coordinator requests: the `testCaps` overrides of the crate-boundaries law and their schema support are deleted; `value-native-decoding` and `value-retained-clone` test closures are inside the global caps (laws moved into `value-native-decoding-tests`, fixtures read at run time); `os-store-testing-fixtures` lost its `prelude` into `os-store-testing-prelude` (1565 lines) and its ceiling is back at 1700; corpus rows added for every new framework crate. `bun test ./🧰️framework/🛍️products/💻️os/🧪️tests/📦️crate-boundaries/🟦️.ts` from the repo root: 127 pass, 0 fail.
- Also fixed in the value family (found by compiling): `controlled_semantic_admission` visibility (derive-expansion), `SharedIssuer` enum/struct mismatch in shared-issuance, `next_demand` missing on the dynamic and retained-clone owner cursors, stale mutation-diff imports, glue name gaps of the decoding crates.

## 3. Verification

- Gate measurement: `bun rg-core-tools/closure.ts <crate dir> both` for every crate above (numbers in section 1): all inside the gate, prod at or below 0.90 except none (job 0.882, pack-json 0.874, value 0.835); all test cfg at or below 0.985.
- `bun test ./🧰️framework/🛍️products/💻️os/🧪️tests/📦️crate-boundaries/🟦️.ts` (repo root): 127 pass, 0 fail (overrides deleted, ceiling 1700, corpus rows for all new framework crates).
- `rustfmt --edition 2021 --check --config skip_children=true` parse: 0 parse errors on the 151 changed Rust files.
- Cargo (`rg-core-tools/fwcheck.sh`, offline lock refresh then `bun repo cargo … check -p … --lib --tests --keep-going`, one pass takes about one hour under the fleet queue, load average 46). Passes fw9 to fw16 each removed one layer of errors; what the last complete pass (fw16, `🗑️generated/rg-core-fw16.log`) still reported:
  - `value-retained-map` (30 errors, not part of my splits): glue gaps (`retained_clone_grant_admits`, `nested_retained_clone_*` not in its `retained_clone` alias), `next_take_demands` missing on its owner, `K` vs `usize` mismatches: a half-finished refactor at HEAD.
  - `value-persistent` (mine, 19 errors): `T: RetireOwned` bounds after the `SharedIssuer` enum to struct change; fixed after fw16 (bounds added to the persistent family, `ToValue`/`NativeEncodeControl` imports in the glue, `RetainedCloneBinding::demands` renamed to `close_demand`), rerun fw17 pending (`🗑️generated/rg-core-fw17.log`).
  - `value-retained-clone` tests (3): the `crate::RetainedClone` derive sites (`#[value(crate="crate")]` added after fw16).
  - Crates whose pass was blocked behind these (facade `value`, `value-edit`, `value-native-owned`, `value-native-decoding-tests`, job, job-payload, replication, messages, retained, tool-run-tick-wire, pack-json family, derive-expansion): NOT compile-verified. Earlier passes did compile `value-derive-expansion`, `value-derive`, `value-registry`, `value-schema`, `value-native-decoding` and the decoding core after the fixes in section 2.
- The coordinator's teaching proctor check (`check --manifest-path 🎓️teaching/Cargo.toml -p <proctor crate>`) was NOT run: it needs the value family to compile first.
- Root-workspace crates (course, course-presence, pets-staging, pets-staging-draft, os-store-testing-fixtures, os-store-testing-prelude) are not checked: `update --workspace --offline --manifest-path Cargo.toml` fails on a foreign defect (`os-kernel-dsl-derive-tests` needs a `testkit` feature of `…-derive-expand`).

## 4. Open items

1. Re-run `pl check -p …` for the list in `rg-core-tools/fwcheck.sh fw16` and fix residual errors with `rg-core-tools/widen.py` (privacy) and the glue names; then the teaching proctor check requested by the coordinator.
2. Lock refresh for every workspace that now sees new crates (framework done offline; root, teaching, ✏️s plugin workspaces, hub pending).
3. Test cfg of `value-native-owned`, `value-native-decoding-tests`, `value-retained-clone` tests use `#[derive(crate::RetireOwned)]`; sites got `#[value(crate="crate")]` but are not compiled.
4. `value` facade test closure is 0.985 and `job` test closure 0.981: tight; further split candidates are the projection tests and the `OriginalJobFixtureCaller` module.
5. The corpus row of every new crate uses the generic forbidden-dependency list; rows of crates that legitimately depend on the facade (`job-payload`, `retained`, `source-sink`, `tick-wire`) omit `semio-framework-value`.
