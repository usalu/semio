# 📓️ BMP v3 `✳️any` — READER-based external oracle retrofit

Subset: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any`

Followed the AVI 1.0/`✳️any` reference
(`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any`) end to end. This subset had
**no generator, no probes, no fixtures** before this session — everything below was built from scratch.
`🧪️oracle/🦀️component.rs` (the existing `cross-semio-implementation` reference) and the existing oracle
entry `image-bmp-3-mutate` were read only as a spec reference and left **byte-for-byte untouched**.

## 1. What was already there (verified, not assumed)

- `🧪️oracle/🔣️.json` had exactly one oracle, `image-bmp-3-mutate`, `kind: "cross-semio-implementation"`,
  package `image` `0.25`, `productionReachable: true` (with a `productionDebt` naming `🎞️animate`'s video
  engine and `🧰️framework/🔨️modules/🗺️surface`'s tiled-map as reachable-from sites) — left untouched.
- `mutationCatalogs[0].kinds` declares exactly **5** kinds: `change-header-fields`,
  `insert-palette-entry`, `remove-palette-entry`, `replace-palette-entry`, `replace-pixel-data`, all with
  `outcomes: ["applied"]` only.
- Every `oracleRequirements[]` entry had `qualifyingKind: "third-party-library"` but no `oracle` field —
  the gap this session fills.

## 2. Witnessability — investigated against the vendored crate source, not assumed

Read `image`'s actual public BMP decode/encode surface directly in the vendored source
(`~/.cargo/registry/src/index.crates.io-*/image-0.25.10/src/codecs/bmp/{decoder,encoder}.rs`), per the
peer session's request to verify this the same way the PNG retrofit did, rather than trusting the
existing `component.rs`'s own summary of it (that file is a spec REFERENCE only, never a dependency).

- `BmpDecoder`'s public API is exactly: `new`, `new_without_file_header`, `set_indexed_color`,
  `get_palette`, plus the `ImageDecoder` trait's `dimensions`/`color_type`/`read_image`. The struct's
  `top_down`, `bit_count`, `colors_used`, `data_offset` fields are **private with no accessor at all**
  (`decoder.rs:474-491`).
- `get_palette()` returns `Some(&[[u8;3]; …])` **iff** the file's own `image_type` is
  `Palette | RLE4 | RLE8` — decided automatically during `read_metadata()` from the file's real bit
  count, never from calling `set_indexed_color` first (`decoder.rs:844-848`). This is how this session's
  codec tells indexed from direct-colour apart, without hand-parsing a bit-count byte itself.
- `get_palette()` **always zero-pads its return to exactly 256 entries** regardless of the file's real
  declared table length (`decoder.rs:900-935`, comment: "Allocate 256 entries even if palette_size is
  smaller, to prevent corrupt files from causing an out-of-bounds array access"). This has a real
  consequence — see the recipe bug in §3 below.
- `BmpEncoder::encode_with_palette` (`encoder.rs`) **hard-codes**: horizontal/vertical pixels-per-metre
  to `0` (`write_i32::<LittleEndian>(0)` twice, unconditionally), "colours important" to `0`
  (`write_u32::<LittleEndian>(0)`), colour planes to `1`, compression to "none" (for non-V4 headers), and
  **always** stores rows bottom-up (`for row in (0..height).rev()`). None of these round-trip through the
  public API in either direction.

Conclusion: **4 of 5 kinds are witnessable, 1 is not.**

- `insert-palette-entry` / `remove-palette-entry` / `replace-palette-entry` — witnessable. `get_palette()`
  exposes the real table (zero-padded) distinctly from the index buffer, and this subset's own
  semantics ("a palette edit changes the table, leaves the picture alone") are exactly what a
  before/after palette-only diff demonstrates.
- `replace-pixel-data` — witnessable via `image::load_from_memory(..).to_rgb8()`.
- `change-header-fields` — **not witnessable, registered `bmp-3-mutate-uncarried`.** Its own payload
  schema (`../🧬️schema/🧬️mutations/📐️change-header-fields/🔣️payload.schema.json`) covers 12 fields:
  `headerSize`, `width`, `height`, `rowOrder`, `planes`, `bitsPerPixel`, `compression`, `imageSize`,
  `xPixelsPerMeter`, `yPixelsPerMeter`, `colorsUsed`, `colorsImportant`. Of these, only `width`/`height`
  round-trip through `image`'s public API (`dimensions()`); the other 10 are architecturally invisible
  per the source evidence above. This subset's own COMMITTED production test scenario for this mutation
  (`../🧬️schema/🧬️mutations/📐️change-header-fields/🧪️tests/direct-behavior/🦠️mutation/🔣️component.json`)
  exercises `width` together with `xPixelsPerMeter`/`yPixelsPerMeter` — exactly the two fields this
  reader cannot see — so a width-only recipe would dodge the mutation's own real target fields rather
  than honestly witness it. No recipe/fixture was built for this kind.

## 3. A real bug this session's own gate caught (not a hypothetical)

The first `remove-palette-entry-applied` recipe used a 6-entry palette with the removed entry (index 5)
LAST. Because `get_palette()` always zero-pads to 256, removing the final real entry only shifts
zero-padding into its place — an invisible no-op. `bmp-compare` on the real before/after pair returned
`{equal:true, diffCount:0}` — **a false pass**, caught by running the gate on the actual fixture, not
assumed clean. Fixed by adding a second, non-zero spare palette entry (index 6, `[10,20,30]`) after the
removed one, so removal is now provably visible: re-measured
`{equal:false, diffCount:6, diffs:["$.palette[5].r: 0 ≠ 10", "$.palette[5].g: 0 ≠ 20", "$.palette[5].b: 0 ≠ 30", "$.palette[6].r: 10 ≠ 0", "$.palette[6].g: 20 ≠ 0", "$.palette[6].b: 30 ≠ 0"]}`.
This is exactly the pilot playbook's own warning in practice: "a probe handed a carrier that cannot
encode the property must return unsupported, never an empty ok."

## 4. `productionReachable` — verified independently, and a discrepancy found

Per the peer session's flag and the ticket's own instruction not to copy `🔒️dependencies.json` blindly:
grepped every non-test, non-scratch `Cargo.toml` in the repo for its `image` dependency line.

```
🧰️framework/🔨️modules/🗺️surface/…/Cargo.toml:            features = ["png"]
🧰️framework/…/os/…/♾️infinite/…/Cargo.toml:                features = ["png", "jpeg", "webp", "gif"]
🧰️framework/…/os/…/📺️renderer/…/🧊️wgpu/Cargo.toml:        features = ["png", "jpeg"]
✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/Cargo.toml: optional=true, features = ["bmp", "tiff", "jpeg"]
✏️s/🔌️plugins/🎞️animate/…/Cargo.toml:                      features = ["png", "jpeg"]
✏️s/🔌️plugins/🖍️draw/…/Cargo.toml:                          features = ["png"]
```

`image`'s BMP codec is feature-gated behind `bmp` (`image` crate's own `Cargo.toml`: `bmp = []`, and
`#[cfg(feature = "bmp")]` gates the codec module). **None of the five real production dependents enables
it** — only png/jpeg/webp/gif. The only `Cargo.toml` in the repository enabling `image`'s `bmp` feature
outside a scratch ticket directory is the shared test-oracle host crate itself
(`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/Cargo.toml`), which is `optional = true`.

So `image`'s BMP support is genuinely test-only in this repository today. This session's new
`image-bmp-3-mutate-reader` entry is registered `productionReachable: false` on that direct evidence.
**This is a narrower, more accurate claim than the sibling `image-bmp-3-mutate` entry's own
`productionReachable: true`** (with a `productionDebt` naming `🎞️animate`/`🗺️surface` as reachable-from
sites) — that claim is true of the `image` package as a whole (real production dependency, for
png/jpeg/webp/gif), not of its BMP codec specifically, which is what this new entry's capability is
scoped to. Left `image-bmp-3-mutate` exactly as-is per the ticket's own scope; the discrepancy is
recorded in the new entry's own `rationale` field and here, not silently resolved.

## 5. What was built

- **`🏭️generator/🦀️image-bmp-codec/`** — standalone crate, own `[workspace]`, depends on ONLY
  `image = "=0.25.10"` (patch-pinned to the exact version already resolved in
  `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust/Cargo.lock`), `default-features = false`, only the
  `bmp` feature enabled. `cargo build --offline` and `cargo test --offline` both succeed with **zero
  network access**. 4 unit tests, all passing: `every_declared_recipe_id_resolves`,
  `indexed_round_trip_preserves_indices_and_palette`, `direct_round_trip_preserves_solid_fill`,
  `insert_palette_entry_recipe_leaves_indices_untouched`. Two subcommands mirroring `riff-avi-codec`
  exactly: `build <recipe-id> <out-dir>` and `project <path>`. `project` only calls `image`'s own public
  decoder methods — no BITMAPINFOHEADER byte-offset parsing anywhere in this file.
- **`🏭️generator/📜️script.ts`** — `generate [--only <id>]` / `manifests [--only <id>]`, mirrors AVI's
  CLI/recipe shape. Respects `SEMIO_FIXTURE_OUT`. Shells out to the codec; computes no BMP semantics.
- **`🔬️probes/📜️script.ts`** — `bmp-import` / `bmp-project` / `bmp-compare`, `ProbeReport` v2 shape.
  Marshals to the codec's `project` subcommand, hashes the returned hex payload (index buffer or
  resolved RGB buffer) into a size+digest pair in TypeScript with `node:crypto` — exactly the AVI
  probe's own division of labour (Rust emits hex, TS digests) — and performs the GATING structural
  comparison itself.
- **Fixtures**: 4 recipes, all `<kind>-applied`, `before.bmp`+`after.bmp`, real bytes written by
  `image`'s own `BmpEncoder` via the codec's `build` subcommand: `insert-palette-entry-applied`,
  `remove-palette-entry-applied`, `replace-palette-entry-applied`, `replace-pixel-data-applied`. All
  four 4x4 pixels. The three palette recipes share one 7-entry base palette (5 real+referenced colours,
  2 deliberate unreferenced spares) and one 16-index buffer that is **byte-identical between before and
  after** in every palette recipe — the fixture itself demonstrates this subset's "a palette edit
  changes the table, leaves the picture alone" semantics, not just the report claiming it.
- **`🧪️oracle/🔣️.json`** updated additively (Python `json.load`/mutate/`json.dump`, not hand-edited, to
  avoid formatting drift across a 400+ line file):
  - new oracle `image-bmp-3-mutate-reader`, `kind: "third-party-library"`, capability `bmp-3-mutate`, no
    `hostPath`, `testOnly: true`, `productionReachable: false`, 5 platforms.
  - new `comparisonProfiles[1]` (`semantic-bmp-reader-v1`) and `comparisonPipelines[0]`
    (`bmp-3-image-reader-compare-v1`, GATING).
  - new `probes` array, 3 entries (`bmp-import`, `bmp-project`, `bmp-compare`), each with measured
    `qualification.evidence`.
  - `oracle: "image-bmp-3-mutate-reader"` added to 4 mutations' `oracleRequirements[]`; `capability`
    changed to `bmp-3-mutate-uncarried` (no `oracle` field) for `change-header-fields`.
  - `fixtureManifests` — 4 entries appended, `schema: "semio.repository-test.fixture/v2"`,
    `class: "third-party-generated"`.

## 6. Verification — real command output

### `fixture verify --artifact s.stdio.bmp --standard v3 --subset any`

```
[fixture verify] 4 fixture(s), 0 file problem(s)
```

### `fixture reproduce --artifact s.stdio.bmp --standard v3 --subset any --mutation <m>`, run PER MUTATION KIND in a loop (never batched)

```
=== change-header-fields ===
[fixture reproduce] 0 generated fixture(s), 0 problem(s)      (no fixture exists for this uncarried kind — expected)
=== insert-palette-entry ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
=== remove-palette-entry ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
=== replace-palette-entry ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
=== replace-pixel-data ===
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
```

4/4 witnessable fixtures reproduce byte-identically, each regenerated and hash-checked individually.

### Compare probe/gate, demonstrated BOTH ways with real numbers, for all 4 fixtures

Accept — every fixture's own `before.bmp` compared against itself, all four: `{"equal":true,"diffCount":0}`.

Reject — real before/after pairs, diff naming the exact field:

```
replace-palette-entry-applied: {"equal":false,"diffCount":3,"diffs":[
  "$.palette[5].r: 0 ≠ 200","$.palette[5].g: 0 ≠ 150","$.palette[5].b: 0 ≠ 100"]}
replace-pixel-data-applied:    {"equal":false,"diffCount":1,"diffs":[
  "$.pixels.digest: \"sha256:485eaf0f…\" ≠ \"sha256:8917afc0…\""]}
insert-palette-entry-applied:  {"equal":false,"diffCount":16, first 6:[
  "$.palette[2].r: 0 ≠ 128","$.palette[2].g: 0 ≠ 64","$.palette[2].b: 255 ≠ 32", …]}
remove-palette-entry-applied:  {"equal":false,"diffCount":6,"diffs":[
  "$.palette[5].r: 0 ≠ 10","$.palette[5].g: 0 ≠ 20","$.palette[5].b: 0 ≠ 30",
  "$.palette[6].r: 10 ≠ 0","$.palette[6].g: 20 ≠ 0","$.palette[6].b: 30 ≠ 0"]}
```

`bmp-import` returned `bothImport: true` for every fixture's before/after pair.

### `contract` and `matrix` (both repo-wide; no `--artifact`/`--standard`/`--subset` scoping flag actually
narrows either command — confirmed by grepping the full output for other subsets' identically-shaped
lines, not assumed)

`contract` exits non-zero from **1751 pre-existing high-priority breaches** across the whole repo (mesh
subset `manifold3d-three` gaps, PDF/GIF/DXF/etc. oracle-capability mismatches, discovery-baseline
overages) — none touched by this session. Lines actually naming this subset, checked one by one:

- `No runtime inventory has been produced for s.stdio.bmp@v3/any` — pre-existing shape, identical to
  every other stdio artifact's own line in the same run (e.g. `s.stdio.avi@1.0/any`); means the SUBJECT
  side hasn't executed, which needs the currently-broken `semio-s-plugin-stdio` full-workspace build —
  out of scope per this ticket's own briefing.
- `image-bmp-3-mutate-reader is registered as a qualifying third-party oracle, but this owner predicts
  mutation output in its own Rust` — checked against the reference: this exact breach ID fires for
  `riff-avi-1-0-mutate-reader`, `three-gltf-2-0-mutate-reader`, `jszip-bcf-2-1-mutate-reader`,
  `quick-xml-1-0-mutate-reader`, and every other completed sibling reader-oracle retrofit — a known,
  accepted, fleet-wide checker limitation of this exact dual-oracle pattern (one `component.rs` hosting
  both the untouched compute-path registration and the new reader registration), not a regression this
  session introduced.
- 5x `Mutation <kind> is owned by "any" and s.stdio.bmp@v3 declares no narrower subset at all` —
  identical shape to every other single-subset (`✳️any`-only) stdio artifact (confirmed against AVI's own
  13 identically-worded lines in the same run); pre-existing, not introduced here.
- `s.stdio.bmp: mutation change-header-fields requires a third-party-library for capability
  bmp-3-mutate-uncarried, and none is registered` — this is the EXPECTED, by-design shape of the
  `-uncarried` convention itself (confirmed identical for every other subset's own uncarried mutations,
  e.g. `s.mathematical.mathematical`, `s.fem.fem2d`, `s.sequence.sequence`): a capability deliberately
  named so no oracle can ever satisfy it, which is what marks the mutation honestly un-oracled rather
  than fabricating a pass.
- The `📜️component.protocol.semio` "5 mutation kind(s) have no wire record" line and the two other
  artifacts' bmp-export-serializer stub findings are pre-existing and untouched by this session.

`matrix`, filtered for `s.stdio.bmp::*`: only `s.stdio.bmp::change-header-fields` appears in "Which
mutations have no external oracle?" — `insert-palette-entry`/`remove-palette-entry`/
`replace-palette-entry`/`replace-pixel-data` do **not** appear there, confirming all four are now
correctly recognized as oracled. `image-bmp-3-mutate` (untouched) still appears in "Which tests still
use a Semio-derived oracle?"; the new `image-bmp-3-mutate-reader` does not, confirming it registers as
genuinely third-party.

## 7. Files touched (all inside this subset's own tree, plus this report)

- `…/🖼️bmp/…/✳️any/🏭️generator/📜️script.ts` (new)
- `…/🖼️bmp/…/✳️any/🏭️generator/🦀️image-bmp-codec/Cargo.toml` (new)
- `…/🖼️bmp/…/✳️any/🏭️generator/🦀️image-bmp-codec/src/main.rs` (new)
- `…/🖼️bmp/…/✳️any/🔬️probes/📜️script.ts` (new)
- `…/🖼️bmp/…/✳️any/🧫️fixtures/{insert-palette-entry,remove-palette-entry,replace-palette-entry,replace-pixel-data}-applied/{before,after}.bmp` (new, 8 files)
- `…/🖼️bmp/…/✳️any/🧪️oracle/🔣️.json` (updated additively)
- `🧪️oracle/🦀️component.rs` and the `image-bmp-3-mutate` oracle entry — **untouched**, as required.
- This report.

`🦀️image-bmp-codec/target/` is present in the working tree (needed for the offline `cargo build`/
`cargo run` calls above) but is repo-gitignored (`.gitignore:337`), matching `riff-avi-codec`'s own
convention; `Cargo.lock` is tracked, also matching the sibling.

## 8. What remains open / unverifiable

- End-to-end SUBJECT execution (production `s.stdio.bmp@v3/any` mutation dispatch actually run against
  these fixtures) could not be verified: it requires `semio-s-plugin-stdio` to compile as a full
  workspace member, which is currently broken by an unrelated peer's in-flight migration, per this
  ticket's own briefing. Everything above was verified through the standalone codec crate and the
  framework's `fixture`/`contract`/`matrix` tooling directly.
- `change-header-fields` has no reader-oracle coverage at all (registered `-uncarried`) — this is an
  honest gap in mutation-testing coverage for this subset, not a fixable oversight of this retrofit: it
  is a real limitation of `image` 0.25's public BMP API, evidenced at the source level in §2.
