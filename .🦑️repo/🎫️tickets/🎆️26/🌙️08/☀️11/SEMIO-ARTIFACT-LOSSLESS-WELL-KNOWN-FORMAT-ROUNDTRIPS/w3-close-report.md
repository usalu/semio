# W3 Closer Report

Closer for W3 (mp4+avi, mp3+wav, epw+tsv, html — 7 new stdio format artifacts). Read
`w3-verify-report.md` and all 4 W3 sub-agent reports (`w3-mp4avi-report.md`, `w3-mp3wav-report.md`,
`w3-epwtsv-report.md`, `w3-html-report.md`) before starting. Baseline: verifier's crate did **not
compile** (`cargo test -p semio-s-plugin-stdio --lib` — 42 own-scope errors: epw 34, mp3 3, wav 3,
html 2), policy **21523/25**.

## 1. Verifier-flagged compile bugs — fixed

All 4 bugs the verifier identified (each a real, own-scope regression, not foreign churn):

1. `🌦️epw/…/🧬️mutations/🦀️component.rs` test module — added `use protocol::MutationDiff;`.
2. `🎵️mp3/…/🧬️mutations/🦀️component.rs` test module — added `use protocol::MutationDiff;` +
   `use protocol::command::DiffAlgebra;` (the verifier's report predicted `MutationDiff` alone;
   actual compiler output additionally needed `DiffAlgebra` for `.inverse()` — both added).
3. `🔊️wav/…/🧬️mutations/🦀️component.rs` test module — same two imports as mp3.
4. `🌐️html/…/📸️snapshot/🦀️component.rs:711` — `include_str!` path had one `../` too many (7 instead
   of 6); fixed.

After these 4, `cargo check -p semio-s-plugin-stdio --lib --tests` compiles clean (0 errors) —
verified twice.

## 2. New runtime bugs found once the crate could finally run tests — fixed

The verifier could only reach `cargo check --tests` (compile-only); it explicitly could not run
`cargo test`. Once (1) unblocked compilation, running each of the 7 artifacts' own scoped test
suites (`cargo test -p semio-s-plugin-stdio --lib "artifacts::<x>::"`) surfaced **9 more real bugs**,
all newly discovered this session, all cheap/mechanical, all fixed:

- **mp4** — `mp4_visual_sample_entry` wrote only 12 zero bytes for the
  `pre_defined(2)+reserved(2)+pre_defined[3](12)` slot before `width`/`height` (spec/read-side
  needs 16) — a 4-byte encode/decode misalignment that shifted every field after it, including the
  nested `avcC` child box, so a freshly-encoded AVC file failed to re-decode
  ("truncated box stream"). Fixed to 16 zero bytes. Also fixed the sibling
  `non_avc_codec_round_trips_via_raw_sample_entry` test, which fed `encode_mp4` a hand-rolled
  20-byte-payload "raw" sample entry — below the 24-byte minimum a real `VisualSampleEntry` needs,
  since `decode_trak` parses that fixed header for every codec branch, not just AVC — now builds a
  real sample entry via the module's own `mp4_visual_sample_entry` helper.
- **avi** — `audio_stream_round_trips_via_wave_format`'s test fixture used an empty
  `fcc_handler: String::new()`; the real codec's `fourcc4` space-pads to 4 bytes on encode (same
  convention as `fcc_type`), so a genuinely empty handler round-trips to `"    "`, not `""` — fixed
  the test's expected value, not the (correct) encoder.
- **avi** — `default_snapshot_round_trips_through_real_codec` built its input via
  `..AviSnapshot::default()` (derived `Default`: `schema: ""`, `reserved: vec![]`), but the real
  codec always stamps `schema` from `STDIO_AVI_DOCUMENT_SCHEMA` on decode and `avih`'s
  `dwReserved[4]` is always 4 real DWORDs on the wire — fixed the test to start from the codec's own
  normal form.
- **mp3** — `frame_header_bit_layout_round_trips` called `parse_frame_header` with only the 4
  header bytes, but the function honestly bounds-checks the WHOLE 417-byte frame (128kbps/44100Hz)
  against the buffer — fixed the test to pad to 417 bytes.
- **wav** — `WavData` was `#[serde(tag = "kind", …)]` (internally tagged) with tuple variants
  wrapping `Vec<T>` — serde cannot serialize an internally-tagged newtype variant wrapping a
  non-map type, the same constraint already on record for `HtmlNode`/`JsonValue` elsewhere in this
  codebase. This made `op_text_binary_roundtrip_law` fail (`print_op` silently returned `""` via
  `unwrap_or_default()`). Fixed by switching to adjacently tagged (`tag = "kind", content =
  "value"`) — zero call-site changes needed.
- **wav** — `codec_retention_law`'s independent re-synthesis formula used a continuous
  `0.5 * 32767.0 = 16383.5` amplitude, but the real fixture generator (`make_wav.py`) truncates
  `int(AMPLITUDE * 32767)` to `16383` BEFORE multiplying by `sin(t)` — a genuine 1-LSB mismatch at
  peak samples. Fixed the test to reproduce the generator's truncation order exactly.
- **epw** and **tsv** — `field_sweep_every_mutable_field_changes` asserted `removed`, `modified`,
  AND `added` all non-empty from a single `between()` call, but both diffs are honestly documented
  as positional ("rows/records have no stable identity beyond position") — a `min_len`-bounded
  comparison can only ever populate `removed` XOR `added` (whichever side is longer), never both, in
  one call. The `sweep_a`/`sweep_b` fixtures happened to be equal-length, so real behavior was
  "modified only", not what the assertions claimed. Fixed both tests to assert what the documented
  positional model actually produces, and added two small dedicated cases (shrink a snapshot →
  `removed` populated; grow a snapshot → `added` populated) so all three variants are still
  genuinely exercised.

All fixes are within the 7 artifacts' own write scope; the wav/mp4 test-fixture reasoning (empty
fourcc round-tripping to spaces, positional-diff XOR semantics) is documented inline at each fix
site rather than silently patched.

## 3. Verification — real numbers, not claims

Per-artifact scoped test runs (evidence: `w3close-scoped-test-<artifact>.txt`):

| Artifact | Tests | Result |
|---|---|---|
| mp4 | 25 | **0 failed** |
| avi | 19 | **0 failed** |
| mp3 | 17 | **0 failed** |
| wav | 16 | **0 failed** |
| epw | 13 | **0 failed** |
| tsv | 14 | **0 failed** |
| html | 25 | **0 failed** |

**129 tests total across the 7 artifacts, 0 failures.**

Full-crate gate (`cargo test -p semio-s-plugin-stdio --lib`, evidence: `w3close-final-gate-cargo-test.txt`):
**1484 passed, 13 failed, 1 ignored**. All 13 failures are confirmed foreign — 4 in `csv`, 5 in
`json`, 4 in `semio`'s `mesh`/`model` subsets (verified via `git status --porcelain`: all in files
dirty from other concurrently-running agents, entirely outside the 7 W3 artifacts' write scope).
Zero failures anywhere under `mp4`/`avi`/`mp3`/`wav`/`epw`/`tsv`/`html`. Re-ran the full crate test
twice; the foreign failure set was stable and identical both times (not flaky/mid-edit churn).

## 4. `📜️script.ts` shrink-only allowlist entries removed

Verified each entry's underlying check before removing (grep for the real signal the policy rule
looks for), then re-ran `bun ./📜️script.ts policy` to confirm no regression.

- **`POLICY_DIFF_COMPLETENESS_ALLOWLIST`** (checks: does the diff type's file contain
  `dsl::DslDiff` or `DiffCodec for`?) — removed **html, epw, mp3, tsv, wav** (all 5 confirmed via
  grep to have a real `impl protocol::DiffCodec for <X>Diff` in the same file). **Kept mp4 and avi**
  — grepped both artifacts' entire trees for `DiffCodec for Mp4Diff` / `DiffCodec for AviDiff`:
  zero matches. Both artifacts implement `MutationDiff`/`DiffAlgebra` for their diff types but
  never gave them a `DiffCodec` impl — a real, still-open gap (their "Op codecs" section is
  `OpText`/`OpBinary` for the *Mutation* type, not `DiffCodec` for the *Diff* type). Removing these
  two would have created 2 new high-priority breaches; left them in place.
- **`POLICY_ROUND_TRIP_TEST_ALLOWLIST`** (checks: does `⚙️engine/🦀️component.rs` itself contain
  `#[cfg(test)]` plus a round-trip-signal test name?) — removed **epw, mp4, mp3, avi, wav** (all 5
  confirmed via grep to have real round-trip-named tests in their own `⚙️engine/🦀️component.rs`).
  **Kept html** — html's `⚙️engine/🦀️component.rs` is only 34 lines (`sniff_real_bytes` + one sniff
  test); its real `codec_retention_law` round-trip test lives in `📸️snapshot/🦀️component.rs`
  instead. This rule is scoped specifically to the engine file, so html's engine file genuinely has
  no round-trip signal under this rule's own detection method — a real architectural mismatch
  between where html's parser lives and where the rule looks, not a fixed-and-forgotten entry.
  Documented as a follow-up (see §6) rather than moved, since relocating html's parser is a design
  decision outside this closer's scope. (`tsv` was never in this allowlist to begin with — nothing
  to remove there.)

Before: `bun ./📜️script.ts policy` → 21524/25 (`w3close-policy-before-allowlist-edit.txt`).
After: 21524/25, byte-identical high-priority summary (`w3close-policy-after-allowlist-edit.txt`,
`diff` of the summary block is empty) — confirms the 10 removed entries were genuinely satisfied
and introduced zero new breaches.

## 5. `catalog.json` `depends` spot check — accurate, left alone

Checked `✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json`'s `stdio_roster` rows for all 7 artifacts:

- `mp4`/`avi`/`mp3`/`wav` → `["binary"]`
- `epw`/`tsv`/`html` → `["txt"]`

Verified accuracy two ways: (1) grepped every one of the 7 artifacts' Rust trees for
`use crate::artifacts::` referencing any sibling artifact outside their own tree — zero matches
(no hidden cross-artifact type/logic dependency, despite mp4avi/epwtsv reports mentioning other
artifacts as *structural templates* they read, never actually import from); (2) grepped for
`DEP_\w+: Dialect` composer-level dependency declarations — none of the 7 declare any. `binary`/
`txt` are the correct, minimal, genuinely-accurate dependencies (real binary codecs wrapping raw
bytes for the first 4; real text codecs for the last 3). `stdio_dag_edges` matches the roster
exactly for all 7. No fix needed — catalog structure itself is W1b's job; this was a depends-value
accuracy check only, and it checked out.

## 6. Final gate

- `cargo test -p semio-s-plugin-stdio --lib 2>&1 | tail -20` → **1484 passed; 13 failed** (all 13
  foreign — csv/json/semio-mesh/semio-model). Evidence: `w3close-final-gate-cargo-test.txt`.
- `bun ./📜️script.ts policy 2>&1 | tail -50` → **21524 high-priority breaches / 25 rules**.
  Evidence: `w3close-final-gate-policy.txt`.

## 7. Follow-ups (design judgment, not fixed this session)

1. **mp4/avi have no `DiffCodec` impl for their Diff types** — `POLICY_DIFF_COMPLETENESS_ALLOWLIST`
   entries kept for both. Needs a hand-rolled `impl protocol::DiffCodec for Mp4Diff`/`AviDiff`
   (bracket/hex grammar, same pattern already used by their `OpText`/`OpBinary` Mutation codecs) —
   real implementation work, not a mechanical fix.
2. **html's round-trip test lives in the wrong file for `POLICY_ROUND_TRIP_TEST_ALLOWLIST`'s
   detection method** — its real codec is in `📸️snapshot/🦀️component.rs`, not
   `⚙️engine/🦀️component.rs` (html has no ISO-BMFF/RIFF-style box-walking engine layer the way
   mp4/avi/mp3/wav do). Either move/duplicate a round-trip-signal test into html's engine file, or
   widen the policy rule to also scan the snapshot file for html-shaped artifacts. Left the
   allowlist entry in place with this reasoning documented inline in `📜️script.ts`.
3. Both follow-ups are pre-existing (not new regressions introduced this session) and orthogonal to
   the ticket's core "real codecs" goal, which all 7 artifacts now genuinely satisfy.

## Files touched

**Bug fixes (7 artifacts' own trees):**
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`

**Allowlist cleanup:**
- `📜️script.ts` (`POLICY_DIFF_COMPLETENESS_ALLOWLIST`: −5 entries; `POLICY_ROUND_TRIP_TEST_ALLOWLIST`: −5 entries)

**Not touched (verified, no change needed):**
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json`

**Ticket evidence files (this folder):**
- `w3close-check-lib-tests.txt`, `w3close-scoped-test-{mp4,avi,mp3,wav,epw,tsv,html}.txt`,
  `w3close-policy-before-allowlist-edit.txt`, `w3close-policy-after-allowlist-edit.txt`,
  `w3close-final-gate-cargo-test.txt`, `w3close-final-gate-policy.txt`, this report.

No `git commit`/`stash`/`checkout` run. No `ticket_close` called by this report (append to STATUS.md
and hand off per the parent task's own instructions).
