# Store Snapshot Producer Source Frontier

## Changes

Schema-first member decoder capability and corpus define exact request-owned Pack borrow, selected typed snapshot, original member request/StepContext authority, original concrete-body/input receipt and paid-close custody. Store now defines `MemberSnapshotDecoder<P>` and `ProducedMemberSnapshotOpen<P,D>`; the old `PackMemberSnapshotOpen` type/export has been removed, with all actual Rust/TypeScript/macro consumers corrected. The original operation is retained in place: framed input reservation/chunks, caller/callee grant minima, input receipt debit, authority checks, snapshot/request take, active retirement, input free and terminal paid close remain original. A zero-sized function-type marker selects its decoder without another runtime owner or allocation. Existing native members select their authored Pack decoder explicitly. Actual Flow selects child-owned `FlowSnapshotPackProducer` and keeps its existing DSL record/field construction child-owned.

The existing actual Flow member law now follows that producer and compares its actual typed output with the existing portable serde JSON corpus, followed by the original retirement. Existing Store physical input/receipt source laws target the new defining source, and an independent Ajv/JSONPatch corpus law refuses replacing original source custody. Whole-snapshot decoding remains the original synchronous decode; this source extraction does not qualify bounded cooperative native decoding.

Schema-first genesis producer capability and corpus define encode-once birth, decode-once exact stored Pack reload, exact Pack hash, original typed admission and shared snapshot/Pack allocation identity. `ArtifactGenesisCodec` on snapshot types has been removed. `ArtifactGenesisProducer<P>` is now explicitly borrowed by `AdmittedArtifactGenesis::born` and `from_stored_pack`; exact original Pack/hash/verified admission mechanisms are unchanged. Generic native Pack callers select `ArtifactPackGenesisProducer`. Flow's original native encode/decode mechanism is owned by `FlowSnapshotPackProducer`, used by its existing native Pack implementation and selected member/genesis producer interfaces. Native Flow genesis law compares exact stored buffer pointer, digest, shared Arc identity and portable independent JSON; no new fixture authority, Heap witness or synthetic source was introduced. The original immutable genesis native law still uses its existing codec/counters/byte vectors/hash oracle through its selected producer. Existing source genesis oracle validates the new producer corpus independently.

The ordinary ArtifactStore implementation, original paid CloseView and equality/convergence operations now require neutral typed value/snapshot ownership bounds. `new` and `construct` retain their specific `ArtifactStoreSnapshot` bound for original native publication; `snapshot_pack` retains its specific representation bound. The two actual media ports `snapshot_owner()` and `owned_snapshot_retirement_factory()` are unchanged. The existing actual derived-read native law now calls a generic helper which can compile with only neutral snapshot/value bounds, captures its same original Arc, borrows the installed original supplier and checks both pointer identities without introducing a source copy or owner replacement.

## Remaining Integration

Store construction still selects native publication from the snapshot. The retained initializer, generic envelope/FromValue paths, composition member owner declaration and selected Store document codec still have implicit snapshot representation bounds. The next required integration is explicit constructor publication and producer-driven envelope/native receiving over exact original sources, followed by moving the neutral Flow snapshot/mutation contracts and their consumers. The Flow parent still has its concrete Flow artifact dependency and concrete child dev-dependency/child-law references; these are not deletion-qualified. Parent test closure must move real selected child laws beside a valid composition/child owner once defining contracts permit that without a dependency cycle. Nothing in this bounded turn qualifies root Cargo deletion admission or parent survival after deletion.

Catalog ownership confirmed no shared Store/DocumentStoreOwners writes/holds before integration. Only an unused former implicit genesis import was removed in DocumentStoreOwners; catalog structure, actual source admission/constructor prices, factories, retirement tickets and source ownership remain unchanged. No modifying Git/worktree/AGENTS, foreign lock/process changes, quota renewal, alternative initializer, fake Source receipt or raw Cargo/test bypass occurred.

## Qualification

No source/native route was dispatched in this continuation. Root owns the genuine dependency lock/sync publication and subsequent source cohort; the fresh root sync3 receipt is not a Store/genesis native or deletion receipt. Authored laws remain unrun. Static reads checked exact literal Rust include targets, exact literal TypeScript URL targets and JSON parsing for this source roster. Maximum absolute roster path length: 152. These reads do not prove Rust compilation, source assertions, native allocations or runtime behavior.

## Source Observation Roster

SHA-256 values below observe exact shared file contents after this bounded source extraction. They are not an immutable Source capsule, proof of source custody, or task lock. Concurrent owner edits remain present.

| Source | SHA-256 |
| --- | --- |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🦀️.rs | `ac5cbec93f17f40ab09487e58327d3dac3b26b7fcafa7c85013b84094b8993b9` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🦀️.rs | `ecaf1a46aea7e885023622152a9530edec99d94e6ab7d42993b73f63c701d13f` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🧵️retained/🧪️tests/🔬️unit/🦀️.rs | `2d08a630ccd2c808f4504baeb9bc55fa27151cd064489999b4260f6bccd2a5f6` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🦀️.rs | `fbe6ad3672a5d5040a1eda6645329a8f308abc5dc6bd4f6fd4245fb14ef87fd1` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🌿️vcs/🧪️tests/🌿️vcs/🦀️.rs | `35291ef5a6793064083a3436f55893166e2cb3295082187aed0476adf197413f` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🚪️io/💾️binary/📸️snapshot/🦀️.rs | `d8805da01b5ac3ebf8f59470e06f504bf47f48250f71b0331cf99316b433d759` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🌱️genesis/🦀️.rs | `0ec7b0a4b25a65345c3bc310be4aebf8f7d4eab57ecf5a50726cab60d3bcab1d` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🌱️genesis/🧬️schema/🔣️.json | `fee33dc55ad4ab772a375855b6640baffc06244c74689a4249e12302e1713a9e` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🌱️genesis/🧬️schema/🧪️tests/🧫️fixtures/🔣️.json | `4c711d72e13c34c6f08e4475e69b77ce8fb3722a93070ebfb3e9c6ed974950f6` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/📝️text/🌱️genesis/🦀️.rs | `cf0723d75b4f802c162ec8059e1c06eaff180133efb3eb4cdeead7d15d49ddef` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/📝️text/🌿️vcs/🦀️.rs | `b9e69c17b768daf71f7b6a624fb60b54f41a438fc7460b4dad8a3afaeccaf229` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/🦀️.rs | `803356cf8832e3490aebca5e98871e48cd9a1cf56f536212e8d0fe605f47796e` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏠️owners/🦀️.rs | `2d62deb525246acbb7ad3a9b00d8bb61b1367e224e5850456f2a38c24221ae25` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🦀️.rs | `81ab4cb378a5f7401f0ec2430f4361bb66b6435f2811a8253811a9c044f19d8e` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🦀️.rs | `0eaff28ca03a31dd7070e9ca1af3a16dcab08267da5e3e2642b696ae6f54c1c4` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/🦀️.rs | `67fd8c4ffa1a474b08c038fdf4ccdfbf0a976954386f4ce98a692a8c2fbf7061` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🦀️.rs | `e2c0723dd8eb615f4f00cb296ce3dcb32e500807261023e70868ddbbbf2d5281` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🚪️io/📝️text/🦀️.rs | `d4edfbeaad147ad826b8f4b0f2e23a517231f833b8905035216b83dbc31790d0` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs | `edc3bb9cf41c5870eb3fdace1078102816d3c86699fce5b05c847401e5743f63` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs | `95e260108918ae0dd6291d9b80ff57ee93e8fd02993bdb5b29f688be91f44813` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🧬️schema/📦️decoder/🔣️.json | `1d9d21047f4020103b617afe44d63581e336586f8fb6395f9c89476db96623a8` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🧬️schema/📦️decoder/🦀️.rs | `20ab2cc6e1c6b531054f1d456d0ddef6a03fb065677c04024db7c7f173a0914b` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🧬️schema/📦️decoder/🧪️tests/🧫️fixtures/🔣️.json | `70c4f3a4b4a8fba5563c57293ef556a2dae9aa355f59af41789f3e4489850d6b` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🦀️.rs | `bbcbec771e236eb1347ff8e4717385be3a8299140a96e1b1755badc4c63ee2c7` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🧪️tests/🔬️unit/📏️retirement/🟦️.ts | `6aa073688b0c5f7ef51f9c27faae5e58ad7960ce8a7819023ebedc47270a7e01` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🦀️.rs | `e17399dc03e26500407161bbbcd8a68214157c47233e2360cf70104110384812` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs | `a4c17c524edc086d65ce0545c4ad83df1f5e9fab03fb53c458948242da20a7fb` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧪️supersede-replay/🟦️.ts | `0899d4ecc4ee0b70dbfb483d3cad79c15784bd8b13d29f57203d916be97df644` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/📥️snapshot-capabilities/🔣️.json | `9e1b073374761d727104298d53ed709fb7fb65351a663fc36a61f81daa927f55` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/📥️snapshot-capabilities/🧪️tests/🧫️fixtures/🔣️.json | `1ab3bc074d687cf7975aea16ff707a3ff3e31d146d62c87c2384775598639562` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/🦀️.rs | `aa9b2a4cb3e83576d20f807d3ee218bbb300a61dab69f80c7d69130ff3f17a5f` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs | `fd8c49db1d089dc66c3188b91359ecfd1a75cb6027d88b232a896d6a749c424d` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document/🦀️.rs | `1062b4176b10a01ef21a980d472a81450aa9800c698ead6b66c622ed3649199d` |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs | `82c611bc1f1ccf6af3af420d966c7b7b112eb098e8798b5adc45507b8460d904` |
| 🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs | `70a9fe577e6f6f70104f5222c1b99db4578d2c393f5bcfd43964f75950edcf5c` |
