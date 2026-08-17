# P2 FG4 Independent Verification Report

Verifier scope: FG4's 5 fan-out agents (docx, xlsx, pptx, bcf, ifc/2x3) — the last wave, completing
all 32 standards. Everything below was checked against disk and by re-running tests myself; no
self-report claim was taken on trust.

## Summary

All 5 standards pass independent verification. Real dialect headers, real binary frames (not the
F6 `print().into_bytes()` shortcut) for both `DiffCodec` and `OpBinary` on all 5, real OPC-family
container restatement (docx/xlsx/pptx/bcf all byte-identical in shape to zip's own real pilot file),
real Part-21 restatement for ifc/2x3 (byte-identical in shape to ifc/4's own real family), genuine
fixtures, 5-role `LanguageSpec` registration, all 6 conformance-law tests present and green per
standard, and ifc/2x3's `serde_json` elimination confirmed (zero live hits, only doc-comment
mentions of what was replaced). **No binary-frame shortfall recurrence found anywhere in FG4.**

| artifact | scoped tests | dialect | binary frame | fixtures | registration | pitfalls | json-ban |
|---|---|---|---|---|---|---|---|
| docx | 56/0 | real | real | real | real (5-role) | avoided | n/a (never violated) |
| xlsx | 49/0 | real | real | real | real (5-role) | avoided | n/a (never violated) |
| pptx | 58/0, 1 ignored (harmless ambient test) | real | real | real | real (5-role) | avoided | n/a (never violated) |
| bcf | 27/0 | real | real | real | real (5-role) | avoided | n/a (never violated) |
| ifc/2x3 | included in ifc's 82/0 | real | real | real | real (5-role) | avoided | confirmed eliminated |

## Per-standard detail

### docx (OPC pattern-setter)

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::docx"` → **56 passed, 0 failed** (self-report
  matches exactly, re-run fresh by me).
- `📸️snapshot/💾️binary/📡️component.protocol.semio`: real `dialect protocol` / `repeat entries {...}`
  / `backward eocd magic 0x504B0506 {...}` / `jump central_dir_start from cd_offset` /
  `repeat central_directory {...}` — read in full, byte-identical in structural shape to
  `🎒️zip/🏅️standards/🔖️2.0/…/📡️component.protocol.semio`'s own real file (diffed the block-level
  directives: identical `repeat`/`backward`/`jump`/`arm` lines).
- `📸️snapshot/📝️text/📖️component.grammar.semio`: real `dialect grammar` header (`comment none`,
  `string double raw`, `string single raw`), models `[Content_Types].xml`/`_rels/.rels`/
  `word/document.xml`/`word/styles.xml` with real element/attribute vocabulary traced from
  `⚙️engine/🦀️component.rs`. Fused-slash lexer discovery (`<w:b/>` self-closing tags) is real and
  documented inline, correctly modeled as one literal token.
- `🔺️diff/🦀️component.rs`: read `encode_diff`/`decode_diff` bodies directly (lines 2470-2491) — real
  `store::pack_rt::OP_BINARY_FORMAT` header byte + `flags` byte + field-by-field recursive binary
  sub-encoders via `store::ByteReader`. **Zero live `.into_bytes()` calls** — the only 2 matches in
  the whole file are doc comments describing what was replaced.
- `🧬️mutations/🦀️component.rs`: read `encode_op`/`decode_op` (lines 521-594) — same real frame shape
  (`format u8 | tag u8 | variant payload`), 13-tag match, `store::ByteReader`-based decode.
- 5-role `register_pilot_languages()` confirmed wired into `register()`, 5 `dsl::register_language`
  calls present.
- All 6 conformance-law tests present (`committed_facet_files_parse`, `grammar_conformance_law`,
  `ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
  `fixture_honesty_law`) and green in my own run.
- Fixtures real: `example.dsl.semio` starts `semio stdio.docx.dsl v1` followed by genuine hex OPC
  bytes; `example.pack.semio` exists (1678 bytes), not the old fake `"68656c6c6f"` placeholder.

### xlsx (OPC-family sibling)

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::xlsx"` → first attempt hit a **transient
  compile break** (`E0603`/private-item error inside a wholly different concurrent session's
  in-progress `🧿️semio` artifact work, confirmed via `git status` showing 169 modified files under
  `🗿️artifacts/🧿️semio/**`, none of which are xlsx's own files) — retried once per this ticket's own
  guidance, got a clean **49 passed, 0 failed** (self-report matches exactly).
- Protocol file: same real `repeat`/`backward`/`jump` OPC container shape as docx/zip, confirmed by
  direct read (block directives byte-identical in structure).
- `🔺️diff/🦀️component.rs` and `🧬️mutations/🦀️component.rs`: read both `encode_diff`/`decode_diff`
  and `encode_op`/`decode_op` — real `store::pack_rt::write_varint_u64`/`store::ByteReader`-based
  frames throughout, `.into_bytes()` appears only in doc comments describing the pre-upgrade
  shortcut, never in live code.
- 5-role registration confirmed (5 `dsl::register_language` calls, wired into `register()`).
- All 6 conformance-law tests present.
- Fixtures real (`example.dsl.semio` starts `semio stdio.xlsx.dsl v1` + genuine hex; pack file 2250
  bytes).
- The self-report's own "part-order fixed point" bug (a real bug found and fixed by the agent's own
  `fixture_honesty_law`, not a shortcut) is corroborated by the fixture-file content and the passing
  test — no reason to doubt it.

### pptx (OPC-family sibling)

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::pptx"` → **58 passed, 0 failed, 1 ignored**
  (self-report matches exactly; the 1 ignored test is the ambient `zzz_generate_p2p1_fixtures`
  concurrent-session artifact the report documents finding and correctly leaving untouched).
- Protocol file: same real OPC container shape, confirmed.
- `🔺️diff/🦀️component.rs` / `🧬️mutations/🦀️component.rs`: real binary frames confirmed by direct
  read of `encode_diff`/`decode_diff`/`encode_op`/`decode_op` — `store::pack_rt`/`store::ByteReader`
  throughout, no live `.into_bytes()`.
- 5-role registration confirmed.
- All 6 conformance-law tests present.
- Fixtures real (`example.dsl.semio` starts `semio stdio.pptx.dsl v1` + genuine hex; pack file 5199
  bytes).
- The documented `x-elem-fused-empty-tag-ambiguity` gap is a genuine, honestly-scoped-out structural
  limitation (not a corner cut on this wave's own deliverables) — the typed productions that
  actually matter for this wave's fixtures all correctly model the fused-slash case per-literal.

### bcf (deliberately non-OPC container-family member)

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::bcf"` → **27 passed, 0 failed** (self-report
  matches exactly).
- Correctly deviates from the ticket's literal "OPC container" framing for the GRAMMAR facet only
  (bcf has no `[Content_Types].xml`/`_rels/.rels` — confirmed by 3 independent doc-comment citations
  in the real Rust codec, plus this wave's own report explaining why) while still restating the real
  OPC-family **container** shape (zip's own real `repeat`/`backward`/`jump` layout) for the binary
  protocol facet, since bcf's container is byte-identical real ZIP 2.0 one layer below OPC. Verified
  this deviation is real, not an excuse: `⚙️engine/🦀️component.rs` genuinely calls
  `zip::engine::{encode_zip,decode_zip}` directly, never `zip::opc::*`.
- `🔺️diff/🦀️component.rs` / `🧬️mutations/🦀️component.rs`: real binary frames confirmed by direct
  read — `store::pack_rt`/`store::ByteReader` throughout, no live `.into_bytes()`.
- The `comment` reserved-keyword collision (pitfall #3) was caught and fixed pre-commit — confirmed
  the grammar files use `bcf-comment`, not the bare reserved word.
- 5-role registration confirmed.
- All 6 conformance-law tests present.
- Fixtures real (`example.dsl.semio` starts `semio stdio.bcf.dsl v1` + genuine hex; pack file 2458
  bytes).

### ifc/2x3 (the JSON-transfer-ban closer)

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::ifc"` → **82 passed, 0 failed** (covers both
  v4 and v2x3; self-report's own number matches exactly).
- **`serde_json` elimination — independently re-grepped**: `grep -rn serde_json` across
  `🏗️ifc/🏅️standards/🔖️2x3/**` → **zero live calls**, only 4 doc-comment mentions of what was
  replaced (in grammar-file prose and Rust doc comments). Confirms the program's own "last remaining
  `POLICY_STDIO_JSON_TRANSFER_BAN` violation" is genuinely closed.
- Grammar file (`📸️snapshot/📝️text/📖️component.grammar.semio`): real ISO 10303-21 Part-21 header
  (`comment line none`, `comment block "/*" "*/"`, `string single doubled`) — diffed directly against
  `ifc/4`'s own real grammar file body: **structurally identical** (only `grammar`/`envelope-mark`
  ids and doc-comment wording differ; every production line is byte-identical).
- `encode_diff`/`decode_diff` (read directly, lines 479-509+): real field-by-field binary frame —
  notably **not even an opaque-tail shortcut**, since `Part21Value` is fully spec-expressible
  per-variant and every field bottoms out through real recursive binary sub-encoders
  (`enc_part21_value_bin`/`enc_part21_header_bin`/`enc_instance_list_bin`), all via
  `store::pack_rt`/`store::ByteReader`.
- `encode_op`/`decode_op` (read directly, lines 178-215+): same real frame shape
  (`format u8 | tag u8 | variant payload`), 5-tag match, real recursive sub-encoders throughout.
- This was the ONE standard with **no prior `DiffCodec` impl at all** — confirmed the impl now
  exists and is genuinely real, not merely present.
- 5-role registration confirmed (own standard-local fixtures under `🏅️standards/🔖️2x3/📚️examples/`,
  correctly not touching the shared `ifc/📚️examples/` UI-facing demo entry `4` owns).
- All 6 conformance-law tests present, plus the 2 new round-trip law tests
  (`diff_codec_text_binary_roundtrip_law`, `op_text_binary_roundtrip_law`).
- Fixtures real: `example.dsl.semio` starts `semio stdio.ifc.2x3.dsl v1` followed by genuine
  `ISO-10303-21;` exchange-file text (`FILE_SCHEMA(('IFC2X3'))`, satisfying `decode_ifc2x3`'s own
  gate); `example.pack.semio` exists (300 bytes).
- One legitimate, correctly-flagged loose end: the agent's own report notes
  `POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST` in `📜️script.ts` still contains a now-stale entry for
  this standard (harmless "low priority" self-flagged breach per the policy script's own logic) —
  correctly left for the ticket's periodic policy-shrink pass since editing `📜️script.ts` is out of
  this agent's (and my) ownership boundary.

## Ownership boundary

Confirmed via `git status` per-artifact scoping that each FG4 agent's diff stays inside its own
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<artifact>/**` tree (ifc: `🔖️2x3/**` only, `🔖️4/**` untouched).
`📦️glue.rs`, `📜️script.ts`, SDK traits, schema/dsl/protocol/registry modules, and `🏪️store` were not
touched by any of the 5 reports, and I did not touch them either during verification.

## Full crate test — blocked by unrelated, confirmed-external churn (not a FG4 regression)

`cargo test -p semio-s-plugin-stdio --lib` (whole crate) was attempted 4 times during this
verification session; every attempt failed to COMPILE (not a test failure) with the same stable
error set:

```
error: couldn't read `.../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🎹️composer/../../../../../📚️examples/🌐️graph/🖼️assets/🗣️example.dsl.semio`: No such file or directory
error: couldn't read `.../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️object/🎹️composer/../../../../../📚️examples/🌐️graph/🖼️assets/🎒️example.pack.semio`: No such file or directory
error[E0425]: cannot find value `enc_face` / `dec_face` / function `enc_face` / `dec_face` in this scope
  --> .../🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️cad/🧬️schema/🔺️diff/🦀️component.rs
```

`git status --porcelain -- "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/"` shows **169 modified files**
under that tree right now — a large, actively-changing concurrent session working on the `🧿️semio`
artifact (a different artifact family entirely, not one of stdio's 32 format standards, and not any
of FG4's 5 artifacts). This matches the ticket's own documented "large concurrent session actively
adding new artifact types under `🧿️semio/**`" ambient-churn warning exactly. Per the task's own
"classify via file path, don't chase, retry once" instruction (retried 3 times, not just once, given
the importance of a full-crate number) — the error is stable across retries (same file paths, same
symbol names each time), consistent with an in-progress multi-file edit rather than transient
build-lock noise. **This is not attributable to any FG4 artifact** — none of the failing files are
under docx/xlsx/pptx/bcf/ifc, and every one of FG4's own 5 scoped test suites compiles and passes
cleanly in isolation (5/5, 272 total individual tests across the 5 scoped runs, 0 failures). I could
not obtain a clean whole-crate number this session; the reliable signal is the 5 clean scoped runs
above, each independently re-run by me, not taken from any self-report.

## Verdict

**No binary-frame shortfall recurrence.** All 5 FG4 standards land real dialect files, real binary
frames for both diff and mutation facets, real fixtures, real 5-role registration, and (for ifc/2x3)
a confirmed-clean JSON-transfer-ban closure. The self-reports' claims all independently verified
against disk and fresh test runs — no orchestrator fix wave is needed for FG4.
