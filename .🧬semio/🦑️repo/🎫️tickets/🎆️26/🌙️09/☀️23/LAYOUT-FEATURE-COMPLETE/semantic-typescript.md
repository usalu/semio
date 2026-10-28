# TypeScript Semantic Mutation Evaluation

The canonical Layout mutation module now defines all 58 operations using owned Layout model types, Binary32/Binary64 words and bigint byte indices. The evaluator at `🧬️schema/🧬️mutations/⚙️evaluate/🟦️.ts` produces sparse events, diagnostics and stored-order semantic inverses. All publication and replay use the central Layout diff applier. Shared document and targeted graph admission belong to `🧬️schema/✅️validation/🟦️.ts`; native JSON conversion belongs to the named snapshot/mutation boundary modules, maintained by the root agent.

The evaluator covers selection transforms, page/spread membership, parent inheritance/overrides, UTF-8 story style ranges, text threading, links, layers, typography, flags, nested drawing text ownership and whole-document replacement. It reports errors/no-ops without publishing invalid state. It preserves exact floating words and uses the existing child drawing owner through an explicit caller-supplied context. The test context proves the seam and does not claim a full Drawing file decoder.

The story splice implementation is shared in `🧬️schema/✂️story-splice/🟦️.ts`; style ranges remain UTF-8 byte indexed, while the mutation's edit positions count Unicode scalar values. Corrected golden snapshots carry explicit valid spread rows and preserve independent spread order on global page reorder.

## Executed Evidence

`semantic-ts-check.mjs` compiles actual source with strict TypeScript and executes `🧬️mutations/🧪️tests/⚙️semantic/🟦️.ts`. Latest result: 372 source files, target diagnostics 0, full diagnostics 0; 96 portable semantic cases, 51 golden cases, all 58 operations, 40 IEEE word cases and two drawing cases pass. The test-owned oracle wraps fast-json-patch for independent publication projection. Input immutability, sparse application, central inverse, stored-order semantic inverse and diagnostic codes are checked. Canonical words include signed zero, extreme finite values and opaque equal/distinct NaN payloads; nonfinite geometry/style candidates are refused.

The shared fixture owners are `🧪️tests/🧫️fixtures/🧬️structure/🔣️.json`, `🕸️graph/🔣️.json`, `⚙️semantic/🔣️.json`, `🟰️words/🔣️.json` and the 51 golden mutation scenarios. Frame-selection tests now use the canonical parser and semantic evaluator instead of a separate numeric payload API.

No native execution is claimed by these results. The native counterparts and new native laws are source/grammar verified only; see `ieee-parity.md` and `semantic-graph-repair.md`. Public facade and repository verification route mounting are owned by the root agent.
