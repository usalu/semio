# Current Native Matrix Assembly Compile Failures

The current57-crate assembly group stopped before any editor assertions. Compiler diagnostics are current-source evidence, not acceptance failures. The previous39-crate group stopped at the Workflow borrowed fields, subsequently repaired.

## 🖼️bitmap: 23 Diagnostics

- 12 × error[E0433]: cannot find `io` in `crate`
- 3 × error[E0432]: unresolved import `set_solve`
- 2 × error[E0061]: this method takes 5 arguments but 4 arguments were supplied
- 1 × error[E0425]: cannot find function `bitmap_artifact_inference_descriptor` in module `schema::inferences`
- 1 × error[E0425]: cannot find function `bitmap_json_decode` in module `super`
- 1 × error[E0425]: cannot find function `bitmap_json_encode` in module `super`

```text
error[E0432]: unresolved import `set_solve`
 --> 🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🚪️io/💾️binary/🧬️mutations/🦀️.rs:8:5
  |
8 | use set_solve::SetSolve;
  |     ^^^^^^^^^ use of unresolved module or unlinked crate `set_solve`
  |
  = help: if you wanted to use a crate named `set_solve`, use `cargo add set_solve` to add it to your `Cargo.toml`

error[E0432]: unresolved import `set_solve`
```

```text
error[E0432]: unresolved import `set_solve`
 --> 🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/📦️packages/🦀️rust/../.././././🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🚪️io/📝️text/🧬️mutations/🦀️.rs:8:5
  |
8 | use set_solve::SetSolve;
  |     ^^^^^^^^^ use of unresolved module or unlinked crate `set_solve`
  |
  = help: if you wanted to use a crate named `set_solve`, use `cargo add set_solve` to add it to your `Cargo.toml`

error[E0433]: cannot find `io` in `crate`
```

```text
error[E0433]: cannot find `io` in `crate`
 --> 🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/📦️packages/🦀️rust/../.././🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🚪️rooms-16/🧪️tests/🧩️example/🦀️.rs:5:12
  |
5 | use crate::io::text::snapshot::{parse_dsl, print_dsl};
  |            ^^ unresolved import
  |
help: a similar path exists
  |
5 | use crate::core::io::text::snapshot::{parse_dsl, print_dsl};
```

## 🌦️epw: 40 Diagnostics

- 16 × error[E0433]: cannot find type `EpwSnapshot` in this scope
- 12 × error[E0425]: cannot find type `EpwSnapshot` in this scope
- 2 × error[E0433]: cannot find `text` in `snapshot`
- 2 × error[E0433]: cannot find `binary` in `snapshot`
- 1 × error[E0428]: the name `io` is defined multiple times
- 1 × error[E0119]: conflicting implementations of trait `ArtifactPack` for type `subsets::any::schema::snapshot::component::EpwSnapshot`

```text
error[E0428]: the name `io` is defined multiple times
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📦️packages/🦀️rust/../../🦀️.rs:198:1
    |
159 |                 pub mod io;
    |                 ----------- previous definition of the module `io` here
...
198 | pub mod io;
    | ^^^^^^^^^^^ `io` redefined here
    |
```

```text
error[E0433]: cannot find `text` in `snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📦️packages/🦀️rust/../../🦀️.rs:115:33
    |
115 |         grammar: Some(snapshot::text::COMPONENT_GRAMMAR_SEMIO),
    |                                 ^^^^ could not find `text` in `snapshot`

error[E0433]: cannot find `text` in `snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📦️packages/🦀️rust/../../🦀️.rs:116:38
    |
```

```text
error[E0433]: cannot find `text` in `snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📦️packages/🦀️rust/../../🦀️.rs:116:38
    |
116 |         grammar_path: Some(snapshot::text::COMPONENT_GRAMMAR_PATH),
    |                                      ^^^^ could not find `text` in `snapshot`

error[E0433]: cannot find `binary` in `snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📦️packages/🦀️rust/../../🦀️.rs:117:34
    |
```

## 📼️avi: 48 Diagnostics

- 17 × error[E0425]: cannot find type `AviSnapshot` in this scope
- 15 × error[E0433]: cannot find type `AviSnapshot` in this scope
- 6 × error[E0425]: cannot find type `AviStreamFormat` in this scope
- 1 × error[E0428]: the name `io` is defined multiple times
- 1 × error[E0433]: cannot find type `AviStreamFormat` in this scope
- 1 × error[E0119]: conflicting implementations of trait `ArtifactPack` for type `standards::v1_0::subsets::any::schema::snapshot::component::AviSnapshot`

```text
error[E0428]: the name `io` is defined multiple times
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/📦️packages/🦀️rust/../../🦀️.rs:186:1
    |
147 |                 pub mod io;
    |                 ----------- previous definition of the module `io` here
...
186 | pub mod io;
    | ^^^^^^^^^^^ `io` redefined here
    |
```

```text
error[E0425]: cannot find type `AviStreamFormat` in this scope
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/📦️packages/🦀️rust/../../././././🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:5:175
  |
5 | ...work_dsl_record::Shape::Statements(encoded)=<AviStreamFormat as semio_framework_dsl_record::DslField>::shape_controlled(&mut encod...
  |                                                 ^^^^^^^^^^^^^^^ not found in this scope
  |
help: consider importing this enum through its public re-export
  |
1 + use crate::standards::v1_0::subsets::any::schema::snapshot::AviStreamFormat;
```

```text
error[E0425]: cannot find type `AviStreamFormat` in this scope
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/📦️packages/🦀️rust/../../././././🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:6:175
  |
6 | ...work_dsl_record::Shape::Statements(decoded)=<AviStreamFormat as semio_framework_dsl_record::DslField>::shape_controlled(&mut decod...
  |                                                 ^^^^^^^^^^^^^^^ not found in this scope
  |
help: consider importing this enum through its public re-export
  |
1 + use crate::standards::v1_0::subsets::any::schema::snapshot::AviStreamFormat;
```

## 🔊️wav: 111 Diagnostics

- 29 × error[E0433]: cannot find type `WavSnapshot` in this scope
- 27 × error[E0433]: cannot find type `WavData` in this scope
- 15 × error[E0425]: cannot find type `WavSnapshot` in this scope
- 9 × error[E0422]: cannot find struct, variant or union type `WavSnapshot` in this scope
- 9 × error[E0433]: cannot find type `WavChunkRef` in this scope
- 3 × error[E0422]: cannot find struct, variant or union type `WavFmt` in this scope

```text
error[E0428]: the name `io` is defined multiple times
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📦️packages/🦀️rust/../../🦀️.rs:180:1
    |
141 |                 pub mod io;
    |                 ----------- previous definition of the module `io` here
...
180 | pub mod io;
    | ^^^^^^^^^^^ `io` redefined here
    |
```

```text
error[E0425]: cannot find function `wav_data_spec_producer` in module `crate::standards::riff_pcm::subsets::any::io::sqlite::snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📦️packages/🦀️rust/../../././././🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:41:88
    |
 41 | ...te::snapshot::wav_data_spec_producer(),crate::standards::riff_pcm::subsets::any::io::sqlite::snapshot::wav_chunk_ref_spec_produc...
    |                  ^^^^^^^^^^^^^^^^^^^^^^ not found in `crate::standards::riff_pcm::subsets::any::io::sqlite::snapshot`
    |
note: function `crate::standards::riff_pcm::subsets::any::schema::snapshot::component::wav_data_spec_producer` exists but is inaccessible
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:131:1
    |
```

```text
error[E0425]: cannot find function `wav_chunk_ref_spec_producer` in module `crate::standards::riff_pcm::subsets::any::io::sqlite::snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📦️packages/🦀️rust/../../././././🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:41:177
    |
 41 | ...:snapshot::wav_chunk_ref_spec_producer()].iter().enumerate(){
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `crate::standards::riff_pcm::subsets::any::io::sqlite::snapshot`
    |
note: function `crate::standards::riff_pcm::subsets::any::schema::snapshot::component::wav_chunk_ref_spec_producer` exists but is inaccessible
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📦️packages/🦀️rust/../../././././././🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:270:1
    |
```

## 🎥️mp4: 46 Diagnostics

- 15 × error[E0433]: cannot find type `Mp4Snapshot` in this scope
- 13 × error[E0425]: cannot find type `Mp4Snapshot` in this scope
- 6 × error[E0433]: cannot find type `Mp4CodecFormat` in this scope
- 1 × error[E0428]: the name `io` is defined multiple times
- 1 × error[E0432]: unresolved import `crate::standards::isobmff::subsets::any::schema::Mp4AnalyzerAnalysis`
- 1 × error[E0119]: conflicting implementations of trait `ArtifactPack` for type `isobmff::subsets::any::schema::snapshot::component::Mp4Snapshot`

```text
error[E0428]: the name `io` is defined multiple times
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/📦️packages/🦀️rust/../../🦀️.rs:186:1
    |
147 |                 pub mod io;
    |                 ----------- previous definition of the module `io` here
...
186 | pub mod io;
    | ^^^^^^^^^^^ `io` redefined here
    |
```

```text
error[E0432]: unresolved import `crate::standards::isobmff::subsets::any::schema::Mp4AnalyzerAnalysis`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/📦️packages/🦀️rust/../../././././🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️codec/🦀️.rs:114:9
    |
114 |         Mp4AnalyzerAnalysis,
    |         ^^^^^^^^^^^^^^^^^^^ no `Mp4AnalyzerAnalysis` in `standards::isobmff::subsets::any::schema`
    |
    = help: consider importing this struct through its public re-export instead:
            crate::Mp4AnalyzerAnalysis

```

```text
error[E0433]: cannot find type `Mp4Snapshot` in this scope
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/📦️packages/🦀️rust/../../././././🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:7:14
  |
7 |   assert_eq!(Mp4Snapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),snaps...
  |              ^^^^^^^^^^^ use of undeclared type `Mp4Snapshot`
  |
help: consider importing this struct through its public re-export
  |
1 + use crate::Mp4Snapshot;
```

## 🌐️html: 262 Diagnostics

- 36 × error[E0433]: cannot find type `SnapshotEncoding` in this scope
- 29 × error[E0433]: cannot find type `HtmlNode` in this scope
- 26 × error[E0433]: cannot find type `SqliteDatabaseLimits` in this scope
- 20 × error[E0433]: cannot find type `SqliteSnapshotControl` in this scope
- 19 × error[E0425]: cannot find function `parse_html_document` in this scope
- 18 × error[E0425]: cannot find type `HtmlSnapshot` in this scope

```text
error[E0428]: the name `io` is defined multiple times
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/📦️packages/🦀️rust/../../🦀️.rs:180:1
    |
141 |                 pub mod io;
    |                 ----------- previous definition of the module `io` here
...
180 | pub mod io;
    | ^^^^^^^^^^^ `io` redefined here
    |
```

```text
error[E0428]: the name `io` is defined multiple times
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/📦️packages/🦀️rust/../../🦀️.rs:183:1
    |
141 |                 pub mod io;
    |                 ----------- previous definition of the module `io` here
...
183 | pub mod io;
    | ^^^^^^^^^^^ `io` redefined here
    |
```

```text
error[E0432]: unresolved import `crate::standards::v5::subsets::any::schema::snapshot::component::controlled_native`
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/📦️packages/🦀️rust/../../././././🏅️standards/🔖️5/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🚦️native/🦀️.rs:2:70
  |
2 | use crate::standards::v5::subsets::any::schema::snapshot::component::controlled_native::{HtmlSnapshot, HtmlNode, HtmlAttr, RawTextKind};
  |                                                                      ^^^^^^^^^^^^^^^^^ could not find `controlled_native` in `component`

error[E0425]: cannot find type `AviStreamFormat` in this scope
 --> 🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/📦️packages/🦀️rust/../../././././🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:7:602
  |
```

## 📑️tsv: 8 Diagnostics

- 2 × error[E0433]: cannot find `text` in `snapshot`
- 2 × error[E0433]: cannot find `binary` in `snapshot`
- 2 × error[E0425]: cannot find function `read_tsv_source_binary` in this scope
- 1 × error[E0425]: cannot find function `read_tsv_source_text` in this scope
- 1 × error[E0433]: cannot find `sqlite` in `snapshot`

```text
error[E0433]: cannot find `text` in `snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📦️packages/🦀️rust/../../🦀️.rs:109:33
    |
109 |         grammar: Some(snapshot::text::COMPONENT_GRAMMAR_SEMIO),
    |                                 ^^^^ could not find `text` in `snapshot`

error[E0433]: cannot find `text` in `snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📦️packages/🦀️rust/../../🦀️.rs:110:38
    |
```

```text
error[E0433]: cannot find `text` in `snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📦️packages/🦀️rust/../../🦀️.rs:110:38
    |
110 |         grammar_path: Some(snapshot::text::COMPONENT_GRAMMAR_PATH),
    |                                      ^^^^ could not find `text` in `snapshot`

error[E0433]: cannot find `binary` in `snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📦️packages/🦀️rust/../../🦀️.rs:111:34
    |
```

```text
error[E0433]: cannot find `binary` in `snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📦️packages/🦀️rust/../../🦀️.rs:111:34
    |
111 |         protocol: Some(snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
    |                                  ^^^^^^ could not find `binary` in `snapshot`

error[E0433]: cannot find `binary` in `snapshot`
   --> 🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📦️packages/🦀️rust/../../🦀️.rs:112:39
    |
```

