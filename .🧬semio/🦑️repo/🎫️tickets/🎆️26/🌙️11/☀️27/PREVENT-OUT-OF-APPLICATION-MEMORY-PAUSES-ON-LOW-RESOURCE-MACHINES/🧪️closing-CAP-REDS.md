FINAL (2026-11-28 00:40: all red host-capacity tests cured; 123 of 123 test files green via `repo test-file`; typecheck 0 errors)

# 🧪 Closing Report, Lane CAP-REDS

The 4 red files of `🎚️closing-MEM-ADMIT.md` section 4 (5 tests), the cause of each, the cure. `R` = repo root. Run: `repo test-file` over the 123 test files of `host-capacity`, `🎟️capacity` and the product's `host-capacity` (the two helper scripts `🔁️restart/📜️child` and `🪶️lazy-graph/🧲️probe` live under `🧪️tests` but are not tests). `bun test` needs the repo root as cwd and one file per process (single-birth envelope); the pinned bun is in `.cache/b/t/m/bun/*/bun`.

## 1. `🪆️scope` (2 tests) and `🔐️inheritance` (1 test)

| Test | Cause | Whose | Cure |
|---|---|---|---|
| `🪆️scope`: census 100 expected, 99 found (2 tests) | Commit 757 (`9b948fc54a4`) split the 1173-line `component-pass/🧪️tests/🟦️.ts` into `🦀️real/🟦️.ts` and `🎭️stub/🟦️.ts` (one literal capacity call each; the monolith held them); the frozen caller census still listed the monolith, now with 0 calls | other agent (the split), pre-existing red; not MEM-ADMIT, MEM-GUARD or MEM-CARVE | census source of truth `🎟️capacity/🧪️tests/🪆️scope/🧫️fixtures/🔣️.json`: the monolith row replaced by the two new files (`callers`, `census.rows`, `files` 52 to 53, `literalCalls` 100 to 101). Assertions untouched: the test still checks every listed file by AST, and the census rows equal the observed rows |
| `🔐️inheritance`: Cargo launcher namespace (1 test) | The test regex demands that the authenticated policy read uses `X.storage,X.control`; `withRepositoryComponentLaneV1` reads it with `options.policyControl??options.control` (the explicit store control, commit `ccac6866cc1`, asserted by `🔒️leases/🧭️order/🧪️tests/🧩️component-driver`) | pre-existing red, not today | regex now admits exactly that one extra form (`options.storage,options.policyControl??options.control`) beside the old one; every other form still fails |

## 2. Product `🌤️followup` (g13/g29)

Not the fixture. The product fixtures (`🧪️tests/🌤️followup/🧫️fixtures/*.json` of the product) were last written Nov 14 and are unchanged; the modified fixture MEM-ADMIT saw is the framework's `🌤️followup/🧫️fixtures/🔣️.json` (decision replay, `memory`/`queue-order` reasons), whose test is green. The cause is code: the staged, not yet committed change in `🔼️upgrade/🟦️.ts` (written 03:52 today with its test `🧪️tests/🔼️upgrade/👥️agents`, "an upgrade whose size is a guess (`rampSlot`) asks for the jobs that fit at once") narrows a ramp-slot upgrade at once, and the narrowed request carries one spare job of memory (`protocol.growSpareJobs`). Before it, the narrowing came only after `wait.bypassMilliseconds`. The quiet g13/g29 grants (class sized by a guess, so `rampSlot` true) therefore grew to `8190 + 936 = 9126 MB`, over the 8192 MB class ceiling of the follow-up contract. Not MEM-GUARD, MEM-CARVE or MEM-ADMIT (MEM-ADMIT's own check, stale trust at 0, was right that it is not theirs). I found no ticket that owns the change.

Cure in `🔼️upgrade/🟦️.ts`, keeping the other lane's behaviour and test: the spare job stays in the ask (it is what the decision needs), but a grant admitted by the immediate ramp-slot narrowing, before the bypass window has passed, is published with `min(decided reservation, full request fixed + jobs * per job)`; a grant that waited past the bypass keeps the decided reservation with the spare (the test "past the bypass window ... the memory of those jobs" still asserts that). Both of the following pass: the product `🌤️followup` and `🔼️upgrade/👥️agents`. Two earlier attempts of mine failed other tests and were reverted (clamp for every narrowing; no spare before the bypass); the file holds only the final change (grep `reserved=asked!==request`).

Open point for that lane: whether the spare job should be held at all by an immediately narrowed ramp-slot upgrade is theirs to decide; my clamp is the smallest change that keeps their test and the committed ceiling contract both true.

## 3. `🖼️image-slack` (product partition table)

Cause: the committed registry row `bun-cache` (priority 91, class `derivable`, "removed last", HEAD `fa8a184f7cb`, schema `🧹️disk-recovery/🧬️schema/🔣️.json`) sits above `replica-store` (85) and `image-slack` (90); the older image-slack test asserted every derivable row below the store. The newer `🧪️tests/🧺️bun-cache` test requires `bun-cache` to be the last writer for bytes and inodes (its priority the maximum of all non-manual rows). The two tests could not both hold; the Bun cache is documented as removed last (it must not vanish under a running install), so the older test was the stale one, not the registry.

Cure: in `🖼️image-slack/🧪️tests/🟦️.ts` the loop over the derivable rows names its one exception (`bun-cache`), and the exception is asserted as strictly as the rest: `bun-cache` stays `derivable`, above `image-slack`, and the maximum priority of all non-manual rows. Nothing was weakened for any other row.

## 4. Verification

- `repo test-file` (123 files): 123 passed, 0 failed (258.6 s).
- `repo typecheck` of `🔼️upgrade/🟦️.ts`, `🔐️inheritance/🟦️.ts`, `🖼️image-slack/🧪️tests/🟦️.ts` and their importers: 0 errors. The census fixture is JSON.
- No git command that modifies the repository was run; a temporary debug test file and a temporary script were created beside the tests and removed again (no `[DEBUG]` left). I started only my own `repo test-file` runs; a long `grep`/`find` of mine was stopped by task id.

## 5. Files

- `R/🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/🧫️fixtures/🔣️.json`
- `R/🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🔐️inheritance/🟦️.ts`
- `R/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🔼️upgrade/🟦️.ts`
- `R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧹️disk-recovery/🖼️image-slack/🧪️tests/🟦️.ts`
