# CAD and Raster Progress Inline Layout

Added neutral layout budgets of 64 bytes for CAD configuration mutations and 128 bytes for raster asset progress. The CAD regression retains inverse and text codec checks, adds a binary round trip, and compares selected IDs with serde_json. The raster regression decodes a neutral asset through serde, wraps the actual mutation, and compares its first-party JSON projection with the original fixture through serde_json. Both are added to the combined regression catalog together with existing contribution, durable-chunk, and mesh-envelope regressions. Baseline runtime size failures are expected before representation repair.

All complete Rust sources parsed before guarded writes. Strict compilation and runtime validation remain pending.

- ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧫️fixtures/📦️inline-layout/🔣️.json
- ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️run-reconstruction/🧫️fixtures/📦️inline-layout/🔣️.json
- ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️run-reconstruction/🧪️tests/🔬️unit/🦀️.rs
- /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/📜️script.ts
