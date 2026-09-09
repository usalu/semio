# BMP Base64 Feature Scope

The combined integration build emitted an unused Base64 import in BMP after the earlier TIFF warning. Its two UI callers now reference the same first-party helper directly, and the unconditional root reexport is removed. Read-only review confirmed GIF, PNG and SVG already gate their root aliases on component-app-assembly. No algorithm or bytes changed; the existing Base64 oracle remains in the regression fleet.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs
