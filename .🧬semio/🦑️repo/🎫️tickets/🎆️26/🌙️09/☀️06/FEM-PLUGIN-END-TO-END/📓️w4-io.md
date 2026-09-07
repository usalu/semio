# W4 — `✏️s/🔌️plugins/🏗️fem` io layer: ported off `ComposerEntry`, registered on `io_mechanism`

Scope: the two `🚪️io/` directories (`🦀️.rs` + `🟦️.ts`) of `fem2d`/`fem3d`, plus the single `io:` field in
each subset root. Every path below is relative to `/Users/ueli/Documents/semio`.

> **NOTHING WAS COMPILED.** No `cargo`, no `bun`, no `nx`, no build of any kind was run (host swap
> exhausted; the coordinator compiles later). The only tool run against these files was `rustfmt`
> (a parser/formatter, not a compiler) — see §5. Everything in §2/§3 that is Rust is "written to the
> declared trait signatures and cross-checked symbol by symbol against the definitions", **not**
> "verified to compile" and **not** "verified to pass".

## 1. What was wrong, and what the port did

`🚪️io/🦀️.rs` in both artifacts carried a `pub mod io_registry` with a six-row `ComposerEntry` table
(`fem<N>d`, `csv`, `md`, `json`, `stl`, `obj`) built on `composer_entry_of` / `ComposerEntry { writes,
reads, compose }` / `ErasedComposeSource`. That table had **zero callers repo-wide** — a
`grep -rn "io_registry\|ComposerEntry" ✏️s/🔌️plugins/🏗️fem/` before the edit found only the table
itself, its own doc comment, and two bookkeeping comments in the artifact roots
(`🗿️artifacts/◻️2d/🦀️.rs:317`, `🗿️artifacts/🧊️3d/🦀️.rs:236`). Meanwhile each subset root built its own
local `io_declaration()` with `entries: &[]`, self-documented as a DEVIATION/lease-request. Net
effect before this packet: **every foreign-format hop of fem2d/fem3d was unregistered.**

The port follows `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs` and
block's W3 (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/BLOCK-PLUGIN-END-TO-END/📓️w3-io.md`) exactly:

1. Each leaf becomes a typed `Serializer<Fem<N>dSnapshot>` / `Deserializer<Fem<N>dSnapshot>` unit
   struct with `INTO`/`FROM`, `FIDELITY`, `async fn serialize`/`deserialize` (and `sniff` where a
   cheap anchor exists).
2. `🚪️io/🦀️.rs` gains `pub fn io() -> IoDeclaration` whose `entries` is a `OnceLock<Vec<IoEntry>>`
   built from `serializer_entry::<S, T>(FEM<N>D_DIALECT)` / `deserializer_entry::<S, T>(...)`.
3. Each subset root's `io:` field is now `io: io::io()`; its local `io_declaration()` and the
   DEVIATION paragraph are **deleted, not shimmed**.
4. The whole `io_registry` module, plus the dead `import_stdio_kinds()`/`export_stdio_kinds()`
   free functions, are **deleted, not shimmed**. The live stdio-kind lists are the artifact roots'
   own `artifact_kind()` fields, which were never wired to those functions.
5. `derived_composition` **stays** (the `ArtifactComposition` facet `derive_artifact_facets!` binds
   at `🧬️schema/🦀️.rs:268`) but is trimmed to **native-dialect only** — its four foreign branches
   (`csv`/`json`/`md`/`txt`, which called the leaves' now-deleted `deserialize_bytes` free functions)
   moved into the typed leaves.

## 2. Per-format decision table (identical shape in `◻️2d` and `🧊️3d`)

| format | dialect | export | import | fidelity | real / `Err` |
|---|---|---|---|---|---|
| txt | `s.stdio.txt@utf-8/*` | ✅ | ✅ | `Exact` | **real** — `store::ArtifactDsl::print_dsl`/`parse_dsl`; the exact bytes `📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio` carries |
| json | `s.stdio.json@rfc8259/*` | ✅ | ✅ | `Exact` | **real** — `dsl::ToValue` tree → `dsl::json::from_dsl_value` → stdio's `write_json_text`; back via `parse_json_text` + `dsl::FromValue` |
| csv | `s.stdio.csv@rfc4180/*` | ✅ | ✅ | `Exact` | **real** — single-column `payload` RFC 4180 envelope (header row + one quoted cell holding the DSL text) written/read by stdio's `encode_csv`/`decode_csv_with` |
| md | `s.stdio.md@commonmark/*` | ✅ | ✅ | `Exact` | **real** — one fenced code block, info string `fem2d`/`fem3d`, literal = the DSL text; via `MdSnapshot::to_text`/`from_text` |
| stl | `s.stdio.stl@ascii/*` | ✅ | — | `Lossy` | **real geometry** — `fem<N>d_engine::meshing::build_semio_mesh_snapshot` → `SemioMeshToStl` → `encode_stl_ascii` |
| stl | `s.stdio.stl@ascii/*` | — | ✅ | `Lossy` | **typed `Err`** — registered refusal, never a default snapshot |
| obj | `s.stdio.obj@3.0/*` | ✅ | — | `Lossy` | **real geometry** — same kernel → `SemioMeshToObj` → `encode_obj` |
| obj | `s.stdio.obj@3.0/*` | — | ✅ | `Lossy` | **typed `Err`** — registered refusal |

12 `IoEntry` rows per subset, 24 total.

**Why the two refusing hops stay registered.** An unregistered `(from, into)` yields a bare "no
route" at the router; a registered one at the weakest fidelity (`Lossy`, rank 0, so `resolve_route`
never prefers it over a real hop) hands the caller the actual reason. The message shape is
`"<fmt> import not supported for a fem<N>d model: a mesh carries triangles only — it has no
node/element topology, no FemMaterial, no FemSection, no FemSupport, no FemLoadCase and no analysis
settings, …"`.

**Why the refusals live in `🚪️io/🦀️.rs` and not in an import leaf.** The crate root's `#[path]` mount
tree (`✏️s/🔌️plugins/🏗️fem/📦️packages/🦀️rust/🦀️.rs:500-648` for fem2d, `:1127-1275` for fem3d) mounts
**four** import leaves (txt/csv/md/json) and **six** export leaves (those four plus stl/obj). There is
no `📥️import/…/{🔺️stl,🧊️obj}` mount, and the crate root is not this packet's file to edit, so a new
leaf file there would be dead code. Both refusals therefore live in a `pub mod geometry_import`
inside the subset's own io root (`🚪️io/🦀️.rs:84-117`), which the mount tree already reaches.

**Fidelity rationale.** `IoFidelity` is declared per the trait's own doc ("the strongest fidelity
this serializer achieves") about the *snapshot* being converted, not about the foreign envelope:
txt/json/csv/md all carry the fem document losslessly (csv/md wrap the verbatim DSL text in a real,
exactly-reversible container), so `Exact`; stl/obj keep only extruded surface geometry and drop
every material/section/support/load-case/analysis field, so `Lossy`.

**Defects repaired on the way (each was silently wrong before):**

- `📥️import/…/📊️csv` ignored `bytes` entirely and returned `Ok(Fem<N>dSnapshot::default())` — silent
  total data loss on every csv import. Now a real reader, or a typed `Err` naming why the document
  is not a fem envelope.
- `📤️export/…/📊️csv`'s `serialize_bytes` returned `<CsvSnapshot as store::ArtifactPack>::encode_pack`
  — a `.spk` binary container mislabelled as `s.stdio.csv` text. Now real RFC 4180 text.
- `📤️export/…/📝️md`'s `serialize_bytes` threw away the `MdSnapshot` it had just built and returned
  bare `print_dsl` bytes under the `s.stdio.md` name — DSL text mislabelled as CommonMark. Now real
  CommonMark, and the fence carries an info string (the old envelope wrote `info: None`).
- `📤️export/…/🔤️txt` and `📥️import/…/🔤️txt` were both `Err("txt … not yet implemented")` stubs, and
  the **export** one additionally carried a stray `deserialize_bytes` (an import-direction function
  in the export tree, a copy-paste of stdio's json↔txt bridge). Now real, and the stray is gone.

## 3. Files changed (46)

**Rust — 20 io leaves rewritten as typed impls** (10 per artifact), under
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{◻️2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/`:

- `📤️export/🧵️serializers/🗿️artifacts/{🔤️txt/🔖️utf-8,🔣️json/🔖️rfc8259,📊️csv/🔖️rfc4180,📝️md/🔖️commonmark,🔺️stl/🔖️ascii,🧊️obj/🔖️3.0}/✳️any/🦀️.rs`
- `📥️import/🧩️deserializers/🗿️artifacts/{🔤️txt/🔖️utf-8,🔣️json/🔖️rfc8259,📊️csv/🔖️rfc4180,📝️md/🔖️commonmark}/✳️any/🦀️.rs`

**Rust — 2 io roots rewritten**
(`…/{◻️2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🦀️.rs`): truthful module doc with the format
table; `derived_composition` trimmed to native-only; `io_registry`/`import_stdio_kinds`/
`export_stdio_kinds` deleted; new `geometry_import` module; new `pub fn io()`; 6 unit tests each.

**Rust — 2 subset roots, minimal edits**
(`…/{◻️2d,🧊️3d}/🏅️standards/🔖️1/🪆️subsets/🌐️any/🦀️.rs`): `io: io_declaration()` → `io: io::io()`;
the local `io_declaration()` fn deleted; the DEVIATION paragraph replaced by the same
"`io: io::io()` matches the template" note block's subset roots carry; imports narrowed
(`{io, schema}` + `FEM<N>D_DIALECT`; `IoDeclaration`/`LanguagePair`/`NativeCodecs`/`Fem<N>dMutation`/
`Fem<N>dSnapshot`/`FEM_<N>D_SCHEMA` dropped).

**TypeScript — 22 twins** (11 per artifact): the io root `🚪️io/🟦️.ts` now declares the twin of the
entry table (`FEM2D_IO_ENTRIES`/`FEM3D_IO_ENTRIES`, plus `IoFidelity`/`IoDialect`/`IoEntryTwin`,
`CSV_PAYLOAD_COLUMN`, `MD_FENCE_INFO`, `DSL_PREAMBLE`); each leaf `🟦️.ts` names the Rust type it
mirrors and what that hop does, and stays `export {};`. All 22 were `export {};` with no content
before.

No file was created, renamed or deleted — the on-disk file set of both `🚪️io/` trees is byte-for-byte
the same list as before the packet (verified by `find … -type f` before and after).

### 3.1 Why the TS side is declaration-only

The task asked for a TS json codec + DSL reader mirroring block's `🟦️w3-fixture.ts` **if fem's vitest
config can pick up a test for it**. It cannot: `✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript/🧪️tests/🟦️.ts:8`
sets `include: ["🗿️artifacts/**/📚️examples/**/🧪️tests/🟦️.ts"]`, so **no** file under `🚪️io/` is
discoverable, and that config file belongs to another lane. Block's W3 got around this by running
`bun test ./<explicit path>` — which this packet is forbidden to run. A hand-written TS json writer
would additionally have to reproduce `pack::json::write_float`'s lexeme rule and the DSL grammar's
layout rules byte for byte, and its parity fixture could not be generated or checked against the
Rust output without executing something. Writing one anyway would mean shipping a fabricated
fixture, so the twins stay declaration-only and say so in their own docstrings. **Closing this needs
one line in the vitest `include` glob (owner: the TS/package lane) plus a follow-up packet that can
actually run `bun test`.**

## 4. Symbols used, with file:line

Framework (`🧰️framework/🔨️modules/🚪️io/`):

- `🦀️.rs:2373` `pub trait Serializer<S>` — `INTO`, `FIDELITY`, `fn serialize(&S) -> impl Future<…>`
- `🦀️.rs:2387` `pub trait Deserializer<S>` — `FROM`, `FIDELITY`, `CONFORMANCE`, `sniff`, `deserialize`
- `🦀️.rs:2401` `pub struct IoEntry`
- `🦀️.rs:2669` `pub fn serializer_entry<S: store::ArtifactPack, T: Serializer<S>>(own: Dialect)`
- `🦀️.rs:2710` `pub fn deserializer_entry<S: store::ArtifactPack, T: Deserializer<S>>(own: Dialect)`
- `🧬️schema/🦀️.rs:41` `SubsetId::ANY`, `:48` `Dialect`, `:204` `IoPayload`, `:222` `Confidence`
  (`None`/`Low`/`Medium`/`High`), `:247` `IoFidelity` (`Exact`/`Canonical`/`Semantic`/`Lossy`),
  `:270` `IoError`, `:280` `IoOutcome<T>` + `:287` `IoOutcome::clean`, `:293` `IoResult<T>`
- `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:539` `pub fn from_dsl_value` (reached as `dsl::json::from_dsl_value`
  via `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs:143` `pub use pack::json;`)

Plugin declarations (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`):

- `:27872` `LanguagePair`, `:27884` `NativeCodecs`, `:27904` `IoDeclaration`
- `:744-747` the `pub use semio_framework::{… Dialect, IoPayload, StandardId, SubsetId, resolve_ready …}`
  re-export block, `:751` `pub trait ArtifactSerializer` (the trait `SemioMeshToStl`/`SemioMeshToObj`
  implement)

fem crate:

- `🗿️artifacts/◻️2d/🦀️.rs:6` `FEM_2D_SCHEMA`, `:15` `FEM2D_DIALECT`, `:368` `pilot_languages()`
- `🗿️artifacts/🧊️3d/🦀️.rs:6` `FEM_3D_SCHEMA`, `:15` `FEM3D_DIALECT`, `:287` `pilot_languages()`
- `…/◻️2d/…/🧬️schema/📸️snapshot/🦀️.rs:44` `impl store::ArtifactDsl for Fem2dSnapshot` with **sync**
  `:49 parse_dsl` / `:57 print_dsl`, and `:64` `impl store::ArtifactPack` (the bound
  `serializer_entry`/`deserializer_entry` require). Same lines in the `🧊️3d` twin.
- `…/{◻️2d,🧊️3d}/…/🧬️schema/🦀️.rs:264-268` `derive_artifact_facets!(… composition:
  super::super::io::derived_composition::Fem<N>dComposerComposition …)` — the reason
  `derived_composition` had to survive.
- `📦️packages/🦀️rust/🦀️.rs:15-18` `extern crate semio_framework_os_kernel as {dsl,protocol,store,vcs};`
  — where the bare `dsl::`/`store::` paths in every leaf come from.
- `✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🕸️meshing/🦀️.rs:173` and `…/🧊️3d/🕸️meshing/🦀️.rs:146`
  `pub(crate) fn build_semio_mesh_snapshot` — the real mesh exporter (its own doc names the
  `🕸️mesh/🧫️fixtures/**/{🗿️expected.obj,🧊️expected.stl}` fixtures as what it is pinned against).

stdio crate:

- `🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:460 parse_json_text`,
  `:475 write_json_text`, `:607 JsonSnapshot::from_value`, `:612 JsonSnapshot::to_serde_value`,
  `:66 impl From<pack::JsonValue> for JsonValue`
- `🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:20 CsvField`,
  `:34 CsvRecord`, `:47 CsvSnapshot`, `:151 decode_csv_with`, `:162 encode_csv`; `🗿️artifacts/📊️csv/🦀️.rs:11
  STDIO_CSV_DOCUMENT_SCHEMA`
- `🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:60 MdBlock`
  (`CodeBlock { info: Option<String>, literal: String }` at `:82`), `:103 MdSnapshot`,
  `:119 from_text`, `:125 to_text`; `🗿️artifacts/📝️md/🦀️.rs:11 STDIO_MD_DOCUMENT_SCHEMA`
- `🗿️artifacts/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/🦀️.rs:63 encode_stl_ascii` (emits
  `"solid <name>\n…"`, which the new test asserts)
- `🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🚪️io/🦀️.rs:215 encode_obj`
- `🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs:51
  SemioMeshToStl` and `…/🧊️obj/🔖️3.0/✳️any/🦀️.rs:24 SemioMeshToObj`

Note: stdio's on-disk directory emoji changed under the `ENFORCE-UNIQUE-SEMANTIC-EMOJIS` refactor
(`🔣️json`→`🧾️json`, `🧊️obj`→`🗽️obj`, `✳️any`→`🧱️base`/`📐️geometry` in places), but the **Rust module
paths** the leaves import (`artifacts::json::…`, `artifacts::obj::standards::v3_0::engine::…`) are
unchanged, because the stdio crate root re-mounts them under the old module names (barrel shims at
`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs:2741` for obj, and the analogous stl block).

## 5. Verification actually performed (and what it does NOT prove)

1. **`rustfmt --edition 2021 --emit stdout` over all 24 edited Rust files: 24 parsed, 0 failures.**
   `rustfmt` builds a full AST, so this proves every file is **syntactically valid Rust** with
   balanced delimiters. It proves **nothing** about types, trait bounds, or path resolution.
2. `rustfmt --edition 2021` applied in place with the repo's own `rustfmt.toml`
   (`edition 2021`, `max_width 250`, `use_small_heuristics = "Max"`), then re-checked: 0 files
   still differ, so the committed files match the repo formatting gate.
3. Every external symbol referenced by the new code was grepped to a definition and is quoted with
   `file:line` in §4 above. `IoOutcome::clean`, `SubsetId::ANY`, `Confidence::Medium`,
   `IoFidelity::{Exact,Lossy}`, `MdBlock::CodeBlock`'s field names, `decode_csv_with`'s
   header-retention semantics and `encode_stl_ascii`'s `"solid "` prefix were each read in source,
   not assumed.
4. `grep -rn "ComposerEntry\|io_registry\|import_stdio_kinds()\|export_stdio_kinds()"
   ✏️s/🔌️plugins/🏗️fem/` now returns **only comment text** — the two io roots' own "this is DELETED"
   module docs and the two stale artifact-root bookkeeping comments (see §6.1). No code references
   the old channel.
5. Both subset roots were re-read in full: each sets exactly `io: io::io()`, carries no
   `fn io_declaration`, and imports `{io, schema}` + `FEM<N>D_DIALECT`.
6. `find … -type f` over both `🚪️io/` trees before and after: identical file lists (22 files each),
   so nothing was created, renamed or removed.

**Not done, on instruction:** no `cargo check`/`build`, no `cargo test`, no `bun test`, no `nx`, no
dev-server boot. **The Rust half is unbuilt and the six new unit tests per subset have never run.**
The TypeScript half was never type-checked either.

## 6. Open risks

1. **Stale comments in the two artifact roots, not this packet's files.**
   `🗿️artifacts/◻️2d/🦀️.rs:317-318` and `🗿️artifacts/🧊️3d/🦀️.rs:236-237` still say
   "`io_registry::entries()` registers SEVEN composer rows, not five/six … plus
   `composer_entry_of::<Fem<N>dAnyComposer>()`". That module no longer exists. The comments are
   inert (a `//` line in a `definition()` body), but they now describe a deleted thing and should be
   rewritten by whoever owns the artifact roots.
2. **`Fem<N>dAnyComposer` / `Fem<N>dAnyBuilder` may now be unreferenced.** They were used only by the
   deleted `io_registry`. Both are derive-generated `pub` items in `🧬️schema/`, so `dead_code` should
   not fire, but if the workspace lint set is stricter than expected this is where a warning would
   land. Not touched — they are schema-lane files.
3. **`artifact_kind()`'s `{import,export}_stdio_kinds` lists disagree with what is now registered.**
   Both artifact roots list `["stdio.csv", "stdio.json", "stdio.md"]` in *both* directions
   (`◻️2d/🦀️.rs:295-296`, `🧊️3d/🦀️.rs:213-214`) — they omit `stdio.txt` (now real in both directions)
   and `stdio.stl`/`stdio.obj` (real on export). A UI driven by those lists will under-offer.
   Trimming/extending them is an artifact-root/UI-lane decision, deliberately left alone.
4. **md round-trip is asserted at snapshot level, not byte level.** `MdSnapshot::to_text` strips
   trailing newlines from the whole rendered document, and `render_block` adds a `\n` before the
   closing fence when the literal lacks one. So `md_text → from_md_text` can differ from the input
   in a trailing newline **inside the code fence**, which `parse_dsl` ignores. The `Exact` claim and
   the test both compare *snapshots*, which is the honest level — but if a caller ever expects
   byte-identical md re-encode, this is where it will break.
5. **csv envelope assumes stdio's tokenizer keeps quoted newlines.** It does
   (`📊️csv/…/📸️snapshot/🦀️.rs:73-76` documents whole-text, not line-by-line, tokenization), and
   `encode_csv` re-quotes any field containing `,`/`"`/`\n`/`\r` (`escape_field`, `:124-133`). If
   that ever changes, the csv hop silently stops round-tripping multi-line DSL text. The new
   `csv_round_trips_the_example_as_a_real_rfc4180_envelope` test is the tripwire — once it can run.
6. **`s.stdio.txt@utf-8/*` is also the framework's `CARRIER_TEXT`** (`🚪️io/🧬️schema/🦀️.rs:213`).
   Registering a fem↔txt hop therefore also registers a route to/from the generic text carrier.
   Block's W3 made the same choice deliberately; flagged here because a router that treats
   `CARRIER_TEXT` specially could see a fem2d→fem3d path via txt that neither subset intends.
7. **`preflight_io_entries` is untested here.** Its one law is "no two entries claim the same
   `(from, into)` at a different fidelity". All 24 new rows touch a fem dialect on exactly one side
   and each `(from, into)` pair is unique, so it should pass — but that is reasoning, not a run.
8. **fem3d's leaves were machine-derived from fem2d's** (a token-substitution pass, then hand-edited
   for the 3d-specific prose: `FemSolid`/`height`/`base_z`/`Bar`/`Frame` instead of
   `FemRegion`/`thickness`/`Bar`/`Beam`, and eight tables incl. `solids` instead of eight incl.
   `regions`). Both trees were re-read after generation, but a 2d-flavoured phrase surviving
   somewhere in a fem3d doc comment is the most likely residual cosmetic defect.
