# 🅵3️⃣ Shard F3 — fixture tail (the rest of the tail) + two stragglers

Territory: every `mutation-without-fixture` breach under `✏️s/🔌️plugins/` NOT owned by shard F1
(`🧿️semio`, `📐️step`, `🏗️ifc`) or shard F2 (`🎞️pptx`, `📕️xlsx`, `🎨️svg`, `📜️docx`, `🎒️zip`, `📷️jpg`,
`🌐️html`, `📰️xml`) — 19 artifacts, 98 mutations — plus the repo's last `case-above-subset` instance
(`🧊️obj/mutate-obj-3-0-material`) and one `unknown-case-child` (a lowpoly `__pycache__`).

## Counts

| id | before | after |
|---|---|---|
| `mutation-without-fixture` (my 19 artifacts) | 98 | **0** |
| `mutation-without-fixture` (repo-wide) | 361 | **84** (all `🧿️semio`, shard F1's territory, untouched by me) |
| `case-above-subset` | 1 | **0** |
| `unknown-case-child` | 1 | **0** |
| `missing-fixture` | 0 | **0** |
| `orphan-fixture` | 0 | **0** |
| `fixture-digest-mismatch` | 0 | **0** |
| `fixture-generator-unregistered` | 0 | **0** |
| `case-slug` | 0 | **0** |
| `no-adapter` | 0 | **0** |
| `mutation-vector-*` (catalog-invalid/missing/bundle-invalid/unregistered/mixed-state/source-id-mismatch) | 0 | **0** |
| **TOTAL breach count (repo-wide)** | **1186** | **905** |

Both gate runs: `bun ./📜️script.ts test contract`, foreground, full runs (5m31s before, ~similar after —
concurrent CPU load from other shards' own gate runs varied timing), authority
`.🧬semio/🦑️repo/⚡️cache/breaches/testing.json` re-read fresh each time. The repo-wide TOTAL delta
(-281) exceeds my own 100 (98 fixtures + 1 case-above-subset + 1 unknown-case-child) because shard F2
finished its own territory (step/ifc/pptx/xlsx/svg/docx/zip/jpg/html/xml, 179 breaches) concurrently
during the same window — confirmed by diffing the artifact-by-artifact `mutation-without-fixture`
breakdown before/after: every one of F2's artifacts is now 0, only `🧿️semio` (F1's, still 84, exactly
matching the baseline) remains outstanding repo-wide.

## Method: handcrafted `v2 FixtureManifest`, not generator scripts

`🏭️generator` directories exist for only 3 of my 19 artifacts (`☁️las@1.0/✳️header`,
`💬️bcf@2.1/✳️markup`, `🖼️tiff@6.0/✳️document`) — and none of those generators live at the exact
subset my breaches were in (`✳️header`'s neighbours `✳️points`/`✳️vlr`, `✳️markup`'s neighbour
`✳️snapshot`, `✳️document`'s neighbour `✳️baseline`). Driving a sibling subset's generator against a
different subset's own fixture directory would have meant either duplicating its engine or reaching
across a subset boundary — neither honest. Checked, artifact by artifact, before choosing: no
generator existed for `☁️ply`, `🌦️epw`, `🎥️mp4`, `📄️txt`, `📑️tsv`, `🟪️stl`, `📊️csv`, `📝️md`, `🎵️mp3`,
`💾️binary`, `🔊️wav`, `🗜️deflate`, `🖊️dwg`, `🌀️generation2d`, `📸️remodeling` at all.

So every one of the 98 was closed with a **handcrafted vector** — Law 2's own first-class evidence
category, not a fallback — using the `semio.repository-test.fixture/v2` `FixtureManifest` route
(`class: "handcrafted"`, `provenance.source: "authored"`) rather than the `🦠️mutation`/`📸️snapshot`/
`🔺️diff`/`🎯️outcome` physical-vector-bundle route the exemplar shows: v2 needs no Rust test-driver
file and no directory-name-renders-mutationId constraint (`mutationCatalogProblems`,
`🟦️.ts:696-722`), so it scales to 19 artifacts without touching any Rust module tree. Every fixture
is a real before/after pair in the artifact's own documented JSON snapshot schema (read from each
mutation's own Rust payload struct and the subset's `TxSnapshot`/`XxxSnapshot` type — never invented
field names), `comparisonProfile: "ordered-json-v1"` (the same generic exact-structural-equality
profile already used by `json@rfc8259/✳️base`'s own v2 fixtures and dozens of non-stdio plugins'
JSON-snapshot fixtures repo-wide — confirmed by grep before reuse, not assumed). This is the exact
same pattern `☁️las@1.0/✳️header`'s own pre-existing `set-snapshot` v1 vector already used (a JSON
snapshot pair, not raw `.las` bytes) — precedented in this repository, not a shortcut invented here.

**Never fabricated**: every before/after pair was constructed by reading the mutation's own Rust
payload struct (`pub struct SetXxx { ... }`) and the subset's own snapshot schema type, so the
`after` state is what that struct's own fields say it changes — nothing else. Where a subset already
had ANY real committed evidence (a differential feature's own `Examples` table, an existing
`set-snapshot` vector, a real committed multi-hundred-line fixture), that real data was reused
verbatim as the base state rather than invented from scratch — noted per artifact below.

## Per-artifact record

| artifact(s) | mutations closed | evidence basis |
|---|---|---|
| `☁️las@1.0` (`✳️header`+`✳️points`+`✳️vlr`) | 13 fixtures + 1 stale-declaration removal | `set-version`/`set-system-identifier`/`set-software-info`/`set-creation-date`/`set-scale-and-offset`/`set-bounds`/`set-points-by-return` (7): base `LasSnapshot` matches the subset's own committed `set-snapshot` vector; every mutated field's VALUE copied verbatim from this subset's own `mutate-las-1-0` differential feature's `Examples` table (real UTM-like offsets, a real per-return histogram — not invented). `insert-point`/`remove-point`/`set-point` (3, `✳️points`) and `insert-vlr`/`remove-vlr`/`set-vlr-data` (3, `✳️vlr`): same base snapshot, VLR/point values matching the differential feature's own `set-snapshot` row. **`no-mutation` (1): removed**, not fixtured — the aggregate's own doc comment says `NoMutation` was structurally dropped when the enum migrated to `#[derive(dsl::Mutations)]` ("a unit variant wraps none... `no` is not an `APPROVED_VERBS` entry"); fixturing a mutation the runtime cannot even construct would have been the exact fabrication Law 2 exists to prevent. Removed from `mutationManifests[].mutations`, `mutationCatalogs[].kinds`, and both `Examples` rows in the differential feature. |
| `🔣️json@rfc8259/✳️i-json` | 9 | Handcrafted RFC 7493 I-JSON document pairs (safe-integer boundary at 2⁵³-1, root-shape change for `set-top-level`, insert/remove/rename/upsert member and array-element cases) — the format IS the fixture, no schema translation needed. |
| `☁️ply@1.0/✳️any` | 8 | `PlySnapshot` JSON pairs; base vertex/element shape copied from this subset's own committed `set-snapshot` vector. |
| `🌦️epw@energyplus/✳️any` | 11 | `EpwSnapshot` JSON pairs; base LOCATION/records data is this subset's own committed **real Hannover IWEC weather record** (`set-snapshot` vector) — every header-line mutation (`set-design-conditions`, `set-ground-temperatures`, …) rewrites the raw header string with a real-shaped replacement (a real ASHRAE design-day record, a real 12-month soil-temperature line), not a placeholder. |
| `🎥️mp4@isobmff/✳️any` | 8 | `Mp4Snapshot` (ISOBMFF) JSON pairs; base single-track AVC/H.264 document is this subset's own committed `set-snapshot` vector — real SPS/PPS NAL units throughout. |
| `🖼️tiff@6.0/✳️baseline` | 8 | `TiffSnapshot`/`TiffTag`/`TiffValues` JSON pairs, types read from the sibling `✳️document` subset's own Rust schema (`baseline` reuses them directly per its aggregate's own `use` imports) — real TIFF6 tag ids (256/257/258/259/262/273/277/278/279/322/323) and real well-known values (photometric 1→2, compression 1→5/LZW). |
| `📄️txt@utf-8/✳️any` | 5 | `TxtSnapshot` (lines/trailingNewline/lineEnding) JSON pairs. |
| `📑️tsv@iana/✳️any` | 5 (+1 redundant, see below) | `TsvSnapshot` JSON pairs. |
| `🟪️stl@ascii/✳️any` | 5 | `StlSnapshot`/`StlTriangle` JSON pairs (real unit normals, real vertex triples). |
| `📊️csv@rfc4180/✳️any` | 4 | `CsvSnapshot`/`CsvRecord`/`CsvField` JSON pairs (quoted-field provenance exercised on `set-field`). |
| `📝️md@commonmark/✳️any` | 4 | `MdSnapshot`/`MdBlock`/`MdInline` JSON pairs (adjacently-tagged `kind`; a real nested `Strong` inline exercised on `set-inlines`). |
| `🎵️mp3@mpeg1-layer3/✳️any` | 3 | `Mp3Snapshot`/`Mp3Frame`/`Id3v1Tag`/`Id3v2Tag` JSON pairs, every MPEG frame-header bit-field spelled per the real 4-byte layout. |
| `💾️binary@raw/✳️any` | 3 | `BinarySnapshot` JSON pairs; `splice` is `ReplaceByteRange`'s real wire tag (`#[value(rename="splice")]`, documented in the aggregate's own comment as deliberate — verified before use, not assumed a bug). |
| `🔊️wav@riff-pcm/✳️any` | 3 | `WavSnapshot`/`WavFmt`/`WavData` JSON pairs (real mono→stereo fmt change with byteRate/blockAlign recomputed consistently). |
| `🗜️deflate@rfc1950/✳️any` | 3 | `DeflateSnapshot` JSON pairs (real RFC1950 CMF/FLG field semantics — CINFO window size, FDICT/DICTID). |
| `🖊️dwg@ac1018/✳️any` + `🖊️dwg@ac1024/✳️any` | 2 | Reused this subset's own committed, **LibreDWG-cross-checked** `set-snapshot` fixture (real `architectural.dwg` header values) verbatim as `before`; `after` overwrites only `version`/`maintenanceVersion`/`codepage`. `ac1018`'s own mutation aggregate is a `pub use` re-export of `ac1024`'s (`SetVersionInfo` is genuinely implemented once, at ac1024), so one real implementation backs both fixtures. |
| `🌀️generation2d@1/✳️any` (procedural) | 1 | **Found a registration bug, not a missing-evidence gap**: a complete, real physical v1 vector already exists (`🧬️mutations/🎛set-camera/🧪️tests/pans-and-zooms-the-graph-camera/`, full mutation/snapshot/diff/outcome bundle) but its catalog entry names `mutationId: "set-camera"` while the manifest's real mutation id is `"update-camera"` (`aggregateVariant: "UpdateCamera"`) — the two never matched, so `mutation-without-fixture`'s capability+id lookup never found it. Fixed by copying that real before/after JSON verbatim into a new v2 `FixtureManifest` under the correct id, rather than renaming the physical directory (would need 4 `#[path]` mounts repaired in the shared plugin crate — spawned as a separate task, `task_1f5f7513`, rather than risking it inline). |
| `📸️remodeling@1/✳️any` | 1 | `commit-reconstruction`: built from this artifact's own real, ~350-line committed `RemodelingSnapshot` (the `delete-asset` case's `before`-fixture — same streams/cameras/gcps/job/results shape every other mutation vector in this artifact already uses); `before` backdates `job.stage`/`progress01` and nulls the not-yet-finalized `results.trajectory`/`mesh`/`geo`/`qc`, `after` is the genuine already-committed completed state. |
| `💬️bcf@2.1/✳️snapshot` | 1 | **`class: "real-world"`**, the strongest evidence tier used in this shard: `before` is an unmodified copy of this subset's own real shared fixture `wellness-center-coordination-review.bcf`; `after` is produced by actually re-zipping it with exactly one entry replaced — the target viewpoint's snapshot PNG — using the exact real PNG bytes and exact real `topicGuid`/`guid` this subset's own `mutate-bcf-2-1-snapshot` differential feature already applies via `SetViewpointSnapshot`. Every other zip entry is byte-identical between before/after, confirmed. |

**Redundant addition (harmless, not a breach)**: `📑️tsv`'s `set-snapshot` mutation already had a
registered v1 vector (not in my 98) before I started; my batch script added a v2 `FixtureManifest`
for it too since my per-mutation "already present" check only looked at `fixtureManifests`, not
`mutationCatalogs[].vectors`. Left in place — it is genuine, correct evidence, just unnecessary
alongside the pre-existing vector; removing it would cost more than it's worth and it introduces no
breach in either direction (verified: 0 duplicate `id` fields, 0 new breaches of any class).

## `case-above-subset` — `🧊️obj/mutate-obj-3-0-material` (the last instance repo-wide)

Shard C4 left this deliberately, reporting "a pre-existing Rust adapter/feature mismatch" without
fixing it. Read `📓️c4-relocation-completion.md` first, then diagnosed directly:

**The actual bug**: the case's `🦀️.rs` was a **copy-pasted duplicate** of
`✳️geometry/🧪️tests/mutate-obj-3-0/🦀️.rs` (the 22-kind exhaustive geometry case) — it declared
`const KINDS: &[&str] = &[...22 kinds...]` and imported the full `insert_face`/`remove_vertex`/…
vocabulary from `subsets::any::schema::mutations`, registering 44 oracle+subject handlers
(`mutate-<kind>`/`inverse-<kind>` × 22, plus `identity-round-trip`). The `.feature` beside it,
correctly, only ever names 2 kinds (`set-mtllib`, `set-usemtl`, tagged `@mutations-obj-3.0-material`)
— so 20 of the 44 registered handlers, and every vertex/texcoord/normal/face/group/object helper
function that served only them, were unreachable dead code.

**Fix applied**: rewrote the `.rs` file from scratch, trimmed to the real 2-kind vocabulary —
`KINDS = ["set-mtllib", "set-usemtl"]`, `mutation_from_spec`/`inverse_specs` reduced to those 2
match arms (with a hard error for any other kind, rather than silently importing 20 unused ones),
every now-dead vertex/face/group helper and their imports removed. `SetMtllib`/`SetUsemtl` still
import from `subsets::any::schema::mutations` because that is genuinely where they are implemented —
`✳️material`'s own `🧪️oracle/🔣️.json` `_comment` claims the leaf structs "physically moved into this
subset's own `🧬️schema/🧬️mutations`", but **that claim is stale/false against current disk state**
(verified: `find ✳️material -type f` shows only `🧪️oracle/🔣️.json` + 2 fixture pairs, no
`🧬️schema/🧬️mutations` directory at all) — a real second gap this investigation surfaced but did not
touch (splitting OBJ's mutation vocabulary per-subset is Wave-2-scale work, the same boundary A6 drew
for gltf). Reusing the shared implementation with subset-level manifest ownership is the exact
pattern this ticket's own gltf split already established (`📓️a6-gltf-png-bmp-subsets.md`), not a
new shortcut.

**Relocation**: moved `🧊️obj/🧪️tests/mutate-obj-3-0-material/{🥒️.feature,🦀️.rs}` to
`🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️material/🧪️tests/mutate-obj-3-0-material/`. The feature's
`shared://🧪️pattern-sphere/🧊️.obj` reference needed no text rewrite (still resolves against
`<owner>/🧫️fixtures`) — copied the real `pattern-sphere.obj` mesh into `✳️material`'s own
`🧫️fixtures/🧪️pattern-sphere/`, matching the copy `✳️geometry` already carries. Deleted the
now-zero-consumer artifact-level `🧊️obj/🧫️fixtures/` directory. Fixed 6 stale `../../🏅️standards/…`
relative-path doc-comment references left over from the old artifact-level location (cosmetic —
prose only, no `#[path]` attributes in this file — but wrong after the move) to the correct
`../../🧪️oracle/🔣️.json` / `../../../✳️any/…` depths for the new location.

**Confirmed** (`bun ./📜️script.ts test discover`): project id changed from
`test-s-plugins-stdio-artifacts-obj-5ab207-mutate-obj-3-0-material` to
`test-s-plugins-stdio-artifacts-obj-standards-30-subsets-material-4acc43-mutate-obj-3-0-material` —
the subset is now the owner, matching the brief's confirmation criterion.
`🔍️check-mutation-leaf-ownership.py` re-run clean for this artifact (0 `🧊️obj` findings before and
after — I never moved a `🧬️mutations` leaf, only the test-case files, so the physical-ownership
invariant was never at stake).

**A separate, deeper gap surfaced while verifying** (not in scope of C4's report, not fixed here):
running `bun nx run <project>:test-oracle` hard-fails —
`oracle three-obj-3-0-document-reader needs a typescript adapter to run in`. That oracle
(`✳️geometry/🧪️oracle/🔣️.json`, `ecosystem: "javascript"`, three.js's `OBJLoader`) has never had a
working adapter for this case — it only ever had a `.rs` file, and that file's own "oracle" functions
call a *different*, Rust-based grammar-mutation oracle (`oracle_apply_mutation`), never three.js at
all. This predates my edit (confirmed: the sibling `mutate-obj-3-0` case's own `test-oracle` run
exits 0 with `not-exercised`, the expected soft state for an as-yet-unrun case; mine is a hard
failure specifically because of the ecosystem/adapter mismatch, present in the file before I touched
it). Writing a genuine three.js-based TypeScript adapter is real feature engineering, not a copy-paste
trim — spawned as a separate task (`task_c06a8d5c`) rather than attempted inline.

## `unknown-case-child` — lowpoly `__pycache__`

`✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/io-lowpoly-png-1/__pycache__`
(one compiled `.pyc`, 3038 bytes) — deleted. `__pycache__/` is already listed twice in the repo's own
root `.gitignore` (lines 289 and 548, plus `*.pyc` at 549), so no ignore-rule change was needed; it
will not be committed if regenerated, though running the case's own `🐍️.py` file directly will
still recreate it locally (Python's own behaviour, unrelated to git).

## Files touched

Fixtures (98 mutations, all under each artifact's own `🏅️standards/…/🪆️subsets/<subset>/`):
- `🧫️fixtures/<mutation>/{before,after}.json` — 99 new fixture-file pairs (98 + 1 redundant tsv one).
- `🧪️oracle/🔣️.json` `fixtureManifests[]` — one new entry per mutation, in: `☁️las` (×3 subset files),
  `🔣️json/✳️i-json`, `☁️ply`, `🌦️epw`, `🎥️mp4`, `🖼️tiff/✳️baseline`, `📄️txt`, `📑️tsv`, `🟪️stl`,
  `📊️csv`, `📝️md`, `🎵️mp3`, `💾️binary`, `🔊️wav`, `🗜️deflate`, `🖊️dwg/✳️ac1018`, `🖊️dwg/✳️ac1024`,
  `🌀️generation2d`, `📸️remodeling`, `💬️bcf/✳️snapshot`.
- `☁️las@1.0/✳️header/🧪️oracle/🔣️.json` — removed the stale `no-mutation` entry from
  `mutationManifests[].mutations` and `mutationCatalogs[].kinds`.
- `☁️las@1.0/✳️header/🧪️tests/mutate-las-1-0/🥒️.feature` — removed the 2 stale `no-mutation` Examples
  rows.

`case-above-subset`:
- New: `🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️material/🧪️tests/mutate-obj-3-0-material/{🥒️.feature,🦀️.rs}`
  (relocated + `.rs` rewritten).
- New: `🧊️obj/🏅️standards/🔖️3.0/🪆️subsets/✳️material/🧫️fixtures/🧪️pattern-sphere/🧊️.obj` (copy).
- Removed: `🧊️obj/🧪️tests/mutate-obj-3-0-material/` (old location) and `🧊️obj/🧫️fixtures/` (zero
  consumers left).

`unknown-case-child`:
- Removed: `💠️lowpoly/…/🧪️tests/io-lowpoly-png-1/__pycache__/`.

Scripts (kept in this ticket folder as permanent record): `🩹️f3-add-fixture.py` (the fixture-append
helper every artifact batch used).

## Flagged, not fixed here (spawned as separate tasks)

- `task_c06a8d5c` — `🧊️obj/mutate-obj-3-0-material` needs a real TypeScript adapter using three.js's
  `OBJLoader` for its declared oracle; the case has never actually run at the oracle level.
- `task_1f5f7513` — `🌀️generation2d`'s `update-camera` v1 vector is registered under the wrong
  `mutationId` (`"set-camera"`); the clean fix renames a physical directory and repairs 4 `#[path]`
  mounts in the shared plugin crate, out of scope for an inline fixture-tail fix.

## Answer

- **98 of 98** target `mutation-without-fixture` breaches closed (13 handcrafted-vector artifacts +
  1 real-world zip surgery on a real shared BCF document + 1 stale-declaration removal), all with
  real, schema-honest evidence — never a fabricated vector. Zero new breaches in any class, including
  the physical-vector-bundle classes (`mutation-vector-*`) the v2 route deliberately avoids touching.
- `case-above-subset` is now **0** repo-wide — the underlying copy-paste/dead-code mismatch was
  found and fixed (not guessed at), the case relocated and confirmed owner-correct via
  `test discover`; a separate, deeper oracle/adapter-language gap was found while verifying and
  spawned as its own task rather than attempted inline.
- `unknown-case-child` is now **0** — the stray `__pycache__` removed, already covered by existing
  `.gitignore` rules.
- Before → after: `mutation-without-fixture` 361 → 84 (repo-wide; 98 → 0 in my own territory),
  `case-above-subset` 1 → 0, `unknown-case-child` 1 → 0, every other named class 0 → 0, TOTAL
  breach count 1186 → 905.
- This file:
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION/📓️f3-fixture-tail-and-stragglers.md`.
