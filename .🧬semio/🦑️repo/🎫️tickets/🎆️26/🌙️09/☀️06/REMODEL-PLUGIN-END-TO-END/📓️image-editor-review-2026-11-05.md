# Semio Image Editor Prerequisite Review — 2026-11-05

Read-only review of engine-owned natural-file repair under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/✏️editor`. No native build, formatting, source edits or test execution was started.

The exact diff removes semio_natural_file! from ArtifactOwnedToolJobFactory and inserts it directly into impl ArtifactEditor for SemioImageEditor beside Snapshot. The macro defines natural_file_codec, encode_natural_file and decode_natural_file, and accesses Self::Snapshot; these are editor capabilities rather than factory publication contracts. Its underlying pack dependency legitimately exports crate name pack through its [lib] name, matching the macro expansion. No duplicate forwarding API, deprecated method or compatibility layer was introduced.

Own package root mounts crate::schema::snapshot and the editor source; the latter explicitly includes its unit test module under cfg(test). New natural-file JSON sits beside that test and its include_str binding is local and correct. Own editor manifest already has serde_json dev dependency plus pack/plugin/pack-json dependencies required by macro/test.

The new fixture describes .semio natural pack metadata (stdio.semio.image@v1, .semio, application/vnd.semio, binary true), a raw 1×1 RGBA snapshot [12,34,56,255] and Unicode metadata 🧱️ Größe. This is a native image-snapshot/pack fixture, not a PNG encode/decode test. The native law compares actual editor codec metadata, admits the fixture via independent serde_json spelling and production pack-json decode, encodes/decodes actual editor natural file, checks exact typed snapshot equality and independently parses production JSON output with serde_json for fixture equality. These assertions cover image bytes and Unicode preservation through native pack plus third-party JSON representation. They do not independently decode PNG or compare a third-party .semio pack implementation.

No concrete placement, type-path, serialization-expectation or own test-module binding mistake was found in scoped source review. Actual Rust compilation remains decisive. Root component/engine/WASM gates compile the repaired dependency, but they do not execute this dependency's new unit law automatically.

Exact focused follow-up gate, once current normal-capacity owners settle, is:

```
repo cargo --store=shared --configuration=prototype --parallelism=4 -- test --manifest-path "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/Cargo.toml" -p semio-s-artifact-stdio-semio-image-editor --lib natural_file_codec_belongs_to_the_image_editor_and_round_trips_the_portable_snapshot
```

This lane did not launch that command. Engine owner received the review directly. Component descriptor generation remains the real owned gate; no binary/generated metadata was patched.
