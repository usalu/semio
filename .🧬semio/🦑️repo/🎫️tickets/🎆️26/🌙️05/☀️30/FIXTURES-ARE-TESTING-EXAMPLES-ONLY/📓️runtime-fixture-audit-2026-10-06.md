# Fixture Runtime Audit

Read-only source audit on 2026-10-06. Paths below are repository-relative. No code or Git changes, tests, or runtime verification were performed by this audit.

## Confirmed Runtime Ownership Violations

Runtime examples should become semantic catalog/sample sources only when product behavior actually needs them. A rename from fixtures to assets is insufficient: identify the real document, catalog, locale or policy owner; remove runtime examples that exist solely for tests; place test-only artifacts and replay helpers in test support. Tests can exercise canonical product examples, but fixtures themselves must remain exclusively test examples.

- `🌎️hub/🧩️compositions/🪐️space/🦀️.rs` unconditionally mounts `🧫️fixtures/🦀️.rs` as `demo_fixtures`. Assembly calls `admit`; export-media calls `document_for_app`. This implementation registers actual draw and writer demo examples. Move it to canonical example/document source ownership and remove fixture terminology from APIs.
- `✏️s/🔌️plugins/🪐️space/🫀️core/🧫️fixtures/🦀️.rs` supplies runtime `SpaceFixtureSource`, `SpaceFixtureFormat`, `SpaceFixtureCodec`, preparation and registration. Core reexports these unconditionally. The canonical domain is authored example sources and decoding, not test fixtures.
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs` owns the unconditional `OS_FIXTURE_DOCUMENTS` registry, `register_os_fixture_documents`, and `os_fixture_document`. These are runtime document ownership APIs; replace their fixture vocabulary and connect them to canonical example/document ownership.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/⏪️time-travel/🦀️.rs` embeds `🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band/🔣️.json` unconditionally as its labels/refusal policy. `band_corpus` parses it for production. Extract canonical locale and refusal-policy data, then retain independently asserted testing examples.
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` unconditionally embeds `../🧫️fixtures/🪆️child/🏢️initial/🪆️content/🔣️.json` as `NAKAGIN_CHILD`; `nakagin_fixture` constructs an authored Nakagin asset. Move the asset to its actual example/asset owner and retain fixtures only as test input.

## Runtime Test Support

- Store sync `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs` unconditionally defines `ActorFixture`, `FixtureInbound`, raw manifests and `load_fixtures` in its Fixtures region. Repository references found only in its unit tests. Move the entire region into test ownership or gate it with `cfg(test)`.
- Store root `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` unconditionally exports `test_support`, with fixture-report and assertion functionality used by downstream artifact tests. A simple `cfg(test)` gate cannot support dependency tests, since dependencies compile without that flag. Extract a dedicated dev-dependency test-support owner, or use an explicit feature enabled exclusively by dev-dependencies.

## Runtime Vocabulary Requiring Canonicalization

Infinite directed and undirected board modules expose `FixtureJson`, `fixture_layout`, `apply_*_to_fixture_v1_*`, `set_fixture_drop_preview_json`, and `parse_fixture_json`. These operate on runtime host snapshots and scene descriptors. Their domain contracts and names should reflect those meanings. Playbook's production snapshot has `fixture_slug`; it should identify its actual example/document source.

## False Positives to Preserve

- Puzzle retained-catalog fixture includes are guarded by `cfg(all(test, feature = "component-app-assembly"))`.
- TIFF mutation `test_case` fixture includes are guarded by `cfg(test)`.
- Store root fixture mutation module is guarded by `cfg(test)`.
- OS fixture sweep is guarded by `cfg(test)`.
- Fixture paths in Nx inputs represent dependency/cache inputs and do not alone prove production bundling.

## Validation

Use the existing Bun/Nx project test and check targets for OS kernel, OS host, hub space and Trinity rewriting after extraction. Hub space project name is `@semio-tech/space-plugin`, with `test`, `test-quick`, and `test-long` targets. Add a language-neutral boundary corpus with positive test-only references and negative runtime references, validate it using a third-party parser oracle, and ensure the gate understands Rust `cfg(test)` and test modules. Verify production builds without test-only features as well as tests that consume shared support through dev-dependencies. Audit runtime includes, imports and paths with `rg`; distinguish fixtures from comments describing tests.
