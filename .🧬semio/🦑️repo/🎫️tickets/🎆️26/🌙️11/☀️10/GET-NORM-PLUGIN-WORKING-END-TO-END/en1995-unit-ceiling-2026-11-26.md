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

Later attempts (logs in `🗑️generated/en1995-unit-ceiling`):

- The `E0432` is resolved at lock level: hub, family and norm-bridge locks already list `semio-os-store-schema-exports`; the fixture-sweep lock was missing it and now has the package and the kernel edge.
- An admitted run then stopped before the compiler: `semio-framework-job-context exceeds its source boundary: 262309 of 262144 bytes` (first 262180). The cause is a concurrent, uncommitted edit of `job/⏱️context/🎟️source/🦀️.rs` by another agent; not touched here.
- Before that, repo tooling refused for a minute because `🕸️graph/🕰️generations/🟨️.mjs` was edited past 65,536 bytes (now back under).

No post-carve peak is recorded yet, and the EN 1995 Rust tests (edit-rules, schema, bridges) have not run because every cargo run stops at that boundary. Retry once the job-context file is back under 262,144 bytes: `en1995-measure-retry.sh semio-s-artifact-norm-en1995-mutations m1`, then read `en1995-peak.py <sinceMs>`; afterwards run the same script with `-p` set to `semio-s-artifact-norm-en1995-mutations-edit-rules`, `semio-s-artifact-norm-en1995-schema`, and test-run the bridges package one at a time.
