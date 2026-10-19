# Result tst3c: bun suites of the neutral framework modules, hub, teaching, mit-bestand, semio-tech

Executor tst3c (Sonnet 5.5), foreground, at most 2 concurrent `bun test ./<explicit file>` (each under `perl alarm 300`), no sub-agents, no cargo, no wasm, no manifest edit, no modifying git command. A codex ticket edits the course; I re-read before touching `proctor-build` and changed one import line only.

## Scope and totals

Scope (exclusive): every file importing `bun:test` under `🧰️framework` (modules, products except `🦑️repo` and `💻️os`, `📏️script-boundary`, `📦️packages`, `🧪️tests`), `🌎️hub`, `🎓️teaching`, `♻️mit-bestand`, `🏢️semio-tech`. The first enumeration (458 files) missed minified imports (`from"bun:test"` without a space); the corrected scope is **649 suites**.

| measurement | green suites | tests pass / fail |
|---|---|---|
| BEFORE, original 458, plain `bun test` with runner env | 362 / 458 | 2841 / 198 |
| AFTER, all 649, plain (runner env: process envelope, short artifact dir, `--timeout 120000`) | 542 / 649 | 3591 / 259 |
| AFTER, all 649, with environment emulation (emulated root hoist for undeclared packages; ticket artifact dir for the absence/neutrality suites) | 574 / 649 | 3663 / 195 |
| original 458 only, emulated: before 361 / 457 existing files -> after 407 / 457 | | |
| 191 suites found late: first measurement 158 / 192 -> final 167 | | |

`before` for the late 191 is the first measurement (taken after most fixes); no earlier state exists for them.

## What the harness has to provide (found, not source defects)

1. Law suites on `registerBuildTestLawV1` need `SEMIO_SCRIPT_PROCESS_INVOCATION` (`tst3c-tools/envwire.ts` mints the root envelope), `SEMIO_TEST_ARTIFACT_DIR` (short, unique per run; shared dirs make `cold`/lease laws fail with EEXIST), `SEMIO_TICKET_DIR`, `SEMIO_BUILD_MODE=prototype`, `SEMIO_REPO_ROOT`, `--timeout 120000`.
2. `🚮️absence`/`🧱️neutrality` copy modules into the ticket tree: they need `SEMIO_TEST_ARTIFACT_DIR` under `.🧬semio/…/🗑️generated` (real path, no symlink).
3. 49 suites (all of `🏃️process` that is red, hub foundation-source/unit-census, both `neutral-owner` cargo laws, filesystem durability) need live Original issuance/admission/nightly cargo from `bun repo run`. `NX_FORCE_REUSE_CACHED_GRAPH=true bun repo run @semio-tech/framework-process:test-compiler-capacity` stalled in the cache-pin queue until its 15 min alarm; the retry died with `ENFILE` (system file table exhausted fleet-wide, see coordinator rule). Not verifiable by me.
4. Bun 1.3.14 `TextEncoder().encode("k".repeat(65537)+"é")` returns 65538 bytes ending in 0 (Buffer.from is right): toolchain bug, worked around in the four named-key tests.

## Changes (all edits are test/law readers, fixtures, Rust mount paths, a few real code defects; no manifest, no `package.json`)

- **Fixture placement (root cause of 7 law suites):** 113 fixtures under `…/🧫️fixtures` outside `🧪️tests` moved to `D/🧪️tests/🧫️fixtures/…` (value 48, replication 24, schema/pack/ui/actor/process/job/tool-run/math/async/io/…); 243 literal references in 158 files re-pointed (`tst3c-tools/fixmove.py`, plan `tst3c-tools/fixmove-plan.json`), 51 module specifiers re-prefixed with `./`, 14 `join(base|owner|root,…)` readers fixed. Also: dsl `🌿️variant-field` fixture out of the `🧬️schema` source dir, proctor `🏗️bootstrap` fixture (+ the one import in course `🧪️tests/🏗️proctor-build`), 4 process package fixture entries turned into `export { default } from … with { type: "json" }` facades. Result: fixture-placement green for value, actor, pack, ui, schema; process and products stay red only because of ✏️s FEM and 💻️os fixtures (other fleets).
- **Rust mount/path defects (cargo-unverified, all static):** `🧵️job/✍️payload-writer-and-grant` `#[path]` to the staged writer (`../📬️outcome/…`); `📡️replication/🎮️mutation/🚦️outcome-and-message` `inverse_admission_tests` path; new mount `native_diff_contract_tests` in `🎮️mutation/🦀️.rs`; `🧊️3d/🥽️mesh/📄️document` journal/structure paths; `🧵️job/👷️worker` staged-projection path; `🧵️job/📬️outcome/📮️return/🧪️tests` payload path; value sealed `🚚️frontier` dir moved to `🔗️arc-custody/🔒️sealed` (+ value package script path); `🌱️value/📋️list/🔺️edit` Cargo.toml persistent path dep (`../../../🔗️persistent`).
- **Real code defects:** tool-run `🧾️json` called `validateTickPayload` without import and `🔤️codec` lacked the `validatePayloadUpdate` import (+ export); actor `📦️packages/🦀️rust/📜️script.ts` imported the repo product (`🛍️products/🦑️repo`) -> process package; `🐧️directory` native storage imported `bun:ffi` statically, making the General preview unloadable under Node -> lazy `createRequire`; value sealed `alias_count` read `Arc::strong_count` outside the one allowed observer -> removed, projection test uses `original_aliases_are_returned`; `SEMIO_TEST_ARTIFACTS_DIR` vs `SEMIO_TEST_ARTIFACT_DIR` (raster svg-export + assets script) unified; 4 `📋️project.json` Nx targets (replication x2, value x2) gain the explicit `cwd`; 36 public SDK facades with `export *` made explicit (names/types from the TypeScript checker, `tst3c-tools/explicit-facades.ts`) and 12 json facades re-exported.
- **Law/data re-points (owner moved or renamed the text):** value controlled/queue/bulk-vec/custody/close-demand readers; replication io ownership (`semanticOwner` -> `🔖️sync-protocol`, new `constantsOwner`, fixture + schema + test), ingress closure re-blessed (`tst3c-tools/rebless-closure.ts`), `📦️crate-boundaries` dependency row, pack-json ownership dependency, mesh packaging schema (8 -> 11 packages), pixels descriptor inputs, dashboard `--dry-run` in four law copies, owned-execution neutral allowlist, async API test-owner roster, script-boundary command roster, 2d-compute lru-cache 11 wrapper, transient lru-cache 11 import, hub puzzle `--features` law, hub oracles double `🧪️tests/🧪️tests` paths, dsl absence roots, esbuild `external: ["bun:*"]` in script-boundary and graph bundles, build/typescript refusal wording, 4 named-key tests (TextEncoder).

## Per-suite table (changed or still red; before = plain, plain = final plain run, final = with emulation)

| suite | before p/f | plain p/f | final p/f | final | class | cause / fix |
|---|---|---|---|---|---|---|
| `♻️mit-bestand/🧺️demonstrator/M/🧩️runtime/🧪️tests/🟦️.ts` | 0/1 | 1/0 | 1/0 | green | FIXED |  |
| `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🏷️type/🟦️.ts` | 2/1 | 3/0 | 3/0 | green | FIXED | double 🧪️tests/🧪️tests path in the law |
| `🌎️hub/🧩️compositions/🗄️stdio/🔮️oracles/🧪️tests/🖊️drawing-reader/🟦️.ts` | 6/1 | 7/0 | 7/0 | green | FIXED | double 🧪️tests/🧪️tests path in the law |
| `🌎️hub/🧩️compositions/🗄️stdio/🧩️component/🏗️compiler/♻️capture/🧪️tests/🟦️.ts` | 1/2 | 1/2 | 1/2 | red | BLOCKED-USER-DECISION | reads .vscode/launch.json (deleted in bd9e4e02bd0, user decision pending); router bundle no longer reaches native-build/lease |
| `🌎️hub/🧩️compositions/🧩️puzzle/🧵️retained/🧪️tests/🔮️ownership/🟦️.ts` | 7/2 | 9/0 | 9/0 | green | FIXED | law forbids --features only on the package route, not on the presence/host-admission commands |
| `🌎️hub/🧪️tests/🌳️unit-census/🟦️.ts` | 4/1 | 4/1 | 4/1 | red | HARNESS-LAUNCHER | cargo metadata refused: Original compiler ingress requires the repository launcher |
| `🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts` | 15/1 | 15/1 | 15/1 | red | HARNESS-LAUNCHER | Original native caller capabilities required |
| `FW/📏️script-boundary/🧪️tests/🟦️.ts` | 1/2 | 3/0 | 3/0 | green | FIXED | command roster gains window-utility-check; esbuild bundle externalises bun:* |
| `FW/📦️packages/🟦️typescript/🧪️tests/🔌️sdk-surface/🟦️.ts` | 2/4 | 3/3 | 3/3 | red | MISSING-DEP | facade-explicitness test now green (36 facades explicit); 2 load tests need @dnd-kit/core |
| `FW/M/⏯️tool-run/🎞️tick/🛬️receive/🧪️tests/🟦️.ts` | (new) 0/1 | 0/1 | 1/0 | green | FIXED | imports re-pointed (codec/json) and shared 🎞️ticks.json path |
| `FW/M/⏯️tool-run/🧪️tests/🧪️conformance/🟦️.ts` | 23/5 | 28/0 | 28/0 | green | FIXED | validateTickPayload/validatePayloadUpdate were used but not imported (json wire) -> imported/exported from codec; real bug |
| `FW/M/⏳️async/📏️api/🧪️tests/🟦️.ts` | 5/1 | 6/0 | 6/0 | green | FIXED | testOwners roster gains 🔁️jittered-backoff |
| `FW/M/◻️2d/🧮️compute/🧪️tests/📍️ownership/🧭️direction/🟦️.ts` | 2/1 | 2/1 | 3/0 | green | FIXED |  |
| `FW/M/◻️2d/🧮️compute/🧪️tests/🟦️.ts` | 34/11 | 45/0 | 45/0 | green | FIXED | lru-cache 11 declared but test used the v5 API -> v11 LRUCache wrapper |
| `FW/M/🌱️value/♻️retirement/🎮️controlled/🧪️tests/🟦️.ts` | 15/1 | 16/0 | 16/0 | green | FIXED | shared cursor slice start moved to the impl that now owns step() |
| `FW/M/🌱️value/♻️retirement/🔗️arc-custody/🔐️unique/🧪️tests/🟦️.ts` | 7/2 | 9/0 | 9/0 | green | FIXED | same sealed fix |
| `FW/M/🌱️value/♻️retirement/🔗️arc-custody/🔒️sealed/🧪️tests/🟦️.ts` | 3/1 | 4/0 | 4/0 | green | FIXED | cfg(test) alias_count (strong_count outside the observer) removed; projection test uses original_aliases_are_returned |
| `FW/M/🌱️value/🏷️type/🧪️tests/🚮️absence/🟦️.ts` | 0/1 | 0/1 | 1/0 | green | FIXED | dsl neutralRoots gain filesystem + compiler syntax (process closure grew) |
| `FW/M/🌱️value/📋️list/🔺️edit/🔑️named/📇️index/🧪️tests/🟦️.ts` | 3/1 | 4/0 | 4/0 | green | FIXED | Bun 1.3.14 TextEncoder drops one byte of Latin-1 strings >= 65536 chars -> Buffer-based encoder in the long-prefix tests |
| `FW/M/🌱️value/📋️list/🔺️edit/🔑️named/🧪️tests/🟦️.ts` | 1/1 | 2/0 | 2/0 | green | FIXED | Bun 1.3.14 TextEncoder drops one byte of Latin-1 strings >= 65536 chars -> Buffer-based encoder in the long-prefix tests |
| `FW/M/🌱️value/🗂️ordered/♻️retirement/🧪️tests/🟦️.ts` | 7/1 | 8/0 | 8/0 | green | FIXED | step_original now quotes one unified next_demand |
| `FW/M/🌱️value/🧪️tests/🧩️neutral-owner/🟦️.ts` | 16/1 | 16/1 | 16/1 | red | HARNESS-LAUNCHER | cargo compiler ingress refuses without the repository launcher |
| `FW/M/🌱️value/🧪️tests/🧭️fixture-placement/🟦️.ts` | 1/1 | 2/0 | 2/0 | green | FIXED | 113 fixtures moved under 🧪️tests/🧫️fixtures |
| `FW/M/🌱️value/🧬️retained-clone/♻️close/🎟️demand/🧪️tests/🟦️.ts` | 2/1 | 3/0 | 3/0 | green | FIXED |  |
| `FW/M/🌱️value/🧬️retained-clone/🔗️source/🧪️tests/🔐️custody/🟦️.ts` | 2/1 | 3/0 | 3/0 | green | FIXED | fixture caller path moved to ➕️insert-cursor |
| `FW/M/🌱️value/🧬️retained-clone/🚚️bulk-vec/🧪️tests/🟦️.ts` | 3/3 | 6/0 | 6/0 | green | FIXED | contract read as source family; exact-cargo policy name |
| `FW/M/🎒️pack/🔤️json/🧪️tests/🚮️absence/🟦️.ts` | 0/1 | 0/1 | 0/1 | red | MISSING-DEP | copied workspace needs node_modules/bun-types at the root |
| `FW/M/🎒️pack/🧪️tests/🧭️fixture-placement/🟦️.ts` | 0/1 | 1/0 | 1/0 | green | FIXED | 113 fixtures moved under 🧪️tests/🧫️fixtures |
| `FW/M/🎠️kernel/🏛️ownership/📥️command-ingress/🧪️tests/🟦️.ts` | 1/1 | 1/1 | 2/0 | green | FIXED |  |
| `FW/M/🎭️actor/🎟️retained-turn/🎟️context/🧪️tests/🏛️boundary/🟦️.ts` | 0/1 | 0/1 | 2/0 | green | FIXED | script imported repo product -> imports process package runRepositoryCommand |
| `FW/M/🎭️actor/🖼️wire-turn/⚠️diagnostic/🧪️tests/🟦️.ts` | 13/0 | 0/1 | 0/1 | red | IN-FLIGHT-PEER | wire-turn root 🟦️.ts is being split by another fleet right now (modified 15:03); tests import names no longer re-exported |
| `FW/M/🎭️actor/🖼️wire-turn/📬️typed-operation/📥️receiving/🧪️tests/🟦️.ts` | 1/0 | 0/1 | 0/1 | red | IN-FLIGHT-PEER | wire-turn root 🟦️.ts is being split by another fleet right now (modified 15:03); tests import names no longer re-exported |
| `FW/M/🎭️actor/🧪️tests/🏛️owner-boundary/🟦️.ts` | 0/1 | 1/0 | 1/0 | green | FIXED | same |
| `FW/M/🎭️actor/🧪️tests/🧭️fixture-placement/🟦️.ts` | 1/1 | 2/0 | 2/0 | green | FIXED | 113 fixtures moved under 🧪️tests/🧫️fixtures |
| `FW/M/🏃️process/🎛️owned-execution/📣️progress/🛑️interruption/🧪️tests/🟦️.ts` | 1/1 | 2/0 | 2/0 | green | FIXED |  |
| `FW/M/🏃️process/🎛️owned-execution/📥️capture/📡️channel/🧪️tests/🟦️.ts` | (new) 0/1 | 0/1 | 0/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/📥️input/🟦️.ts` | (new) 0/1 | 0/1 | 3/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🎛️owned-execution/📥️capture/🧪️tests/🟦️.ts` | (new) 0/1 | 0/1 | 6/0 | green | FIXED |  |
| `FW/M/🏃️process/🎛️owned-execution/🧪️tests/🟦️.ts` | 4/1 | 5/0 | 5/0 | green | FIXED | neutral owners allowlist gains filesystem observation descriptor |
| `FW/M/🏃️process/🏗️build/🎟️capacity/🛟️custodian/🧪️tests/🟦️.ts` | 0/1 | 0/1 | 0/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🏗️build/🎟️capacity/🧪️tests/🟦️.ts` | 0/5 | 0/5 | 0/5 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/🟦️.ts` | (new) 4/1 | 4/1 | 4/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🏗️build/🗃️state/🧪️tests/🟦️.ts` | 1/1 | 1/1 | 1/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🏗️build/🟦️typescript/🧪️tests/🟦️.ts` | 6/1 | 7/0 | 7/0 | green | FIXED | oversized root refusal message is the schema 256-char refusal |
| `FW/M/🏃️process/🏗️build/🧪️tests/🏗️native/🟦️.ts` | 5/4 | 5/4 | 5/4 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🧪️tests/🟦️.ts` | 6/2 | 6/2 | 6/2 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/🎯️recipe/🧪️tests/🟦️.ts` | (new) 2/3 | 2/3 | 3/2 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/🎰️jobserver/🧪️tests/🟦️.ts` | 4/2 | 4/2 | 4/2 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🧪️tests/🟦️.ts` | 11/1 | 9/3 | 9/3 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🧪️tests/🟦️.ts` | (new) 11/35 | 38/8 | 38/8 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/📚️witness/🪟️windows/🧪️tests/🟦️.ts` | (new) 10/17 | 26/0 | 26/0 | green | FIXED |  |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧪️tests/📉️rebudget/🟦️.ts` | (new) 3/1 | 3/1 | 3/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧪️tests/🟦️.ts` | 10/0 | 9/1 | 9/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧪️tests/📥️paths/🟦️.ts` | 1/0 | 0/1 | 0/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧪️tests/📥️receipt/🟦️.ts` | 2/0 | 1/1 | 1/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/🎛️command/🧪️tests/🟦️.ts` | (new) 3/1 | 3/1 | 3/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧹️cleanup/📥️operation/🧪️tests/🟦️.ts` | (new) 2/8 | 2/8 | 2/8 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/📈️telemetry/🧪️tests/🟦️.ts` | 5/1 | 3/4 | 3/4 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🎯️target/🧪️tests/📋️callers/🟦️.ts` | 0/1 | 0/1 | 0/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧪️tests/🏎️overlap/🟦️.ts` | 1/1 | 1/1 | 1/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/📥️inputs/🟦️.ts` | (new) 0/3 | 0/3 | 0/3 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧪️tests/🟦️.ts` | 9/20 | 9/20 | 9/20 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🏗️native-build/🧪️tests/🟦️.ts` | 18/9 | 18/9 | 18/9 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/🐹️go-build/🧪️tests/🟦️.ts` | 1/1 | 1/1 | 1/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/💾️cache/🧪️tests/🟦️.ts` | (new) 27/16 | 27/16 | 27/16 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/💾️cache/🧹️retirement/🧪️tests/🟦️.ts` | (new) 55/2 | 55/2 | 55/2 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/📦️artifacts/📤️publication/📥️sources/🧪️tests/🟦️.ts` | (new) 16/5 | 16/5 | 16/5 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🔒️leases/♻️retirement/🧪️tests/📋️inventory/🟦️.ts` | (new) 3/4 | 3/4 | 3/4 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🔒️leases/♻️retirement/🧭️continuation/🧪️tests/📦️child/📜️script.ts` | (new) 0/1 | 0/1 | 0/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🔒️leases/🧪️tests/🎟️writer/🟦️.ts` | (new) 0/5 | 0/5 | 0/5 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🧪️testing/📥️invocation/🧪️tests/🟦️.ts` | (new) 1/14 | 1/14 | 1/14 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🧪️testing/📥️invocation/🧪️tests/🧫️.ts` | (new) 0/1 | 0/1 | 0/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🧪️testing/🟨️bun/📁️files/🧪️tests/🟦️.ts` | (new) 1/1 | 1/1 | 1/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🍎️image.ts` | 0/1 | 0/1 | 0/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🧪️tests/🟦️.ts` | (new) 23/5 | 23/5 | 23/5 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🧪️testing/🦀️cargo/🔒️lease/🧪️tests/🟦️.ts` | 1/2 | 1/2 | 1/2 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🧪️testing/🦀️cargo/🧪️tests/🟦️.ts` | 3/5 | 3/5 | 3/5 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🧪️tests/🧭️fixture-placement/🟦️.ts` | 2/1 | 2/1 | 2/1 | red | OTHER-FLEET | remaining offenders are 11 FEM fixtures under ✏️s (not in tst3c scope) |
| `FW/M/🏃️process/🧭️routing/🎛️command/🧪️tests/❄️cold/🟦️.ts` | 0/1 | 0/1 | 0/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🧭️routing/🎛️command/🧪️tests/🚧️boundary/🟦️.ts` | 0/3 | 1/2 | 1/2 | red | HARNESS-LAUNCHER | bundling/inventory part fixed (4 project.json cwd added); remaining part needs the launcher |
| `FW/M/🏃️process/🧭️routing/🎛️command/🧪️tests/🟦️.ts` | 2/3 | 2/3 | 2/3 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🧭️routing/🏠️workspace/🧪️tests/🟦️.ts` | 1/2 | 1/2 | 1/2 | red | PRODUCT-DRIFT | compiles repo 🧪️test/🏃️runner (unresolved import) and reads OS mcp workspace source |
| `FW/M/🏃️process/🧭️routing/📥️invocation/🏃️process/📤️child/🧪️tests/🟦️.ts` | (new) 0/1 | 0/1 | 0/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📊️profile/🟦️.ts` | (new) 0/1 | 0/1 | 0/1 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🧭️routing/📥️invocation/🏃️process/🧪️tests/📤️fractional/🟦️.ts` | 0/2 | 0/2 | 0/2 | red | HARNESS-LAUNCHER | needs live Original issuance / admission / nightly cargo from `bun repo run`; `bun repo run` stalled in the cache-pin queue (15 min alarm) and then ENFILE (fleet-wide) |
| `FW/M/🏃️process/🧭️routing/🧪️tests/🟦️.ts` | 55/1 | 55/1 | 55/1 | red | PRODUCT-DRIFT | 55/56 green; profile law reads repo library 🟨️.mjs that no longer names SEMIO_BUILD_PROFILE_INPUTS |
| `FW/M/📁️filesystem/🛡️durability/🧪️tests/🟦️.ts` | 0/1 | 0/1 | 0/1 | red | HARNESS-LAUNCHER | Live Original resource control issuance required |
| `FW/M/📁️filesystem/🧾️observation/🧪️tests/🎛️control/🟦️.ts` | (new) 1/1 | 1/1 | 1/1 | red | HARNESS-ENV | test re-spawns bun test of itself without the tsconfig override, so the nested run cannot resolve packages |
| `FW/M/📚️compiler/📖️syntax/🌳️module-tree/🧪️tests/🟦️.ts` | 7/3 | 7/3 | 10/0 | green | FIXED |  |
| `FW/M/📚️compiler/📖️syntax/🦀️rust/📁️paths/🧪️tests/🟦️.ts` | 2/1 | 2/1 | 3/0 | green | FIXED |  |
| `FW/M/📚️compiler/📖️syntax/🦀️rust/🧪️tests/🚮️absence/🟦️.ts` | 0/1 | 0/1 | 0/1 | red | MISSING-DEP | tree-sitter-wasms not resolvable from the copied workspace |
| `FW/M/📚️compiler/📖️syntax/🦀️rust/🧪️tests/🟦️.ts` | 6/1 | 6/1 | 7/0 | green | FIXED |  |
| `FW/M/📡️replication/⚠️error/🧪️tests/🟦️.ts` | (new) 3/1 | 4/0 | 4/0 | green | FIXED | same |
| `FW/M/📡️replication/🎮️mutation/🧳️retained/📏️body/🧪️tests/🟦️.ts` | 2/2 | 2/2 | 2/2 | red | SOURCE-LAW-DRIFT | 5 guard rows read ✏️s stdio/deflate 🧬️schema/🧳️retained/🦀️.rs files that no longer exist |
| `FW/M/📡️replication/📡️wire/🎮️command/📥️ingress/🏛️ownership/🧪️tests/🟦️.ts` | 2/2 | 1/3 | 4/0 | green | FIXED | closure re-blessed (value -> hash, 31 rows), cache expectation true |
| `FW/M/📡️replication/🔗️causal/🔀️transition/📝️drafts/🧪️tests/🟦️.ts` | 0/1 | 0/1 | 0/1 | red | PRODUCT-DRIFT | reads the OS store lifted 🏪️store/🦀️.rs which moved |
| `FW/M/📡️replication/🚪️io/🧪️tests/🏛️ownership/🟦️.ts` | 16/5 | 18/3 | 21/0 | green | FIXED | semanticOwner -> 🔖️sync-protocol, backbone constantsOwner |
| `FW/M/📡️replication/🧪️tests/🏛️crate-partition/🟦️.ts` | 19/3 | 20/2 | 20/2 | red | REAL-DEBT | history crate native-test source closure is 296,587 bytes over the 230,000 budget; one more orphan Rust test (🎚️record/🧪️tests/🦀️.rs) unmounted |
| `FW/M/🕸️graph/🧪️tests/🧩️suite/🟦️.ts` | 21/2 | 22/1 | 23/0 | green | FIXED | bun:* external in both bundles; native directory storage no longer imports bun:ffi statically (real node-loadability bug) |
| `FW/M/🕸️graph/🧪️tests/🪆️record-owner/🟦️.ts` | 2/1 | 2/1 | 3/0 | green | FIXED | see above |
| `FW/M/🖌️raster/🎥️video/🖋️svg-export/🧪️tests/🟦️.ts` | 2/1 | 2/1 | 2/1 | red | NEEDS-BROWSER | Playwright chromium-1248 not installed |
| `FW/M/🖱️ui/🎨️styling/🧪️tests/🧩️suite/🌗️contrast/🟦️.ts` | 7/1 | 7/1 | 7/1 | red | MISSING-DEP | package 'color' not installed |
| `FW/M/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🧪️tests/🟦️.ts` | (new) 9/5 | 9/5 | 14/0 | green | FIXED |  |
| `FW/M/🖱️ui/🧪️tests/♻️physical-job-close/🟦️.ts` | 4/5 | 4/5 | 8/1 | red | PRODUCT-DRIFT | reads OS renderer close_input_step which moved under 📺️renderer/🧑‍🎨engine |
| `FW/M/🖱️ui/🧪️tests/📁fixture-placement/🟦️.ts` | 3/2 | 5/0 | 5/0 | green | FIXED |  |
| `FW/M/🖱️ui/🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🟦️.ts` | 3/2 | 3/2 | 5/0 | green | FIXED |  |
| `FW/M/🖱️ui/🧬️contract/🧪️tests/🧱️owners/🟦️.ts` | (new) 6/1 | 6/1 | 7/0 | green | FIXED |  |
| `FW/M/🖱️ui/🪟️viewport/🧪️tests/🪆️binding/🟦️.ts` | 1/2 | 1/2 | 3/0 | green | FIXED |  |
| `FW/M/🗣️dsl/🧪️tests/🚮️absence/🟦️.ts` | 0/1 | 0/1 | 1/0 | green | FIXED | dsl neutralRoots gain filesystem + compiler syntax (process closure grew) |
| `FW/M/🚪️io/📝️text/🗿️artifact-reference/🪆️binding/🧪️tests/🏛️ownership/🟦️.ts` | 1/2 | 1/2 | 3/0 | green | FIXED |  |
| `FW/M/🚪️io/🧬️schema/🏛️ownership/🧪️tests/🟦️.ts` | 1/2 | 1/2 | 3/0 | green | FIXED |  |
| `FW/M/🚪️io/🪶️sqlite-snapshot/🧪️tests/🏛️ownership/🟦️.ts` | 4/1 | 4/1 | 5/0 | green | FIXED |  |
| `FW/M/🚪️io/🪶️sqlite-snapshot/🧪️tests/🤝️interoperability/🟦️.ts` | 0/3 | 0/3 | 0/3 | red | NEEDS-NATIVE | needs the cargo-built semio-io-sqlite-snapshot-oracle binary |
| `FW/M/🧊️3d/🥽️mesh/🛠️modeling/✂️clipping/🧪️tests/🟦️.ts` | (new) 6/9 | 7/8 | 15/0 | green | FIXED |  |
| `FW/M/🧊️3d/🥽️mesh/🧪️tests/📦️targets/🧪️packaging/🟦️.ts` | (new) 0/2 | 1/1 | 2/0 | green | FIXED | schema maxItems 8 -> 11 |
| `FW/M/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🚪️command/🧪️tests/🟦️.ts` | 1/2 | 1/2 | 0/1 | red | MISSING-DEP | execa resolves to an old copy lacking npmRunPathEnv |
| `FW/M/🧪️test/🎮️mutation/🏭️inventory/🏃️execution/🧪️tests/🟦️.ts` | 2/3 | 2/3 | 0/1 | red | MISSING-DEP | execa resolves to an old copy lacking npmRunPathEnv |
| `FW/M/🧪️test/🔌️adapter/🧪️tests/🟦️.ts` | 6/2 | 6/2 | 8/0 | green | FIXED |  |
| `FW/M/🧬️schema/✅️validator/🏗️compilation/🧪️tests/🟦️.ts` | (new) 7/1 | 8/0 | 8/0 | green | FIXED | dashboard run-script command gains --dry-run |
| `FW/M/🧬️schema/✅️validator/🧪️tests/🧩️neutral-owner/🟦️.ts` | 0/1 | 0/1 | 0/1 | red | HARNESS-LAUNCHER | cargo compiler ingress refuses without the repository launcher |
| `FW/M/🧬️schema/📄️source/🔗️closure/🧪️tests/🚮️absence/🟦️.ts` | 0/1 | 0/1 | 1/0 | green | FIXED | dsl neutralRoots gain filesystem + compiler syntax (process closure grew) |
| `FW/M/🧬️schema/🧪️tests/🧭️fixture-placement/🟦️.ts` | 1/1 | 2/0 | 2/0 | green | FIXED | 113 fixtures moved under 🧪️tests/🧫️fixtures |
| `FW/M/🧬️schema/🧪️tests/🧱️neutrality/🟦️.ts` | 13/1 | 13/1 | 14/0 | green | FIXED | needs ticket-path artifact dir (harness) |
| `FW/M/🧵️job/⏱️context/📦️owner/🧪️tests/🟦️.ts` | 4/2 | 5/1 | 5/1 | red | SOURCE-LAW-DRIFT | Err(failure)=>failure.primary.retained_progress() no longer exists after the domain-failure rework |
| `FW/M/🧵️job/♻️retirement/🎟️context/🧪️tests/🟦️.ts` | 11/3 | 12/2 | 12/2 | red | SOURCE-LAW-DRIFT | close_original_retained_step(...) and the generation/raster close text no longer exist |
| `FW/M/🧵️job/🏃️work/🎟️grant/🧪️tests/🟦️.ts` | 2/1 | 3/0 | 3/0 | green | FIXED | #[path] of ✍️payload-writer-and-grant staged writer was relative to the wrong directory (real compile bug) |
| `FW/M/🧵️job/📬️outcome/📮️return/🧪️tests/🟦️.ts` | 0/1 | 0/1 | 0/1 | red | PRODUCT-DRIFT | reads OS 🌍️world/📡️assets/🦀️.rs which no longer exists |
| `FW/M/🧵️job/📬️outcome/🤝️loan/🧪️tests/🟦️.ts` | 0/1 | 1/0 | 1/0 | green | FIXED | #[path] of ✍️payload-writer-and-grant staged writer was relative to the wrong directory (real compile bug) |
| `FW/M/🧵️job/🧪️tests/📏️close-demand/🟦️.ts` | 4/1 | 5/0 | 5/0 | green | FIXED | #[path] of ✍️payload-writer-and-grant staged writer was relative to the wrong directory (real compile bug) |
| `FW/M/🫧️transient/🧪️tests/🟦️.ts` | 0/1 | 0/1 | 6/0 | green | FIXED | lru-cache v11 LRUCache named export |
| `FW/🛍️products/📓️print/📦️artifacts/🎛️operation/🧪️tests/🟦️.ts` | (new) 47/1 | 47/1 | 47/1 | red | SOURCE-LAW-DRIFT | law expects runRepositoryCargoCommandV1 in the native emission router; the router now uses a different cargo helper |
| `FW/🛍️products/📓️print/📦️artifacts/📥️material/🧪️tests/🟦️.ts` | (new) 41/1 | 41/1 | 41/1 | red | SOURCE-LAW-DRIFT | law expects runRepositoryCargoCommandV1 in the native emission router; the router now uses a different cargo helper |
| `FW/🛍️products/📓️print/🧪️tests/🚪️typed-inference/🟦️.ts` | 4/3 | 4/3 | 7/0 | green | FIXED |  |
| `FW/🛍️products/📓️print/🧪️tests/🧬️native-chart-grammar/📥️emission/🧪️tests/🟦️.ts` | (new) 1/1 | 1/1 | 1/1 | red | SOURCE-LAW-DRIFT | law expects runRepositoryCargoCommandV1 in the native emission router; the router now uses a different cargo helper |
| `FW/🛍️products/🧪️tests/🧭️fixture-placement/🟦️.ts` | 1/2 | 2/1 | 2/1 | red | OTHER-FLEET | offenders are fixtures under 💻️os (tst3b) |

## Still red, by class (75 suites)

- HARNESS-LAUNCHER: 49
- MISSING-DEP: 6
- SOURCE-LAW-DRIFT: 6
- PRODUCT-DRIFT: 5
- IN-FLIGHT-PEER: 2
- OTHER-FLEET: 2
- BLOCKED-USER-DECISION: 1
- HARNESS-ENV: 1
- REAL-DEBT: 1
- NEEDS-BROWSER: 1
- NEEDS-NATIVE: 1

## Open items (owners)

- deps owner: `bun-types` (pack json absence copy), `tree-sitter-wasms`, `color`, `execa` (a version with `npmRunPathEnv`), `@dnd-kit/core`, `@iarna/toml`, Playwright chromium 1248; hoist/lock policy for suites that sit outside a package dir.
- coordinator: run the 49 HARNESS-LAUNCHER suites with `bun repo run <project>:test-…` once the cache pin and the file table are free; the Linux/native `bun:ffi` loader must be re-checked there.
- cargo owner: orphan Rust tests mounted now (native_diff_contract_tests) and the `#[path]` fixes are uncompiled; `semio-framework-replication-history` native test closure exceeds its byte budget (296,587 > 230,000) and `🎚️record/🧪️tests/🦀️.rs` is still unmounted; `semio-io-sqlite-snapshot-oracle` build; mesh Cargo.lock refresh.
- user decision: `.vscode` retirement (hub `♻️capture` law reads `.vscode/launch.json`).
- job owner: two laws quote retired close text (`⏱️context/📦️owner`, `♻️retirement/🎟️context`); plugin owner: five `📏️body` guard rows point at deleted ✏️s sources; OS owner (tst3b): `📝️drafts`, `📮️return`, `♻️physical-job-close`, products fixture-placement, wire-turn root split in flight; print owner: native emission router helper name.
- tst3a/b: the 11 ✏️s FEM fixtures and the 💻️os fixtures can be moved with `tst3c-tools/fixmove.py apply <list>` (the same mover).

## Verification

`python3 tst3c-tools/run.py <list> <out.jsonl> <tag> <budget>` (2 workers, `perl alarm 300` per file; `TST3C_HOIST=1` emulated hoist; `TST3C_ART=<ticket dir>` for ticket-path suites). Results kept: `tst3c-tools/baseline-results.jsonl` (before, 458 suites) and `tst3c-tools/final-results.json` (final, 649 suites). No `[DEBUG]` logs were added to the repo. Generated output (`🗑️generated/tst3c-out`, `🗑️generated/ta`) is deleted; tools stay in `tst3c-tools/`.
