# EN 1995 Mutation Unit Ceiling

## Carve (source level, done)

The 206 payload variants, both ordinal orders, wire twins, edit rules and bridge are unchanged. Two non-dispatch owners left `semio-s-artifact-norm-en1995-mutations`, following the EN 1996 precedent:

- `semio-s-artifact-norm-en1995-mutations-edit-rules` (new, `🧬️mutations/🧭️edit-rules/📦️packages/🦀️rust`) owns `EDIT_RULES`, `ID_REMOVALS`, `resolve_edit`. The editor (7 files) now imports it. Its two unit tests live in the bridges test package.
- `semio-s-artifact-norm-en1995-schema` now assembles the mutation `FacetLeaves` itself from the four twin crates. `MUTATION_LEAVES` is gone.

The dispatcher lost six production dependencies: four twins, `semio-framework-schema-registry`, `semio-s-plugin-norm-app-surface`. The concrete inverse extraction from 2026-11-25 is retained.

Updated: family Cargo.toml and script.ts, four lockfiles (family, hub, fixture-sweep, norm bridge; the bridge lock pruned the four orphaned twins), build-owners.json, the inspector-admission fixture.

## Checks run

- `en1995-mutation-owner-oracle.py`: 206 JSON/Rust/Protobuf leaves, 126 native ordinals, 31 owners, 11 leaves in the largest owner, 206 local inverses. Passed.
- Norm Bun suites via `repo test-file`: inspector-admission 17, insert-bindings 2, payload-bindings 3, edit-bindings 1, mutation-aggregate 30, mutation-label-census 2, schema-sync 4, package-runner 1, wire-twins 11956. All passed.

## Measurement (not complete)

Source of truth for peaks: `peakRssBytes` of the unit `miss` record in `.cache/b/r/m/u/x/<lane>.jsonl`, read with `en1995-peak.py <sinceMs>`. Earlier one-worker records of this crate: 1,075 MiB (77 s) and 1,395 MiB (66 s). The 2,258 MiB figure came from the component driver.

Final run, one worker, through the capacity gate (`--memory=2663 --cpu=1`), after the job-context carve landed:

- `repo cargo ... build -p semio-s-artifact-norm-en1995-mutations --target wasm32-wasip2`: rc 0. Unit `peakRssBytes` of the mutations crate: 925 MiB (ceiling 1,536 MiB; earlier one-worker records of the uncarved crate 1,075 and 1,395 MiB; component-driver figure 2,258 MiB). Largest other EN 1995 unit in the same run: `en1995-diff` 1,381 MiB, then `en1995-model` 1,002 MiB; all owners below 1,536 MiB.
- `repo cargo ... test -p` (native): edit-rules rc 0 (no tests of its own; compiles), schema rc 0 (0 tests), mutations-bridges rc 0 (4 passed, 0 failed: edit-rule kinds, value-tree edit resolution, kinds-catalog/vector round trips).
- Earlier blockers (job-context source boundary, repo tooling size limit, E0432 lock entry in fixture-sweep) were resolved or cleared; one test attempt was cancelled by the host warden and rerun.

Logs: `🗑️generated/en1995-unit-ceiling` (delete when done).
