# WP-ST2 — Stdio Completeness: Native Text Codecs, Nx Dist Inputs, Per-Family Components

Slice ST2, session 14 (2026-09-27 18:3x, Opus executor). Coordinator = `main`. Continues [ST1](📓️wp-st1.md) and
[CX1](📓️wp-cx1.md) (design: [LB](📓️wp-lb.md) "Option A"). Ports: hubs 8070–8079, serves 6570–6579. Scripts + patches:
`wp-st2/` (copies of `wp-cx1/cx1-apply.py`, `wp-st1/st1-{gen,apply}.py` evolve here as `cx1-apply.py`, `st2-gen.py`,
`st2-apply.py`). Captures `wp-st2/generated/`. Durable data `.🧬semio/🌐hub/s14-st2-*/`. Guest-linked edits = prepared
patches + overlay proofs until the coordinator announces WINDOW 3 OPEN in `📓️fleet-14-agents.md`.

## Session 14b

| # | Item | State | Evidence |
|---|------|-------|----------|
| 0 | live tree carries no half-applied ST2/CX1/1b edit | **verified 12:1x**: all three dry runs on the live tree 0 problems (every anchor exactly once, no "already applied" edit, no new file present); no `🗄️stdio/🧩️extensions`, no `🏘️composition`; diff vs yesterday's dry diff = context line numbers only | `generated/st2-dry-live-3.txt`, `cx1-dry-live-3.txt`, `nx1b-dry-live-3.txt`, `st2-gen-live-3.txt` |
| 1 | 1b registry Nx soundness | held for window 3: it edits chain inputs the running chain reads (`🔁️rebuild/🔣️.json` via `rebuild-all --from`, registry + caching `📋️project.json`, `⚡️caching/🔣️policy.json` → every `bun nx` graph) — rule 2 freezes them even though no guest links them | — |
| 2 | CX1 txt/tsv/html codecs (26 → 29) | dry run clean; overlay re-proof pending | — |
| 3 | ST per-family stdio components | generator + dry run clean on the live tree (9 families, 158 + 18 apps, 18 edits / 36 new / 1 move); overlay re-proof pending | — |
| 4 | every stdio kind opens/edits/exports in `s` + hub | window 3; hub publisher rows for the 9 families added to the code part (guest-codec packages, `linkedCodecRegistry: null`, right after `stdio` in `--packages all`) | `generated/st2-dry-live-code-5.txt` |
| U1 | URGENT chain fix (coordinator 12:4x): `XlsxSnapshot` became OPC `xml_parts` (peer 04:35) → vcs xlsx import/export red (E0609/E0560), same break in forms + architect program exporters | vcs: export = stdio `build_minimal_xlsx`, import = `XlsxSnapshot::project_workbook`, first column = col 0 (was B); calamine oracle law replaces the bare round trip; forms: same export fix + new calamine law; architect: exporter + unit test + oracle-case projection onto `project_workbook`. Test-only `calamine 0.36.1` dev-dep in vcs + forms with the two Cargo.lock edges (`cargo metadata --locked --offline` rc 0). Native test queued (priority stamp) | `generated/native-vcs-1.txt`, backups `vcs-xlsx-before/` |

| U2 | forms LIB red on the live tree (peer 04:41/04:46): 3 panels called `crate::editor::forms::forms_play_labels` (only `…::terminology::` has it) + test-only: inspection test on the old `render(…, labels, …)` signature, editor unit test `include_str!` one dir too high | obvious path fixes (non-test lib code → chain-failure category, like U1) | `generated/native-forms-1.txt`, `native-fix2-1.txt` |
| U3 | H14 relay: `🛂️manifest/🧫️fixtures/🗄️artifact-kind-formats.json` fails its schema (`componentKind` const `hostSnapshot` since the 09-15 fixture→hostSnapshot terminology rename, fixture kept `"fixture"`; Rust `component_kind` is a free string) | fixture value → `hostSnapshot` (rule 22, test-only); hub `trusted-stdio-gis-bundle-check --source` now passes it and stops at the NEXT red: `🔗️compiled-dependencies` rawCases hex descriptors still carry `appChannelVersion` 18 vs constant 19 (H14's channel-19 fixture pass, hub owner) | `generated/kind-formats-source-1.txt`, `native-fix2-1.txt` |
| — | CX1 generation anchor | `BOOTSTRAP_GENERATION_OLD` → `76d1a92f…` (H14 channel 19); NEW in window 3 via `bun .tmp-ticket/wp-h14/h14-bootstrap-generation.ts 19` after the CX1 codec change (my overlay probe `cx1-generation.py` gave `0021ba99…` on the 12:26 channel-18 tree — superseded); dry run 0 problems | `generated/cx1-dry-live-5.txt` |

### Window-3 runbook (ST2, compile-atomic, one step at a time; start on "WINDOW 3 OPEN")

1. Re-dry-run on the live tree: `python3 wp-st2/nx1b-apply.py`, `cx1-apply.py`, `st2-gen.py` + `st2-apply.py --dry-run --part code`
   (all must report 0 problems; else re-derive the refused anchor first).
2. 1b: `nx1b-apply.py --write` → `bun wp-st2/nx1b-law.ts /Users/ueli/Documents/semio` + `bun wp-st2/nx1b-policy-vectors.ts` → row.
3. CX1: `cx1-apply.py --write` → `bun .tmp-ticket/wp-h14/h14-bootstrap-generation.ts --write 19` (stdio receipts 26 → 29 change
   the stdio-gis generation; the tool writes all 4 occurrences) → native lane: `cargo test -p semio-s-plugin-stdio --lib --tests`
   (+ `--features full-artifact-catalog --test native_openable_provider`), `-p semio-s-artifact-stdio-{txt,tsv,html} --lib --tests`,
   `-p semio-hub --lib` (fence/provider laws) → row.
4. ST code: `st2-apply.py --write --part code` → native: `cargo test -p semio-s-plugin-stdio --test shipped_fleet --test editor_catalog`
   + `cargo check -p semio-s-plugin-stdio-{image,media,cad,bim,mesh,pdf,office,semio,binary} --lib --tests`; wasm lane:
   `cargo check --target wasm32-wasip2` of the 10 stdio packages, then `bun nx run @semio-tech/stdio-plugin:editor-component-check`
   (links + validates every stdio package component; the 1M-function ceiling proof, stdio-semio = 38 apps is the largest) → row.
5. RELAY R10: `st2-apply.py --write --part r10` as its serialized step, then its refresh/taxonomy/render (158 stdio launch rows).
6. Descriptors: stdio + 9 families `describe` (wasm; stdio's committed descriptor goes stale with steps 3–4 — its
   `descriptor_is_fresh` law is red until then), `plugin-registry:generate`, play pane coverage law, stdio catalogue contract.
7. Item 4: `serve s react dev` (6570) opens/edits/exports one document per stdio kind (os-dev program-matrix rows), then a
   `--packages all` publish (families included via the hub publisher rows of step 4) → hub-document-sweep on the fresh hub.

### Session 14b Log

- 12:0x start (successor). Read preamble 14 (+14b), AGENTS.md, fleet log, this report, the Codex stdio ticket (plan, validation):
  the peer still ships the nine-editor stdio component; its gate `testEditorCatalogContract(…, shipping)` + `--full-catalog`
  diagnostic are the 19:11 shape the ST patch already rewrites (union over the ten stdio packages, no `[DEBUG]`).
- 12:09 generator re-run on the live tree: identical partition (`generated/st2-gen-live-3.txt`); plan/payload of 09-27 kept in
  `generated/prev-s14/`.
- 12:1x overlay `s13-cx1-overlay` re-synced (2 395 + 86 files; docx/xlsx changed 12:12–12:19 by the chain-green fixer) and
  CX1 + 1b + ST (all) re-applied (`generated/{cx1,nx1b,st2}-write-overlay-4.txt`). Overlay proofs: taxonomy load valid
  (`taxonomy-probe-overlay-3.txt`); stdio catalogue contract **88 editors / 10 packages / 36 formats** (`stdio-contract-overlay-3.txt`),
  mutant (family 🔢️binary moved away) **red 32/36** (`…-mutant-3.txt`); live tree still red 7/36 as the peer expects
  (`stdio-shipping-contract-live-3.txt`); 1b receipt law **PASS** (`lane-job1-1.txt`); native `cargo check -p semio-s-plugin-stdio`
  + the 9 families `--lib --tests` **rc 0** in 11.5 min (overlay lane, private build-dir; only warning in the families = unused
  `extern crate value_derive` → removed from the generator template together with the unused `semio-framework-value-derive` dep).
  Play pane coverage: wrong vitest filter (the law is registered in-source by `🔨️modules/🧩️runtime/🟦️.ts`) → rerun pending.
- 12:2x `st2-apply.py --part code|r10|all`: the r10 part (taxonomy, workspace-contract counts, root policy row, every
  `📋️project.json`, the composition move + referrers, launch seed/json) is R10's serialized window-3 step; code-then-r10 ==
  all byte-identical (scratch mini-root), re-application refused. RELAY R10 sent via main.
- 12:3x hub publication for the families (item 4 prep): the hub admits the same kind in several packages (codec identity =
  plugin + package + kind + schema; `GuestArtifactCodecBinding` for packages without a linked provider), so each family
  publishes like note/draw: own component-probed codec rows, open targets bound to them. Added to the code part
  (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`: 9 `TRUSTED_BOOTSTRAP_PACKAGES` rows + `--packages all` order) — lands only after
  W4's publish (the chain's `--packages all` reads that list).

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
