# Dep-info without base directory (2026-11-24)

## Symptom
Descriptor emitter prerequisite build ended with `Cargo dep-info has no base directory` (r21, `completedCargoClosureRosterV1`).

## Root cause: parser/validator bug, not a torn file
- Lane `.cache/b/r/m/u/l/qda3ae2`: all 579 `.d` files are complete (trailing newline, rule line, checksum lines). `semio_ui_contract_identity-*.d` (the ENOSPC victim) is intact (1201 bytes, both hashes).
- Offending units: `semio-os-store-space-schema` and `semio-os-store-durable-group-wire`. Their `.d` lists absolute compiler-resource captures `semio-compiler-resources/<ms>-<pid>-<n>/observation.json` of an earlier compilation (pid 9118, before the image was recreated). Those capture directories are gone; the units were restored from the unit cache together with their `.d`.
- `CargoProvenanceSession.writeInvocation` (`native-build/🟦️.ts`) chose the base directory among the Cargo.toml owners and required EVERY dep-info source, including absolute ones, to exist. A single absent absolute capture voided every owner, so `baseDirectory` became `null`, and `cargoDepInfoResolvedSourcesV1` threw later in the retention roster.
- Absolute sources do not depend on the base directory, so they must not take part in choosing it. Absent captures are already recorded as absent inputs (kind null) and accepted by the roster.
- A simulation of the base choice over the cargo messages of the r21 log found exactly one failing unit (space-schema; the second one was not in the log excerpt).

## Fix
`native-build/🟦️.ts`: the base validity loop skips absolute sources (`if (isAbsolute(source)) continue;`). Missing relative sources still yield `null` (fixture `incomplete-roster` unchanged).

No torn-dep-info invalidation was added: no torn file exists, and a truncated relative path is still refused by the unchanged missing-relative-source rule.

## Test
`native-build/🧪️tests/🟦️.ts`: "compiler dep-info keeps its base when a restored unit names an absent absolute compiler capture". It fails without the fix (`baseDirectory` null) and passes with it, together with the 4 `depInfoBases` vectors. Other failures in that file (path envelope over 256 chars in the test temp dir, `cargo metadata` in the internal dep-info test) are environmental and pre-existing.

## Verification
r22 (`draw-native-component-r22.log`) rebuilt the emitter (`os-plugin-describe-rs`, 337 s) and moved on to the component build ("component-jobs") with the stale-capture `.d` files still in the lane. I did not start a separate component stage.
