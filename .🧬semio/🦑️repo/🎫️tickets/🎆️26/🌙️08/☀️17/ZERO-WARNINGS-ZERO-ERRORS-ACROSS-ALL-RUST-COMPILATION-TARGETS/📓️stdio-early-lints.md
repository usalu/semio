# Stdio Early Compiler Lints

Pass 366 fixes the early lint failures in WASI357: explicit DWG bit-flag arithmetic grouping, redundant curvature parentheses, a PDF iterator’s unused mutability, orphaned PNG/PPTX documentation left after helper removal, and three duplicate CSV/XML/TIFF example modules. Artifact roots now reexport the examples from their owning standard/subset, so each example source is compiled once. Removed documentation spans were verified to contain no executable tokens.

These early errors stopped the strict compiler before the remaining type-level lint pass. The 12 diagnostics in WASI357 are therefore not a complete remaining-warning count. Fresh checking is required.

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/💡️inferences/🌀️curvature/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs
