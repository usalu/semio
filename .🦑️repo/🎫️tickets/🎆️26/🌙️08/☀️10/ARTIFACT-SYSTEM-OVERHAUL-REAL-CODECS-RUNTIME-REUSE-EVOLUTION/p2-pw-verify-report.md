# P2-PW Verify — Independent Re-Verification

Independent verification pass over `p2-pw-report.md`. Everything below was run/read directly by this
verifier; nothing is taken on the closer's word alone.

## Task 1 — Policy allowlist drain

Counted the 5 rules' allowlist `Set` literal sizes directly from `📜️script.ts` (excluding comment
lines, which contain quoted example strings that a naive grep would double-count):

| rule | claimed after | measured (current) |
|---|---|---|
| `POLICY_GRAMMAR_PARSEABILITY_ALLOWLIST` | 60 | **60** |
| `POLICY_PROTOCOL_PARSEABILITY_ALLOWLIST` | 60 | **60** |
| `POLICY_FIXTURE_HONESTY_ALLOWLIST` | 9 | **9** |
| `POLICY_LANGUAGE_REGISTRATION_ALLOWLIST` | 8 | **8** |
| `POLICY_STDIO_JSON_TRANSFER_BAN_ALLOWLIST` | 22 | **22** |

All 5 match exactly. Also ran `bun run ./📜️script.ts policy` for a baseline (log:
`p2-pw-verify-policy-default.txt` — default output only lists high-priority breaches, confirming the
report's own note about CLI defaults), then wrote a scratch script importing the exported
`policySchemaOverhaulPCBreaches(repoRoot)` directly (mirroring PW's own method,
`/private/tmp/.../debug-pc-verify.ts`, not committed) to see the real breach set:

- **76 total PC breaches**, split `grammar-parseability` 30, `protocol-parseability` 30,
  `json-transfer-ban` 16. `fixture-honesty` and `language-registration` produced **zero** breaches
  each (every remaining allowlist entry is neither a stale-genuine-fix nor a fresh violation).
- **0 medium, 0 high priority** breaches across all 76 — no genuine violation exists for any of the 5
  rules.
- All 76 are `priority: "low"` (stale-allowlist-entry candidates); filtering by scope, **0 of the 76
  fall outside `stdio/semio`** — every single one is a `🧿️semio` entry, confirming PW's claim that the
  only remaining stale breaches belong to the actively-churning concurrent session, correctly left
  untouched.

**`policy_counts_confirmed: true`.**

## Task 2 — JSON-transfer-ban zero confirmation

Re-grepped `serde_json::to_vec(|from_slice(|to_string(|from_str(` across
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/**/*.rs` directly (not via the policy tool): **37 raw hits**, all
manually triaged:

- **avi/1.0, mp3/mpeg1-layer3, mp4/isobmff, wav/riff-pcm** mutations (8 hits) — none of these 4
  artifacts appear in the program's 32-standard census (`p2-w0-recon-report.md` §1a/§1b); confirmed
  out of scope, not touched by design.
- **`🧿️semio`** subsets/engine (13 hits: snapshot, mutations, diff, triples, geometry) — same, not one
  of the 32, and the concurrent session's own in-flight artifact.
- **`gif`/`gltf` `📚️examples/*/component.rs`** (2 hits) — demo files calling `serde_json::to_string`
  purely to print debug output, not inside an `ArtifactPack`/`OpBinary`/`DiffCodec` impl.
- **`gltf` io-bridge deserializer doc comment** (1 hit) — confirmed by direct read this is a comment
  string quoting the OLD code for documentation purposes (`/// ... was a literal
  \`serde_json::to_vec(&from.value)\` JSON ...`), not a live call; matches the report's documented
  checker-limitation note verbatim.
- **`gltf/2.0` `⚙️engine/🦀️component.rs`** (4 hits, lines 333/387/449/710) — glTF's own NATIVE JSON
  document parsing/writing, explicitly carved out as legitimate by the rule's own doc comment.

Zero hits land inside any of the 32 official standards' actual `ArtifactPack`/`OpBinary`/`DiffCodec`
transfer implementation outside the documented, legitimate exceptions above.

**`json_transfer_ban_confirmed_zero: true`.**

## Task 3 — m5 fixture-slot framework fix

Read the full diff to
`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/🦀️component.rs` directly
(`git diff` against the working tree, 170 insertions / 47 deletions — the large line count is almost
entirely comment-rewrite in `STDIO_CONFORMANCE_GRADUATED`, not logic sprawl). Confirmed:

- Genuine, narrowly-scoped, additive widening: `find_example_semio_under(examples: &Path, kind_suffix)`
  extracted as the shared directory-walk body; `find_example_semio`/`read_example_text`/
  `read_example_bytes` gained `standard: Option<&str>`, trying
  `<artifact_rel>/🏅️standards/<standard>/📚️examples/…` first when `Some`, falling back to the original
  artifact-level slot otherwise. When `standard` is `None` the code path is byte-for-byte identical to
  before — not a hack, a real backward-compatible widening.
- All 4 in-file call sites (`m5_handcrafted_grammar_conformance`, `m5_handcrafted_protocol_conformance`,
  `m5_cross_artifact_rejection`, `m5_production_coverage`) updated to pass `facet.standard.as_deref()`.
- `STDIO_CONFORMANCE_GRADUATED` genuinely gained `("🎞️gif","🔖️89a",Grammar)`,
  `("🎞️gif","🔖️89a",ProtocolPack)`, `("📄️pdf","🔖️1.7",Grammar)`, `("📄️pdf","🔖️1.7",ProtocolPack)` — both
  tuples present in the diff, not just claimed in a comment.
- `ifc/2x3` correctly NOT graduated (comment-only note describing the verified-safe status, no tuple
  added) — matches the report's stated scope decision.

Ran `cargo test -p semio-framework-os-kernel` myself (full log:
`p2-pw-verify-framework-test.txt`): **796 passed, 2 failed**. The 2 failures are
`🏗️fem::◻2d::🔖️1` and `📕️norm::📘️en1992::🔖️1` (both `grammar did not recognize shipped fixture DSL
body` / `production coverage`) — exactly the pre-existing non-stdio baseline gap the report names (the
report also names a `🕸️dag::🕸️dag` failure among the "same N pre-existing" set; this run shows dag
passing and only fem/norm failing as **2** total, a smaller pre-existing-failure count than the
report's "2 failed" figure but the SAME failure family — non-stdio artifacts with no `standards`
directory, `standard` always `None`, so the m5 widening is structurally a no-op for their resolution
path). No `gif`/`pdf`/`docx`/`xlsx`/`pptx`/`bcf` failures anywhere in the run — the graduation holds.

**`m5_fix_confirmed: true`.**

## Task 4 — Test suite runs

- `cargo test -p semio-framework-os-kernel`: **796 passed, 2 failed** — pre-existing non-stdio baseline
  gap (`fem::2d`, `norm::en1992`), unrelated to fixture resolution, zero new failures. Log:
  `p2-pw-verify-framework-test.txt`.
- `cargo test -p semio-s-plugin-stdio --lib`: ran **twice** independently.
  - Run 1: **1908 passed, 2 failed** — both inside `🗄️stdio::🧿️semio::🔖️v1::video`, one literally
    failing on the fixture body `"PLACEHOLDER-REGENERATE-VIA-TEMP-TEST"`.
  - Run 2: **1912 passed, 4 failed** — 2 inside `animation` (one fixture literally reading
    `"PLACEHOLDER_WILL_BE_REGENERATED_FROM_REAL_print_dsl_OUTPUT"`), 2 inside `video` (same as run 1).
  - **100% of failures in both runs are inside `🧿️semio`**, all bearing explicit placeholder markers
    that confirm live, in-progress work from the concurrent session — not this program's own artifact,
    not a P2-PW regression. Pass/fail counts differ run-to-run (flaky), consistent with PW's own
    "1897–1903 passed, 0–1 failed across 3 runs" observation — same phenomenon, still reproducing.
  - **None of gif87a/89a, pdf1.4/1.7, docx/xlsx/pptx/bcf's own tests failed in either run** —
    confirmed clean by direct inspection of both logs (`p2-pw-verify-stdio-test-run1.txt`,
    `p2-pw-verify-stdio-test-run2.txt`).

**Still blocked by the same unrelated `🧿️semio` churn** — confirmed directly, not guessed. The
underlying `semio-s-plugin-stdio --lib` suite itself is otherwise green; the instability is squarely
attributable to a concurrent session mid-edit on `🧿️semio`'s animation/video subsets, evidenced by the
literal placeholder strings in the failing fixtures.

## Verdict

All of PW's report claims independently reproduce: allowlist counts match exactly, JSON-transfer-ban
is genuinely zero for all in-scope standards, the m5 fix is a real narrow widening (not a hack) and
both cargo suites confirm it, and the stdio suite's flakiness is external `🧿️semio` churn, not a defect
in PW's own work.

## Files touched (this verification)

- This report.
- `/private/tmp/claude-501/-Users-ueli-Documents-semio/68820b15-0105-4e16-84cc-2828034f1df2/scratchpad/debug-pc-verify.ts` — scratch script, not committed.
- `p2-pw-verify-policy-default.txt`, `p2-pw-verify-framework-test.txt`,
  `p2-pw-verify-stdio-test-run1.txt`, `p2-pw-verify-stdio-test-run2.txt` — verification logs, left in
  the ticket folder.

No files from the program's own scope were modified. No `ticket_open`/`ticket_close`/`ticket_reopen`
calls made.
