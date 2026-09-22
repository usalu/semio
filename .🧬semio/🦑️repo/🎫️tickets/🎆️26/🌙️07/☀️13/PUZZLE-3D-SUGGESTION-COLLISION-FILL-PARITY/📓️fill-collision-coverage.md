# Fill Collision Coverage

## Scope

Ensure every new puzzle 3D fill placement is checked against every existing object and each earlier accepted placement, including final revalidation. Preserve cooperative progress and cancellation.

## Inspection

- Fill preparation silently omits existing objects whose collision mesh cannot be resolved or is not registered. Candidate acceptance does not verify that every existing object was indexed.
- The fill planner adds accepted placements to its spatial index before considering the next candidate and does not exclude the docking host.
- Final revalidation only checks the original scene bodies, omits unavailable meshes, and treats a missing candidate mesh as collision-free. It does not check earlier surviving provisional placements.
- The spatial query records truncation but the planner does not check that flag before accepting its results.

## Validation Plan

Add language-neutral scene cases and Rust regressions for docking hosts, unrelated blockers, earlier fill placements, unavailable meshes, and revalidation after mesh changes. Cross-check cube intersections with the existing parry3d development dependency. Run the artifact suite through Bun/Nx and record actual results here.

## Additional Runtime Path

The fill tool preparation replaces any missing real mesh with the generic box collision body. The renderer independently uploads actual GLB geometry, so a run can collision-test boxes while displaying larger real objects. The fix must reserve the box body for the actual built-in `box` mesh identity and refuse unresolved real geometry. The React world mounts mesh registrars independently of fill execution.

## Test Execution

Initial Bun/Nx runs waited on shared Cargo deliverable locks. Stopped only these task-owned test processes and relaunched the artifact test target with deliverables under this ticket’s generated folder, retaining the repository’s shared compiler intermediates.

The compiler sample showed all active test-build worker threads blocked in Cargo prebuild file locks for over nine minutes. Verification now uses a private APFS clone of the existing debug intermediates under `🗑️generated/build`, plus a private target directory; the shared cache and other developers’ processes remain untouched. Both temporary directories will be removed when validation finishes.

## Reproduced Failures

The focused regression run executed all three new tests and failed all three before the production change:

- An unregistered GLB identity incorrectly received an available box collision body.
- The unavailable-blocker scene accepted one object although the fixture requires zero. Existing host, unrelated-blocker, late-blocker, and earlier-placement cases passed, isolating the unchecked-geometry path.
- With candidate meshes enlarged after planning, final revalidation returned `[false, false]` while the independent parry3d oracle returned `[false, true]`.

## Implementation

- Fill preparation only uses the built-in box for the actual `box` mesh identity. An unavailable real mesh is left unavailable, causing an explicit placement refusal.
- Candidate construction verifies that every base and previously accepted object has a collision entry.
- A truncated spatial query cannot reach acceptance.
- Final revalidation refuses unavailable geometry and adds each survivor to the collision set before checking the next placement.
- Head preparation and non-intersecting pair checks consume cooperative fuel.

## Regression Coverage

The three new Rust tests consume 15 language-neutral fixture cases: three mesh-preparation cases, eight placement cases, and four final-revalidation cases. Accepted placements and final conflict vectors are checked against the existing `parry3d` development dependency. No runtime dependency was added.

The focused post-fix run passed all three tests. Test runtime diagnostics confirmed that an unavailable blocker accepts zero placements, coincident targets accept only one placement, and mesh growth produces `[false, true]` conflicts, matching the independent oracle.

The first broader run completed 95 tests: 89 passed, five lifecycle tests failed because their synthetic meshes had relied on the removed implicit fallback, and one wall-clock latency assertion exceeded its 2 ms ceiling at 2.478208 ms. The lifecycle harness now registers its synthetic cube geometry explicitly before starting a fill run. The collision fixture documents this test geometry. The timing threshold remains unchanged.

Validation uses the existing target, with private generated-directory Cargo paths:

```sh
bun nx run '@semio-tech/puzzle-3d-rs:test' -- --lib all_objects --no-fail-fast -- --nocapture
bun nx run '@semio-tech/puzzle-3d-rs:test' -- long --lib --no-fail-fast -E 'test(fill) | test(precompute::geometry)' -- --nocapture
```

Two serial-run attempts using `--test-threads=1` failed argument parsing without running tests. The installed nextest help confirms `--nocapture` after the second `--` runs tests serially. A test-harness module path compilation error was also corrected before this run.
