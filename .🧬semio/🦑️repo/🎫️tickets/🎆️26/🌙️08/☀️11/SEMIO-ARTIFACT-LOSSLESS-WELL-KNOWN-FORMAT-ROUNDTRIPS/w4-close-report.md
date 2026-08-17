# W4 Closer Report

Agent: W4 closer, following `w4-verify-report.md`'s CONDITIONAL PASS and the six W4 group reports
(G1 brep↔step, G2 mesh↔gltf/stl/obj/ply/las, G3 model↔ifc/bcf + object↔json/xml/csv, G4
drawing↔svg/dxf/pdf + cad↔dxf/dwg/step + image↔png/jpg/gif/bmp/tiff — no report filed, G5
video↔mp4/avi + audio↔mp3/wav + animation↔gltf/mp4/gif, G6 document↔docx/md/txt/pdf +
presentation↔pptx + workflow↔json).

## 1. Fixed: the one real bug (drawing↔pdf, shared `pdf` engine)

Root cause per `w4-verify-report.md` §2: `pdf`'s own shared
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/⚙️engine/🦀️component.rs::extract_text`
never inserted a newline for the `T*` content-stream operator (move to start of next line, PDF
32000-1 §9.4.2), while `encode_pdf` emits exactly `T*`-then-`Tj` for every line after the first in
multi-line text. Fixed by adding a `"T*" if in_text => { if !out.is_empty() { out.push('\n'); } }`
arm to `extract_text`'s operator match (this is the general, spec-correct behavior of `T*` — not a
special case for this one test). Verified:
- `real_byte_round_trip_through_pdf_codec` (the previously-failing test): now passes.
- All 143 pdf-related tests (`cargo test -p semio-s-plugin-stdio --lib "pdf"`) pass, including the
  real-world `bachelor_thesis` fixture decode/encode/decode tests — no regression.

## 2. Fixed: shared-infra gap in `⚙️engine/🧰️triples/🦀️component.rs`

File: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/⚙️engine/🧰️triples/🦀️component.rs`.

- **Spurious `Default` bound**: added `#[serde(bound(serialize = "...", deserialize = "..."))]`
  overrides to both `IndexedTripleDiff<D, T>` and `NamedTripleDiff<K, D, T>`, exactly matching the
  pattern already proven in bcf's own local copy
  (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:27`).
  Added a `serde_json_round_trips_a_non_default_item_type` test with a deliberately non-`Default`
  item type (`NoDefault(u32)`) proving the override actually matters — this test would fail to
  *compile* (not just fail at runtime) without the fix.
- **`NamedTripleDiff::added` positional fidelity**: added a shared `NamedAdded<T> { index, item }`
  wrapper type (mirroring the existing `IndexAdded<T>`) plus generic `enc_named_added`/
  `dec_named_added` codec helpers, hoisted from the reference implementation in
  `s.stdio.object`'s own `🧬️schema/🔺️diff/🦀️component.rs` (`NamedAdded<T>`, `enc_named_added_entry`/
  `dec_named_added_entry`, etc.). Consumers that need positional fidelity for a named/keyed
  collection now instantiate `NamedTripleDiff<K, D, NamedAdded<T>>` and import the shared wrapper
  instead of reinventing it — `NamedTripleDiff`'s own `added: Vec<T>` field type is unchanged (a
  breaking change was neither needed nor requested; most named-collection consumers don't need
  positional fidelity at all). Added a `named_added_round_trips_through_hex_shape` test.
- Per the brief: the 5 subsets that already worked around this locally (bcf's own `NamedTripleDiff`
  copy; brep/mesh/model/object's local `NamedAdded` wrappers) were **not** retrofitted — their local
  copies still work fine. This fix only stops the pattern from being needed for W4/W5's future
  consumers of the shared engine.

Verified: `cargo test -p semio-s-plugin-stdio --lib "engine::triples"` — 6/6 pass (4 pre-existing +
2 new).

## 3. Verifier-flagged issues not fixed (documented as follow-ups)

- **G4 never filed `w4-*-report.md`** (CLAUDE.md violation, verifier §0). This is a process/paper-
  trail gap, not a code defect — the underlying drawing/cad/image code itself is real and
  substantial per the verifier's direct inspection (§0, §3 samples #6–#8). Not backfilled here
  (out of a closer's scope to author another group's first-person work report); flagged for the
  orchestrator. Follow-up for W5.
- **2 confirmed-foreign failures the verifier saw at verify-time** (`png`×4, `zip`×5 conformance-law
  tests from a different concurrent session's uncommitted work) were **not present** in this
  closer's own final gate run — that other session evidently finished/committed its work in the
  interim. No action was needed here.

## 4. `script.ts` shrink-only allowlist sweep (task 3)

Checked every `POLICY_*_ALLOWLIST` constant in `📜️script.ts` for entries that name any of W4's 12
touched subsets (brep/mesh/model/object/cad/drawing/image/video/audio/document/presentation/
workflow) or their new format pairs. Also specifically re-ran the io-leaf-coverage
(`artifact-io/io-matrix-migrated` / legacy `stdio-artifacts/io-matrix`) and composer-dependency
(`artifact-io/composer-dependency`) rules live and grepped the fresh policy output for their `kind`
strings.

**Result: zero entries removed — verified there was nothing to remove.**
- Both `policyIoMatrixMigratedBreaches` and `policyComposerDependencyBreaches` are fully computed
  (disk + catalog lookups), not allowlist-gated — there is no shrink-only allowlist attached to
  either rule to prune. A live `bun ./📜️script.ts policy` run confirms **zero breaches** of kind
  `artifact-io/io-matrix-migrated`, `stdio-artifacts/io-matrix`, or `artifact-io/composer-dependency`
  anywhere in the current tree — W4's io leaves fully satisfy both rules already, with nothing left
  to shrink.
- Grepped every non-empty `POLICY_*_ALLOWLIST` (`POLICY_ROUND_TRIP_TEST_ALLOWLIST`,
  `POLICY_FLAGSHIP_MUTATION_ALLOWLIST`, `POLICY_SNIFF_REALITY_ALLOWLIST`,
  `POLICY_DIFF_COMPLETENESS_ALLOWLIST`, `POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`, etc.) for
  any of W4's 12 subset names or their format pairs — none found. The entries these allowlists do
  carry (e.g. `stdio/mp4/standards#isobmff-...-diff-component`,
  `stdio/gltf/standards#2.0-...-mutations-component`) are about FORMAT artifacts' own
  DiffCodec/mutation-vocabulary completeness (pre-existing gaps from earlier waves, orthogonal to
  io-bridge leaves — io leaves are deserializer/serializer files, they don't touch a format's own
  diff/mutation machinery), not io-leaf coverage.
- Per the brief's own "only remove entries you've verified" instruction, made no speculative edits.

## 5. Full final gate (task 4)

`cargo test -p semio-s-plugin-stdio --lib` (`w4-close-final-cargo-test.txt`):
```
test result: ok. 1657 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 7.89s
```
Fully green — the drawing↔pdf fix cleared the last real failure; the two foreign png/zip failures
the verifier saw are gone (other session finished in the interim, not chased/touched here).

`bun ./📜️script.ts policy` (`w4-close-final-policy.txt`):
```
21532 high-priority breach(es) across 25 rule(s)
```
Byte-identical total to the verifier's own snapshot (`w4-verify-policy-out.txt`, same 21532/25) —
confirms the pdf-engine fix and the shared triples-engine fix introduced **zero new breaches** and
**zero regressions**. Spot-checked the full breach output for any line mentioning
`🧰️triples` or the touched `📄️pdf/🏅️standards/🔖️1.7` tree: only pre-existing, unrelated patterns
(`os-state-authority/item-scope-global` on composer `OnceLock`s, `os-state-authority/authority-
struct-map` on `XrefState`, `stdio-artifacts/composer`, `taxonomy/emoji-prefix` on `📄set-snapshot`)
— all present before this session's edits, none newly introduced by them.

## 6. Files touched this session

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/⚙️engine/🦀️component.rs` — added a
  `"T*"` operator arm to `extract_text` (newline-on-next-line-move), fixing the drawing↔pdf
  round-trip bug for every consumer of this shared engine, not just drawing.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/⚙️engine/🧰️triples/🦀️component.rs` —
  added `#[serde(bound(...))]` overrides to `IndexedTripleDiff`/`NamedTripleDiff`, added shared
  `NamedAdded<T>` + `enc_named_added`/`dec_named_added`, added 2 new tests (6/6 pass total).
- `📜️script.ts` — inspected only, no edits (verified nothing to remove, see §4).

## 7. Remaining gaps for W5

- G4's missing `w4-*-report.md` (process gap, code itself is real — see §3).
- The 5 subsets with local `NamedTripleDiff`/`NamedAdded` workarounds (bcf, brep, mesh, model,
  object) were intentionally left as-is (per the brief) — future W5 consumers should use the shared
  `engine::triples::NamedAdded<T>` directly instead of reinventing a 6th local copy.
- No other open code defects found in W4's 26 delivered pairs beyond what the verifier already
  itemized (all real, honestly-documented, zero-codec-reimplementation bridges).
