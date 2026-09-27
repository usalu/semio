# WP-R9 — AGENTS.md Compliance of the Runtime Paths

Slice: R9 (session 13). Source: `📓️audit-s13-rules.md`. Ports 8170–8179 / 6670–6679. Private cargo: `.tmp-ticket/wp-r9/target`.
Durable captures: `.🧬semio/🌐hub/s13-r9-captures/`. Expendable captures: `.tmp-ticket/wp-r9/generated/`.

## Session 13

| # | Item | State | Evidence |
|---|---|---|---|
| 1 | launch.json registration: executable-command rule, generator + law | **DONE** (manifest-input freshness = prepared discovery patch, not landed in window 2 — see §Item 1): 2 417 declared targets / 439 projects → 0 unregistered; launch.json 1 397 configs, 29 family pickers, VS Code-loadable; laws 14/14 | §Item 1 |
| 2 | `[DEBUG]`-tagged permanent status lines (hub `📜️script.ts`) | **DONE** — hub `📜️script.ts` 26 → 0 tags (status lines kept, tag dropped; parsed payload channel renamed on both ends); 56 temporary test prints deleted in 15 hub files; hub+MCP native check 0 errors | §log 21:0x, `native-check-7.txt` |
| 3 | CRUD `DELETE` hub auth routes → commands (hub + all clients) | **DONE** — `POST /auth/sessions/me/sign-out` + `POST /auth/agent-delegations/{id}/revoke`; native GREEN: kernel client (10:07), hub + os-mcp incl. H9's fence laws (10:2x, 0 errors); TS laws 98/98 + relay 4/4; hub bin-unit laws not run by me (rule 25 — H11 re-runs) | §Item 3, `native-check-6/7.txt` |
| 4 | `🔌️plugin/🦀️.rs` fallback / weak-linkage shim | **LANDED** — forbidden implicit 2nd init path removed; native GREEN 10:07 (plugin + kernel lib+tests); wasm32 = REBUILD gate | §Item 4, landing row |
| 5 | `@emoji` docstring tag investigation (report only) | **DONE** — residue, not a convention; codemod + law recommended | §Item 5 |
| 7 | (coordinator 21:0x) T13 repo-contract HIGHs: ui `event-feed-time-format` inline test, os schema-oracle JSON in `🧪️tests` | **DONE** — both moved (ui module since renamed `🕰️host-temporal-format` by a peer, layout kept) | §log |
| 6 | comments inside definitions: parser count + clean hub/MCP; codemod plan for guest-linked | **DONE** — hub + MCP: 221 blocks hoisted into docstrings, tree-sitter census 0 left, native check 0 errors; guest-linked plan: 10 296 blocks / 1 919 files, codemod dry-run clean (§Item 6) | §Item 6 |
| 8 | (coordinator 05:3x) `framework-os:typecheck` 64 errors → 0 (+ the other TS programs) | **DONE**: framework-os rc=0, framework rc=0, os-hub-ts rc=0, ui-react 0, renderer-react 0 (10:5x); touched laws green | `generated/fos-typecheck-*.txt`, `tc-*-8.txt` |
| 9 | (coordinator 06:3x) `verify dependencies literal-external` audit | classifier root fix + 4 manifest moves: **oracle conflicts 20 → 6, production-reachable 100 → 84**; literal-external stays 247 (zero-target counts every third-party row incl. tooling/tests); remaining runtime classes + plan in §Item 9 | `s13-r9-captures/deps-summary-{1..4}.json` |
| 10 | (coordinator 11:0x) repo-lib typecheck (6 errors: workspace-contract law imported the Neo4j graph-export helpers deleted with the feature in 09-26 15:13) | **DONE** — the law's `Neo4j graph database registry` block + import removed with the feature; repo-lib tsc rc=0; workspace-contract quick 604 pass / 5 fail → owned-generator fixture gained W3's `mutation-source-authority` (law 1/1); 4 left need the repo harness's active-ticket artifact dir | `tc-repo-lib-10.txt`, `workspace-contract-quick-3.txt` |

### Session 13 log

- 19:36 slice start; read AGENTS.md, preambles 13/12, `audit-s13-rules.md`, `wp-r8.md` (launch resolver probe P1-6).
- 19:4x H11 answered "go now" for item 3 (not editing those handlers; asked to move its H9 revocation-fence laws and the
  two-client e2e too, fence byte-identical). V1 asked which launch mechanism to use → told: seed row + identical rendered row for
  now; R9 converts every seed row itself if the source of truth moves.
- 20:0x item 3 edits (see §Item 3); 20:15 `cargo check -p semio-hub --lib --bins --tests` started (queued behind a peer's hub check).
- 20:2x item 5 measured (see §Item 5).
- 20:3x rule 25: my `cargo check -p semio-hub` waited 28 min with no rustc child (convoy) → stopped (pid 62168).
- 20:4x TS side of item 3 measured: relay admission law **4/4** (`bun test`), `hub-sign-in-spaces-check` **EXIT 0 — oracle 15 checks
  clean (incl. the new sign-out-path pin), 3 files / 98 tests pass** (`generated/hub-sign-in-spaces-check-1.txt`).
- 20:53 item 4 applied (`wp-r9/plugin-link-shim-removal.py`, dry run clean first). 20:54 combined native check launched on the
  shared build-dir → convoy (8 min, no rustc child) → stopped at 21:03 per rule 25/26; relaunched on `build-fleet-b`
  (pid 5396, capture `.🧬semio/🌐hub/s13-r9-captures/native-check-2.txt`).
- 21:0x item 2 applied (`wp-r9/debug-residue-removal.py`): 56 test prints deleted in 15 hub files, 26 hub `📜️script.ts` tags dropped,
  the `native-catalog-payload=` channel renamed on both ends. Item 7 (coordinator) applied. Item 6: tree-sitter census + codemod
  (`wp-r9/comment-census.ts`, `wp-r9/comment-hoist.ts`) applied to hub + MCP: 221 blocks.
- 21:1x W3 relay: registry launch law finds no `workspace:generate` row (removed by the 15:13 auto-commit together with
  `purge`) → `🏭️generate` row restored in seed + launch.json at its old order −49.8 (root verb `generate` exists); the law's
  lifecycle list drops `purge` (no root `purge` verb exists any more). Both files parse (Bun.JSONC).
- ~21:30 cut by the account usage limit (preamble rule 28); every process of mine died (native check pid 5396).
- 05:00 (09-27) resume + reconcile: all my edits are in the tree (auto-commit 40a2736e66 22:00). A peer renamed the ui module
  `🕰️event-feed-time-format` → `🕰️host-temporal-format` and kept my canonical test layout (`🧪️tests/🕰️host-temporal-format/🦀️.rs`).
  Shim removal present (0 `linkage`, `install_guest_panic_report` ×3); auth routes present; hub script 0 `[DEBUG]`; seed 433 /
  launch.json 658 configs, both parse, `workspace:generate` present. 05:04 native check relaunched on `build-fleet-b`
  (pid 93815, capture `native-check-3.txt`); wasm32 script `wp-r9/r9-wasm-check.sh` prepared for right after it.
- 05:2x roll-call sent to main. 05:39 combined check replaced by a guest-critical one (plugin SDK, kernel, ui-contract, playbook,
  flow-extension-primitive; `--lib --tests`, pid 23531, `native-check-4.txt`) — build-fleet-b convoy (0 rustc children for
  minutes at a time; 40 rustc, load 113). `wp-w3/requests/r9.txt` written (6 crates + reasons).
- 05:3x item 8 (coordinator): framework-os typecheck run 1 **rc=2, 64 errors / 22 files** (359 s). Root fixes (TS only):
  wgpu `♿️accessibility-mirror` element typed `HTMLElement` (union lost the `keydown` overload → 9× `key` on `Event`);
  `🧩️dynamic-extension` door host is a named type + `declare global` for its two globals (weak-type TS2559 in `🎞️frame-worker`
  and `🎬️renderer-boot`); host-io picked package is `Uint8Array<ArrayBuffer>` (BodyInit); os `tsconfig.json` maps
  `dom-accessibility-api` to its declarations (same as the framework program; 2 new laws import it); `♿️retained-toggle-semantics`
  fixture rows typed as the toggle variant + a full `StyleSpec` (no `as UiNodeRecord`); `🧩️wgpu-extension-store-door` fetch mock
  typed; `🌳️wgpu-document-reconcile` binding args typed `UiValue` + the now-required intent `seq`. S17 fixed its playground
  session twin on request. **Run 2: rc=2, 46 errors** (123 s, `fos-typecheck-2.txt`).
- ~06:35 cut again (usage limit); every process died, incl. my build-landing check (pid 50480, still in third-party crates).
- 09:55 resume (rule 30: REBUILD START 09:53, chain b3). Landing row written (`📓️landing.md`, R9): native plugin+kernel check
  queued in the `native` lane (pid 71026) — earlier runs 3/4/5 never reached our crates (convoys/cut); wasm32 = REBUILD gate.
  `ui-contract-rs:generate` queued behind it in the same lane (pid 71763).
- 10:0x item 8 continued (all remaining errors were in files idle since ≤ 05:27; I fixed them at the root): TreeItem literals
  gain `inlineToolbar: null, detail: null` (engine-contract ×5, window-kits tree — the Rust props added both); engine-contract
  text-editor mocks typed as `ActionDescriptor`, paste/compose step reader narrowed per kind, `UiComponentSceneNode` imported,
  locale restore via `detectShellLocale`; block-list law guards its fixture locale with `isShellLocale`; `common.delete`
  label added schema-first (I18n type + en "Delete" / de "Löschen") for `🧩️BlockListHost`; `🪟️maximize-resync` descriptors carry
  `iconId` + the ingress law passes `activeWindowId={null}`; puzzle3d settings law passes `onIntent` and `UiLabel`/children props;
  command-panel/surface-document optionality; slider a11y value built from the declarative control (`unit ?? null`);
  duplicated `headerToolbar` key removed (typed-wire law); stdio kit TS twin: `SetSnapshot` in the `SemioKitMutation` union +
  its parse branch (`parseSemioKitSnapshot`), replacing the mis-anchored codemod. **Run 5: 12 errors, all NumberStepper min/max.**
  Laws of the touched files: renderer-react 6 files **46/46**, wgpu config 2 files **16/16**, engine-contract run 1 **16 red**
  (runtime: `TextEditorHost` now calls `session.setCaretVisible` — caret cadence — and the six fake sessions lacked it) → fakes
  gain `setCaretVisible: vi.fn()` → **704/704**; vitest still exits 1 on 3 unhandled `WebSocket ws://localhost:3000/semio-stream-mux`
  errors (present before my edits: run 1 had them too) → F2's stream-mux opens a real socket under jsdom (routed).

## Item 3 — hub auth `DELETE` routes → commands

Design: queries stay `GET`, every state change is a `POST` command on the resource (the hub's existing `…/cancel`,
`…/redeem`, `…/approval` shape). The two `DELETE` routes become:

| was | now | handler |
|---|---|---|
| `DELETE /auth/sessions/me` | `POST /auth/sessions/me/sign-out` (empty body, `DefaultBodyLimit::max(0)`, non-empty → 400) | `post_session_sign_out` |
| `DELETE /auth/agent-delegations/{id}` | `POST /auth/agent-delegations/{id}/revoke` (empty body, non-empty → `malformed-request`) | `post_agent_delegation_revoke` |

Both still append their facts (`session-revoked`, `agent-delegation-revoked`) exactly as before; the delegation fence (hold every
revoked session's binding exclusively before invalidating grants/answering) is byte-identical (only a body check was added in
front). No hub route answers `DELETE` any more (a law asserts `DELETE /auth/sessions/me` → 405), CORS `allow-methods` drops
`DELETE`, the `HttpMethod::Delete` variant is gone from the kernel directory client (queries `GET`, commands `POST`).

Files: hub `🔐️auth/🦀️.rs` (`SESSION_SIGN_OUT_ROUTE`), `🔐️auth/🤖️agent/🦀️.rs` (`AGENT_DELEGATION_REVOKE_ROUTE`, module doc),
`🔐️auth/🧬️schema/🟦️.ts` (`AUTH_SESSION_SIGN_OUT_ROUTE`), `🏗️bootstrap/🦀️.rs` (handlers, route table, rate-limit class, CORS),
`🧪️tests/🔬️bin-unit/🦀️.rs` (13 direct calls + 7 raw HTTP calls incl. H9's three revocation-fence laws, +405 law),
`🧫️fixtures/🚧️hostile-input-v1/🔣️.json` (2 rows, empty-body vectors), `📇️directory/🧫️fixtures/🚻️space-journey-v1/🔣️.json`
(self-revoke step + route inventory), `📦️packages/🦀️rust/📜️script.ts` (4 probes, journey method taxonomy, relay body read),
`🔐️auth/🧪️tests/🤝️live-sign-in/🟦️.ts` (+405 check), `🧪️tests/🤝️two-client-document/🟦️.ts`, `🚀️local-relay/🧭️routing/🟦️.ts`
+ its admission law. Clients: kernel `📇️directory/🔌️client/🦀️.rs` (`sign_out` → POST command; native wgpu shell uses it),
wgpu `📇️directory-door/🦀️.rs` + its law, wgpu `🔐️HubSignIn` (`HUB_SESSION_SIGN_OUT_PATH_V1` + fixture law), React
`🔗️HubConnection/🟦️.tsx` (sign-out + revoke), `📇️directory/🔐️sign-in/🟦️.ts` (`HUB_SESSION_SIGN_OUT_PATH_V1`, re-exported from
`💻️os/🟦️.ts`), sign-in contract fixture + schema (`signOutPath`, pinned by the TS law, the wgpu law and the hub-auth oracle in the
React `📜️script.ts`), `📇️directory/🤖️delegations/🟦️.ts` (`…/revoke`) + AgentDelegations law, MCP laws
`🤖️hub-agent-participant`, `🤝️hub-edit-durability`. The MCP Rust side has no revoke/sign-out call (verified by grep).

## Item 5 — the `@emoji` docstring tag

Verdict: **residue of an authoring style, not a machine-readable convention.** Recommend a codemod (strip the tag) + a law.

Evidence (all measured):
- **No consumer.** Every occurrence of `@emoji` in the repo's code (224 057 source files scanned, 9 188 hits,
  `.🧬semio/🌐hub/s13-r9-captures/at-emoji-all-1.txt`) is inside a comment/docstring except (a) three generators that EMIT it into generated files
  (`🖱️ui/🎨️styling/📽️projection/🟦️.ts` ×2, `📇️registry/📽️projection/🟦️.ts` ×3, wgpu `🏗️builder/🦀️.rs`) and (b) two
  `indexOf("/** @emoji 🛂️ …")` / `indexOf("/// @emoji 🧬️ …")` text anchors in `🌎️hub/…/📜️script.ts:13126` and
  `💻️os/🖥️host/…/📜️script.ts:1024`. No parser, taxonomy law, docs extractor or uniqueness check reads the tag; nothing checks
  docstring emojis at all. AGENTS.md (then and now) says "start with a unique emoji" — never `@emoji`.
- **Not introduced by the 09-02 rename.** 21fbcd3538f only moved files: its parent already had the same lines (e.g.
  `🏪️store/🦀️component.rs:1761`). Per-revision history of that file: `@emoji` present from its creation (2026-08-06, 183 → 377).
  Tree-wide bisect (first-parent, `.rs/.ts/.tsx`): 0 at 7e243651ba (≤ 2026-05-01) → **287 at 17c93d6fe6 ("2026-04-16", kit
  JS/React `semio/js/index.ts`)**, 5 870 by mid-June, 8 901 by August — an agent JSDoc habit that spread by copy.
- **Minority, mixed style.** Docstring openings today (`at-emoji-census.py`, first line of each `///` block / `/** */`):
  Rust 71 336 bare emoji vs **4 093 `@emoji`** (1 002 other), TS 18 462 bare vs **4 664 `@emoji`** (2 909 other) → 8.9 % of
  emoji docstrings; 741 files carry it, 385 of them mix both styles. Top: `🏪️store/🦀️.rs` 411 (+279 bare), ui React 403,
  spatial-kernel geometry 218, `🛂️manifest/🦀️.rs` 195, `🔌️plugin/🦀️.rs` 177 (+1 361 bare), hub bootstrap 161 (+90 bare).
- **It breaks the language-native docstring in TypeScript.** TypeScript's own compiler (5.9.3, `at-emoji-jsdoc-probe.ts`)
  parses `/** @emoji 🧹️ Tagged summary. */` as documentation `""` + block tag `emoji` = "🧹️ Tagged summary." — IDE hover,
  quick info and TypeDoc show NO summary for 4 664 TS definitions. `/** 🧹️ Plain summary. */` → documentation "🧹️ Plain summary.".
  Rustdoc renders the literal "@emoji" as the first word of the summary line.

Recommendation (next cycle, after PUBLISH DONE — it touches guest-linked crates, 741 files): one codemod in a ticket folder that
rewrites only a docstring's first token `@emoji ` (Rust `///`/`//!`, TS `/** @emoji`, `* @emoji` first content line, the
generators' templates and the two `indexOf` anchors — anchors first, or those laws break), dry-run diff reviewed per file
(memory: codemod incidents on emoji-heavy files), compile-atomic per crate; plus a repo-lib law "a docstring's first token is an
emoji, never `@`" (and, optionally, per-file uniqueness) so it cannot regrow.

## Item 8 — framework-os typecheck triage (run 2: 46 errors)

Attribution: our fleet was cut ~21:30 and resumed ~04:50 (rule 28); every file edited between those times is a non-fleet
actor's — the two Codex GPT teams with open tickets `26/09/26/COMPLETE-STDIO-ARTIFACT-EDITING-EXPERIENCE` (stdio editors, incl.
"media-spatial-patch-snapshot-rollout") and `26/09/26/COMPLETE-DRAW-VECTOR-EDITING-EXPERIENCE`, or a human dev.

| errors | file(s) | root cause | owner / action |
|---|---|---|---|
| 12 | ui `♿️accessibility` (2), `🧵️retained/🛡️validation/🔬️graph` (8), `🗣️Interpreter` 1435/1436 (2) | `NumberStepperProps.min/max` exist in the Rust schema metadata but the gitignored TS mirror `🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts` (00:12) is stale (the ONLY differing type, measured) | run `@semio-tech/ui-contract-rs:generate` (a `cargo test --features typegen` build → W3's rebuild chain / native mutex after 07:00) |
| 6 | engine-contract treeItem ×5, window-kits `🌳️tree` | `TreeItemProps` gained `inlineToolbar` + `detail` (Rust, in the 22:00 auto-commit); literals lack them; tree also got `headerToolbar` at 00:27 | non-fleet (00:27 edit) — whoever owns the toolbar rows; literals need `inlineToolbar: null, detail: null` |
| 10 | engine-contract (rest: tuple `[]` index, possibly-undefined, `UiComponentSceneNode` missing, locale `string` vs `"en"|"de"`, paste/compose index) | test file edited 05:27 (in flight now) | active editor of `🔬️engine-contract` (unknown — not R9) |
| 4 | stdio `🧿️semio/…/🧰️kit/🧬️mutations/🟦️.ts` | a mis-anchored codemod (00:36, uncommitted) appended `\| { SetSnapshot: SetSnapshot }` to the parser's last `return` instead of the `SemioKitMutation` union (Rust already has `SetSnapshot`) | Codex stdio team |
| 3 | `⚙️puzzle3d-settings-document` law (23:13) | overloads (render/props) | non-fleet |
| 4 | `🧩️BlockListHost` + `🧩️block-list-presentation` (01:41) | overload + locale `string` vs `"en"\|"de"` | Codex draw/stdio |
| 2 | `🪟️maximize-resync` (00:14) | `ModeWindowDescriptor` shape | non-fleet |
| 1 | `🗣️Interpreter` 1163 (01:21) | slider control passed where `Component` is expected | non-fleet |
| 1 each | `🎛️command-panel` (00:14), `📄️surface-document` (01:12), `typednodefields…` duplicate key (00:27) | optionality / duplicate property | non-fleet |
| 1 | `🌳️wgpu-document-reconcile` | fixed after run 2 (`seq`) | R9 (verify in run 3) |

- 10:07 **native lane check GREEN**: `cargo check -p semio-framework-plugin -p semio-framework-os-kernel --lib --tests` (build-fleet-b,
  `native-check-6.txt`): `Finished`, 0 errors, 453 warning lines — the SDK shim removal and the kernel directory-client command
  compile natively; landing row updated. Queued next in the lane: `ui-contract-rs:generate` (pid 71763), `cargo check -p semio-hub
  -p semio-framework-os-mcp --lib --bins --tests` (pid 83360). 10:07 dependency truth summary (JSON) running (pid 83212).

- 10:1x–10:3x item 9 (dependency audit): baseline census (`deps-summary-1.json`, 10:10): literal-external 247, production-reachable
  100, oracle conflicts 20, toolchain-owner conflicts 2. Root cause of 11 false conflicts: the classifier's test domain was a
  hardcoded regex (`🧪️test|🧪️tests|🧫️fixtures`), so manifests under the taxonomy's own test-domain directories `🔬️probes`,
  `🏭️generator`, `📚️examples`, `🔮️oracles` counted as production. Fix (repo tooling TS): `dependencyTestDomain(taxonomy)` reads
  every test directory name + `testDomainPath` from the taxonomy; oracle classification and a new rule (a manifest inside the test
  domain never ships → its runtime/build sections are `test-runner`) use it; self-test gains 7 hostile ownership rows (probe,
  generator, oracles, domain module owned; `🏭️bridge`, `🧪️oracle`, `🧪️test-copy` not) — `verify dependencies self-test` clean;
  test-platform ownership laws 4/4 (`bun test -t`). → conflicts 9, prod-reachable 89. Then manifests: MCP TS package
  `@modelcontextprotocol/sdk` + `ajv` → devDependencies (only 🧪️tests and independent oracles import them), mathematical + lowpoly
  `ajv` → devDependencies (tests / 📜️script.ts only), ui-react `dagre` removed (no importer in ui; only a print law uses it, print
  declares it), `bun.lock` workspace snapshots edited to match (parity js: lock-mismatches 0). → **conflicts 6, prod-reachable 84**
  (`deps-summary-4.json`). The ratchet `verify dependencies` reports 13 NEW deps added by peers (accesskit*, hayro, json-patch,
  naga, pngjs, picomatch, @types/*) — baseline write is the coordinating session's call (not run by me).
- 10:2x native lane: `ui-contract-rs:generate` refreshed the TS mirror; `cargo check -p semio-hub -p semio-framework-os-mcp --lib
  --bins --tests` **Finished, 0 errors, 571 warning lines** (item 3 hub side + MCP comment hoist compile; `native-check-7.txt`).
- 10:3x typecheck: framework-os **rc=0** (after 4 numberStepper literals got `min/max: null` for the regenerated mirror),
  ui-react 0, renderer-react 0; framework 8 and os-hub-ts 5 errors are outside framework-os (see table row 8).

## Item 9 — dependency audit (literal-external)

Measured (`bun ./📜️script.ts verify dependencies summary --format json`): before 247 literal-external / 100 production-reachable /
20 oracle conflicts → after **247 / 84 / 6**. The zero-target counts every third-party row (tests, tooling, Python oracles
included), so 247 only falls when a library is replaced by an in-repo implementation — not by reclassification.

Production-reachable 84, by class (`generated/deps-prod-classification.txt`, 86-row snapshot before the last fix):
| class | rows | verdict |
|---|---|---|
| platform bindings (wasm-bindgen/js-sys/web-sys, wgpu, winit, raw-window-handle, ash, windows, objc2-metal, accesskit*, arboard, rfd, libc, getrandom, wasmtime(-wasi), wit-bindgen) | 18 | OS/GPU/browser/wasm ABI — AGENTS.md "system libraries provided by the frameworks"; keep behind the renderer/host interfaces |
| hub/native server + db drivers (tokio, tokio-tungstenite, axum, futures, rusqlite, sqlx*, neo4rs, ureq) | 10 | native-only, behind the kernel-db/hub traits; replacing needs its own ticket |
| proc-macro/build (proc-macro2, quote, syn, serde, schemars) | 5 | build-time; `serde`/`schemars` runtime derive use overlaps the repo's own ToValue/FromValue → migrate |
| text/graphics engines (parley, swash, rustybuzz, taffy, vello*, peniko, kurbo, tiny-skia, usvg, resvg, typst*, bytemuck, fdg-sim) | 16 | large engines; keep behind ui-render/raster/typeset interfaces |
| format codecs with in-repo stdio twins (image, png, zip, serde_json, base64, markup5ever_rcdom→moved, rust_xlsxwriter→moved) | 5 | **runtime violations with an in-repo replacement**: image (🗺️surface, ♾️infinite, wgpu renderer), png (os host), zip (kernel `.sxt` + os host), serde_json (120 manifests), base64 — guest-linked → prepared patches after PUBLISH, one crate family per patch |
| React renderer stack (react, react-dom, i18next, dnd-kit, R3F/drei/three, xyflow, resizable-panels, tailwind, katex, pdfjs-dist, reveal.js, xstate) | 19 | the React shell target itself; `three` is ALSO a registered oracle → conflict stays until the React 3D path is behind an interface |
| JS tooling/servers declared as `dependencies` (typescript, nx, @nx/js, @nxlv/python, esbuild, chevrotain, next, pg, pg-boss, sharp, @napi-rs/canvas) | 11 | the Nx bootstrap tools package (`⚡️caching/🚀️bootstrap/🛠️tools`) legitimately installs nx+typescript as its own runtime and its script verifies `manifest.dependencies` → the gate should treat that owned installation manifest like root `package.json` (policy decision, dev) |

Remaining oracle conflicts (6): `three` (ui-react, r3f, renderer-react — runtime), `typescript` (Nx bootstrap tools — policy above),
`image` (🗺️surface, ♾️infinite, wgpu renderer), `png` (os host), `serde_json` (120), `zip` (kernel + os host). All six are guest-
linked or policy-level → next cycle (build-quiet now).

### Item 9 — ratchet decision for the 13 NEW peer dependencies (coordinator rule, 10:4x)

Rule: (1) AccessKit set approved (WG10, behind `♿️native-accessibility`); (2) other runtime deps only for unavoidable platform
integration behind one interface module with no exported third-party types; (3) test/oracle-only → dev-deps. Measured per row
(manifest section read from the Cargo.toml / package.json):

| dependency | declared in | section | verdict |
|---|---|---|---|
| rust accesskit 0.25.1, accesskit_winit 0.34.1 (+ linux `accesskit_unix`/`async-io` features) | wgpu renderer | `[target.'cfg(not(wasm32))'.dependencies]` | APPROVED (1): only `🎯️targets/🧊️wgpu/♿️native-accessibility/🦀️.rs` + its law import it; 0 `pub` items mention an accesskit type |
| rust accesskit_consumer 0.39.1 | wgpu renderer | `[target…dev-dependencies]` | APPROVED (1/3): oracle dev-dep |
| rust hayro 0.4.0 | stdio semio | `[dev-dependencies]` | (3) dev-only oracle — compliant |
| rust json-patch 4.1.0 | stdio registry contract | `[dev-dependencies]` | (3) dev-only oracle — compliant |
| rust naga 29.0.4 | ui render | `[dev-dependencies]` | (3) dev-only (WGSL validation oracle) — compliant |
| js pngjs 7.0.0 + @types/pngjs | renderer-react | `devDependencies` | (3) compliant |
| js @types/bun, @types/markdown-it, @types/micromatch, @types/picomatch, picomatch | root | `devDependencies` | repository tooling (dev) — compliant |

No row violates the rule, so nothing to remove. Accepting them needs `bun ./📜️script.ts verify dependencies write-baseline`
(shared `🔒️dependencies.json`; coordinating session only) — handed to main.

- 10:4x ratchet decisions recorded (§Item 9 table); write-baseline handed to main.
- 10:5x remaining TS errors fixed at the root: the repo's ambient `bun:sqlite` declaration (`📚️library/🏃️process/🌿️environment/
  🟦️.d.ts`) lacked `Database.transaction`, which `🧑‍💻dev/🔌️vite-plugins` has used since HEAD (5 calls) → declared with Bun's real
  signature (framework + os-hub-ts: 5 errors each); `fresh` (styling suite's 304 oracle) declared in the framework ambient file
  beside `semver` (exact call surface); pixels layer law admits each fixture layer list through its own Ajv schema as a typed
  guard (`compile<{layers: RasterStackLayer[]}>`) instead of passing raw JSON strings. **framework rc=0, os-hub-ts rc=0,
  framework-os rc=0** (`tc-*-8.txt`); laws: pixels layers 11/11, styling `fresh` law 1/1.

- 10:4x–11:0x **item 1**: a peer's mis-anchored insertion (`⚖️gate🌉️os-mcp💼️inference-quartet` spliced into the middle of the
  `🛡️security` row, 10:3x) had broken BOTH `🧩️launch.seed.jsonc` and `launch.json` JSONC → repaired (moved the security row's
  presentation back; rule 14 one-obvious-fix). Generator extended + seed policy + laws (§Item 1); launch.json rendered by the
  registry's own `generateLaunchJson` (`wp-r9/launch-render-probe.ts --write`, same bytes `plugin-registry:generate` writes).
  Measured: 439 projects, 2 417 declared targets, **0 unregistered**, 1 397 configurations (was 669), 29 family rows + 29
  pickers, 0 duplicate names, `jsonc-parser` (VS Code's parser) 0 errors, 21 024 lines, byte-identical re-render. Registry
  launch laws **14/14** (`launch-laws-4.txt`); framework-os typecheck rc=0. `check-generated`: launch.json fresh (the stale
  `🔌️plugins.json`/`🖥️hosts` catalog files are W3's generate). repo-lib typecheck has 6 pre-existing errors: the workspace-contract
  law imports Neo4j graph-export helpers the 09-26 15:13 commit removed from root `📜️script.ts` (not R9's; routed).

## Item 1 — launch.json registration

**Rule (what is an executable command).** Every target declared in a `📋️project.json` is an executable command: per AGENTS.md
each declared target is one `📜️script.ts <command>` invocation a dev runs, so each must be launchable from `launch.json`.
Targets the nx plugins only *infer* and no manifest declares — `describe`, `component-dev/-release`, `materialize-dev/-release`,
`test-contract/-oracle/-subject/-parity`, `nx-release-publish`, the inferred `test-quick/-long/-exhaustive` siblings — are
pipeline steps: they run only as `dependsOn` of declared targets or through the aggregates, so they are not registered
(measured: 10 550 graph targets vs 2 417 declared).

**Mechanism (extends the one existing generator, `🔌️plugin/📇️registry/🚀️launch/🟦️.ts`; V1 confirmed it is the only one).**
- Schema: the seed gains `projectLaunchers` (after `devLaunchers`, outside the generated output): `familyMinimumProjects` (3),
  `familyEmoji` (📋️), verb `classes` (dev 🛠️ `3_dev`, build 📦️ `4_build`, gate ⚖️ `4_gate`, run ▶️ `3_dev`; each with the
  target-name tokens that select it and an `orderBase` placing generated rows after every curated one), `languageSegments`,
  `transparentSegments`, `skipDirectories`.
- `declaredProjectTargets(repoRoot, view)` walks the manifests through a tree view (the registry's `RegistryCatalogInputView`
  in preview, the filesystem otherwise — measured identical, 439/439).
- `generateLaunchJson(repoRoot, playgrounds, projectTargets, readText?)` (the unused `_components` parameter replaced; the dead
  `componentLaunchers` projection removed): a curated seed row that runs `nx run P:T` wins and keeps its name/place; a target
  name declared by ≥ 3 projects becomes ONE family row `bun nx run ${input:projectTarget.<name>}:<name>` with a generated
  `pickString` input listing exactly the declaring projects; every other declared target gets `<class emoji><target><label>`
  where the label is the shortest unique trailing run of the project's path segments (`🌎️hub🦀️`, `💻️os🦀️`); generated names
  may not collide (throws).
- Laws (`🧪️tests/🚀️launch/🟦️.ts`, new block "declared project targets"): every declared target is launchable (row or picker);
  every picker lists exactly the declaring projects; the render parses with TypeScript's independent JSONC reader, equals Bun's
  parse, and is byte-identical to the committed file (and `check`/`check-generated` keep failing on a stale launch.json).
- **Manifest inputs — window-2 incident and re-derivation (prepared, NOT landed).**
  - *Incident.* 14:1x I applied `wp-r9/launch-inputs-taxonomy.py` (`**/📋️project.json` in the plugin-registry `inputPatterns`).
    `validateTaxonomy` (`🔍️discovery/🟦️.ts` 4812–4921) rejects any wildcard within the first N segments of a depth-N opaque root
    (`compose`, `temp/compose`, `♻️mit-bestand/🔎️recherche`); `**` at index 0 → "inputPatterns[0] can cross an opaque boundary"
    → every `serve s react dev` refused to start. I replaced it with 12 literal-prefix patterns (`wp-r9/launch-input-prefixes.ts`,
    `launch-inputs-taxonomy.ts`), which my `loadCatalogTaxonomy` probe accepted, but the serves had already failed; main
    restored `🔣️taxonomy.json` to HEAD at 14:53. Both taxonomy scripts are superseded — do not re-apply them.
  - *Why no taxonomy edit is needed.* The plugin-registry contract already has a discovery-declared input:
    `registryCatalogInputPaths` feeds `repo:generator-inputs`, whose receipt `plugin-registry:generate` consumes via
    `dependentTasksOutputFiles`, and that target's nx inputs already list `{workspaceRoot}/**/📋️project.json` (nx globs are outside
    the taxonomy validator). The missing piece is only that the receipt must hash every manifest's *content*: at HEAD it holds
    80 of the 439 launch-projected manifests, 51 more only as directory witnesses and 308 not at all (`launch-manifest-coverage.ts`).
    The 12-prefix `inputPatterns` would also make normalization's `generatorInputInventory` walk the whole `✏️s/🔌️plugins` and
    `🧰️framework/🛍️products` trees on every registry transaction.
  - *Re-derived change* (`wp-r9/launch-manifest-inputs.discovery.patch`, 4 hunks, `git apply --check` clean): `scanRepo`'s
    catalog walk records every `📋️project.json` it meets (owner directories, package language directories and the
    `target-inside-package-boundary` subtree used by `🎤️presentation`), and the rust-plugin-only special case goes. Law
    (`wp-r9/launch-manifest-inputs.law.patch`): every launch-projected manifest is a registry catalog content input. It uses a
    view pruned to the manifests' ancestry and runs in 11 s instead of 47–109 s.
  - *Evidence while applied (15:08–15:31).* The probe printed `taxonomy valid` 3×. Full unpruned walk: 438/439, and the 439th
    (presentation's targets subtree) was fixed by the 4th hunk. Pruned walk: 439/439 (`launch-manifest-pruned.ts`). The "declared
    project targets" law block passed 4/4, including the new law (`launch-laws-8`). launch.json stayed byte-identical (1 244
    configurations).
  - *Not landed.* No serve boot was possible before the 15:45 freeze (load 40–48, three peer serves running since 14:05/14:56). Rule 33 requires
    a serve boot, so both hunks were reverted at 15:31: discovery and taxonomy are back to HEAD, and the probe prints `taxonomy valid`.
    To land next window: `git apply` the two patches → probe → one `serve s react dev` boot → launch law block → landing row.
- **Launch fixture (applied, test-only).** Law 1 ("distinguishes standards and subsets…") failed because the playgrounds
  `stdio-dwg-*`, `stdio-pdf*-a` and `stdio-gif*` no longer exist: the staged stdio `🔣️.json` now ships only the text families,
  and launch.json has 0 `ac1018` rows vs 8 at HEAD. Its `prefixes` now use the surviving variants (`stdio`, `stdio-json`,
  `stdio-json-i`, `stdio-xml`, `stdio-xml-valid`), matching `playground-prefixes.ts` exactly. The keycap-folder slugs stay covered
  by `slugs`. The vitest re-run was killed by the 300 s budget under load, so this is unverified by vitest.

- 11:0x item 10: the 09-26 15:13 commit deleted the Neo4j Cypher graph export (root `📜️script.ts`: exporter, `purge`, `mcp neo4j`)
  but not its law → the `Neo4j graph database registry` describe block (4 tests) and its import removed from the workspace-contract
  law (the law leaves with the feature). **repo-lib typecheck rc=0.** Law file at quick (`bun test`, artifact dir under the ticket's
  `🗑️generated/r9-workspace-contract`): **604 pass / 86 skip / 5 fail**; 1 fail was W3's new owned generator
  `mutation-source-authority` missing from the language-neutral `🏭️owned-generator-preview-inventory` fixture → added (law 1/1);
  the other 4 (glue-content classification ×3, JCO physical matrix) refuse any artifact dir outside "the active ticket generated
  directory" the repo harness resolves — environment, not code (run them through `@semio-tech/repo-lib:test-quick`). The Draw
  launch-seed law (my generator change) passes at exhaustive (1/1).

## Item 6 — comments inside definitions

Measured with a real parser (`wp-r9/comment-census.ts`: tree-sitter-rust via `web-tree-sitter`; every comment node classified by its
enclosing construct): hub before = fn-body 234, impl-between-members 77 (74 of them `//#region` markers, allowed), type-body 2;
MCP before = fn-body 511, impl 14 (all region markers), macro-body 3. After `wp-r9/comment-hoist.ts --apply` (221 blocks: each
block of consecutive comment lines becomes a paragraph of the enclosing fn/const/static docstring, or of the next member's /
field's docstring; in-body `//#region` markers deleted; 12 new docstrings got hand-picked emojis) → **0 comments inside
definitions** in both crates (`comment-census-2.json`); `cargo check -p semio-hub -p semio-framework-os-mcp --lib --bins --tests`
0 errors.

Guest-linked plan (next cycle, after PUBLISH DONE — build-quiet now): dry run over `🧰️framework/🔨️modules`,
`🧰️framework/🛍️products/💻️os/🔨️modules`, `✏️s/🔌️plugins`, `✏️s/🔨️modules` = **10 296 blocks in 1 919 files**; 1 215 of them
land on a definition without a docstring and need an emoji picked (the tool refuses to write such a file until its
`EMOJI_CHOICES` row exists). Largest: stdio semio brep engine 329, wgpu Shell target 247, plugin SDK 221, stdio table schema
125, ui-contract builder 97, dwg ac1024 io 93, plugin builder-contract law 91, kernel store 90. Steps: (1) generate the
needs-emoji table from the dry run and pick one unique emoji per new docstring (reviewed, not generated); (2) apply per crate
family (SDK → kernel store → ui → wgpu shell/renderer → infinite → plugins), each followed immediately by native + wasm32
`cargo check -p` of the touched crates (compile-atomic), diff reviewed for moved text; (3) a repo-lib law running the same
tree-sitter census (fn-body / type-body / non-region impl comments = 0) so the count cannot regrow. Only 17 `// SAFETY` comments
exist repo-wide and no clippy `undocumented_unsafe_blocks` lint is enabled; they move into the fn's `# Safety` docstring section.
