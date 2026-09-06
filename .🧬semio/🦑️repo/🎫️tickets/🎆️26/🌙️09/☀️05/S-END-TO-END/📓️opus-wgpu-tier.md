# Lane L — `wgpu-tier` (Wave 3): `🛠️dev🖥️s🧊️wgpu🌐️wasm` boot + `parity verify s`

Opus 5 implementer, 2026-09-06 05:20 → (in progress). Spec: `📓️explore-wgpu-tier-readiness.md` (8-step
checklist), `📓️explore-wgpu-and-native-shells.md`, plan entry 05:20 (`PackInteger` carriers).
Private cargo target `target-s-e2e-wgpu`, `RUSTC_WRAPPER=""`. Logs: `🗑️generated/lane-l/`.

## 1. Cheap RED gates (checklist steps 1-3)

### 1.1 `check-frame-worker` — was RED, now green (and re-goes-RED on every OS-TS edit)

Lane K's report said the deployment-catalog rename made it green again; **it was RED again at 05:24**:

```
$ bun ./📜️script.ts check-frame-worker
error: 🎞️frame-worker.js is stale; run the generate-frame-worker target
    at checkFrameWorker (…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:238:105)
```

`bun ./📜️script.ts generate-frame-worker` rewrote
`…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🎞️frame-worker.js` (+22/-3 lines vs HEAD) and
`check-frame-worker` then reported `🎞️frame-worker.js is fresh`.

**Structural finding for the coordinator**: the artifact is a committed `Bun.build` of
`🎞️frame-worker/🟦️.ts`, whose import graph includes `🧰️framework/🛍️products/💻️os/🟦️.ts`. Any edit to
that file (including this lane's own §2 edit) makes the gate RED again until `generate-frame-worker`
re-runs — it went stale a second time inside this session for exactly that reason and was regenerated
again. Every lane that touches the OS TS package must re-run `generate-frame-worker`, or the wgpu
`serve`/`dev`/`parity verify` path aborts before `trunk serve` (`TrunkServeScript.run`, `📜️script.ts:262`).

### 1.2 `lint` — 3 raw colour literals → theme tokens

`🧊️renderer/🦀️.rs:15240-15242` constructed `ui_wgpu::wgpu::Rgba::from_srgb8(…)` inline for the
`JobProgressKind` overlay. The `ui_wgpu::Theme` had only `error` as a semantic outcome colour, so two
tokens were added rather than relocating the literals:

- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🎨️theme.rs:131-136` — new
  `Theme::success` / `Theme::progress` fields (docstringed, beside the existing `error`).
- `…/🎨️theme.rs:242-243` — `success: Rgba::from_srgb8(36, 158, 91, 255)`,
  `progress: Rgba::from_srgb8(67, 132, 245, 255)` in `from_chrome`, the module the lint rule scopes
  colour construction to.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:15239-15244`
  — the match arms now read `theme.success` / `theme.error` / `theme.progress`, with the theme taken
  from `self.interaction` (falling back to `Theme::default()` when no interaction state exists).

```
$ bun ./📜️script.ts lint
framework-renderer-wgpu: color-literal lint passed
```

Note the fault colour is now the theme's `error` (`Rgba::new(0.95, 0.35, 0.35, 1.0)`), not the former
inline `from_srgb8(218, 74, 74, 255)` — a deliberate token adoption, not a regression.

### 1.3 `test-preview-generated` — was RED twice over

Two independent failures, both fixed:

**(a) real test failure, not a budget artifact.** Running the suite outside the budget showed
**2 of 12 cases failing**:

```
FAIL … > activates only the current no-follow package through both independent TypeScript compilers
AssertionError: current-catalog-and-package: expected false to be true
FAIL … > validates the current package catalog with Ajv and independent WebCrypto integrity vectors
AssertionError: reviewed-catalog-bytes: expected false to be true
```

Cause: `🔣️taxonomy.json`'s `generatorContracts["wgpu-frame-worker"].packageGeneration.catalogSha256`
was `72823763f2bcf38a…`, while
`…/🎯️targets/🧊️wgpu/🪪️package-catalog.json` hashes to `fe9cc288e4f6411a…` — both the working tree and
the file's only commit (`fe7c8a8f8b`, 2026-09-05 03:53, which introduced both). The pinned digest
matches no committed version of the catalog; the catalog's own four `artifacts[].content` blocks were
verified byte-identical to the four files on disk, so the catalog is the authority and the pin was
stale. Fixed at
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json:16820`.

**(b) level.** Even green the suite exceeds the 15 s `fundamental` budget: `import 13.88 s`
(the TypeScript compiler, Ajv, emoji-regex and `loadTaxonomy` — its four independent oracles) before
any case runs, plus two full browser-bundle renders; 18 s idle, 33 s under load ~60. Levelled to
`long` rather than trimmed, by giving suites a way to declare a floor:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:1154-1169` —
  `resolveTestLevel(segments, minimum: TestLevel = "fundamental")`; the resolved level is
  `max(requested, minimum)`, so an explicit `test-preview-generated exhaustive` still wins and every
  existing call site is unchanged.
- `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts:567-575` — `PreviewGeneratedTestScript` passes
  `"long"`.

```
$ bun ./📜️script.ts test-preview-generated
 Test Files  1 passed (1)
      Tests  15 passed (15)
   Duration  36.90s
```

(12 pre-existing + 3 new from §2.)

### 1.4 Stale ship path at dev `📜️script.ts:1373`

Confirmed: `BuildScript` resolved `…/🧑‍🎨engine/🧊️wgpu/📦️packages/🦀️rust/📜️script.ts` — no `🎯️targets/`
segment, a path that does not exist. Three call sites carried the same literal (two correct, one not),
so they now share one constant instead of a fourth chance to drift:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:84-87` — new
  `WGPU_PACKAGE_ROOT` / `WGPU_SCRIPT_PATH`.
- `…:1325-1326` (dev `serve`), `…:1378` (ship `wasm --release`, the bug), `…:5094` (bench `native`) all
  route through them; the ship path additionally gained the correct `cwd`.

`bun build --target=bun` of the dev script: `Bundled 941 modules in 2932ms`.

## 2. `PackInteger` carriers on the wgpu wire (plan entry 05:20)

The 03:51 commit made `decodePackValue` return lossless carriers. Proof of the two shapes:

```
$ bun -e 'import {encodePackValue,decodePackValue,decodePackWire,packUInt} from "./🟦️.ts"; …'
[DEBUG] raw decodePackValue: {"children":[{"kind":"uint","value":"9n"}],"id":{"kind":"uint","value":"7n"}}
[DEBUG] projected decodePackWire: {"children":[9],"id":7}
```

The projection now lives once, beside the codec it projects, instead of a second copy per renderer:

- `🧰️framework/🛍️products/💻️os/🟦️.ts:1538-1563` — new exported `describePackWireValue`,
  `packWireNatural(raw, field)` (WIT `bigint` / `PackInteger` carrier / absent→0, everything else a
  named fault) and `decodePackWire(bytes, path)` (`packValueToExactJson ∘ decodePackValue`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx:46,539-548`
  — the React shell's own `wireNatural`/`decodeWirePack`/`describeWireValue` bodies (added at 05:20)
  are now the shared implementations; all ~15 call sites unchanged.
- `…/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/🐚️plugin-bridge.ts:71` — imports `decodePackWire`
  and `packWireNatural`.
- `…/🐚️plugin-bridge.ts:127-134` — new exported `reconcileRetainedWindowPatch(previous, patch)`:
  `decodeWirePatchOps(patch.ops, decodePackWire)` (was `decodePackValue`) and both revisions through
  `packWireNatural` (was `patch.revision ?? 0` — a WIT `u64` `bigint` there fails
  `applyUiPatchToRetained`'s `baseRevision !== previous.revision` identity check forever, i.e. a
  permanent silent desync, the same class as the React fault).
- `…/🐚️plugin-bridge.ts:172-184` — new exported `decodeInvocationPayloads(frame)` replaces the four raw
  decodes the spec cited at `:171-175` (`output`, `diagnostics`, `ui_scope`, `history_patch`).
- `…/🐚️plugin-bridge.ts:198` — `wireEffectToFriendly(effect, decodePackWire)` (was `decodePackValue`),
  so `openWindow.params` and every other packed effect field is projected too.

Fault decoding (`decodeFaultFromWire`/`faultDisplayMessage`) deliberately keeps raw `decodePackValue`,
matching `PluginRuntime`'s own treatment.

Tests — `…/🎯️targets/🧊️wgpu/🧪️tests/🧩️package-integration.ts`, new describe block
`framework renderer wgpu pack integer carriers` (3 cases, run by the registered `test-preview-generated`
target):
1. carriers + `bigint` revisions off the retained-window boundary land as exact JSON and a second patch
   with `baseRevision: 1n` reconciles instead of desyncing;
2. `packUInt(2n ** 60n)` throws naming `$.id`, and a `2n ** 60n` revision throws naming
   `uiPatch.revision` — a rejection, never a rounding;
3. all four `Invocation` payloads compared against an independent `JSON.parse` oracle, plus
   `JSON.stringify(output)` must not contain `"kind"`.

## 3. `wasm32-unknown-unknown` compile check (checklist step 4)

_(pending — see §6)_

## 4. Boot (checklist step 6)

_(pending)_

## 5. `parity verify s` (checklist step 8)

_(pending)_

## 6. Blockers and hand-offs

_(pending)_
