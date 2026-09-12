# Hydration Pack Progress And Physical Retirement Review

The shared extraction and five recursive tests prove functional root/member hydration and preservation behavior. They do not yet prove bounded Pack construction or physical retirement.

## Findings in the new common cursor

Source: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs`.

1. `Phase::ScanPack` computes remaining length, increments `pack_scanned`, and consumes fuel. It does not inspect or parse input bytes. `Phase::DecodePack` subsequently calls the whole `P::decode_pack` under one unit. Advancing a counter before a synchronous decode does not make the actual decode interruptible or bounded; the admission test must witness actual parser/typed-construction progress.
2. Pack cleanup truncates a `Vec<u8>` and reports the length decrease as released bytes. Truncating byte elements does not reduce backing capacity. A later empty-vector branch drops that entire backing and reports zero bytes. This repeats the physical-versus-logical ownership defect already repaired and independently tested in shared job payload close.
3. The member opener still retains unreachable pre-extraction typed-hydration phases and owners. The execution agent independently identified these for removal. The shared cursor must become the sole typed hydration implementation.

## Required correction

Use a genuine retained Pack parser and typed snapshot construction factory at the shared document boundary, reusing existing first-party Pack/schema facilities. Each admitted step must perform bounded actual work and expose meaningful cancellation. Preserve domain/app initialization ownership.

Retire actual backings. Either retain input as real pages with one-page retirement or require the complete observed contiguous allocation grant before releasing it. An insufficient grant must preserve exact pointer and capacity; successful release must report actual physical bytes and the correct item count. Counting length reductions cannot stand in for backing retirement.

The focused tests need independent neutral expectations for actual parser progress and physical retirement, including cancellation after partial typed construction. The proposed one-byte scan/cleanup test is not acceptance evidence for these claims until corrected. Suite52 remains green functional evidence; no bounded Pack-decode claim is made.

The Recursive Sol agent has received these findings and owns the correction plus dead-path removal. Referenced shared source remains untouched during the next root FEM validation interval.
