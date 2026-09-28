# Exact Ellipse Conversion

Circle and ellipse shape-to-path conversion previously emitted four cubic approximations using the usual 0.5522847498 coefficient. These introduce an outward contour error and make conversion alter exact geometry. Rust and TypeScript now emit four quarter-ellipse arc segments, retaining the four cardinal anchors, closure, identity, style and affine transform. Picking consumes the same indexed primitive geometry.

The language-neutral shape conversion fixture now specifies exact arcs. The TypeScript regression failed against the cubic implementation (run 14793), then passed after both implementations changed (run 35051): 108 tests, 18,031 assertions, plus 44 Ajv cases. Three.js EllipseCurve provides independent quarter-ellipse points; reconstructed arcs must agree within 1e-12 for the fixture radii.

## Native Validation

Run 31911 finished with 371 of 373 tests passing. It captured the new exact-arc fixture while still compiling the earlier cubic implementation, which accounts for its conversion failure. The second failure was the multi-selection movement fixture: its pointer-down at world (0,0) now hits the top-left resize handle. Moving an interior point by (30,20) is the appropriate movement test. It now starts at world (20,20), checks preserved scale, then verifies the same preview, commit, cancellation and undo/redo behavior. The precision tolerance retained from the prior repair is 1e-10; no movement implementation is changed for this fixture correction.

Fresh full native run 94642 remains live, log `🗑️generated/tests-native-exact-ellipse-current.txt`. The full suite still needs a green result before acceptance.

## Other Validation

Renderer run 81252 failed on two unsupported mutable-index accesses in the shared window test fixture. They now use NonEmptyVec.first_mut(). Run 99794 then failed on a shared projection method being added concurrently; current source includes that method, so scoped canvas validation is rerunning as 40003. No unverified renderer success is claimed.

Draw materialization was recovered from verified shared Cargo lock contention; see [build recovery](🔒️component-build-contention.md). Component run 67402 exposed a shared value-codec compilation error, corrected in source; the current component build is 58527. Native path painting, direct node gestures, other full editor acceptance workflows and browser verification remain required.
