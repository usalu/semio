# Process3D Retirement Error Authority

Read-only source census; no Cargo or source writes.

Actual factory authority is Process3D `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs`. `advance` at 1327 first removes the owner frame via pop_owner. `release_string` at 1310 consumes its String, checks capacity against grant and returns InvariantViolated when too large; the rejected String is ordinarily dropped on that return. String branches 1497–1510 have already taken the field out of the popped owner. Parent reconstruction is committed only at 1575 onward. Early error therefore loses retained frame authority and can ordinarily drop remaining nested ownership.

Vector backing branches (tool solids1340, workshop machines1362, capability arrays1392, rules1413, parameters1424, capability mutation1566) take or consume and drop backing before the final 1572 released_bytes check. That check returns InvariantViolated after release, before reinserting parent. The fixed factory close1627 also caps grants at PROCESS3D_OWNER_BYTES. SQLite draft ownership retire18 drives close_step(256,65536).expect, so a completed typed SQLite owner with larger String/vector capacity can panic during cleanup; this is not a supported unrestricted cleanup path.

Derived DslField retirement (shared derive730–737,816) dispatches each owned field to its declared DslField or DslVariants retirement, including child owners. It does not by itself establish whole-graph iteration or allocator refusal safety. A completed-root cleanup alternative must establish the actual BRep/Flow child retirement behavior and keep partial guards until final validation. Ownership helpers already guard typed collections with declared retirement, pay complete vector backing before children, and use Reconstruction for text/blob; borrowed Cell references are paid later by Projection. These distinct authorities must remain distinct.

Missing witness: actual completed Process3D with text capacity above 65536, empty-but-reserved vector backing above that grant, real child owners, and a failure/cancellation after complete construction. It must prove original refusal retained without cleanup panic, all owned state retired, and allocator-refused cleanup with narrow stack if that guarantee is claimed. No such runtime result is inferred here.

## EncodedRecord Insert Producer Candidates

The following exact files contain `EncodedRecord::new`; direct guarded insert calls within these producers must propagate the new typed insertion result. Ordinary `RecordValue.fields.insert` is a separate API. Shared/OS derive token generation needs its controlled guarded insert updated without altering ordinary field insertion.

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🛫️encode/🦀️.rs`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📦️pack/🦀️.rs`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🛬️native/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🧬️schema/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/🌀️procedural/🫀️core/🧬️generation/🪶️sqlite/🚦️native/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🔗️reference/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/📦️pack/🛫️encoding/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🧬️schema/📸️snapshot/🪶️sqlite/🚦️native/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/📸️snapshot/🪶️native/🛫️encoding/🦀️.rs`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🧪️tests/🫳️borrowed-object/🦀️.rs`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📝️text/🦀️.rs`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧩️component/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header/🧬️schema/📸️snapshot/📝️text/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/📝️text/🦀️.rs`
