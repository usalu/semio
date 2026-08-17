# W2 Packet P2-stdio-data Report

Lane: W2 packet P2-stdio-data, plugin `🗄️stdio`, kinds `📄️pdf`/`📜️docx`/`🎞️pptx`/`📕️xlsx`/`📊️csv`/
`📑️tsv`/`📄txt`/`🔣️json`/`📰xml`/`🌦️epw`/`🎒️zip`/`🗜️deflate`/`💾️binary`. Recipe followed:
`📓️w2-cad-report.md` (adapted per the brief: these kinds had **no app to migrate**, so both surfaces
are authored fresh straight against each artifact's own schema, mirroring `🔋️energy`'s exemplar rather
than moving an `🎛️apps` tree). Contract: `📋️contract-freeze.md` §1, §2, §2.6.

## Ground-truth correction (measured, not assumed)

The brief's subset count (27) undercounts by 4 against what's actually on disk: `🔣️json` has **2**
subsets (`any`, `i-json`), not 1; `📰xml` has **2** (`any`, `valid`), not stated as 2 explicitly but
confirmed; total **31 real subsets** across the 13 kinds (measured via `find … -regex '.*🪆️subsets/[^/]*'`
per kind, not grepped or assumed):

| kind | standard(s) | subsets | window kit |
|---|---|---:|---|
| `📄️pdf` | 1.4, 1.7 | 10 (`a`,`any`,`x`; `a`,`any`,`e`,`h`,`ua`,`vt`,`x`) | `DocumentWindowKit` |
| `📜️docx` | ecma-376 | 3 (`any`,`strict`,`transitional`) | `DocumentWindowKit` |
| `🎞️pptx` | ecma-376 | 3 | `DocumentWindowKit` |
| `📕️xlsx` | ecma-376 | 3 | `TableWindowKit` |
| `📊️csv` | rfc4180 | 1 | `TableWindowKit` |
| `📑️tsv` | iana | 1 | `TableWindowKit` |
| `📄txt` | utf-8 | 1 | `TextWindowKit` |
| `🔣️json` | rfc8259 | 2 (`any`,`i-json`) | `TreeWindowKit` |
| `📰xml` | 1.0 | 2 (`any`,`valid`) | `TreeWindowKit` |
| `🌦️epw` | energyplus | 1 | `TableWindowKit` |
| `🎒️zip` | 2.0 | 2 (`any`,`iso21320`) | `TreeWindowKit` |
| `🗜️deflate` | rfc1950 | 1 | `TextWindowKit` (hex/metadata summary) |
| `💾️binary` | raw | 1 | `TextWindowKit` (hex dump) |

**31 subsets × 2 surfaces = 62 real editor/viewer pairs**, every one built from the frozen kit APIs
(§2.6), no exceptions, no faked mutations.

## Execution model

Given the scale (31 subsets, ~186 real content files: surface-root rs+ts, mode rs, window rs+ts, per
surface), the work was split: I built `📊️csv`/`📑️tsv`/`📄txt`/`🔣️json`(×2)/`📰xml`(×2) — 7 subsets —
directly, and delegated four parallel, tightly-briefed subagents for the rest, each given the exact
`ArtifactEditor`/`ArtifactViewer` trait signatures, the seven `WindowKit` APIs, the `🔋️energy` exemplar
files to copy the shape of, and pre-measured `Dialect` ground truth for every one of their subsets (so
no subagent had to guess a dialect). Every subagent was forbidden from touching `🧬️schema/**`,
`🚪️io/**`, `📦️glue.rs`, any plugin-root file, or any kind outside its own assignment. I did every
`📦️glue.rs` mount and plugin-root `.editor()`/`.viewer()` registration myself, serially, re-reading the
shared files fresh before each edit (they are concurrently edited by two sibling W2 packets and one
live peer ticket).

- Me directly: csv, tsv, txt, json (any + i-json), xml (any + valid) — 7 subsets.
- Subagent 1: epw, zip (any + iso21320), deflate, binary — 5 subsets.
- Subagent 2: xlsx (any, strict, transitional) — 3 subsets.
- Subagent 3: docx (any, strict, transitional) + pptx (any, strict, transitional) — 6 subsets.
- Subagent 4: pdf, all 10 subsets.

## What landed, per kind

Every subset's editor gained one real window on the fitting kit's `editable_window_kind()`, dispatching
through a small typed `Command` enum whose single real action maps to a genuine mutation the artifact's
own `🧬️schema/🧬️mutations` already declares (every kind here has real per-field mutations beyond
`SetSnapshot` — verified before building, none needed a "no mutations declared" fallback). Every
subset's viewer gained the read-only twin on `window_kind()`, `Command` = a one-variant `Noop`,
`handle` always returns `Ok(ViewEmit::default())`. Both surfaces carry a real `🟦️component.ts` twin at
every window (never empty), en-then-de `LocalizedLabel::native` everywhere, and the 3-test manifest
pattern + window definition/render tests the `🔋️energy` exemplar establishes.

- **`📄️pdf`** (10× `DocumentWindowKit`): one page per `PdfPage`, text = the page's real `MediaBox`/
  `CropBox` geometry plus its own `text` field (ToUnicode-extracted or authored — never fabricated).
  `set-page` → `PdfMutation::AppendPageContent` (the closest real primitive; PDF has no "replace this
  page's whole text" mutation, documented honestly as an append, not a silent misrepresentation).
- **`📜️docx`** (3× `DocumentWindowKit`): one page per top-level `DocxDocument.body` block. `set-page` →
  `DocxMutation::SetBlockContent`, only when the block is a `Paragraph` (`Table` blocks and
  out-of-range index are a documented no-op).
- **`🎞️pptx`** (3× `DocumentWindowKit`): one page per slide, text = every text-bearing shape
  concatenated. `set-page` → `PptxMutation::SetShapeText` targeting only the FIRST text-bearing shape
  (multi-shape slides are read-only beyond shape 0, documented).
- **`📕️xlsx`** (3× `TableWindowKit`): every sheet's cells flattened into one `sheet`/`row`/`col`/`value`
  row-per-cell table (not a single-sheet pick — a fixed pick would silently hide multi-sheet data).
  `set-cell` → `XlsxMutation::SetCell`, write-back infers `Boolean`/`Number`/`InlineString`.
- **`📊️csv`**, **`📑️tsv`** (`TableWindowKit`): header-row/no-header convention respected (csv only;
  tsv has none), `set-cell` → `CsvMutation::SetField` / `TsvMutation::SetCell`.
- **`📄txt`** (`TextWindowKit`): whole-buffer `lines`/`line_ending` join; `replace-text` → whole-document
  `TxtMutation::SetSnapshot` (the honest mapping for a buffer-replace kit action).
- **`🔣️json`** (2× `TreeWindowKit`): full `JsonValue` tree, node ids are `k=`/`i=`-tagged paths;
  `set-node` → `JsonMutation::SetScalar` at the decoded path (any node, not just leaves — replaces
  whichever subtree was there; write-back always lands as `JsonValue::String`, documented).
- **`📰xml`** (2× `TreeWindowKit`): full `XmlNode` tree, node ids are `/`-joined child-index paths;
  `set-node` fires only when the resolved node is `Text` (→ `XmlMutation::SetText`) — `Element`/
  `CData`/`Comment`/`ProcessingInstruction` render read-only, documented.
- **`🌦️epw`** (`TableWindowKit`): one row per hourly `EpwRecord`, all 35 EPW-spec columns editable via
  `EpwMutation::SetRecordField`; the 8 scalar header lines are not surfaced (no natural table slot,
  documented).
- **`🎒️zip`** (2× `TreeWindowKit`): root = archive comment (renamable), one leaf per entry showing
  `name (n bytes)` (name renamable via `ZipMutation::RenameEntry`/`SetArchiveComment`, byte count
  read-only) — real per-byte entry content editing isn't representable through a tree label control,
  documented, matching the ticket's own "container/opaque → TreeWindowKit entry listing" framing.
- **`🗜️deflate`** (`TextWindowKit`): header metadata (`method`/`windowBits`/`levelHint`/
  `presetDictionary`) as an editable `key=value` summary, `replace-text` → `SetCompressionParams`/
  `SetPresetDictionary`; the compressed `payload` is a trailing `#`-comment byte count only, never
  editable text (a compressed byte stream has no honest text form) — matches the brief's
  "TextWindowKit hex/metadata summary" framing.
- **`💾️binary`** (`TextWindowKit`): lowercase-hex dump capped at 4096 bytes for display;
  `replace-text` splices the WHOLE persisted buffer via `BinaryMutation::Splice`, with the truncation
  caveat spelled out in the rendered `#`-comment itself, not hidden.

Every `<KIND>_DIALECT` const was verified against the artifact's own real `Dialect` literal (its
`🚪️io`/`🧬️schema` `DIALECT` const or `ArtifactAnalysis::DIALECT`), never guessed — the full table of
all 31 measured dialects is in the session's tool history; every subset's editor/viewer file duplicates
its own literal locally (never imported cross-surface), matching the pilot's precedent.

## Bugs found and fixed in delegated work

The docx/pptx subagent's two `set_page_*` unit tests (×6 files: docx any/strict/transitional, pptx
any/strict/transitional) hand-constructed `ArtifactView`/`ConfigView`/`DraftView` with wrong field
names (`ConfigView { config: … }` instead of `{ snapshot: … }`, `ArtifactView { generation: … }` — a
field that doesn't exist) — a real compile break, not a style nit. Fixed by extracting the mutation-
building logic into a standalone pure `build_set_page_mutation(snapshot, index, text) -> Option<Mutation>`
helper in all 6 files (mirroring the pattern I used for my own csv `handle()` test, specifically to
avoid this exact pitfall — `ArtifactView`'s real fields are `snapshot`/`history`/`children`, not
trivially constructible in a unit test without a testkit helper that doesn't exist yet), and rewrote
the 12 broken tests to call the helper directly. Verified via a second `cargo check` run: the ~24
cascading errors these 12 tests produced are gone.

Also caught and fixed two of my own instances of the "🏅️standards vs 🏅️标准" emoji-typo trap while
hand-typing `📦️glue.rs` mount paths mid-session (`📊️csv` and `📄txt` stray directories) — both caught
immediately via `ls`-verification and removed before they could propagate; switched to building
`📦️glue.rs` insertions via Python string-templating off `find`-verified base paths for every kind after
the first two typos, which is why none of the later 20+ mount blocks needed a fix.

## `📦️glue.rs` / plugin-root wiring

`📦️glue.rs` already had a live-populated `//#region ✏️Editor` / `//#region 👁️Viewer` pair (two sibling
W2 packets, P1-stdio-media and P3-stdio-geometry, were mounting concurrently) — I added four
subregions (`P2-stdio-data`, `P2-stdio-data-pdf`, `P2-stdio-data-docx-pptx`, `P2-stdio-data-xlsx`,
`P2-stdio-data-epw-zip-deflate-binary`) inside the existing blocks, never touching a sibling's own
subregion. Module nesting follows whatever each subagent's files actually reference internally (verified
by grep before mounting, not assumed): flat `pub mod <kind>` for single-subset kinds (csv/tsv/txt/epw/
deflate/binary), `pub mod <kind>_<subset>` for json/xml (matching the sibling packets' own established
`jpg_any`/`tiff_baseline`-style convention), `pub mod zip { pub mod any {…} pub mod iso21320 {…} }` for
zip, `pub mod <kind>_<n>` flat slugs (`pdf14a`, `pdf17ua`, …) for pdf, and
`pub mod <kind> { pub mod standards { pub mod v_ecma_376 { pub mod subsets { pub mod <subset> {…} } } } }`
for docx/pptx/xlsx. Verified with the disk-resolution script (adapted from the pilot) against the WHOLE
file after every insertion: **4735 `#[path]` attributes, 0 missing**, checked five times (once per
insertion round).

Plugin root `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs`: added `.editor()`/`.viewer()` pairs for all 31
subsets across three appended `#[region]`s (`SurfacesP2StdioData`, `SurfacesP2StdioDataDocuments`,
`SurfacesP2StdioDataMisc`), re-reading the file fresh before the third insertion since the concurrent
P3-stdio-geometry packet had legitimately rewritten its own region in the meantime (documented an SDK
gap — `PluginBuilder::editor`/`::viewer` require `Mutation: SemanticMutation`, which most of P3's
hand-rolled mutation enums don't derive — not this packet's concern, verified via `git status`/`git log
--date=iso` before attributing, confirmed live-edited today).

## Outside-lease referrers

None found. `grep -rn "apps::(pdf|docx|pptx|xlsx|csv|tsv|txt|json|xml|epw|zip|deflate|binary)"` across
the repo (outside stdio's own tree) returns nothing — these kinds never had an `🎛️apps` tree to begin
with (schema-owned, zero-app plugin), so there is no legacy app path for any other plugin to have
depended on.

## SDK gaps confirmed (already reported by earlier lanes, re-confirmed here)

Nothing new. The seven `WindowKit`s' curated crate-root re-export gap (`TextWindowKit`/`TableWindowKit`/
`TreeWindowKit`/`DocumentWindowKit`/`ImageWindowKit`/`MediaWindowKit` still require the
`semio_framework_plugin::app::` prefix; `MeshWindowKit`/`WindowKit` are bare) persists as of this
packet — every file in this packet imports them via `app::`, per the still-current gap.

## Verification

- `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-stdio --all-targets --keep-going`, four runs across
  the session as work landed, final output in `🧪️w2-stdio-data-cargo.txt`:
  - Final run: 1256 errors total, **0 anchored in any `✏️editor/**` or `👁️viewer/**` file under any of
    my 13 kinds** (`grep -oE "🗿️artifacts/[^ ]*🦀️component\.rs:[0-9]+:[0-9]+" cargo.txt | grep -E
    "/✏️editor/|/👁️viewer/"` → empty after the docx/pptx test fix). Every remaining error/warning is
    inside `🧬️schema/**`/`🚪️io/**` (the live peer FULL-STDIO ticket, confirmed via `git status`/
    `git log --date=iso` showing today's commits on those exact files, not mine) or inside sibling
    packets' own `🧿️semio` files (not mine, P3-stdio-geometry's lease).
  - Two harmless "unnecessary qualification" warnings in my own `📊️csv` window test files, fixed for
    cleanliness (not a compile blocker).
- Live-filesystem scoped policy check (Python script mirroring `policySubsetSurfaceCompletenessBreaches`/
  `policyViewerPurityBreaches` exactly, since a full-repo `bun ./📜️script.ts policy` run would take
  a long time and report on 100+ other still-scaffolded subsets outside this lease): **31/31 subsets
  measured, 0 scaffold-residue breaches, 0 viewer-purity breaches, 0 surface-completeness breaches.**
- `📦️glue.rs` `#[path]` resolution: 4735 total attributes repo-wide in that file, 0 missing, verified
  after every insertion round (5 rounds total).

## Files touched

Created/edited (content, not new taxonomy dirs — the scaffolder already created every directory in W1):
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{📄️pdf,📜️docx,🎞️pptx,📕️xlsx,📊️csv,📑️tsv,📄txt,🔣️json,📰xml,🌦️epw,🎒️zip,🗜️deflate,💾️binary}/**/{✏️editor,👁️viewer}/**`
  — surface-root `🦀️component.rs`+`🟦️component.ts`, mode `🦀️component.rs`, window
  `🦀️component.rs`+`🟦️component.ts`, across all 31 subsets × 2 surfaces (≈186 content files; the
  `🎚️config`/`👥️presence`/`🫧️transient`/`🎮️commands`/`🌉️wasm`/`📚️examples` facet dirs were left as the
  scaffold's own `📌️empty.md`, per contract).
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` — 5 new subregions inside the pre-existing
  shared `✏️Editor`/`👁️Viewer` blocks.
- `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs` — 3 new subregions registering all 31 subsets'
  `.editor()`/`.viewer()` calls.

Not touched: `🧬️schema/**`, `🚪️io/**` anywhere (peer FULL-STDIO ticket's live lease), any other stdio
kind, any plugin outside `🗄️stdio`.

Scratch (ticket folder): `🧪️w2-stdio-data-cargo-run1.txt`, `🧪️w2-stdio-data-cargo-run2.txt`,
`🧪️w2-stdio-data-cargo.txt` (final).
