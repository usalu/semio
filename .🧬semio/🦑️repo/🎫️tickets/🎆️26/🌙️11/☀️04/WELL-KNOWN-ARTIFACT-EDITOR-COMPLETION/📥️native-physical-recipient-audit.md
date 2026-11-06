# Native Physical Recipient Audit

The current construction recipient trait in 🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🦀️.rs exposes only grant and accept. receive_construction_step_typed posts its selected actual progress through accept; record_constructed_allocation and ordinary native charge change native allocation only. The genuine Job recipient in 🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs:456-459 owns real cumulative receiving_spent and updates it on accept. StepContext receiving_child_progress exposes it only outside the borrowed control; there is no accessor on the recipient/control for nested observers.

The generic IO owner in 🧰️framework/🔨️modules/🚪️io/⏱️control/🦀️.rs records local wallet progress only. Its snapshot implementation 🛫️snapshot/🦀️.rs posts frame birth, body receipts, intermediate retirement and publication into the local driver/performed fields, with charge on native allocation. It does not call construction accept for these fresh effects. A child construction step can already have called accept, so accepting full cumulative wallet progress again double posts. Gap remains on the real original Job caller.

A bounded correction needs the recipient to expose its actual cumulative accepted physical progress with no default fake value, the control to borrow that accessor and accept-only publication on the same original recipient, and per-phase before/after snapshots. Subtract actual child-posted delta from the corresponding local physical delta with checked arithmetic; accept only fresh remainder under real original grant and keep the complete primary and receipt refusal in custody. Frame birth must be published before child demand; retain phase separation for body, close and publication and do not recharge native allocation. Tests must compare actual heap and original Job cumulative accepted receipt for success, post-effect refusal, already-posted child and paid close, plus denied zero work. A scope-wide subtraction without respecting grant timing is insufficient.

Current direct implementer files requiring real accessor handling:

- `✏️s/🔌️plugins/🏗️fem/⚙️engine/📍node/🧪️tests/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/⚙️engine/🧱️material/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/🛬️receiving/🧪️tests/🔬️custody/🦀️.rs`
- `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/🎟️work/🧪️tests/📥️bank/🦀️.rs`
- `🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/🌳️intrinsic/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/📋️list/🔺️edit/🔑️named/📇️index/🌳️root/🛬️receive/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/📝️shared-utf8/🎟️native/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🎟️work/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🧳️original/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/🧪️testing/📥️admission/🦀️.rs`
- `🧰️framework/🔨️modules/📡️replication/🆔️ids/🛬️receiving/🧪️tests/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/native.rs`
- `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🔺️diff/🫴️receiving/🧪️tests/🦀️.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/native.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/native-tests.rs`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/native-tests.rs`
- `🧰️framework/🔨️modules/🖱️ui/🌐️locale/🧪️tests/🌐️axis-value/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️decode/🧳️receiving/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/🔤️json/🧪️tests/🥣️companion/🧮️owned-bytes/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🖼️presentation/🛬️receive/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🌱️values/🗺️fixed-map-and-bytes/🛬️receive/🧪️tests/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/🧭️prelude/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/⚖️law/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/📨️emission/🧳️receiving/🧪️tests/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🧪️tests/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧬️original/🌉️bridge/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧬️original/📦️pack/🧪️tests/🦀️.rs`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧳️state/🧪️tests/🧰️helpers/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️command/🛬️receiving/🧪️tests/🦀️.rs`
- `🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛬️receive/🧪️tests/🦀️.rs`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/💾️store-preparation/🏭️member/🧪️.rs`

Read-only source audit; no native execution.
