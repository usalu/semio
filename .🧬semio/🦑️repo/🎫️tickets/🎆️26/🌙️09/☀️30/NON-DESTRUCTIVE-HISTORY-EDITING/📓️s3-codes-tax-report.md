# 📓️ S3-CODES-TAX — Outcome-Law Gate Proofs, Typed Refusals, Sealed Evidence

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`, executor S3-CODES-TAX (session 3, successor of S2-CODES + S2-TAX), started
2026-10-02 19:40, cut ~21:00 by the usage limit, resumed 2026-10-03 10:45. Findings: `📓️audit-s2-wave-a.md` C-1…C-5, T-1, T-2;
owed reruns `📓️w3-codes-report.md` S2.7. Scratch: `🗑️generated/s3-codes-tax/`. Legend: DONE (ran, saw it pass), WRITTEN BUT
UNVERIFIED (reason), OPEN (whose).

## Session 3 — 2026-10-02 / 10-03

### Status

| finding | status |
|---|---|
| C-1 gate proofs as a repo test | DONE (25/25 direct, route and nx) |
| C-2 typed `refuse` | DONE (replication 260/0); ✏️s callers owed on TREE GREEN |
| C-3 one level spelling (builders + gate) | DONE (gate green, core check 0 errors); ✏️s compile owed |
| C-4 generators regenerate layout fixtures | DONE (4 generators, dry run = 0) |
| C-5 checked-adapter results in other editors | DONE (puzzle 2d preview); 2 notes |
| T-1 sealed CAD golden / energy pin | DONE (liveBindings split, seal ledger) |
| T-2 stale red observation + ownerless reds | 4 green (incl. 💥️ re-sealed via the ledger), 1 deleted (dead), Draw producer → S3-INFRA; extra `🔤️` green |
| S2.7 reruns (TREE GREEN) | kernel vocabulary 1/1, replication 260/0; ✏️s + plugin lib-test owed (peer) |

### Resume repair (10-03 10:45)

The 10-02 cut left C-3 half applied: the replication builders were already `warning`, but ~850 Rust files still called `.warn(` /
`MutationMessage::warn(` (the coordinator's 05:50 "space `MutationOutcome::warn`" red was this, not the peer). C-2 callers were
complete. Repaired by the idempotent codemod `🧪️s3-codes-tax-warning-builders.py` (873 sites in 848 files; second dry run: 0).

### C-1 — the gate's planted-violation proofs are a permanent repo test — DONE

- Owner: the gate lives in the root `📜️script.ts` (`//#region 🔧️PolicyRuleMutationOutcomeMergePolicy`), so the law sits in the root
  package: `🧪️tests/🧪️outcome-law-gate/🟦️.ts`, fixture `🧫️fixtures/🧫️outcome-law-gate/🔣️.json` (16 planted files: 11 with breaches,
  5 canonical controls; every position of both ticket scripts — 13 codes + 6 level aliases — plus retired builders and a multi-line
  outcome document), schema `🧬️schema/🔣️outcome-law-gate/🔣️.json` (draft-07, breach `oneOf` per fault class).
- Gate refactor (no behaviour change except the fixes below): rule 2 is now the pure per-file `policyOutcomeCodeFileBreaches(vocabulary,
  relPath, content)` driven by the git-inventory walker `policyMutationMessageCodeBreaches`; `policyOutcomeVocabulary` exported.
  Fixes found by the vectors: a scanned level is typed as spelled (the `as PolicyOutcomeLevel` casts that contradicted the type are
  gone — audit C-3 second half); an outcome document repeating a code now attributes each breach to its own line (was: first
  occurrence); a `null`/non-object outcome document or message no longer throws.
- Oracles: Ajv validates the vectors against the schema; the vocabulary document is the level oracle (expected levels and classes are
  re-derived from it); the TypeScript compiler re-derives the TypeScript positions (`refuse(..)` calls, `{level, code}` objects, leaf
  literals) and must equal the gate; Ajv re-derives the outcome-document positions from a schema built from the vocabulary.
- Routing: root `📜️script.ts` `test outcome-law-gate`; root `📋️project.json` target `test-outcome-law-gate`, and `mutation-outcome-law`
  now `dependsOn` it (the gate cannot run without its proofs); `.vscode/🧩️launch.seed.jsonc` row `⚖️gate🎯️mutation-outcome-law🧪️planted`
  (4_gate, order 411.427, after `⚖️gate🚫️history-closure`). Read-only render probe (`🗑️generated/s3-codes-tax/probe-launch.ts`): the
  row appears in the rendered `launch.json`, nothing removed.
- Runs: `bun test ./🧪️tests/🧪️outcome-law-gate/🟦️.ts` **25/0**; `bun ./📜️script.ts test outcome-law-gate` **25/0**;
  `NX_DAEMON=false bun nx run workspace:test-outcome-law-gate --skip-nx-cache` **25/0**; `verify taxonomy report --scope` on the three
  new directories: clean (0 errors, 0 warnings each).
- The ticket scripts `🧪️w3-codes-gate-negatives.ts`, `🧪️s2-codes-level-alias-negatives.ts` are superseded (kept as ticket inputs; they
  plant `.warn(` and would now also see the retired-builder breach).

### C-2 — `MutationOutcome::refuse` takes only vocabulary codes — source complete, compile proven, unit tests blocked by a peer

- `📡️replication/🎮️mutation/🦀️.rs`: `pub enum OutcomeCode` (9 variants, `ALL`, const `as_str`, const `level`); `OUTCOME_CODES` is
  derived from it in a const block (one source); `refuse(code: OutcomeCode, ..)` — the `unwrap_or(Fatal)` degradation is gone, a
  non-vocabulary code is unrepresentable. `From<OutcomeCode>` for `&'static str` and `FaultCode` (wire spelling where a bare code
  travels). Kernel facade re-exports `OutcomeCode` (`📡️spr/🦀️.rs`).
- Callers typed: raster `validate` ×5 (`Result<(), protocol::OutcomeCode>`; editor `patch-layer` maps to `Fault` by `as_str`; binary
  store `?` converts), stdio contract `SnapshotEditError::outcome_code() -> kernel::OutcomeCode`, glTF `rejection_outcome_code -> OutcomeCode`
  (+ fixture-corpus test `.as_str()`), gif 87a/89a, bmp `replace-pixel-data`.
- Tests: `🎮️mutation/🧪️tests/🧪️outcome-code/🦀️.rs` — `the_typed_codes_are_the_vocabulary` (typed = fixture, distinct),
  `refuse_picks_the_vocabulary_level` (every typed code, level, target), `the_warning_builders_build_the_warning_level`.
- `cargo check -p semio-framework-replication -p semio-framework-os-kernel -p semio-framework-plugin --lib`: **exit 0** (147 plugin
  warnings — type-check proof). `cargo test -p semio-framework-replication --lib`: the lib-test target was red
  from the finished schema-registry split (`🧾️wire/🧪️tests/🔬️unit/🦀️.rs:2` imported only part of the moved items → E0425
  `artifact_inference_catalog_len`, `…_descriptor_registered`); fixed by importing them from their public home
  `semio_framework_schema_registry` (coordinator instruction). `cargo test -p semio-framework-replication --lib` (11:25, gated,
  private target): **260 passed / 0 failed**, incl. `the_typed_codes_are_the_vocabulary`, `refuse_picks_the_vocabulary_level`,
  `the_warning_builders_build_the_warning_level`, `the_vocabulary_table_is_the_fixture`.

### C-3 — one level spelling in the API as on the wire — source complete, gate green

- Builders renamed `MutationMessage::warn` → `warning`, `MutationOutcome::warn` → `warning` (docs included); codemod
  `🧪️s3-codes-tax-warning-builders.py` (all git-visible `.rs` outside `.🧬semio/` + this ticket's Rust-emitting generators; embedded
  `console.warn(` excluded; idempotent): 873 sites / 848 files on resume (the 10-02 run had converted ~440 files before the cut).
- Exact file list for the closing audit: `📓️s3-codes-tax-warn-rename-files.md` — against `HEAD` 202c4b7b5b1, 1,305 sites in 1,263 tracked
  files: **1,196 pure** (applying exactly the three codemod rules to the `HEAD` bytes reproduces the working tree byte for byte) and
  67 mixed (the file also carries other uncommitted changes — peers or other WPs — audit with `git diff HEAD -- <file>`), plus 2
  untracked files.
- Gate: Rust builders and the chainable form are matched as `warning`; `MutationMessage::warn(..)` anywhere and `.warn(..)` inside an
  outcome-building body are reported as the retired builder spelling (vectors `rust-retired-builders`, control `rust-canonical-leaf`).
- Runs: `bun ./📜️script.ts verify mutation-outcome-law` **passed** (0 breaches, 7 rules, 107 s, after the codemod). Root crates touched
  by the codemod (`space-space`, `space-collection`, `workflow-workflow`, `os-kernel-db`, `semio-framework-os`): `cargo check --keep-going`
  shows only the peer ValueError/TextError wave (E0277/E0308/E0053, 0 method-resolution errors); `✏️s` crates wait for TREE GREEN.
- Note (hub owner): `🌎️hub/🏗️bootstrap/🦀️.rs:6225` builds a `MutationMessage` at Warning with the non-vocabulary code `hub.unavailable`
  (a check-in refusal wire, not a persisted outcome; the gate sees only literal codes).
- Peer evidence: `.🧬semio/…/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🔬️lexical-grammar-contract/🧫️fixtures/{🧬️caller-cut,🧬️original-inputs}`
  snapshot source text containing the old `MutationMessage::warn(` spelling (their ticket evidence; not touched).

### T-1 — sealed CAD golden and the "previous contracts unchanged" pin — DONE

- History (git, probe `🗑️generated/s3-codes-tax/probe-ledger.ts`): the golden `cad-draw-projection-vectors-v1` was re-sealed four times
  since the energy pin (1410a74c → ff356208 09-25 re-authoring → 9264c9de 09-30 → 8bfd3766 10-01), the last two only because
  `liveBindings` (carrier names) sat inside the sealed bytes; three other contracts were re-sealed silently on 09-09/09-12/09-19.
- Split: `liveBindings` removed from the sealed golden (only that key; JSON-equal otherwise, sealed coordinates byte-identical:
  `coordinatesSha256` 51393583…) and moved to the UNSEALED `📚️library/🧫️fixtures/🧫️cad-draw-live-bindings/🔣️.json` (schema
  `🧬️schema/🔣️cad-draw-live-bindings`): one row per golden mapping `{mapping, live}`, `live` a full repository path so moves rewrite
  it like any reference. Seal → `eca5acc0…` in `🔣️taxonomy.json` (Edit tool) and the mirror `🧫️fixtures/❄️frozen-coordinate-evidence`.
  `🔬️workspace-contract` reads the new fixture (`cadProjectionLiveSources`), plus a new law (Ajv-valid, one row per mapping, no live
  data sealed, a `../` row is refused by schema and reader).
- The pin's meaning restored as a ledger: `📚️library/🧫️fixtures/🧫️frozen-seal-ledger/🔣️.json` (schema `🧬️schema/🔣️frozen-seal-ledger`)
  pins the 40 contracts present when the energy source was registered (`9b605a4550f`) by seal digest and records every later re-seal
  `{seal, coordinates: changed|unchanged, coordinatesSha256, recordedBy, reason}` (9 rows: cad ×4, readme-current ×2, readme-reviewed ×2,
  purity ×1); contracts registered after the pin go to `later` (none). The law (`🕰️historical-json-source-encoding`): every live contract
  is pinned or `later`; the last recorded seal is the live seal; an `unchanged` re-seal carries the previous coordinate digest; the last
  digest equals the coordinates the live bytes resolve. A re-seal without a ledger row (or a coordinate edit without one) fails. The
  energy fixture's stale `originalContracts {count, canonicalSha256}` snapshot is deleted (it was 38 at its own commit while 40 existed).
- Runs: `🕰️historical-json-source-encoding` **24/0**; `🔬️workspace-contract -t "CAD|artifact-example-model-catalog|artifact path
  projection authority"` 22/6 — the 6 are the Draw-producer reds below (T-2), every CAD test passes.

### T-2 — stale and ownerless reds in S2-TAX's files

| test | before | now | change |
|---|---|---|---|
| `📍️draw-destination-observation` | 0/1 | DELETED (rule 32) | it observed an 11-file live Draw destination that no longer exists (live Draw = `🦀️.rs` only; the golden's Draw projection is the synthetic scenario); its reader lived only in the test. Deleted: `🧪️tests/📍️draw-destination-observation/`, `🧫️fixtures/📍️draw-destination-observation/` (+`🧪️registration`), `🧬️schema/📍️draw-destination-observation/` (+`🧪️registration`), library `📜️script.ts` branch, `📋️project.json` target, `package.json` script, seed row `🧹clean🧩️taxonomy🎯️draw-destination-observation`, print oracle `productionDebt.reachableFrom` entry. `git grep` outside tickets: only generated `.vscode/launch.json` remains (coordinator regeneration). |
| `🏺️historical-package-owner-identity` | 25/1 | **26/0** | the transaction test needed the census row 29 file (`🖍️draw/…/📦️glue.rs`, pre-09-25 Draw shape) inside the current Draw scenario; it now derives the census row from the scenario's `✨️macros/🦀️.rs` mapping (same fields, same frozen coordinates) and proves the frozen census stays byte-identical while the live neighbour is rewritten. |
| `☂️frozen-coordinate-wildcard-coverage` | 4/1 | **5/0** | the 40 hard-coded byte offsets were tokens of the 2026-08-29 bytes (all 40 stale after the 09-19 re-seal, 229 → 227 rows); the law now derives every row's column-0 token from the live bytes with jsonc-parser (UTF-16 offsets) and requires the registration to cover all of them (stronger than 40 samples). |
| `❄️frozen-markdown-coordinates` | 34/2 | **36/0** | (1) "ten contracts / 38 JSON contracts" snapshot → the ten stay byte-exact, JSON contracts must equal the seal ledger's set; (2) the transaction wrote into a swept closed-ticket `🧾️runs` dir → OS temp (realpath, cleaned in `afterAll`), no evidence writes into tickets; (3) codemod damage restored: `source` was rewritten from `🦀️component.rs` to `🦀️.rs` (= `final`, no move possible); (4) the history documents moved out of ticket paths, which the `ticket-workspace` historical-evidence population now excludes from reference scanning wholesale; (5) the live neighbour's rewrite is asserted semantically (file-relative or root-relative form). |
| `💥️nested-cargo-collision-authority` | 25/1 | 25/1 — OWNER named | sealed `nested-cargo-packages-v1` catalog names wgpu target members with pre-09-05 emojis (`🧵️frame-worker`, `🧊️renderer-boot`, `🧵️browser-boot`, …) that the 09-05 member-emoji pass renamed in `members-of-wgpu-target` (`🎞️frame-worker`, `🎬️renderer-boot`, `🚀️browser-boot`, …), and `typescript-language` is no longer an allowed `🦀️rust` boundary kind; its `semanticOwnerRoot` (`📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu`) no longer exists. Owner: the wgpu renderer package projection (re-seal the catalog with ledger rows, or retire the wgpu row) — coordinator to assign (renderer / REPO-PATH-BUDGET). |
| `🔬️workspace-contract` Draw producer | 6 red | 6 red — OWNER named | plugin-registry generator input discovery follows static imports only: `📜️script.ts` `registerLazy(… await import("./📽️projection/🟦️.ts"))` (+ `🎮️playground/*`, `🗿️taxonomy-validation`, `🚀️launch` via projection) and the `createRequire(import.meta.url)("./🟨️.cjs")` modules of `🗂️workspaces/{📦️payload,🟦️bun}` are outside the closure → isolated producer misses `🚀️launch`, fixture producer `Cannot find module './🟨️.cjs'`, workspace-import binding not captured — and the generator's Nx inputs likely miss them (cache-correctness defect). Owner: plugin-registry / S3-INFRA (discovery must admit literal dynamic `import()` and literal `createRequire` specifiers). Fixed on the way: `📚️library/🟦️.ts` `import { type ProcessOwnerContextV1 }` → `import type` (production parser and TS oracle disagreed on an all-type named import). |

### C-4 — generators regenerate every committed file — DONE

- `🧪️w3-t-layout-author-vectors.py` (the slug map had already been re-pointed to the §14 short slugs by S3-LAYOUT: the 79-file drift
  of S2.8 was down to 17): the remaining 17 per-case Rust readers differed only because the peer pack/diagnostic extraction rewrote the
  committed readers (`dsl::os_pack::from_json_str(X)` → `semio_framework_pack_json::from_json_str(X, JsonMemberPolicy::Reject)`,
  `dsl::os_pack::to_json_string` → `semio_framework_pack_json::to_json_string`, `protocol::Severity` →
  `semio_framework_diagnostic::Severity`); the template now emits the current API. `.venv/bin/python 🧪️w3-t-layout-author-vectors.py`
  (dry run): **17 vectors checked against numpy and jsonschema; 0 files pending**.
- Same drift in the other two generators S2-CODES had edited: `🧪️w3-t-puzzle-author-vectors.py` (`dsl::json::*` → pack_json) and
  `🧪️w1-f-author-selection-vectors.py` (`protocol::Severity`). They have no dry-run mode, so a read-only harness
  (`🗑️generated/s3-codes-tax/dry-run-generator.py`: every write captured in memory and compared to disk) proves them: puzzle 3d
  **138 identical / 0 differing**, puzzle 5d **120 / 0**, puzzle 2d selection **151 / 0** — the last after regenerating its one stale
  file, the 2d fixture digest catalog `◻️2d/…/🧫️fixtures/🧬️mutations/🔣️.json` (2 outcome digests moved by the 10-01 `warning` rename,
  20 lines for a peer's new `🌱create-node/🌱️appends-node-c` case), written through the generator's own output (only that file).

### C-5 — refused leaves in preview overlays — DONE (other editors checked)

- Census of every discarded apply result outside tests (`let _ = apply_*_mutation(` / checked adapters): generation2d/3d are named
  by S3-PROCEDURAL (`_refused_paints_nothing` + docstring). Fixed the same defect in puzzle 2d's select-tool preview
  (`🧩️puzzle/◻️2d/…/🪛️utilities/🖱️select/🦀️.rs` `puzzle2d_select_tool_preview`): the binding names the intent and the docstring states
  that the commit of the same entry reports the refusal as the history row's outcome. WRITTEN BUT UNVERIFIED (✏️s compile owed).
- Notes, not changed (owners): (a) 10 stdio `📸️set-snapshot/🦠️mutation/🦀️.rs` facets (tsv, avi, wav, semio ×7) wrap the aggregate apply
  in `pub fn apply(..) { let _ = apply_X_mutation(..); }` — the `MutationOutcome` is discarded and no caller was found (S3-STDIO: delete
  the dead facet or return the outcome). (b) wgpu Shell `replay_local_folder_events` (`🐚️Shell/🎯️targets/🧊️wgpu/📎️local-folders`) skips a
  refused committed local-folder event, mirroring React `replayLocalFolderEventsV1` (a replay of a committed local-only log, not a
  preview; S3-W2C decides whether a refused committed event should surface).

### Also fixed on the way (red library law, cause in a peer move)

- `🔤️taxonomy-leading-grapheme` 4/6 → **10/0**: `splitLeadingEmoji` moved from `🧹️normalization/🟦️.ts` to `🧹️normalization/🛣️path/🟦️.ts`
  (202c4b7b5b1) and became `export`ed; the law now parses the helper's new home and strips `export` from the extracted closure it
  compiles with Bun and TypeScript.

### Verification (10-03, all run by me)

| check | result |
|---|---|
| `bun test ./🧪️tests/🧪️outcome-law-gate/🟦️.ts` / `bun ./📜️script.ts test outcome-law-gate` / `NX_DAEMON=false bun nx run workspace:test-outcome-law-gate --skip-nx-cache` | 25/0 each |
| `bun ./📜️script.ts verify mutation-outcome-law` (after the codemod) | **passed**, 0 breaches, 7 rules |
| `cargo check -p semio-framework-replication -p semio-framework-os-kernel -p semio-framework-plugin --lib` | exit 0 (147 plugin warnings) |
| `cargo test -p semio-framework-replication --lib` | **260/0** |
| `cargo test -p semio-framework-os-kernel --lib -- persisted_messages_admit_exactly_the_outcome_vocabulary` | **1/0** |
| `cargo test -p semio-framework-plugin --lib -- command_rejection_tests time_travel history_code` | BLOCKED: plugin lib-test target red, 129 errors, all the peer ValueError/sqlite-snapshot trait wave in plugin test fixtures (E0277/E0053/E0560; 0 from this WP) |
| `cargo check --keep-going` space-space, space-collection, workflow-workflow, os-kernel-db, semio-framework-os | only the peer ValueError/TextError wave; 0 method-resolution errors |
| `cargo check --manifest-path ✏️s/Cargo.toml -p …layout-layout -p …lowpoly-lowpoly -p …raster-raster -p …puzzle-2d --lib` | BLOCKED: stdio jpg/gif/ply/dxf/svg/pdf red from the peer sqlite-snapshot migration (0 errors naming `warning`/`OutcomeCode`/`refuse`) |
| library `bun test` (absolute paths) | `🕰️historical-json-source-encoding` 24/0, `🏺️historical-package-owner-identity` 26/0, `☂️frozen-coordinate-wildcard-coverage` 5/0, `❄️frozen-markdown-coordinates` 36/0, `🔤️taxonomy-leading-grapheme` 10/0, `🧪️mutation-leaf-identity` 10/0, `🧪️mutation-wire-witness` 11/0, `💥️nested-cargo-collision-authority` 25/1 (owner named), `🔬️workspace-contract -t "CAD\|artifact-example-model-catalog\|artifact path projection authority"` 22/6 (owner named) |
| layout / puzzle 3d / puzzle 5d / selection generators (dry run) | 0 / 0 / 0 / 0 files pending |
| `verify taxonomy report --scope` on the 7 new directories | clean ×7 |
| `.vscode/launch.json` render probe (seed → launch) | my row present, draw-observation row gone, no other drift (the coordinator regenerated launch.json at 11:32) |
| `verify dependencies literal-external` | red, unrelated (270 literal-external, oracle conflicts `js:xstate`, `rust:image`, `rust:serde_json`) |

### Owed (TREE GREEN for `✏️s` — stdio sqlite-snapshot peer migration)

1. `cargo check --manifest-path ✏️s/Cargo.toml --workspace --lib --keep-going` — proves the 848-file builder rename and the typed
   `refuse` callers (raster ×5 + `patch-layer`, gif 87a/89a, bmp, glTF, stdio contract) across every plugin crate.
2. S2.7 reruns: `cargo test -j 2 --manifest-path ✏️s/Cargo.toml --no-fail-fast --lib -p semio-s-artifact-layout-layout -p
   semio-s-artifact-lowpoly-lowpoly`; `-p semio-s-artifact-stdio-gltf -p semio-s-artifact-stdio-zip -p semio-s-artifact-wfc-bitmap`;
   media `-p semio-s-artifact-stdio-{png,jpg,wav,tiff,mp4,gif,mp3,avi,bmp,pptx}`; `-p semio-s-artifact-energy-model`; raster + puzzle 2d
   (C-2/C-5); remodel `parity exhaustive --case 📸️mutate-remodeling-1`.
3. Plugin `cargo test -p semio-framework-plugin --lib -- command_rejection_tests time_travel history_code` once the peer ValueError
   sweep reaches the plugin test fixtures.

### Coordinator actions

- None for activation/describe/schema generate from C-1/T-1/T-2/C-4 (TypeScript, fixtures, ticket generators). The Rust API change
  (`OutcomeCode`, `warning` builders) reaches every plugin component on the next activation chain run.
- Assign the two named owners: wgpu nested-cargo catalog vs `members-of-wgpu-target` (💥️), plugin-registry generator input discovery for
  dynamic `import()` / `createRequire` (🔬️ Draw producer ×6).
- Notes for owners: S3-STDIO (10 dead set-snapshot `🦠️mutation` apply facets), S3-W2C (`replay_local_folder_events` skips refusals),
  hub (`hub.unavailable` built as a Warning `MutationMessage`).

### Files (session 3)

- Created: `🧪️tests/🧪️outcome-law-gate/🟦️.ts`, `🧫️fixtures/🧫️outcome-law-gate/🔣️.json`, `🧬️schema/🔣️outcome-law-gate/🔣️.json` (repo
  root); `📚️library/🧫️fixtures/🧫️cad-draw-live-bindings/🔣️.json`, `📚️library/🧬️schema/🔣️cad-draw-live-bindings/🔣️.json`,
  `📚️library/🧫️fixtures/🧫️frozen-seal-ledger/🔣️.json`, `📚️library/🧬️schema/🔣️frozen-seal-ledger/🔣️.json`; ticket inputs
  `🧪️s3-codes-tax-warning-builders.py`, `📓️s3-codes-tax-warn-rename-files.md`.
- Edited: root `📜️script.ts` (rule-2 region: pure per-file rule, level typed as spelled, outcome-document line attribution, retired
  builder breach, `test outcome-law-gate` route), root `📋️project.json`, `.vscode/🧩️launch.seed.jsonc`; `📡️replication/🎮️mutation/🦀️.rs`
  (+ `🧪️tests/🧪️outcome-code`), `📡️replication/🧾️wire/🧪️tests/🔬️unit/🦀️.rs`, `💻️os/🔨️modules/📡️spr/🦀️.rs`; raster ×5 leaves + transform
  test + `patch-layer` editor, gif 87a/89a, bmp, glTF top-level + fixture-corpus test, stdio contract `✏️editing`; 848 files of the
  builder rename (list above); `📚️library/🔣️taxonomy.json` (one seal), `📚️library/🟦️.ts` (`import type`), library tests
  `🔬️workspace-contract`, `🕰️historical-json-source-encoding`, `🏺️historical-package-owner-identity`, `☂️frozen-coordinate-wildcard-coverage`,
  `❄️frozen-markdown-coordinates`, `🔤️taxonomy-leading-grapheme`; fixtures `📐️cad-draw-path-projection` (liveBindings out),
  `❄️frozen-coordinate-evidence`, `🕰️historical-json-source-encoding/🧬️energy-source-coordinates` (stale pin removed), puzzle 2d
  `🧫️fixtures/🧬️mutations/🔣️.json` (regenerated); puzzle 2d select tool; `📓️print/🔮️oracles/🔣️.json`; library `📜️script.ts`,
  `📋️project.json`, `package.json`; ticket generators `🧪️w3-t-layout-author-vectors.py`, `🧪️w3-t-puzzle-author-vectors.py`,
  `🧪️w1-f-author-selection-vectors.py`, and the Rust-emitting `🧪️*.py` generators touched by the rename.
- Deleted (rule 32, zero references left outside tickets): `📚️library/🧪️tests/📍️draw-destination-observation/`,
  `📚️library/🧫️fixtures/📍️draw-destination-observation/` (+`🧪️registration`), `📚️library/🧬️schema/📍️draw-destination-observation/`
  (+`🧪️registration`).
- Scratch (mine, `🗑️generated/s3-codes-tax/`): probes `probe-launch.ts`, `probe-seal.ts`, `probe-reseals.ts`, `probe-ledger.ts`,
  `probe-offsets.ts`, `probe-nested.ts`, `layout-diff.py`, `dry-run-generator.py`, `dry-diff-one.py`, `author-outcome-law-gate.py` + outputs.

### Resume 2 (10-03 11:35) — `💥️` re-sealed through the seal ledger — DONE

- Cause, proven from history: the sealed `nested-cargo-packages-v1` catalog (`📚️library/🖼️assets/📽️nested-cargo-package-projection/🔣️.json`)
  still named seven wgpu target member directories by their pre-09-05 spelling. Commit `fe7c8a8f8bd` (2026-09-05) renamed them in
  `members-of-wgpu-target`: `🧵️browser-boot` → `🚀️`, `🧵️browser-frame-transport` → `🚚️`, `🧵️browser-interactive-job-port` → `🔌️`,
  `🧵️browser-worker` → `🌐️`, `🧵️frame-worker` → `🎞️`, `🧵️interactive-job-registry` → `📇️`, `🧊️renderer-boot` → `🎬️`.
- Re-seal: exactly those seven names renamed where they are destination directory segments (followed by `/`; 15 sites: destination paths,
  adapter target + content, generated-source retirement, authored destination fragments, coverage include). Source-side file names
  (`🟦️typescript/🧵️frame-worker.ts`, joined-path reads) keep their historical spelling. Seal `9445dea9…` → `21b8e001…` in `🔣️taxonomy.json`
  (Edit tool).
- Ledger: `🧫️frozen-seal-ledger` gains `catalogs` (schema extended): every `authorityCatalogSha256`-pinned projection contract
  (`nested-cargo-packages-v1`, `readme-license-owner-leaves-v1`) with the seal registered at `sealedBy` (`8add1df1473`) and, for nested
  cargo, ONE re-seal row `{seal 21b8e001…, recordedBy 2026/09/30/NON-DESTRUCTIVE-HISTORY-EDITING, evidence {revision fe7c8a8f8bd, date
  2026-09-05}, reason, renames[7]}`. New law (`🕰️historical-json-source-encoding`): every pinned catalog is in the ledger, the live seal is
  the last recorded one, no retired name survives as a directory segment, and reversing the recorded renames on the live bytes reproduces
  the previous seal byte for byte (so nothing but those names changed). Negative check: reversing six of the seven renames gives
  `a28c9992…` ≠ `9445dea9…`.
- Two stale engine prerequisites that hid behind the member red (`🔍️discovery/🟦️.ts`, `semanticPackageProjectionAuthority`):
  (1) the wgpu row required `typescript-language` among the `🦀️rust` boundary's directory kinds, which `8add1df1473` (09-12) moved out
  (the TypeScript package sits beside the Rust package under `📦️packages/🟦️typescript`) → now requires the `typescript-language` kind to
  exist; (2) the Cargo-workspace member check threw on absent workspace evidence since the 10-01 contribution refactor (`4e36b2b5012`
  replaced a `nestedCargoField(...)?.includes` read with the throwing `cargoWorkspaceDeclaresMemberV1`) → absent or malformed evidence is
  the existing problem "WGPU is absent from the authored Cargo workspace member patterns", as before 10-01 (the authority reports, never
  reads or throws).
- Runs: `💥️nested-cargo-collision-authority` **26/0** (was 25/1); `🕰️historical-json-source-encoding` **25/0**; `🏺️historical-package-owner-identity`
  26/0; `☂️frozen-coordinate-wildcard-coverage` 5/0; `❄️frozen-markdown-coordinates` 35/1 at load 45: the scoped-transaction test completed
  every phase (empty re-plan reached) but exceeded its own 15 s budget (30 s); it passed 36/0 at 11:20 under lower load — load timing, not
  this change. `verify taxonomy report --scope …/🖼️assets/📽️nested-cargo-package-projection`: 1 pre-existing `directory-kind-unresolved`
  (the directory has no registered kind; unchanged by this WP).
- The `🧑️‍🎨️engine` root drift is closed in Resume 3 below.
- Draw producer `🔬️workspace-contract` ×6 (generator input discovery misses dynamic `import()` / `createRequire`) → S3-INFRA (coordinator).
- Files: `📚️library/🖼️assets/📽️nested-cargo-package-projection/🔣️.json` (re-sealed), `📚️library/🔣️taxonomy.json` (one seal),
  `📚️library/🧫️fixtures/🧫️frozen-seal-ledger/🔣️.json`, `📚️library/🧬️schema/🔣️frozen-seal-ledger/🔣️.json`, `📚️library/🔍️discovery/🟦️.ts`
  (two prerequisites), `📚️library/🧪️tests/🕰️historical-json-source-encoding/🟦️.ts` (catalog ledger law).

### Resume 3 (10-03 11:50) — engine root re-sealed through the ledger — DONE

- Evidence: the same commit `fe7c8a8f8bd` (2026-09-05) moved the renderer engine directory from `🧑️‍🎨️engine` (U+FE0F inside the ZWJ
  sequence) to `🧑‍🎨engine`: `git ls-tree` shows 115 tracked files under the old spelling and 0 under the new at `fe7c8a8f8bd~1`, and 0 /
  123 at `fe7c8a8f8bd`; `git log --follow` on the engine's `📋️project.json` shows the `R` rename in that commit.
- Re-seal: every catalog path segment `🧑️‍🎨️engine/` → `🧑‍🎨engine/` (127 sites; source and destination share the ancestor, so both
  layouts follow it and the catalog keeps one spelling). Seal `21b8e001…` → `0b43c775…` in `🔣️taxonomy.json` (Edit tool). The wgpu
  `semanticOwnerRoot` and `destinationRoot` exist again; 22 of the 32 wgpu destination files exist live (the other 10 moved individually
  after the projection; separate drift, renderer owner).
- Ledger: ONE more row on `catalogs.nested-cargo-packages-v1.reseals` (`seal 0b43c775…`, evidence `fe7c8a8f8bd` / 2026-09-05, reason,
  `renames [🧑️‍🎨️engine → 🧑‍🎨engine]`). The reverse-the-renames law already walks every re-seal newest first, so it now reproduces
  `21b8e001…` after the engine reversal and `9445dea9…` after both, byte for byte. Negative check: reversing only the member renames
  gives `f97b2d55…`.
- Runs: `🕰️historical-json-source-encoding` **25/0**, `💥️nested-cargo-collision-authority` **26/0**, `🖼️runtime-taxonomy-asset-paths` 1/0,
  `🔬️workspace-contract -t "separates current paths from immutable evidence"` 1/0.
- Note: `📚️library/🧫️fixtures/🦀️nested-cargo-package-authority` and `💎️nested-cargo-package-purity` still spell `🧑️‍🎨️engine`, but no test
  or engine reads them (they are referenced only by each other and by the sealed `🧼️remaining-package-purity-authority` census) →
  candidates for retirement by their owner. `🔏️path-emoji-statutes` keeps the old spelling on purpose (statute vector).
- Files: `📚️library/🖼️assets/📽️nested-cargo-package-projection/🔣️.json`, `📚️library/🔣️taxonomy.json`, `📚️library/🧫️fixtures/🧫️frozen-seal-ledger/🔣️.json`.

## Session 4 — 2026-10-04

Continued by S4-GATES in `📓️s4-gates-report.md` (one report for the four inherited WPs, rule 34).
