# Neural Registry Retirement Surface

Read-only source census on 2026-08-27 found no retained retirement API for `Arc<neural::Registry>`. The engine Registry owns private schema/operator HashMaps plus producer/provider indexes. `Operator` exposes only synchronous `retire_cold(self: Box<Self>)`; `ColdRetire for Registry` performs synchronous domain cleanup. Neither is an interactive retirement authority.

The existing owned `ValueRetirement` supports Dictionary, Value, shared Value, text, evaluation channels, snapshots, and OperatorInfo. It does not accept Registry, Schema, OperatorRecord, or a boxed Operator implementation. `NeuralCacheRetirement` is a separate cache owner and cannot retire Registry authority.

A new retained Registry owner therefore needs an exact final-owner/reader-release protocol, retained one-entry index traversal, typed Schema/OperatorRecord payload cursors, and an operator-owned retirement factory. Unsupported operator payload owners must be denied before adoption. Do not wrap Registry in ColdOwner, invoke synchronous `retire_cold` from a bounded step, or drop the last Arc as one credited item. `Arc::into_inner` is safe as a final-owner transfer only if every release path participates; plain reader Arc drops must not remain capable of becoming final.

Sources: `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️component.rs` (Operator and Registry), `🧊️cold/🦀️component.rs` (ColdRetire), and `🧵️retirement/🦀️component.rs` (ValueRetirement). No source mutation or native execution was performed for this scout.
