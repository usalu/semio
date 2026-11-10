# Clone Source Fixture Effect Frontier

The current 🔗️source/🧪️testing/🦀️.rs is stale: drive uses grant-only advance and return_inputs, two constructor calls still supply only grant/work/marker, take_output is grant-based, and work accounting uses u64 although RetainedWorkOutcome.work is u128. The helper is not a working current Source-bound EffectBank consumer.

This cannot be fixed faithfully inside the helper alone. In current 🔗️source/🎟️admission/🦀️.rs, advance accepts an effect but owner, payload, lease and publication branches report nonzero Work while making no effect.performed call; return_inputs similarly reports one unit for several mutations without a performed call; take_output accepts only a grant and reports one. EffectBank.run issues genuine Source-bound Work and expressly compares returned work with its actual performed receipt. Wrapping these producers and charging their reported or quoted work in the helper would fabricate authority/receipt and is prohibited. 🔗️source/🪪️staged/🦀️.rs also consumes grant-only take_output, so changing that production interface has additional actual callers. The source close implementation has some real effect.performed calls but its complete downstream path is not established here.

Root and server were notified; no production/helper source patch, Cargo job or fake adapter was made. Repair requires partitioning the actual producer and callers, schema-first tests with real Work receipts and original allocation marker, before a helper can safely rely on EffectBank. This observation is static, not a compiled diagnosis.

## Exact Downstream and Caller Boundary

Admission currently delegates to grant-only SealedShared::{admit,prepare_birth,publication.commit,return_borrowed_alias,try_unwrap} in ♻️retirement/🔗️arc-custody/🔒️sealed/🦀️.rs. These have distinct actual Work by validation, issuance and allocator failure; they do not accept an effect. Claiming quoted work before their operation or mirroring their returned count into a receipt afterward would invent or misstate performed work. Effect migration must reach the actual producer boundaries and preserve their partial failures and original markers.

A scoped value-tree source scan identified current production take_output callers beyond staged and the source package wrapper: 📸️snapshot/👁️read/{📋️field-clone,🪜️staging}, 📽️projection/👁️post, 🧬️retained-clone/📦️original-slot, and 📋️list/🔺️edit/🔑️named/📇️index/{♻️transfer,🌳️root/🛬️receive}; source test helpers and multiple Rust tests also call the old form. Several of those production callers already have stale grant-only advance/return calls. This is a scoped text match, not a complete workspace graph. An effect-only take_output signature would require coordinated caller changes.

Coordinator sequenced this behind the current production build frontier; no partial admission migration, synthetic receipt, shim, or selected-cursor registration was added.
