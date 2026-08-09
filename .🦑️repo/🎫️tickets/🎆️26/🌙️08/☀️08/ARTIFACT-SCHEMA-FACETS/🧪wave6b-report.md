# Wave 6b — TypeScript Projection → Snapshot twin

## Inventory (`🧪wave6b-inventory.txt`)

- **970** lines matching `[Pp]rojection` across `*.ts` / `*.tsx` excluding `compose/**`.
- Top files (all camera/world UI unless noted):
  - `♾️infinite/🌍️world/🎨️r3f/🟦️component.tsx` — 416 (camera taxonomy)
  - `World3dHost/🟦️component.tsx` — 91
  - `cad/📺️renderer/🟦️component.tsx` — 43
  - `ui/🎬️Scene/🟦️component.tsx` — 33
  - `framework-renderer-react` `🧪️index.test.ts` — 21 (camera/world)
  - `📜️script.ts` — 21 (policy; updated this wave)
  - `ShellHost` / `ShellHelpers` / `ChromePanels` — world projection templates
- **Zero** hits for `WriterProjection`, `LowpolyProjection`, `CadProjection`, etc. in TS.
- **Zero** `Projection` in `🧬️schema/🟦️component.ts` leaves (framework + plugins); snapshot facets already use `XSnapshot`.

## Renamed (document-state / policy / kernel docs)

| Location | Change |
| --- | --- |
| `📜️script.ts` `PolicyDocumentAppUsage` | `projectionType` → `snapshotType` |
| `📜️script.ts` `policyDocumentAppUsages` | parses `type Snapshot = …` (was `type Projection = …`) |
| `📜️script.ts` `policyDslCompletenessBreaches` | labels/reasons/solutions use `DocumentApp::Snapshot` |
| `📜️script.ts` allowlist comments | `DemoProjection` → `VcsSnapshot`; alias examples `RasterSnapshot` |
| `📜️script.ts` command-envelope / mutation-impl solution strings | `Projection` type param → `Snapshot` |
| `🎠️kernel/🟦️component.ts` ephemeral lane docstrings | “draft projection” → “draft snapshot” |

No renderer/UI/camera identifiers renamed (correct).

## Deliberately left as “projection”

1. **3D camera / world** — `WorldProjectionSpec`, `worldProjectionDefaults`, `projectionSpec`, `registerPendingWorldProjection`, `pendingProjections`, `OrbitCameraProjection`, `updateProjectionMatrix`, `SceneProjectionKind`, `ui.host.projection` labels, icon keys `projection.*`, CAD renderer orbit overrides, etc.
2. **CQRS / db** — not present in TS inventory targets this wave.
3. **GIS reprojection** — no TS hits in scope.
4. **SQL-style** — `cad/🔍️query` `projections` field (query algebra).
5. **Shooting** — `ShootingCamera.projection` string (camera mode on saved camera).
6. **Metaphor in policy** — pack/command-envelope comments still say “projection of the same value model” (not document-state noun).
7. **xstate** — `SnapshotFrom` re-export (library type).

## Files edited

- `📜️script.ts`
- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts` (docstrings only)

## Gates (verbatim tails)

### `bun nx run @semio-tech/framework-renderer-react:test`

```
 RUN  v4.1.10 /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react

[budget] /Users/ueli/.bun/bin/bun /Users/ueli/Documents/semio/node_modules/vitest/vitest.mjs run --config 🧪️vitest.config.ts --passWithNoTests --testTimeout 15000 --hookTimeout 15000 --teardownTimeout 15000 exceeded 15000ms — killed. Trim it, or assign it to a higher level (quick/long/exhaustive).
Warning: command "bun ./📜️script.ts test" exited with non-zero status code


 NX   Running target test for project @semio-tech/framework-renderer-react failed

Failed tasks:

- @semio-tech/framework-renderer-react:test

Hint: run the command with --verbose for more details.
```

**Note:** Direct `vitest` (no wall-clock budget wrapper) completes in ~9–10s with **6 failing tests** (mit-bestand logo path `logo` vs `logos`, `ring-primary`, VFS host, fault boundary, etc.) — unrelated to Snapshot rename. Also ran `nx run @semio-tech/framework-renderer-react:test-exhaustive`: budget OK, same assertion failures.

### `bun nx run @semio-tech/plugin-registry:check`

```
  - 🪵️sourcing: 🧩️extensions/🪵️beams/🦀️component.rs is not declared by any #[path] in 📦️glue.rs
  - 🪵️sourcing: 🧩️extensions/🧱️slabs/🦀️component.rs is not declared by any #[path] in 📦️glue.rs
  - 🪵️sourcing: 🧩️extensions/🪟️windows/🦀️component.rs is not declared by any #[path] in 📦️glue.rs
Warning: command "bun ./📜️script.ts check" exited with non-zero status code


 NX   Running target check for project @semio-tech/plugin-registry failed

Failed tasks:

- @semio-tech/plugin-registry:check

Hint: run the command with --verbose for more details.
```

(Rust `📦️glue.rs` — wave-6a / sibling agent scope.)

### `bun ./📜️script.ts policy` (tail)

```
[DEBUG] runPolicyScript starting for /Users/ueli/Documents/semio/📜️script.ts
[DEBUG] runPolicyScript parsing policy file export
[DEBUG] runPolicyScript resolving folder/bundle entity
[DEBUG] runPolicyScript importing module dynamically from url /Users/ueli/Documents/semio/📜️script.ts
[DEBUG] runPolicyScript imported module successfully
[DEBUG] runPolicyScript invoking policy function for kind technology
```

Exit **0** after Snapshot-aware `policyDocumentAppUsages` update.

## Could not validate / blocked

- **plugin-registry:check** — sourcing plugin Rust glue paths (not TS).
- **framework-renderer-react:test** at default `fundamental` level — wall-clock budget (15s) kills vitest before completion on this host; exhaustive level runs but tests fail on pre-existing assertions.
- **Runtime** document-store Snapshot APIs in TS — already aligned; no remaining document-state `Projection` types found in TS/TSX.
