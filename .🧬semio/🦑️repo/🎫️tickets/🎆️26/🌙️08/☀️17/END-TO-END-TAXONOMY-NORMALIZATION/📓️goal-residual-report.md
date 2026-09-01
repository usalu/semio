# Residual Rows — Slice Report (wgpu generator / prompts / stdio Rust singletons)

Baseline: `bb06c41f73f0122fbed315b7487428b976f99921`. Scope for all measurements: `🧰️framework/🔨️modules/🖼️assets`.

Real 16-row baseline (this session, `🗑️temp/🔣️residual-plan-before.json`):

```
[clean taxonomy plan] moves=1089 roots=0 relocations=0 symlinks=0 removals=1 edits=40 regenerations=2 unresolved=16
```

My slice: 8 of the 16 rows (prompts 2, dwg singleton 1, obj singleton 1, wgpu bundle 4). The other 8
(1 python fixture, 5 `.feature` files, 1 `.feature` inside a scope I don't own) belong to the sibling's
Gherkin family and were left untouched.

## (c) `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md` — 2 rows — RESOLVED (schema-driven, detection intact)

New `historicalDocumentEvidencePopulations["dev-prompt-log"]` entry (`🔣️taxonomy.json`), directory
`**/.🧬semio/🦑️repo/💬️prompts/**`, leaf `^.+\.md$`. Wired through the existing, unmodified
`historicalDocumentEvidence()` choke point in `🧹️normalization/🟦️.ts` — no new predicate, no
special-casing of this ticket's own scope.

Both existing negatives generalized, not bypassed: `ticketPackageBoundaryOwns` renamed to
`historicalEvidenceBoundaryOwns` and given a second anchor pattern
(`HISTORICAL_PROMPT_LOG_ROOT_PATTERN`) alongside the ticket one, so a hypothetical package manifest
dropped directly under `💬️prompts/` still disqualifies its siblings from the exemption, exactly like
a ticket-embedded Cargo crate does today. `fixedFilenameContracts` matches are still refused
unconditionally (that check is population-agnostic already). Proved with a real, git-free fixture
(`🧪️historical-document-evidence/🟦️.ts`, new test "dev-prompt-log honors both existing negatives").

`HISTORICAL_DOCUMENT_EVIDENCE_POPULATION_IDS` in `🔍️discovery/🟦️component.ts` grew from 3 to 4
(exact-order validator, exact-shape validator branch added), and the `referenceClosure.
historicalDocumentEvidence` contract label was renamed to include the new population — both the
schema value and the TS literal type updated together.

**Fail-before/pass-after, both directions, verified for real**: deleted the population from the live
`🔣️taxonomy.json` on disk (not a mock), ran the real test suite — all 7 tests in
`🧪️historical-document-evidence` failed with `Invalid taxonomy schema`; restored the file, all 7
passed again. `discovery.validateTaxonomy()` returns 0 problems on the current schema.

Regression: `🧪️historical-document-evidence` 7/7, `🧪️preflight-reference-basis` 30/30 (updated its
sandboxed-extraction symbol list for the `ticketPackageBoundaryOwns` rename),
`📦️packages/🟦️typescript/🧪️index.test.ts`'s one relevant case (stub-injected, unaffected by the
rename) 1/1.

## (d)(1) `stdio` crate-root `📦️glue.rs` blocking the dwg singleton — 1 row — RESOLVED (rewritability, not detection)

Guard `rustFiniteManifestTargets`'s ancestor trust-scan (`🧹️normalization/🟦️.ts`) distrusts an
ancestor file the instant its comment/known-construct-stripped text contains any `#`/`!`. `stdio`'s
`glue.rs` fails it for three, unrelated reasons — all now read and proven individually, per the
method that has worked for every guard in this class so far:

1. **`macro_rules! impl_serde_op_codec { … }`** (line 38) — a crate-local macro *definition*, not an
   invocation of an external one (the existing `RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS`
   mechanism only strips call-site text). Read the full body: two trait impls
   (`protocol::OpText`/`protocol::OpBinary`) built from `serde_json::to_string`/`to_vec`/`from_str`/
   `from_slice` and `.expect`/`.map_err` — zero `mod` tokens anywhere in the expansion template.
   New registry `RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_DEFINITIONS` (bare macro names), scrubbing
   *both* the `macro_rules ! name` definition head and any local `name !` invocation, since (unlike
   an external macro) both live in the same ancestor file being trusted.
2. **`format!`** (line 111, `semantic_fingerprint`'s error path) and **`unreachable!`** (line 101,
   `hash_hex_bytes`'s exhaustiveness arm) — ordinary Rust standard-library macros. New registry
   `RUST_MODULE_STRUCTURE_TRANSPARENT_STD_EXPRESSION_MACROS`: the language itself guarantees these
   are expression-position only and can never emit an item-position `mod`, so trusting them is a
   *language* claim, not a per-macro body-reading exercise like the other two registries — kept
   deliberately narrow (exactly the two macros actually present, not "all of std").

⚠️ Confirmed the trap named in the brief: `mod tests { … }` at a much later line is the *macro
crate's own* test module, structurally outside `impl_serde_op_codec!`'s definition — a naive
`grep -c mod` on the file would have produced a false positive; the fix only strips the macro's
*declared* head/invocations, never touches unrelated `mod` items.

**Fail-before/pass-after, both directions, verified for real**: added case
`ancestor-crate-local-macro-and-std-expression-macros` to `🧪️rust-finite-target-consumption`
(fixture ancestor text: a local `macro_rules!` definition + one invocation + `format!`/`unreachable!`
in a plain fn). Temporarily reverted just the two new stripping loops in
`rustCodeOnlyTextForMacroTrust` — both compiler oracles (Bun, TypeScript) failed the new case with
`physicalInterpretation: undefined` instead of `"rust-finite-manifest-targets"`; restored, both pass.
Full suite 86/86 (was 84 before the new case).

This is a rewritability fix, not a detection weakening: the guard's condition
(`/[#!]/u.test(...) || /\bmacro\b/u.test(...)`) is untouched; only the *scrubbed input text* it reads
grew two new, individually-justified carve-outs, exactly the pattern already used for
`plugin_exports!`/`cfg`/`allow`/`derive`/`async_test`.

## (d)(2) `mutate-obj-3-0/🦀️.rs:109` — 1 row — DIAGNOSED, NOT IMPLEMENTED (different shape, said plainly)

`unsupported-path-syntax` here is not a `rustFiniteManifestTargets` ancestor-trust question at all —
different mechanism from (d)(1). The token sits inside a plain `const ORIGINAL_UNKNOWN_STATEMENTS:
[&str; 7]` array element (line 109: `"# source: shared-glb 🧰️framework/🔨️modules/🖼️assets/
🖼️images/🧊️pattern-sphere.glb"`), a byte-for-byte copy of the real committed
`🧫️fixtures/🧊️pattern-sphere.obj`'s own leading comment line (verified identical, `head -2` on both).
This is the generic, adapter-shared `unsupportedReferenceTokens()` catch-all (`🧹️normalization/
🟦️.ts:4712`) — it flags any path-shaped quoted string in any scanned file that no language-specific
extractor (here, Rust's `.join()`/`#[path]`/manifest-reference candidates) has claimed as a rewritable
construct. A bare `&str` array element is not such a construct.

Two things rule out the cheap fixes:
- **Not a `historicalDocumentEvidence` case**: that mechanism exempts *whole documents* by directory
  (ticket reports, prompt logs, …); this is one coordinate inside an otherwise fully live, compiled
  `.rs` test file — exempting the whole file would blind the scanner to every real path reference
  elsewhere in it.
- **Not a drop-in fit for either existing frozen-coordinate mechanism**: `frozenCoordinateEvidenceContracts`
  is hard-required to end in `.json` (JSON-pointer coordinates); `frozenMarkdownCoordinateEvidenceContracts`
  is hard-required to end in `.md` (byte-span coordinates restricted to inline-code/list-item Markdown
  syntax via `markdownSourceCoordinateSpans`). Neither validator accepts `.rs`. The honest fix is a
  *third* grammar, `frozen-rust-source-coordinates-v1`, admitting only spans that
  `rustTokens()` (already imported, already used for the (d)(1) fix) proves are inside a real Rust
  string-literal token — structurally parallel to the Markdown one, not a quiet widening of it.

Sized and not attempted this session: a fourth validator branch + a `rustSourceCoordinateSpans`
admission function + wiring into `frozenEvidenceContractIndex`/`frozenEvidenceCoordinateAuthority`'s
two-mechanism fallback + a taxonomy.json contract entry + its own test fixture — the same shape of
work the Markdown grammar itself took to land earlier in this ticket. A source-level alternative
(deriving the array from the loaded `shared://🧊️pattern-sphere.obj` fixture bytes at test time instead
of hand-copying them) was considered and rejected: this adapter crate is deliberately built without
linking the subject crate (its own docstring, line 15-18: "this adapter's oracle-only build never
links the subject crate"), and reworking how `set-unknown-statements`' inverse spec is assembled is a
semantic change to someone else's plugin test I do not have enough context to make safely.

**Not resolved. Said plainly: this row's count does not move.** Detection is untouched — I did not
add an exemption, a suppression, or a scanner change that would make this row silently disappear.

## (b) wgpu generated bundle — 4 rows — GENERATOR FIXED AND TESTED; ARTIFACT NOT REGENERATED (pre-existing, unrelated blocker)

**Mechanism, argued**: `Bun.build({ target: "browser", format: "esm" })` (wgpu's own
`renderBrowserEntry`, `📜️script.ts`) escapes every astral-plane (non-BMP) code point in string
literal text to a `\uXXXX\uXXXX` surrogate-pair escape — verified empirically, not assumed, with an
isolated repro: `🧰` (U+1F9F0, astral) comes out escaped, `café`/`日本語`/`é` (BMP) do not.
`registry/📜️script.ts`'s own generator (`emitAssetSpecTypeScript`, `generatePlaygroundRegistry`)
already emits plain, unescaped `JSON.stringify(...)` text into
`📇️registry/🤖️generated/🟦️playgrounds.ts` — the escaping is introduced *downstream*, by Bun's browser
bundler, when wgpu's own `📜️script.ts` re-bundles that generated module into `🟨️frame-worker.js`.

Chose **(ii) fix the generator** (specifically `renderBrowserEntry`, the function that actually owns
and writes both `🟨️boot.js` and `🟨️frame-worker.js`) over the other two options, argued:
- **Not (i) teach the scanner to decode `\uXXXX`**: broadens a shared, heavily-used heuristic
  repo-wide for every file kind, and doesn't solve the real problem — the *file on disk* would still
  contain escaped bytes, so `apply`'s rewrite step still couldn't write a replacement path into it at
  the right offsets. Decoding on read without decoding on write is a half-fix.
- **Not (iii) treat as non-authoritative and regenerate after the move**: the brief's own rule — "a
  row that disappears because it stopped being DETECTED, not because it became REWRITABLE, is a
  silent regression" — rules this out directly; making the plan skip a generated file wouldn't detect
  a genuine future case where the *generator itself* regresses.
- **(ii) is provably lossless**: a well-formed UTF-16 surrogate-pair escape and its literal character
  are byte-identical to any JS engine — decoding one back to the other changes zero runtime behavior,
  only which physical bytes land on disk.

`decodeAstralEscapes()` (new, exported) reverses exactly this — and only this — inside
`renderBrowserEntry`, so both `buildBootScript` and `generateFrameWorker` get corrected bytes.

**Fail-before/pass-after, both directions, verified for real, through the actual `Bun.build`
pipeline** (not a mock): `🟦️boot.ts` already exercises the exact defect today (`🟨️frame-worker.js` as
a literal filename in a `new URL(...)`) and its bundle currently succeeds, so it was used as the live
end-to-end proof instead of `frame-worker.ts` (see below). New tests in `🧪️index.test.ts`:
`decodeAstralEscapes` cross-checked against `JSON.parse` (an independent, spec-compliant `\uXXXX`
decoder) on real escaped fixtures; `renderBrowserEntry(🟦️boot.ts)` asserted to contain the literal
`🟨️frame-worker.js`, not any `\uD8xx` escape. 8/9 pass (see below for the 1).

**The actual `🟨️frame-worker.js` file on disk was NOT regenerated this session — say so plainly, the
4 rows do not clear yet.** `Bun.build` on `🧵️frame-worker.ts` itself currently fails for a real,
pre-existing, unrelated reason confirmed present *before* any of my edits (isolated repro against
unmodified `📜️script.ts`): its import graph (`🎠️kernel/🟦️component.ts` →
`🎭️actor/…/🧵️shard-client.ts` → `🖱️ui/🎨️styling/🟦️vite-elements-assets.ts` → the shared
`🦑️repo/…/📦️typescript/📦️index.ts` barrel) transitively pulls in `playwright-core`, which requires
Node builtins (`child_process`/`tls`/`inspector`/…) forbidden under a browser-target bundle. Not
mine to fix: it's a shared, hundreds-of-consumer barrel file mixing browser-safe exports with
test-orchestration-only ones, outside all three of my assigned families and too large/risky to
untangle unilaterally. `🟦️boot.ts`'s own bundle is unaffected (confirmed bundles clean) and was used
instead to prove the mechanism end-to-end. The remaining 1/9 wgpu test failure
("renders identical bytes twice…") is this same pre-existing break, confirmed present before my
changes via an isolated `renderFrameWorker` probe.

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — `dev-prompt-log` population, `referenceClosure.historicalDocumentEvidence` label
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts` — population id list/validator/contract label
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` — `HISTORICAL_PROMPT_LOG_ROOT_PATTERN`, `historicalEvidenceBoundaryOwns` (renamed), `RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_DEFINITIONS`, `RUST_MODULE_STRUCTURE_TRANSPARENT_STD_EXPRESSION_MACROS`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️historical-document-evidence/🟦️.ts` — new/updated cases
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️preflight-reference-basis/🟦️.test.ts` — symbol-extraction list updated for the rename
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️rust-finite-target-consumption/🟦️.ts`, `🔣️.json`, `🧬️schema/🔣️.json` — new case
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📜️script.ts` — `decodeAstralEscapes`, exported `renderBrowserEntry`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🧪️index.test.ts` — new tests

## Verification (real, pasted output)

```
B=$(git rev-parse HEAD)   # bb06c41f73f0122fbed315b7487428b976f99921
bun ./📜️script.ts clean taxonomy plan --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION \
  --scope "🧰️framework/🔨️modules/🖼️assets" --baseline "$B" --workers 6 \
  --plan "$T/🗑️temp/🔣️residual-plan-before.json"   # before my edits
  --plan "$T/🗑️temp/🔣️residual-plan-after.json"    # after my edits

before: moves=1089 roots=0 relocations=0 symlinks=0 removals=1 edits=40 regenerations=2 unresolved=16
after:  moves=1089 roots=0 relocations=0 symlinks=0 removals=1 edits=54 regenerations=2 unresolved=6
```

**16 → 6, a drop of 10 — not all mine, said plainly.** Row-by-row diff of the two plan artifacts:

| cleared | count | mine? |
|---|---:|---|
| `.🧬semio/…/💬️prompts/🐙️ueli.md` (2 rows) | 2 | **yes** — (c) |
| `✏️s/…/🖊️dwg/…/🧪️tests/🦀️test.rs` (`rust-path-join`) | 1 | **yes** — (d)(1) |
| 5 `.🥒️.feature` files (`mutate-ply`, `mutate-svg`×3, `mutate-obj`, `mutate-semio-image` — 7 rows total across those files) | 7 | **no** — sibling's Gherkin-scanner family, landed concurrently between my before/after runs; flagging per the brief rather than claiming it |

**Still unresolved (6) — not silently dropped, both mine stay honestly open:**
- `mutate-obj-3-0/🦀️.rs:109` — (d)(2), diagnosed not implemented (see above); unchanged before→after.
- `🟨️frame-worker.js` ×4 — (b), generator fixed and tested but the artifact itself not regenerated
  (unrelated pre-existing `Bun.build` break, see above); unchanged before→after.
- `extract_positions.py` (closed-ticket fixture-derive script) — not one of my three families, not
  touched, unchanged before→after.

Net attributable to this slice: **3 of my 8 assigned rows cleared** (both are rewritability fixes —
detection logic in `historicalDocumentEvidence`/`rustFiniteManifestTargets` was widened by exact,
individually-justified carve-outs, never weakened or bypassed), **5 diagnosed/fixed-but-blocked and
left honestly open** (1 fully diagnosed-not-implemented, 4 generator-fixed-but-not-regenerated).
`edits` rose 40→54 (+14) consistent with the 2 prompts rows plus the sibling's 5-file Gherkin family
becoming rewritable; `moves`/`regenerations` unchanged, as expected — none of this slice's fixes
create or remove a move.

`discovery.validateTaxonomy()`: 0 problems, both before and after.
