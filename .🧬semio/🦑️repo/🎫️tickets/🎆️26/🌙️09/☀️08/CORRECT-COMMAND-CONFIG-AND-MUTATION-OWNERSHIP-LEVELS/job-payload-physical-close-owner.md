# Job Payload Physical Close Ownership

## Implemented Shared Boundary

The shared job owner retains each page in a fixed 16 KiB `Box<[MaybeUninit<u8>; JOB_PAYLOAD_PAGE_BYTES]>`. Its operation/process ledgers already counted physical page bytes, but committed and staged page close checked and reported initialized length. Rejected writer pages ignored the byte grant and reported zero bytes. All three branches now require one physical page grant before taking the exact backing and report 16 KiB after release. Committed logical payload length still decreases by initialized bytes.

Generic grant-forwarding methods preserve caller grants. The job budget fixture cleanup, plugin live-dispatch test cleanup and Generation3D retained-authority test cleanup now request the shared physical job-page constant. The recursive archive scheduler's production grant is owned by the recursive execution agent and remains subject to its separate native proof. No change is claimed for unrelated metadata allocations or page admission policy.

## Executed Evidence

| Run | Observed result |
| --- | --- |
| `job-physical-close-1.log` | Ajv/JSONPatch and strict TypeScript pass. Both new native laws reproduce physical release under insufficient grants. The full parallel crate also exposes pre-existing cross-test contention in its process-global pool/session-capacity tests, ending in fail-closed cleanup. |
| `job-physical-close-2.log` | With one test thread, 26/27 pass including both new laws. The remaining old callback quarantine assertion expected 30 initialized bytes; actual correct physical release is 32,768 bytes. |
| `job-physical-close-3.log` | **All 27 native job-crate tests pass**, zero filtered, normal 2 MiB stack, 0.29s runtime. Ajv, independent JSONPatch and strict TypeScript pass. Corrected callback assertion matches two physical pages and the exact pre-close ledger. |

The canonical root/ticket Nx route is `framework-job-physical-close`, launch order 311.210. It runs the whole job crate with `--test-threads=1` because several tests intentionally own the entire process-global session registry or pool while asserting exact capacity. Their explicit internal contention laws still run. This is not a claim that the uncoordinated parallel test harness passed.

The neutral fixture covers empty, one-byte, 20-byte and full 16 KiB pages. Native committed-page coverage executes all four cases across all five streams. A second law exercises staged plus rejected page backing. Insufficient grants preserve exact pointers, exact grants report one physical page, and local ledgers reach terminal zero. Each regression drains owned backing before failure assertions. Runtime debug output confirms both laws and real callback quarantine release.

FEM's numerical-child consumer still needs its selected native suite after these shared changes. Its pre-existing full-page test is not substituted for the shared short/empty-page proof.

## Changed Files

- [🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️retained-ownership/🦀️.rs)
- [📜️script.ts](/Users/ueli/Documents/semio/📜️script.ts)
- [📋️project.json](/Users/ueli/Documents/semio/📋️project.json)
- [launch.json](/Users/ueli/Documents/semio/.vscode/launch.json)
- [🧩️launch.seed.jsonc](/Users/ueli/Documents/semio/.vscode/🧩️launch.seed.jsonc)
- [📜️script.ts](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/📜️script.ts)
- [project.json](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/CORRECT-COMMAND-CONFIG-AND-MUTATION-OWNERSHIP-LEVELS/validation/project.json)
- [🟦️.ts](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/🧪️tests/📦️physical-close/🟦️.ts)
- [🔣️.json](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/🧪️tests/📦️physical-close/🧬️schema/🔣️.json)
- [🔣️.json](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/🧪️tests/📦️physical-close/🧫️fixtures/🔣️.json)

- [🧰️framework/🔨️modules/🧵️job/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/🦀️.rs)
- [🧰️framework/🔨️modules/🧵️job/⏱️budget/🧪️tests/⏱️budget/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧵️job/⏱️budget/🧪️tests/⏱️budget/🦀️.rs)
- [🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📡️live/📨️dispatch/🧪️tests/📨️dispatch/🦀️.rs)
- [✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️retained-authority-laws/🦀️.rs](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️retained-authority-laws/🦀️.rs)
