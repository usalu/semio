# Semio Semantic Artifact, Full Import/Export Architecture, Ad-hoc Codec Extraction, MediaFormat Retirement

## Context

At the heart of semio are artifacts: apps interoperate by importing/exporting common artifacts. The stdio plugin (`✏️s/🔌️plugins/🗄️stdio`) today holds 28 *format* artifacts (step, gltf, png, pdf, docx, …), 31/31 standards codec-complete (1075 passing tests), a policy-enforced taxonomy, a dialect registry (`Dialect{artifact_kind, standard, subset}` + `io_dispatch`) and a WIT-based cross-plugin IoRouter. Missing — and delivered end to end by this plan:

1. **The inbuilt semio semantic artifact**: ONE artifact `semio` (standard `v1`) whose **subsets** are the semantic types — brep, mesh, model, object, document, cad, drawing, image, video, audio, animation, presentation, workflow — **each subset carrying its own schema** (user decision).
2. **7 new format artifacts**: mp4, avi, mp3, wav, epw, tsv, html (user decision: all).
3. **Full import/export lattice** semio↔formats: brep↔step; mesh↔gltf/stl/obj/ply/las; model↔ifc/bcf; object↔json/xml/csv; document↔docx/md/txt/pdf; cad↔dxf/dwg/step; drawing↔svg/dxf/pdf; image↔png/jpg/gif/bmp/tiff; video↔mp4/avi; audio↔mp3/wav; animation↔gltf/mp4/gif; presentation↔pptx; workflow↔json.
4. **Extraction of ~15k LOC ad-hoc codecs** from plugins into stdio (remodel MP4/AVI/H.264 + PNG/JPEG, cad's 2 extra STEP paths, animate FFmpeg/Typst/HTML, energy EPW, architect CSV/TSV, 7× svg/dwg pattern, fem/puzzle/norm degenerate leaves) and rewiring all plugins hub-and-spoke through semio subsets.
5. **Deletion of the deprecated framework MediaFormat/ArtifactCodec layer** (55 files / 346 lines footprint) incl. fixing `semio-framework-os-run` and verifying the cross-plugin IoRouter natively (user decision: everything).

Executed by a parallel-agent workforce (Workflow tool), waves with single-writer closers.

## Locked decisions (user-confirmed)

- Semio shape: **literal sketch** — one artifact `semio`, standard `v1`, schema-bearing subsets.
- New formats: **all 7** (mp4, avi, mp3, wav, epw, tsv, html).
- V7: **full MediaFormat deletion incl. os-run fix** + IoRouter e2e verification.

## Verified ground truth (bake into every agent prompt)

- Policy gate is `bun ./📜️script.ts policy` — **never** `verify`. Test gate: `cargo test -p semio-s-plugin-stdio --lib` (baseline 1075/0, monotonically growing).
- Catalog SSOT: `✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json` (the STATUS.md claim it moved to `🚪️io/📇️registry/` is stale — that dir doesn't exist; W0 re-confirms script.ts path resolution).
- Crate deps: 32 plugins already depend on `semio-s-plugin-stdio`; framework has **zero** dependency on stdio → **semio subset snapshot types ARE the neutral types, exported from stdio**. No new crate. Framework/OS speak dialect strings + `IoPayload` via `io_dispatch` only.
- os-run blocker today: 3× E0063 missing `topic_contributions` (`🔌️plugin/🦀️component.rs:5884,:6120`; `🔌️plugin/🖥️host/🦀️component.rs:816`) + `🔁️workflow` module unmounted in the os **kernel** glue (`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs`; the run crate aliases the kernel as `workflow`) + run-crate duplicate `artifact_pack_path`/`artifact_spr_path` and non-exhaustive `AppFrame` match. Iterate-until-green; poll `git status` first (file historically owned by another session).
- Law set is **8**: field_sweep, mutation_diff_law, inverse_law, absorb_law, between_roundtrip_law, codec_retention_law, op_text_binary_roundtrip_law, diff_codec_text_binary_roundtrip_law. Recipe: ticket `🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/🧬️schema-design.md`; state: `f6-final-summary.md`.
- dsl-derive gaps (f6 §4: generics E0107, tuples, nested fixed arrays, `record` hygiene) → **hand-roll all op/diff codecs** using bcf/docx `enc_named_triple`/`enc_indexed_triple` patterns; schema style bans tuples/nested arrays (named structs `SemioPoint3{x,y,z}`, `SemioRgba`…).
- Schema-per-subset is already the compliant shape: `policySchemaRepresentationBreaches` (📜️script.ts:7628) demands a full schema tree per subset; today's 181 outstanding breaches are the *delegating* subsets. W1 formalizes "schema-owning subset" (full tree required) vs "delegating subset" (exactly the rs+ts re-export pair) — clears the 181 and legitimizes semio with no special-casing.
- `MediaFormat` definition: `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs:816` (26 variants; `store::ArtifactCodec` document codec is a DIFFERENT thing — don't confuse in sweeps).
- Fixtures: real 43KB mp4 exists (`🧰️framework/🔨️modules/🖼️assets/🪧️logos/🎥️logo.mp4`); avi/mp3/wav/epw/tsv/html must be handcrafted in W0. All 28 existing artifacts have real `📚️examples/🎬️demo/🖼️assets` files.
- Gitignore trap: `🔖️<bare.digit>` dirs need the `!**/🔖️*/` negations — `git check-ignore -v` every new dir.
- Hot single-writer files: stdio `📦️glue.rs`, os kernel `📦️glue.rs`, framework `📦️glue.rs`, `📜️script.ts`, `🔣️taxonomy.json`, `📇️catalog.json`, `🔺️mesh/🦀️component.rs`, `🚪️io/🦀️component.rs`, os `🦀️component.rs`, `🏃️run/🦀️component.rs`. Closers only.

## Architecture

### The semio artifact

- Dir `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/`, kind `s.stdio.semio`, standard `🔖️v1`, subsets `✳️any` (envelope union — mandatory: vocabulary rule requires `"*"`; `SemioSnapshot` = tagged union of the 13) + 13 schema-owning subsets `✳️brep ✳️mesh ✳️model ✳️object ✳️document ✳️cad ✳️drawing ✳️image ✳️video ✳️audio ✳️animation ✳️presentation ✳️workflow`.
- Each subset = full unit identical in shape to step's `✳️any`: `🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations}` × 5 facet mirrors + `📝️text`(8 leaves)/`💾️binary`(6 leaves) grammars + mutation triad dirs; `🏗️builder`/`🧐️analyzer` (real impls); `🎹️composer` (ArtifactComposer + **mandatory** SubsetValidator that decodes payload as the subset's own snapshot + referential invariants; pdf `✳️a` composer is the copy template); `🚪️io` leaves.
- Engine at **standard level only**: `🔖️v1/⚙️engine/` with `🧮️geometry` (SemioPoint3/Transform/Rgba/Nurbs helpers) + `🧰️triples` (shared generic enc/dec_indexed_triple, enc/dec_named_triple) + required round-trip test.
- Schema descriptor ids `s.stdio.semio` + `s.stdio.semio.<subset>`; per-subset `register_document_codec` (duplicate-id panic keeps collisions loud).
- Subset snapshot cores (informing sources; spec-mandated cross-reuse only — model embeds brep/mesh snapshots; presentation mirrors document's block shape with own types):
  - **brep**: id-keyed vertices/edges/loops/faces/shells/solids + `BrepSurface`/`BrepCurve` enums (Plane/Cylinder/…/Nurbs) — from step `⚙️engine/🧱️brep` + StepSnapshot.
  - **mesh**: meshes→primitives{topology, positions/normals/uvs/colors, indices, material}, materials (PBR), textures{mime, bytes} — from gltf.
  - **model**: spatial tree (site/building/storey/space) + elements{class enum, placement, GeometryRef{Brep|Mesh|None}, psets} + relations — from ifc/4.
  - **object**: ordered, lexeme-preserving typed object graph (`SemioValue` incl. Ref) — from json.
  - **document**: styles + block tree (Paragraph/Heading/List/Table/Code/Quote/Image/PageBreak) + runs + images — from docx/md; replaces PageDoc/TextDoc.
  - **cad**: layers/blocks/entities (Line/Arc/Circle/Ellipse/Polyline/Text/Insert/Solid/Dimension) — from dxf/dwg + 📐️cad plugin.
  - **drawing**: canvas/styles/layers + recursive `DrawNode`{Path/Text/Group/Image} — from svg (SvgNodeDiff is the recursive-diff template); replaces DwgDrawing-as-neutral.
  - **image**: width/height/colorspace/bit-depth + frames{delay, rgba8 pixels} + icc/metadata — from png/gif; replaces RasterImage.
  - **video**: streams{kind, codec, dims, rate} + samples{pts, key, opaque bytes} — container-typed, payload-opaque (honest boundary).
  - **audio**: sample_rate/format + channels{f32 samples} + tags — wav-shaped.
  - **animation**: timelines→channels{target, interpolation, keyframes{t, AnimValue}} — from gltf animations.
  - **presentation**: masters/layouts/slides→shapes (TextBox/Picture/Table/Placeholder) + notes — from pptx.
  - **workflow**: id-keyed nodes{kind, params, position} + port-ref edges — from OS `🔁️workflow` WorkflowNode + 🌊️flow/🕸️dag.

### io architecture (unchanged convention, hub-and-spoke)

- Keep `🚪️io/{📥️import/🧩️deserializers,📤️export/🧵️serializers}/🗿️artifacts/<art>/🔖️<std>/✳️<subset>/` exactly. **No** importers/exporters rename, **no** extra representation level (the "deserializer kind" of the sketch is already the `<art>` segment: txt/binary/json/xml are roster artifacts).
- **All semio↔format codecs live under the semio artifact's io tree, both directions** (zero edits to the 28 format trees). Forward entries (writes semio@v1/<subset>, reads format dialects) + reverse entries (writes fmt, reads semio subset) give all four IoKeys via the existing two-row `register_composer_entries`.
- Leaves are real trait impls (`ArtifactSerializer`/`ArtifactDeserializer` with typed From/Into + const FROM/INTO dialects), erased via **new SDK helpers `deserializer_entry_of::<D>()` / `serializer_entry_of::<S>()`** next to `composer_entry_of` in `🔌️plugin/🦀️component.rs`; registered through the subset composer's `register()` into the single existing `IoKey→ComposerEntry` registry (subset validation keeps firing).
- **Domain plugins go hub-and-spoke**: each of the 54 domain artifacts replaces direct format leaves with 1–3 semio-subset leaves (cad↔semio/cad+brep+mesh, draw↔semio/drawing, raster↔semio/image, writer↔semio/document, …). Format reach = two-hop compose via **new framework helper `io_compose_via(hub, target, sources)`** in `🚪️io/🦀️component.rs` (max 2 hops, hub always a semio subset). Catalog owners keep curated import/export lists as capability statement + gain `"via": "semio/<subset>"`; new `policySemioHubCoverageBreaches` (owner has semio leaf ∧ semio has format leaf) replaces the per-owner format-leaf check for migrated owners. The ~546 direct domain leaf-dirs are deleted **after harvesting** their conversion logic into the semio-side codecs.

### New format artifacts (gif dir shape template; full recipe + 8 laws each)

| Artifact | Standard | Snapshot | Codec seed | Fixture |
|---|---|---|---|---|
| 🎥️mp4 | `isobmff` | ftyp + tracks{codec Avc{sps,pps}∣Other, flattened sample tables, samples} + unknown boxes typed-raw; H.264 baseline as engine accessor | remodel `⚙️engine/🎥️video` (5,163 LOC) moved wholesale, split `⚙️engine/{🎥️h264,📦️boxes}` | `🎥️logo.mp4` (43KB, real) |
| 📼️avi | `1.0` (RIFF) | avih + streams{strh, strf BitmapInfo∣WaveFormat∣Raw, chunks{fourcc,data,keyframe}} + idx1 | same remodel file (RIFF half) | handcraft (W0) |
| 🎵️mp3 | `mpeg1-layer3` | Option<Id3v2> + frames{typed header, payload typed-raw — no Huffman decode day 1, honest boundary} + id3v1 | hand-roll fresh (~200 LOC sync scan) | handcraft (W0) |
| 🔊️wav | `riff-pcm` | fmt typed + data Pcm16/Pcm8/Float32/Raw + other RIFF chunks verbatim; NO type sharing with avi | hand-roll fresh | handcraft (W0) |
| 🌦️epw | `energyplus` | all 8 header lines typed (location 10 fields, design conditions, ground temps, data periods) + records with **all 35 columns** (lossless — the energy seed reads 15 with silent defaults; rewrite full-fidelity) | energy `⚙️engine/site` parser as seed | handcraft 24-record day (W0) |
| 📑️tsv | `iana` | records + trailing_newline + line_ending; no quoting (IANA TSV); own types, not merged into csv | csv as template; architect's `write/parse_delimited` informs edges | handcraft (W0) |
| 🌐️html | `5` (WHATWG) | doctype + node tree {Element{attrs incl. valueless}, Text, Comment, RawText script/style}; void-element set in encoder; well-formed-only `✳️any` (honest boundary); own types — HTML is not XML | fresh; svg/xml structural template | handcraft (W0) |

Emojis finalized by the W1b closer against roster uniqueness.

### Catalog/registry/glue

- `📇️catalog.json`: +8 roster rows (semio: `dir 🧿️semio, mime application/vnd.semio, ext .semio, depends [all bridged formats]`; 7 formats with mimes/exts/deps), DAG mirrored (acyclic: nothing depends on semio), `counts.stdio_artifacts: 36`, semio owner row (machine-checked io coverage), **retire `neutral` field** (zero script.ts readers), recompute curated_io_pairs. Byte-identical JSON round-trip check before every edit.
- stdio `🦀️component.rs plugin()`: `+ artifacts::semio::engine::register()` (14 descriptors, codecs, all composer entries incl. reverse rows, 13 validators) + 7 format `engine::register()` + 8 `.artifact_kind(...)`.
- stdio `📦️glue.rs`: semio mount block + 7 format blocks — generated by ticket `generators/` tooling, mounted by the W1b closer only.
- `🔣️taxonomy.json`: no structural change (optional additive comment). `📜️script.ts`: the schema-owning/delegating generalization; vocabulary reason-string widening; field-sweep keyed per schema-owning subset; catalog count text; `policySemioHubCoverageBreaches`; shrink-only allowlist seeds with **programmatically computed keys** (path-normalization bug history).

## Extraction map (per plugin: moves / stays / deleted)

- **📸️remodel**: video engine → stdio mp4+avi (move, not copy); `🖼️images` engine (1,878 LOC PNG/JPEG) deleted → stdio engines; PLY/LAS/OBJ exporter impls deleted → build `{Ply,Las,Obj}Snapshot` + stdio encode. Photogrammetry/sfm/dense domain engines stay.
- **📐️cad**: delete TS STEP writer/reader (`🔨️modules/📐️geometry/🟦️component.ts:1418-1545`), brepjs STEP io surface (kernel geometry ops stay behind interface), byte-reinterpret STL/IFC placeholder exporters, 11 orphaned JSON schemas (`🧬️schema/🔣️json/`), `mesh_to_obj_text`. Engine MediaFormat sites → dialect entries; STEP import body seeds semio/brep↔step.
- **🎞️animate**: delete FFmpeg subprocess path (violates no-external-runtime rule) → mp4 engine builds `Mp4Snapshot` frame-by-frame (audio track passed through typed-raw); GIF sidecar → stdio gif + domain scaler; site emitter rewritten to build `HtmlSnapshot`+json; typst stays, isolated behind a `TextRenderer` trait emitting `SvgSnapshot`.
- **🔋️energy**: EPW parse → stdio epw import; keeps `WeatherRecord`/psychrometrics derived `From<&EpwSnapshot>`.
- **🏛️architect**: delimited-text codecs die → Csv/TsvSnapshot; `MergeStrategy`/upsert domain semantics stay.
- **svg/dwg pattern plugins** (recon confirms final roster of ~7: 🗒️note, 📏️layout, 🌍️gis, 🎥️shooting, 🌀️procedural, 🖨️raster, 🖍️draw + 🧩️puzzle app + 🪐️space handler): each swaps hand-rolled writers for semio/drawing (or image) leaves; 🖍️draw's ad-hoc SVG+DWG writer deleted outright; 🌀️procedural stubs deleted.
- **🏗️fem, 🧩️puzzle**: all 24 JsonCodec-under-format-name leaf trees deleted; honest `→ Stl/Obj/ZipSnapshot` builders only where a real mapping exists (no lying formats).
- **📕️norm**: 150 degenerate one-cell-CSV leaves deleted (real `CsvSnapshot` tables or nothing); 15 handcrafted codec copies → derive where it works, else ONE shared fn in norm's root.

## V7 deletion (serial, after all plugins migrated)

1. OS media registry rewrite: `registry_export_media`/`registry_import_media` (os `🦀️component.rs:3471-3606` + host dupes `:3584,:3680`) → `io_dispatch` (hard error, no fallback); delete legacy stringly handler maps + MediaFormat-typed wrappers; handlers keyed on MediaFormat (:2673-2735, :3355-3409) → dialect-string keys.
2. Format metadata: `stdio_format_entry`/`normalize_stdio_format_kind`/`stdio_accept_filter`/`stdio_mimes_csv` → io module `FormatDescriptor` APIs (:519-609); stdio registers descriptor rows from catalog.
3. `MediaWireFormat::Binary{format: MediaFormat}` → `Binary{format_kind: String}`; `ArtifactKindSpec.{export,import}_formats: Vec<MediaFormat>` → `Vec<String>`.
4. Delete 🔺️mesh regions: MediaFormat enum, `ArtifactCodec<T>` trait+impls, Mesh exporters/importers, DWG/OBJ/GLB/STL codec bodies, neutral models, STDIO_FORMAT_CATALOG; strip framework `📦️glue.rs:38` re-export; fix brep kernel (18 uses) to emit semio snapshots.
5. Gate: `grep -rn "MediaFormat" --include="*.rs" ✏️s 🧰️framework` (excl. tickets) → 0.

## Wave DAG

```
W0 recon+fixtures ─► W1 mechanisms (SERIAL) ─► W1b scaffold+mounts (SERIAL)
                                   ┌───────────────┼────────────────┐
                                   ▼               ▼                ▼
                             W2a semio subsets(6)  W2b subsets(7)   W3 formats(4)
                                   └───────┬───────┘                │
                                           ▼                        │
                                    W4 io leaves (6) ◄──────────────┘
                                   ┌───────┴────────┐
                                   ▼                ▼
                             W5a heavy plugins(8)  W5b pattern plugins(7)
                                   └───────┬────────┘
                                           ▼
                             W6 MediaFormat deletion + OS rewrite (SERIAL)
                                           ▼
                             W7 os-run + IoRouter e2e (SERIAL)
                                           ▼
                             W8 final gate (verify + closer)
```

- **W0** (2 agents): fresh baselines (stdio test count, policy breach snapshot, os-run error list, hot-file `git status`); per-plugin extraction ledger (files/LOC/MediaFormat census per plugin — becomes each W5 agent's contract); catalog-path confirmation; final pattern-plugin roster; fixture handcrafting (avi/mp3/wav/epw/tsv/html + copy logo.mp4) into ticket `fixtures/`.
- **W1** (1 agent, serial): script.ts schema-owning/delegating generalization + rule edits + programmatic allowlist seeds (round-trip-verify each rule: delete-one-seed → observe breach → restore); SDK `deserializer_entry_of`/`serializer_entry_of`; framework `io_compose_via`; verify `register_document_codec` multi-schema-per-standard behavior; **attempt** os-run fix (3× `topic_contributions`, kernel-glue 🔁️workflow mount, run-crate reconciliation) if `git status` shows the files quiet (poll 3×10 min), else defer to W7. Gate: policy zero-new (181 delegating-subset breaches cleared), stdio 1075/0, `cargo check -p semio-framework` clean.
- **W1b** (1 agent, serial): the single glue/catalog transaction — full semio skeleton (14 subset units with compiling honest placeholders) + 7 format skeletons with fixtures installed + ALL stdio glue.rs mounts + catalog rows + allowlist population + cross-subset type-ownership table (one page, feeds W2 briefs). Gate: crate compiles 0 failures, policy zero-new, `git check-ignore -v` every new dir.
- **W2a/W2b** (6+7 agents ∥, +verify+closer each): one agent per subset, full recipe — complete snapshot, handcrafted sparse diff + DiffAlgebra, named-variant mutations with handcrafted diff()/inverse(), hand-rolled op-codecs via `🧰️triples`, 5×3 facet mirrors, all grammar leaves handcrafted-honest, SubsetValidator, all 8 laws in existing test regions. W2a: brep, mesh, model, object, cad, drawing. W2b: document, image, video, audio, animation, presentation, workflow (+ ✳️any envelope by the closer). Gate per sub-wave: scoped tests green with pasted numbers, full crate green, policy zero-new + allowlist shrink; verify agent greps for apply-and-capture (`snapshot: Option<`, catch-all `other =>` arms).
- **W3** (4 agents ∥, runs beside W2): A1 mp4+avi (remodel move), A2 mp3+wav, A3 epw+tsv, A4 html. Real codecs, honest boundaries typed and documented; mp4 must round-trip the real 43KB fixture byte-preserving (codec_retention_law).
- **W4** (6 agents ∥ by pair family): G1 brep↔step; G2 mesh↔gltf/stl/obj/ply/las; G3 model↔ifc/bcf + object↔json/xml/csv; G4 drawing↔svg/dxf/pdf + cad↔dxf/dwg/step + image↔png/jpg/gif/bmp/tiff; G5 video↔mp4/avi + audio↔mp3/wav + animation↔gltf/mp4/gif; G6 document↔docx/md/txt/pdf + presentation↔pptx + workflow↔json. Trait-impl leaves reusing format engines (zero codec reimplementation); every leaf gets a fixture-backed round-trip test; e2e scenario seeds (a)–(c) land here.
- **W5a/W5b** (8+7 agents ∥, one per plugin): extraction map above. Agents own exactly one plugin dir; read stdio, never edit it (gaps → `stdio_gaps` report section); **leave MediaFormat call sites compiling** (W6's cut). Gate per plugin: `cargo check -p semio-s-plugin-<id>` + plugin tests green + grep-proof of deleted codecs with LOC delta.
- **W6** (1 agent, serial): V7 deletion steps 1–5 above, checklist = W0's 55-file census. Gate: grep-zero, framework/os-kernel/plugin-host checks clean, stdio green, policy zero-new.
- **W7** (1 agent, serial): os-run fix if deferred; dev-boot `io-router: N plugins / M keys` smoke line via existing `io_router_stats()`; build 2 real wasm components (`bun nx run @semio-tech/framework-os-dev:build -- stdio` + one migrated plugin); native routed cross-plugin compose test in the existing run/host test region. Gate: os-run checks+tests green, wasm builds succeed, smoke boots.
- **W8** (verify + closer, fresh eyes): all e2e scenarios re-run from disk; allowlist burn-down audit; `cargo test --workspace --lib` (zero failures in stdio/framework/os-kernel/os-run/plugin-host/all 15 plugins; classify-don't-chase foreign crates); final policy zero; ticket close by orchestrator only.

## End-to-end acceptance scenarios

| # | Scenario | Test home (existing regions only) |
|---|---|---|
| a | cad → semio/brep → .step → reimport → semio/brep → semio/mesh → .gltf, geometry-equivalent | 📐️cad engine tests + semio subset engine tests |
| b | draw → semio/drawing → svg AND dwg; svg re-parses; dwg magic+section asserts | 🖍️draw engine tests |
| c | animate → semio/video → real mp4: decode-clean under the mp4 codec, box-walk + duration/track invariants | 🎞️animate present video engine tests |
| d | 2 real wasm components, routed cross-plugin compose via IoRouter, stats non-zero | 🏃️run / plugin-host test region |
| e | `bun ./📜️script.ts policy` zero above W0 snapshot, allowlists burned down | `w8-policy-final.txt` |
| f | Workspace check+test, zero own-program failures | `w8-workspace-test.txt` |

## Hazard management

- Scope claims: orchestrator writes the roster table (agent → directory globs) into ticket STATUS.md before dispatch; write authority = exactly the globs; everything else queues in `glue_followup`/`stdio_gaps`/`foreign_breakage` report sections.
- Only closers touch hot files; catalog/script.ts edits get pre-edit byte snapshots in ticket scratch + byte-identical round-trip verification; allowlist keys always computed programmatically.
- Concurrent sessions: `git status` scope before editing; foreign unstaged mods → poll 3×10 min, don't chase; lagging call-sites of landed foreign refactors may be completed, mid-edit files may not; gate failures classified own/foreign via git status + symbol grep, foreign recorded never silently fixed.
- Recovery is forward-only (auto-commit, no git mutations, no worktrees): closer resumes the failing agent with concrete failures; unrecoverable → hand-restore from byte snapshot as a new edit.
- Agent report without verbatim test output ≠ verification — closer re-runs.
- Subagents never run `ticket_close`; ticket_close/reopen always with explicit path.

## Ticket & execution mechanics

- Open new master ticket via repo MCP `ticket_open`: title "Semio Artifact, Unified Import Export and MediaFormat Retirement" → `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/<SLUG>/`, goal `🎯aioptimizedrepo`. (Prior overhaul ticket stays as reference at its G gate.)
- Ticket contents: `STATUS.md` (append-only real state), `📌️important.md` (ground-truth corrections + hot-file list + gates), this plan copied in, `fixtures/`, `generators/` (glue-block generator, allowlist-key + census scripts), per-wave reports `w<N><agent>-report.md` + raw outputs as `.txt` (never `.log`), temporary logs `[DEBUG]`-prefixed.
- Fan-out via Workflow tool per wave (pipeline/parallel ≤8 agents), each agent prompt containing: reads-first list (wave section of this plan, 🧬️schema-design.md, f6-final-summary §4, W0 ledger row, exemplars gif 89a/png 1.2/bcf/docx), scope globs, hard rules (no new test files — extend existing `//#region` test regions; no glue/catalog/script.ts/taxonomy edits; regions + emoji docstrings; quote emoji paths; absolute paths; verbatim test numbers; no ticket_close), exit checklist.
- Every wave: fan-out → verify agent → closer (hot-file edits, gates, STATUS.md append, report).

## Verification quick reference

```
cargo test -p semio-s-plugin-stdio --lib                   # ≥1075/0, growing
bun ./📜️script.ts policy                                   # vs W0 snapshot
cargo check -p semio-framework
cargo check --workspace --keep-going                        # classify own/foreign
cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::"   # scoped
bun nx run @semio-tech/framework-os-dev:build -- <pluginId>     # wasm
```
