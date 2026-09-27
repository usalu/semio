# WP-ST2 — Stdio Completeness: Native Text Codecs, Nx Dist Inputs, Per-Family Components

Slice ST2, session 14 (2026-09-27 18:3x, Opus executor). Coordinator = `main`. Continues [ST1](📓️wp-st1.md) and
[CX1](📓️wp-cx1.md) (design: [LB](📓️wp-lb.md) "Option A"). Ports: hubs 8070–8079, serves 6570–6579. Scripts + patches:
`wp-st2/` (copies of `wp-cx1/cx1-apply.py`, `wp-st1/st1-{gen,apply}.py` evolve here as `cx1-apply.py`, `st2-gen.py`,
`st2-apply.py`). Captures `wp-st2/generated/`. Durable data `.🧬semio/🌐hub/s14-st2-*/`. Guest-linked edits = prepared
patches + overlay proofs until the coordinator announces WINDOW 3 OPEN in `📓️fleet-14-agents.md`.

## Session 14

| # | Item | State | Evidence |
|---|------|-------|----------|
| 1a | CX1 txt/tsv/html hub-native codec factories (26 → 29) | dry run on the live tree **clean** (80 hunks / 34 files / 0 problems, 18:3x); applied to the overlay 20:07; native tests queued in the overlay lane (11 waiters ahead) | `generated/cx1-dry-live-1.txt`, `generated/cx1-write-overlay-1.txt`, `generated/cx1-test-stdio-1.txt` |
| 1b | registry Nx soundness (drop `--skip-nx-cache`) | **patch prepared** `nx1b-apply.py` (8 files; dry run on the live tree **0 problems**): `generator-inputs` uncached (out of `cachedExact`), registry `check` keyed on the receipt, rebuild chain without `--skip-nx-cache` ×4; receipt law + Nx replay oracle **PASS** in the overlay, **red** with the cached producer (mutant); nx-contract policy vectors 34/34 (overlay + live). Lands window 3 (📋️project.json / rebuild JSON are frozen chain inputs) | `generated/nx1b-dry-live-1.txt`, `nx1b-law-1.txt`, `nx1b-law-mutant-1.txt` |
| 2 | ST1 per-family stdio components (88 subsets in `s`) | generator re-run on the live tree (9 families, 158 + 18 apps) + dry run **clean** (18 edits / 36 new / 1 move, 18:3x); overlay proof pending | `generated/st2-gen-1.txt`, `generated/st2-dry-1.txt` |
| 3 | every stdio kind opens/edits/exports in `s` + hub, laws + oracles | window 3 | — |

### Session 14 Log

- 18:3x start. Read preambles 14/13/12, AGENTS.md, `📓️fleet-14-agents.md` (no CHAIN LAUNCHED yet; window 2 closed since
  15:45 → no guest-linked tree edits until WINDOW 3 OPEN), `📓️wp-st1.md`, `📓️wp-cx1.md`, `📓️wp-lb.md`, the A13-w3 rows,
  the session-13 coordinator log from 14:00. CX1's overlay warm build finished 16:08 (`wp-cx1/generated/warm-stdio-2.txt`)
  but started before its 16:00 overlay write → no CX1 law was run; ST1's overlay checks never ran.
- 18:3x dry runs on the live tree: CX1 **0 problems**; ST1 generator + apply **clean**.
- 19:0x coordinator: Codex peer (ticket 26/09/26 COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE) edits stdio; its plan wants all 88
  editors shipped (gate `testEditorCatalogContract`: 36/36 formats + 88 playground rows, red "7/36" on lb-p3's tree). Coordinator
  decision 19:2x: the family packages ARE the resolution (every editor shipped, each component under the 1M-function / rustc
  ceilings); that gate's intent is ST2's acceptance target; never put `full-app-catalog` back into one component; do not edit the
  peer's files while it is active in them; re-diff every target right before landing. ST1's patch already rewrites that gate as the
  union over the 10 stdio packages (36/36 formats, 88 rows, no `[DEBUG]`).
- 19:1x–19:4x overlay: new clone overlay `s14-st2-overlay` (APFS clonefile, `st2-overlay.py`, 22 min under load 100) — then chose
  CX1's overlay path instead (`s13-cx1-overlay`, synced to the live tree 19:4x–20:05: 631 cloned, 46 214 stale removed) because its
  private build-dir `s13-cx1-build` already holds the third-party dependency closure (same absolute path → warm units).
- 20:0x item 1b root cause (read-only, `nx show project` of registry + repo, captures `generated/registry-project-1.json`,
  `repo-project-1.json`): `generate` hashes `dependentTasksOutputFiles` of the receipt `repo:generator-inputs` writes, but
  `generator-inputs` is `cachedExact` in `⚡️caching/🔣️policy.json` with inputs = Cargo.toml/project.json/taxonomy/discovery only,
  while `registryCatalogInputPaths` digests every plugin descriptor `🔣️.json`, example and the implementation TS closure. A describe
  that rewrites a descriptor leaves the key unchanged → stale receipt → stale catalog = chain run 4's "catalog stale".
  Probe `wp-st2/nx-replay-probe.ts` (two throwaway git+Nx 23.2.0 workspaces, producer → gitignored receipt → cached consumer):
  cached producer: initial runs=1 v1 | unchanged runs=1 v1 | **edited runs=1 v1 (stale replay)**; uncached producer: initial 1 v1 |
  unchanged 1 v1 (hit) | **edited runs=2 v2**. Registry `check` is cache-forced by the `check*` family and hashes neither the
  receipt nor descriptors.
- 20:07 CX1 applied to the overlay (`cx1-apply.py --write --root overlay`, 34 files); `st2-cargo.sh` (overlay lane, private
  build/target dirs) queued pid 222: stdio + txt/tsv/html `--lib --tests` → `generated/cx1-test-stdio-1.txt`.
- 20:1x item 1b patch `wp-st2/nx1b-apply.py` + payload (`nx1b-payload/`): `⚡️caching/🔣️policy.json` drops `generator-inputs`
  from `cachedExact`; its target `cache: false` (inputs removed — an uncached producer's inputs mean nothing); registry `check`
  `dependsOn repo:generator-inputs` + `dependentTasksOutputFiles` of the receipt (its descriptor/example view was unkeyed);
  `🔁️rebuild/🔣️.json` drops `--skip-nx-cache` from generate/check/activate-s/verify-s (activate-s keys transitively through
  session-s → generate → receipt and materialize-dev outputs, which the probe shows Nx hashes even when gitignored; verify-s is
  uncached); nx-contract fixture row `generator-inputs` → `authored: true`, Gherkin scenario rewritten ("re-digests on every
  run"); receipt law (`🔏️inputs/🧪️tests/🔏️receipt`): asserts the uncached producer (authored + resolved through the plugin) and
  that `generate` AND `check` key on the receipt, plus the Nx replay oracle `testGeneratorInputReceiptReplay` (fixture section
  `replay`, Ajv-validated; producer = the REAL `publishGeneratorInputReceipt`; cached producer must replay a stale catalog,
  uncached must hit while unchanged and re-run once edited); its `[DEBUG]` log became an emoji result line.
  Proof (overlay, `bun wp-st2/nx1b-law.ts`): **PASS** 65 s (`generated/nx1b-law-1.txt`); mutant (overlay project back to
  `cache: true`): **AssertionError** "the receipt digests bytes its own cache key cannot name…" (`nx1b-law-mutant-1.txt`),
  restored. `bun wp-st2/nx1b-policy-vectors.ts` (the cache-contracts policy loop): 34/34 overlay and live.
