# L5 TypeScript lane (2026-11-21)

Execution: `repo run @semio-tech/puzzle-js:test --lang=en` queued 300s on "Waiting for the repo graph service"; fell back to native bun (runtime in ~/.cache/semio/runtimes/bun/*/bun). The `📜️script.ts` entry points need a parent-process envelope, so the underlying commands were run directly (vitest config / the exported run*Script functions via l5-audit.ts, l5-verify.ts, l5-debug*.ts in this folder).

| Target | Native command | Result |
| --- | --- | --- |
| @semio-tech/puzzle-js:test | bun vitest.mjs run --config puzzle/🧪️tests/🎚️config/🟦️.ts | 14 files, 54 tests pass |
| @semio-tech/puzzle-js:publication-authority-audit | bun l5-audit.ts (runPublicationAuthorityAuditScript) | pass; 3 owners, windowOwnershipCases=7 (was FAILING) |
| @semio-tech/puzzle-5d-react:test | vitest run --config 5d-react/🧪️tests/🎚️config/🟦️.ts | 1 file, 1 test pass |
| s-puzzle-composition-tests:verify-puzzle-fill-policy-self-tests | bun l5-verify.ts puzzle-fill-policy-self-tests | checks=73 pass |
| s-puzzle-composition-tests:verify-browser-contribution | bun l5-verify.ts browser-contribution | pass |
| tool-job-puzzle-reserved-routes (framework tool-job-coverage original-host test) | bun test ./🧰️framework/.../original-host/🧪️tests/🟦️.ts | 19 pass, puzzleSourceCases=37 |

## Fixes (publication-authority-audit was stale vs the refactored editor trees)
- `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🧫️fixtures/🔏️publication-authority/🔣️.json`: 2d mounted += `📸️snapshot/🧰️retained-tool-ids-and-scene-builders/🦀️.rs`; 5d mounted: removed 4 dead modules (paste-job, import-job, command-variants, retained-command-works), added `🖌️session/🧱️support/🦀️.rs`, `🧰️kit/💾️store-preparation/🦀️.rs`.
- `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️typescript-package-commands/🟦️.ts`: oracle needles updated to current code: 2d close_step/generic `puzzle2d_dispatch_emit::<A|Self>` call sites; 3d config envelope error (`ValueError::literal(...OwnershipLimit...)`), `self.owners.close_step(effect)`, single-line authority-generation hostile mutation; 5d reserved-route guard + `admit_reserved_work` decode anchor and hostile mutation.

## Incident
Disk (/home) hit 100% during the fixture write; the fixture json was truncated to 0 bytes. Restored from `git show HEAD:` (14731 bytes) + my edits. Any uncommitted edits other agents had made to that fixture before ~23:48 were lost (diff vs HEAD was only mine afterwards; unknown earlier). Re-check the fixture if someone reports a regression. Disk is nearly full fleet-wide (109M free) from multi-GB cargo logs under 🗑️generated.

## Not run / remaining
- compose5d-preparetopologymodel (.tsx), time-travel, renderer-boot-variants lifecycle scenarios: renderer-boot-variants is part of puzzle-js vitest (passed); time-travel/compose5d belong to L4/Rust-invoked.
- `repo run` path itself unverified (graph service queued).
