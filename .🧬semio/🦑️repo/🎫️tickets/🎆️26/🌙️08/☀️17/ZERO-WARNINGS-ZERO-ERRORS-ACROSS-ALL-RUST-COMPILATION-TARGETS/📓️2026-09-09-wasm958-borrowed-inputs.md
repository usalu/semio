# Borrowed Read-Only Inputs

Flow status projection and the shared BREP error mapper now borrow their input; 48 mapper call sites use the same display error text. FEM encodes 13 borrowed scenes and borrows eight layer identifiers, including the existing language-neutral vector oracle. Fourteen note mutation leaves borrow their updated block during JSON projection. Sequence host construction no longer clones its snapshot. Energy installs preview data directly from its borrowed packet projection. Drawing snapshot diff projection borrows its input, and reconstruction queue emission carries an optional borrowed continuation.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs
- ✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs
- ✏️s/🔌️plugins/🏗️fem/⚙️engine/🖥️app-surface/🦀️.rs
- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs
- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧱️model/🦀️.rs
- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs
- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs
- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧱️model/🦀️.rs
- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs
- ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🧪️tests/🔬️vector-json-contract/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧮️math/🧬️schema/🧬️mutations/🧮️edit-block-math/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬅️remove-table-column/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/👀️change-block-visible/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬇️insert-table-row/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/➡️insert-table-column/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📝️text/🧬️schema/🧬️mutations/📝️edit-block-text/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔒️change-block-locked/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/📊️table/🧬️schema/🧬️mutations/⬆️remove-table-row/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🎨️edit-block-ink-stroke/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔤️change-block-font-size/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/↔️resize-block/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/📍️move-block/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🖋️ink/🧬️schema/🧬️mutations/🖊️change-block-ink-width/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/🧱️block/🧬️schema/🧬️mutations/🔖️rename-block/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
- ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/🦀️.rs
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️run-reconstruction/🦀️.rs
