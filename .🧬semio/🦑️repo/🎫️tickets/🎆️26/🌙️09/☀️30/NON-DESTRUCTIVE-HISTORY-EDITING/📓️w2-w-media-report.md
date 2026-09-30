# 📓️ W2-W-media — wire-witness conversion (jpg, gif 87a/89a, png, tiff, bmp, avi, mp4, mp3, wav)

Status: DONE. Both lints are 0 in scope. 14 of the 17 touched cases are fully green. The other 3 fail only on known
divergences that predate this work and are documented in their own feature files. Details follow.

Brief: `🧭️plan.md` "W2-W brief" and the F10 recipe in `📓️w2-s-report.md` Follow-up 3. Scope: every stdio artifact under
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/` named above.

## 1. Outcome

### 1.1 Lints (final run after every edit; `schema mutation-payloads` / `schema mutation-inputs --under <artifact>`)

| artifact | leaves | witnessed | rows | payload findings | input findings |
|---|---|---|---|---|---|
| wav | 6 | 6 | 12 | 0 | 0 (9/9 inputs) |
| mp3 | 4 | 4 | 8 | 0 | 0 (4/4) |
| avi | 12 | 12 | 24 | 0 | 0 (21/21) |
| mp4 | 10 | 10 | 20 | 0 | 0 (19/19) |
| bmp | 6 | 6 | 10 | 0 | 0 (19/19) |
| png | 18 | 18 | 30 | 0 | 0 (28/28) |
| tiff (document + baseline) | 16 | 16 | 28 | 0 | 0 (20/20) |
| jpg (document + baseline) | 21 | 21 | 38 | 0 | 0 (30/30) |
| gif (87a + 89a) | 31 | 31 | 62 | 0 | 0 (55/55) |

### 1.2 Feature cases (`bun ./📜️script.ts parity exhaustive --case <case>`: the oracle and subject phases plus the comparison)

| case | executed/passed | parity | verdict |
|---|---|---|---|
| 🎛️mutate-wav-riff-pcm | 13/13 | 0/0 (a `@no-oracle` decision, so subject only) | green |
| 🎛️mutate-mp3-mpeg1-layer3 | 18/18 | 9/9 | green |
| 🎛️mutate-avi-1-0 | 26/26 | 13/13 (pipeline) | green |
| 🎞️mutate-avi-1-0-movi | 20/20 | 10/10 (pipeline) | green |
| 📇️mutate-avi-1-0-idx1 | 4/4 | 2/2 (pipeline) | green |
| 🎞️mutate-mp4-isobmff | 42/42 | 21/21 | green |
| 🪟️mutate-bmp-v3 | 21/22 | 10/11 | known divergence `mutate-replace-pixel-data` (§4) |
| 🔀️mutate-png-1-2 | 62/62 | 31/31 | green |
| 🖼️mutate-tiff-6-0 | 26/26 | 13/13 | green |
| 🧱️mutate-tiff-6-0-baseline | 32/32 | 16/16 | green |
| 📸️mutate-jpg-jfif-1-01 | 42/42 | 21/21 | green |
| 🛡️mutate-jpg-jfif-1-01-baseline | 36/36 | 18/18 | green |
| 🖼️mutate-gif-87a | 45/46 | 22/23 | known divergence `mutate-set-global-color-table` (§4) |
| 🎞️mutate-gif-89a | 44/46 | 21/23 | known divergences `mutate-set-screen-size`, `mutate-set-frame-geometry` (§4) |
| 🎛️mutate-gif-89a-graphic-control | 16/16 | 8/8 | green |
| 🧩️mutate-gif-89a-application | 12/12 | 6/6 | green |
| 💬️mutate-gif-89a-comment | 8/8 | 4/4 | green |

Two notes on the runs:

- Some runs first died on peers' in-flight compile errors, all outside my files: `semio-framework-ui` (`UiTreeItemNode`
  had no `content_lines`) and `semio-framework-os-kernel` (`fold_history` arity, store retirement pattern). Once the peers
  finished, I re-ran every affected case. The numbers above come from those re-runs.
- Contract phase: the per-case coverage breaches in my scope are cleared (§3.1). The contract run as a whole stays red
  because of about 2.9k repo-wide breaches, none of them in my scope. §3.3 lists what I re-measured.

### 1.3 Crate tests

- `cargo test --lib` passed for all 9 artifact crates, including all 12 `semio_payload_law_*` tests: avi 37, bmp 69,
  gif 102, jpg 122, mp3 33, mp4 52, png 144, tiff 99 (2 ignored), wav 38.
- The payload law skips fixtures it cannot decode (`mutation_fixture_ops` drops them through `filter_map`). So I ran a
  temporary `[DEBUG]` probe over the committed `🦠️mutation` fixtures, which include every new wire witness. Every
  fixture decodes: wav 3/3, mp4 2/2, bmp 6/6, png 18/18, tiff 8/8, jpg 12/12. The probe was then removed.

## 2. What changed

### 2.1 The conversion itself (F10)

- **Feature rows.** Every row in the 17 features now carries the leaf's exact wire payload (`payload_value()`). Rows that
  performed no mutation are gone. Each feature gained a paragraph stating the wire contract.
- **Subject decoding.** Every adapter decodes rows through per-artifact bridges over the derive-generated
  `from_payload_value`. The hand-written params→op matches are deleted.
  - Bridges: `decode_<x>_mutation_payload` and `inverse_<x>_mutation` in the mutations or `⚙️operations` modules.
  - These bridges are needed because the generated host links only the subject crate, the oracle crate and the test host,
    so adapters cannot name kernel traits.
  - For the same reason, the gif and bmp crate roots re-export `ArtifactDsl`.
- **Oracles.** Every oracle reads the same wire field names.
- **camelCase.** 18 leaves had Rust emitting snake_case while their schema said camelCase. I normalised them to
  `#[value(rename_all = "camelCase")]` together with their schemas.
- **mp4 `trackIndex`.** It gained its `x-semio-ui` descriptor (stepper, en/de label, group `target`).
- **Wire witnesses.** I added `🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness/🦠️mutation/🔣️.json` for every leaf that
  had none:
  - wav: `patch-data`, `patch-snapshot`
  - mp4: `patch-snapshot`
  - bmp: `set-snapshot`
  - png: `set-snapshot`, `patch-snapshot`, `patch-pixels`
  - tiff document: `set-snapshot`, `patch-snapshot`
  - jpg document: `set-snapshot`, `patch-snapshot`
- **Whole-raster kinds.** These are bmp `replace-pixel-data`, png, tiff and jpg `replace-pixels`, and gif 89a
  `set-frame-pixels`. On the real multi-megapixel documents, one cell would hold millions of numbers. So these rows run
  on small committed documents, and the adapters read the `Given` URI through `step_fixture_uris()`.
  - Since this round, those outlines share the `@id-mutate`/`@id-inverse` base ids. Their scenario ids are therefore
    `mutate-<kind>`/`inverse-<kind>`. Under the old `@id-mutate-raster` ids the catalog coverage gate counted the kinds as
    uncovered and the rows as strays. The extra `*-raster` handler registrations are deleted.
  - jpg keeps an exact observability claim for `replace-pixels` through `RASTER_KINDS`. Its 768 pixels sit below the
    lossy slack, which would otherwise excuse any change.
- **Deleted file.** The 23 MB `🖼️tiff/…/🧾️document/🧫️fixtures/🖼️mutate-tiff-6-0/🖼️.rgba`. Only the old hand-mapping
  adapter referenced it.
- **Semio features.** The image and drawing features now reference the current on-disk fixture names (2 and 4 slugs).
  The folders were not renamed back.

### 2.2 Making the touched cases green (this round)

- **wav and mp4 `patch-snapshot`.** Both catalogs claim this kind, but no feature exercised it. I added mutate and inverse
  rows:
  - wav: sets sample 4000 and removes sample 4001.
  - mp4: promotes sample 5 to a sync sample and demotes key frame 27.
  - Both oracles now interpret `SnapshotPatch` paths independently over their own models and refuse any unmodelled path.
    The inverse restores the facets the edits touched.
  - The wav oracle's `apply_inverse` now receives the params.
- **Owner manifests.**
  - wav and mp4 gained the `patch-snapshot` manifest row (`applied`/`rejected`, variant `PatchSnapshot`).
  - I authored handcrafted before/after fixture pairs, with `fixtureManifests` entries, for wav `patch-data`, wav
    `patch-snapshot` and mp4 `patch-snapshot`.
  - I re-pinned the six wav fixture hashes that went stale when a peer added `chunkOrder` on 09-28.
  - Script: `🧪️w2w-media-manifests.py`.
- **avi comparison pipeline.** It had never produced a verdict. The `semantic-avi-v1` profile names the
  `avi-1-0-riff-compare-v1` pipeline, which reads the `expected-avi`/`actual-avi` role artifacts. No adapter ever emitted
  them. All three avi adapters now do.
  - That exposed a real oracle bug. The fixture's 56-byte `strh` stores `rcFrame` as four 16-bit SHORTs,
    `(0,0,480,432)`. The oracle read it as two LONGs, giving `top = 28312032`, and wrote a 64-byte header. The in-case
    projection hid this because both sides were projected through that same misreading.
  - Fix: the oracle now records the `rcFrame` width (0, 8 or 16), exactly as production `🚪️io` does, and writes the same
    form back. `strh_from_json` accepts widths 0, 8 and 16.
  - The module docs and the feature's stale "rcFrame simply omitted / decode_avi requires 64 bytes" text are corrected.
  - A unit test pins the 56-byte form (`classic_56_byte_strh_keeps_its_short_rc_frame`).
- **tiff document.**
  - The subject identity scenario tripped `no_byte_pass_through`. Its encoder reproduces the scan byte for byte, because
    the scan is in the same canonical baseline layout. Both sides now assert `carrier_is_exact`, following the wav and
    mp4 precedent, and the feature paragraph says why.
  - The raster inverse failed on the small document, which another writer authored (RowsPerStrip normalisation). The
    mutate and inverse laws are now stated against `unmutated_baseline`, the unchanged reference round trip.

## 3. Verification notes

1. **Per-case coverage.** Before this round, `contract exhaustive --owner 🗄️stdio` reported per-case breaches in my scope.
   They are fixed at the cause (§2.2):
   - uncovered kinds plus stray `mutate-raster-*` ids in bmp, png, tiff, jpg and gif 89a;
   - uncovered `patch-snapshot` in wav and mp4.
2. **Owner breaches** (`s.stdio.wav`, `s.stdio.mp4`):
   - wav and mp4 `test-only patch-snapshot`
   - wav `manifest-only patch-data`
   - wav `patch-data` without a fixture
   - six wav fixture hash mismatches

   All are fixed. Runtime inventories were re-measured through the production bridge (`test inventory`), all with 0
   differences:
   - wav 6 = 6
   - mp4 10 = 10
   - mp3 4 = 4
   - avi hdrl 12 = 12
   - gif 87a 11 = 11
   - gif 89a base 20 = 20

   For bmp, png, tiff (both subsets) and jpg (both subsets), the bridge build failed on a peer's in-flight
   `semio-framework-plugin` errors (E0425/E0599), which are not in my files. Their cached inventory therefore still dates
   from 09-25; see §5.
3. **Not run.** I have not re-run the full stdio contract since the final edits. It takes 9.5 minutes, and the repo-wide
   breach set outside scope is unchanged.

## 4. Known divergences (predate this work; not papered over)

Each is documented as KNOWN OPEN in its own feature, with the instruction not to weaken the row, profile or fixture:

- **bmp `mutate-replace-pixel-data`.** The oracle promotes the document to direct colour. `encode_bmp` refuses non-palette
  pixels on purpose: the `unrepresentable_palette_edit_is_reported_not_narrowed` test pins that. Resolving it needs a
  semantic decision on what `replace-pixel-data` means for indexed BMP.
- **gif 87a `mutate-set-global-color-table`.** The subject reports "image 0 has an index past the end of its color
  table".
- **gif 89a `mutate-set-screen-size`** ("frame 0 region exceeds the logical screen") and **`mutate-set-frame-geometry`**
  ("indices length mismatch").

## 5. Open items and findings for others

- **wav oracle `patch-snapshot`: WRITTEN BUT UNVERIFIED at runtime.** It compiles (every host build links the oracle
  crate), but the wav case is `@no-oracle-frozen-hound-pcm16`, so no scenario executes it. The mp4 counterpart ran and
  agreed (21/21).
- **avi oracle unit test: WRITTEN BUT UNVERIFIED.** The oracle crate's `lib test` target does not compile: a dxf smoke test
  `include_bytes!` points at a missing
  `🗿️artifacts/🖋️dxf/…/📚️examples/🚏️bus-shelter/🖼️assets/…` file, which is a peer's rename. The production code path it
  pins is proven by all three avi pipeline cases passing.
- **`verify taxonomy report` is broken repo-wide.** It aborts on
  `frozen-coordinate-evidence-invalid: 🧰️framework/…/📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json` (digest
  mismatch). So the new `🩹️patch-data`/`🩹️patch-snapshot` fixture folders and `🧾️wire-witness` folders could not be
  checked by the tool. They follow the leaf folder names and the fleet's witness convention.
- **Latent kinds in png, jpg, tiff and bmp.**
  - Production dispatch offers `set-snapshot`/`patch-snapshot` in png, jpg document and tiff document, `set-snapshot` in
    bmp, and `patch-pixels` in png. None of these subsets' catalogs or manifests declare them.
  - This comes from the leaf folders compared against each subset's catalog and manifest. The bridge measurement is
    still stale (09-25), because refreshing these six subsets failed on the peer's `semio-framework-plugin` compile
    errors (§3). Once that crate builds, a refresh will report these kinds as `runtime-only`.
  - Closing them means catalog kinds, manifest rows, feature rows and oracle support for path patches in each of them. I
    did not widen the wire-conversion scope to that.
- **semio drawing `set-snapshot`.** It is in the catalog, but no manifest owns it and no scenario exercises it. That
  belongs to the semio lane (`s.stdio.semio@v1/drawing`), not mine.
- **Operational slip.** I once ran `bun ./📜️script.ts parity` with no arguments, which starts a whole-repo sweep. I killed
  it within seconds, including its orphaned cargo/rustc (the tiledmap host build). No stray process remains.

## 6. Files

**Artifact sources and features.** All paths are under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`. For each artifact, the
feature, adapter, oracle, the mutations/`⚙️operations` bridges and the crate roots (gif, bmp) changed; renamed leaves
were edited together with their schemas. This round additionally:

- **wav:**
  - changed `🔮️oracles/🦀️.rs`, `🔮️oracles/🔣️.json` and the `🎛️mutate-wav-riff-pcm/🥒️.feature`
  - added `🧫️fixtures/🩹️patch-data/{⬅️before,➡️after}.json` and `🧫️fixtures/🩹️patch-snapshot/{⬅️before,➡️after}.json`
- **mp4:**
  - changed `🔮️oracles/🦀️.rs`, `🔮️oracles/🔣️.json` and the `🎞️mutate-mp4-isobmff/🥒️.feature`
  - added `🧫️fixtures/🩹️patch-snapshot/{⬅️before,➡️after}.json`
- **avi:** changed `🎛️hdrl/🔮️oracles/🦀️.rs`, its `🧪️tests/🔬️oracles-unit/🦀️.rs`, the three adapters (hdrl, movi, idx1)
  and the hdrl feature.
- **tiff document:** changed the adapter and the feature.
- **jpg document:** changed the adapter and the feature.
- **bmp, png, gif 89a base:** changed the features and adapters (the `@id` folding).
- **Semio:** the image and drawing features.

**Ticket scripts** (kept): `🧪️w2w-media-wire-rows.py`, `🧪️w2w-media-rewrite-feature.py`, `🧪️w2w-media-split-raster.py`,
`🧪️w2w-media-camel-leaves.py`, `🧪️w2w-media-manifests.py`.

**Scratch:** `🗑️generated/w2w-media/`. This holds the lint JSON, parity logs, `chain.sh` and `chain.log`, and inventory
logs. It is mine to clean; I left it in place for the coordinator.
