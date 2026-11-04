# 📗️ Recipe: Natural-File Codec and Native Receipt (T15b, static)

A = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`, H = `🌎️hub/🧩️compositions/🗄️stdio/`, F = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/`, S = `…/📺️renderer/🧑‍🎨engine/🧱️elements/`.

## Natural-File Codec (Hand-Written)

1. In the subset `✏️editor/🦀️.rs`, override `natural_file_codec`, `encode_natural_file` and `decode_natural_file`. References:
   - txt: `A/🔤️txt/…/✳️any/✏️editor/🦀️.rs:338-349`
   - csv: `A/📊️csv/…/✏️editor/🦀️.rs:514-522`
   - png (binary): `A/📷️png/…/✏️editor/🦀️.rs:234-242`

   `format_kind` must equal `"<kind>@<standard>"`, because the shell matcher compares `kindId === formatKind`.
2. The struct is `NaturalFileCodec` (`F/📜️declaration/🧩️app/🗂️natural-file-and-media-artifacts/🦀️.rs:5-12`). Actions and mounting are at the same file, lines 84-111. Mounting is automatic once the editor declares the codec.
3. On the shell side, `S/🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts:63-121` (`openNaturalFileOwnerV1`) handles opening.
4. For tests, add a row to `F/🧪️tests/🧫️fixtures/📄️natural-file-lifecycle/🔣️.json` and bump the count at `F/🧪️tests/📄️natural-file-lifecycle/🦀️.rs:59` (currently 28; remove tiff from `unsupported` when it is added). That test checks metadata only, so each batch must also add a real encode → decode → edit → encode round-trip test with a third-party cross-check.

## Native Receipt

1. `native_codec()` uses `store::artifact_codec` with an extension and `pack_schema_hash`. References: txt `A/🔤️txt/…/utf-8/🦀️.rs:43-48` (SHA of `📡️.protocol.semio`) and csv `…/✏️editor/🦀️.rs:732-737` (`pack::record::schema_hash`).
2. Declare `NativeCodecFactory { id, artifact, kind, codec }` (txt :72-74), export it in `contribution()` (txt :131-133), and add `document_codec_bare::<Snapshot, Mutation>` (txt :88).
3. Fill in the definition JSON `codecs[].native_factory`. Its fields are rewritten by the generator.
4. The bijection check `✏️s/🔌️plugins/🗄️stdio/🦀️.rs:25-44` requires factory = declaration = receipt.

## Generators (Run Through the T06 Heavy Queue)

1. `composition-prepare` (H project.json:597-610) regenerates the roster `H/🤖️generated/🧩️composition/🦀️.rs`, mutations and editor-laws.
2. `native-codec-projection` (H project.json:110-128) regenerates `📜️native-codec-factories.json`, `📇️publication/📜️native-catalog.json` and the definition `native_factory` fields.
3. Hand-edit the integrity pointers in `H/📇️publication/🔣️.json`, then publish the trusted catalog with `OS_HUB_DATA=<abs> os-hub trusted-catalog publish < command.json` (`H/…/README.md:299`). No project target exists for this step.
