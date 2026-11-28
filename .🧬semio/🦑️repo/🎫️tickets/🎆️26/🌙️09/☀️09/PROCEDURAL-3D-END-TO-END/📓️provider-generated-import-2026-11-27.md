# Provider generated import (2026-11-27)

## Root cause
`scriptProviderGroupV1` refused the `cargo-workspace` provider walk (`build.closure.provider-generated-import`, target `@semio-tech/ui-rs:generate`).

Instrumented (temporary `[DEBUG]` log, removed) the single offender:
`🧰️framework/🔨️modules/🛂️manifest/🏷️localized-labels/🟦️.ts`, created 00:58, value-imported `SHELL_LOCALES` and `SHELL_TERMINOLOGIES` from `../🤖️generated/🎚️ui-axes/🟦️.ts`, an output of `ui-rs:generate`.

Chain into the walk: provider walk -> plugin store (`🏪️store/🟦️.ts`) -> `📥️installation/🟦️.ts` -> `localized-labels` -> generated ui-axes. Every other manifest module imports ui-axes with `import type` only (erased, not walked).

## Fix
`localized-labels` now reads the locale and terminology ids from the owned source `🖱️ui/🎚️axes/🔣️.json` (JSON import, same source the generator reads) and keeps only `import type` from the generated file. The check is unchanged.

## Verification
- Dry run `@semio-tech/s-composition-laws-rs:test-generation3d-app-laws`: graph inference passed ("graph: plan answered and verified", 1 stale task); the later admission step then ended with an unrelated internal error ("Actual declared requested Source scope required", ref 7566ad79) after a 30 min graph read under concurrent edits.
- Laws result: deferred. The retry run was cancelled (SIGTERM) and stopped by the coordinator to free memory; the coordinator reruns `app-laws-retry.sh` later.
