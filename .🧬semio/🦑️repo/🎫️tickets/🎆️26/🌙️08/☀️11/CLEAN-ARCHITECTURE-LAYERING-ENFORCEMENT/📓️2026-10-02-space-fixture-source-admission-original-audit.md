# Space Fixture Source Admission Original Audit

Actual initial source capture: 33 source/fixture/native caller inputs; original modified source bodies remain in the authored ownership fixture, while foreign Draw/Writer asset bytes are never copied. Original source hashes below are observed before production changes. Existing source callers are enumerated mechanically, not inferred from file placement.

## Actual Current Behavior

The lower Space kernel directly includes Draw and Writer demo DSL and passes it to a JSON-only registry. The registry reader is only consumed by its unit test, so existing seed/export operations do not actually retrieve either demo. Space export registers them then passes a fabricated schema-only JSON document to media export. Higher existing Space composition selects Home and Space only; it must explicitly select the actual demo provider packages and consume their unique ExampleSource declarations.

## Exact Proposed Ownership And Runtime Contract

Lower Space owns only generic typed descriptors (opaque slug, Json/Dsl format, codec identifier, source text), explicit owned codec callbacks, validation and admission operations. Actual higher Space composition owns the two original slugs, plugin/app-to-demo bindings and typed snapshot codecs. It reads each existing examples::demo::source() declaration, parses the real DSL through its owned snapshot ArtifactDsl and serializes with owned JSON. OS canonical registry admits validated nonempty snapshot objects atomically and returns normalized documents. No unchecked JSON registration/lookup aliases remain. Admission before catalog/app construction and actual-content resolution during export preserve registration and close the schema-only/empty-JSON defect.

## Required Native Law Scope

Real Draw/Writer original DSL → own snapshot → owned JSON → independent serde_json projection → own snapshot equality; original slug/schema/id/text/layer content retained. Missing codec, mismatched format, malformed DSL/JSON, duplicate slug, empty provider, empty object and failed second item all refuse admission with no partial registry change. Every original Space/Home/OS export and catalog scenario remains required. Native queues compile/run exclusively; portable proofs never imply native materialization.

## Original Inputs

| Path | SHA-256 |
| --- | --- |
| ✏️s/🔌️plugins/🪐️space/🫀️core/🦀️.rs | 5b224ae5fc85d7c5d9fcc5f2d2d37d1a395ec4c5beabd9af383352a075046378 |
| 🌎️hub/🧩️compositions/🪐️space/🦀️.rs | 9f88452ac2c8ef6d5b56241ba28a694c6cd5be3ab16fcd054c40972aaba0b6d8 |
| 🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs | bbcb30dcd7461ec22ea774f90e8e93536183e346666f4320cbd4952d291c6745 |
| 🧰️framework/🛍️products/💻️os/🖥️host/🧪️tests/🔬️instance-unit/🦀️.rs | c5657aa6394906a28556e52e2dd5f20e9db69bafaa0911b277a2cf6008c927cd |
| 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/📤️export-media/🦀️.rs | bfca9887117277b5a31b251d51571ea00aabd3a3144018b7e31d95e20bb3a735 |
| 🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/Cargo.toml | e2ed56369c85cca7a66cd905c9b3c8c36c8be3e522dad125a0e5e3b27ee4db54 |
| 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🦀️.rs | 061851d4b6ff5227c40b828af567479effcc6d05cbe3c131125d441246de8881 |
| 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/⚙️engine/🧪️tests/🔬️unit/🦀️.rs | 473a4d8fe0bb39fa2ba9c0937218fa46946e256fcef61e2819de111292cb2834 |
| 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🧪️tests/🔬️unit/🦀️.rs | 1bf4037aa3777890534713932fbb0140fc8d4db485f024cee853fa243ae8595f |
| 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/🕸️compiled-dag/🦀️.rs | 5e3b1e6d9b910a1b311bc3803d3abcec929635126dca18f4cc2a4af16d0d9a69 |
| 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/🕸️compiled-dag/🧪️tests/🔬️unit/🦀️.rs | 5fb28483f53c0e0fd413b7726315946f2ee543e47ede3eaeaae43127b37912d1 |
| 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/🩹️patch-parameter/🧪️tests/🔬️unit/🦀️.rs | e9231507e6ad39a5ab9c782985b38622ae705ee1927ff53175bd080dc25c9dc9 |
| 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/🚪️open-space/🦀️.rs | f6cd8c4c7fbfa6a4019db5182405f0ad841ba1479f37a4fa15f26f26bcb6b78e |
| 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs | 6989d0a8dbf9a3839410f1635fe960da76d553f6072afa08177da48883681a9a |
| 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs | 090ece4862207cbba0ad0994f55041b221084f7d979d8a058f4edf62d8029c12 |
| 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎭️modes/🌐️main/🪟️windows/🔄️workflow/🦀️.rs | 14439a2077fc7916aec3262e671fc5f39f74dae759fd2cce702e7937fb39f241 |
| 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/🚀️spawn-app/🧪️tests/🔬️unit/🦀️.rs | c82e27b36bcde16f508917f5432d15a04e3a91a8dc72cde5ddf388cc94a7ba4e |
| 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/🔌️connect-media-ports/🧪️tests/🔬️unit/🦀️.rs | 782fe4db812840773a778f94d6c35328808d06c19e5a7930354bc845dc600c67 |
| 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/👥️presence-heartbeat/🧪️tests/🔬️unit/🦀️.rs | ce6b4e1a7c94819fca25b11ac14166cde293d68b32cd5b39513fdce3e8f55b2f |
| 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/📤️export-media/🧪️tests/🔬️unit/🦀️.rs | 787c76464571e4b9c7799de56b26fca2d68b3cf7f5300a113515c342f8cfee08 |
| 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/🔍️open-instance/🧪️tests/🔬️unit/🦀️.rs | 20073c5efbbaea980168054571635735dfcf0595117cdeed537b6700cc1d3499 |
| 🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🎮️commands/🧳️import-space-pack-payload/🦀️.rs | 992ecf8e5f5b629bf121b2cf36cd450e28f9565380fa9cf56260bcd44017003a |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs | f6d884b9c976601cdaf910c93966b50eb29e08b312838e75d57d8e673b0b7cb5 |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🔎️explore/🪟️windows/🏠️main/🧪️tests/🔬️unit/🦀️.rs | cf3c4c652cf7459aa8eb5063711b619036658b9e5988bf61194541da81365a3e |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📎️bind-space-file/🦀️.rs | 2a7dbb6e52a197cf216d2ce752522346df9b7cd690c293ab9fba4f55c7a5ee21 |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-space/🦀️.rs | 70bf67a48c408acb62e34712dd7828e1fb6ee8e90a7c72fc3b5b77b9965c6eca |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💾️persist-locally/🦀️.rs | 3cade1010169a68aa9607ffd504b326c5bb3227de9af2c08c2d808cd71b160e6 |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏗️create-studio/🧪️tests/🔬️unit/🦀️.rs | 8039426bb381e2d9c732bb37cbb7ce09abb61675e6bd339cb6c861557fbab733 |
| ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗃️apply-local-catalog-document/🦀️.rs | 7fb8deba9124d96b4c877d0dca527497dc39c01d4703dea9d94ac85e30353d8a |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs | a69f3f5fbf5d619e3136d6f44b287d612b62652856f6b248f1afaedd22910e5c |
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio | 747a31a8f3aed19638f3c660abe7e47d748f9e81f33715cf443d914458a42f69 |
| ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs | a69f3f5fbf5d619e3136d6f44b287d612b62652856f6b248f1afaedd22910e5c |
| ✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio | 3771413b465c864fd60515e5772714e4e4a052ee1eec559e4bb19b7387479622 |

## Original Unique Examples

- draw: slug `🖍️semio.draw.json`; format dsl; codec `draw.snapshot.v1`; unique declaration `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs`; asset SHA-256 `747a31a8f3aed19638f3c660abe7e47d748f9e81f33715cf443d914458a42f69`,31335bytes.
- writer: slug `✒️jack.writer.json`; format dsl; codec `writer.snapshot.v1`; unique declaration `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs`; asset SHA-256 `3771413b465c864fd60515e5772714e4e4a052ee1eec559e4bb19b7387479622`,297bytes.
