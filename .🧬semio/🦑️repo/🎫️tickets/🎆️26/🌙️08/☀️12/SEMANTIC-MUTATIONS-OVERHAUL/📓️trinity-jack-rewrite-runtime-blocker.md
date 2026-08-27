# Trinity JACK and Rewrite Runtime Blocker

## Command

```text
bun nx run @semio-tech/trinity-plugin:test-quick
exit 1
```

The single combined run started the registered Trinity `test-quick` target and failed while compiling the `semio-s-plugin-stdio` dependency, before JACK or Rewrite tests could execute. Cargo reported `116 previous errors; 378 warnings emitted`. No emitted diagnostic named a JACK or Rewrite path or symbol.

## Captured Compiler Output

```text
error[E0432]: unresolved import `crate::artifacts::gltf::schema::mutations::top_level_private`
error[E0425]: cannot find function `gltf_mutation_leaf_descriptors` in module `crate::artifacts::gltf::schema::mutations`
error[E0425]: cannot find value `COMPONENT_GRAMMAR_SEMIO` in module `crate::artifacts::txt::schema::mutations::text`
error[E0425]: cannot find value `COMPONENT_GRAMMAR_PATH` in module `crate::artifacts::txt::schema::mutations::text`
error[E0425]: cannot find value `COMPONENT_PROTOCOL_SEMIO` in module `crate::artifacts::txt::schema::mutations::binary`
error[E0425]: cannot find value `COMPONENT_PROTOCOL_PATH` in module `crate::artifacts::txt::schema::mutations::binary`
error[E0425]: cannot find value `COMPONENT_GRAMMAR_SEMIO` in module `crate::artifacts::json::schema::mutations::text`
error[E0425]: cannot find value `COMPONENT_GRAMMAR_PATH` in module `crate::artifacts::json::schema::mutations::text`
error[E0425]: cannot find value `COMPONENT_PROTOCOL_SEMIO` in module `crate::artifacts::json::schema::mutations::binary`
error[E0425]: cannot find value `COMPONENT_PROTOCOL_PATH` in module `crate::artifacts::json::schema::mutations::binary`
error[E0425]: cannot find value `COMPONENT_GRAMMAR_SEMIO` in module `crate::artifacts::xml::schema::mutations::text`
error[E0425]: cannot find value `COMPONENT_GRAMMAR_PATH` in module `crate::artifacts::xml::schema::mutations::text`
error[E0425]: cannot find value `COMPONENT_PROTOCOL_SEMIO` in module `crate::artifacts::xml::schema::mutations::binary`
error[E0425]: cannot find value `COMPONENT_PROTOCOL_PATH` in module `crate::artifacts::xml::schema::mutations::binary`
error[E0425]: cannot find function `dec_xml_snapshot` in module `crate::artifacts::xml::schema::mutations`
error[E0425]: cannot find function `enc_xml_snapshot` in module `crate::artifacts::xml::schema::mutations`
error[E0425]: cannot find function `enc_xml_snapshot_bin` in module `crate::artifacts::xml::schema::mutations`
error[E0425]: cannot find function `dec_xml_snapshot_bin` in module `crate::artifacts::xml::schema::mutations`
error[E0425]: cannot find type `GltfJson` in this scope
error[E0425]: cannot find type `GltfMutation` in module `super`
error[E0433]: cannot find `GltfMutation` in `super`
error[E0425]: cannot find value `COMPONENT_GRAMMAR_SEMIO` in module `crate::artifacts::svg::standards::v1_1::subsets::any::schema::mutations::text`
error[E0425]: cannot find value `COMPONENT_GRAMMAR_PATH` in module `crate::artifacts::svg::standards::v1_1::subsets::any::schema::mutations::text`
error[E0425]: cannot find value `COMPONENT_PROTOCOL_SEMIO` in module `crate::artifacts::svg::standards::v1_1::subsets::any::schema::mutations::binary`
error[E0425]: cannot find value `COMPONENT_PROTOCOL_PATH` in module `crate::artifacts::svg::standards::v1_1::subsets::any::schema::mutations::binary`
error[E0425]: cannot find function `dec_svg_snapshot` in module `crate::artifacts::svg::schema::mutations`
error[E0425]: cannot find function `enc_svg_snapshot` in module `crate::artifacts::svg::schema::mutations`
error[E0425]: cannot find function `enc_svg_snapshot_bin` in module `crate::artifacts::svg::schema::mutations`
error[E0425]: cannot find function `dec_svg_snapshot_bin` in module `crate::artifacts::svg::schema::mutations`
error[E0559]: variant `PdfMutation::AppendPageContent` has no field named `index`
error[E0559]: variant `PdfMutation::AppendPageContent` has no field named `text`
error[E0599]: no variant named `SetSnapshot` found for enum `TxtMutation`
error[E0559]: variant `JsonMutation::SetScalar` has no field named `path`
error[E0559]: variant `JsonMutation::SetScalar` has no field named `value`
error[E0559]: variant `XmlMutation::SetText` has no field named `path`
error[E0559]: variant `XmlMutation::SetText` has no field named `text`
error[E0599]: no variant named `SetSnapshot` found for enum `SvgMutation`
error[E0559]: variant `PdfMutation::InsertPage` has no field named `index`
error[E0559]: variant `PdfMutation::InsertPage` has no field named `page`
error[E0559]: variant `PdfMutation::SetInfo` has no field named `info`
error[E0599]: no variant, associated function, or constant named `NoMutation` found for enum `JsonMutation`
error[E0599]: no variant named `SetSnapshot` found for enum `JsonMutation`
error[E0559]: variant `JsonMutation::SetMember` has no field named `path`
error[E0559]: variant `JsonMutation::SetMember` has no field named `key`
error[E0559]: variant `JsonMutation::SetMember` has no field named `value`
error[E0559]: variant `JsonMutation::RemoveMember` has no field named `path`
error[E0559]: variant `JsonMutation::RemoveMember` has no field named `key`
error[E0559]: variant `JsonMutation::InsertArrayElement` has no field named `path`
error[E0559]: variant `JsonMutation::InsertArrayElement` has no field named `index`
error[E0559]: variant `JsonMutation::InsertArrayElement` has no field named `value`
error[E0559]: variant `JsonMutation::RemoveArrayElement` has no field named `path`
error[E0559]: variant `JsonMutation::RemoveArrayElement` has no field named `index`
error[E0599]: no variant, associated function, or constant named `F32` found for enum `GltfComponentType`
error[E0277]: the trait bound `GltfCameraProjection: serde::Serialize` is not satisfied
error[E0277]: the trait bound `GltfCameraProjection: serde::Deserialize<'de>` is not satisfied
error: could not compile `semio-s-plugin-stdio` (lib) due to 116 previous errors; 378 warnings emitted
Warning: command "bun ./📜️script.ts test quick" exited with non-zero status code
NX Running target test-quick for project @semio-tech/trinity-plugin failed
```

Repeated instances of the same field-shape errors were emitted from multiple STDIO consumers; the transcript above preserves each distinct diagnostic class and the terminal failure summary.

## First Actionable Owner Paths

### glTF, TXT, JSON, XML, and SVG

- glTF stale private-module import: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/🧬️mutation-support/🧱️structure-geometry/🦀️component.rs:2`.
- glTF stale component-type variant: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️🔺️bind-primitive-indices/🦀️component.rs:12`.
- TXT missing mutation codec constants are consumed at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🦀️component.rs:103`; stale `SetSnapshot` construction begins at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🧪️tests/mutate-txt-utf-8/🦀️component.rs:167`.
- JSON missing mutation codec constants are consumed at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🦀️component.rs:106`; stale enum constructors begin at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🧪️tests/mutate-json-rfc8259/🦀️component.rs:212`.
- XML missing mutation codec constants are consumed at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🦀️component.rs:112`; stale snapshot codec calls are at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:729`; stale `SetText` construction begins at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰xml/🧪️tests/mutate-xml-1-0/🦀️component.rs:291`.
- SVG missing mutation codec constants are consumed at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🦀️component.rs:124`; stale snapshot codec calls are at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:1332`; stale `SetSnapshot` construction begins at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🧪️tests/mutate-svg-1-1/🦀️component.rs:206`.

### PDF

- The first stale struct-style `PdfMutation` constructors are at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🧪️tests/mutate-pdf-1-7/🦀️component.rs:219`, with additional `AppendPageContent` and `SetInfo` instances at lines 223–224.
- Production stale constructors occur at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/✏️editor/🦀️component.rs:139`, `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🚪️io/🦀️component.rs:3022`, and `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/🧬️schema/🦀️component.rs:116`.

## Registered Target Check

`bun nx show project @semio-tech/trinity-plugin --json` exited `0` and showed only `test`, `test-quick`, `test-long`, and `test-exhaustive`; every target routes the same `semio-s-plugin-trinity` crate, whose manifest directly depends on `semio-s-plugin-stdio`. There is no narrower registered Nx target that can bypass this compilation blocker.
