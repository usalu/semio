# Aggregate Capture Difference

Compared retained preparation manifest bazel-aggregate-source-readiness-59284.json (efb397e6…) with actual aggregate execution manifest bazel-aggregate-execution-source-census-65174.json (943f1684…), independently verified byte-identical to the observed common census. Both contain440 paths; no added or removed paths. Exactly two changed rows explain all526 bytes:

- bootstrap/📦️dependencies/💾️storage/🟦️.ts:19168→19195 bytes (+27), SHA5817d119…68096→b994fb75…49b20.
- bootstrap/📦️dependencies/🟦️.ts:13450→13949 bytes (+499), SHA22f07a1f…81a73→3bb355a9…db6cd.

Exact full paths/hashes/byte counts and manifest SHA bindings are retained in generated/bazel-aggregate-capture-difference-low.json. This compares actual captured metadata, not substituted current disk files; no actor or mutation-time attribution is made. The actual execution captured its new identity and enforced its guards; this is not evidence that it executed stale authority. Separate tooling role currentness and SDK provenance claims retain their own scopes. No SDK/store/tree opened.

Both changed bootstrap dependency paths are absent from every reader115/runtime190/compiler70 manifest in the fresh phase-current receipt (375 per-role rows). They therefore lie outside that captured SDK role closure. This path intersection does not prove all SDK behavior or currentness beyond those declared captured inputs.
