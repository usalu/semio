# 📓️ TIFF 6.0/document — reader-based external oracle retrofit

Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️document` only. Mirrors the
avi `riff-avi-1-0-mutate-reader` pattern (`📓️pilot-playbook.md`, `📓️reproducibility.md`), adapted per
this ticket's TIFF-specific brief. Did not touch png/jpg/bmp, did not edit the existing
`image-tiff-6-0-mutate` (`cross-semio-implementation`) oracle entry or its sibling
`🧪️oracle/🦀️component.rs`, did not touch `.🧬semio` ticket metadata files.

## 1. Investigation — which crate, and what can it actually see

Read the vendored crate source directly (offline, no assumption):
`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/{image-0.25.10,tiff-0.11.3}/src/`.

**`image` 0.25's public TIFF surface is single-IFD-only, exactly as the existing
`🧪️oracle/🦀️component.rs` header claims** — confirmed independently rather than trusted:
`image::codecs::tiff::TiffDecoder::new` reads only the first IFD; `TiffEncoder::write_image`
always emits exactly one. No public tag get/set, no byte-order accessor, no multi-IFD navigation.
This crate alone cannot witness `insert-ifd`/`remove-ifd`/`replace-tag`/`remove-tag`/
`change-byte-order` at all.

**`tiff` 0.11.3 — the crate `image` itself uses for TIFF — has full IFD/tag/byte-order visibility**
on the read side: `Decoder::{more_images, next_image, seek_to_image}` walk the whole IFD chain,
`Decoder::tag_iter()` enumerates every `(Tag, Value)` pair per IFD (including an `unknown(u16)`
tag variant for non-baseline tags), `Decoder::byte_order()` reports the file's own `II`/`MM` mark.
On the write side, `TiffEncoder::{image_directory, extra_directory}` chain arbitrary IFDs and
`DirectoryEncoder::write_tag` writes arbitrary tag/value pairs. This is the exact structural
vocabulary the catalog's 6 kinds need, so the codec depends on `tiff` **directly** — `image` is not
a dependency of it at all (see `🏭️generator/🦀️tiff-ifd-codec/src/main.rs`'s own header for the full
trail). Same composition shape this repo's `avi`/`bcf` oracles already use: a lower, more generic
crate composed for real structural visibility, one layer below the higher-level umbrella crate.

**One genuine write-side platform limitation, found by trying, not by reading a comment:**
`tiff` 0.11.3's encoder hardcodes the byte-order mark to the *compiling target's* native
endianness at compile time — `src/encoder/writer.rs`, `write_tiff_header`/`write_bigtiff_header`,
`#[cfg(target_endian = "little"|"big")]`, never runtime-selectable. Every platform this oracle
targets (`darwin-arm64`, `darwin-x64`, `linux-x64`, `linux-arm64`, `win32-x64`) is little-endian,
so the library can only ever **write** `II` bytes on any of them — it cannot produce an `MM`
"after" fixture for `change-byte-order` without this session hand-swapping bytes outside the
library, which this ticket's fixture rule forbids ("built BY the library, never hand-rolled").
`tiff-ifd-codec build change-byte-order-applied` deliberately refuses (exit 1, explanatory
stderr) rather than fabricate one; verified live:

```
[tiff-ifd-codec] change-byte-order-applied: REFUSED — tiff 0.11.3's encoder hardcodes the
byte-order mark to the compiling target's native endianness at compile time (...). Every
platform this oracle targets (...) is little-endian, so this library can only ever WRITE II
(little-endian) TIFF bytes on them — it cannot produce an MM (big-endian) after.tiff without
this binary hand-swapping bytes outside the library, which this ticket's fixture-authoring
rule forbids. No before.tiff or after.tiff written for this recipe.
exit=1
```

## 2. Witnessable vs uncarried — 5 of 6

| kind | witnessable? | evidence |
|---|---|---|
| `insert-ifd` | yes | `Decoder::more_images`/`next_image` walk the appended IFD; `ifdCount` 1→2 |
| `remove-ifd` | yes | same walk; `ifdCount` 2→1 |
| `replace-tag` | yes | `tag_iter()` shows the changed `ImageDescription` value |
| `remove-tag` | yes | `tag_iter()` shows the tag entirely absent |
| `replace-pixels` | yes | `read_image()` raster digest differs, same dimensions/tags |
| `change-byte-order` | **no — uncarried** | writer-side platform lock (§1); genuinely un-buildable on every declared platform, not merely un-readable |

The 5 witnessable kinds got `"oracle": "image-tiff-6-0-mutate-reader"` added to their existing
`oracleRequirements` entry (kept `qualifyingKind: "third-party-library"` as-is). `change-byte-order`
had its requirement's `capability` renamed `tiff-6-0-mutate-uncarried` with the `oracle` field left
absent — the exact convention already used 116 times elsewhere in the repo (confirmed via
`gltf`'s own `gltf-2-0-mutate-uncarried` entries and, independently, via `bun … contract`'s own
output, which reports the identical `requires a third-party-library for capability
mathematical-1-mutate-uncarried, and none is registered` shape for every other uncarried mutation
in the registry — my one new line matches that shape exactly, not a novel failure mode).

## 3. The five artefacts

1. **`🏭️generator/🦀️tiff-ifd-codec/`** — standalone Cargo crate, own `[workspace]` (isolated from
   root), `tiff = "0.11.3"` only. `cargo build --offline` and `cargo test --offline` both verified
   green using the already-resolved local registry cache (`~/.cargo/registry/src/index.crates.io-…`)
   — no network reached. `build <recipe-id> <out-dir>` writes `before.tiff`[`+after.tiff`];
   `project <path>` emits the typed JSON projection.
2. **`🏭️generator/📜️script.ts`** — `generate`/`manifests` CLI, mirrors avi's shape (`RECIPES` table,
   `spawnSync` into the codec, sha256+bytes per file, `fixtureManifests`-shaped JSON on
   `manifests`). Respects `SEMIO_FIXTURE_OUT`; hashes each file exactly once, never rewrites after.
   `change-byte-order-applied` is deliberately absent from `RECIPES` (§1) rather than attempted and
   silently dropped on failure.
3. **`🔬️probes/📜️script.ts`** — `tiff-import`/`tiff-project`/`tiff-compare`. Only shells out to the
   codec's `project` subcommand and performs an ordered structural diff; computes zero TIFF
   semantics itself.
4. **`🧫️fixtures/<recipe>/{before,after}.tiff`** — 5 recipes (one per witnessable kind), generated
   by running `bun 📜️script.ts generate` (not hand-written). Deterministic: the codec never reads
   wall-clock/process state, and `tiff::encoder::ImageEncoder` writes only constant
   `Rational{1,1}` resolution tags — no `DateTime`/`Software` stamp to canonicalize (the exact TIFF
   re-encode-instability trap `📓️reproducibility.md` calls out did not need a fix here because
   nothing time-derived is ever written).
5. **`🧪️oracle/🔣️.json`** edited in place — added oracle `image-tiff-6-0-mutate-reader`
   (`third-party-library`, `capabilities: ["tiff-6-0-mutate"]`, no `hostPath`, `testOnly: true`,
   `networkDuringExecution: false`, 5 platforms), `probes` (3), `comparisonPipelines`
   (`tiff-6-0-document-ifd-compare-v1`, referencing `semantic-raster-v1` — defined once, globally,
   at `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/🔣️.json`; not redefined locally), the 5+1 `mutationManifests`
   edits from §2, and 5 `fixtureManifests` entries (schema `semio.repository-test.fixture/v2`,
   class `third-party-generated`).

One field-naming note worth recording: the shared `semantic-raster-v1` profile's own `ignoreKeys`
includes `byteLength` — the projection's raster field was named `sampleByteLength` instead of
`byteLength` specifically to avoid that collision (the sibling `component.rs`'s own comment flags
this exact discipline for its own field names).

## 4. `productionReachable` — measured, not copied

Determined honestly for the **new** entry by grepping every non-test, non-oracle `Cargo.toml`
declaring `image` as a real dependency (cross-checked against `🔒️dependencies.json`'s `image`
entry, `users` list) and reading each call site directly:

* `✏️s/🔌️plugins/🎞️animate/📦️packages/🦀️rust/Cargo.toml` — `features = ["png", "jpeg"]`
* `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml` — `features = ["png"]`
* `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml` — `features = ["png"]`
* `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml` — `features = ["png","jpeg","webp","gif"]`
* `…/📺️renderer/…/🎯️targets/🧊️wgpu/Cargo.toml` — `features = ["png","jpeg"]`

**None enables `image`'s `tiff` feature**, and reading the two call sites the existing
`image-tiff-6-0-mutate` entry's own `productionDebt.reachableFrom` names
(`🎞️animate/…/🎥️video/🦀️component.rs`, `🗺️surface/🗺️tiled-map/🦀️component.rs`) directly confirms
neither imports `image::codecs::tiff` or decodes TIFF bytes — both use `image` only for PNG/RGBA
buffers. So `image` the crate is production-reachable for other raster formats, but **`image`'s
TIFF support specifically is not**, and this new entry depends on `tiff` directly, which appears
nowhere outside this session's own oracle crate. `productionReachable: false` is correct and does
not contradict the sibling `image-tiff-6-0-mutate` entry's own `productionReachable: true` — both
are true simultaneously, about different packages/capabilities; this coexistence is spelled out in
the new entry's own `rationale` field so a future reader doesn't need to re-derive it.

## 5. Gate qualification — measured both ways, real output

Known-good pair (byte-identical):
```
$ bun 🔬️probes/📜️script.ts tiff-compare --input …/replace-tag-applied/before.tiff --input …/replace-tag-applied/before.tiff
equal: True diffCount: 0
```

Known-bad pair (one deliberate field change, the exact recipe already in the corpus):
```
$ bun 🔬️probes/📜️script.ts tiff-compare --input …/replace-tag-applied/before.tiff --input …/replace-tag-applied/after.tiff
equal: False diffCount: 1
diffs: ['$.ifds[0].entries[5].value.value: "original scan" ≠ "rescanned copy"']
```

Additional real measurements, quoted (not summarized):
* `tiff-import` both files: `bothImport: True`.
* `insert-ifd-applied` after.tiff project: `ifdCount: 2, tagCounts: [12, 12]`.
* `insert-ifd-applied` before vs after: `equal: False, diffCount: 2` (ifdCount differs at `$.ifdCount` and the appended IFD's own array slot).
* `replace-pixels-applied` before vs after: `equal: False, diffCount: 1`, naming `$.ifds[0].raster.samplesDigest`.
* `remove-tag-applied` before vs after: `equal: False, diffCount: 23` — this is the expected cascade of the ordered/positional array-diff model when an entry is removed from the MIDDLE of a sorted array (every later tag's index shifts by one, so every later slot reports a mismatch too) — the same characteristic the sibling avi probe's own `diffAt` would show for a middle-removal recipe; not a defect, and not the case the single-field qualification evidence above is meant to demonstrate (that's `replace-tag`'s job, deliberately chosen because it changes exactly one value at a fixed array position).

## 6. `fixture verify` / `fixture reproduce` — real numbers

```
$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture verify --artifact s.stdio.tiff --standard 6.0 --subset document
[fixture verify] 5 fixture(s), 0 file problem(s)
```

Per-mutation `fixture reproduce` (6 separate invocations would be needed for a full corpus;
`change-byte-order` has no fixture to reproduce, so 5 were run, never one whole-corpus double-run
per `📓️reproducibility.md`'s own lesson about order-dependent state hiding in a batched check):

```
insert-ifd:      [fixture reproduce] 1 generated fixture(s), 0 problem(s)
remove-ifd:      [fixture reproduce] 1 generated fixture(s), 0 problem(s)
replace-tag:     [fixture reproduce] 1 generated fixture(s), 0 problem(s)
remove-tag:      [fixture reproduce] 1 generated fixture(s), 0 problem(s)
replace-pixels:  [fixture reproduce] 1 generated fixture(s), 0 problem(s)
```

**5/5 fixtures verify, 5/5 reproduce byte-identically, per-mutation.**

## 7. `contract` / `matrix` — attempted, real output, honest interpretation

Both commands run **repository-wide** regardless of the `--artifact`/`--standard`/`--subset` flags
passed (confirmed: the output enumerates dozens of unrelated artifacts — mesh, mathematical, fem,
pdf, …) — they are not scoped tools despite their CLI surface suggesting otherwise, so a "tiff-only"
signal has to be extracted by filtering their output rather than by trusting the flags.

`contract` exits 1. TIFF-relevant lines, each checked individually:

* `Oracle image-tiff-6-0-mutate does not declare capability tiff-6-0-mutate` — **pre-existing**,
  confirmed via `git log` on the referencing feature file (`🧪️tests/mutate-tiff-6-0/🥒️.feature`,
  last touched at commit `f7b265d581`, before this session) and by inspection: it refers to the
  *existing* `image-tiff-6-0-mutate` entry (declares `tiff-6-0-mutate-second-implementation`, not
  `tiff-6-0-mutate`) — an entry this session was explicitly told to leave untouched.
* `Unknown mutation catalog @mutations-tiff-6-0-any` / catalog `tiff-6-0-baseline` issues —
  **pre-existing**, a different subset (`✳️baseline`, not `✳️document`) and a different catalog id
  (`…-any`) than anything this session touched.
* `Mutation catalog tiff-6-0-document (6 kinds) is claimed by no feature` — **pre-existing**; this
  catalog (id, 6 kinds) already existed verbatim before this session started, unmodified by it.
  `No runtime inventory has been produced for s.stdio.tiff@6.0/document` — **expected**, per this
  ticket's own brief: `semio-s-plugin-stdio` does not compile repo-wide right now.
* `mutation change-byte-order requires a third-party-library for capability
  tiff-6-0-mutate-uncarried, and none is registered` — the **one new TIFF-relevant line**, and it
  is the expected, by-design shape for a deliberately-uncarried mutation (§2; matches 116 identical
  occurrences elsewhere in the same `contract` output for other subsets' `-uncarried` entries).
* Remaining ~1700 lines of `contract` output are mesh/`manifold3d-three`, `🧰️framework`/
  `♻️mit-bestand`/`✏️s` test-discovery-baseline counts, and two unrelated serializer stub findings in
  `🎥️shooting`/`🖨️raster` — none reference this session's paths.

`matrix` exits 0. TIFF-relevant: `fixtureProvenanceCoverage 100.00%`, `fixtureReproducibilityCoverage
100.00%` (both repo-wide totals unaffected downward by this session's 5 additions — they're
counted IN the 100%). `s.stdio.tiff@6.0/document` appears under "no runtime inventory" (expected,
stdio doesn't compile) and "no real-world fixture" (true and out of scope — deriving a real-world
multi-page scan fixture, as `🧪️oracle/🦀️component.rs`'s own `#[ignore]`d
`derive_real_world_fixture` test already does for the *other* oracle, was not part of this ticket).
`dependencyIsolationCoverage 99.60% (247/248)` — traced to source
(`🧰️framework/…/🧪️test/📦️packages/🟦️typescript/📦️index.ts`): this dimension counts an oracle/probe
as "leaky" only when it declares `productionReachable: true` with **no** `productionDebt` block;
the new `image-tiff-6-0-mutate-reader` entry declares `productionReachable: false` (not leaky by
construction) and the existing `image-tiff-6-0-mutate` entry already carries a `productionDebt`
block (also not leaky). The 1/248 gap is therefore some other, pre-existing repo-wide entry,
unrelated to this session — not investigated further as out of scope for a TIFF-only retrofit.

## 8. What could not be verified, and why

`contract`/`matrix` cannot give a clean isolated "TIFF subsystem passes" signal because
`semio-s-plugin-stdio` does not compile repository-wide (a known, pre-existing, unrelated-peer-
migration state this ticket explicitly says not to try to fix) and because both commands measure
the whole repository regardless of the scoping flags. What COULD be verified directly and was: the
standalone codec crate builds and its own unit tests pass fully offline; the generator produces
byte-identical, reproducible fixtures; `fixture verify`/`fixture reproduce` (the platform's own
per-fixture checks, which do NOT depend on the plugin compiling) both pass 5/5; the probes'
`tiff-import`/`tiff-project`/`tiff-compare` all produce the expected real output against the real
generated fixtures, including both a passing and a failing gate case with an exact field-level
diff.

## Summary

* **Fixtures:** 5/5 generated, verified, and reproduced byte-identically per-mutation.
* **Witnessable / uncarried split:** 5 witnessable (`insert-ifd`, `remove-ifd`, `replace-tag`,
  `remove-tag`, `replace-pixels`) / 1 uncarried (`change-byte-order` — writer-side platform lock in
  `tiff` 0.11.3's encoder, not a reader-visibility gap).
* **Gate:** accepts a known-good pair (`equal:true, diffCount:0`) and rejects a known-bad pair
  naming the exact field (`equal:false, diffCount:1`, `$.ifds[0].entries[5].value.value`).
* **`contract`/`matrix`:** run repo-wide, not scoped; every TIFF-relevant line traced individually —
  all pre-existing except one expected `-uncarried` line matching 116 existing precedents.
