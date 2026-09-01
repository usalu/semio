# 📓️ gif@87a/✳️any — third-party-generated fixture corpus, mutation manifest, gate validation

Scope: close the ONE remaining gap on
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any` — it already had a
qualifying third-party oracle (`gif-87a-mutate`, `gif` 0.13, `semantic-raster-v1`) and a 12-kind
`mutationCatalogs` block, and had **zero** fixtures, no `mutationManifests`, no `fixtureManifests`
and no `🏭️generator/`. The oracle choice was NOT re-litigated and is unchanged.

Precedent followed field-for-field: the sibling `🔖️89a/🪆️subsets/✳️any`, one standard up, same crate.

## Delivered

```
🏭️generator/📜️script.ts                    bun wrapper: generate | manifests, honours SEMIO_FIXTURE_OUT as a fixtures ROOT
🏭️generator/🦀️engine/Cargo.toml            own [workspace]; one dependency, `gif = "0.13"` (resolves 0.13.3)
🏭️generator/🦀️engine/src/main.rs           writes the fixture with the REGISTERED crate's own encoder
🏭️generator/🦀️engine/Cargo.lock            committed (gif 0.13.3, weezl 0.1.12, color_quant 1.1.0)
🧫️fixtures/mosaic-strip/mosaic-strip.gif   129 bytes, sha256:b7237377b1e8cb6181cd1c38aeba08f06eddb53b45d38c426cdcd73f71c3e94e
🧪️oracle/🔣️.json                           + mutationManifests (1 manifest, 12 mutations) + fixtureManifests (1 fixture)
```

Nothing under `🔖️89a/` was touched (read-only reference). `fixture verify --standard 89a` still
reports `1 fixture(s), 0 file problem(s)` afterwards.

## The fixture, and why one is enough

`mosaic-strip.gif` — 6×4 logical screen, 8-colour Global Color Table, background colour index **3**,
pixel aspect ratio **49**, three images:

| # | rect | palette | interlaced | indices |
| --- | --- | --- | --- | --- |
| 0 | 0,0 4×4 | global | no | `i % 4` over 0..16 |
| 1 | 2,0 4×4 | own 4-grey Local Color Table | **yes**, four mutually distinct rows | rotating 0..3 |
| 2 | 1,1 2×2 | global | no | `5,6,7,4` — the only image reaching the table's second half |

Nothing GIF87a lacks appears: no delay, disposal, transparency, user-input flag, loop count, comment
or application extension. Those are the eight kinds 89a has and 87a does not, exactly as this
subset's own oracle rationale states.

`📓️gif-las-pdf17-findings.md` proved from `measureCoverage`'s source that
`oracleEvidenceCoverage`/`subsetOwnershipCoverage`/`externalOracleCoverage`/`fixtureProvenanceCoverage`
key on `${artifact}@${standard}/${subset}` appearing among fixture targets — ANY fixture targeting the
subset counts for EVERY mutation in it. Re-checked here against the real matrix (below): one base
fixture suffices, and the witnessability check below is what makes it a good one rather than merely a
sufficient one.

### The three header bytes `gif::Encoder` writes as constants

`gif` 0.13.3 `src/encoder.rs:338-347` — `write_screen_desc` emits `b"GIF89a"`, then `0u8 // bg index`
and `0u8 // aspect ratio`, from constants, never from its input; the crate exposes no setter for any
of them. The generator patches byte 4 to `'7'`, byte 11 to the background index and byte 12 to the
aspect ratio — the same technique `🧪️oracle/🦀️component.rs`'s `oracle_encode` uses, and the reason the
fixture deliberately carries **non-zero** values for both scalars: with zeros, a dropped patch would
be invisible and `set-background-color-index`/`set-pixel-aspect-ratio` would report as passing while
being silently discarded.

Producing the bytes the same way the oracle re-serializes them is also what makes `no-mutation`'s
declared `byte-identical-round-trip` invariant **true** rather than merely asserted — measured below.

The fourth constant `oracle_encode` handles (the phantom two-entry Global Color Table `Encoder::new`
writes for an empty palette) is deliberately not exercised by this fixture, which declares a real
8-colour table; that branch belongs to `set-global-color-table {gct: null}`, not to the base document.

## Witnessability — all 12 of 12, measured

Per playbook Step 1/5, checked per kind against the real `project_gif_87a`, not assumed from the 89a
precedent. Driver: `🔬️gif-87a-any-verify/` (own `[workspace]`, links the committed shared oracle crate
by path and calls only `oracle_apply_mutation`/`oracle_inverse_spec`/`project_gif_87a`), judged by the
FRAMEWORK'S OWN `compareProjections` under the real `semantic-raster-v1` profile from the loaded
registry — no comparison was written for this wave.

Base projection, as the oracle actually reports it:

```json
{"format":"gif87a","width":6,"height":4,"gctColors":8,"gctDigest":"2e3eab5a1426d7c5","backgroundColorIndex":3,"pixelAspectRatio":49,"imageCount":3,
 "images":[{"left":0,"top":0,"width":4,"height":4,"interlaced":false,"lctColors":0,"lctDigest":"cbf29ce484222325","indexDigest":"0a447104407b5355"},
           {"left":2,"top":0,"width":4,"height":4,"interlaced":true,"lctColors":4,"lctDigest":"7d3ea5d44a13aea5","indexDigest":"16ab1377049c757d"},
           {"left":1,"top":1,"width":2,"height":2,"interlaced":false,"lctColors":0,"lctDigest":"cbf29ce484222325","indexDigest":"61432a560c39ffd1"}]}
```

Real output:

```
[kind] no-mutation                bytes= 129 byteIdentical=true  movedDiffs= 0 inverseRestoresProjection=true  inverseRestoresBytes=true
[kind] set-snapshot               bytes= 129 byteIdentical=false movedDiffs= 3 inverseRestoresProjection=true  inverseRestoresBytes=true  moved: backgroundColorIndex, height, width
[kind] set-screen-size            bytes= 129 byteIdentical=false movedDiffs= 2 inverseRestoresProjection=true  inverseRestoresBytes=true  moved: height, width
[kind] set-global-color-table     bytes= 117 byteIdentical=false movedDiffs= 2 inverseRestoresProjection=true  inverseRestoresBytes=true  moved: gctColors, gctDigest
[kind] set-background-color-index bytes= 129 byteIdentical=false movedDiffs= 1 inverseRestoresProjection=true  inverseRestoresBytes=true  moved: backgroundColorIndex
[kind] set-pixel-aspect-ratio     bytes= 129 byteIdentical=false movedDiffs= 1 inverseRestoresProjection=true  inverseRestoresBytes=true  moved: pixelAspectRatio
[kind] insert-image               bytes= 165 byteIdentical=false movedDiffs=18 inverseRestoresProjection=true  inverseRestoresBytes=true  moved: imageCount, images, images[1].*, images[2].*, images[3]
[kind] remove-image               bytes=  89 byteIdentical=false movedDiffs=11 inverseRestoresProjection=true  inverseRestoresBytes=true  moved: imageCount, images, images[1].*, images[2]
[kind] move-image                 bytes= 129 byteIdentical=false movedDiffs=18 inverseRestoresProjection=true  inverseRestoresBytes=true  moved: images[0..2].*
[kind] set-image-geometry         bytes= 129 byteIdentical=false movedDiffs= 2 inverseRestoresProjection=true  inverseRestoresBytes=true  moved: images[2].left, images[2].top
[kind] set-image-pixels           bytes= 131 byteIdentical=false movedDiffs= 1 inverseRestoresProjection=true  inverseRestoresBytes=true  moved: images[0].indexDigest
[kind] set-image-interlace        bytes= 129 byteIdentical=false movedDiffs= 1 inverseRestoresProjection=true  inverseRestoresBytes=true  moved: images[0].interlaced

[summary] 12 kind(s); 0 not behaving as declared; 0 inverse law failure(s)
```

**12 of 12 witnessable, 0 uncarried exemptions.** Every kind moves the compared surface (`no-mutation`
deliberately does not, and is byte-identical), and every kind's oracle-computed inverse restores both
the projection AND the exact 129 bytes. `set-image-geometry` was aimed at image 2 with the rectangle
kept the same size on purpose: `gif::Encoder` rejects a frame whose buffer is smaller than
`width × height`, so a geometry case that also grows the rectangle would be testing the encoder's
input validation rather than the mutation.

One thing worth pinning, because it is a near-miss rather than a design: `semantic-raster-v1`'s
`ignoreKeys` contains **`interlace`**, and `project_gif_87a` emits the key **`interlaced`**. Had the
projection used the format's own spelling, `set-image-interlace` would have been silently canonicalized
away and would have reported as a mutation nothing can see. Measured, it moves 1 diff.

## The gate, both ways, with real numbers

Playbook Step 3: a gate only ever tested on good input is not a gate.

```
[gate accept] fixture vs itself: equal=true diffs=0
[gate reject] fixture vs set-image-pixels-aimed-at-image-2: equal=false diffs=1
[gate reject]   $.images[2].indexDigest: oracle="61432a560c39ffd1" subject="394d8a667dcb4f41" (values differ)
[gate reject] fixture vs global-color-table-byte-13-flipped: equal=false diffs=1
[gate reject]   $.gctDigest: oracle="2e3eab5a1426d7c5" subject="5c5f4323dc55c05a" (values differ)
```

Both wrong documents are genuine content differences, not fixtures invented to look different: the
first is this subset's own `set-image-pixels` applied to the WRONG image (index 2 instead of 0), the
second is the committed file with one Global Color Table byte flipped in place. Accept: 0 diffs.
Reject: 1 diff each, at the exact member the corruption lives in.

## Outcome classes — read off the code, not the doc comments

Playbook Step 2. `🧬️schema/🧬️mutations/🦀️.rs` has exactly two `MutationOutcome` call sites:
`MutationOutcome::new(match self { … })` (line 138, one arm per variant, uniform) and
`MutationOutcome::error(…)` (line 128, the `MutationDiff::apply` failure path, common to all kinds).
No `::empty`, no `::fatal` anywhere in the subset. The one per-kind exception is `set-snapshot`'s own
leaf (`🧬️mutations/📄set-snapshot/🦀️.rs:19`), which returns
`MutationOutcome::new(GifDiff::default()).warn("mutation.no-op", …)` when the replacement snapshot is
identical.

So: `["no-op"]` for `no-mutation` (its diff is unconditionally `GifDiff::default()`),
`["applied","no-op","rejected"]` for `set-snapshot`, `["applied","rejected"]` for the other ten. This
is the same pattern the 89a sibling declares — **verified against 87a's own dispatch, not carried
over**.

*Observation, not changed here*: `🧬️mutations/📄set-snapshot/🔣️.json` declares
`"outcomeClasses": ["applied"]`, omitting the `no-op` branch its own sibling `🦀️.rs` reaches three
lines away. That leaf descriptor is read by the `dsl::Mutations` derive to generate production
registration, so correcting it is a production-registration change, out of this wave's scope. The 89a
sibling's descriptor has the identical omission.

## Reproducibility — the three traps

1. *Path resolution* — already fixed in the shared script; the generator writes
   `<SEMIO_FIXTURE_OUT>/mosaic-strip/mosaic-strip.gif`, i.e. treats the env var as a fixtures ROOT.
2. *Process-global counters* — checked for real rather than assumed. The `gif` crate has no
   OCCT-style translator state: `write_screen_desc`/`write_frame_header`/`write_image_block` are pure
   functions of their arguments, `lzw_encode` is deterministic, and nothing reads a clock. Confirmed
   empirically both ways — two separate processes into two separate roots, and **two writes inside one
   process** (the order-dependence case the reproducibility note says a batch regeneration cannot
   see) — all four digests `b7237377…c3e94e`, identical to the committed file.
3. *Never rewrite after hashing* — the generator writes the `.gif` and nothing else; the digest is
   recorded in `🧪️oracle/🔣️.json`, a different file, and no command rewrites the fixture afterwards.

Also corrected relative to the 89a precedent: its `manifests` subcommand emits
`platform: process.platform` (`"darwin"`), which does not match the framework's `Platform` pattern
`^(linux|darwin|win32)-(x64|arm64)$`. This subset's script emits
`` `${process.platform}-${process.arch}` `` and the committed manifest carries `darwin-arm64`.

## Real command output

All from the repo root, this session.

```
$ cd ✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust && cargo check --features oracles --offline
warning: `semio-s-plugin-stdio-test-oracle` (lib) generated 3 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s

$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture verify    --artifact s.stdio.gif --standard 87a --subset any
[fixture verify] 1 fixture(s), 0 file problem(s)

$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture audit     --artifact s.stdio.gif --standard 87a --subset any
[fixture audit] third-party-generated    s.stdio.gif@87a/any / licence=public-domain (synthetic, no third-party content embedded) reproducible=true generator=gif-87a-mutate(gif)
[fixture audit] 1 fixture(s), 0 with contract problems

$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture reproduce --artifact s.stdio.gif --standard 87a --subset any
[fixture reproduce] 1 generated fixture(s), 0 problem(s)
```

`contract` does not honour selector flags (1616 lines repo-wide, exit 1, overwhelmingly pre-existing
and unrelated). Filtered for `gif@87a`, exactly 13 lines, all of one benign class already documented
for the 89a sibling and none of them a fixture/oracle/schema problem:

```
No runtime inventory has been produced for s.stdio.gif@87a/any
Mutation <each of the 12> is owned by "any" and s.stdio.gif@87a declares no narrower subset at all
```

`s.stdio.gif@87a` declares exactly one subset (`🪆️subsets/🔣️component.json`: `"subsets": {"*": …}`),
so the wildcard has nothing to collide with — the same reading the 89a/las wave already settled, and
the same reason `fixture audit` reports `0 with contract problems` (the fixture-side wildcard check,
`isWildcardSubsetFor`, resolves single-subset artifacts as non-wildcard).

`matrix --json`, repo-wide, filtered:

```
oracleEvidenceCoverage       262/658   gif@87a missing: []   gif@89a missing: []
subsetOwnershipCoverage      645/658   gif@87a missing: []   gif@89a missing: []
externalOracleCoverage       436/658   gif@87a missing: []   gif@89a missing: []
fixtureProvenanceCoverage    419/444   gif@87a missing: []   gif@89a missing: []
runtimeMutationCoverage       30/68    gif@87a missing: ["s.stdio.gif@87a/any (no runtime inventory)"]
                                       gif@89a missing: ["s.stdio.gif@89a/any (no runtime inventory)"]
```

All twelve gif@87a mutations are at 100% on the four fixture/oracle dimensions. The 24 matrix ROWS the
new manifest creates (12 mutations × their declared outcomes) all read `status: "missing"`,
`"no fixture declares this mutation × outcome"` — that field reflects executed test RESULTS in
`📤️results.jsonl`, of which this subset has none; the 89a sibling's 42 rows read identically. It is
not a corpus gap and is not what the four coverage dimensions measure.

The one remaining gif@87a shortfall, `runtimeMutationCoverage`, needs the production bridge to emit a
runtime inventory. Identical for 89a, and blocked by the same thing:

## What could NOT be verified, and why that is fine

`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust` (the production `semio-s-plugin-stdio` crate) still does not
compile — `error: could not compile … due to 124 previous errors; 733 warnings emitted`, a peer's
in-flight `protocol` refactor already documented in this ticket's `📓️session-close.md`. **Zero** of
those 124 errors is in a `🎞️gif` file (`grep -c 🎞️gif` over the error paths: `0`); they are in docx,
`🧿️semio` and the plugin root.

This wave never needed that crate. Both things it builds are standalone `[workspace]` roots:
`🧪️oracle/📦️packages/🦀️rust` (which the 87a oracle module is `#[path]`-linked into, and which compiles
cleanly, quoted above) and `🏭️generator/🦀️engine` (one dependency, `gif`). The consequence is only that
`runtimeMutationCoverage` for this subset cannot move until that crate builds, exactly as for 89a.

## Deviation from the precedent, deliberate

- `payloadSchema` points at `../🧬️schema/🧬️mutations/🦀️.rs#<Variant>` — the file that actually exists.
  The 89a manifest points at `🦀️component.rs` in that directory, which **does not exist** (both
  standards' mutation vocabulary lives in `🦀️.rs`); nothing validates the path today, so it went
  unnoticed. Not fixed here — `🔖️89a/` is read-only for this wave.
- The 89a `fixtureManifests[0].notes` field carries corrupted trailing text (stray empty tags, a
  "751 units, 62 prefixes" fragment belonging to a different format). Not copied forward; this
  subset's `notes` describes what the fixture actually exercises.
- `🏭️generator/🦀️engine/src/main.rs` restates GIF §20's four-pass interlace row order rather than
  reaching for the oracle's shared `raster::gif_reorder_rows`. That module is a
  `semio-repo-test-host`/`png`/`gif` consumer, and a fixture generator that depends on exactly one
  registered crate is worth more than deduplicating six lines of a fixed grammar rule; the reason is
  written at the function.

## Scratch kept / removed

Kept in this ticket folder: `🔬️gif-87a-any-verify/` (the `[workspace]`-isolated gate-validation
harness — `🦀️probe/` and its `📜️script.ts` driver; build artefacts under `target/` are gitignored).
Removed: `🗑️temp/gif-87a-any/` command-output logs and the throwaway manifest-splice script.
