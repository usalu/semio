# Wave 2 — Flatten Solver Report

## Landed
- 3d `⚙️engine/📐️geometry/🎛flatten/`: compose-parity absolute plane flatten + diagram centers; constants exact; seed_centers support
- 5d `⚙️engine/📐️flatten/`: maps parts/grips/fasteners → 3d graph, runs 3d flatten, writes origins/orientations + 2d x/y; grip angle → t
- 2d `⚙️engine/📐️layout` fastened layout mode: diagram-center rule for Derived nodes
- Tests: geometry::flatten (4) + 5d flatten (1) + fastened_tests (1) green
