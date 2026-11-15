# Leaf Changed Race Killing a Graph Plan

## Root cause
`persistInferenceBlockV1` (observation `🟨️.mjs`) publishes shared, content-addressed observation blocks while a project's inference header is written. It read the block (`readObservationBlockV1`, whose final `observationLeafFenceV1` recheck throws `BUILD_INFERENCE_LEAF_CHANGED` when a peer/farm worker/fleet take renames a newer copy over the leaf) and rechecked it (`present.recheck()`, and again after publishing) without any retry. `readInferenceRecordOrMissV1` retried three times for lookups only. One concurrent replacement of a shared block therefore failed the project and, via the memory governor, the whole 4774-project plan.

Blocks are content-addressed: every read inflates the bytes and refuses them unless `sha256(bytes) === name` and the rows validate. A replacement holding the same content is therefore valid; other content fails that read (damage path: unlink and republish).

## Lost message
`failureRefusalV1` logged `error.stack` verbatim; the stack of this error carried no message ("Error" only), and `semioProject` (set by `tagProjectFailureV1`, so the failure WAS tagged) and `code` were never printed.

## Fix
- `settleObservationLeafV1(attempt,{attempts=3})` (exported): re-runs a leaf read while it throws `BUILD_INFERENCE_LEAF_CHANGED`; after the last attempt the typed error propagates (tagged with the project by `tagProjectFailureV1`); every other error propagates at once.
- `persistInferenceBlockV1`: the presence read plus recheck (including the damaged-leaf unlink, now tolerant of ENOENT when a peer already replaced it) and the post-publication read plus recheck run inside `settleObservationLeafV1`.
- `failureDetailV1` (resident failure service): the logged detail now leads with `Name: message code=... project=...` whenever the stack lacks them.

## Files
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs`, `🟨️.d.mts`
- `.../👁️observation/🧪️tests/🧫️fixtures/🔣️.json` (export roster)
- `.../👁️observation/🧪️tests/🍂️leaf-race/🟦️.ts` (unit retry law; real shared block replaced once during publish succeeds; replaced persistently fails with the code)
- `.../🏡️resident/🏛️service/🧯️failure/🟨️.mjs`, `.../🏡️resident/🧪️tests/🧯️failure/🟦️.ts`

## Tests
leaf-race 6 pass; failure 9 pass; observation main suites see the task output below.
