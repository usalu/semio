# Warm component build recovery

The warm Stdio and PDF component build read the shared BREP representation before a peer published additional exports. The newer consumers attempted to import those exports from the older rlib. Both final components remained unlinked. The full production pass held one Cargo invocation for both targets. Existing compile drift recovery only admitted a single component.

The live change permits the existing held-lane retry for one warm, unsplit Cargo pass with one or more selected components. A failed pass needs a checksum witness from a transitive dependency and must have produced no linked component. The replacement unit graph is observed on both sides of source admission, and every selected root must fit together in the retained disk budget. Cargo stays in the original lane. Existing current-file, cancellation and compiler refusal handling remain in force.

For a multi-component pass, the first link starts publication immediately and releases the accumulated provenance messages to normal prefetch. A pass that has produced a linked component follows the ordinary finalization path. This keeps already linked outputs and their provenance out of a retry.

The language-neutral two-component eligibility case initially failed against the existing code. The live failure-churn law passes 21 cases and 58 assertions against the independent SQLite and BLAKE3 oracles. The repository's isolated component driver law changes a dependency after the first compiler attempt. Both component roots share it. The test observes the replacement graph, checks both published outputs, requires the same target and build paths in the two attempts, and confirms that sealed artifacts use the replacement dependency. That law passed with the final streaming code. It runs in a small ticket scratch workspace with a stub compiler-artifact stream and the pinned compiler's version probe. The actual Stdio and PDF gate runs independently against the real components.

- [Source staging and digest review](📜️component-retry-stage.py)
- [Isolated law launcher](📜️component-retry-law.ts)
- [Actual source custody analysis](📋️browser-source-custody.md)

The five reviewed files are live and match their verified staging digests. The existing w8jCBpl process had already loaded the previous TypeScript driver before this write.
