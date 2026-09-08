# Canonical Test Source Evidence Integration

## Observed Consumer

The root policy function `toolJobArtifactEnvelopeRejectionTransferExact` inspects both production Store declarations and named Rust law bodies inside a single source string. Store now correctly declares its tests through explicit `#[cfg(test)]` and `#[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;`. The root `runToolJobCoverage --p2a1-only` dispatcher still supplies only the raw Store source. Other source-policy functions may have the same assumption and require an audit.

`policyReadFileSafe` is a general raw file reader used throughout repository policy; it should not silently turn source reads into synthetic inline test modules. The discovery library already exports `inspectRustModuleGraphFacts` and `inspectRustModuleGraph`, with explicit module paths and source contexts. Canonical test evidence should preserve the defining source path and test scope and follow the actual module wiring. Checks that prove a law exists must inspect that resolved evidence; production-only checks must continue to inspect production source.

## Integration Requirements

- Keep executable test bodies in their canonical files; do not restore inline modules or compatibility proxies.
- Resolve only actual declared module edges, with canonical destination validation, cycle handling, source identity and cancellation/progress where the operation is expensive.
- Preserve hostile source mutations and the original behavioral assertions. Do not hide missing evidence or broaden a passing condition to accommodate missing bodies.
- Cover module wiring and evidence selection with language-neutral fixtures and an independent compiler/parser oracle where applicable.
- Verify suites requiring source/context arguments through their real dispatcher or exactly the same argument construction. The initial all-function probe omitted required arguments for 17 functions; those results are invocation errors, as recorded in the runtime audit.

## Current Validation

The root is running the public Bun/Nx `workspace:verify-interactivity -- tool-jobs --p2a1-only --self-test` dispatcher and separately rerunning nine repository-context suites with the actual workspace root. Runtime results will be retained in this ticket before generated logs are removed.

- Real dispatcher RED: `bun nx run workspace:verify-interactivity -- tool-jobs --p2a1-only --self-test` reached the extracted canonical suite and failed its `valid Store rejection transfer was rejected` baseline assertion. Nx recorded the failed target with a 1.2-second task run. The source reader and fixture arguments were supplied by the existing dispatcher.
