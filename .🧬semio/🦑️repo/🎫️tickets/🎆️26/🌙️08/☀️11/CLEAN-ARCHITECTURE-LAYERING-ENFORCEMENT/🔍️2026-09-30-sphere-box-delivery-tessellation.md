# Sphere Box Delivery Tessellation Audit

Read-only checkpoint, 2026-09-30. No tests executed, production sources edited, fixtures weakened, or budgets changed in this audit. Parent-provided runtime receipts were inspected together with current source and `git show HEAD` source.

## Observed Failure

Both `🗑️generated/canonical-preview-claims/exact-cargo-laws-AUayZH/00/law-16.stdout` and `🗑️generated/canonical-preview-determinism/exact-cargo-laws-SnjB8D/00/law-16.stdout` report exactly 398 triangles and 25 edge segments for sphere-box-fuse. Both report two round trips, one chunk, 14552 base64 pack bytes, complete phase, no diagnostics, one mesh, and solid role. Edit and view bounds agree: minimum approximately [-1.18176925, -1.18176925, -1.20000005], maximum [1.5, 1.5, 1.5]. The preserved fixture requires at least 428 triangles. Its edge minimum, bounds, roles, and selection assertions pass.

The count is already 398 in the decoded delivered mesh before preview payload publication. Preview publication, renderer labels, and fallback removal downstream of delivery therefore do not explain this loss.

The fine-fidelity sphere-box union law passes in the same receipts: tolerance 0.0025, 6006 triangles, 3074 vertices, 65 edge segments, closed mesh with zero boundary/nonmanifold/orientation defects, volume 9.692821424, independently checked Parry volume 9.692812920. This establishes valid fine geometry; it does not establish the preserved coarse count.

## Source Comparison

Current delivery source is `✏️s/🧑‍💻dev/🧩️composition-laws/🧪️tests/🧩️generation3d-example-geometry/🦀️.rs`; HEAD predecessor is `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️geometry/🦀️.rs`.

The fixture, default delivery tolerance 0.05, fine tolerance 0.0025, and coarse/fine LOD constants are unchanged. Both old and current delivery helpers evict every cached tolerance for the exact output handle and cancel their own tessellation jobs before starting delivery. Exact-or-finer cache selection explicitly chooses the highest eligible tolerance, independent of map iteration order.

Current `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs` and former global `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs` use `Brep::new()`, the same validation and `tessellate_job_sync(handle, tolerance)` entry point, the same retained-job stepping, mesh transfer conversion, envelope encoding, and chunking. No changed meshing flags or deflection were found.

The BREP engine changes add explicit retirement and ordered live-handle storage; numerical sphere, boolean, and tessellation entry points are unchanged. Tessellation edge-cache storage changes from HashMap to BTreeMap, but meshing retrieves edges through preserved encounter-order vectors, not map iteration. This storage change alone does not reorder tessellation. The angular constant remains 1.4. Adaptive refinement already evaluates UV edge midpoint chord error as well as vertex-normal angles; a missing midpoint check is not the cause.

New delivery transfers all actual evaluation handles into the host geometry port before releasing producer claims. The parent repeated the delivery with corrected handoff and still observed 398. `Body::compact` frees only unreachable slots and does not mutate or renumber surviving entities, coedge links, shell face vectors, or solid shell vectors. It therefore cannot directly change the kept fused solid's tessellation traversal. Boolean loop starts can depend on arena vertex indices during construction, so historical allocation conditioning remains unproven, but post-evaluation compaction is not a source-supported loop-reordering explanation.

## Historical Mesher Change After Fixture Measurement

Read-only history inspection establishes a substantive change before this extraction:

- The 428 floor was authored in commit `82c0bdf59a9` on 2026-09-24 at 12:11:40 +0200 (`git log -S '428'` for the fixture). The fixture has not changed since that commit.
- Commit `3eeee4f9119` on 2026-09-30 at 00:11:58 +0200 introduced `split_pole_branches` in the BREP tessellator.
- `git diff 82c0bdf59a9 HEAD` for tessellation contains exactly 36 added lines: `POLE_BRANCH_TOL = 1e-9`, calls after closing-duplicate removal for both outer and inner rings, and the new helper. The Boolean implementation and snapshot topology implementation have no changes over that same interval.
- The helper duplicates a pole's position into two UV ring vertices when the adjacent nonpole vertices differ in u by more than 1e-9. Their u values become the incoming and outgoing branches. Both carry the pole flag and are subsequently welded to one spatial vertex. This changes the constrained-triangulation and adaptive-refinement inputs even though final poles and bounds can agree.

This is a concrete fixture-provenance mismatch candidate, not yet a demonstrated explanation of the exact 30-triangle difference. The Session extraction's current-vs-HEAD tessellation changes do not alter these numerical inputs. A parent-owned diagnostic comparing the same admitted coarse job with the pre-September-30 pole-ring behavior would distinguish this committed change from the extraction without accepting a lower floor by assumption.

The parent's `🗑️generated/canonical-preview-topology.log` inspection at this checkpoint stopped before native execution because a concurrently edited generation3d-app-laws source SHA differed from the ownership fixture. It supplied no topology or independent-volume receipt yet; the parent was notified so that the correct current source fingerprint could be refreshed after that owner settles.

## Job Authority Concern Resolved

`SessionState.jobs` is local to an authority. `Session.port()` constructs a new SessionState with its own Mutex/TessellationJobRegistry while sharing kernel, mesh cache, claims, and family retirement. Consequently `retain_tessellation_jobs(live)` intentionally acts on that authority's jobs; producer release with [] does not cancel separately admitted port jobs. Session clones/captures share their exact authority. The parent also reports an existing native law confirming that cancellation of one authority preserves another. This is not an identified defect.

## Current Limit

The runtime failure against the preserved floor is confirmed, but its numerical cause has not been established by this read-only checkpoint. Neither a changed tolerance/flag nor preview publication explains it. A committed pole-ring correction occurred after the floor was authored and before this extraction; it is the strongest concrete numerical-change lead. No green claim is made for delivery or the full 355-law contribution. Preserve the original fixture and assertion while obtaining coarse topology, independent volume, and the controlled pole-ring comparison.
