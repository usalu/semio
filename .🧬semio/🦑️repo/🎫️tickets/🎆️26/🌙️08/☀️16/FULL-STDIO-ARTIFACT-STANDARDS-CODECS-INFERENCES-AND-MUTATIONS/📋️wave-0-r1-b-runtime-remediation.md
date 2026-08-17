# Wave 0 R1-B Runtime Remediation

## Owned Scope

- `🧰️framework/🔨️modules/🚪️io/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/**`
- Non-plugin, non-stdio runtime callers required to propagate the fallible artifact-store constructor and format/document-codec queries.

No framework plugin or stdio path was edited.

## Runtime Remediation

- Added the shared `ArtifactAssemblyTransaction` and `begin_artifact_assembly()` transaction boundary in the store module.
- Added `ArtifactAssemblyRegistryPlan` and `commit_artifact_assembly_registry_plan()` for composers, subset validators, format descriptors, document codecs, and dialect migrations. The aggregate commit obtains every write guard, preflights every candidate, then performs only infallible insertions.
- Made standalone registry registration acquire the same transaction; added `_in_assembly` registration/preflight forms for coordinated assembly.
- Made remote ingest clone, apply, drain, and validate its candidate DAG before assigning it. Duplicate mutation IDs now require full envelope equivalence.
- Made snapshot merging preflight all collisions, validate candidate history, and assign its candidate envelope/DAG/indexes only after success.
- Validated checkpoint composition pins, rederived checkpoint identities from pin content, rejected malformed/dangling pins, and made pin updates candidate-based.
- Bounded IO wire JSON, source count, dialect component size, and interned dialect count with typed limit errors.
- Changed resource resolution to return only budgeted, cancellable payload access. Resolved access no longer exposes arbitrary unbounded handles.
- Made fallback IO perform the same local subset validation as direct IO.
- Rejected unknown format identifiers instead of silently filtering/falling back.
- Removed owned silent codec fallback and constructor/query panics.

## Exact External Runtime Caller Inventory

| Caller | Propagation |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄sync/🦀️component.rs` | Read/write document codec lookup now returns a typed sync error; test fixtures explicitly unwrap valid construction. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs` | `VcsVersionGraph::with_store` propagates failed store construction instead of panicking. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs` | Artifact import maps failed store construction into `SpaceZipError::Pack`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs` | Run persistence maps failed store construction into its runtime error. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs` | Media conversion propagates format lookup errors and rejects missing format descriptors. |

Framework plugin and stdio callers are intentionally absent from this inventory and were left for their respective lanes.

## Repaired Kernel Tests

- Corrected the IO conflict test's expected export-direction key.
- Made an empty DSL fixture sweep a valid no-candidate sweep rather than an assertion that absent input files exist.
- Replaced the dangling alternative-checkpoint fixture with an assertion that construction rejects it.
- Added adversarial coverage for rejected remote ingest state preservation, duplicate remote payload mismatch, snapshot merge preflight atomicity, and composition-pin identity/atomicity.

## Verification Evidence

| Gate | Evidence |
| --- | --- |
| `cargo test -p semio-framework-os-kernel --lib` | Passed: 904 passed, 0 failed. This was run before the final sync-feature-only test propagation and an IO comment-only edit. |
| `cargo check -p semio-framework-os-kernel --lib` | Passed. |
| `cargo check -p semio-framework --lib` | Passed. |
| `cargo test -p semio-framework-os-kernel --lib --features sync` | Unconfirmed after fixes. Its first run exposed 52 test compile errors from the now-fallible constructor; those test call sites were repaired. The rerun was stopped while waiting on the shared Cargo build lock, per the freeze instruction. |
| `bun nx run @semio-tech/framework-os:test` | Unconfirmed. The inferred target recursively invokes the package `test` script, which invokes the same Nx target, so no test runner was reached. No task configuration was changed in this lane. |

The successful Cargo checks emitted pre-existing warning classes (unexpected `js` cfg, unused imports/qualifications, and dead code); they did not fail the gates.

## Handoff

The shared transaction API is ready for the plugin/schema assembly lane. Whole-plugin atomicity still depends on that lane staging all of its own domains into an equivalent prevalidated commit unit; this lane did not edit or wait for plugin/stdio integration.
