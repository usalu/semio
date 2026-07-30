---
technology: block
emoji: 🧱
---

# Block2d - Block3d - Block5d

# NodeKind (2d) - ObjectKind (3d) - PartKind (5d)

# HandleKind (2d) - VortexKind (3d) - GripKind (5d)

# WireKind (2d) - CableKind (3d) - RopeKind (5d)

Each block document edits exactly ONE kind definition — never an assembly. `puzzle` edits assemblies
(nodes/objects/parts placed and linked); `block` edits the definitions those assemblies reference
(the rim of handle/vortex/grip templates a kind ships with, its representations/meshes, and which
other kinds it is compatible with). A block document's catalog fragment (`puzzleNd_catalog_fragment`
in each app's `engine` crate) is the seam puzzle imports through its `Kit×Type` media port — see
`s/plugin/puzzle/AGENTS.md` for the assembly-side vocabulary this mirrors.
