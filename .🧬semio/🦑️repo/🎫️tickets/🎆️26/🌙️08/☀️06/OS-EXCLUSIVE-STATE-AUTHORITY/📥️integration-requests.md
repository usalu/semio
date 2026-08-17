# Integration Requests (append-only)

**Audience:** Integrator agent (owns root `Cargo.toml`, `Cargo.lock`, `📜️script.ts`, nx/eslint/dependency-cruiser, `launch.json`).

**Format:** Append new sections at the **bottom**. Never edit or delete prior entries.

```markdown
## YYYY-MM-DD — <wave/agent id> — <short title>

**Why:** one sentence

**Files / globs:**
- `path/or/glob` — what to change

**Exact ask:**
- [ ] bullet list of concrete edits (members, deps, scripts, launch config)

**Depends on:** ticket/wave ids or "none"

**Status:** open | applied | rejected — <note>
```

---

<!-- entries below this line -->

## 2026-08-06 — Wave 1a M2 Engine — ArtifactKind::Engine

**Why:** Engine derive/read WIT imports need a capability gate; `ArtifactKind` in framework core has no `Engine` variant yet.

**Files / globs:**
- `🧰️framework/🔨️modules/🧩core/🧩️ui/🧠️kernel/🦀️component.rs` — `ArtifactKind` enum

**Exact ask:**
- [ ] Append `Engine` variant to `ArtifactKind` (after `Backbone`), so host can grant `CapabilityRequirement { artifact: ArtifactKind::Engine, rights: Invoke|Read, … }` for `engine-derive` / `engine-read`.
- [ ] Do **not** change OS kernel `Cargo.toml` for blake3/thiserror — already present and used by engine via glue path-include.

**Depends on:** Wave 1b WIT imports (`engine-derive` / `engine-read`)

**Status:** applied — ArtifactKind::Engine added 2026-08-06; WIT engine-derive/engine-read added.

## 2026-08-06 — Wave 3 Integrator — Policy functions landed (flag-gated)

**Why:** Prepare Wave 3 enforcement without failing the default verify/policy gate while Wave 2 migrations remain open.

**Files / globs:**
- `📜️script.ts` — `policyOsStateAuthorityBreaches`, `policyDocumentAppShapeBreaches`, gated registration
- `$TICKET/🧪w3-enforcement-draft.md` — dep-cruiser / eslint / launch.json / verify-gate snippets (not applied)

**Exact ask:**
- [x] Implement both policy functions fully in root `📜️script.ts`
- [x] Register behind `SEMIO_OS_STATE_AUTHORITY=1` (102 OS breaches today — would fail CI if unconditional)
- [ ] After Wave 2 clears breaches: remove the env gate and apply snippets from `🧪w3-enforcement-draft.md`
- [ ] Wire `VerifyScript.gate` + launch.json only after zero-breach flip

**Depends on:** Wave 2 migrations complete (zero `os-state-authority/*` breaches)

**Status:** applied — policy functions + flag gate in `📜️script.ts`; root dep-cruiser/eslint/launch/verify snippets drafted only

## 2026-08-06 — Wave 2 🧊3d — Brep document ops + host injection

**Why:** `BrepDocumentOpEngine` is a stub; CAD/process still use `OnceLock<BrepEngineHost>` until `DocumentApp::handle` receives `EngineHost`.

**Files / globs:**
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**` — M3 `EngineHandles` / `EngineHost` injection into guest `handle`
- `✏️s/🔨️modules/🧊️3d/📐️brep/⚙️engine/🖥️host/🦀️component.rs` — implement opcode pack in `BrepDocumentOpEngine::compute`

**Exact ask:**
- [ ] Pass `&dyn EngineHost` (or `EngineHandles`) into CAD/process compute paths; delete `CAD_BREP_HOST` / `PROCESS_BREP_HOST` `OnceLock`s
- [ ] Incremental brep: `derive(BREP_ENGINE_ID, pack(parent_engine_handle, op))` with serialized kernel snapshot in cache value

**Depends on:** Wave 1b WIT `engine-derive` / `engine-read` (status: applied per prior entry)

**Status:** open — Wave 2 🧊3d landed host wrapper + content-addressed handles; see `🧪w2-3d-engine.md`

## 2026-08-06 — Wave 2 TypeScript — OS chrome document + ui-react vitest

**Why:** Wave 2 TS moved chrome compute/intro prefs to `StoragePort`; durable chrome should eventually live in OS config document, not browser storage. `ui-react` vitest is blocked by a duplicate `Cursor` export.

**Files / globs:**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs` — `ChromePrefsState` / chrome persistence
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📦️index.tsx` — duplicate `export { Cursor }` (~7937)

**Exact ask:**
- [ ] Wire `scope.storage` chrome reads/writes to OS chrome document projection (replace long-term `StoragePort` → `localStorage` for `ui.chrome.*` / `ui.compute.*` / `ui.introduction.seen.*`)
- [ ] Remove duplicate `Cursor` re-export so `@semio-tech/ui-react:test-quick` can run

**Depends on:** `🧪w2-typescript.md`

**Status:** open

## 2026-08-06 — Wave 2 plugins — Home studio port registry + play-app session lanes

**Why:** `HomeApp` still uses process-wide `OnceLock<Arc<Mutex<studio_ports>>>`; draw/flow/procedural play apps keep `Mutex` session fields instead of config/host lanes.

**Files / globs:**
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/**` — `SHomeDocument` / `SHomeOperation` for `space_id → folder_path` bindings
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🏠️home/**` — remove `shared_studio_ports`; ZST `HomeApp`; resolve ports from document bindings
- `✏️s/🔌️plugins/🪐️space/🎛️apps/🪐️space/**` — pass home projection into `resolve_studio_document` (or read bindings from `SpaceConfig`)
- `✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/🎚️config/**` — `gesture_session_json` + ops; ZST `DrawPlayApp`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/**` — `FlowEvalSession` host ownership / remove `FLOW_SESSION_GEOMETRY` process mutex

**Exact ask:**
- [ ] Persist folder-backed studio ports in home document ops, not `HomeApp::default()` + global registry
- [ ] Host injects `FlowEvalSession` (or serializable eval baseline) per document session; delete `Mutex<FlowEvalSession>` on flow/procedural play apps
- [ ] `presence_peers` on `SpaceApp` → `SpaceConfig` registry ops

**Depends on:** `26/08/06/OS-EXCLUSIVE-STATE-AUTHORITY` Wave 1b `DocumentSession`

**Status:** open — see `🧪w2-plugins-globals.md`


## 2026-08-06 — Wave 2 framework — MapHost feature tables + surface cargo

**Why:** `MapHost` GIS tables moved to `host.features` (`MapFeatureTables`); integrators must not assume flat `host.positions` fields.

**Files / globs:**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/**` — any direct `MapHost` field access
- Root `Cargo.toml` / workspace — unblock `cargo check -p semio-framework-surface` after `semio-s-3d` brep handle fix

**Exact ask:**
- [ ] Use `MapHost::sync_*` / `host.features.positions` (not removed flat fields) in EngineCanvas / Scenes if any call sites still use old shape
- [ ] Restore `GeometryHandle::content_addressed` (or align kernel) so `semio-s-3d` compiles and surface crate checks in CI

**Depends on:** `🧪w2-framework.md`, Wave 2 🧊3d engine ticket

**Status:** open

## OS host: store_sync / space / workflow remount (2026-08-06)
- `semio-framework-os` host_core still imports dissolved crates `store_sync`, `space`, `workflow`.
- Implementations live under `🛍️products/💻️os/🔨️modules/{🏪️store/🔄️sync,🪐️space,🔁️workflow}` and expect `crate::os_*` (kernel).
- Registrar must path-mount them into `semio-framework-os-kernel` glue and re-export; host then `use store::…` / kernel aliases. Do not mount into host facade.
- Host Cargo.toml needs tokio/time, futures_util, tokio-tungstenite, notify, zip if sync/space stay host-side (not recommended).

## 2026-08-06 — Wave 3 Integrator — Zero-breach flip applied

**Why:** `policyOsStateAuthorityBreaches` + `policyDocumentAppShapeBreaches` report 0; enforce unconditionally.

**Files / globs:**
- `📜️script.ts` — remove `SEMIO_OS_STATE_AUTHORITY` gate; verify gate asserts OS policies only (full `policy` CLI still has unrelated breaches)
- `.dependency-cruiser.cjs` — `no-state-outside-os`
- `eslint.config.mjs` — OS state authority TS rules (module `let`/`var`, empty Map/Set, `*Store` classes, browser storage globals); framework-core StoragePort/ephemeral lane allowlisted
- `.vscode/launch.json` — `⚖️policy`, `✅verify🎛gate` under `4_build` 900/901

**Exact ask:**
- [x] Remove env gate
- [x] Verify gate OS authority assertion
- [x] dep-cruiser / eslint / launch entries

**Depends on:** Wave 2 authority-struct-map rename + TS ephemeral lane migration

**Status:** applied 2026-08-06
