# 📓️ Coordination — Diff-Only Mutations

## Wave 1 (2026-10-08) — audits (Haiku), all complete
Reports: `🔍️audit-*.md` (framework-spine, framework-os, stdio-semio, stdio-pdf-gltf, stdio-mid, stdio-small, norm-a/b/c,
energy, architect, puzzle-trinity, block-wfc, fem-procedural-layout, seven-mid, small-plugins).
Repo-wide heuristic: 2,887 hand `MutationKind` impls in 96 artifacts; ~1,700 carry a V1–V3 code; 0 leaves had an
inverse-sum (L3) test; `ApplyCapability` absent; 57 whole-state `MutationDiff` impls; 91 hand `impl Mutation<`.

## Wave 2 — executors (Sonnet), launched 2026-10-08
| Label | Scope | Report |
|---|---|---|
| fw-spine | protocol trait, central `apply_diff`, store/plugin/tool folds, derive, law helper | `📓️exec-fw-spine.md` |
| fw-gate | `verify mutation-outcome-law` rules R8–R16 | `📓️exec-fw-gate.md` |
| fw-os-leaves | framework kinds + hand impls (config/print/run/window/presence) | `📓️exec-fw-os-leaves.md` |
| energy | 🔋️energy | `📓️exec-energy.md` |
| architect | 🏛️architect | `📓️exec-architect.md` |
| norm-a / norm-b / norm-c | 📕️norm split (+ `commit_value_tree_edit` owned by norm-a) | `📓️exec-norm-*.md` |
| stdio-semio / stdio-pdf / stdio-gltf / stdio-mid / stdio-small | 🗄️stdio split (`snapshot_patch_leaf!` owned by stdio-small) | `📓️exec-stdio-*.md` |
| puzzle-trinity | 🧩️puzzle 🔱️trinity | `📓️exec-puzzle-trinity.md` |
| block-wfc | 🧱️block 🀄️wfc | `📓️exec-block-wfc.md` |
| draw-cad-gis-raster | 🖍️draw (reference violation) 📐️cad 🌍️gis 🖨️raster | `📓️exec-draw-cad-gis-raster.md` |
| shooting-remodel-note | 🎥️shooting 📸️remodel 🗒️note | `📓️exec-shooting-remodel-note.md` |
| fem-procedural-layout | 🏗️fem 🌀️procedural (kinds) 📏️layout | `📓️exec-fem-procedural-layout.md` |
| small-plugins | 14 small plugins + procedural hand config | `📓️exec-small-plugins.md` |

Build gate: `🚦️gate.sh` (4 slots, < 10 rustc, `CARGO_BUILD_JOBS=3`).
