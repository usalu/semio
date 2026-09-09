# Native Import Paths

The focused native build emitted two unnecessary AppActionRegistry qualifications and an unused JPEG Base64 reexport when component UI features are absent. Use the registry type already imported in scope and resolve the first-party Base64 helper directly at its two UI consumers, so the codec-only crate has no unused import.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/✏️editor/🎭️modes/✏️edit/🪟️windows/🪟️main/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/👁️viewer/🎭️modes/👁️view/🪟️windows/🪟️main/🦀️.rs
