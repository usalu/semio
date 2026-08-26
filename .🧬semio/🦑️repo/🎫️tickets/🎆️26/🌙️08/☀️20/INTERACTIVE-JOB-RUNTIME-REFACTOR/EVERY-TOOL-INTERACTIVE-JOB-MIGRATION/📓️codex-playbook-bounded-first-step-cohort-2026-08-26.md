# Playbook Bounded First-Step Cohort

Date: 2026-08-26  
Status: source/static cohort complete; full-operation acceptance remains blocked by the shared typed-command foundation.

## Scope

The Playbook editor declares nine typed commands and one separate `import-media` route. The nine commands now have a schema-first language-neutral ledger, an owner-local bounded-first-step proof, and `Migrated` declarations. The importer remains explicitly fail-closed pending its own retained factory and is not counted as migrated.

## Implementation

- The proof and fixture bind the exact controller `s.playbook.playbook@1/*#editor`, document schema `playbook.program`, `BoundedFirstStepCommandJobFactory`, and all nine command ids.
- The fixture is parsed by the owned line parser and the test-only `serde_json` oracle, then compared with `PlaybookCommand::every_command()`.
- `addStep` derives both id and title from the admitted operation id without reading or cloning the document.
- `addBlock` uses the explicit step id or the schema-owned initial step id `s`; it no longer clones the composed Playbook working scene to discover the first step.
- All nine declarations are `InteractiveJobClassification::Migrated`. The shared gate still reports them as `owner-local handler proof exists but full prepare/job/commit operation is not bounded`, which is correct until `TypedCommandFullOperationJobFactory` becomes runnable and independently accepted.

## Verification

- `rustfmt --edition 2021 --check` on the five changed Rust files: PASS.
- Language-neutral fixture parse/cardinality: PASS, `playbookTools=9 schema=playbook.program`.
- Production command-body census for whole Playbook scene reads/serialization: PASS; remaining `steps()` matches are confined to `#[cfg(test)]` law bodies.
- `git diff --check` on Playbook, the root gate script, and launch configuration: PASS.
- Live tool-job ledger: expected RED globally, `boundedRows=0`; all nine Playbook commands are recognized as owner-local proofs and remain blocked by the shared full-operation predicate rather than missing/forged proof identity.

Cargo, Nx, Wasm, browser, and runtime checks were not run while overlapping Rust packets were active.

## Remaining Gate

Accept these nine rows only after the shared typed-command job proves bounded preparation, lane transfer, reducer execution, validation, publication, ACK/close, exact handback, and production activation. Migrate Playbook `import-media` separately through a retained media factory.
