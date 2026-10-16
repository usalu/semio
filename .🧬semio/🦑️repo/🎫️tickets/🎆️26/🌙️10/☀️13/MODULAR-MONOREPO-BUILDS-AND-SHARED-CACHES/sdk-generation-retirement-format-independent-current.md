# SDK Generation Retirement Format Independent Audit

Read-only Source plus Root-authorized bounded actual metadata census, 2026-10-15. No image opening, writer/delete, full-tree hashes, publication or tests. Current activation was protected and only observed.

## Actual Format Counts

Inspected exactly 24 regular 64-hex generation directories under `.cache/b/t/m/nx`, below the 256-directory ceiling. Read 6673739 bytes total, below 32 MiB, using at most 1 MiB per regular single-link metadata file with pre/post identity/size/timestamp checks. Inputs were package/lock, explicitly closed patch paths, authorizer manifests, optional SDK material and tooling marker. No dependency tree was read or hashed.

| Material format | Count | Noncurrent |
|---|---:|---:|
| shared-pristine | 3 | 2 |
| immutable-binary-objects | 8 | 8 |
| absent material file | 13 | 13 |

All 24 bounded input collections independently reconstruct their directory's nx-tooling-install-v3 identity for the host darwin/arm64 environment. All 24 tooling markers have version 1 and matching directory identity. This is input/marker evidence only, not full authorizer/binary/tree authentication or retirement approval.

Current generation: `4ee0fbdbe890ca6bea226c0ed38fd047f149393079894d357741cb2d30206b21`. Other shared-pristine generations: `ab981a02c2015e5d694b043691194e6322e2a850c171a415ce3b335bb86d07c4`, `adf0d10512fe706db798b12155c75585f40a792c054bb942a782c64fedf5734a`.

Typed immutable-binary-object generations: `55e49e835fe3d4a5c1723bf41c0894f29ec91804eb01ed9614ce0c95e3700acb`, `5b6cb1c1dc56e6031fb27a93c852928550160ccbdc35b7b36fb842f791012e3f`, `60f95aa15d05e3957d142b61b7d3a663484c180683142df4820ca2ec71de1c2a`, `654369b122c5ba40da16445f28cea1fe4515ec7bd61dea16ae161896062b295b`, `886b6b4483b171e22751076d3d5d459e0dbcd2a870cdada6beec3d838639f3d6`, `ba4fb523a6972bfd8eb055db99bf893f1bd404e1264756105b174f25eb366951`, `db49db21d1298457ae5b0278faf7968b87a5e84eb21eeafca6d79686e24e8f77`, `fda70a84aa2dc2d072029dd36d272be3c6060cc18f7ae9f59fdf5acc0a66b3ff`.

Material-absent generation prefixes (unambiguous among the 24): `0c3a89a0`, `1065ee1f`, `233dd781`, `29951bc6`, `3497672a`, `37243d80`, `58899fe5`, `66033330`, `6abcae71`, `823d0c86`, `b11ab199`, `f9a4dab3`, `fc753266`.

## Concrete Retirement Coverage Gap

`tools/🧹️retention/📥️physical/🟦️.ts:42` authenticates generation inputs and marker, applies ordinary/quiescent eligibility, then unconditionally parses `.semio-sdk-material.json` as derivative material inside `try/catch{return}`. `derivative/📥️reader/🟦️.ts:14` only accepts the shared-pristine material schema and its reader requires a base source index. Consequently the eight explicitly typed copy-material generations and thirteen absent-material generations cannot reach the retirement inspect callback, even under actual image EX and even if every prior ownership/witness check succeeds. The exclusion is a demonstrated necessary blocker for 21 current noncurrent directories; it does not prove these directories pass all other guards or explain every byte of the capacity deficit.

The new reader is correct for shared derivatives but is not an authored ownership verifier for existing copied artifacts. Historical artifact cleanup needs an explicit authenticated copy-material retirement verifier under actual whole-image EX, current-link protection, exact original input/authorizer/binary receipts, complete physical namespace inspection and known shared-binary group checks. Material-absent artifacts require independently authored provenance/ownership evidence or retention; absence is never permission. Do not weaken unknown-hardlink checks, reinterpret copied artifacts as shared-pristine, or add a runtime legacy/migration API.

No actual retireability, reclaimable byte quantity, active-holder absence or successful recovery is claimed. Exact data was sent to Nx and Root for the implementation boundary.
