# Factory payload work inventory

The factory payload trait exposes physical birth/copy/depth, staged preparation demand and reported turn work but no computational quote for synchronous prepare_close_state. Blanket preborn allocates a FactoryTicket and invokes this arbitrary method, then reports work 1, which does not prove its actual cost. The derive emits a fixed child-slot array and recursively invokes inline field prepare_close_state; the later staged preparation delegates actual child and inline preparation turns separately. A quote/admission must account for actual synchronous implementations and failed work as well as recursive inline construction.

Handwritten production implementations observed: atomic scalar factory and VCS ArtifactGroupVisibility construct unit; ReadOwnershipRegistry constructs None. Test support DemoPublishedRootWitness constructs empty fields and DemoOneItemPreparationFactory constructs an empty fixed two-slot state; its child preparation is later staged. No production norm implementation or derive was found by this scoped textual search.

Direct preborn call sites include FactoryAuthority and FactoryChildSlot, which preserve turn.work, and Flow retained copy/🦀️.rs:516, which extracts result and discards turn.work. Physical demand and receipt are independent from computational quotation.

The following source files contain derive occurrences (textual inventory, includes tests and macro definition; reachability is not established):

- 🧰️framework/🔨️modules/🛠️tool-machine/🔗️read/🧪️tests/🦀️.rs
- 🧰️framework/🔨️modules/🛠️tool-machine/⌨️typing/✍️edit/🧪️tests/🦀️.rs
- 🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️.rs
- 🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️decode/🧳️receiving/🧪️tests/🦀️.rs
- 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📦️original-slot/🧪️tests/🦀️.rs
- 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🦀️.rs
- 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🚼️construction/🧪️tests/🦀️.rs
- 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🧪️tests/🦀️.rs
- 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/🧪️testing/🦀️.rs
- 🧰️framework/🔨️modules/🌱️value/♻️retirement/🏭️factory/📋️vector/🧪️tests/🦀️.rs
- 🧰️framework/🔨️modules/🌱️value/📸️snapshot/👁️read/🔗️source/🧪️tests/🦀️.rs
- 🧰️framework/🔨️modules/🌱️value/🧪️testing/👥️presence-retirement/🦀️.rs
- 🧰️framework/🔨️modules/📡️replication/🎮️mutation/♻️retirement/🦀️.rs
- 🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/📖️reader/🧪️tests/📖️reader/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/♻️retirement/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧳️source/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧪️tests/🔬️unit/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/🏁️publication/↩️inverse/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🎮️operation/🧬️native/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🧵️borrowed/🧪️tests/🧵️borrowed/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️operation-wire/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧰️test-support/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧪️tests/🔗️source-custody/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/❄️cold-apply/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️paged-one-item/🧬️mutation-apply/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧮️cursor-ledger-and-vcs-retirements/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🗃️history-ledger/♻️retirement/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🧪️tests/🔬️unit/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/✏️config-apply/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/🎭️doubles/♻️retirement/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/🎭️doubles/🌱️retained-edit/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️testing/🎭️doubles/🗄️store/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🧱️fields/📥️lifted/🧬️owned-schema/🧵️owned-spr-edit-authority-and-history-decoder/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🧱️fields/📥️lifted/🧬️owned-schema/🗂️envelope-field-constants-and-authority-traits/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/🧪️tests/🔬️unit/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🧩️preparation/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🫧️ephemeral/📢️publication/🔁️transfer/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/📑️copy/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌴️bin-06/🔬️unit/🪙️retained-backing-and-terminal-cursor-demand-laws/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌆️bin-02/🔬️unit/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌈️bin-03/🔬️unit/🧳️retained-clone-preparation-fixtures/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🏙️bin-01/🔬️unit/📜️edit-retirement-message-ledger-and-displaced-owner-laws/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/♻️retirement/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🧬️native/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🧬️native/🧪️tests/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🗝️hash-projection-and-recovery-fixtures/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👥️presence/♻️retirement/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🔐️two-turn-presence-and-root-retirement/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/📨️emission/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-7/🆔️request-context-serializer-and-shared-job/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-config/🧬️preparation/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-1-x/🌳️field-catalog-parent-app-and-branch-snapshot/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract-part-3/🪦️snapshot-retirement-gesture-and-agent-transaction/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️lifecycle/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📖️capture/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/♻️retirement/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📝️draft/🚫️none/♻️retirement/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🕹️interaction/📃️query/🧪️tests/📃️query/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🧾️issuer/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧪️tests/📷️camera-owner/🧬️preparation/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧬️preparation/🔁️replace/🦀️.rs
