# Explicit Cargo Workspace Census

Read-only filesystem audit of explicit double-quoted members and each package's explicit workspace declaration. This does not infer Cargo automatic path-dependency membership or execute Cargo.

Correction: the initial audit reused a prior declaration for a member with no explicit package.workspace because a function-level var was not reset. Its one foreign-owner finding was invalid. This corrected census scopes the declaration per member. Direct source inspection by the execution owner caught the audit error before a membership change.

## Cargo.toml

Explicit members: 564. Explicit mismatches or missing manifests: 0. Members with no explicit package.workspace: 1.


- Implicit owner: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/🧪️tests/📥️reader/📦️packages/🦀️rust`

## 🧰️framework/Cargo.toml

Explicit members: 405. Explicit mismatches or missing manifests: 0. Members with no explicit package.workspace: 0.



