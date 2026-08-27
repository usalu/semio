# W8 — semio Pattern-A ✳️audio / ✳️video / ✳️animation exhaustive mutation cases

Ticket 26/08/23/END-TO-END-TESTING-REFACTOR. Scope: the three 🧿️semio Pattern-A subsets
`✳️audio`, `✳️video`, `✳️animation`.

## 1. Step 1 of the brief does not apply here — with evidence

The brief's first step ("if it has no `🧬️schema/🧬️mutations` of its own, HANDCRAFT one") is moot
for all three. Each already OWNS a full named-variant vocabulary and does not delegate to `✳️any`:

| Subset | Enum | Variants | Wire tag table |
|---|---|---|---|
| `✳️audio` | `SemioAudioMutation` | 10 | `OP_KEYWORDS: [&str; 10]`, kebab-case |
| `✳️video` | `SemioVideoMutation` | 9 | `OP_KEYWORDS: [&str; 9]`, kebab-case |
| `✳️animation` | `SemioAnimationMutation` | 13 | `OP_KEYWORDS: [&str; 13]`, two-letter tags |

None of the three re-exports `✳️any`'s schema; each has its own `📸️snapshot`, `🔺️diff`,
`🧬️mutations`, `💡️inferences` facets, its own hand-rolled `OpText`/`OpBinary` codecs and its own
`📄set-snapshot` fixture leaf. The gap these three had was the one the rest of this step names:
no `pub const KINDS`, no `🧪️oracle` manifest, no test case.

## 2. What was added

Per subset:

* `🧬️schema/🧬️mutations/🦀️component.rs` — `pub const KINDS: &[&str]` beside the enum, plus a plain
  `#[test] kinds_match_the_enum_and_the_catalog`. For audio and video it asserts `KINDS ==
  OP_KEYWORDS`; for animation, whose wire tags are abbreviations (`IT`, `KV`, …), it asserts equal
  length and positional agreement instead. All three additionally assert a bijection between the
  declared variants and the module's own per-variant case list (`all_variants` / `sample_mutations`
  / `demo_mutation_cases`) via `variant_ordinal`, and that every `KINDS` entry appears in the
  committed oracle manifest.
* `🧪️oracle/🔣️.json` — one recorded `noOracleDecision` plus one `mutationCatalog`
  (`semio-v1-audio` / `semio-v1-video` / `semio-v1-animation`).
* Two free-function bridges, because the generated test host links only `semio-s-plugin-stdio` and
  cannot name `protocol`/`store` (both are `extern crate semio_framework_os_kernel as …` aliases
  private to `📦️glue.rs`, so `use protocol::Mutation;` — the shape the W7 semio adapters used — is
  not resolvable from a host):
  * `inverse_semio_<subset>_mutation(&mutation, &base) -> Vec<…>` beside `apply_semio_<subset>_mutation`;
  * `parse_semio_<subset>_dsl` / `print_semio_<subset>_dsl` beside the `store::ArtifactDsl` impl.
* `🧪️tests/mutate-semio-<subset>/` — `component.feature`, `🦀️component.rs`, and one committed
  `(kind, params, before, after)` specification vector per kind under `🧫️fixtures/🦠️<kind>.json`,
  each declared as a `local://` URI so both roles read the same bytes through
  `Context::fixture_json`. 32 vectors in total (10 + 9 + 13).

## 3. Why a recorded no-oracle decision, not a registered oracle

`.dsl.semio` / `.pack.semio` is this repository's own envelope; no library in any ecosystem reads
it, so the survey was done one level down, on the interchange formats each subset imports from. The
cross-language hosts HAVE landed (Python venv + npm TypeScript host, see
`📓️cross-language-oracle-hosts-2026-08-24.md`), so Python and JavaScript candidates were checked
too and fail for the same reason one level earlier.

| Subset | Candidate surveyed | Why it cannot judge this vocabulary |
|---|---|---|
| audio | `hound` 3 (already the wav oracle) | Round-trips RIFF PCM, so it could judge 4 of 10 kinds; models NO `LIST INFO` chunk, so `insert-tag` / `remove-tag` / `set-tag-value` would have had nothing to compare against. It is also the wrong altitude: `SemioAudioFormat` records the ORIGINAL encoding the always-`f32` samples were decoded FROM, which a WAV writer cannot represent independently of its own storage. |
| video | `mp4` 0.14, `riff` 2.0 (both registered here) | Both round-trip CONTAINERS. This snapshot is container-neutral and payload-OPAQUE — a sample's `data` is an uninterpreted byte string, `rate` an exact rational, and a stream carries no track id, edit list or codec-configuration record. Judging through either would mean inventing fields the snapshot does not carry. |
| animation | `gif` 0.13 (registered here); no glTF crate is registered at all | `gif` exposes per-frame delays and nothing else — no channel target, no interpolation mode, no quaternion, no morph-weight vector — so 9 of 13 kinds would have had nothing to compare against. |

In each case registering the crate anyway would have left the uncoverable kinds comparing our
output against our own output, which the fleet brief names as the precise failure to avoid. The
decisions therefore rest on `specification-vectors` + `metamorphic-laws`, exactly as the seven
already-landed semio Pattern-B cases do.

## 4. The fixture is real

For a semio-native format the most real input available is this standard's own committed artifact,
and that is what every vector starts from. Each subset's BEFORE snapshot is the exact decoded
content of a committed `🗣️example.dsl.semio`:

| Subset | Real committed artifact | What it carries |
|---|---|---|
| audio | `✳️any/📚️examples/🎵️tone/🖼️assets/🗣️example.dsl.semio` (226 B) | 44.1 kHz stereo `f32`, two 4-sample channels, one `title` tag |
| video | `✳️any/📚️examples/🎥️clip/🖼️assets/🗣️example.dsl.semio` (172 B) | 1920×1080 30/1 h264 track of two samples beside a 48000/1000 aac track |
| animation | `✳️any/📚️examples/🚶️walk/🖼️assets/🗣️example.dsl.semio` (278 B) | one named timeline whose four channels cover every `AnimTargetProperty`, every `AnimInterpolation` and every `AnimValue` shape |

Each case also carries an `@id-identity-round-trip @level-long @mode-round-trip` scenario that reads
that very file by `asset://` and parses → prints → parses it through the subset's own DSL codec, so
the real bytes are in the loop rather than merely described.

**This claim was verified, not asserted.** `semio-av-anim-mutate/📊️verify-before-snapshots.py`
decodes all three committed artifacts from the wire grammar alone, in a different language from the
implementation, and compares against every committed vector:

```
mutate-semio-audio       10 vector(s), 0 before-snapshot mismatch(es)
mutate-semio-video        9 vector(s), 0 before-snapshot mismatch(es)
mutate-semio-animation   13 vector(s), 0 before-snapshot mismatch(es)
TOTAL MISMATCHES 0
```

## 5. Verification actually run

```
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-audio        # exit 0
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-video        # exit 0
$ bun ./📜️script.ts contract --owner 🗄️stdio --case mutate-semio-animation    # exit 0
```

A full-owner `contract` run reports zero breaches naming any of the three subsets or cases; the
breaches it does report (ifc, json, flow, cad, document) belong to peer sessions in flight.

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-audio
[test] not-exercised …/mutate-semio-audio (recorded no-oracle decision semio-audio-mutation-semantics
       — its evidence is discharged by the subject phase)
[test] level=exhaustive cases=1 executed=0 passed=0 failed=0 errored=0 parity=0/0 not-exercised=1
                                                                                        # exit 0
```

Identical output for `mutate-semio-video` and `mutate-semio-animation`. **`executed=0` is the
correct and expected result, not a failure**: `oracleDecision` in `📜️script.ts` returns
`implementation: null` for any feature tagged `@no-oracle-…`, so the oracle phase runs nothing at
all for a recorded-no-oracle case. The seven semio Pattern-B cases landed in W7 behave exactly the
same way and are counted among that wave's 9 `not-exercised`.

## 6. Honest limits

* **The Rust subject phase could not be run, and neither could the new `#[test]`s.** The workspace
  does not compile: `semio-framework-job` fails with 6 errors (`ManuallyDrop<Option<…>>` vs
  `Option<…>` at `🦀️component.rs:513/523/546/650`, a missing `WorkerJobSession::generation`, and a
  double mutable borrow at :489) — the live `RetainedJobPayload` refactor the fleet brief warned
  about. `semio-s-plugin-stdio` is never reached. Every `🦀️component.rs` this task touched was
  syntax-checked with `rustfmt --edition 2021 --check` (all parse), but nothing was type-checked and
  `kinds_match_the_enum_and_the_catalog` has never executed.
* **`executed=0` in the oracle phase is structural, not incidental.** No amount of work on these
  three cases changes it while they rest on a no-oracle decision; the evidence is discharged by the
  subject phase, which is blocked. Both facts are visible in the output above rather than papered
  over.
* **`📊️verify-vectors.py` is a typo guard, not evidence.** It replays every vector through mutation
  semantics TRANSCRIBED from each subset's own Rust `diff`/`inverse` impl and reports 0 forward and 0
  inverse mismatches across all 32 vectors. Because the semantics are read off the implementation,
  agreement is a transcription check, not an oracle — it catches a hand-authoring slip while the
  Rust phase is down, and claims nothing more.
* **Two free-function bridges were added to production code.** They are additive, mirror the
  existing `apply_semio_<subset>_mutation` precedent, and exist because the generated host provably
  cannot name the os-kernel traits. The W7 semio adapters' `use protocol::Mutation;` will not
  resolve in a host crate; those adapters have never been compiled, so the problem has not surfaced
  yet. Worth flagging to whoever unblocks the subject phase.

## Files

Production (modified):
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️{audio,video,animation}/🧬️schema/🧬️mutations/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️{audio,video,animation}/🧬️schema/📸️snapshot/🦀️component.rs`

Production (new):
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️{audio,video,animation}/🧪️oracle/🔣️.json`

Cases (new):
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-{audio,video,animation}/component.feature`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-{audio,video,animation}/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-{audio,video,animation}/🧫️fixtures/🦠️<kind>.json` (32 files)

Ticket scratch:
- `semio-av-anim-mutate/📊️verify-before-snapshots.py`
- `semio-av-anim-mutate/📊️verify-vectors.py`
