# TIFF Base64 Feature Scope

The native integration build emitted an unused root import in the TIFF artifact because its UI callers are feature-dependent. Call the existing first-party Base64 helper explicitly from all four baseline/document editor/viewer windows, then remove only the root helper reexport. The mutation codec macro remains exported. The algorithm and image bytes are unchanged; the existing contract Base64 oracle remains in the regression catalog.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs
