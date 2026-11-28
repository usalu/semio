# H78: g1-m01 workspace stage "Bounded closure reference DATA required" (state 12:50, 11/27)

## Where it is thrown
`references(value)` in `…/🧾️closures/👷️closure-read-worker/🟨️.mjs` line 46: it counts every key and value of a parsed closure payload as one node and refuses beyond `Protocol.maximumNodes` (4194304, `🧬️schema/🔣️.json`) or `maximumDepth` (64). The parent (`📡️session/🟦️.ts`) only wraps it as "Original closure read child refused: Error: …", and the service reports it as an unexpected failure (refs 85357a2e 11:44, 2e4f643b 3x deterministic 12:08).

## Cause (partly proven)
- The node bound is a fixed number unrelated to `maximumExpandedBytes` (256 MiB): any payload over 4.19M nodes is refused, whatever its byte size. Payload node counts grow with the graph (inference publications / selection entries list one row per project and input).
- Measured at 12:25 on m01 (`scripts/H78/shape.mjs`, whole `.cache/b/g/m` store except pack/authority/rows files): largest payload 3,521,865 nodes (`inference/selections/<sel>/ce/e/*.json.br`) = 84 % of the bound; `emoji-project-json.publication.json` 3.12M; depth max 7. Nothing at rest exceeds the bound; the failing row is not in the store any more (retired) or is a kind I could not parse (pack, authority, `.rows.br`, which are not JSON). So: the bound is within 16 % of the largest stored payload and the growth over today's H9 reads (many new selections) is the consistent explanation; the exact row is UNPROVEN.
- Why no `[DATA] Fault detail`/row path: `closurePayloadRefusalContextV1` returns the bare error for kinds outside `PayloadRefusal.kind` (`publication`, `receipt`, `json`, `pack` are not in the enum), so the failure frame carries only name/message.
- Why only m01: it holds the largest store (generations 15.9 GB, 413k files); other workers read smaller graphs.

## Staged patch (NOT landed; graph/closure code is frozen in the seal window)
`T/🧰️scripts/H78/stage.py <outdir>` (applies to anchors in the live files, writes three patched copies; idempotent assertions on unique anchors):
1. worker line 46: `throw refuseCachingV1("invalid","closure.reference-data-bound",{nodes,depth,maximumNodes,maximumDepth})` instead of a bare Error.
2. `📖️texts/🔣️.json`: new key `closure.reference-data-bound` (en/de) naming nodes, depth and both bounds.
3. schema: `maximumNodes` 4194304 -> 16777216 (= `maximumExpandedBytes`/16, so bound follows the byte bound); `PayloadRefusal.kind` enum gains `publication`, `receipt`, `json`, `pack` so the failure carries the row path/hash (only applies if the enum text is present; check at land time).
Test: `T/🧰️scripts/H78/references-bound.test.mjs <outdir>`: 3.6M-node payload passes, > bound refused with the typed key, message <= 128 chars (ReadFailure limit), depth 70 refused, de text present. Run: green ("references-bound ok, bound 16777216").
Land after the seal lifts: run stage.py with the real paths copied back (`cp` the three outputs over the files, unlike scratch), then `repo typecheck` and the closures tests (`🧪️tests/🌊️authority-stream`, `📡️session`); the authority stream shares `maximumNodes` per record (its tests only assert tokens). The bounded-paging alternative (streaming reference scan like authority) is the long-term fix; the byte-derived bound is the minimal correct one.
Needs: coordinator restart of the workspace stage on m01 after landing (worker is captured per issued proof, new source digest).
