# Census — `🎠️kernel` / `📡️replication`

Baseline `bb06c41f73f0122fbed315b7487428b976f99921`. Starting point (per `📓️goal-shadow-report.md`):
kernel moves=50 unresolved=7, replication moves=64 unresolved=8 — all 15 rows `reference-syntax-unsupported`.

## Kernel (7 rows, enumerated)

0. `.dependency-cruiser.cjs` — plain `forbiddenRendererTargets`/`allowedRendererTargets` array, real repo
   paths used for `.some(pattern.test)` boundary comparison, not `.map(read)` — no existing authority.
1. `…/FULL-STDIO-…/📋️mutation-diff-result-contract.md` — bullet list, backtick-quoted real path, no
   markdown backtick-path scanner exists at all (only `[text](url)` links).
2. `…/FIX-DEMONSTRATOR-…/📜️script.ts:222` — `const paths = [...]; paths.map(p => resolve/readFileSync)`
   — real path-collection idiom, but (a) lowercase `paths` misses the `Sources|Paths|Files` suffix
   regex, AND (b) the file has unrelated `for…of` loops elsewhere, tripping the file-wide
   `hasForOfCollection` conservative gate.
3. root `📜️script.ts:10134` — `policyReadFileSafe(root, "…kernel/🦀️component.rs")` — the repo's own
   `readFileSync` wrapper (367 call sites), absent from the recognized read-function name list.
4. `🟦️glue.ts` — mid-file `//` comment, clean backtick path — TS never scans ordinary `//`/non-leading
   JSDoc comments (only the file's very first `/** */` block).
5. `⚛️reactor/📮️requests/🦀️component.rs` — backtick doc-comment embeds a real path inside a longer
   `grep -n "…" path` example string — Rust's `rust-comment-path` only ever offered the WHOLE span.
6. `📇️registry/🟦️catalog.ts` — JSDoc block preceded by `// #region`, so it isn't "leading" — same gap
   as row 4.

## Replication (8 rows, enumerated)

0. `.dependency-cruiser.cjs` — same family as kernel row 0.
1–2. root `📜️script.ts` — two more `policyReadFileSafe(root, "…replication/…")` calls — same as kernel row 3.
3–4. `🧪️vitest.config.ts` — `coverage.include`/`includeSource: ["../../🟦️component.ts"]`, the exact
   anti-pattern `@semio-tech/framework-kernel`'s own config docstring already warns about (stale name
   on move, or double-counts against `include`).
5–7. `🟦️component.ts` — three comment-path refs: a `//!` module doc naming `🧫️fixtures/wire/`, and two
   JSDoc blocks naming `📡️wire/🦀️component.rs` — same gap as kernel rows 4/6.

## Fixes (mechanism, `🧹️normalization/🟦️.ts`; one vocabulary edit, `🔣️taxonomy.json`)

- `typescriptTokens`: added `policyReadFileSafe` to the read-function regex (rows 3, 1–2).
- New `dependencyCruiserBoundaryReferenceAuthority`, gated by exact basename `.dependency-cruiser.cjs`
  only (never generalized) (rows 0, 0).
- New `typescriptCommentPathReferenceAuthority`: scans every `/* … */`/`/** … */` block and every
  whole-line `//` comment for a clean backtick path — mirrors `rustTokens`'s unconditional
  `rust-comment-path` scan (rows 4, 6, 5–7).
- `rustTokens`'s comment-path extraction: when a backtick span has whitespace (a command-example, not
  a bare path), extracts just the trailing real path substring instead of the whole span (row 5).
- New `markdownCommentPathReferenceAuthority`, reusing the existing, already-tested
  `markdownSourceCoordinateSpans` (built for frozen-coordinate evidence) — resolves plain backtick/
  path-list-item spans in prose (row 1).
- Path-collection regex: added bare `paths|sources|files` beside the `Sources|Paths|Files` suffix
  form (kernel row 2's naming half only — the file-wide for-of gate still applies, see below).
- Replication's `🧪️vitest.config.ts` rewritten to the glob form kernel already uses (rows 3–4).
- `🔣️taxonomy.json`: `json-fixture-case` gets `inferWithoutEmoji: false` — it shares `fixture-case`'s
  slug pattern and `parentKindIds`, and `📡️replication/🧫️fixtures/wire` (the ONLY no-emoji fixture-case
  folder repo-wide, verified by search) newly reached ambiguous once the other rows unblocked deeper
  traversal. Exact precedent: `asset-video-subject` vs `asset-subject`.

## What did NOT get fixed, and why (disproven hypothesis, not a bug)

Kernel row 2 (`paths` array, FIX-DEMONSTRATOR ticket) stays unresolved. I tried narrowing
`typescriptTokens`'s file-wide `hasForOfCollection` gate to a name-scoped for-of check; this broke two
already-passing, deliberately-authored tests in `🧪️typescript-path-collection`
("independent-map-in-for-of-source-conservatively-suppressed" expects `[]` even for an UNRELATED
`for…of` elsewhere in the file). The whole-file conservativism is intentional, tested behavior, not a
proxy bug — reverted. FIX-DEMONSTRATOR's script.ts has unrelated `for…of` loops elsewhere, so this row
is a correct refusal.
