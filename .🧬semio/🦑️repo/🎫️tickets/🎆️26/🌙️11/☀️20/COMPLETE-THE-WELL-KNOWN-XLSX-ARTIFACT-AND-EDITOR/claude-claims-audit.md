# Claude Claims Audit: artifact-definition.json codec and route claims

Scope: every `📜️artifact-definition.json` under `✏️s/🔌️plugins` (53 files, norm and stdio plugins).

## Result

- Files scanned: 53
- Files fixed: 0
- Length-frame mismatches (`<len>` != UTF-8 byte length of id): 0
- Entries with `s.` owner prefix in a codec-extension claim: 1 (`stdio/artifacts/ifc`, see below). Left unchanged on purpose.
- Cross-check test against runtime claims: none exists for all definitions (see below).

## Runtime form (source of truth)

- `✏️s/.../identity/🦀️.rs`, `ArtifactIdentityClaim::codec_extension`: `format!("{}:{schema}:{extension}", schema.len())`, so the id is the schema passed in, unprefixed unless the runtime schema itself carries `s.`.
- `composition_route`: `format!("{}:{from}{}:{into}", from.len(), into.len())`, two length-framed dialect coordinates; `parse_composition_route` enforces the frame.
- Runtime schema ids for stdio.semio codecs (`🧿️semio/🦀️.rs`): `stdio.semio` plus extension, e.g. `stdio.semio.animation` (21 bytes), matching `21:stdio.semio.animation:semio` in the definition.
- IFC 4x3 runtime document schema is literally `s.stdio.ifc@4x3/*` (`🏅️standards/🔖️4x3/.../✏️editor/🦀️.rs`, `IFC4X3_DOCUMENT_SCHEMA`).

## Per-file findings

| File | Entries checked | Before | After |
| --- | --- | --- | --- |
| `🗄️stdio/🗿️artifacts/🧿️semio/📜️artifact-definition.json` | 19 codec-extension claims | `21:stdio.semio.animation:semio` (already corrected, valid) | unchanged |
| `🗄️stdio/🗿️artifacts/🏗️ifc/📜️artifact-definition.json` | codec-extension `17:s.stdio.ifc@4x3/*:ifc` (line 176, descriptor line 173) and 5 composition-route claims (`17:s.stdio.ifc@4x3/*...`) | `s.` prefix, length frames valid | unchanged, flagged (matches runtime `IFC4X3_DOCUMENT_SCHEMA`) |

All other definitions: no codec-extension or composition-route claim with a length mismatch or `s.` prefix.

## Files (before -> after)

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚡️din18599/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏋️en1991/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏛️en1992/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🔩️en1993/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧩️en1994/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪨️en1996/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪶️en1999/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/⚡️idf/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📜️artifact-definition.json` - unchanged, valid
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/📜️artifact-definition.json` - unchanged, valid

## Note on the ifc 4x3 entries

The `s.` prefix on the 4x3 codec schema is not a stale prefix; the runtime emits it. Removing it would make the definition diverge from `IFC4X3_DOCUMENT_SCHEMA` and from `validate_native_codec_binding`. The `s.` rule should be confirmed for dialect-coordinate schemas (`s.stdio.ifc@4x3/*`, `s.raster.raster@1/*`, `s.cad.cad@1/*`) before any change. Plugins such as `flow`, `space` and `home` also pass `s.`-prefixed schemas to `codec_extension` at runtime, so the prefix is an accepted runtime form, not a definition error.

## Cross-check test

- Partial coverage only. `🗿️bundled-definitions/🦀️.rs` (under `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧪️tests/`) assembles every stdio definition via `definition_from_schema_with_executables`, and `validate_native_codec_binding` (`🏗️assembly/🦀️.rs`) compares native codec runtime claims with `ArtifactIdentityClaim::codec_extension`. Neither compares the JSON claims of every definition against runtime-produced claims, and the bundled test only reads `stdio/artifacts`, not `norm` or other plugins.
- Recommended location for a full cross-check: `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🧪️tests/🗿️bundled-definitions/🦀️.rs`, extended to (a) read the `norm` and other plugin artifact directories, and (b) for each `codec-extension` and `composition-route` JSON claim, assert it equals the value rebuilt by `ArtifactIdentityClaim::codec_extension` / `composition_route` from the runtime codec and route specs (the ones `ready-declaration-builder` derives in `document_codec_async`).
