# PDF image-authoring dependencies staging audit

Attempt 31 (driver `wIM8tHe`) reported unresolved imports for the pixels and stdio contract crates in the PDF document editor's image-authoring code.

The shared [PDF document editor manifest](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/✏️editor/📄️document/📦️packages/🦀️rust/Cargo.toml:16) declares both direct dependencies. The pixels path resolves to the framework pixels package, which publicly mounts `image_decoding` and exposes the input, job and effect-shaped advance used by the importer. The PDF and Hub workspaces supply the stdio contract dependency. That package exposes the three window-kit argument readers used by picker and completion admission.

All seven located `semio-s-artifact-stdio-pdf-editor-document` lockfile records list both direct dependencies. This covers Hub, PDF, Draw, Shooting, Puzzle 5D, Layout, and Animate. A source review of the new module found no further immediate compile blocker. It verified the original effect call and the 32-byte revision witness against their owner declarations. No source edits, lockfile edits or competing Cargo jobs were needed.
