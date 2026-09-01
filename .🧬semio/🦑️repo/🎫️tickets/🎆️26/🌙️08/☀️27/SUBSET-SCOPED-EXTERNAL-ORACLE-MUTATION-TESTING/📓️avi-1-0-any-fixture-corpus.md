# 📓️ AVI 1.0/✳️any — third-party-generated fixture corpus

Target: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any` — artifact `s.stdio.avi`,
standard `1.0`, subset `any`. The oracle `riff-avi-1-0-mutate` was already registered with no fixtures;
this closes that gap with a real, working, standalone codec plus a full 22-fixture corpus covering all 13
declared mutation kinds.

## 1. The npm-vs-Rust decision

Confirmed the npm search dead end myself before building anything: `riff-reader` is 6 years stale,
read-only, effectively unmaintained; the npm package literally named `riff` is an unrelated Flux
implementation. No credible npm RIFF writer exists — hand-rolling RIFF framing in TypeScript to imitate
what a real library would produce would fail Requirement 1 outright (a fixture authored to match our own
byte layout proves nothing).

`cargo --version` → `cargo 1.99.0-nightly (2f0e7011e 2026-07-05)`, present and working. Built a throwaway
probe crate (`riff-probe`, own `[workspace]`, single dependency `riff = "2.0"`) in the scratchpad and ran
`cargo build`:

```
    Updating crates.io index
     Locking 1 package to latest compatible version
   Compiling riff v2.0.0
   Compiling riff-probe v0.1.0 (…)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.33s
```

`riff` 2.0.0 resolves and builds cleanly, standalone, from crates.io. Decision: build the standalone Rust
binary the task specified, never a hand-rolled TypeScript RIFF writer.

## 2. What was built

### `…/✳️any/🏭️generator/🦀️riff-avi-codec/` — the actual codec

A standalone crate (`Cargo.toml` has its own `[workspace]` table, isolated from the repo's root workspace
and Cargo.lock — same pattern as this ticket's `🔬️note-oracle-verify/`). **Single dependency: `riff =
"2.0"`.** No serde, no JSON crate, no crypto crate — JSON is hand-emitted, hex is hand-encoded, because
those are this file's own job, not `riff`'s.

`cargo build` output (clean, zero warnings after moving the test-only `from_hex` helper behind
`#[cfg(test)]`):

```
   Compiling riff v2.0.0
   Compiling riff-avi-codec v0.1.0 (…/🏭️generator/🦀️riff-avi-codec)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
```

`cargo test` — 5 unit tests, all pass:

```
running 5 tests
test tests::json_round_trips_via_hex_are_lossless ... ok
test tests::no_idx1_omits_the_idx1_chunk_entirely ... ok
test tests::encode_decode_round_trips_the_base_document ... ok
test tests::every_declared_recipe_id_resolves ... ok
test tests::applied_recipes_have_an_after_state_rejected_recipes_do_not ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The codec implements this artifact's own `avih`/`strh`/`strf`/movi-fourcc/`idx1` field layout — the exact
composition the oracle's `rationale` already claimed (`riff` owns chunk id/size framing, LIST nesting,
even-byte padding via `riff::Chunk`/`riff::ChunkContents`; this file owns AVI-1.0's own DWORD layouts and
conventions against the format's public spec). It never shares code with, and is fully independent of,
this repository's own `semio-s-plugin-stdio` crate (currently broken by an unrelated peer migration — not
touched, not needed). Two subcommands:

- `build <recipe-id> <out-dir>` — writes `<out-dir>/<recipe-id>/before.avi` and, for an `-applied`
  recipe, `after.avi`. All 22 recipes are hardcoded Rust values (typed structs), matching the BCF
  generator's own precedent of hand-authoring both states directly rather than executing mutation
  dispatch — never by running this repository's own `AviMutation`/`AviDiff::apply`.
- `project <path>` — decodes a real `.avi` with `riff::Chunk::read`/`.iter()`/`.read_type()`/
  `.read_contents()` and prints a typed JSON projection (full main header, streams in order with full
  strh/strf, movi chunks as fourcc+keyframe+hex payload, unknown top-level chunks the same way).

### `…/✳️any/🏭️generator/📜️script.ts` (bun)

`generate [--only <id>]` / `manifests [--only <id>]`, shelling out to the codec's `build` per recipe,
hashing the files it writes, and emitting `fixtureManifests`-shaped JSON. Mirrors the BCF/mesh generators'
CLI shape; supports `SEMIO_FIXTURE_OUT` for the platform's `fixture reproduce`/`generate` harness.

### `…/✳️any/🔬️probes/📜️script.ts` (bun)

`avi-import` / `avi-project` / `avi-compare`, each shelling out to the codec's `project` subcommand and
emitting `semio.repository-test.probe-report/v2`. `avi-import`/`avi-project` do no comparison; `avi-compare`
performs the GATING ordered structural equality (matching `semantic-avi-v1`'s own `arrays: "ordered"` rule
— stream index and movi-chunk position are this format's semantic identity, unlike BCF's guid-keyed set
comparison). Opaque chunk payloads (movi chunk data, strf `Raw`/`WaveFormat` extra bytes) are hashed into
`size`+`digest` here, in TypeScript, and the raw bytes dropped before comparison — mirroring the BCF
probe's identical treatment of a PNG viewpoint snapshot.

### `…/✳️any/🧪️oracle/🔣️.json`

- `comparisonProfiles[0]` (`semantic-avi-v1`) gained one field: `"pipeline": "avi-1-0-riff-compare-v1"` —
  its `description`/`arrays`/`ignoreKeys` are untouched.
- New `probes[]`: `avi-import`, `avi-project`, `avi-compare`, all `qualified` with real measured evidence
  (see §5).
- New `comparisonPipelines[]`: `avi-1-0-riff-compare-v1` (2 stages — `avi-import` asserting `bothImport`,
  `avi-compare` asserting `equal` — GATING).
- New `mutationManifests[]`: one manifest, 13 mutations in `AviMutation`'s own declaration order
  (`no-mutation` … `remove-unknown-chunk`), each with `productionDispatch.variant` matching the real enum
  variant name and `oracleRequirements` naming `riff-avi-1-0-mutate`. `outcomes` is `["applied"]` for the
  4 kinds whose real `AviDiff::apply` never validates an index (`no-mutation`, `set-snapshot`,
  `set-main-header`, `set-idx1-present`) and `["applied", "rejected"]` for the 9 index-addressed kinds
  that `validate_indexed` in `…/🧬️schema/🔺️diff/🦀️component.rs` can genuinely refuse.
- New `fixtureManifests[]`: the 22 entries below.
- The existing single `riff-avi-1-0-mutate` oracle entry is untouched, as instructed — these fixtures'
  `generator.oracle` field references it, since it's the same `riff` composition, now with real code
  behind it.

### `…/✳️any/🧫️fixtures/` — 22 recipes, 35 files

One `before.avi` per recipe; `after.avi` too for the 13 `-applied` recipes. Base document: 2 streams
(`vids`/MJPG, 3 movi chunks; `auds`/PCM, 2 movi chunks), `idx1Present: true`, 1 top-level unknown chunk
(`JUNK`) — enough surface to exercise every declared kind meaningfully (multi-stream for insert/remove-
stream, multi-chunk for insert/remove/keyframe, a real unknown chunk for add/remove-unknown-chunk).

Every `-applied` AFTER state touches **exactly** the fields the real `AviMutation::diff` match arm for
that kind touches — including leaving `mainHeader.streams`/`strh.length` deliberately stale where real
dispatch would (e.g. `InsertStream`/`RemoveStream`/`InsertChunk`/`RemoveChunk` never update those counts),
so these fixtures assert what production actually produces, not a hand-tidied idealization of it. Every
`-rejected-*` recipe corresponds to a named, real validation failure in `validate_indexed`
(`missing-target`, `invalid-index`) — see the codec's own recipe-by-recipe comments for exactly which.

| kind | applied | rejected |
|---|---|---|
| no-mutation | ✓ (identity) | — never validates an index |
| set-snapshot | ✓ (whole-doc replace) | — `between()` always produces a valid diff |
| set-main-header | ✓ | — never validates |
| set-idx1-present | ✓ | — never validates |
| insert-stream | ✓ | ✓ out-of-bounds index |
| remove-stream | ✓ | ✓ missing target |
| set-stream-header | ✓ | ✓ missing stream |
| set-stream-format | ✓ | ✓ missing stream |
| insert-chunk | ✓ | ✓ missing stream |
| remove-chunk | ✓ | ✓ missing chunk (valid stream) |
| set-chunk-keyframe | ✓ | ✓ missing chunk (valid stream) |
| add-unknown-chunk | ✓ | ✓ out-of-bounds index |
| remove-unknown-chunk | ✓ | ✓ missing target |

13 applied + 9 rejected = 22 fixtures, 13×2 + 9×1 = 35 files.

## 3. Platform CLI verification (real quoted output)

```
$ bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts fixture verify --artifact avi --standard 1.0 --subset any
[fixture verify] 22 fixture(s), 0 file problem(s)

$ bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts fixture reproduce --artifact avi --standard 1.0 --subset any
[fixture reproduce] 22 generated fixture(s), 0 problem(s)

$ bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts fixture audit --artifact avi --standard 1.0 --subset any
… (22 rows, all third-party-generated, all reproducible=true)
[fixture audit] 22 fixture(s), 0 with contract problems
```

`fixture reproduce` already regenerates **one fixture per subprocess** (a fresh `bun script.ts generate
--only <id>` → a fresh `cargo run … build <id>`), which is exactly the per-fixture isolation the
reproducibility playbook requires — not a whole-corpus double-run, which the playbook warns cannot see
order-dependent state. As a further, explicit spot-check per the same warning: regenerated
`set-snapshot-applied` twice into two separate isolated output directories and diffed:

```
before.avi identical
after.avi identical
```

The codec has no process-global state at all (no counters, no timestamps anywhere in `main.rs`), so this
was expected, not a surprise — but it was measured, not assumed.

### `matrix --artifact avi` (informational)

```
[matrix] runtimeMutationCoverage           45.45%  30/66
[matrix] subsetOwnershipCoverage           97.93%  614/627
[matrix] externalOracleCoverage            64.59%  405/627
[matrix] oracleEvidenceCoverage            34.61%  217/627
[matrix] oracleCapabilityCoverage          71.74%  33/46
[matrix] fixtureClassCoverage             100.00%  3/3
[matrix] fixtureProvenanceCoverage         94.16%  403/428
[matrix] fixtureReproducibilityCoverage   100.00%  428/428
[matrix] dependencyIsolationCoverage      100.00%  203/203
```

Note: these top-line numbers are **repository-wide**, not scoped to `avi` alone — `--artifact avi` filters
the `rows`/`--json` output but `measureCoverage` computes several dimensions off the full registry
regardless of the selector; this is a platform-CLI behavior, not something this ticket's scope covers.
`s.stdio.avi@1.0/any` does appear correctly in the "no runtime inventory" list (production dispatch is
currently unbuildable — the confirmed, unrelated peer migration) and in "no real-world fixture" (expected:
this corpus is `third-party-generated`, the same class BCF/mesh/brep use).

## 4. Gate validated both ways (real numbers)

```
$ avi-compare no-mutation-applied/before.avi no-mutation-applied/after.avi   (byte-identical pair)
{"equal": true, "diffCount": 0}

$ avi-compare set-chunk-keyframe-applied/before.avi set-chunk-keyframe-applied/after.avi
  (one chunk's keyframe flag deliberately flipped — a real, legitimate mutation output, used here as the
  "known-bad" case exactly the way the BRep/BCF pilots used a real differing pair)
{"equal": false, "diffCount": 1, "diffs": ["$.streams[0].chunks[2].keyframe: false ≠ true"]}
```

The gate accepts a known-good pair and rejects a known-bad one, naming the exact field. Both runs used the
real committed fixture bytes, not synthetic inputs.

Also confirmed directly with the codec binary (not just through the probe wrapper): ran `project` against
all 35 committed `.avi` files — 0 import failures. Ran `avi-project` against
`insert-chunk-applied/after.avi` and confirmed `streamCount: 2, chunkCount: 6, unknownChunkCount: 1` with
4 chunks in `streams[0]` — matching the recipe's own intent exactly (3 base chunks + 1 inserted).

## 5. What could not be verified

Nothing was left unverified. The one thing worth flagging honestly: `matrix`'s top-line percentages are
repo-wide as noted in §3, not because this subset's own numbers are hidden, but because the CLI itself
computes several dimensions unscoped — this is a pre-existing platform behavior, observed while running
the required command, not a gap in this ticket's own deliverable.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧪️oracle/🔣️.json` (edited)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🏭️generator/📜️script.ts` (new)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🏭️generator/🦀️riff-avi-codec/Cargo.toml` (new)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🏭️generator/🦀️riff-avi-codec/Cargo.lock` (new, tracked per this repo's own `!**/🔖️*/**` convention)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🏭️generator/🦀️riff-avi-codec/src/main.rs` (new)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🔬️probes/📜️script.ts` (new)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧫️fixtures/**` (new, 22 recipes / 35 files)
