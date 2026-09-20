# React And Host Census Triage

## Basis

This is a read-only classification of Root’s completed long React Engine census, recorded in `📓️astra-react-engine-census.md`: 1,777 / 1,811 tests passed; 34 failures in eight files. I did not rerun the census, build, activate a renderer, or change production code.

## Classification

| Census group | Failures | Classification | Verified basis and repair packet |
|---|---:|---|---|
| Resident refresh budget | 1 | Fixture/test contradiction; not a capacity regression | The TypeScript count at `🧪️tests/🎟️resident-refresh-budget/🟦️.ts:203` remains four while the fixture has five laws. The fifth is false; see Resident Accounting. |
| World3d scene shading | 1 | Obsolete exact expected shape | Schema requires `maximumRgbError: 1` at `🧬️schema/🎨️scene-shading/🔣️.json:225-257`, and the fixture supplies it at `🧫️fixtures/🎨️scene-shading/🔣️.json:743-759`; only the test expectation omitted it. |
| Engine contract | 13 | Twelve stale assertion/harness failures; one test-order race | Detail below. None independently establishes a React/WGPU behavior regression. |
| WGPU extension dispatch | 1 | Production performance failure | `wgpuContributionsIngressSize` pack-encodes command and view then materializes both byte arrays as JS number arrays at `🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts:761-776`. The actual-shaped 190.7k contribution command at `🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts:84-102` times out while exercising production `setContributions` admission at `plugin-bridge:1612-1618`. Optimize the sizing/admission encoding without weakening its page-cap refusal law. |
| World3d interaction | 7 | Obsolete neutral fixture expectations | Fixture scene default is `handle` at `🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json:1-18`, it omits instance overrides, but expects `object` at `:90-120`. React correctly chooses instance override then scene default at `🌐️World3dHost/🟦️.tsx:5419-5438`; marquee intentionally sends `rectangle` or `lasso` at `:7003-7014`, not `pick`. Add explicit instance `object` granularities if that is intended, otherwise update expected targets to `handle` and marquee to `rectangle`. |
| WGPU package integration | 7 | Two package-authority failures; five test-runtime failures | Actual catalog SHA-256 is `97b7fca0a4e1a74abbb81f649524faf5b634577e48d13a33e31892ef32b05f1b`; taxonomy pins `5a4c510292d2c1f56aad20fe33adfde7b4313dc118e30ae5eb3087919c43b7da` at `🛍️repo/🔨️modules/📚️library/🔣️taxonomy.json:28553-28559`. Parser correctly refuses drift at `📚️library/🔍️discovery/🟦️.ts:5396-5398`, causing the first two failures. Reconcile the canonical catalog’s sole digest authority. The other five read `Bun.Transpiler` or `Bun.version` directly at `🧪️tests/🧩️package-integration/🟦️.ts:312,392,445`, while this Vitest worker has no `Bun` global. Repair injected/test runtime capability while retaining the pinned-Bun production check. |
| Window fault | 2 | Obsolete source-text anchors | The shared publisher returns `empty` before install for `[]` at `🧱️elements/🛠️ShellHelpers/🧩️contributions/🟦️.ts:94-105`; ShellHost only builds the scoped pack at `🏛️ShellHost/🟦️.tsx:4910-4918`. Point the source test to the helper’s behavioral seam. |
| PluginRuntime host effects | 2 | Obsolete fixture and logging assertion | WIT names `artifact-json` at `🔌️plugin/🧬️schema/📜️.wit:401-410`; the fixture supplies old `documentJson` at `🧱️elements/🔌️PluginRuntime/🧫️fixtures/🎯️host-effect-address.json:7-13`. Rename the fixture field. The second test observes the intended null refusal but additionally demands warnings at `🧪️tests/🔌️plugin-runtime/🟦️.tsx:387-397`; remove that logging-only expectation unless diagnostics are made a contract. |

## Engine Contract Detail

- **Catalog dialog:** stale tuple. The creation relay deliberately discards decoded schema/dialect/labels, retaining catalog-confirmed `kindId`, documented at `🏛️ShellHost/🟦️.tsx:1372-1379` and enforced at `:1458-1468`. Update `engine-contract:267-276` to assert canonical choice and resulting kind ID, not the removed `schema: "gis.map"`.
- **Board peer scope:** race. The test waits for `attach_canvas`, clears spies, and immediately asserts at `engine-contract:1562-1578`. Every mounted board applies `scene.selectionJson` (`"[]"`) in an effect at `🖥️Board2dHost/🟦️.tsx:1092-1105`; the two late empty calls therefore match pending mount synchronization. Await initial selection synchronization, clear, then prove routing remains inside the passed `BoardPeerScope` (`pushPuzzle2dLiveMirrorMutations` iterates `board2dPeers(scope, …)` at `:542-558`).
- **Sync browse:** component uses localized `ui.sync.browse` for title and accessible name at `🔄️ShellSync/🟦️.tsx:95-103,149`; the exact `Browse…` lookup is locale residue. Query `[data-semio-sync-browse]` for callback routing and test label wording in the locale corpus.
- **Sync fixture:** `readFileSync(new URL(..., import.meta.url))` at `engine-contract:2297-2318` gets a browser-style transformed module URL. Resolve the fixture by file-system test path; it is not a host-I/O behavior failure.
- **Declarative control:** the failed sync test leaves its DOM alive, so the next role query sees a previous sync input. Scope the query to its view or restore cleanup.
- **Flow graph parameter:** current generalized host emits `nodeGraphEdit`; the test expects retired `setGraphParameter`. Rebase expected action on current declared command; do not reintroduce a plugin-specific control verb for an old test spelling.
- **Edge hover:** component hover intentionally maps to `highlighted`, not old grey `hovered`, at `🌐️World3dHost/🟦️.tsx:492-506`; update `engine-contract:6320-6332` to that theme contract.
- **Self-gated world lane:** a brittle old source substring fails, but marker and instance paths return `dispatchSettled` at `🌐️World3dHost:5931-5938,6328-6340`. Replace text matching with a controlled-promise mounted law.
- **Option-lock triple:** assertions snapshot the former publication path. Utility writes now funnel through `writeUtilityRegister`, publishing a scoped leftover at `🏛️ShellHost:4448-4461`; effects route there at `:5777-5787`. Rebase on `LeftoverWorldOverlayScopeV1` outcomes, retaining per-window isolation and stale-selection retirement.
- **Per-window IDs:** test calls absent `CSS.escape` in jsdom. Inject/polyfill browser API in test environment.
- **Window action scope:** descriptor now includes `translateSelection` for `generation3d-generate-preview` and `procedural-preview`. Update expected owners after retaining an assertion that both are declared owners; this is not a foreign-app leak.

## Resident Accounting

The fifth fixture law says every one of 64 slots can hold a full document and byte/slot refusal occurs together. Current source and measurements disprove it.

- Aggregate is `64 × 821,248 = 52,559,872` at `🎟️resident/🦀️.rs:16-31`.
- Ledger starts with static `CONTRACT_BACKING_BYTES` charged at `:104-106`. Fixture measures `566,352` backing bytes, 128 nodes, 6,416 bytes per record, and 1,906 open bytes at `🎟️resident/🔄️refresh/🧫️fixtures/🔣️.json:3-10,45-54`.
- A populated 128-node document reserves `(128 + 2) × 6,416 + 1,906 = 835,986` through `ui_document_resident_limits` at `📃️document/🦀️.rs:465-470`.
- `floor((52,559,872 − 566,352) / 835,986) = 62`; the 63rd full document overflows before the 64-slot ledger is full. The new Rust law only opens 64 cold builders and checks one-record price at `🎟️resident/🔄️refresh/🧪️tests/🔄️refresh/🦀️.rs:197-216`; it never tests this boundary.

Keep existing production limits. Replace the unsupported “ledgers refuse together” claim with independent byte/slot ceilings and add a 62-admit/63-refuse populated-document law. Rename the TypeScript test “admits only three” at `🧪️tests/🎟️resident-refresh-budget/🟦️.ts:181`, since the fixture now correctly declares six ceiling-sized roots.
