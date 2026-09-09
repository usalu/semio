# glTF Material Fixture Inference

Native718 reported E0283 for the double-sided material undo assertion. The schema field is bool; the assertion now instantiates the fixture decoder as bool explicitly so other PartialEq implementations cannot leave the generic result ambiguous. The direct mutation, inverse, and codec law remains registered in the runtime catalog.

Rust syntax parsing passed. Compiler and runtime verification pending.

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/💎️material/🪞️change-sides/📜️contract/🧪️tests/🔬️unit/🦀️.rs
