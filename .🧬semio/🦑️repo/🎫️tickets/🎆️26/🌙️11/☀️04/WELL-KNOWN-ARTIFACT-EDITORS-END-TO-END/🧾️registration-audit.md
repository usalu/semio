# 🧾️ Registration Audit (T19 haiku-registration)

Scope: stdio artifact `📜️artifact-definition.json` files under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/*`, compared against the stdio registry, the composition native-codec catalog (`🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📜️native-codec-factories.json`), the generated trusted catalog (`✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/trusted-stdio-catalog.json`), the definition roster fixture, the root collection `🗿️artifacts/🔣️.json`, and fixture references.

Scripts (input, kept): `audit-receipts.py` in this ticket folder.

## Counts

- Artifact directories under `🗿️artifacts`: 39 (38 with a definition, plus `📇️inventory`, which has no definition and is a TypeScript package only; not an artifact).
- Definitions: 38. Native codecs: 31 codec entries across 30 artifacts (`pdf` has two). Native receipts: 30.
- Generated trusted catalog `nativeCodecs`: 30, identical to the receipts by factory id, artifact kind, schema, pack hash, extension and protocol sha256.
- Protocol files referenced by receipts: all exist; `protocol_source_sha256` matches the file bytes for all 30.

## Checks performed

1. Definition `id` equals `s.stdio.<artifact>`: all pass.
2. Definition `directory` equals the real directory name: 1 failure (json), fixed.
3. Dependencies resolve to existing definition ids: all pass.
4. MIME values lowercase, extensions dot-prefixed and lowercase: all pass.
5. `native_factory` internal consistency (`artifact_kind` = def id, `artifact_schema` = `stdio.<artifact>`, `factory_id` = `stdio.native.<artifact>.v1`, `runtime_capability_id` present in the same definition's `runtime_capabilities`): pass, except the items under "Ambiguities".
6. Native factory `extension` present in the representation's extension list: 4 failures (see Ambiguities A1).
7. Receipts vs definitions field by field (factory_id, artifact_kind, artifact_schema, extension, pack_schema_hash, runtime_capability_id, descriptor_codec_id, definition_path): match for all 30 receipts.
8. Receipts vs generated trusted catalog (factory id keyed): match for all 30.
9. Root collection `🗿️artifacts/🔣️.json` vs definitions: all 37 members exist and their directories match; `📇️idf` is missing from the collection.
10. Definition roster fixture `🧪️tests/🧫️fixtures/📜️definition-roster/🔣️.json` vs definitions: `idf` and `idml` are absent (see A3).
11. Fixture references: repo-rooted paths in stdio/hub/catalog sources checked for existence (2456 rooted strings, most are glob or template fragments). Real dangling references are doc-comment only (see F-list below). No definition-level fixture ids are dangling (`conformance_suites.fixtures` contains only `s.stdio.idml.fixture.package-events`, which is an id, not a path).

## Changes made

| File | Change | Reason |
| --- | --- | --- |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/📜️artifact-definition.json` | `"directory": "🔣️json"` changed to `"directory": "🧾️json"` | Definition directory did not match the real directory; the registry assembly uses `directory` as `dir_name` in `FormatDescriptor` (`📇️registry/🧬️contract/🏗️assembly/🦀️.rs`), so the wrong value would mis-address the json artifact folder. Receipts, root collection and generated catalog all already use `🧾️json`. Validated with `python3 -m json.tool`. |

No status fields were changed. No registrations were removed. No generated files were edited.

## Ambiguities for sonnet-registry (not changed)

A1. Native factory `extension` is `semio` for avi, bcf, mp3, mp4 (`extension: "semio"`), while each representation declares `.avi`, `.bcf`, `.mp3`, `.mp4`. The same value is present in the definition, the receipt, the generated catalog and the runtime capability ids (`...codec-stdio-avi-extension-semio.v1`). Changing it changes the runtime capability id and likely `pack_schema_hash`, so it is not mechanical. Decide whether `semio` is an intentional pack-container extension or a copy-paste placeholder.

A2. `pdf` second codec (1.7) uses `artifact_schema: "stdio.pdf.1.7"` and `factory_id: "stdio.native.pdf17.v1"`, while the 1.4 codec uses `stdio.pdf` and `stdio.native.pdf.v1`. Both share `artifact_kind: s.stdio.pdf`. Decide whether the per-revision schema name is intended. Changing it changes the pack hash binding.

A3. `idml` has an implemented native codec (`s.stdio.idml.standard.8-0.codec.native-document.v1`) and a definition, and is in the root collection, but has no native receipt, no entry in `native-codec-factories.json`, no entry in the generated `trusted-stdio-catalog.json` `nativeCodecs`, and no entry in the definition roster fixture. `idf` is in neither the root collection nor the roster fixture and has no codec. Both receipts and roster are generated, so this needs the generator, not a hand edit.

A4. `idf` has `dependencies: []`. Its sibling text-based artifact `epw` depends on `s.stdio.txt`. IDF is a text format, so `s.stdio.txt` is the likely dependency, but the owner should confirm.

A5. Definition-roster note vs fixture: the fixture note says definition-only artifacts "publish no native codec". `definitionOnly` lists only `ifc` and `semio`, while `epw`, `gif`, `bmp`, `wav` and `binary` also publish no native codec yet are not in `definitionOnly`. These have their own runtime crates in the full roster, so this may be fine, but the note and the list disagree.

A6. Missing extension aliases (requested example). Not applied, because the semantics are not settled:
- `representations[].aliases` is consumed as additional format lookup keys next to `kind_id` and `short_id` (`format_descriptor_keys` in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/📥️lifted/🚪️io/🦀️.rs`). Its values are dot-less spellings in that context (`short_id` is the extension without its dot). Dotted values such as `.jpeg` would not match a short-id lookup.
- Adding the alias to `extensions` instead would change the export default. Format normalization sorts `extensions` (`row.extensions.sort()`) and workflow code uses `extensions.first()` as the export extension (`🖥️host/🔀️workflow/🦀️.rs`). `.jpeg` sorts before `.jpg`, `.htm` before `.html`, `.tif` before `.tiff`, so jpg, html and tiff would export with the alias extension. Adding claims also changes the `representation:mime|extension` runtime capabilities.
- Candidate dot-less aliases if the owner chooses the alias route: `jpg` → `jpeg`; `tiff` → `tif`; `html` → `htm`; `md` → `markdown`; `step` → `stp`. Empty or duplicate-of-short-id values are rejected by the validator.

A7. `deflate` declares mime `application/zlib` with extension `.zz`, and the native factory uses `zz`. `.zz` is not a standard zlib extension (`.z` is the common one). Changing it changes the pack hash and runtime capability ids, so it is left as is.

A8. `gif` (87a, 89a), `pdf` (1.4, 1.7), `dwg` (ac1018, ac1024) and `ifc` (2x3, 4, 4x3) declare multiple representations with the same mime and extension. They have distinct representation ids and standards, so this is intended per-revision modelling, not duplication. Recorded only so the registry owner knows the format lookup will receive identical mimes from several rows (the registry rejects duplicate mime/extension claims across different kind ids; check that the intra-artifact case is accepted).

## Fixture reference findings (not changed)

F1. Doc-comment references to fixtures that do not exist (documentation drift only, no test loads them):
- `🖼️tiff/🧫️fixtures/🖼️abbau-aufbau-masterarbeit-grundriss.tiff` (referenced from the jpg/tiff area; actual fixture lives under `🖼️tiff/🏅️standards/🔖️6.0/.../🧪️tests/🧫️fixtures/🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff`).
- `📽️pptx/🧫️fixtures/📽️.pptx` (doc comment in `🧿️semio/.../📽️mutate-semio-presentation/🦀️.rs`).
- `🌦️epw/📚️examples/🎬️demo/🖼️assets/🌦️example.epw` (doc comment in epw `🔬️unit/🦀️.rs`).
- `📑️tsv/📚️examples/🎬️demo/🖼️assets/📊️.tsv`, `📊️csv/📚️examples/🎬️demo/🖼️assets` (examples dir may be absent).
- `🖋️dxf/.../🏭️generator/🧫️fixtures/🦀️.rs`, `📐️step/.../⚙️generator/📜️script.ts` referenced by hub fixture comments (generator paths not present).
- `target/release/reader` binaries referenced from gif/gltf/pdf probes are build outputs, not checked in. Ignore.

F2. `🖊️dwg-artifact-ownership` hub fixture references `🖊️dwg/🏅️standards/4️⃣ac1018/🧬️schema/🦀️.rs` as an allowed path. That fixture is a path-policy case list (virtual paths), and the file is not expected to exist. Not dangling in the sense of a file reference; left as is.

F3. Per-standard fixtures under `📸️jpg` (`🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg`) exist in both the `🧱️baseline` and `🧾️document` subsets. The `shared://` references in the jpg baseline tests refer to a shared fixture scheme; not verified as resolvable at runtime.

## Dependencies / notes for other workers

- The receipts `native-codec-factories.json` and the generated `trusted-stdio-catalog.json` are generator outputs. Adding `idml` (A3) or fixing A1/A2 requires re-running their generator (`🧩️catalog` and `🌎️hub/.../🔌️plugin/📇️catalog` builders), which is a heavy build and was not run here.
- The T19 change (json directory) does not change any receipt, hash or generated output.

## Addendum: stale doc-comment fixture references (F1) fixed

| File | Change | Evidence |
| --- | --- | --- |
| `🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs` (doc, line 46) | `🌦️epw/📚️examples/🎬️demo/🖼️assets/🌦️example.epw` replaced by `🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🖼️assets/🎬️demo/🧪️example/🌦️.epw` | Only `.epw` file in the artifact; the sibling sqlite test loads the same file from that path; 24 data rows after 8 header lines match the comment. |
| `🧿️semio/.../🧊️brep/🏭️generator/📜️script.ts` (line 26) and `🧿️semio/.../🔺️mesh/🏭️generator/📜️script.ts` (line 27) | `6️⃣cc6/⚙️generator/📜️script.ts` replaced by `6️⃣cc6/🏭️generator/📜️script.ts` | The step cc6 generator directory is `🏭️generator`; `⚙️generator` does not exist. |

Left unchanged (ambiguous or generator output):
- `🖼️tiff/.../🔮️oracles/🧪️tests/🔬️fixture-derivation/🦀️.rs` writes its derived output to `🖼️tiff/🧫️fixtures/🖼️abbau-aufbau-masterarbeit-grundriss.tiff`. That is an output path of an ignored derivation test, not a reference to an existing file. Changing it changes generator behaviour; the existing copies are `🖼️tiff/.../🧾️document/🧪️tests/🧫️fixtures/🧪️abbau-aufbau-masterarbeit-grundriss/🖼️.tiff` and the `🧱️baseline` one.
- `🧿️semio/.../📽️mutate-semio-presentation/🦀️.rs` cites `📽️pptx/🧫️fixtures/📽️.pptx`. Two byte-identical copies exist (`🔒️strict` and `🌉️transitional` tests/fixtures), so the canonical one is ambiguous.
- `🖋️dxf/.../📰️header/🏭️generator/🧫️fixtures/🦀️.rs` is cited by a hub fixture and does not exist; no unambiguous replacement.
- `📚️examples/🎬️demo/🖼️assets/📊️.tsv` (tsv) and `📊️csv` assets are not dangling; they exist under their artifact subset folders (the earlier check used the wrong root).

## Addendum: fixture manifest for T06

- File: `/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️04/WELL-KNOWN-ARTIFACT-EDITORS-END-TO-END/🧫️fixture-manifest.json` (valid JSON; built by `build-fixture-manifest.py` in this ticket folder).
- 38 artifacts. Each entry has dialect coordinates (`artifactKind/standard/subset`, derived from the `🏅️standards/*/🪆️subsets/*` folders with emoji prefixes stripped; idf and idml have no folders, so subset is null), editor and viewer surface ids (from `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/*/🧩️component/🔣️.json`, the stdio composition component manifest, and the trusted catalog `openTargets`), mimes, extensions, native codec id, 1 to 3 valid sample paths on disk, and invalid samples where present.
- Sample paths are repo-relative and were verified to exist. Invalid samples are files whose basename says invalid, malformed, truncated, corrupt, broken, damaged, or `reject-dib` (bmp). Operation-rejection fixtures (`…-rejected-…/⬅️before.*`) are valid inputs and are not counted as invalid.
- Lacking a valid fixture: `idml` (no sample in its artifact folder, `♻️mit-bestand`, or anywhere else in the repo).
- Lacking a surface: `idf` and `idml` (no editor or viewer in any extension manifest). `idf` has a valid sample from the energy plugin (`🔋️energy/…/📤️export/⚡️idf/🧪️tests/1ZoneUncontrolled.idf`), marked `repo-external`.
- Artifacts lacking a valid fixture or a surface: 2 (`idf`, `idml`).
- Notes for T06: the json editor surface `stdio.json.editor` comes from the trusted catalog and does not follow the `s.stdio.<kind>@…#editor` pattern. `semio` has 19 editor and viewer ids. Some valid samples come from `🧪️tests` or `🧫️fixtures` folders that are mutation before/after pairs, not standalone documents.

## Addendum: details-split default layout and generated invalid fixtures

Details split (script `apply-details-split.py`): `.default_layout(edit::layout())` replaced by `.default_layout(semio_s_artifact_stdio_contract::editing::snapshot_details_split_layout(main::WINDOW_KIND_ID, "Main"))` in 29 editor files (17 artifacts): mp3, las, gif 87a and 89a, mp4, svg base/basic/tiny, avi, jpg baseline/document, bmp, tiff baseline/document, stl, ply, dxf, dwg ac1018/ac1024, obj, gltf, step base and cc1 to cc6, ifc 4. None was already changed. Each file's `main` window module defines `WINDOW_KIND_ID`. ifc 4x3 uses a `layout` variable and was not in the list, so it was left alone.

Generated invalid fixtures (script `generate-invalid-fixtures.py`): 34 artifacts with a valid sample in their own artifact folder. Each gets `✂️truncated/<name>` (first half of the bytes), and binary artifacts also get `🪄️wrong-magic/<name>` (first 4 bytes replaced with `ff fe fd fc`). That is 54 files. Folders are the sample's own `🧫️fixtures` folder, or the subset's `🧪️tests/🧫️fixtures` for the samples under `📚️examples` (binary, dwg, epw). Max path length 230 bytes. idf (sample outside stdio) and idml (no sample) are not covered, so they still lack an invalid fixture.

## Addendum: enumerator risk review for generated invalid fixtures

Scope: stdio artifacts, hub stdio composition, and OS plugin tests. Enumerators found (`read_dir`, `readdirSync`, `walk`, `glob`, `Bun.Glob`, `include_dir`) were checked for whether they would pick up `✂️truncated/` or `🪄️wrong-magic/` files as valid.

Breaks, fixed:
- IFC 2x3 test `committed_ifc2x3_fixtures_read_through_the_canonical_part21_codec` (`🏗️ifc/🏅️standards/🔖️2x3/🚪️io/🧪️tests/🔬️unit/🦀️.rs`) walks every `.ifc` under `🔖️2x3` and asserts it parses as valid Part21. The truncated IFC file would fail. Fix: moved to `🏗️ifc/🧪️tests/🧫️fixtures/✂️truncated/complex-item-truncated.ifc`, outside the walked tree. The old folder was removed.
- T06 acceptance matrix `candidateSamples` (`🧪️tests/🎯️artifact-acceptance/📇️matrix/🟦️.ts`) walks the whole artifact tree and takes every file with the row extension whose basename does not match `invalid|truncated|corrupt|malformed|broken`. Folder names are not checked, so the generated files were candidates. A directory relocation cannot avoid this walker (it covers the whole artifact tree), so all 54 generated files were renamed: `<stem>-truncated<ext>` and `<stem>-invalid-magic<ext>`. The folder names are kept. Recommended for the T06 owner: also check path segments (`✂️truncated`, `🪄️wrong-magic`).

Checked and safe (no change):
- Step `🎛️history-inputs` readers (`🧪️tests/🟦️.ts`, `🔮️oracles/🧪️tests/🔬️unit/🦀️.rs`, `🧫️fixtures/🎛️history-inputs` per subset): they read only the `🎛️history-inputs` folder, which the generated files do not touch.
- GLTF `fixture-corpus` (ts and rs): recurse for directories holding `🦠️mutation/🔣️.json`. The generated folders have no such file.
- GIF generators (`🏭️generator/🟦️.ts`, 87a and 89a): `readdirSync(seedDir).find(name => existsSync(seedDir/name/BEFORE_FILE))`. The generated folders have no `BEFORE_FILE` child, so they are not selected.
- DWG ac1024 metadata test: reads `.rs` files only.
- Semio brep `rustSources` and probes `registerSchemas`: `.rs` only, or skip `🧫️fixtures`.
- Semio conversions `named()`: skips `🧪️tests`.
- Hub stdio text walk (`stdioWalkText`) and the hub scripts: scan text for forbidden or artifact-directory references. They do not parse fixtures. The generated text copies (json, txt, md, xml, svg, html, csv, tsv, step, stl, obj, ply, dxf, ifc, semio) repeat content from the originals, so they add no new references. Low risk, not changed.
- OS plugin tests and modules: the enumerators that touch fixtures (`directory-client-http`, `backbone-uris-and-workflow`, `shell` json fixtures, `history-edit-acceptance`, `fixture-sweep` m5 and pilot-resolve, `store/sync load_fixtures`) walk non-stdio fixture trees or repo-wide text scanners. None reads the stdio artifact fixtures as valid corpora. Not changed.

Manifest: `🧫️fixture-manifest.json` `invalidSamples` updated to the new paths (all 58 entries exist, max 230 bytes). `relocate-invalid-fixtures.py` performed the move and rename. The earlier `generate-invalid-fixtures.py` is superseded for the current layout.

## Addendum: single reproducible builder and /tmp hygiene

- `build-invalid-fixtures.py` replaces `generate-invalid-fixtures.py` and `relocate-invalid-fixtures.py`, which are deleted. It writes the current layout directly: truncated files go to `<fixtures>/✂️truncated/<stem>-truncated<ext>`, wrong-magic files to `<fixtures>/🪄️wrong-magic/<stem>-invalid-magic<ext>`, and IFC truncated files to `🏗️ifc/🧪️tests/🧫️fixtures/✂️truncated/` (outside the 2x3 walker root). It removes its previous outputs first, asserts walker safety and the 256-byte limit, and updates the manifest. A rerun on the current tree reproduces the same 54 files (same content hashes, same names).
- `/tmp` hygiene. Files modified today under `/tmp` that are named after workers or tasks:
  - `/tmp/b08-csv.log` and `/tmp/b08-idml.log`: 0 bytes. Still open: the write ends of live `tail -30` (pid 813431) and `head -80` (pid 820035) pipes. Copied as snapshots to `🗑️generated/` with the same names. Not deleted.
  - `/tmp/t13/wav-codec.log`: the writer removed it, and `/tmp/t13/` is now empty. Not deleted. A copy named `🗑️generated/t13-wav-codec.log` was already present (written by another worker). Not overwritten.
  - `/tmp/t04_files.txt` and `/tmp/t20_stdio_cargo.txt`: no open handles. Copied to `🗑️generated/` with the same names.
  - Listed, not copied (not clearly worker logs): `/tmp/align-en1990.py`, `/tmp/align-en1990-changed.txt` (task-named, probably en1990 alignment work), `/tmp/en1992.err` through `/tmp/en1999.err` (tiny batch stderr files), `/tmp/euro_dirs.txt`, `/tmp/gather.py`, `/tmp/samples.py`, `/tmp/scan_euro_fields.py`, `/tmp/dbg.ts`.
  - Not task-named, ignored: `/tmp/cursor-agent-logs-1000/`, the `semio-broker-*.sock` sockets, and `/tmp/claude-1000/`.
  - The `check44.sh` to `check74.sh` scripts are dated Nov 1, so they are not from today.

## Addendum: unmembered bridge crates

- `semio-s-artifact-energy-model-snapshot-bridge` (energy snapshot `🧬️original/🌉️bridge`): referenced by `workspace.dependencies` in `🔋️model/Cargo.toml` and used by the snapshot carrier crate. Its manifest declares the energy model workspace as its root, but it was not listed in `members` and not excluded. Fix applied: added `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧬️original/🌉️bridge/📦️packages/🦀️rust` to `members` in `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml`. Verified: the TOML parses and every member directory exists. `cargo metadata` could not be run: the one gate run was cancelled while waiting for host capacity, and it was not retried, per the coordinator's once-only instruction.
- `semio-semio-v1-base-bridge` (`stdio/🧿️semio/…/✉️base/🏭️bridge`): deliberately excluded. The stdio workspace excludes `**/🏭️bridge/**`, and the crate's manifest declares its own `[workspace]` root with a comment saying all bridges and probes are standalone. Only the stdio `📜️script.ts` references it. Not changed. It is not dead: the process script calls it.

## Addendum: shell locale fallbacks replaced with localizedLabelTextV1

Each fallback that chose English for an unknown tag now calls `localizedLabelTextV1` from `🧱️elements/🏛️ShellHost/📣️replay-refusal/🟦️.ts`. An unknown tag renders every language joined by ` | `.
- `🏛️ShellHost/⌨️window-scope/🟦️.ts`: `keybindingUnownedTextV1`.
- `🏛️ShellHost/🎯️input-ledger/🟦️.ts`: `inputRefusalNoticeTextV1`. `inputIdentitySettleNoticeTextV1` moved its inline en/de pair into `INPUT_IDENTITY_SETTLE_LABEL_V1`.
- `🏛️ShellHost/🪟️spawned-program/🟦️.ts`: `spawnProgramRefusalNoticeTextV1` (`{program}` replaced in every joined language).
- `🎯️targets/⚛️react/🔗️hub-connection-and-spaces/🟦️.ts`: `spacesUiLabel`. The table `SPACES_UI_LABELS` is now exported. Its default `locale = "en"` is left as is.
- Note: `hub-connection-and-spaces` is under `🎯️targets/⚛️react/`, not under `elements/`.
- Tests updated: `🧪️tests/🎯️input-ledger`, `🧪️tests/🪟️spawned-program-session`, `🧪️tests/⌨️window-scope`, `🧪️tests/🔁️refresh-coalescing`. Each now iterates every reason or key. The fixture row `unownedHint` `fr` in `🧱️elements/🏛️ShellHost/🧪️tests/🧫️fixtures/⌨️window-scope/🔣️.json` now expects the joined text.
- `localizedLabelTextV1` was changed by another worker while this was in progress: it now lowercases the tag and uses its primary subtag, so `DE` and `de-DE` render German. The tests follow this contract.
- Verified with a ticket-local config (`vitest.shell-labels.config.mjs`, since the shared host config only includes its own list): 4 files, 151 tests, all pass.
- Bridge crate fix (above) is unaffected by these changes.
