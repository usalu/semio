# Lane B — descriptor producer, extension `describe` route, fail-closed registry gates

Ticket `26/09/05/S-END-TO-END` · lane B (`descriptor-producer`) · Opus 5 · 2026-09-05 04:10–05:15.

Scope delivered: (1) the owner descriptor receipt contract, (2) the shared extension `describe` route + 26 owner registrations, (3) fail-closed registry `check` gates including the `interactiveJob` classification-drift gate, (4) tests, (5) the re-emission census. Task 3's discovery-walk half (`target*` skipping / `ENOENT` tolerance in `📚️library/🔍️discovery/🟦️.ts`) was taken by the coordinator at 04:20 and is **not** in this packet; it had already landed when I ran `check`, which now completes in seconds instead of crashing after 20 minutes.

## 1. State of the art before this packet

The audit's line anchors were stale. A peer had already split the one-file producer: `buildPluginComponent` / `extractPluginCore` / `emitPluginDescriptor` existed, and the Rust emitter (`🔌️plugin/🖨️describe/📦️packages/🦀️rust/🦀️.rs:308-312`, `:326-360`, `:424-462`) already hashed raw and core independently, refused equal raw/core hashes, blanked exactly `descriptor_sha256` for its two-pass self hash, wrote the pair as a rollback-safe transaction, and refused the `assembly-failed` placeholder. Note the directory is `🖨️describe`, not the audit's `📇️describe`.

What was still missing at the TypeScript boundary, and is what I built: an explicit `(rawComponentPath, extractedCorePath, ownerRoot)` entry point (the old `describePluginComponent` always built the component itself, so a caller could never supply two independently produced artifacts), input validation (non-regular / symlink / out-of-root / same-file / same-hash), a bounded deadline + cancellation, verification of the emitted pair *before* any owner byte moves, and a receipt.

## 2. Changes

### 2.1 Owner descriptor receipt contract — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts`

| Anchor | What |
|---|---|
| `:115-141` | `DescriptorEmissionRequestV1` (`rawComponentPath`, `extractedCorePath`, `ownerRoot`, optional `artifactRoot`), `DescriptorEmissionReceiptV1`, `DescriptorEmissionControlV1` (`cancelled`, `deadlineMs`, `checkpoint`). |
| `:143-147` | `emissionGuard` — one cancellation + deadline + checkpoint per stage (`validate`, `emit`, `describe`, `verify`, `publish`). |
| `:149-156` | `emissionDirectory` — rejects a symlink or non-directory owner/artifact root and anything resolving outside the repository. |
| `:158-181` | `emissionArtifact` — rejects symlink/non-regular/empty/over-64 MiB inputs and anything outside the declared `artifactRoot`, then chunk-hashes SHA-256 and re-stats to reject a file that changed mid-hash. |
| `:183-234` | `emitOwnerDescriptorPairV1` — the contract. Validates both inputs, refuses `raw === core` by path **and** by SHA-256, emits into a staging dir created inside `ownerRoot`, verifies the emitted pair against the two hashes it just computed, and only then renames both forms onto the owner root. Any failure leaves the previous pair untouched; the staging dir is always removed. |
| `:404-421` | `describePluginComponent` rewired onto the contract: builds, extracts the core into a scratch dir **inside the cargo target root** (previously `tmpdir()`, which the containment rule now forbids), emits, prints the receipt, returns 0/1. |
| `:425-428` | `describeExtensionComponent(repoRoot, rsDir)` — the shared extension route. |
| `:12-19` | imports + `DESCRIPTOR_PACK_FILENAME` / `DESCRIPTOR_JSON_FILENAME` constants (`tmpdir` import dropped). |

Deliberate: the emitter binary writes into staging, never into the owner root, so the "all checks pass first" property does not depend on the Rust side. The two `renameSync` calls are one syscall apart — POSIX has no atomic two-file rename — and a torn pair is now a hard `check` error rather than a silent half-publish.

### 2.2 Neutral pair verifier — `🔌️plugin/📇️registry/📜️script.ts`

| Anchor | What |
|---|---|
| `:2428-2434` | `CatalogDescriptorIdentity` — the narrow identity `validateCatalogDescriptorValue` actually needs, replacing a `PluginRegistryEntry` cast. |
| `:2474-2475` | `CATALOG_PLACEHOLDER_PLUGIN_IDS` (`empty`, `assembly-failed`, `unknown`, `placeholder`) and `CATALOG_PLACEHOLDER_VERSION` (`0.0.0`). Verified safe: every committed real descriptor is `0.1.0` (workspace version, `Cargo.toml:144`). |
| `:2490-2492` | `rejectPlaceholderCatalogIdentity`. |
| `:2496-2523` | `verifyDescriptorPairBytesV1(jsonBytes, packBytes, {wasmSha256, coreWasmSha256})` — derives identity from the descriptor itself (so it serves plugins and extensions alike), strict-decodes both forms, requires the hashes to name the exact artifacts, requires semantic JSON/pack equality after enum normalisation, requires canonical pack re-encoding with no trailing bytes, re-derives the blanked self hash, and rejects a placeholder identity. |
| `:2525-2534` | `verifyFreshCatalogPackageV1` now delegates to it and only adds the fresh-package identity/execution equality, so there is exactly one implementation of the pair contract. |

### 2.3 Fail-closed descriptor gate — `🔌️plugin/📇️registry/📜️script.ts`

| Anchor | What |
|---|---|
| `:1989-1991` | `INTERACTIVE_JOB_RUST_NAMES` (camelCase mirror of `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:837-845`), the `action_interactive_job` call regex, and the walk's skip set (`🧩️extensions` excluded so a host's map is not polluted by its children). |
| `:1996-2016` | `rustInteractiveJobClassifications(ownerRoot)` → `actionId → Set<classification>`; a set, because one id legitimately carries different dispositions on editor vs viewer. |
| `:2018-2035` | `auditInteractiveJobClassificationDrift(pluginId, ownerRoot, descriptor)` — reports an action only when the committed `interactiveJob` is explicit **and** the owner's own Rust declares that id **and** the committed value is not in the declared set. |
| `:2037-2050` | Rewritten gate docstring: what is now an error and why the old warn-only split no longer matches the strict catalog gate. |
| `:2051-2090` | `validateDescriptors` (now exported so it can be probed and tested without the whole `check` prelude): a missing or half-present owner pair is an **error**; every present pair goes through `validateCatalogDescriptorPair` (which is where JSON/pack divergence, packageId/pluginId/role/host mismatch, non-canonical packs and self-hash mismatches surface), then `rejectPlaceholderCatalogIdentity`, then the classification-drift audit. Redundant hand-rolled packageId/pluginId re-reads deleted. The one remaining **warning** is "no `wasm-release` publication artifact" — publication identity belongs to `catalog-complete` against a dedicated fresh root, not to whatever ambient `target/` holds. |

`generate` is untouched and stays permissive (`GenerateScript` → `renderCatalogFiles` never calls `validateDescriptors`), so `dev s` still boots against a partially described catalog.

**Why the narrow drift rule.** I measured both rules over all 29 owners that have a committed `🔣️.json` (probe: `🔨️classification-drift-probe.py` in this folder). A bidirectional rule produces **742** findings — dominated by two false-positive families: shared framework-builder actions (`copy`, `cut`, `clearSelection`, `checkoutCheckpoint`, …) that no plugin `.rs` declares, and descriptors that simply predate a source migration. The narrow rule produces **206** findings across exactly **4** owners, every one a genuine contradiction: `animate` 13, `fem` 64, `puzzle` 97 (descriptor claims `migrated`, source says `batchOnlyPendingRewrite`), `sourcing` 32 (the reverse — the known `SOURCING-END-TO-END` symptom).

### 2.4 Shared extension `describe` route — 26 owners

`parseExtensionCargoManifest` exported from `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts:3253` (with a docstring). `runExtensionComponentPackage` is untouched and still owns only the runtime `.sxt`; nothing infers a descriptor from a `.sxt` or copies one from a host.

52 files changed by `🔨️register-extension-describe.ts` (kept in this ticket folder with its input list `🧩️extension-owners.txt`): each owner's `📜️script.ts` gained the `describeExtensionComponent` import, a `DescribeScript`, `.register("describe", DescribeScript)` and an updated header; each `📋️project.json` gained a `describe` target mirroring the plugin convention exactly (`executor: nx:run-commands`, `cwd`, `bun ./📜️script.ts describe`, `forwardAllArgs: true`).

Owners: flow ×9 (bim, brep, dictionary, draw, list, logic, math, primitive, text), imperative ×5 (control, effect, logic, math, text), process ×4 (concrete, metal, robotic, wood), sourcing ×3 (beams, slabs, windows), cad ×4 (aec-building, aec-building-energy, aec-building-structure, spatial-shape), playbook ×1 (procedural). `🌊️flow/🧩️extensions/🧪️fixtures` is not a crate (it holds only a `🔣️.json`) and is not one of the registry's 26 extension rows.

**`.vscode/launch.json`: intentionally not touched.** The brief conditions this on the existing convention. `grep -c describe .vscode/launch.json` → `0`: none of the 33 existing plugin `describe` targets is registered there, so mirroring the convention means registering nothing.

### 2.5 Tests — `🔌️plugin/📇️registry/✅️catalog-complete.test.ts`

Three new neutral cases, all with a WebCrypto (`webcrypto.subtle.digest`) oracle alongside the `node:crypto` path:

- `:315` *verifies one emitted owner pair against its exact raw/core bytes with a WebCrypto oracle* — distinct raw/core bytes yield distinct hashes; a swapped core fails; a divergent JSON/pack pair fails; a tampered self hash fails; trailing pack bytes fail; both placeholder shapes (`pluginId: "empty"`, version `0.0.0`) fail; an extension keeps its declared host as dependency zero and a hostless extension fails.
- `:360` *refuses every unusable emission input and never half-publishes an owner pair* — 8 rejection cases (outside the artifact root, empty artifact, symlinked artifact, file-as-owner-root, same file twice, byte-identical raw/core, cancellation, exhausted deadline) each asserting the previous owner pair is byte-identical afterwards; plus a checkpoint that aborts after `emit` and proves the stage order `["validate", "emit"]` and an untouched owner root.
- `:404` *reports only committed classifications that contradict the owner's own Rust* — agreeing, per-surface-disagreeing-but-declared, unclassified, absent-from-Rust and `🧩️extensions`-scoped declarations are all silent; exactly the one true contradiction is reported.

## 3. Commands and real output

```
$ cd /Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-s-e2e-b \
  bun …/vitest.mjs run --config 🧪️tests/🟦️.ts --testTimeout 240000 --reporter=verbose ✅️catalog-complete.test.ts

 ✓ handpicked module deployment directories > admits only schema-owned module routes …            1016ms
 ✓ handpicked module deployment directories > uses the declared authored bridge …                  121ms
 ✓ handpicked module deployment directories > matches the schema and independent emoji oracle …    492ms
 ✓ strict plugin catalog completion > validates the neutral contract and withholds …               174ms
 ✓ strict plugin catalog completion > rejects max+1, duplicates, missing parents and cycles …       60ms
 ✓ strict plugin catalog completion > strict-decodes both descriptor forms …                       533ms
 ✓ strict plugin catalog completion > refuses ambient roots and detects the exact artifact max+1 …  19ms
 ✓ strict plugin catalog completion > verifies one emitted owner pair against its exact raw/core …  409ms
 ✓ strict plugin catalog completion > refuses every unusable emission input and never half-…         50ms
 ✓ strict plugin catalog completion > reports only committed classifications that contradict …      14ms
 ✓ strict plugin catalog completion > independently enumerates the 59 real manifests …            75328ms

 Test Files  1 passed (1)
      Tests  11 passed (11)
```

`bun nx run @semio-tech/plugin-registry:test -- --run` cannot be used as-is: the default level's 15 s budget kills the run (`[budget] … exceeded 15000ms`) because the 59-manifest audit case alone takes ~75 s. That budget mismatch is pre-existing and is lane A's `test-quick` honesty item.

Whole-project vitest run (`3 files, 19 tests`): **17 passed, 2 failed**, both in `🚀️launch.test.ts` and both pre-existing, neither touched by this packet (`git diff HEAD -- 🚀️launch.test.ts` is empty):

- *exposes every owned generator preview exactly once in contract order* — 16 discovered generator contracts vs 15 expected. `🔣️taxonomy.json` is `+314/−157` in the working tree from a concurrent peer; this is that edit.
- *keeps generated native, root preflight, and MCP runtime profiles identical without debug* — asserts the describe `📜️script.ts` contains the literal `"wasm32-wasip2", "wasm-dev"`. `git show HEAD:…/🖨️describe/…/📜️script.ts | grep '"wasm32-wasip2", "wasm-dev"'` returns nothing, so the assertion was already failing at HEAD; the peer refactor that introduced `buildPluginComponent` dropped the literal. Its sibling assertion (`--target", "wasm32-wasip2", "--profile", "wasm-dev"` exactly twice) still holds. **Not patched deliberately** — satisfying a string-match policy gate by re-injecting a literal I do not own would hide a real regression; flagging it for whoever owns that refactor.

### `check`

```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/target-s-e2e-b bun nx run @semio-tech/plugin-registry:check --skip-nx-cache
plugin registry catalog is stale: .vscode/launch.json
run `bun nx run @semio-tech/plugin-registry:generate` to refresh.
exit=1
```

The walk no longer crashes — `check` now returns in seconds (coordinator's 04:20 fix) — but it stops at the staleness prelude because `.vscode/launch.json` is dirty in the working tree from a peer. I did **not** run `generate`, which would rewrite `🤖️generated/*` and `.vscode/launch.json` over that peer's in-flight edit.

So I ran the new gate directly instead:

```
$ bun -e 'const {generatePluginRegistry, validateDescriptors} = await import("…/📇️registry/📜️script.ts");
          const {warnings, errors} = validateDescriptors(generatePluginRegistry(repoRoot), repoRoot); …'
descriptor gate: 1/59 crates own a verified 🔣️.json + 🛂️.descriptor.semio pair.
entries=59 errors=58 warnings=1

32   stale descriptor: no packageId field
19   missing owner pair
 4   role/version mismatch (the CAD extension placeholders)
 2   pack does not decode ("wire frame varint: truncated")
 1   descriptor exceeds 4 MiB
TOTAL 58
```

The single warning is `cad: has hashes.wasmSha256 but no canonical wasm-release publication artifact`.

**The headline finding is bigger than the audit's.** The audit reported 8 semantic JSON/pack divergences; the real number is **32 owners whose committed `🔣️.json` has no `packageId` field at all** — they predate its addition to `PackageDescriptor`. Verified directly:

```
$ … writer entry.packageId = "semio:writer"   descriptor.packageId = undefined
```

Only `cad` — re-described 2026-09-05 02:31 — carries one. So the true state is **1/59 valid owner pairs**, not 28; the other 58 are missing (19), stale-schema (32), placeholder (4), corrupt-pack (2: `flow-extension-brep`, `flow-extension-math`) or over the 4 MiB descriptor bound (1: `puzzle`). Every one of them must be re-emitted through the new contract.

Classification drift contributes **0** errors today only because all four drifting owners (`animate`, `fem`, `puzzle`, `sourcing`) fail the pair-validity check first and are skipped — a valid pair is a precondition for auditing its contents. The gate engages the moment those pairs are re-emitted; its behaviour is proven by the unit test rather than by the current corpus.

## 4. Re-emission census

Only **one** raw `wasm32-wasip2` component exists anywhere under the repo:

```
$ for d in target target-block target-block-3d target-block-io target-demonstrator \
           target-demonstrator-dev target-gen3d target-p3d-agentE target-p3d-e2e \
           target-s-e2e target-sourcing-e2e; do ls "$d"/wasm32-wasip2/*/*.wasm | wc -l; done
… all 0 except:
target-demonstrator-dev/wasm32-wasip2/wasm-dev/semio_s_plugin_cad.wasm   46557344 bytes  Sep 5 02:31
```

The dev module cache holds only jco-extracted `*.core.wasm`, never a raw component, so no other owner can be re-emitted without a wasm build — which this lane was told not to do.

For `cad` I verified the committed pair is already exactly the receipt the new contract would produce, so re-emitting it is a provable no-op:

```
$ shasum -a 256 target-demonstrator-dev/wasm32-wasip2/wasm-dev/semio_s_plugin_cad.wasm
919ca3b975a3d0786fd750f95f138b65239223402b91fd765379edc3cea1bb42
$ jq .hashes ✏️s/🔌️plugins/📐️cad/🔣️.json
wasmSha256      919ca3b975a3d0786fd750f95f138b65239223402b91fd765379edc3cea1bb42   ← identical
coreWasmSha256  d884d1a39ca11fd8f82249bd71ac50075bd49e59d6f3b214db1556656a9f4aa0
$ find ✏️s/🔌️plugins/📐️cad -name '*.rs' -not -path '*/🧩️extensions/*' -newer <that wasm> | wc -l
0
```

and I re-extracted the core independently with `extractPluginCore` (jco, no cargo):

```
core sha256 = d884d1a39ca11fd8f82249bd71ac50075bd49e59d6f3b214db1556656a9f4aa0   ← identical
```

So `cad`'s owner pair is confirmed fresh against **both** artifacts, no `.rs` is newer than the component, and the gate already reports it as the 1/59 verified pair.

**Re-emitted: none.** **Left for wave 2 (all 58):** the 19 missing pairs (`block`, `playbook`, `stdio`, `trinity` + the 15 extensions), the 32 stale-schema pairs (`animate`, `architect`, `dag`, `demonstrator`, `draw`, `energy`, `fem`, `flow`, `flow-extension-{dictionary,list,logic,primitive,text}`, `forms`, `gis`, `imperative`, `layout`, `lowpoly`, `mathematical`, `norm`, `note`, `procedural`, `process`, `raster`, `reasoning-mindmap`, `remodel`, `s`, `sequence`, `shooting`, `sourcing`, `vcs`, `writer`), the 4 CAD placeholders, the 2 corrupt packs, and `puzzle`.

I did not build the `semio-framework-plugin-describe` emitter binary: no target root holds one, a cold build of it (wasmtime + framework) is a 10–60 min job the brief tells me to avoid, and the coordinator's 04:45 log records `semio-framework-os-kernel` E0432 blocking every Rust check on both shared targets. The contract's Rust-facing leg is therefore verified by construction and by the guard-rail tests, not by an end-to-end emitter run.

## 5. Remaining blockers

1. **`check` cannot reach green until `generate` is re-run** by whoever owns the dirty `.vscode/launch.json` / `🔣️taxonomy.json` working-tree state.
2. **58 owner pairs must be re-emitted** through `describe`, dependency-first, once `stdio` compiles and the emitter binary can be built. 32 of them are not "divergent" but schema-stale (`packageId` absent) — the audit under-counted this by 24.
3. **`puzzle`'s `🔣️.json` exceeds `CATALOG_DESCRIPTOR_MAX_BYTES` (4 MiB).** Re-emission alone will not fix it; either the bound or `puzzle`'s 129-action descriptor has to change. This is a hard gate stop with no owner yet.
4. **`flow-extension-brep` and `flow-extension-math` packs are truncated** (`wire frame varint: truncated`) — corrupt bytes, not drift; likely fallout of a repo-wide codemod. Re-emit, do not repair.
5. **206 classification contradictions** across `animate` (13), `fem` (64), `puzzle` (97), `sourcing` (32) will surface as gate errors the moment those pairs are valid. They are source-side fixes (or genuine descriptor staleness), not producer bugs.
6. **`🚀️launch.test.ts` WASI profile-policy assertion** on the describe script is failing at HEAD and needs its owner (the `buildPluginComponent` refactor) to restore the literal or update the policy.
7. The registry `test` target's 15 s budget cannot hold the 59-manifest audit case (~75 s). Either the case moves to `test-long` or the walk gets faster.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-complete.test.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`
- 26 × `✏️s/🔌️plugins/<host>/🧩️extensions/<ext>/📦️packages/🦀️rust/📜️script.ts`
- 26 × `✏️s/🔌️plugins/<host>/🧩️extensions/<ext>/📦️packages/🦀️rust/📋️project.json`
- ticket inputs: `🔨️register-extension-describe.ts`, `🧩️extension-owners.txt`
