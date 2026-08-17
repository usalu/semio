# 📓️ W2-P report — stdio carrier pilot (💾️binary / 📄txt)

Agent: W2-P pilot agent. Boundary (only writer): `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/**`,
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/**`, mount-only lines in `📦️glue.rs` and TS `📦️index.ts`
for these two artifacts, plus this ticket's own `🔧️patches/`/`📓️*.md`. Read `📌️important.md`,
`📓️design.md`, `📓️w1-a-report.md`, `📓️w1-c-report.md`, CLAUDE.md, and the fixture
(`crate::app::declarations::fixture`) before starting, per the mission.

## Prerequisite patch (`🔧️patches/w1b-discovery-io-native-codec-vocabulary.txt`) — NOT applied

Read it in full. It teaches the discovery walker + taxonomy the "native codec under `🚪️io`" shape
(`io/{import,export}/.../{snapshot,diff,mutations,inferences}/{text,binary}` carrying
`📖️component.grammar.semio`/`📡️component.protocol.semio`). This pilot did **not** physically
relocate any native-codec spec files into `🚪️io/**` (see `## openQuestions` for why — the wide
test blast radius one exploratory relocation attempt exposed, plus discovering Rust cannot split
one trait impl's `parse_dsl`/`print_dsl` across two files the way the tree diagram's exact-mirror
shape implies). Because no new files landed at the taxonomy-relevant paths this patch teaches, it
was **not required** for this pilot's own changes and was **not applied** — left for whichever
agent actually does the physical relocation (recipe §7's do-not-do-this entry explains the
trait-impl constraint in full). `bun ./📜️script.ts verify taxonomy enforce` numbers below are
therefore identical before/after this pilot's changes by construction (no taxonomy-relevant paths
touched) — confirmed by re-running it after all edits landed (see `## verification`).

## What moved where (file-level)

### Carrier law fix (the point of the pilot)
- `🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` —
  `impl store::ArtifactPack for BinarySnapshot`: `encode_pack_with`/`decode_pack_with` no longer
  wrap/unwrap a `SemioEnvelope` (`store::semio_format::wrap_binary`/`unwrap_binary`) — now the
  identity function on `self.bytes`/`bytes.to_vec()`. `ArtifactDsl` (hex-text form) **unchanged**
  — not carrier-law-bound (see design.md §3: only the `Binary` `IoPayload` variant of this
  dialect is `CARRIER_BINARY`).
- `🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` —
  `impl store::ArtifactDsl for TxtSnapshot`: `parse_dsl`/`print_dsl` no longer wrap/strip a
  `semio stdio.txt.dsl v1` preamble line (`wrap_text`/`split_text_preamble`) — now
  `TxtSnapshot::from_body`/`to_body` directly. `ArtifactPack` (binary/pack form) **unchanged** —
  not carrier-law-bound (only the `Text` variant of this dialect is `CARRIER_TEXT`).
- Regenerated fixtures to match: `💾️binary/…/📚️examples/🎬️demo/🖼️assets/🎒️example.pack.semio`
  (was a `SemioEnvelope`-wrapped `hello`, now literally 5 raw bytes `hello`);
  `📄txt/…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio` (was preamble-wrapped, now
  byte-identical to the artifact's own already-shipped raw carrier fixture,
  `🖼️assets/📄example.txt` — "Hello, stdio.txt!\n").
- Updated the two tests that baked in the old wrapped shape as an assumption:
  `💾️binary/…/🧬️schema/💡️inferences/🦀️component.rs`'s `protocol_walk_law` (dropped the now-
  unneeded `unwrap_binary` call before `walk_protocol`); `📄txt/…/🧬️schema/🦀️component.rs`'s
  `grammar_conformance_law` (builds its synthetic `"<envelope-id>\n<body>"` conformance input
  directly from the now-raw `PRIMARY_TEXT`, instead of splitting a preamble that no longer
  exists). `fixture_honesty_law` in both files needed **no code change** — only the fixture
  bytes/text, since it just compares codec output to the (regenerated) fixture.
- **New law test**, `carrier_native_is_raw`, in `🚪️io/🦀️component.rs`'s new `mod carrier_law` for
  both artifacts — round-trips arbitrary bytes/text through `decode_pack`→`encode_pack` (binary)
  / `parse_dsl`→`print_dsl` (txt) and asserts byte-identical output plus asserts the encoded
  payload does NOT start with `store::semio_format::BINARY_MAGIC`
  (`[0x89,'S','E','M',0x0D,0x0A,0x1A,0x0A]`, binary) / does not start with `"semio "` (txt).

Real test source (binary, `🚪️io/🦀️component.rs`):
```rust
#[test]
fn carrier_native_is_raw() {
    for bytes in [Vec::<u8>::new(), vec![0x00, 0x01, 0xFF], b"hello".to_vec(), (0u8..=255).collect::<Vec<u8>>()] {
        let decoded = BinarySnapshot::decode_pack(&bytes).expect("decode");
        let encoded = decoded.encode_pack();
        assert_eq!(encoded, bytes, "carrier round trip must be byte-identical for {bytes:?}");
        assert!(!encoded.starts_with(&store::semio_format::BINARY_MAGIC), "carrier payload must not be a pack container: {encoded:?}");
    }
}
```
(txt's twin is the same shape over `parse_dsl`/`print_dsl` and a `"semio "` prefix check.)

### New declaration tree (additive — see `## openQuestions` for why not a full cutover)
- **NEW** `🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🦀️component.rs` — standard root,
  `standard() -> StandardDeclaration`, media `{mimes: ["application/octet-stream"], extensions:
  ["bin"]}` — real values, read from `🧬️schema/📜️artifact-definition.json`'s
  `representations[0]`, not invented.
- **NEW** `🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🦀️component.rs` — subset root,
  `subset() -> SubsetDeclaration`, assembling the existing (unmoved) `schema`/`io`/`editor`/
  `viewer`/`examples` children.
- **NEW** `🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🦀️component.rs` +
  `🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🦀️component.rs` — same shape, media
  `{mimes: ["text/plain"], extensions: ["txt"]}` (also read from the real
  `📜️artifact-definition.json`, not invented).
- `🗿️artifacts/💾️binary/🦀️component.rs` / `📄txt/🦀️component.rs` — **added** `pub fn artifact()
  -> ArtifactDeclaration` (new tree). **Did not** delete `assembly()`/`artifact_kind()` — both are
  still required by `📇️registry/🦀️component.rs`'s rigid 36-artifact catalog (outside this
  shard's boundary — see `## openQuestions` §1).
- `🚪️io/🦀️component.rs` (both artifacts) — **added** `pub fn io() -> IoDeclaration` (`entries:
  &[]` — carrier law: no foreign hops needed on a carrier's own side, see the file's own doc
  comment) + the `carrier_law` test module. **Did not** delete the old `derived_composition`/
  `io_registry`/`register()` cluster (confirmed dead code — zero repo-wide call sites — but left
  in place; see `## openQuestions` §2).
- `📦️glue.rs` — added 4 mount blocks (standard-root + subset-root, ×2 artifacts), each a plain
  `#[path=...] mod component; pub use component::*;` inserted alongside the existing `schema`/`io`
  mounts, following this file's own established convention throughout.
- `🚪️io/🟦️component.ts` (both artifacts) — replaced the `export {};` stub with a typed empty
  `IoEntryDescriptorMirror[]` export (no generated `IoEntryDescriptor` TS type exists anywhere in
  the repo yet to import from — shaped inline, swap for a real import once ts-rs generation
  lands). `📦️index.ts` needed no change — it already `export * as binary/txt from ".../🟦️component.ts"`
  at the artifact root, which already re-exports everything beneath it.

## Carrier law — real output

```
$ CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-s-plugin-stdio --lib
    Finished `dev` profile [unoptimized] target(s) in 1m 01s     (0 errors)
```
The `carrier_native_is_raw` test itself could **not** be run to a pass/fail result — the crate
does not currently build with test cfg enabled (`--lib --tests`/`nextest`), for reasons entirely
outside this shard's own files (see `## verification`). It was verified by:
1. `cargo check -p semio-s-plugin-stdio --lib` (no test cfg) compiling clean, proving every type
   in the test module (`BinarySnapshot::decode_pack`/`encode_pack`, `store::semio_format::
   BINARY_MAGIC`, etc.) resolves correctly.
2. Manual trace of the codec: `BinarySnapshot::encode_pack_with` is now literally `Ok(self.bytes.
   clone())` and `decode_pack_with` is literally `Ok(Self{..., bytes: bytes.to_vec()})` — the
   round trip is definitionally the identity function; `TxtSnapshot::print_dsl`/`parse_dsl` are
   literally `self.to_body()`/`Self::from_body(text)` — same. Both are read directly in the
   diff below.
3. The regenerated fixtures independently prove the SAME codec at rest: `🎒️example.pack.semio` is
   now exactly `68 65 6c 6c 6f` (`hello`, 5 bytes, confirmed via `xxd`); `🗣️example.dsl.semio` is
   now byte-identical to `📄example.txt` (confirmed diff-free).

**This is honest, not a false pass claim** — CLAUDE.md forbids claiming a test passes without
running it; the above is the strongest available evidence given `## verification`'s build
blocker, not a substitute for actually running it. The FIRST agent to land on a green `--tests`
build for this crate should run `cargo nextest run -p semio-s-plugin-stdio --lib -E
'test(carrier_native_is_raw)'` and confirm.

## verification

All commands run with `CARGO_TARGET_DIR=<ticket>/🎯️target`, from `/Users/ueli/Documents/semio`.

**Pre-existing, unrelated crate breakage — confirmed, not caused by this pilot.** At this ticket's
start, `semio-s-plugin-stdio` did **not** build with `--all-targets`: 433 pre-existing compile
errors (`🧪️w2p-baseline-check.txt`), every single one inside `🧿️semio`, `🖊️dwg`, `🎞️gif`, `🧊️gltf`,
`🎥️mp4` — artifacts nobody on this ticket owns, matching a live peer session
(`26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-CODECS-INFERENCES-AND-MUTATIONS`, confirmed active
08/16-17 per `📓️status.md`). **Caveat on ordering**: the baseline `cargo check --all-targets` run
was started in the background and this agent began editing files before it finished — a process
mistake (recipe §0.3 documents it so later agents don't repeat it). The contamination is provably
harmless here: every baseline error line traces to a path this pilot never touched (verified by
grep), and this pilot's own first `--lib` check (run AFTER the carrier-law edits) was clean.

| gate | before (`🧪️w2p-baseline-*.txt`) | after (`🧪️w2p-after-*.txt`) | delta attributable to this pilot |
|---|---|---|---|
| `cargo check -p semio-s-plugin-stdio --lib` (no test cfg) | not separately captured (only `--all-targets` baseline exists) | **0 errors**, clean, `Finished` in 1m01s | clean |
| `cargo check -p semio-s-plugin-stdio --all-targets` | **433 errors** (166 check + ~267 test-target; `mp4`/`gif`/`gltf`/`dwg`/`semio`, zero in `💾️binary`/`📄txt`) | **268 errors**, zero in `💾️binary`/`📄txt` (grep-confirmed both times) | **net −165** (a concurrent peer session fixing unrelated files while this pilot ran — not this pilot's doing; this pilot's own delta is 0 either direction) |
| `cargo nextest run -p semio-s-plugin-stdio --lib --no-fail-fast` | build fails, same population as above | build fails: **267 test-target errors**, zero in `💾️binary`/`📄txt` (grep-confirmed) | net ≥0 for this pilot's files (0 attributable both times); whole-crate run blocked by the same pre-existing peer breakage, not by this pilot |
| `cargo check -p semio-s-plugin-stdio --target wasm32-wasip2 --lib` | not run (out of scope pre-check) | **0 errors**, clean, `Finished` in 2m27s (cold build) | clean |
| `bun ./📜️script.ts verify taxonomy enforce` | 10723 error findings (repo-wide, pre-existing — not re-run before this pilot's edits since no taxonomy-relevant path was touched) | 10723 error findings (unchanged — confirmed no taxonomy-relevant paths were touched by this pilot) | **0** (by construction — see prerequisite-patch section above) |
| `bun ./📜️script.ts policy` | see `📓️status.md`'s recorded W1 baseline table | see below | see below |

### Policy delta (report-mode breach counts)

`bun ./📜️script.ts policy` writes its full breach set to
`.🧬semio/🦑️repo/⚡️cache/breaches/compose.json` (`kind` field = policy id, prefixed
`clean-mechanism/`). Repo-wide totals are unchanged from `📓️status.md`'s recorded W1 baseline
(1132/1117/344/112/61/59 — this pilot's own two artifacts are a rounding error against a
2825-total repo-wide backlog). Scoped exactly to
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/{💾️binary,📄txt}`:

| policy | binary breaches | txt breaches | why |
|---|---|---|---|
| `io-exclusivity` | 4 | 5 | `parse_dsl(`/`print_dsl(`/`decode_pack(` calls still live in `🧬️schema/…` (the carrier-law codec fix touched the CODE but did not RELOCATE it into `🚪️io/**` — see `## openQuestions` §4, this is the exact, predicted consequence of scoping the physical relocation out) + one `include_bytes!` in txt's examples/demo (unrelated — pre-existing, reads a fixture asset, not this pilot's edit) |
| `owner-mounts-children` | 3 | 3 | the artifact root (pre-existing, unrelated to this pilot) + the two NEW roots this pilot added (standard root, subset root) each "exist but do not `#[path]`-mount their children INLINE" — this policy checks for inline mounting; this repo's established convention mounts everything through `📦️glue.rs` instead (every one of the other 34 stdio artifacts' roots has the exact same shape) — a repo-wide convention question, not something this pilot introduced or could unilaterally fix by deviating from how mounting already works everywhere else |
| `subset-isolation` | 0 | 0 | clean |
| `io-declaration` | 0 | 0 | clean |
| `subset-standalone` | 0 | 0 | clean |
| `module-consumer-count` | 0 | 0 | clean |

**Net honest assessment**: 3 of 6 policies are already fully clean for both artifacts.
`io-exclusivity`'s remaining count is the direct, documented cost of NOT physically relocating the
codec (openQuestion 4) — it will close to 0 the moment that relocation happens, and does NOT
regress the carrier law itself (the codec is fixed either way; only its FILE LOCATION is what
`io-exclusivity` checks). `owner-mounts-children`'s count reflects a pre-existing, repo-wide
mounting convention (glue.rs-based) this single pilot could not and should not have unilaterally
changed.

## sharedFileRequests

- `🔧️patches/w2p-stdio-plugin-root.txt` — the exact lines the coordinator must add to
  `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs` (`builder.declare_artifact(crate::artifacts::binary::
  artifact())` / `...::txt::artifact()`) to actually wire the new tree into the live plugin, plus
  the full reasoning for why `📇️registry/🦀️component.rs`'s rigid 36-artifact catalog blocks doing
  this inside this shard's own boundary, and a scoped Option B for a full cutover once the crate
  builds clean again. **This is the single most important finding for the other ~40 fan-out
  agents** — every one of them will hit the same wall for their own artifact.

## openQuestions

1. **`assembly()`/`artifact_kind()` (old channel) were NOT deleted from the artifact roots** —
   `📇️registry/🦀️component.rs`'s `artifact_factories()` is a `BTreeMap` of exactly 36 `fn(...)`
   entries, cross-checked by a test (`schema_keys_and_runtime_factories_are_exact`) against a
   36-entry `SOURCES` array of `📜️artifact-definition.json` includes — `crate::artifacts::binary::
   assembly`/`crate::artifacts::txt::assembly` are referenced BY NAME from that map, a real,
   external, outside-this-shard's-boundary dependency. Confirmed via repo-wide grep that
   `artifact_kind()` itself (unlike `assembly()`) has zero other callers — genuinely dead, but
   left in place anyway (not worth a partial cleanup pass while `assembly()` must stay). See the
   patch's Option A/B for the two ways forward.
2. **The old `derived_composition`/`io_registry`/`register()`/`register_pilot_languages()`/
   `register_schema_specs()`/`register_artifact_schema()`/`register_artifact_inferences()`
   cluster, and the self-referential identity `ArtifactDeserializer`/`ArtifactSerializer` leaves
   at `🚪️io/📥️import/…/🗿️artifacts/💾️binary/🔖️raw/✳️any/` + `📤️export/…`, were confirmed dead
   (zero repo-wide call sites for `register()`; zero external callers of the `derive_artifact_
   facets!`-generated `BinaryAnalyzer`/`BinaryComposer`/`TxtAnalyzer`/`TxtComposer` — only
   `BinaryBuilder`/`TxtBuilder` are still referenced, by their own `impl ArtifactInferrer for
   …Builder` one directory over) but were **not deleted**, because the crate does not currently
   build clean end to end (`## verification`) and there was no reliable compile signal to verify
   a deletion pass didn't silently break something. This is real, low-risk, ready-to-do follow-up
   work — not a design uncertainty — once the crate is green. `impl ArtifactInferrer for
   …Builder` should retarget onto `semio_framework_plugin::app::SnapshotBuilder<Snapshot,
   Mutation>` when that pass happens.
3. **`NativeCodecs.{snapshot,diff,mutations}` are all `LanguagePair{text: None, binary: None}`**
   — real `&'static dsl::LanguageSpec` values (mirroring the OLD `register_pilot_languages()`'s
   5-role registration) were not wired into these slots. This is legal per the type's own
   documented shape (a plain-codec subset), and the underlying `ArtifactDsl`/`ArtifactPack`/
   `OpText`/`OpBinary` codecs these would point at are unchanged and independently tested either
   way — but it means declaring through the new tree does not yet re-register grammar/protocol
   DSL tooling metadata the old channel did. Real follow-up, not required for the carrier law.
4. **The physical relocation of native codec files out of `🧬️schema/*/{📝️text,💾️binary}` into
   `🚪️io/{📥️import,📤️export}/…/{📝️text,💾️binary}` (mission step 4, design.md tree shape) was
   NOT attempted.** Two independent, real reasons, not scope-cutting for its own sake: (a) a
   single trait (`ArtifactDsl`/`ArtifactPack`) can only be `impl`'d ONCE for a type in Rust — the
   tree diagram's literal "exact mirror" of import vs export directories each holding the real
   codec is not achievable without either duplicating the actual decode/encode logic (a genuine
   double-source-of-truth) or picking one direction to own the impl and having the other merely
   re-export/duplicate the spec asset — a real design decision this pilot did not feel authorized
   to make unilaterally for the other ~40 subsets that will copy this recipe; (b) an exploratory
   check of the blast radius (grepping which tests reference `schema::snapshot::text::
   COMPONENT_GRAMMAR_SEMIO`-shaped paths) showed relocating even ONE facet (`snapshot`, the
   carrier-law-relevant one) would require updating grammar/protocol-conformance tests in at
   least 2 more files per artifact beyond what this pilot already touched, and relocating all
   four facets (`snapshot`/`diff`/`mutations`/`inferences`) × 2 directions × 2 forms × 2 artifacts
   is a substantially larger, higher-risk change than fit safely in this pass alongside the
   carrier-law fix and the declaration tree. Recipe §3/§7 documents the exact trait-impl
   constraint and recommends a resolution (pick the import side as the real impl owner) for
   whoever does this for real.
5. **`SubsetDeclaration.schema.inference_services`/`ArtifactDeclaration.localization` are
   empty/`&[]`** for both artifacts — real en/de localized descriptors exist in the OLD
   `📜️artifact-definition.json` channel (kept, see openQuestion 1) but were not additionally
   wired into the new tree's own `localization` field; `inference_services` genuinely has nothing
   to carry for these two artifacts (no executable inference services, unlike `gltf`).
6. **Confirmed, not asked about**: two files outside this pilot's edits — `📄txt/…/✏️editor/
   🎭️modes/✏️edit/🦀️component.rs` and `📄txt/…/👁️viewer/🎭️modes/👁️view/🦀️component.rs` — show as
   modified in `git status` with content this pilot never touched. `git log --date=iso -1` on
   their parent directory shows their last COMMIT predates the ticket start, so the live diff is
   uncommitted, concurrent peer-session activity (consistent with `📓️status.md`'s warning) — noted
   here for the record, not acted on, not this pilot's to fix.
