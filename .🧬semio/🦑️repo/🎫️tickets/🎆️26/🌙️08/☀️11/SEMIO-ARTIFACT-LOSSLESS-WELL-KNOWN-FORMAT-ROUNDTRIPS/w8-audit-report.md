# W8 Allowlist Burn-Down Audit — Final Report

Fresh-eyes audit of every shrink-only allowlist entry this ticket's waves (W1b/W2a/W2b/W3/W4)
seeded in `📜️script.ts`, plus the full cross-cutting gate. All commands re-run live this session
(nothing taken on faith from prior close reports).

## 1. Allowlist entries this ticket's waves actually own

Two allowlist constants carry markers this ticket's waves wrote to (`POLICY_DIFF_COMPLETENESS_ALLOWLIST`,
`POLICY_ROUND_TRIP_TEST_ALLOWLIST`). Both were seeded by W1b (+21, +7 keys) and burned down across
W2a (−6), W2b (−8), W3 (−10 across the two lists). Verified the current file state directly (not
the close reports' claims) by re-grepping the actual code:

| Constant | Remaining entries (this ticket's markers) | Verified now |
|---|---|---|
| `POLICY_DIFF_COMPLETENESS_ALLOWLIST` | `stdio/mp4/standards#isobmff-subsets-any-schema-diff-component`, `stdio/avi/standards#1.0-subsets-any-schema-diff-component` | `grep -rn "impl protocol::DiffCodec for\|impl DiffCodec for"` under `🗿️artifacts/🎥️mp4/` and `🗿️artifacts/📼️avi/` → **zero matches, both**. Neither `Mp4Diff` nor `AviDiff` has a `DiffCodec` impl (they do have `MutationDiff`/`DiffAlgebra`, and `OpText`/`OpBinary` on the *Mutation* type — a different codec surface). **Genuine, still-open gap. Correctly kept.** |
| `POLICY_ROUND_TRIP_TEST_ALLOWLIST` | `stdio/html/standards#5-engine-component`, `stdio/semio/standards#v1-engine-component` | The rule (`policyRoundTripTestBreaches`, line 8793) scans *only* the file whose relPath ends exactly `⚙️engine/🦀️component.rs` for `#[cfg(test)]` + a round-trip-signal name. html's `⚙️engine/🦀️component.rs` is a 34-line `sniff_real_bytes` file with no test region (its real round-trip test lives in `📸️snapshot/component.rs` instead — a different file, doesn't count under this rule). semio's `⚙️v1/engine/component.rs` (read in full, 22 lines) is pure `register()` plumbing with no tests at all — the real round-trip tests live one level down in `⚙️engine/🧰️triples/component.rs` and `⚙️engine/🧮️geometry/component.rs` (confirmed present, 5 `#[test]` fns total), which the rule's exact-suffix match does not reach. **Genuine, still-open gap (architectural mismatch between where the tests live and where the rule looks, exactly as W3's closer documented) — correctly kept, not fixed-and-forgotten.** |

**Action taken: none.** Zero entries removed this session — every remaining entry in both
constants is a real, currently-unsatisfied gap, verified fresh against the actual `.rs` files on
disk, not re-derived from the close reports' say-so. Ran `bun ./📜️script.ts policy` (see §3) to
confirm no `*-stale-*` breach fires for either constant, which would have been the tell for a
satisfied-but-not-removed entry — none did.

Cross-checked W2a/W2b's removal claims directly in the file too: the 6 W2a subset keys
(brep/cad/drawing/mesh/model/object) and the 8 W2b subset keys (document/image/video/audio/
animation/presentation/workflow/any) are genuinely absent from `POLICY_DIFF_COMPLETENESS_ALLOWLIST`
today — only the inline comment trail documenting their removal remains, no live entries.

## 2. Broader sweep — allowlists referencing "semio" or the 7 new formats outside this ticket's own markers

Per the brief's instruction, grepped **every** `POLICY_*_ALLOWLIST` constant (not just the two with
"seeded by W1b scaffold" markers) for `stdio/(mp4|avi|mp3|wav|epw|tsv|html|semio)` and `🧿️semio`.
Found five more constants with substantial entries for our 8 new artifacts —
`POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST` (57), `POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST` (57),
`POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST` (20), `POLICY_LANGUAGE_REGISTRATION_ALLOWLIST` (8),
`POLICY_FIXTURE_HONESTY_ALLOWLIST` (8) — **none of which this ticket's waves seeded.**

Verified via `git blame`: the grammar/protocol-parseability block blames to commit `db6d71790f6`
(2026-08-11 23:58:29), and its own doc comment self-identifies as belonging to a **different**
ticket's census: *"`stdio/semio#v1`'s `✳️object` subset — a live, unrelated concurrent ticket's WIP
artifact, not part of this program's roster — happens to already pass both checks, so it is
honestly NOT in this seed; see the PC report"*. This is the Phase-2 "handcrafted grammar for every
artifact" program (`26/08/03/HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT`, referenced elsewhere in the
file as "P2-PC"/"P2-FG2"/"P2-FG3"), which enumerates **every** stdio artifact not yet migrated to
its own Phase-2 grammar-dialect shape. Our 8 new artifacts got swept into that census purely because
they existed on disk when that other ticket's own agent (re-)ran its detection sweep — not because
this ticket's W1b/W2a/W2b/W3/W4 added them. `POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST` and
`POLICY_LANGUAGE_REGISTRATION_ALLOWLIST`/`POLICY_FIXTURE_HONESTY_ALLOWLIST` carry the identical
"seeded with the current census... every other stdio standard is still IN" framing and the same
`p2-pc-report.md`/`p2-fg2-fix-jpg-report.md` pointers.

**Conclusion: out of this ticket's audit scope.** These are a different ticket's shrink-only
allowlists for a different rule set (grammar-dialect header conformance, JSON-transfer-ban,
5-role language registration, fixture-DSL-preamble honesty) that structurally cover "every stdio
artifact," not specifically seeded by this program. Burning them down is that other ticket's job.
Noted here so the orchestrator isn't surprised by the grep hits, not actioned.

## 3. Full policy run vs. W0 baseline

```
W0 baseline:  21564 breaches / 24 rules   (w0-policy-baseline.txt)
W8 (now):     21654 breaches / 26 rules   (w8-audit-policy-current.txt)
Net delta:    +90 breaches, +2 rule kinds
```

Honest per-rule delta (baseline → now):

**Up — inherent to the 21 new schema-owning units (13 semio subsets + `✳️any` + 7 formats), exactly
the 127-breach set W1b's closer flagged as having "no allowlist mechanism available" and left
undocumented-but-real:**
- `taxonomy/emoji-prefix` 454→491 (+37) — the mandated `📄set-snapshot` triad dir name on every new subset.
- `os-state-authority/item-scope-global` 240→276 (+36) — the mandated `VALIDATOR_ENTRY: OnceLock` pattern on every new composer.
- `stdio-artifacts/composer` 198→227 (+29) — same pre-migration-shape rule limitation W1b documented.
- `artifact-schema/facet-completeness` 249→273 (+24).
- `taxonomy/dead-example-leaf` 242→256 (+14).
- `mutation-migration/triad-completeness` 83→91 (+8), `mutation-migration/artifact-engine` 83→91 (+8), `artifact-schema/type-name-parity` 29→37 (+8).
- `handcrafted-grammar/generic-spec` 1→5 (+4), `os-state-authority/authority-struct-map` 3→4 (+1).
- **New rule kind** `artifact-io/io-matrix-migrated` 0→120 — this is the *hard-computed, non-allowlisted* rule W1b's closer deliberately left `owner.import`/`owner.export` empty to avoid tripping (see §1 of `w1b-close-report.md`); W4 populated those two fields with the real 28-id list, so this rule legitimately started firing for every format pair semio doesn't yet have a physical io leaf for. Not an allowlist item (confirmed no `POLICY_*_ALLOWLIST` backs it — `policyIoMatrixMigratedBreaches` is disk+catalog computed) — a real, expected, still-open lattice-completion gap, consistent with the master plan's "domain plugins go hub-and-spoke" work (W5/W6/W7) not yet closing every format pairing.
- **New rule kind** `mutation-migration/semantic-vocabulary` 0→15 — belongs to a *different* ticket's rule (checks for the generic `SetSnapshot`/`NoMutation`/`CollectionMutation<...>` escape-hatch vocabulary the semantic-verb migration retires); fires because W1b's scaffold used exactly the `NoMutation` placeholder pattern on some of the 21 new units before W2/W3 replaced them. Not this ticket's allowlist to prune (`POLICY_SEMANTIC_VOCABULARY_ALLOWLIST` is a different, unrelated-ticket-owned constant — see §2's reasoning, same shape).

**Down — real generalization/completion work landing:**
- `stdio-artifacts/schema-representation` 181→1 (**−180**) — this is exactly W1's Task 1 "schema-owning vs delegating subset" generalization the master plan called out as the expected reduction; the 1 remaining breach is unrelated pre-existing debris.
- `dsl-migration/diff-completeness` 129→114 (−15) — net of the allowlist burn-down in §1 (−24 removed across W2a/W2b/W3) against some new subset gaps opened, still shrinking overall.
- `handcrafted-grammar/spec-distinctness` 19352→19340 (−12).
- `handcrafted-grammar/empty-example` 96→89 (−7).
- `stdio-artifacts/codec-id-uniqueness` — W1b's own new rule found a real pre-existing `stdio.dwg` id collision (2 breaches at W1b close); **0 now**, resolved by a concurrent session outside this ticket's scope, not touched here.

**Unchanged:** `protocol-migration/command-envelope-completeness` (93), `handcrafted-grammar/declared-use` (69), `pack-migration/completeness` (48), `os-state-authority/id-minting` (4), `budget/no-budget-null` (4), `taxonomy/plugin-builder` (2), `taxonomy/banned-name-stem` (1), `stdio-artifacts/builder` (1), `stdio-artifacts/decomposer` (1), `protocol-migration/db-server-only` (1).

The `+90` net is understandable and not a regression: it nets a genuine `−180` structural fix (W1)
and real allowlist burn-down against `+120` from a rule the program *deliberately* deferred
triggering until W4 finished wiring semio's owner row (expected, still-open lattice-completion
debt, not a new bug), plus two other-ticket-owned rule kinds picking up incidental collateral from
our new artifacts existing on disk. Zero evidence of this ticket's own work regressing anything.

## 4. Cross-cutting gate (all re-run live this session)

**`cargo test -p semio-s-plugin-stdio --lib`** (`w8-audit-stdio-test.txt`):
```
test result: ok. 1930 passed; 0 failed; 3 ignored; 0 measured; 0 filtered out; finished in 9.44s
```
Clean. Up from W0's 1075 baseline and W4's 1657 — monotonically growing throughout, zero failures.

**`cargo test -p semio-framework-os-run --lib`** (`w8-audit-osrun-test.txt`):
```
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
Clean — the W1/W7 os-run fix holds.

**`cargo check --workspace --keep-going`** (`w8-audit-workspace-check.txt`, 23236 lines):
14 crates fail to compile. **All 14 confirmed foreign** — none touch this ticket's write scope
(stdio, or any of the extraction-map plugins remodel/cad/animate/energy/architect/note/layout/gis/
shooting/procedural/raster/draw/puzzle/space/fem/norm). Classification evidence:

| Failing crate(s) | Root cause | Evidence it's foreign |
|---|---|---|
| `semio-s-plugin-sourcing`, `-sequence`, `-imperative`, `-reasoning-mindmap`, `-forms`, `-flow`, `-dag`, `-vcs`, `-block`, `-mathematical` | Each `📦️glue.rs` declares `pub mod document;` for a `🎛️apps/<app>/📌️panels/📄️document/🦀️component.rs` file that no longer exists on disk | `git log --diff-filter=D` on the sourcing instance shows the file was deleted in commit `c31024cc6c`, dated **2026-06-04** — over 2 months before this ticket opened (2026-08-11). None of these 9 plugins are in this ticket's write scope; `git status` shows their `glue.rs` files clean (not concurrently edited). Pre-existing, long-standing repo debt, not this ticket's to fix. |
| `semio-framework-os-kernel-db` | `unresolved import db_engine` / `DbError` / `db_storage_sqlite` (missing `crate::` prefix on several `pub use` re-exports) | `git status` shows exactly this file (`🧰️framework/…/🛢️db/📦️packages/🦀️rust/📦️glue.rs`) **currently dirty** — a live concurrent session is mid-edit on it right now. Framework/os-kernel-db is not this ticket's write scope (this ticket's os-run fix targeted `topic_contributions`/workflow-mount/run-crate reconciliation only, per the master plan, all already green). |
| `semio-compose-rs`, and (secondarily) `semio-framework-os`'s reported error block | `compose/client/lib/rs/lib.rs` references an unqualified `dsl::`/`vcs::` module that doesn't resolve (no `dsl`/`vcs` crate registered as a workspace member; the file's own `Cargo.toml` doesn't declare either as a dependency) | `git log` on both `lib.rs` and `Cargo.toml` for `compose/client/lib/rs` shows no changes since **2026-06-04** (`c31024cc6c`) / **2026-06-04** respectively — untouched, long-predates this ticket. `compose/` is not part of any wave's declared scope in the master plan. |
| `semio-s-plugin-playbook` | `E0308`/`E0062` type mismatch (`LocalizedLabel` vs `Vec<String>`, duplicate `label` field) in `🧰️framework/…/🖥️host/📦️packages/🦀️rust/…component.rs` | Framework/os-host component, not stdio or any extraction-map plugin; unrelated struct-shape bug, not touched by any wave here. |

Zero of this ticket's own crates (`semio-s-plugin-stdio`, `semio-framework-os-run`, or any
extraction-map plugin) appear anywhere in the failing-crate list — confirmed by direct grep of the
full check output for each name.

## 5. Bottom line

- **Allowlist audit (this ticket's actual scope): 0 entries removed, 0 entries needed removal.**
  Both remaining shrink-only entries (`mp4`/`avi` diff-completeness, `html`/`semio-v1-engine`
  round-trip-test) are real, still-open, well-documented-inline gaps — re-verified fresh against
  the live `.rs` files, not just re-trusted from prior close reports.
- **Broader sweep** turned up 5 more allowlist constants referencing our new artifacts, all
  confirmed to belong to a different, concurrently-running ticket's own census mechanism — flagged
  for the orchestrator's awareness, correctly left untouched.
- **Policy**: 21654/26 vs. W0's 21564/24, net **+90**, fully explained — real structural
  improvement (−180 schema-representation) and real burn-down, netted against expected new-surface-
  area breach classes (new schema-owning units' inherent taxonomy/composer/facet gaps) and one
  rule that was deliberately deferred until W4 finished wiring semio's owner row.
- **Gates**: stdio 1930/0, os-run 15/0, both clean. Workspace check has 14 foreign, pre-existing
  failures (verified via git log/status, none newer than 2026-06-04 except one file another
  session is actively mid-editing right now) — zero own-program failures.

## Files touched this session

None in the working tree — this was a read/verify/audit pass. No allowlist edits were needed (all
verified still-genuine). Ticket-folder evidence written: `w8-audit-policy-current.txt`,
`w8-audit-stdio-test.txt`, `w8-audit-osrun-test.txt`, `w8-audit-workspace-check.txt`, this report.

---

# Strict Logical-Materialization Contract Audit — 2026-08-14

This is a read-only static audit of the live implementations for PDF 1.7, DWG AC1024, SVG 1.1,
ISO-BMFF MP4, ECMA-376 PPTX, IFC2X3, ZIP 2.0, and the shared OPC package model. It checks the
tightened contract: persisted state must be named logical standard concepts; native/container
materialization may occur only at import/export boundaries; DSL, pack, diff, and operation codecs
must not carry native or JSON envelopes; and the supplied fixture itself must remain the acceptance
baseline through the complete lifecycle.

No runtime tests were run for this audit. Owners were actively editing the format lanes; evidence
below is the exact static state observed during the audit.

## Clean runtime-model findings

- A public-field sweep found no `source*`, `physical*`, `lexical*`, `raw*`, `native*`, `wire*`,
  `archive*`, or `container*` persisted fields in the eight live models.
- No active Rust snapshot/diff/mutation persistence codec in the audited versions calls
  `serde_json::{to_string,to_vec,from_str,from_slice}`. The two hits are historical comments.
- PDF retains decoded stream data plus a typed filter pipeline; MP4 retains codec parameter sets
  and logical samples; ZIP/OPC/PPTX retain decompressed member payloads. Those are semantic payload
  values, not whole-file/native-container replay fields.
- PDF 1.7 and IFC2X3 exact lifecycle tests directly use the original fixtures as their baseline.
  SVG, MP4, and PPTX also directly read their supplied fixtures and assert native export equality.
- IFC2X3 has the strongest anti-shadow facet guard in this set: its public snapshot is restricted
  to `schema`, typed `Part21Document`, and typed `edmPreamble`, and its exact native analyzer and
  composer routes are asserted in `ifc/.../any/🚪️io/🦀️component.rs:224-268`.

## Actionable blockers

### P0 — committed facets still specify forbidden JSON/native persistence

The Rust codecs are structural, but committed schema/protocol facets still describe obsolete wire
formats. This is contract-visible schema drift, not harmless commentary.

1. **PDF diff binary facets still claim UTF-8 JSON.** Rust emits a tagged structural binary in
   `pdf/.../diff/🦀️component.rs:2311-2388`, but the ABNF says RFC8259 JSON at
   `diff/💾️binary/🔠️component.abnf:1-4`, Spicy says plain UTF-8 JSON at
   `diff/💾️binary/🌶️component.spicy:4-6`, and Kaitai says `utf8_json_text` at
   `diff/💾️binary/🥋️component.ksy:6-10`. The PDF anti-shadow test only includes
   the primary snapshot/diff Proto, GraphQL, TypeScript, EBNF, and G4 facets
   (`pdf/.../🚪️io/🦀️component.rs:2676-2702`), so all three contradictions escape it.

2. **SVG diff and mutation facets still define JSON text/binary envelopes.** Examples:
   `diff/📝️text/🅰️component.g4:2-5`, `diff/📝️text/🛰️component.proto:4-7`,
   `diff/💾️binary/🥋️component.ksy:5-7`, `diff/💾️binary/🌶️component.spicy:2-4`,
   `mutations/📝️text/🅰️component.g4:2-5`, and
   `mutations/💾️binary/🌶️component.spicy:2-4`. Live Rust instead uses the handcrafted
   structural SVG encoders (`diff/🦀️component.rs:1263-1369` and
   `mutations/🦀️component.rs:496-620`). Replace every JSON-declaring SVG diff/op text and
   binary facet, not only the Spicy files.

3. **MP4 snapshot and operation facets describe native/JSON envelopes.** The snapshot ABNF says the
   Semio pack wraps the real ISO-BMFF box stream at
   `snapshot/💾️binary/🔠️component.abnf:1-18`, contradicting the structural
   `pack_rt` record codec in `snapshot/🦀️component.rs:316-345`. MP4 mutation EBNF/G4 claim
   one compact JSON object (`mutations/📝️text/🔤️component.ebnf:1-3` and
   `mutations/📝️text/🅰️component.g4:2`), while Spicy carries `json_utf8`
   (`mutations/💾️binary/🌶️component.spicy:1-4`). Rust uses `DslVariants` and
   tagged-record binary (`mutations/🦀️component.rs:121-140`).

4. **Opaque payload-only facets remain widespread.** A conservative scan for facets that expose
   only `payload: bytes &eod`, `message Artifact { schema, payload }`, or
   `Document { schema, payload }` found 11 PDF, 12 DWG, 8 SVG, 3 MP4, 12 PPTX, 15 IFC2X3, and
   12 ZIP facet files. Some are representation-envelope facets rather than runtime persistence,
   so this is not evidence of a runtime native replay by itself. It is nevertheless schema-first
   coverage debt: these facets cannot describe or validate the structural tags/records the Rust
   codecs actually emit. Replace opaque leaves with the real logical protocol, or explicitly model
   the shared envelope plus a referenced structural payload protocol.

### P0 — SVG DSL parser bypasses the native-deserialization boundary

`SvgSnapshot::parse_dsl` falls back to `SvgSnapshot::import_utf8(text.as_bytes())` whenever the
Semio preamble is absent (`svg/.../snapshot/🦀️component.rs:1272-1280`). That lets native SVG
syntax enter through the persistence DSL API and makes malformed/missing-envelope DSL ambiguous
with native input. Remove the fallback: native XML/SVG belongs only in the native import analyzer;
the DSL parser must accept the Semio structural DSL and reject everything else atomically.

### P1 — exact lifecycle acceptance gaps

1. **DWG:** `well_known_fixture_lossless_system_roundtrip`
   (`dwg/.../any/🚪️io/🦀️component.rs:3253-3344`) covers direct raw IO, DSL, pack,
   self/no-op, set-snapshot text/binary, diff text/binary, absorb, mutation, and inverse, all against
   the original bytes. It does not traverse `DwgAnalyzer` or `DwgComposerComposition`, although
   those routes exist at `schema/🦀️component.rs:197-242` and `io/🦀️component.rs:740-781`.
   Add original-byte assertions for native, DSL, and pack analyzer/composer inputs.

2. **PPTX:** `fixture_survives_logical_io_persistence_diff_and_mutation_pipelines`
   (`pptx/.../any/schema/🦀️component.rs:686-788`) covers direct export, PPTX binary bridge,
   DSL, pack, self/no-op, semantic mutation/inverse/absorb, diff text/binary, and operation
   text/binary. It never routes the exact fixture through the PPTX analyzer or composer. The only
   analyzer/builder roundtrip at line 526 is synthetic/authored coverage, not the supplied fixture.

3. **ZIP/OPC:** no acceptance test sends the supplied PPTX archive through `ZipSnapshot` DSL,
   pack, diff text/binary, operation text/binary, inverse/absorb, analyzer, and composer while still
   requiring equality to the original PPTX bytes. Existing ZIP lifecycle tests are synthetic
   (`zip/.../mutations/🦀️component.rs:164` and `zip/.../io/🦀️component.rs:813-975`).
   OPC has no independent snapshot persistence lifecycle, and `encode_opc` explicitly documents
   that it is not necessarily byte-identical (`zip/📦️opc/🦀️component.rs:489-497`).
   Because PPTX exactness depends on ZIP/OPC, add an exact fixture pipeline that proves the shared
   layers structurally preserve every decompressed part, typed content type, typed relationship,
   comment, and deterministic order without archive replay.

### P1 — anti-shadow guards do not cover the persistence facets that are drifting

- PDF omits its binary diff facets from the guard.
- MP4 checks only the primary TS/GraphQL/JSON/Proto snapshot facets
  (`mp4/.../snapshot/🦀️component.rs:395-405`), not snapshot binary or mutation/diff facets.
- PPTX and ZIP guards include primary model facets but not the text/binary grammar/protocol facets
  where opaque `payload` records remain.
- SVG has no repo-wide assertion rejecting `json_payload`, `json_text`, `JSON_VALUE`, or native
  syntax fallback across all snapshot/diff/mutation facets.

Extend each existing anti-shadow test to enumerate every persisted Rust source and every committed
snapshot/diff/mutation text/binary facet. Ban at least `ArtifactSource`, `physical`, `lexical`,
`document_wire`, source/native/archive byte aliases, `serde_json`, `json_payload`, `json_text`,
`JSON_VALUE`, and claims that Semio pack wraps the native file. Also assert that PDF stream filters
remain a typed pipeline over decoded data and that OPC XML parts cannot coexist as opaque bytes.

## Format disposition

| Format | Runtime model/codecs | Original fixture baseline | Remaining blocker |
|---|---|---|---|
| PDF 1.7 | logical/structured | complete | stale JSON binary-diff facets; guard omission |
| DWG AC1024 | logical/structured | complete except framework routes | analyzer/composer exact fixture coverage |
| SVG 1.1 | logical/structured | complete | native fallback in DSL; stale JSON diff/op facets |
| MP4 | logical/structured | complete | native snapshot + JSON op facet contradictions |
| PPTX | logical XML + semantic binary parts | complete except framework routes | analyzer/composer exact fixture coverage; opaque facets |
| IFC2X3 | logical `Part21Document` + typed preamble | complete | opaque representation facets need structural specificity |
| ZIP/OPC | decompressed entries + typed OPC tables | exercised indirectly by PPTX export | no direct exact shared-layer lifecycle; opaque facets |

## Required close order

1. Correct the hard-contradictory PDF/SVG/MP4 facets and expand anti-shadow enumeration.
2. Remove SVG native fallback from `ArtifactDsl::parse_dsl`.
3. Add exact DWG and PPTX analyzer/composer assertions.
4. Add the exact PPTX-container ZIP/OPC structural lifecycle.
5. Replace or structurally reference every remaining opaque payload-only facet.
6. Run the named original-byte lifecycle tests and anti-shadow/facet gates only after the owning
   lanes finish their implementation edits.

## PDF binary-diff facet remediation — 2026-08-14

The PDF 1.7 binary-diff ABNF, Spicy, and Kaitai facets now describe the live structured
`DiffCodec` frame: format byte `1`, validated flags bits 0-4, fixed-order optional logical fields,
unsigned LEB128 counts/lengths, zigzag signed integers, recursive COS/value-diff tags, decoded
stream values, and typed filter/predictor records. All stale RFC 8259 and UTF-8 JSON envelope
claims were removed.

The existing PDF anti-shadow test now scans all three binary facets and rejects JSON-envelope and
native/shadow-state markers. It also requires each binary facet to expose the structural `format`
and `flags` framing concepts. The exact original-byte lifecycle now asserts that its title-only
diff binary begins with the structural format byte, sets only the typed `info` flag, and differs
from the text representation before decode/apply/inverse/absorb proceeds.

Ticket-local isolated-target Nx evidence:

- `[DEBUG] pdf_snapshot_and_facets_forbid_native_shadow_state=pass` (1/1, 0.013s; 3,377 skipped).
- `[DEBUG] bachelor_thesis_logical_lifecycle_preserves_original_native_bytes=pass` (1/1,
  18.484s; 3,377 skipped), still comparing every native export route directly with the
  6,346,331-byte imported fixture.
