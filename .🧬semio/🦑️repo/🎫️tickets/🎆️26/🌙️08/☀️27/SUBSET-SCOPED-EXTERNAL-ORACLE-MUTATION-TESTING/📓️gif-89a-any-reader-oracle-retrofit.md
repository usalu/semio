# 📓️ gif@89a/✳️any — reader-oracle retrofit

Scope: register `gif` 0.13 SEPARATELY as a genuine `third-party-library` reader oracle for
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️any`, alongside (never
replacing) the existing `cross-semio-implementation` oracle `gif-89a-any-mutate` /
`🧪️oracle/🦀️component.rs` (untouched, byte-for-byte, verified below). Followed the avi 1.0/any
reference field-for-field; `🔖️87a/` was never touched.

## Delivered

```
🏭️generator/🦀️engine/Cargo.toml           + second [[bin]] "reader" (own dependency, gif 0.13, same crate)
🏭️generator/🦀️engine/src/reader_main.rs   NEW, 340 lines — build/project/list-recipes, independent of component.rs
🏭️generator/📜️script.ts                   + build / build-manifests commands (generate/manifests, pattern-strip, untouched)
🔬️probes/📜️script.ts                       NEW — gif-import / gif-project / gif-compare
🧪️oracle/🔣️.json                           + oracle, comparisonProfile, 3 probes, 1 pipeline,
                                            outcomes+oracleRequirements fixed on all 21 mutations,
                                            + 17 new fixtureManifests (pattern-strip untouched)
🧫️fixtures/<17 recipes>/{before,after}.gif  NEW, 34 files, all committed
```

## Witnessability — 16 of 21 witnessable, 5 genuinely uncarried

Checked directly against `gif` 0.13.3's source
(`~/.cargo/registry/src/.../gif-0.13.3/src/reader/{mod,decoder,converter}.rs`), not assumed from
`component.rs`. `Decoder`'s public surface: `width`/`height`/`global_palette`/`bg_color`/`repeat`,
plus per-frame `next_frame_info`+`read_into_buffer`. No getter anywhere for the pixel-aspect-ratio
byte, comment-extension text, or application-extension payloads — `write_raw_extension` can WRITE
all three, nothing public can read them back. Registered `<capability>-uncarried`
(`gif-89a-mutate-uncarried`, no `oracle` field — the gltf retrofit's exact convention, reused, not
reinvented) rather than routed around with `component.rs`'s own raw-block-scan technique, per the
brief's explicit instruction not to blur that boundary.

**Uncarried (5)**: `set-pixel-aspect-ratio`, `insert-comment`, `remove-comment`,
`add-app-extension`, `remove-app-extension`.

**Witnessable (16)**: `no-mutation`, `set-snapshot`, `set-screen-size`, `set-global-color-table`,
`set-background-color-index`, `set-loop-count`, `insert-frame`, `remove-frame`, `move-frame`,
`set-frame-geometry`, `set-frame-pixels`, `set-frame-interlace`, `set-frame-delay`,
`set-frame-disposal`, `set-frame-transparency`, `set-frame-user-input`.

### A real finding: the interlace flag IS publicly readable

`component.rs`'s own doc comment and the shared `raster::gif_image_interlace_flags` both state
flatly that `gif::Decoder` cannot recover the interlace flag, because `read_next_frame` always
de-interlaces and resets `Frame::interlaced` to `false`. True of that ONE method — but
`Decoder::next_frame_info` returns the frame BEFORE pixel decoding, at which point `interlaced`
still holds the real Image Descriptor flag (`decoder.rs:598`, set well before the
`Decoded::FrameMetadata` event `next_frame_info` waits for). Capturing it there, then separately
calling `Decoder::read_into_buffer` (takes `&self` for the frame it deinterlaces against, so it
never resets the flag itself — only the higher-level convenience wrappers do), recovers it exactly.
`read_into_buffer` itself already checks `frame.interlaced` and deinterlaces correctly either way,
so the returned pixel bytes are natural-order regardless of storage order. Proven three ways this
session: a Rust unit test (`interlace_flag_is_readable_via_next_frame_info_before_pixel_decode`), a
second unit test showing natural- and interlaced-encodings project the identical pixel digest
(`interlaced_and_natural_encodings_project_the_same_pixel_bytes`), and the real probe pipeline
end-to-end (below). `set-frame-interlace` is therefore witnessable — not uncarried — and moves ONLY
the `interlaced` flag in the projection.

## Fixtures — 17 new recipes, all `-applied` (no per-kind rejection exists)

Read the real dispatch, not doc comments (playbook Step 2):
`🧬️schema/🧬️mutations/🦀️.rs:288`, `protocol::MutationOutcome::new(match self {...})` wraps all 21
kinds uniformly — no per-kind `error`/`fatal` branch anywhere in this subset; out-of-range
indices `apply_kind` in `component.rs` documents as "degrade gracefully to a no-op", matching the
production diff's own documented behaviour. The ONE exception is `set-snapshot`'s own leaf
(`📄set-snapshot/🦀️.rs:19`), which reaches a documented `no-op` warn path when the replacement is
identical. So: **corrected** `mutationManifests[].outcomes` for all 20 non-`no-mutation` kinds from
the pre-existing (wrong) `["applied","rejected"]` to what the code actually reaches —
`["applied"]` for 19 kinds, `["applied","no-op"]` for `set-snapshot`, `["no-op"]` for `no-mutation`
(already correct). This was not explicitly asked for but directly follows from the brief's own
Step 1/2 instruction and from CLAUDE.md's "refactor inconsistencies" rule; flagged here plainly
rather than silently changed.

17 recipes, one `🧬️engine/src/reader_main.rs` hand-authored `GifDoc`/`FrameDoc` pair each, encoded
directly via `gif::Encoder`'s own public API (plus the one documented byte-patch for
background-colour-index, since `Encoder::new` hard-codes it to 0 with no setter — a WRITE-side gap,
irrelevant to the READ-side witnessability argument above):

`no-mutation-no-op`, `set-snapshot-applied`, `set-snapshot-no-op`, `set-screen-size-applied`,
`set-global-color-table-applied`, `set-background-color-index-applied`, `set-loop-count-applied`,
`insert-frame-applied`, `remove-frame-applied`, `move-frame-applied`,
`set-frame-geometry-applied`, `set-frame-pixels-applied`, `set-frame-interlace-applied`,
`set-frame-delay-applied`, `set-frame-disposal-applied`, `set-frame-transparency-applied`,
`set-frame-user-input-applied`.

`pattern-strip` (the pre-existing 21-kind base fixture) is untouched — file, hash, and
`fixtureManifests` entry all byte-identical to `git HEAD`, confirmed programmatically.

## The named trap, checked for real

Three separate checks, per the brief's explicit warning that a whole-corpus regenerate-and-diff
cannot see order-dependent LZW/encoder state:

1. **Two independent processes** — built the full 17-recipe corpus into two separate output roots
   (`bun 📜️script.ts build --out <dir>` twice); `diff -rq` reported zero differences across all 34
   files.
2. **Two calls in one process** — added a Rust test,
   `process_local_determinism::encoding_every_recipe_twice_in_one_process_is_byte_identical`, that
   calls `recipe(id)` + `encode_gif` twice for all 17 recipes inside a single test binary run — the
   exact "regenerate once inside a live process" case a batch corpus diff cannot see. Passes.
3. **The framework's own per-fixture checker, in a loop, never batched** — see below.

`gif` 0.13 has no OCCT-style process-global counters: `write_screen_desc`/`write_frame_header`/
`write_image_block` are pure functions of their arguments and `lzw_encode` is deterministic —
independently re-confirmed here (not merely carried over from the 87a sibling's own finding on the
same crate).

## The gitignore trap — a real, silent problem, found and fixed

Cargo's `src/bin/<name>.rs` convention for a second binary collided with this repo's root
`.gitignore:385: [Bb]in/` (a blanket rule aimed at build-output `bin/` dirs). `git check-ignore`
confirmed `src/bin/reader.rs` was silently excluded from git entirely — it would have built and run
locally with zero indication anything was wrong, then vanished on the next clone. Editing the
shared root `.gitignore` is out of this subset's scope, so the file was relocated to
`src/reader_main.rs` (flat, beside `main.rs`, `[[bin]] path` updated accordingly) — confirmed
un-ignored (`git check-ignore` exit 1, `git status` reports `??`), confirmed byte-identical output
before/after the move, confirmed `cargo test` and the full framework pipeline still pass after it.

## Verified — real commands, real output, this session

```
$ cd 🏭️generator/🦀️engine && cargo test --bin reader --offline
running 7 tests ... test result: ok. 7 passed; 0 failed
  (incl. interlace_flag_is_readable_via_next_frame_info_before_pixel_decode,
        interlaced_and_natural_encodings_project_the_same_pixel_bytes,
        process_local_determinism::encoding_every_recipe_twice_in_one_process_is_byte_identical)

$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture verify --artifact s.stdio.gif --standard 89a --subset any
[fixture verify] 18 fixture(s), 0 file problem(s)

$ bun 🧰️framework/…/🧪️test/📜️script.ts fixture audit --artifact s.stdio.gif --standard 89a --subset any
[fixture audit] 18 fixture(s), 0 with contract problems
  (pattern-strip: generator=gif-89a-any-mutate ; all 17 new: generator=gif-89a-any-mutate-reader)
```

`fixture reproduce`, ONE FIXTURE PER INVOCATION as instructed (`--fixture-family mechanical` for
`pattern-strip`, `--mutation <m> --outcome <o>` — unique per recipe within this subset — for the
other 17; verified these selectors are unique before relying on them):

```
[fixture reproduce] 1 generated fixture(s), 0 problem(s)      ×18, zero failures
```

`matrix --artifact s.stdio.gif --standard 89a --subset any --json` (the command does not honour
selector flags repo-wide — confirmed, matches every prior wave's own finding — so filtered the real
repo-wide JSON for `artifact=="s.stdio.gif" && standard=="89a"`):

```
22 rows (21 kinds; set-snapshot contributes 2 outcome-rows)
17/22 rows carry oracle=gif-89a-any-mutate-reader, oracleKind=third-party-library
 5/22 rows carry no oracle at all (the 5 uncarried kinds — exactly as intended, not a gap)
```

`oracleEvidenceCoverage`/`externalOracleCoverage`'s repo-wide `missing` arrays list bare
`s.stdio.gif::<id>` strings (standard is dropped from that ONE display string, confirmed by reading
`measureCoverage`'s own source) — several of those strings are shared kind NAMES with the sibling
`87a` subset (`no-mutation`, `set-snapshot`, `set-screen-size`, `set-global-color-table`,
`set-background-color-index`, `set-pixel-aspect-ratio`), which is still `87a`'s own unaddressed gap,
out of this ticket's scope. Verified this is a display artefact, not a real 89a regression, by
reading the `rows` array directly (which IS standard-qualified): every one of 89a's own 16
witnessable rows shows `oracle: "gif-89a-any-mutate-reader"` there.

`contract` (repo-wide, does not honour selectors, ~1700 lines, exit 1 — pre-existing and
overwhelmingly unrelated, matching every prior wave). Filtered for `gif@89a`, every remaining line
is one of four already-understood, non-blocking classes:

1. `Oracle gif-89a-any-mutate does not declare capability gif-89a-mutate` — pre-existing, about the
   UNTOUCHED cross-semio oracle's own capability name vs. a gherkin file; not touched, not caused
   here.
2. `No runtime inventory has been produced` / `Mutation X is owned by "any" ... declares no
   narrower subset at all` — the same benign wildcard-ownership finding every single-subset gif/las
   wave already reports (nothing to collide with).
3. Five `requires a third-party-library for capability gif-89a-mutate-uncarried, and none is
   registered` — EXPECTED. This is the honest, correct report for the 5 kinds this reader genuinely
   cannot witness; it is not a bug.
4. `1 mutation kind(s) have no wire record` — a `📡️component.protocol.semio` wire-format
   completeness matter, unrelated to oracle/probe registration.

Critically: the `reimplementation-registered-as-third-party` breach that DID fire once mid-session
(a transient read, see below) is **absent** from the final, fully-settled run — confirmed by both a
fresh `contract` re-run and a direct debug script computing the checker's own `judgedByProbes`
condition (`comparisonPipelines.length>0 && probes.length>0 && probes.some(qualified)`), which
returns `true` for this contribution: `comparisonPipelines: 1`, `probes: 3` (all
`qualification.status: "qualified"`).

## The gate, both ways, with real numbers — via the actual probes, not shortcuts

```
$ bun 🔬️probes/📜️script.ts gif-compare --input .../no-mutation-no-op/{before,after}.gif
{"equal":true,"diffCount":0}

$ bun 🔬️probes/📜️script.ts gif-compare --input .../set-background-color-index-applied/{before,after}.gif
{"equal":false,"diffCount":1,"diffs":["$.backgroundColorIndex: 2 ≠ 5"]}

$ bun 🔬️probes/📜️script.ts gif-compare --input .../set-frame-delay-applied/{before,after}.gif
{"equal":false,"diffCount":1,"diffs":["$.frames[0].delayCs: 10 ≠ 99"]}

$ bun 🔬️probes/📜️script.ts gif-compare --input .../set-frame-interlace-applied/{before,after}.gif
{"equal":false,"diffCount":1,"diffs":["$.frames[0].interlaced: false ≠ true"]}
   (indicesDigest identical on both sides — the interlace finding, end-to-end through the real probe)
```

Each reject case is a genuine single-field content difference produced by that recipe's own
mutation, not a fixture invented to look different.

## What could not be verified, and why that's fine

- **`runtimeMutationCoverage`** stays at its pre-existing shortfall for this subset
  (`s.stdio.gif@89a/any (no runtime inventory)`) — needs the production `semio-s-plugin-stdio`
  bridge, which is documented elsewhere in this ticket as broken by an unrelated, in-flight peer
  refactor. Neither this wave's wrapper crate (`🏭️generator/🦀️engine`, its own `[workspace]`) nor
  the shared oracle crate needed that bridge to build or test — confirmed, everything above ran
  clean and offline.
- The `oracleEvidenceCoverage`/`externalOracleCoverage` display-string collision with `87a`'s own
  gap (above) is real but pre-existing and out of scope; not fixed here since it would mean editing
  either the shared coverage formula or `87a`'s own registration, both explicitly forbidden.

## Deliberate deviations from the avi precedent

- avi hand-authors AVI documents because `riff` is a generic RIFF layer with no format-specific
  semantics of its own. `gif` 0.13 IS a real GIF codec, so `reader_main.rs`'s `encode_gif` calls
  `gif::Encoder` directly rather than reimplementing chunk framing — the one byte-patch is the same
  documented gap `component.rs` also patches, not new hand-rolled parsing.
- `src/reader_main.rs`, not `src/bin/reader.rs` — the gitignore trap above.
- 16 witnessable kinds here vs. avi's 13/13; avi has no `-uncarried` kinds at all (RIFF exposes
  everything AVI needs). Genuinely different per-format boundary, not a shortfall of effort.
- No `-rejected-<reason>` recipes — avi/las both have real per-kind validation failures; gif@89a's
  own dispatch degrades out-of-range input to a no-op instead, confirmed from the code, so there is
  nothing honest to build a rejection fixture around.

## Scratch kept / removed

Kept at the ticket root: `📜️patch-gif-89a-any-oracle-json.py` (the one-off script that additively
patched `🧪️oracle/🔣️.json` — oracle/profile/probes/pipeline registration, outcome corrections,
fixtureManifests append; re-run is idempotent, guarded by existing-id checks). Removed: all
`/tmp` command-output logs and the `🗑️temp/` scratch copies made mid-session.
