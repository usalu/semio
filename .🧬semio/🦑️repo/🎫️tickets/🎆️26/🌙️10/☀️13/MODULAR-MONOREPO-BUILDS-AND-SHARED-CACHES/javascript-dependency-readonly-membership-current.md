# JavaScript Dependency Readonly Membership — Current Source Checkpoint

The Bun workspace owner now exports `withCurrentBunWorkspaceOwnershipV1(root, selection, originalControl, body)`. This performs actual controlled membership discovery with published-membership verification enabled, refuses stale Source, and retains the original Source text/topology observations through the callback. Its callgraph does not enter `publishCurrentOwnership` or write a manifest. The existing publisher remains a separate explicit owner. Root owns adoption by the dependency SH reader.

## Lifetime and failure contract

The receiver uses the incoming finite structural signal/progress contract; it creates no Original, control registrar, cache/store authority, or scanner surrogate. Frozen callback DATA contains selected manifests, copied package arrays, and `recheck`. Rechecks enroll synchronously before their first effect, with a private maximum of 32 pending operations. Closing the body closes admission, then joins accepted operations before releasing custody. The first accepted rejection remains recorded even when the caller handles the returned promise; a separate failure boolean preserves thrown `undefined`. Direct body failure remains primary. A successful body and accepted ledger are followed by final replay before release.

Each replay guards live custody and the actual signal before physical effects and after every awaited progress advance. Escaped or delayed rechecks cannot resume effects after release. Original Source observations are reused; unrelated document work retains its existing non-membership semantics.

## Physical bounds and descriptor protocol

Schema-owned limits are copied into private frozen primitives, preventing mutation of imported public JSON DATA from enlarging admission:

| Resource | Bound |
| --- | ---: |
| One manifest | 8,388,608 bytes |
| Unique manifest aggregate | 67,108,864 bytes |
| Unique text documents | 16,384 |
| Observations | 131,072 |
| One directory | 65,536 entries |
| Aggregate unique directory entries | 262,144 |
| Pending rechecks | 32 |
| Path | 256 UTF-16 units |
| Ownership policy | 65,536 bytes |
| Read chunk | 65,536 bytes |

Physical text reads admit lstat size and aggregate budget before allocation, open with `O_NOFOLLOW` where provided, compare pre-open path and descriptor identities, read a bounded exact extent, probe EOF, and compare descriptor/path identity again before accepting UTF-8 text. Identity includes device, inode, mode, link count, size, mtime, ctime and birthtime. Descriptors close in `finally`, preserving exact primary failure including `undefined`. Directory enumeration uses incremental `opendirSync` reads with buffer size one, checks limits before pushing an entry, closes its descriptor, and retains the previous filtered topology policy. Aggregate budgets survive replay and count unique paths rather than repeated reads.

## TDD and independently observed results

The initial export RED failed because the readonly API did not exist. Physical RED then demonstrated oversized manifests and actual during-read growth were accepted before the bounded implementation. An intermediate real module-initialization TDZ failure was retained and corrected by hoisting the identity function. A later genuine public-schema-mutation RED, handle 84947, produced 23 pass / 1 fail / 215 assertions; copying and freezing private numeric policy fixed that bypass.

Nine new readonly laws use actual temporary Source files: current/stale membership with unchanged bytes, topology/cancellation/body failure, detached accepted Error and undefined failures, delayed continuation drainage, oversized pre-parse refusal including hostile public policy mutation, actual descriptor-read growth and close, distinct aggregate bytes, the pending-32 admission bound, and cancellation across an awaited accepted continuation. Existing Ajv, fast-glob, SQLite and Node filesystem observations provide independent schema/membership/byte-count checks. Three existing refusal-diagnostic corpus cases additionally compare actual topology refusal facts with the independent JSON Patch library; the actual selected cohort contains 27 tests, not 28.

| Final gate | Handle | Result |
| --- | --- | --- |
| Source prototype, expanded refusal selector | 73762 | exit 0; 27 tests / 242 assertions; 981 ms |
| Source release, expanded refusal selector | 72828 | exit 0; 27 tests / 242 assertions; 928 ms |
| Owned strict prototype, same implementation epoch | 44526 | exit 0 |
| Owned strict release, same implementation epoch | 82119 | exit 0 |

The final selector change only adds the three pre-existing diagnostic rows to the previously qualified 24/216 Source cohort; no production implementation changed after the listed strict runs. Logs are in `🗑️generated/bun-readonly-refusal-final-{prototype,release}.log` and `bun-readonly-final-types-{prototype,release}.log`. The existing ticket `typescript-artifact/📜️script.ts` route owns the test child under its preauthored finite 120-second Original; no new executable or launch row was created. Own handles are terminal. Fixture cleanup removes only this module's own tracked temporary Source containers after accepted work settles.

## Source freeze

All five absolute Source paths are below 256 units (maximum 123). SHA-256 at handback:

| File below library `🗂️workspaces/🟦️bun` | Bytes | SHA-256 |
| --- | ---: | --- |
| `🟨️.cjs` | 29226 | `d52c7972c71e2e9a88819a2bd5187eca68ccf6ff6f1a3c48cbfe1a0d78737d17` |
| `🟦️.ts` | 6064 | `6ab0d3e6c3011a2cf2052c19dbc958bd3c1d6f61bf32c7085104c9f5a8fb06d5` |
| `🧬️schema/🔣️.json` | 7012 | `2923ab934dc3ad28bbfe6550335b6041468b1bb81044ee49338166babc5c291e` |
| `🧪️tests/🟦️.ts` | 47725 | `0d7394783421f31d04dbb07478935daff91d40f289a6087a7a18a459fbc84578` |
| `🧪️tests/🧫️fixtures/🔣️.json` | 19865 | `935dfe07b84d5fe2b7bc781835201e42aed83aad87fa2b7710fa8588fc2199a2` |

This qualifies Source and actual temporary-filesystem behavior on the current host. It does not qualify dependency installation, physical caches/images, Native/compiler work, Vite/SDK/product runtime, OS-descendant custody, or whole-repository membership affordability. Earlier engine/factory checkpoints remain separately frozen.
