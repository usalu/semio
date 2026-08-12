# P2-G2 — Final Gate Report (Phase 2: Real-Format Grammars & Protocols)

Independent, from-disk re-verification of Phase 2's own definition of done, run fresh in this
session. Every command below was executed directly by this gate, not re-trusted from prior waves'
self-reports — though every one of them cross-checked consistently against this session's own fresh
output. Full history already independently verified wave-by-wave lives in
`~/.claude/plans/the-current-schemas-are-scalable-journal.md`'s "Phase 2 execution log" (M1 through
PW) and is not re-derived here. This report's own STATUS.md append (PW section + G2 section + the
32-standard ledger) is the authoritative summary; this file is the supporting evidence trail.

## Verdict: GO, with caveats

Phase 2's scope — real grammar/protocol files, binary-frame-upgraded diff/op codecs, real fixtures,
5-role `LanguageSpec` registration, and zero JSON-transfer violations across all 32 official stdio
standards — is genuinely complete. The open items (below) are all pre-existing, documented, and
outside this wave's ownership boundary to fix; none of them is a Phase 2 content shortfall.

## 1. Parseability — PASS

`bun run ./📜️script.ts policy` (full log: `/private/tmp/…/scratchpad/g2-policy.txt`) shows **zero
high-priority breaches** under any of the 5 Phase 2 policy rule names. The bare CLI's default output
reports a large `os-state-authority` breach count (21,655) — confirmed by name-grep to be entirely
unrelated repo-wide rules (OnceLock/HashMap-outside-OS-product findings across the whole codebase,
pre-existing, out of this program's scope) — zero matches for `grammar-parseability`,
`protocol-parseability`, `fixture-honesty`, `language-registration`, or `json-transfer-ban`.

Measured all 5 allowlists' `Set` literal sizes directly from `📜️script.ts`:

| rule | count |
|---|---|
| `POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST` | 60 |
| `POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST` | 60 |
| `POLICY_FIXTURE_HONESTY_ALLOWLIST` | 9 |
| `POLICY_LANGUAGE_REGISTRATION_ALLOWLIST` | 8 |
| `POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST` | 22 |

Identical to PW's own drained counts (independently re-verified by PW's own verify pass too) — zero
regrowth since PW landed.

## 2. Conformance laws — PASS

`cargo test -p semio-framework-os-kernel` (full log: `g2-framework-test.txt`):
`os_dsl::fixture_sweep::m5_handcrafted_protocol_conformance::
all_discovered_snapshot_protocols_walk_their_shipped_fixtures` — **green**.
`m5_handcrafted_grammar_conformance` and `m5_production_coverage` fail on the exact same
pre-existing, non-stdio baseline every prior wave has documented: 3 hard failures
(`🏗️fem::◻2d::🔖️1`, `📕️norm::📘️en1992::🔖️1`, `🕸️dag::🕸️dag::🔖️1`) feeding the 2 failing test
*functions* — zero stdio standard in either failure set.

Confirmed the full 6-law suite (`committed_facet_files_parse`, `grammar_conformance_law`,
`ops_grammar_conformance_law`, `diff_grammar_conformance_law`, `protocol_walk_law`,
`fixture_honesty_law`) is present for all 32 standards by direct grep across each standard's
`⚙️engine`/`🧬️mutations`/`🔺️diff` component files (some standards — txt, csv confirmed directly —
split the 6 across those 3 files rather than concentrating them in `⚙️engine` alone; a naive
single-file grep undercounts, verified by re-checking with `grep -rn` across the full standard
directory).

## 3. m5 auto-sweep enrollment — PARTIAL (honest, matches every prior wave's own accounting)

`STDIO_CONFORMANCE_GRADUATED` (read directly, `🧪️fixture-sweep/🦀️component.rs:872-1078`):

- **27 standards**: `Grammar`+`ProtocolPack` graduated (6 pilot + 7 FG1 + 9 FG2 + 5 FG3).
- **4 standards** (`docx`, `xlsx`, `pptx`, `bcf`): `ProtocolPack`-only graduated. `Grammar` stays
  ungraduated — a harness-assumption gap, not a content gap: `check_grammar_recognizes` feeds the
  artifact's whole top-level `.dsl.semio` fixture (a hex-dump of the OPC binary) to the Recognizer,
  but these 4 standards' snapshot TEXT grammar correctly models the individual XML/text PARTS a real
  package contains, never the whole outer binary package — confirmed by the doc-comment's own
  citation of each standard's `grammar_conformance_law`, which decodes the real zip container and
  recognizes each part separately (a stronger, not weaker, conformance proof).
- **1 standard** (`ifc/2x3`): ungraduated — the same `pilot_resolve` single-fixture-slot-per-artifact
  gap `gif/89a` and `pdf/1.7` hit before PW's fix (ifc/4 and ifc/2x3 share one artifact-level fixture
  slot). PW verified staging the fix resolves it too, but correctly left it out — not named in PW's
  own brief, and ifc carries this program's own documented copy-paste-defect history (W0 census)
  warranting a dedicated look rather than a silent addition.

All 32/32 standards have real, tested, in-artifact conformance laws (confirmed in §2) regardless of
harness graduation status — every un-graduated facet here is a harness limitation, independently
diagnosed and documented across 3 separate waves (FG2, FG3, PW), not undone standard-content work.

The 6 non-stdio pilots (lowpoly/dag/cad/en1992/note/fem2d) show the identical, unchanged
3-hard-failure baseline (dag/fem2d/en1992) confirmed in §2 — no new regression against the W0
baseline this program inherited.

## 4. 5-role registration — PASS

Grep-counted `register_language`/`LanguageSpec` occurrences per standard's own registration file
(direct count, not presence-only) for all 32 official standards (excluding the 2 non-census
stdio standards `epw`/`tsv`, which showed only 1 each — pre-existing, out of this program's scope,
correctly untouched):

**All 32/32 show exactly 5.** Zero below, zero above. `jpg`'s FG2-fix registration confirmed still
holding at 5 (was 0 before that targeted fix).

`register_schema_spec` present for 29/32 standards (every one whose snapshot/diff type genuinely
derives `dsl::DslRecord`/`DslArtifact`/`DslDiff`); absent for exactly 3 (`csv`, `dxf`, `gltf`) —
confirmed by direct read of each standard's own snapshot struct: all 3 derive only
`Serialize`/`Deserialize` (`csv` additionally derives `ArtifactSchema`, not a `dsl::` derive) — no
`RecordSpec` genuinely exists, matching the recipe's own documented `json`/`csv`/`zip`/`png`
hand-rolled exception class (json/zip/png each also lack it, but DO carry an explicit "deliberately
NOT called" doc comment `csv`/`dxf`/`gltf` don't — see caveats).

## 5. JSON-transfer ban — PASS

Independently re-grepped `serde_json::to_vec\|from_slice\|to_string\|from_str` across
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/**/*.rs` directly (not via the policy tool). Every hit outside
the 32 standards' own `ArtifactPack`/`OpBinary`/`DiffCodec` transfer paths:

- `gltf/2.0`'s own engine (4 hits): native JSON document parse/print — glTF's OWN wire format IS
  JSON, explicitly carved out as legitimate.
- `gltf`/`gif` `📚️examples/*/component.rs` (2 hits): example-gallery debug-print helpers
  (`ExampleSource::new`), not inside any transfer-trait impl.
- `gltf`'s io-bridge deserializer doc comment (1 hit), `ifc/2x3`, `svg`, `xml` doc comments (3 hits):
  quoting already-replaced OLD code for documentation, confirmed by direct read, not live calls.
- 4 non-census artifacts (`avi`, `mp3`, `mp4`, `wav`) + the concurrently-churning `🧿️semio`: neither
  is one of this program's 32 standards.

Zero hits land inside any of the 32 official standards' actual transfer implementation.

Spot-checked binary-frame reality (real field-by-field writer, not the F6
`print_diff().into_bytes()` text-as-binary shortcut) across a representative sample spanning every
fan-out wave: `obj`/FG1, `gif/89a`/FG2, `ply`/FG3, `pdf/1.7`/FG3, `docx`/FG4. Every sampled
`encode_diff` produces bytes via a genuine `BinWriter` accumulator (`w.into_bytes()` finalizes a
buffer that was built field-by-field, a categorically different thing from calling `.into_bytes()`
on a `print_diff()` text string) — zero shortcut residue in the sample.

## 6. Gates

- **`cargo test -p semio-s-plugin-stdio --lib`** (`g2-stdio-test.txt`): **1922 passed, 1 failed, 3
  ignored**. The 1 failure —
  `artifacts::semio::standards::v1::subsets::video::composer::tests::conformance_laws::
  fixture_honesty_law` — panics on a fixture body containing the literal string
  `"PLACEHOLDER-REGENERATE-VIA-TEMP-TEST"`, unambiguous direct proof of the same unrelated
  concurrent `🧿️semio` session churn every wave from FG1 onward has independently hit and
  documented (that artifact type is never touched by this program's 32 standards). Filtered to
  every test NOT under `artifacts::semio::`: **0 failures**, 1409 tests green — all 32 official
  standards' own tests are 100% clean.
- **`cargo test -p semio-framework-os-kernel`** (`g2-framework-test.txt`): **796 passed, 2 failed**
  — exact match to the long-documented pre-existing baseline (§2).
- **`cargo check -p semio-s-plugin-trinity`** (`g2-trinity-check.txt`): clean — warnings only
  (unused-import, dead-code, lifetime-elision style lints), zero errors.
- **`cargo check --workspace`** (`g2-workspace-check.txt`, retried once 5+ minutes later in
  `g2-workspace-check-retry.txt`): **NOT clean** — 81 errors, byte-for-byte identical error set both
  runs (`diff` confirms, not flaky mid-save noise). Traced every single error location: 100% are
  `E0432 unresolved import`/`E0433 cannot find crate` inside `semio-framework-os-kernel-db`
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/📦️glue.rs` — confirmed
  `git status`-modified right now, i.e. mid-edit by a live concurrent session) and its downstream
  dependent `semio-compose-rs` (`compose/client/lib/rs/lib.rs`). The compiler's own suggested fix
  (`pub use crate::db_engine::{...}` vs. the broken `pub use db_engine::{...}`) is the signature of
  a live module-path refactor mid-flight, not a Phase 2 defect. **Zero of the 81 errors reference
  any of the 32 stdio standards, the `dsl` grammar/protocol framework modules Phase 2 depends on
  (`semio-framework-os-kernel` itself), or the `trinity` crate** — confirmed by filtering every
  error's file path and finding no `🗄️stdio`/`trinity` hits, and independently corroborated by the
  fact that `semio-framework-os-kernel` and `semio-s-plugin-stdio` both compile and fully test clean
  in isolation, which would be structurally impossible if Phase 2's own code were the cause.
  Classified as confirmed unrelated concurrent churn in the `🛢️db` module — entirely outside
  stdio/Phase 2's ownership boundary and this wave's write permissions — per the ticket's own
  standing "classify, don't chase" rule for concurrent churn. Reported honestly as a currently
  non-clean gate rather than papered over; re-run once that other session's `🛢️db` refactor lands.

## 7. STATUS.md ledger — DONE

Appended a PW section (summarizing PW's own work, which had not yet been added to STATUS.md — the
FG4 closer's own last line explicitly said "Ticket left open for the orchestrator's own final PW/G2
gate summary") and a G2 section with the full 32-standard ledger table (grammar+protocol status,
binary-frame status, 5-role registration, schema-spec status, m5 graduation status per standard) to
`STATUS.md`. Existing content untouched — pure append.

## Caveats (none block the GO verdict; all pre-existing, documented, out of this wave's ownership)

1. `cargo check --workspace` is currently not clean, blocked by an unrelated in-progress `🛢️db`
   module refactor (§6) — re-run once that lands; zero Phase 2 content is implicated.
2. `docx`/`xlsx`/`pptx`/`bcf`'s `Grammar` facet and `ifc/2x3`'s both facets remain harness-ungraduated
   in `STDIO_CONFORMANCE_GRADUATED` for real, narrow, already-diagnosed reasons (§3) — not a content
   shortfall, a harness limitation (container-vs-part-blind grammar check; one more `pilot_resolve`
   shared-slot instance intentionally deferred).
3. `csv`/`dxf`/`gltf`'s `register_schema_spec` absence is technically sound (verified: no derivable
   `RecordSpec` exists) but undocumented in-file, unlike `json`/`zip`/`png`'s explicit "deliberately
   NOT called" comment for the same situation — a one-line doc-comment gap, not a functional one.
4. The `🧿️semio` artifact type (a NEW artifact type this program never touches, being actively
   built by a separate concurrent session) continues to cause intermittent, self-evidently
   unrelated test flakiness in the shared `semio-s-plugin-stdio` crate — confirmed again in this
   session's own fresh run, consistent with every prior wave's independent confirmation.

## Logs (this session, in scratchpad — not the ticket folder, pure read-only verification output)

`/private/tmp/claude-501/-Users-ueli-Documents-semio/68820b15-0105-4e16-84cc-2828034f1df2/scratchpad/`:
`g2-policy.txt`, `g2-stdio-test.txt`, `g2-framework-test.txt`, `g2-workspace-check.txt`,
`g2-workspace-check-retry.txt`, `g2-trinity-check.txt`.

## Ticket lifecycle

No `ticket_open`/`ticket_close`/`ticket_reopen` called by this gate, per explicit instruction — the
orchestrating session closes this ticket itself.
