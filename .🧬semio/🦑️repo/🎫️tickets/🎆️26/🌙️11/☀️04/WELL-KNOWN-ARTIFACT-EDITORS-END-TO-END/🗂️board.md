# 🗂️ Task Board: Well-Known Artifact Editors End to End

Coordinator: main chat (claude-opus-5-5, medium). Ticket opened manually because the `repo` MCP server failed to connect (`repo mcp needs the full native command ... repo run repo-mcp:install`); install running as coordinator background task.

Provisional scope: the 40 well-known format artifacts of `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts` (plugin AGENTS.md: "artifacts are well-known file formats"). To be confirmed by T15 inventory.

## Batches (provisional, from stdio artifact directories)

| Batch | Worker | Artifacts |
| --- | --- | --- |
| A | sonnet-editors-a | txt, md, json, 🔣️.json, xml, html |
| B | sonnet-editors-b | csv, tsv, xlsx, docx, pptx, idml |
| C | sonnet-editors-c | png, jpg, bmp, gif, tiff |
| D | sonnet-editors-d | svg, dxf, dwg, pdf |
| E | sonnet-editors-e | obj, stl, ply, gltf, las, step |
| F | sonnet-editors-f | ifc, bcf, idf, epw, inventory |
| G | sonnet-editors-g | mp3, wav, mp4, avi |
| H | sonnet-editors-h | zip (has ✏️editor), deflate, binary, semio |

## Tasks

| ID | Worker | Assignment | Depends | Status |
| --- | --- | --- | --- | --- |
| T01 | sonnet-registry | Canonical artifact registry, MIME/extension/alias precedence, editor resolution | T15 | dispatched |
| T02 | sonnet-host | OS app editor host, open/create dispatch, lazy load, disposal | T01 | dispatched |
| T03 | sonnet-persistence | Document model, save ack, history, dirty, stale-load guards | T02 | dispatched |
| T04 | sonnet-formats | Shared import/export, codec routing, round trips | T01 | dispatched |
| T05 | sonnet-runtime | Toolchain (bun missing from PATH), deps/lockfiles, app startup, services, WASM | — | dispatched |
| T06 | sonnet-validation | Heavy-job queue owner, e2e harness, per-editor acceptance evidence | T05 | dispatched |
| T07–T14 | sonnet-editors-a..h | Batch A–H editors | T01–T05 | dispatched |
| T15 | haiku-inventory | Coverage matrix from registries | — | dispatched |
| T16 | haiku-capabilities | Capability contract per editor | — | dispatched |
| T17 | haiku-data-audit | Persistence/format/data-loss audit | — | dispatched |
| T18 | haiku-entry-audit | App entry points audit | — | dispatched |
| T19 | haiku-registration | Bounded registry/manifest/fixture-ref updates | T15 | dispatched |
| T20 | haiku-documentation | Commands + handoff docs, verified | T05 | dispatched |

## Concurrency log

(Observed values appended by coordinator.)
- 2026-11-04 initial wave: ListAgents observed 20/20 subagents `running` at the same time (6 haiku, 14 sonnet profiles). Configured models come from agents.json; runtime model per worker is not independently verifiable from ListAgents (it shows profile names only).
- repo-mcp:install failed ("Native installation refuses a non-file destination"); reassigned to T05.
- Wave 1 hand-backs: T17 (data audit → 🔍️data-audit.md), T02 (host contract, F1 fix), T18 (entry audit → 🚪️entry-audit.md), T19 (registration audit; json directory fix).
- Refills (resumed same profiles): T17b per-codec preservation audit; T02b F3 publish-after-open + wgpu D1/D2 + pins D4; T18b non-dashboard startup + deep links; T19b fixture manifest. T03 got prioritized persistence defects; T04 export naming/envelope defects; T01 registration ambiguities A1–A8; batches A–H retargeted to hub family surfaces.
- Scope correction: editors = per-subset #editor/#viewer surfaces in 🌎️hub/🧩️compositions/🗄️stdio (+ 9 family extensions). 38 artifacts have definitions; 📇️inventory is a library (no definition). idf + idml lack surfaces (assigned to F, B).
- 06:51 resources: load 20.3 on 12 cores, 7.4 GB free of 15.8 GB, `repo capacity` gate 4 running / 7 waiting, pressure normal. Gate acts as the shared heavy queue; no extra throttle added.
- Wave 2: T15 (inventory → 📊️coverage-matrix.md), T13 (mp4/avi window fix, ffprobe tests, unverified), T12 (interim; idf decision: definition-only → energy model editor). Gap work distributed: NF codecs (C,D,E,F,H), native receipts (C,F,G,H), ifc 4x3 orphan (F), patch-frames/trim (G). T15b: NF+receipt recipe.
- Wave 3: T16 (capability audit → 🎟️capability-audit.md: 17 editors with RO edit windows), T17b (per-codec preservation, appended to data-audit), T18b (non-dashboard start; dashboard `-dev` target bug; no deep links/MCP open), T15b (recipe → 📗️recipe-natural-file-and-native-receipt.md), T10 interim (dxf 2D render + NF, dwg NF, pdf1.7 subsets NF; uncompiled).
- Decisions: pdf1.4 cross-crate dep approved; DWG entity editing in scope for D; dxf viewer must render; build rule `--store=shared` only (two workers used isolated → framework rebuilds).
- Refills: T16b Details-pane editing truth for RO editors; T17c browser/runtime prerequisites; T18c UI selector map; T15c compliance review of first edits.
- Wave 4 hand-backs: T01 (resolver + TS twin, A1 codec-key contract), T02 (F3/D1/D2/D3/D4 + Open-file routing; 55 vitest green via ticket config), T04 (export names, envelope version, wrap_text, import overwrite, file-space port; uncompiled), T10 (dxf/dwg/pdf NF), T12 (idf→energy NF, epw/bcf/ifc NF, BCF fixtures), T14 (zip-slip, zip/deflate/binary NF), T15c review (mp3 compile error, ac1018 model bug, pdf format_kind dup), T16b (Details path exists for all 17 RO editors), T17c (browser prereqs), T18c (selectors + a11y defects), T19b (fixture manifest), T20 (commands.md).
- Decisions: Details split layout default for RO-main-window editors (T19 applying); subset-qualified format_kind for pdf 1.7 / ifc 2x3; NF extension aliases approved (T01); .semio NF dispatcher → T04; duplicate TS resolver twin removed in favor of T01's; refusal texts show all languages for unknown locale (no default language).
- Blocker trend: all Rust changes uncompiled; capacity gate is the bottleneck (5–11 queued). T06 asked to batch -p runs. Third-party tools on host: ffprobe, qpdf, pdfinfo present; ezdxf absent.
- 07:03 resources: load 30.8, capacity gate 5 running / 13 waiting (each cargo granted 1 job), 6.8 GB free. Queue rule issued: ≤1 queued cargo invocation per worker, multi -p. Batch F had 5 separate runs.
- Wave 5: T15d review (semio envelope parse bugs, space-catalog pointer-keyed leak, resolver MIME-before-extension abort, zip drive-colon), T16c edit-rule recipe (📙️recipe-edit-rules.md), T17d T03 verdict (decode-failure overwrite, retry clobber, no ES), T18d Steam Deck audit (🎮️steamdeck-usability-audit.md), T19c details-split applied to 29 files + 54 invalid fixtures, T20b matrix refresh, T13 (ffprobe hard-fail, media_transport_view shared, wav cut-frames, wav native codec placeholder hash), T10 (dxf/dwg 2D preview, pdf1.4 lossless-only open, subset format kinds), T01 (formatCatalog generated + wired).
- Decisions: idf→energy stands (energy AGENTS "no IDF" is about engine API; IDF io already tracked in energy model). pdf owner = 1.7 base (declared owner rule). mp3 new kinds via schema generators, not hand-written generated files.
- T05 runtime and T06 validation: no status files after ~40 min; pinged.
- 07:06 resources: load 19.9 (falling), swap 8924/8947 MB (full), RAM 7.2 GB available, gate 4 running / 8 waiting. Each stdio artifact is its own Cargo workspace → runs must be per-artifact dir, chained.
- Wave 6: T17e save-status trace (CRITICAL: "Saved" before ack via poll race; close silently drops unsaved/in-flight writes) → T03 + T02. T18e media/document/table audit → Tab trap + Details tree keys (T02), media trim/cancel/mm:ss (T13). T19d fixture-enumerator breaks fixed (ifc2x3 walker relocation; 54 renames). T01 owner rule declared (pdf 1.7, gif 89a, dwg ac1024, ifc 4) + content-sniff assigned. T09 bmp/gif native + gif/tiff NF + paint wiring; decisions jpg paint yes, gif SetPixelRegion, bmp palette mutations. T12 ifc 4x3 viewer+registration, cobie/sav/cv20 → domain enums. T14 drive-colon fix, deflate doc fix.
- Fixture count conflict in natural-file-lifecycle (33/51/55/67) → set-based (T10), reconciliation owner T06.
- T05/T06 messages queue undelivered → both appear blocked in long tool calls.
- 07:20 observed: `repo capacity run -- repo run @semio-tech/framework-os-dev:activate-s-react-prototype` running 12m47s (dependency workspace-edge resolution, log 🗑️generated/activate-s.log) — T05/T06 blocked inside it, not stalled. vitest running via ticket config (T02/T13).
- Wave 7: T17f stale-async (archive closure → T03; runtime key + single-flight import → T02). T18f image/pdf/mesh a11y (keyboard paint → T09, PDF/UA text layer → T10, mesh keyboard + GL release → T11, duplicate Tools labels → T19). T01 content-sniff done (gif/dwg/ifc; pdf→1.7 always) + a11y schema enforcement assigned. T20 crate-names.md. T19 invalid-fixture builder merged; /tmp logs copied.
- Load balancing: a11y (a) moved from T02 → T01; media host → T13; Paint2dHost → T09.
- Evidence so far (logged): acceptance contract 192/194 (idml only), os-io format_resolution 5/5 EXIT 0, host vitest 134/134 rc=0 (11 files), registry vitest 4/4, media vitest 20/20 + 10/10, shell-labels vitest 151/151, selection-focus 53/53. DXF first full dependency compile completed; 11 test-only errors fixed, rerun queued.
- Wave 8 reviews: T04 semio second-open panic (fixed via OnceLock), DWG duplicate test fns + add/delete→rename data bug, batch G cut-frames guard bypass + grammar scaffolds + cancel race, batch C duplicate action registration + wav placeholder hash, batch E fake stl cross-check + missing .glb + no GL release, batch A html destructive edit + unreachable i-json mutations, scaffold sweep (🧱️grammar-scaffold-sweep.md).
- T03 re-audit: decode-failure guard + native flush-on-close fixed; remaining partials routed.
- 07:24 resources: load 52 (peak), RAM 5.5 GB available, swap 8947/8947 MB full, capacity gate 7 running / 7 waiting. Non-fleet consumers observed and left alone (not task-owned): a python3 process at ~96% CPU running 2d14h, a cursor-agent session, and a separate `claude -p … agents.json` session (44 min). Our fleet's load is bounded by the repo capacity gate.
- Wave 9: T05 reported — repo-mcp install FIXED (symlink launcher handled; `repo mcp` answers initialize with 55 tools; Claude Code restart needed to reconnect), dashboard `-dev`→`-prototype` and hub secure-suite fixes, Playwright chromium 1248 present with no missing libs (no host blocker), bun via `repo tools exec bun`. Activation blocked by dependency-generation churn ("Bun current ownership changed"); DEPENDENCY FREEZE declared fleet-wide, T05 investigating `✏️s/bun.lock` rewrite (06:36) and concurrent repo processes.
- T06 pass-2 chain queued (generators → 38 per-workspace `test --workspace` → contract). T01: unnamed-control law + scan (14→0), rejection reason surfaced; decisions: localized accessibility labels, multi-format codecs after first compile wave (opened format = document metadata). T14 hex-splice + drawing Python oracles, node-edit pointer hardening (25 cases). T08 idml story crate + surface + sample.
- Incident: T13 ran `pkill -f "repo cargo"` while restarting its own run; batch H chain killed (requeue ordered). Fleet rule added: kill only self-started recorded PIDs. Surviving chains verified via pgrep (A txt, B idml, C batch-c, D dxf, E batch-e, F bcf/epw/ifc, G batch-g, T01 ui-validation, T03 store-sync, T04 os-semio, T06 generator + pass2).
- Wave 10: T13 media contract (trim/cancelled/plurals) + mp3/mp4/avi cut-frames via transport (vitest 27+12 green). T09 keyboard paint (12 vitest), jpg/gif paint, bmp real demo. T01 localized-label authoring law (TS 7 green; 0 gaps). Deferred to post-first-compile-wave (one pass each): multi-format natural codecs (T01, 81 files), typed localized MediaError (T04, 222 producers).
- Evidence: semio crate `semio-framework-os-semio` 12/12 passed (T04; rerun queued to cover the SemioFormat token edit). T01 component vitest 24/24 (Table nested-control fix, Panel fit, export barrel). T02 host vitest 152/152. Load peaked at 61 with swap exhausted.
- Wave 11: T08 idml wiring authored (census red until SQLite owner — owner + native protocol assigned to T08); T11 real geometry for all 6 mesh formats, Python reference cross-checks, oracle crate deps queued for T05's controlled dependency pass; T13 set/remove-id3-frame + grammar sweep + grammar-parse tests; T09 bmp palette oracle via `image` crate; wgpu media Cut channel → T02.
- Root cause of build starvation (T05): each artifact workspace has its own CARGO_TARGET_DIR → framework crates rebuilt per workspace (2.7–4.4 GB each); every cargo `-j1`; 12 cores at load ~51 (oversubscribed by concurrent builds, incl. non-fleet sessions). Lane hashing/gate policy is repo-core and left unchanged.
- Activation blocker: T06's composition-prepare (PID 939199) holds the exclusive JS dependency lease (~48 min). Sequencing: after it exits, T05 takes the lease → `dependencies ensure` → activation → dev server :6070 → batch-E oracle dep pass; then T06 continues generators. Storybook 10.6.2→10.6.5 install happened 06:20–06:39, before the freeze.
- T07 batch A: html reversible splice-source, i-json verbs reachable, BOM/whitespace tests with serde_json/html5ever cross-checks (uncompiled; first txt run hit 50-min timeout in cold framework build).
- 08:12 resources: load 77, RAM 2.7 GB available (-600 MB below gate floor), swap 8947/8947 full, gate 11 running / 4 waiting. Memory-pressure hold issued: no new ungated (vitest/node) runs in parallel; one cargo invocation per worker.
