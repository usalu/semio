# Cooperative GLB JSON Reader Independent Audit

Actual TypeScript endpoint comparisons and read-only source review,2026-10-21. No new confirmed P1/P2 within reviewed scope. No native compilation/runtime claim.

## Immutable comparison

Ran glb-reader-baseline.ts --compare against immutable v2(33cases) and glb-reader-families-baseline.ts --compare against immutable v3(75cases). All acceptance decisions matched; all57 accepted v3 ordered snapshots matched word-for-word, including native binary64 words, bigint decimals, defaults, every declared family, integer-key order, dangerous names and duplicate replacement. All schema-role refusals matched. Exactly three refusal diagnostics changed: two initial BOMs, trailing NUL and trailing false token. Before: engine SyntaxError. After: IfcStepError code glb.json, respectively offset0 Invalid number and offset988 trailing tokens. All three remain refused. Generated comparison-v3 includes exact before/after error records. Raw byte malformed UTF8 behavior also matched baseline.

## Source review

IFC io/gltf-json-reader.ts borrows only already captured private bytes; captures original control; enforces raw chunk byte ceiling before decode, including stripped BOM. One fatal streaming TextDecoder preserves UTF8/BOM state across slices and flushes once. Decode, syntax and projection have original progress/cancel completion checkpoints. Syntax uses existing Replace decoder with explicit finite byte/node/depth bounds; numeric extent is yielded and checked before Number conversion. Capture clamps batch to4096, satisfying shared decoder chunk contract.

Shared glTF wire JSON module registers declaration-reader→syntax-generator metadata in a module WeakMap. The keys are schema-owned reader functions, not per-document values; no per-document global cache is introduced. Existing synchronous readers remain their schema authority. Array, tuple, map, object, tagged and nullable combinators share their exact member/default/role closures with cooperative projection. Closed-record unknown members refuse. Safe defineProperty constructs declared object members; arbitrary extras/maps retain member pairs, so __proto__ never becomes prototype mutation.

Progressive orderedSyntaxMembers classifies canonical array indices below2^32-1, heapsorts numeric keys and appends remaining strings in first-slot insertion order. Each classification/sift/swap/emission yields, avoiding synchronous whole-member sort. Replace syntax supplies last decoded value in first-slot position. Number(text) then existing finite/safe/integral reader preserves prior endpoint semantics rather than broadening indices to arbitrary exact integers. Baseline confirms exponent/decimal integer spellings, -0, rounding boundaries, underflow and overwritten overflow behavior.

## Explicit limitations

Shared syntax decoder still assembles individual strings with parts.join and numeric spelling with source.slice; segmented slices may join one token. Number conversion is synchronous but now has an explicit maxNumericCodeUnits admission bound. Vector/default constructors are bounded declaration constants; large arrays/member loops are progressive. JS allocation growth/Map operations and generator scheduling are reference control semantics, not original native five-axis custody proof.

The decoder's maximumNodes includes syntactically parsed overwritten values; maxDepth starts at root1 and counts scalar children. These are newly enforced finite policies versus prior unbounded JSON.parse. Existing capture maxValues bounds raw JSON bytes independently of BIN. Do not call those policies byte-for-byte backward admission equivalence outside admitted limits. Unknown chunk/framing policy remains the pre-existing reader's separate scope.

Parent's new18-case cancellation/limit suite is reported authored/tested evidence, not independently rerun here. Full registered/strict/official Khronos/Three results remain parent-owned and pending at this checkpoint. This audit establishes actual75-case endpoint parity modulo intentional syntax diagnostics, not universal glTF validity.
