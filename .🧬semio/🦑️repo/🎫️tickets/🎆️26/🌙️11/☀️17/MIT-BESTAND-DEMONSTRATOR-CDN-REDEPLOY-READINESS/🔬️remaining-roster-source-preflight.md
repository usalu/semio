# Remaining Roster Source Preflight

The historical scanner reports below treated measured limits as warnings. Through the 07:32 PST run, a zero-refusal finding covered closure readability only. For example, the 05:24 report's warnings included DIN building at 43 files and Puzzle5d at 4,009 lines. The 07:32 BREP native registry report measured 4,004 lines against a 4,000-line ceiling while its refusal list was empty.

Read-only graph-only snapshot using `pair-workspaces.json`, pinned nightly Cargo and rustc, wasm32-wasip2 release profile and the production source closure scanner. Fourteen of 28 retained workspaces generated locked offline unit graphs, including the hub workspace with its 48 selected package roots. Across those graphs 3,839 distinct workspace/target/cfg source closures were read; the scanner reported zero closure read failures. Near-boundary observations include BREP representation at 3,989 active lines and DSL record emission at 261,923 bytes, both under the current 4,000-line and 262,144-byte limits. Shared-worktree edits may change these figures after the snapshot.

The other 14 workspace plans refused `--locked` because their existing Cargo.lock would need updating; they are not covered by this scan. A separate attempt to place the lockfile in ticket-generated storage was rejected by the pinned Cargo as an unsupported option. No existing lockfile or configuration was changed. A grouped hub graph unifies features across its selected roots, so this is not a guarantee for every independent per-root configuration or future admission.

After the owner refreshed the 14 stale locks and generated the missing locked unit graphs, a combined rescan covered all 28 workspace graphs: 3,870 distinct source closures and zero closure read failures. This remains a live snapshot, and the grouped hub feature-unification qualification still applies.

On 2026-11-21 after the reported 3,871-closure scan's transient GIS `geojson` unresolved-module refusal, a targeted recheck selected `semio_s_artifact_gis_gisterrain_editor` from the retained WASI release graph and ran the same production source closure scanner with the pinned rustc. It checked one closure with zero warnings and zero refusals; `🎮️commands/🌍️geojson/🦀️.rs` existed at the snapshot. This is a live focused observation, not a new full-roster scan or provenance seal.

On November 21 at 04:30 PST the production closure scanner reread 3,871 distinct target/configuration closures across 28 retained roster graphs and the actual previous Procedural and Process release graphs. The scanner reported zero closure read failures. The earlier unresolved GeoJSON file and DSL model overflow were absent in this snapshot. The scan does not exercise Rust type checking or browser behavior.

The November 21 05:18 scan of all 28 retained roster graphs checked 3,870 distinct target/configuration closures. The sole refusal was the missing newly declared stdio catalog bundle. Running its manifest-owned composition prepare-catalog-definition command produced 28,039 bytes for 38 definitions and 35 receipts. Both real catalog tests passed (162 assertions), covering exact source ownership, entry order, bundle size and the consumer include.

The 05:24 rescan read all 3,870 closures from those same 28 graphs. Its warning rows contained the two overruns described above.

## Policy-Aware Rescan on November 21

The preflight now reads the repository's source ceiling and each crate's authored boundary, uses production's effective lower limits and checks files, bytes and active lines. Over-limit closures go in the refusal list and cause a nonzero exit. A successful closure reports a warning only when it nears an effective limit. The scan includes profiles and feature selections from the retained Cargo graphs and remains a read-only source snapshot.

The neutral regression originally reproduced a false green for file, byte and active-line breaches. It now passes six cases, including exact equality and tighter and looser crate boundaries. The production source admission tests passed all six directly executable cases. The additional actual-process test requires an issued Script invocation envelope. The direct Bun run lacked that envelope.

The first corrected run covered all 28 retained roster graphs plus the focused WASI BREP graph. It measured 3,871 distinct target/configuration closures and found one refusal.

| Crate | Target | Files | Bytes | Active lines |
| --- | --- | ---: | ---: | ---: |
| semio_s_artifact_puzzle_5d_puzzle5d | wasm32-wasip2 | 13/32 | 259,292/262,144 | 4,043/4,000 |

The Puzzle5d editor import declarations were grouped by their Rust module namespace. The focused follow-up passed Puzzle5d at 13 files, 256,282 bytes and 3,961 lines. The focused BREP native registry control passed at 3,970 lines after its owner fixed it. The previously flagged DIN building closure was below all current effective warning thresholds.

The full post-fix pass checked 3,871 closures across the same 29 graph inputs. It exited successfully with zero refusals and 97 warnings below their effective limits.
