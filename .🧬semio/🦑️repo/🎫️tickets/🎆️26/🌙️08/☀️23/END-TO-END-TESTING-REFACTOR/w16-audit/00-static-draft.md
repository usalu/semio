# static sections draft (to merge into 📓️w16-final-audit.md)

## Oracle population change (the headline qualifier)

| | w13 (9ed590cd87) | now (8d9b51f081 + dirty) |
|---|---|---|
| cases | 164 | 164 |
| scenarios | 4,564 | 4,910 |
| `@oracle-` cases | 79 | 121 |
| `@oracle-` scenarios | 1,331 | 3,191 |
| `@no-oracle-` cases | 85 | 43 |
| `@no-oracle-` scenarios | 3,233 | 1,719 |
| oracle-backed cases whose reference is a THIRD-PARTY package | 79 | **79** |
| oracle-backed cases whose reference is an IN-REPO second implementation | 0 | **42** |

42 of the 42 new oracles are `*-python-independent` / `*-typescript-three-independent`
registrations with `"package": ""` — a Python (or TypeScript) file written in this repository, in
this ticket, by the same wave that wrote the Rust it is compared against.

## Scenario deltas: nothing deleted

164 features before, 164 after. No `D` status on any `component.feature`.
Per-case scenario counts: 27 cases gained, **zero lost**. 4,564 → 4,910.

## Weakening knobs

* `git diff 9ed590cd87 -- '*🔣️component.json' | grep -E '^[+-]\s*"(ignoreKeys|tolerance|arrays|mode)"'` → **empty**.
* Repo-wide diff for `ignoreKeys|"tolerance"|"arrays"` outside description/rationale prose → **only prose lines**.
* `git diff 9ed590cd87 -- '✏️s/🔌️plugins/🗄️stdio/**' | grep 'const [A-Z_]*(TOLERANCE|WRITER_FREEDOM|UNOBSERVABLE|GUARD_VECTORS)'` → **empty**.
* Fixtures/examples: 4 added, 69 modified — 66 of the 69 are `.rs` demo modules in `📕️norm`;
  the only non-`.rs` modifications are the three PDF 1.4 `📚️examples/🎬️demo/🖼️assets/` files.
* Law calls: 324 → 309 across the 1,725 adapter files that existed at w13. 16 files decreased:
  15 `📕️norm` (−2 each, self-oracle removed and replaced by a Python reference — strengthening)
  and `mutate-block-2d-1` (−3).

## The PDF conformance profile change (deliberate, documented)

Six `semantic-pdf-conformance-*-v1` descriptions rewritten. `font_program`
(`✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📄️document/🦀️component.rs:1299`) now returns
`stream.get_plain_content()` length **plus a SHA digest of the decoded bytes**
(`:2127` projects `programBytes` + `programDigest`). Strictly stronger than the stored-stream
length it replaced.

## The two undeclared oracle libraries

`mutate-semio-image`'s `🐍️component.py:520` does `from PIL import Image`. Its registration
(`…/🪆️subsets/✳️image/🧪️oracle/🔣️.json`) declares `"ecosystem": "python", "package": ""`.
**Pillow appears nowhere in `🔒️dependencies.json`** (17 python entries; pypdf and simplejson are
there, Pillow is not). The runner's venv is created `--system-site-packages`, and this machine
happens to carry `PIL 11.3.0` — which is the exact version the rationale names.

`mutate-semio-mesh`'s `🟦️component.ts:48` does `import * as THREE from "three"`. `three` IS in the
ledger, but as a **production-reachable** js dependency (five `package.json` files), not as a
`test-oracle` — so `dependency` does not print it as `production-debt` the way it prints
`png`, `zip` and `image`.

## Runner remedies still open

* w13 remedy #2 — a case whose host fails to build still returns `{ results: [], problems }`
  (`📜️script.ts:523`, `:528`) and contributes `executed=0 passed=0 failed=0`.
* w13 remedy #10 — `runProbe` still `throw`s on `ETIMEDOUT`
  (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️index.ts:1708-1712`),
  so one per-case 900 s overrun still aborts the whole run with no summary.
* NEW this wave — `ownerShipsImplementation` (`📜️script.ts`) skips subject dispatch in a language
  the owner ships no package in. Exactly one case is affected: `extract-text-pdf-1-4`
  (python-only adapter, 2 scenarios). Those 2 scenarios were `errored` at w13; they are now
  reported only on stderr as `[test] no-subject-implementation` and enter no count.

## Prose / similarity

* sentences >70 chars in ≥3 features: **77**, touching **109 of 164** (w13: 78 over 122).
* max pairwise 5-gram Jaccard over features: **0.839** (`mutate-pdf-1-4-a` / `-x`); 18 pairs > 0.60,
  every one a documented conformance-class family (w13 max: 0.806).
* `todo!` / `unimplemented!` in every `🧪️tests` / `🧪️oracle` tree: **zero**.
* KNOWN OPEN DIVERGENCE paragraphs: 3 (`mutate-bmp-v3`, `mutate-gif-87a`, `mutate-gif-89a`).
  `mutate-tiff-6-0` now reads CLOSED, AT THE CAUSE.
