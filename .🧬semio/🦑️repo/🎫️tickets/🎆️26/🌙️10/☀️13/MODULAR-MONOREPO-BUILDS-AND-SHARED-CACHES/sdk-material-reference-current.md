# SDK Material Reference Correction

## Scope and Status

This is a staged read-only report API correction. Physical leases64 funding remains blocked. No test, build, SDK publication, image open, new Original or retirement was run for this change. The new tests were authored before implementation; their expected predecessor failure has not been executed and is not reported as a genuine RED. Current active SDK remains `db49db21d1298457ae5b0278faf7968b87a5e84eb21eeafca6d79686e24e8f77`.

Read-only inspection of that actual SDK's captured manifests confirmed the material module is absent from both closures: authorizer 120 Source files, compiler 53 Source files, zero `📊️material` paths. These staged edits do not change either immutable captured closure or the installed SDK. No shared compiler, authorizer, patch, provision, daemon or activation Source was edited.

## Exact Defect and Staged Correction

The old `withNxSdkMaterialReportV1` derived its base directory from the generation's patched package/lock/patch inputs. The new pristine architecture intentionally stores a different dependency-only base digest in `.semio-sdk-material.json`; recomputing the patched identity selects the wrong or absent base.

The material reference now has a closed schema: version 1, kind `immutable-binary-objects`, and exactly one canonical lowercase SHA256 snapshot digest. The report captures all exact generation installation inputs plus authorizer Source, authorizer compiler and material-reference bytes under the real provision/generation shared guards. It recomputes the actual installed generation identity and validates its authorizer receipt. The reference bytes therefore participate in the authenticated generation identity; malformed, extra-field, changed-input and changed-reference values refuse.

Only this authenticated reference selects the canonical `tools/sdk/<digest>` report path. Path construction is pure and has explicit POSIX/Windows neutral fixtures. The exact `nx-sdk-snapshot:<digest>` SH is acquired before the first referenced-base physical validation/read, and remains held through base input/receipt/binary verification, full physical inspection and the caller callback. Inside that scope the report independently recomputes identity from the referenced snapshot's own actual inputs; it does not derive a base name from patched generation inputs. Metadata capture also has finite total file/byte ceilings in addition to the existing per-file reader ceiling.

This supplies no retirement authority and changes no physical data. Unknown and retained bases remain charged. It does not resolve other early base openers or authorize base eviction.

## Staged Independent Qualification

The existing material test file now includes:

- Seven closed reference cases: exact reference, extra authority field, wrong kind, wrong version, traversal, noncanonical uppercase digest and absent reference. Ajv is the independent schema oracle; SQLite independently extracts the reference.
- Explicit POSIX and Windows tools roots with literal expected base paths and independent Node path joins.
- A patched generation-input example whose recomputed snapshot identity differs from the declared pristine reference.
- Independent Bun SHA256 framing of the exact generation input map, followed by refusal assertions for changed reference bytes, another changed generation input and changed platform architecture.

All of these are UNEXECUTED at this checkpoint, including TypeScript validation. Actual db49 Native cache BOTH and unchanged ordinary seven-phase pair remain the first execution priority after authentic lease-capacity clearance.

## Changed Source

- `tools/📦️sdk/📊️material/🟦️.ts`: reference parser, pure path projection, generation identity binding, corrected held-base report scope.
- `tools/📦️sdk/📊️material/🧬️schema/🔣️.json`: closed reference and finite metadata capture ceilings.
- `tools/📦️sdk/📊️material/🧪️tests/📥️reference.json`: portable language-neutral corpus.
- `tools/📦️sdk/📊️material/🧪️tests/🟦️.ts`: independent schema/SQLite/path/hash checks.
