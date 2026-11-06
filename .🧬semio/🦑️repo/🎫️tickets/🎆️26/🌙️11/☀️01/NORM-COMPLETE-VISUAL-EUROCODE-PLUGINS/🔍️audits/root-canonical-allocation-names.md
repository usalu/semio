# Canonical Allocation Names in Product Consumers

Exact Rust request/event/budget identifiers were renamed to the shared first-party NativeAllocationRequest, NativeAllocationEvent and NativeAllocationBudget in the following product/actor files. Issuer, progress, outcome and direction-specific native controls remain distinct. No event handler was adapted by this edit, and no test/compile acceptance is claimed; old request-only callbacks must migrate to original issuer loans with actual phase/custody semantics.

- 🧰️framework/🛍️products/📓️print/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/📦️archive/🛬️decode/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🗂️file/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🗂️file/📚️frames/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🧾️metadata/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/📇️identity-records/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🧵️edit/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/⚔️conflict/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🔀️origin/🔣️json/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🔀️transition/🛂️payload/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🔀️transition/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/📋️records/📦️paged/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🚪️io/🪶️sqlite/📸️snapshot/📏️preflight/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🚪️io/🪶️sqlite/📸️snapshot/📏️preflight/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🧪️tests/🪶️sqlite/🫴️native-caller/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🪶️sqlite/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/🚪️io/🪶️sqlite/📸️snapshot/💰️backing/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛬️native-decoding/🧪️tests/🚪️control/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🛫️native-encoding/🧪️tests/🚪️control/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/📏️preflight/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🧪️tests/🪢️partial-projection/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/🎛️control/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🚪️io/🪶️sqlite/📸️snapshot/🛬️reconstruction/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🧬️codec/📥️receiving/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🪪️identity/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🪪️identity/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/🧪️tests/🐎️wasmtime/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/📨️slot/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🚪️io/🎛️operation/🐎️wasmtime/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/📣️host-replies-wasi-and-channel-faults/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🚪️io/📞️imports/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🚪️io/🛂️authority/🪶️snapshot/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🧪️tests/🪶️transfer/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🪶️sqlite/🫴️semantic/🧪️tests/🦀️.rs
