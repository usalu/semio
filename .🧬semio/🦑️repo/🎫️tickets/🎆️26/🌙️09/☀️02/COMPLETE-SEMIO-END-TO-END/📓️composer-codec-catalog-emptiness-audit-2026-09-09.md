# Composer Codec Catalog Emptiness Audit

Status: read-only source audit on 2026-09-09 against the current working tree. No Cargo build, Nx gate, browser, or hub process was run.

Ticket: `26/09/02/COMPLETE-SEMIO-END-TO-END`  
Acceptance anchor: `✅️acceptance-matrix.md` row **Artifact opening** — “Production startup still supplies zero native codec bindings, so the catalog remains empty, readiness fails closed…”

## Executive verdict

The acceptance matrix statement is **half superseded and half still true**.

| Claim fragment | Still true in current tree? | Precise current behavior |
|---|---|---|
| “Zero native codec bindings” at production startup | **No (wording is stale)** | Default `os-hub` is built with `native-artifact-execution` (see `🌎️hub/📦️packages/🦀️rust/Cargo.toml:22,30`). Startup passes `NativeCodecProviderSetV1::linked()` when that feature is on (`🚀️bin.rs:8312-8317`). That provider statically admits **29** factory receipts: stdio 26, GIS 2, VCS 1 (`📇️native-openable-provider/🦀️.rs:8-11,26-29`). |
| “Catalog remains empty” | **Yes** | Default hub data root `./.🧬semio/🌐hub/` has **no** `trusted-catalog/current.json` (verified absent in this audit). `TrustedCatalogLoader::load_current` returns `None` when the pointer is missing (`🔏️trusted-catalog/🦀️.rs:476-482`). `configured_artifact_authority` therefore returns `None` (`🚀️bin.rs:444-459`). |
| “Readiness fails closed / real hub opens fail closed” | **Yes** | `artifact_authority_ready = artifact_authority.is_some()` and `open_plan_ready` requires `open_target_count() > 0` (`🚀️bin.rs:8344-8345,2080-2109`). With no loaded generation both are false; `/readyz` reports `artifactAuthority.ready: false` and `features.openPlan: false`. Document-open plan issue/exchange routes require `openable_catalog` and fail closed without it. |
| Headless hub has zero provider | **Yes (by design)** | Fixture `🌎️hub/🧫️fixtures/🧭️native-artifact-provider-frontier-v1/🔣️.json` and source law `proveNativeArtifactProviderFrontier` require headless `--no-default-features --features sqlite` with **no** plugin deps; startup passes `providers: None` under `#[cfg(not(feature = "native-artifact-execution"))]` (`🚀️bin.rs:8316-8317`). |

**Bottom line:** the old `linked_native_codec_bindings() -> Vec::new()` observation is obsolete. The **effective** production outcome unchanged: **no verified trusted catalog generation is loaded at ordinary `os-hub` main startup**, so **no codecs are registered in the store**, **open-plan readiness stays false**, and **real authenticated hub document open still fails closed**.

Only the **`bun 🌎️hub/📦️packages/🦀️rust/📜️script.ts dev`** orchestration path attempts auto-materialization/publication before spawn (`📜️script.ts:12571-12578`). Ordinary registered React OS serve targets (`framework-os-dev:serve-*-react-*`) do **not** start hub or publish a trusted generation. Direct `cargo run --bin os-hub` (production path) also does **not** bootstrap.

---

## Where native codec registrations are supposed to come from

Native codec authority is a **three-layer** pipeline; only the last layer produces a nonempty **verified hub catalog**.

### Layer 1 — Plugin-owned factory receipts (compile time, inert)

| Package | Generator | Count | Feature gate |
|---|---|---:|---|
| `semio:stdio` | `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs` → `native_codec_factory_receipts()` | 26 | `full-artifact-catalog` |
| `semio:gis` | `✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🦀️.rs` → `native_codec_factory_receipts()` | 2 | always (Map + Terrain) |
| `semio:vcs` | `✏️s/🔌️plugins/🌿️vcs/📇️native-codecs/🦀️.rs` → `native_codec_factory_receipts()` | 1 | always |

Stdio receipts are emitted only after `validate_catalog`, bijective factory checks, and `validate_native_openable_projection` (`📇️registry/🦀️.rs:639-671). Six descriptor ledger rows vs zero executable registrations in the **monolithic stdio Wasm row** remains a separate registry-row blocker documented in `📓️sol-stdio-catalog-root-completion.md`; that is **not** the same as hub trusted-catalog emptiness but blocks fresh stdio component publication.

JSON projection inputs (not authority):  
`✏️s/🔌️plugins/🗄️stdio/📇️registry/📜️native-codec-factories.json`,  
`✏️s/🔌️plugins/🌍️gis/📇️native-codecs/🔣️.json`.

### Layer 2 — Hub static provider (link time, preview only)

`🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs`

- `NativeCodecProviderSetV1::linked()` tables stdio, gis, vcs preview functions (`:26-29`).
- `NativeOpenableCatalogProviderV1::linked(version)` re-instantiates all 26 stdio receipts and checks bijection (`:131-170`).
- GIS/VCS previews revalidate identity, pack hash, factory id, and `into_codec_and_genesis()` (`:66-123`).
- Module is compiled only with `native-artifact-execution` (`🗿️artifact-authority/🦀️.rs:586-588`).

This layer **does not** publish codecs or open targets by itself. It answers `preview()` during trusted-bundle verification.

### Layer 3 — Trusted catalog loader (runtime, authoritative)

`🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`

1. Read server-owned `{OS_HUB_DATA}/trusted-catalog/current.json` pointer.
2. Load immutable generation directory + `trusted-catalog.json` bundle.
3. `providers.preflight_selection(&requirements)` then per-package `providers.preview(...)`.
4. Match bundle native codec rows to previewed bindings bijectively.
5. `os_store::register_document_codecs_in_assembly` — **the only runtime registration** (`:510-516`).

Without step 1–2, layers 1–2 never reach the store.

### Layer 4 — Publication / bootstrap (out-of-band from main)

| Mechanism | File(s) | Role |
|---|---|---|
| Trusted stdio+GIS bootstrap | `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — `materializeTrustedStdioGisBundle`, `validateAndPublishTrustedStdioGisCandidate`, `publishTrustedBootstrapCurrent` | Builds fresh stdio+gis wasm, assembles `local-stdio-gis-open-v1` bundle, child-invokes `os-hub trusted-catalog publish` |
| Dev orchestration | same file `DevScript` (`:12550-12578`) | Publishes candidate if `trustedBootstrapCurrent(dataRoot)` is absent, then starts hub |
| CLI publisher | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/📤️command/🦀️.rs` | `trusted-catalog publish` entry |
| Opened-root atomic writer | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/` | Server-owned generation directories + `current.json` rotation |
| Single stdio registry row | `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts` — `catalog-root` | Fresh isolated stdio raw/core/descriptor triplet + commit marker; prerequisite for any stdio bytes in a bundle |
| Fixture / test bundles | `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — `native_openable_stdio_bundle()` | Synthetic stdio-only generation for readiness law |

Profile contract for the bounded bootstrap: `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🧬️stdio-gis-bootstrap/🔣️.json` — exactly **2 packages**, **28 codec rows**, **1** GIS Map editor open target, profile id `local-stdio-gis-open-v1`. Loader enforces this closed shape at `🔏️trusted-catalog/🦀️.rs:1056-1085`.

---

## Why the verified catalog is empty today

### 1. No published generation in the default hub data root

`main()` reads `OS_HUB_DATA` default `./.🧬semio/🌐hub/` (`🚀️bin.rs:8311`). That tree has no `trusted-catalog/current.json`. Loader returns `None`; hub starts with `verified_catalog: None`.

Historical env vars `OS_HUB_TRUSTED_CATALOG_BUNDLE` / `OS_HUB_TRUSTED_CATALOG_PROFILE` are **removed** from startup (`📜️script.ts:912-913` explicitly deletes them in `startLocalHub`).

### 2. Ordinary OS / hub launch paths do not publish

- Registered React OS dev/release serve (`🧰️framework/.../🧑‍💻dev/.../📜️script.ts` `ServeScript`) runs Vite only; no hub, no trusted-catalog write.
- Production `os-hub` `main()` has no bootstrap hook (contrast `DevScript`).
- Collab e2e uses `hubScript dev`, which **does** attempt bootstrap — but that is not the generic “production OS startup” path named in the acceptance matrix.

### 3. Bootstrap materialization is blocked on fresh stdio Wasm size

`materializeTrustedStdioGisBundle` requires `produceFreshComponentV1` for **both** `semio-s-plugin-stdio` and `semio-s-plugin-gis` (`📜️script.ts:9981-10039`).

Stdio isolated `catalog-root` enforces `COMPONENT_FUNCTION_MAX = 1_000_000` on the JCO-extracted core (`✏️s/.../stdio/.../📜️script.ts:22,247-249`). Acceptance matrix **Registry** row documents stable failure before publication when the monolithic stdio component exceeds the linker/parser function-section ceiling. Until stdio shards or otherwise compiles under that limit, **fresh trusted bytes for the dependency package cannot be produced**, so bootstrap cannot complete even on the dev path.

GIS fresh component path is separate; GIS native codec receipts are source-green per matrix, but they do not populate hub catalog without successful bundle publication.

### 4. Generated plugin / launch catalogs ≠ verified hub catalog

| Catalog kind | Source | Consumed by hub open authority? |
|---|---|---|
| Generated plugin registry | `🧰️framework/.../🔌️plugin/📇️registry/📜️script.ts` → `🤖️generated/🔌️plugins.json` | **No** — inventory only |
| Playground activation receipt | `framework-os-dev` activation under variant runtime roots | **No** — dev module URLs |
| MCP installed discovery | `🧰️framework/.../🌉️mcp/📇️registry/🦀️.rs` scans repo descriptors | **No** — local MCP gateway; explicitly not authenticated hub selection (`📓️terra-mcp-p4-shared-map-current-acceptance-audit.md`) |
| Verified trusted catalog | `{OS_HUB_DATA}/trusted-catalog/generations/<id>/` | **Yes** — sole `openable_catalog` source |

Acceptance matrix Registry row: `catalog-complete` not proven; ~58 descriptor/pair issues; native codec admission “six declared / zero executable” on stdio descriptor ledger.

### 5. VCS is linked in the provider set but not in the bootstrap profile

`NativeCodecProviderSetV1` includes VCS (`📇️native-openable-provider/🦀️.rs:29`), but `local-stdio-gis-open-v1` selected closure is **only** gis+stdio (`🧬️stdio-gis-bootstrap/🔣️.json:5-8`). VCS codecs are irrelevant until a profile/bundle selects `semio:vcs`.

### 6. MCP stdio discovery vs hub authority (stdio angle)

MCP `build_catalog` / workspace discovery reads the **repository** plugin registry, not the hub verified generation (`🌉️mcp/💡️inference/🦀️.rs:93`, `📇️registry/🦀️.rs:1-15`). Even a nonempty MCP tool list can disagree with hub `openable_catalog`.

---

## Current registration vs “all plugins and artifacts”

Counts from ticket audits (`📓️terra-all-plugins-artifacts-os-coverage-p0.md`, `✅️acceptance-matrix.md` Registry row) cross-checked against current provider source:

| Population | Total in repo/fleet | In linked native provider | In `local-stdio-gis-open-v1` trusted profile | Runtime-loaded today |
|---|---:|---:|---:|---|
| Source plugin directories | 33 | 3 packages (stdio, gis, vcs) | 2 packages | 0 |
| Generated component declarations | 59 | 2 with fresh-build path in bootstrap (stdio, gis) | 2 | 0 |
| Direct artifact roots | 92 | 29 codec receipts (26+2+1) | 28 codec rows declared | 0 |
| Selected document-open targets | many declared in descriptors | provider can validate many **if** bundle lists them | **1** (GIS Map editor) | 0 |
| Browser/WGPU catalog-authorized opens | 59 components (potential) | 0 proven | 1 target spec (wasm GIS) | 0 |

**“All plugins and artifacts”** for end-to-end acceptance requires, at minimum:

1. **Per-plugin** (or honestly bounded multi-plugin) native receipt modules mirroring GIS/VCS pattern for every executable artifact root that should open natively.
2. **Fresh artifact materialization** for each selected package (raw component, core, descriptor, optional browser actor) under registry `catalog-complete` / per-plugin `catalog-root` laws.
3. **Trusted bundle profiles** whose `selectedClosure`, `nativeCodecs`, and `openTargets` biject with decoded descriptors and provider previews — not the generated `plugins.json` alone.
4. **Atomic publication** to `{OS_HUB_DATA}/trusted-catalog/` before or as part of production OS startup.
5. **Client consumption** of hub-issued open plans against `openable_catalog`, replacing module-URL / directory-scan bridges (`🐚️plugin-bridge.ts`, WGPU program bridge).

Current tree intentionally implements **bounded** closure first (stdio+gis[+vcs provider inventory]), not fleet-wide activation.

---

## Proof chain for acceptance-matrix row (what would flip it)

To change **Artifact opening** from PARTIAL with “empty catalog” to PASS on the **production path**:

1. `trusted-catalog/current.json` exists under the hub data root used by production launch.
2. Loaded generation’s profile has `openTargetCount >= 1`.
3. `/readyz` shows `artifactAuthority.ready: true`, `features.openPlan: true`.
4. Authenticated open-plan issue + receipt exchange succeeds against the loaded generation (existing D0/D1 laws).
5. Browser worker completes real-hub journey (not synthetic-server-only).

Today steps 1–3 fail on a clean tree; steps 4–5 have synthetic/fixture evidence only.

---

## Implementation recipe (exact files, ordered)

### Phase A — Unblock fresh stdio bytes (registry prerequisite)

**Goal:** `catalog-root` and bootstrap stdio leg succeed under 1M function limit.

| Step | Files to change | Action |
|---|---|---|
| A1 | `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml`, `📇️registry/🦀️.rs`, component/feature split docs in `📓️root-stdio-native-catalog-split.md` | Shard monolithic stdio Wasm surface so JCO core `definedFunctions <= 1_000_000` while preserving 26-receipt bijection under `full-artifact-catalog`. |
| A2 | `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts` | Keep `COMPONENT_FUNCTION_MAX` enforcement; add/adjust sharded build targets invoked by `catalog-root`. |
| A3 | `🧰️framework/.../🔌️plugin/📇️registry/📜️script.ts` | Repair descriptor-invalid / pair-missing rows until `auditPluginCatalogSources` is clean for stdio owner. |
| A4 | Launch gates in `.vscode/launch.json` / `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | Run `stdio` `catalog-root --build-root <empty>` and `@semio-tech/plugin-registry:check-generated`. |

**Exit:** one fresh stdio triplet + commit marker verifiable by `createFreshCatalogBuildVerifier`.

### Phase B — Prove bounded trusted bootstrap (stdio + GIS)

**Goal:** publish `local-stdio-gis-open-v1` into a real data root.

| Step | Files | Action |
|---|---|---|
| B1 | `🌎️hub/📦️packages/🦀️rust/📜️script.ts` | Execute `materializeTrustedStdioGisBundle` + `validateAndPublishTrustedStdioGisCandidate` to completion; fix any failures in `produceFreshComponentV1`, `captureTrustedBootstrapCodecsV1`, GIS closed-browser-actor builder. |
| B2 | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🛡️opened-root/` | Ensure publication rotation laws pass (`trusted-catalog-opened-root-check` / `--native`). |
| B3 | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` | Keep `local-stdio-gis-open-v1` closed validation (`:1056-1085`); adjust only if profile bytes change. |
| B4 | `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` | Extend `native_openable_stdio_provider_is_the_only_atomic_readiness_transition` pattern to published bootstrap bytes (process gate). |
| B5 | `🌎️hub/🧫️fixtures/🧭️native-artifact-provider-frontier-v1/🔣️.json` | Confirm production feature set still matches 29 receipts after any stdio/GIS edits. |

**Exit:** `{dataRoot}/trusted-catalog/current.json` with profile `local-stdio-gis-open-v1`; `/readyz` openPlan true when hub started against that data root.

### Phase C — Wire production OS startup to published generation

**Goal:** ordinary launch path loads catalog without manual dev orchestration.

| Step | Files | Action |
|---|---|---|
| C1 | `🌎️hub/📦️packages/🦀️rust/🚀️bin.rs` | **Do not** eagerly bootstrap in main (keeps fail-closed semantics). Instead document/require that production launch supplies pre-published data root OR invoke a single-shot bootstrap child before exec (mirror DevScript policy without duplicating materializer in-process). |
| C2 | `🧰️framework/.../🧑‍💻dev/.../📜️script.ts` and/or combined launch seed `.vscode/🧩️launch.seed.jsonc` | Add explicit hub boot step for registered full-stack targets: set `OS_HUB_DATA`, run trusted bootstrap once, then `os-hub`. |
| C3 | `🧰️framework/.../💻️os/🟦️.ts` (browser backbone) | Replace installed-module open target validation with hub `openable_catalog` / open-plan issue when `S_HUB_URL` set. |
| C4 | `✅️acceptance-matrix.md` | Update row when production-path runtime evidence exists; replace “zero native codec bindings” with precise “no verified generation loaded unless bootstrap published”. |

### Phase D — Expand provider + profiles toward fleet coverage

**Goal:** move from 2-package / 1-target bootstrap toward “all plugins and artifacts” honestly.

| Step | Files | Action |
|---|---|---|
| D1 | `🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs` | Add `NativeCodecProviderEntryV1` rows only after each plugin owns `📇️native-codecs/🦀️.rs` with passing receipt laws (pattern: GIS, VCS). |
| D2 | `✏️s/🔌️plugins/<plugin>/📇️native-codecs/🦀️.rs` | One per candidate plugin package; do not centralize in hub. |
| D3 | `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🦀️.rs` | Shared receipt types if needed (see `📓️terra-multi-provider-verified-catalog-blueprint.md`). |
| D4 | `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs` | New profile ids with exact closed validators (copy `local-stdio-gis-open-v1` pattern); **never** load eager global binding superset — selected closure only (`:464-468` historical lesson). |
| D5 | `🧰️framework/.../🔌️plugin/📇️registry/📜️script.ts` | `catalog-complete` over explicit fresh root for all selected packages. |
| D6 | `🧰️framework/.../🌉️mcp/💡️inference/🦀️.rs`, `🏠️workspace/🦀️.rs` | Replace repo registry discovery with hub-backed catalog capability when MCP attached to authenticated document (per `📓️terra-mcp-p4-shared-map-current-acceptance-audit.md`). |

### Phase E — Acceptance gates to register in launch.json

| Gate | Script target |
|---|---|
| Provider frontier oracle | `os-hub` source check embedding `proveNativeArtifactProviderFrontier` |
| Native openable provider laws | `os-hub:native-openable-catalog-provider-check` |
| Trusted bootstrap process | `os-hub:trusted-stdio-gis-bootstrap-check` (or equivalent registered target in `📜️script.ts`) |
| Opened-root publication | `os-hub:trusted-catalog-opened-root-native-check` |
| Open plan readiness journey | `os-hub:open-plan-server-check` with real published catalog |

---

## Acceptance-matrix wording recommendation

Replace:

> Production startup still supplies zero native codec bindings, so the catalog remains empty…

With source-accurate:

> Production `os-hub` main startup loads no server-owned trusted-catalog generation from the default data root; although the default build links a 29-receipt native provider preview (stdio+gis+vcs), no codecs are registered and open-plan readiness remains false until an immutable bundle is published under `{OS_HUB_DATA}/trusted-catalog/`. Real authenticated hub document open therefore still fails closed on a clean tree.

Keep Registry row blockers (stdio Wasm ceiling, descriptor audit, `catalog-complete` absent) as separate prerequisites for **fleet-wide** catalog, distinct from the **bounded** stdio+gis bootstrap.

---

## Related ticket documents

- `📓️sol-native-openable-catalog-provider-v1.md` — provider receipt contract  
- `📓️terra-multi-provider-verified-catalog-blueprint.md` — multi-provider profile design  
- `📓️terra-all-plugins-artifacts-os-coverage-p0.md` — fleet counts  
- `📓️terra-gis-hub-stdio-full-catalog-dependency-current.md` — headless vs native feature split  
- `📓️sol-stdio-catalog-root-completion.md` — stdio row + Wasm frontier  
- `🌎️hub/🧫️fixtures/🧭️native-artifact-provider-frontier-v1/🔣️.json` — headless vs production provider frontier fixture  

---

## Audit limitations

- No runtime confirmation of `/readyz` JSON, bootstrap script success, or Cargo compiles.  
- Generated `plugins.json` not re-read byte-for-byte; fleet counts taken from cited ticket audits consistent with acceptance matrix.  
- Concurrent tree changes on stdio taxonomy may advance individual blockers after this snapshot.
