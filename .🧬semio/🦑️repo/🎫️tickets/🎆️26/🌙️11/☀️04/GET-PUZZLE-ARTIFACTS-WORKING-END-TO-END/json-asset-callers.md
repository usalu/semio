# 2D External JSON Asset And Caller Audit

Root read-only scope covers canonical Puzzle2d artifact files, Puzzle Storybook hosts and Hub Puzzle composition. No asset was changed.

## Registered Examples

The actual standard/subset examples() registers Nakagin Capsule Tower and Concrete Forest. Both ExampleSource::deferred declarations carry authored .dsl.semio bytes; the only JSON file under the canonical example roots is Nakagin assets/manifest.json, which is asset metadata rather than a snapshot. Pack, SPR and Op companions also remain their own carriers. The editor demo source carries .cmd.semio. No committed stdio JSON snapshot is established by this scoped inventory.

Both deferred document_json callbacks parse DSL and serialize native ToValue. Their editor set-active-example consumers directly call semio_framework_pack_json::from_json_str<Puzzle2dSnapshot>; native numeric fields are intentional here. Existing every-example law uses the same native semantic parser. ExampleSource itself retains authored bytes for asset identity and merely invokes its callback for document_json/payload; it does not dispatch the declared stdio JSON importer. These callbacks and neutral mutation before/after JSON fixtures must remain native typed state.

## WASM And IO Callers

The Puzzle2d WASM puzzle2d_parse_dsl_json bridge directly parses ArtifactDsl then returns native pack_json output for the numeric board DTO. It does not invoke declared JSON import/export. The scoped direct declared-JSON callers are the real canonical IO component analyze/compose branches and native tests; no numeric external committed asset is identified. The new converter therefore belongs solely at those declared IO leaves. Source adapters, deferred examples and board DTOs retain their actual separate type contracts.

The explicit external wire schema now under review describes exact-word import/export; default/presence policy must agree with native typed decoding, while canonical output retains every independent expected field. No claim that all repository JSON files are external assets, no compatibility fallback, and no runtime asset rewrite follows from a .json suffix alone.

This is source/caller evidence only. Current root11101 is an actual Cargo compile, not a native/guest IO pass. The existing full physical JSON law and new inferences-owned declared_json_ negative/presence laws both need nonzero actual execution.
