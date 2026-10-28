# Cursor Activity Runtime Propagation

## Owned Source

Only 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/⚙️runtime/🐍️.py was edited for this subtask. Existing audience handling, narration alignment, native progress, candidate publication and authored-source guards were retained. No FFmpeg video/audio arguments were changed.

## Portable Activity Interface

- clip_duration delegates to the existing speech probe_duration interface.
- cursor_intervals reads clip.with_suffix('.cursor.json'); a missing sidecar means no activity.
- publish_cursor_intervals atomically writes version 1 and normalized start/end ranges.
- clip_cursor_intervals intersects ranges with a half-open excerpt and rebases to zero.
- propagate_cursor_intervals offsets ordered clip ranges by their measured durations.

Ranges must be finite, numeric and satisfy 0 <= start < end. Normalization sorts and merges overlaps and adjacency within 1e-9. When all inputs have no activity, propagation publishes an empty list without any duration probes. Otherwise each input duration is measured once, and activity must fit its source duration.

## Source, Excerpt And Chapter Lifecycle

Before a new Manim source render, the old raw cursor trace at source_trace_path(...).with_name('👆️.json') is removed. After successful rendering and narration/source validation, the helper's versioned intervals are published to the source cursor sidecar, or an explicit empty list is published when no raw interaction trace exists. Metadata is published before the successful source certificate.

Concatenation propagates metadata after its atomic movie replacement. Excerpts clip/rebase activity after replacement. The single-source chapter copy branch explicitly copies normalized source activity. Missing generated activity sidecars cause their owning source/excerpt/chapter/content stage to repair the metadata rather than silently reuse an incomplete cache entry.

## Metadata Identity And Guards

Excerpt identities include the source activity metadata hash and verify it again after selection. Logical chapter identities include their ordered raw-source activity hashes; legitimate metadata newly generated during source rendering is refreshed after the authored-input guard. Chapter copy/concat verifies the same activity inputs before certifying output.

Full-content identities include the actual chapter activity hashes and check them again before certification. Presentation publication includes the full-content cursor hash and verifies it before replacing the final candidate. This prevents a cached enclosing UI from retaining stale parent emphasis when activity metadata changes.

Raw source fingerprints now explicitly include the pure interaction core source closure, native scene helper and its actual metadata schema path: interaction/🎬️scene/🔣️.schema.json. Canonical cursor/icon assets and interaction contract/schema therefore invalidate affected raw render caches.

## Verification

The interval tests were written and observed failing before the new APIs existed. Three interval tests passed in 1.159 seconds after implementation, including actual cut/concat wiring through the abstract command interface, independent NumPy excerpt intersection/rebasing, sorted merges, invalid finite ranges, duration offsets and the empty-input/no-probe fast path.

Broader runtime and presentation cache/integration tests are coordinated by the testing agent after the final cache wiring.

## Asset Discovery Regression And Correction

Adding the interaction core's explicit canonical styling JSON exposed an existing overly broad asset-owner traversal: segment_state recursively walked the parent of every declared dependency, including the styling package's node_modules junctions. The broader test run stalled; faulthandler captured pathlib traversal inside segment_state before the correction.

Recursive authored-asset discovery now applies only to Python code owners. Explicit canonical JSON, SVG, font and schema files remain directly fingerprinted without traversing their package directories. A real Physical Fundamentals segment fingerprint completed with 32 source files in 0.456 seconds after the correction. No dependency contents or source ownership changed.

The parent helper's latest per-frame pointer hit testing and analytically trimmed visible-entry/exit intervals were re-read and confirmed; runtime propagation preserves these refined ranges exactly.

## Broader Verification Completed

All eight presentation contract tests passed in 3.020 seconds. Two native snapshot tests passed in 12.495 seconds. The broader runtime run passed 24 cases and skipped six intentional optional cases; its sole stale cached-chapter fixture was updated to the current audience media/speaker paths, activity hash list and explicit empty sidecars, then the formerly failing test passed in 0.474 seconds. No further production change was needed.

An additional behavioral asset-isolation regression passed in 0.337 seconds: changing an unrelated sibling SVG leaves segment_state unchanged, while changing an explicitly declared cursor SVG invalidates it. This independently verifies the asset-walk correction and canonical cursor cache dependency.

Runtime source is settled. The public activity APIs, source trace lifecycle, excerpt/concat/copy propagation and activity-aware cache/publication guards are ready for the parent's final UI renderer integration.
