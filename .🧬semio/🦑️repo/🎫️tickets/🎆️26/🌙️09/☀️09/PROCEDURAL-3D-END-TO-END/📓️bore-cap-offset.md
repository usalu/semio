# Retained Offset Face with a circular opening

The original `brep.surf.offset` operator translates a line-bounded planar face and one complete circular opening along the oriented face normal. This covers the two picked caps of a box cut by a perpendicular through-cylinder. The native result retains the analytic circle, its source traversal direction, its complete parameter interval, and the projected planar pcurve. The outer ring and opening receive newly published member labels on the output face. The source face and its labels stay available.

The circular opening owner certifies the original closed coedge, its frame and orientation, then checks clearance against one outer segment per paid turn. Each topology insertion, closure link, inner-list reservation and attachment has its own paid turn. Until the final live commit, cancellation releases the inner-list capacity and retires its native records before unwinding the outer draft. Admission intentionally covers one complete circle that clears the directed outer halfspaces. Other opening families still require their own certification.

The lower solid transaction package also owns the existing single-conic draft cursor, keeping the native registry below its source gate while placing both related rollback owners beside the common topology-slot accounting.

## Verification

- The language-neutral surface fixture gives the stock, off-center bore, net area and signed placement of three cap offsets. The independent Three.js suite passed all 12 tests, including translated cap area and an uncovered bore center.
- The isolated Rust source harness passed all five tests. It compiled the actual polygon, conic and circular-opening owners plus the first-party planar pcurve constructor against an in-memory topology contract. It checked exact same-parameter projection and both normal signs, inward offset, one-item and zero-item turns, overlap refusal, and rollback after acquiring the exact inner-list allocation. It also exercised the relocated single-conic draft and its full retirement.
- The new linked Flow regression in `original-brep-faces` builds Box, Cylinder and Cut, reads the funded viewport face label, resolves that label through BRep Deconstruct, calls Offset Face, and checks its analytic opening, usable edge pick label, preview area and open center. It exercises active cancellation after a draft circle appears. This target has yet to run against the linked workspace.
- Rust AST checks and scoped `git diff --check` passed. The repository source closure counter reported 3,980 active lines for Engine, 3,771 for the native registry, 1,646 for solid transaction and 1,903 for the faces integration target. Their active byte counts were 236,002, 203,739, 83,813 and 226,140 respectively.
