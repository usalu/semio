# glTF Direct Mutation Contract Tests

The ten mounted contract tests still imported removed descriptor/diff/inverse modules and called Serde for types that implement the owned value codec. Rebound them to the current direct mutation leaves. Shared fixture inputs and expected state projections are decoded through the owned codec and checked through Serde JSON; forward application, deterministic diff application, typed diff wire round trips and aggregate inverse restoration remain executable for every vector. Applicable payload range, reference, malformed-wire, no-change and scene-integrity rejections remain covered. Old descriptor-owned touched-path forgery and stale phase payload assertions cannot apply to the current typed diff API and are not represented as passing checks. Existing fixture files are unchanged because other language implementations still consume their independent facets.

12 Rust files parsed. Compilation and runtime execution are pending.

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🧪️tests/🔬️contract/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌿️node-child/🔗️bind/📜️contract/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌿️node-child/✂️unbind/📜️contract/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌲️scene-root/🔗️bind/📜️contract/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌲️scene-root/✂️unbind/📜️contract/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/📝️change-extras/📜️contract/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🌳️node/🏷️rename/📜️contract/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/💎️material/🌫️change-alpha/📜️contract/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/💎️material/🪞️change-sides/📜️contract/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎬️scene/🌱️create/📜️contract/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/🧬️mutations/🎬️scene/🗑️delete/📜️contract/🧪️tests/🔬️unit/🦀️.rs
