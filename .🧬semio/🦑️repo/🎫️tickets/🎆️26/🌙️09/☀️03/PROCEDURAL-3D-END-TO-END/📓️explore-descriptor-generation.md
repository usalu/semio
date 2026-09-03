# Descriptor Generation and Runtime Loading Analysis

## Executive Summary

At **RUNTIME**, the host reads descriptor files from the **dev server** (not from committed files), via HTTP fetch. The committed `🔣️.json` is authoritative for **build-time verification only**. The **descriptor hash mechanism detects stale files**, and a dev-time `check` gate enforces this.

---

## 1. Descriptor Generation: Command & Process

### Generation Command
```bash
bun ./📜️script.ts describe
```

**Location (procedural plugin):**
- `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/📜️script.ts` (lines 13-20)

**Execution Flow:**
1. The `describe` command in each plugin's `📜️script.ts` calls `describePluginComponent()`
2. Location: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📜️script.ts` (lines 100-109)
3. Implementation steps:
   - Builds the plugin's `wasm32-wasip2` component (no extra flags needed)
   - Extracts the core module using `jco transpile`
   - Runs the `semio-framework-plugin-describe` binary with:
     ```
     describe <component.wasm> --core <core.wasm> --out <ownerRoot>
     ```
   - Outputs both `🛂️.descriptor.semio` (pack format) and `🔣️.json` (readable JSON)

**Exact File Locations:**
- Descriptor emitter binary: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/🦀️.rs` (lines 1-50)
- Descriptor generator logic: Lines 422-460

---

## 2. Hashes Content & Staleness Detection

### What is in `hashes`?

Three SHA256 hashes stored in `🔣️.json`:

1. **`wasmSha256`** – Hash of the raw `wasm32-wasip2` component artifact
2. **`coreWasmSha256`** – Hash of the extracted core module
3. **`descriptorSha256`** – Self-hash of the descriptor itself (excluding this field)

**Example from procedural plugin** (`✏️s/🔌️plugins/🌀️procedural/🔣️.json`):
```json
"hashes": {
  "coreWasmSha256": "370310791b85c6cc96ea370aa50e34dea530c256abc8b606139b9049064ee71c",
  "descriptorSha256": "932ed0810ec0b2fd27381f847f343bcdacc1e5b683db7c853d8128779b568a79",
  "wasmSha256": "42503bf34bf77e69d5e730d75f9b58ce3666f42eadd4b3f810d2d80af69a96bd"
}
```

### Staleness Detection

**Build-time verification:** `📇️registry:check` command runs `validateDescriptors()` which:
- Reads each plugin's `🔣️.json` (via `readDescriptorJson()`)
- Extracts hashes from the JSON
- Finds the actually-built wasm under `target/{wasm32-wasip2,debug,release}/`
- Computes `sha256HexOfFile()` on the built artifact
- **FAILS if hashes don't match**, with message:
  ```
  ${pluginId}: hashes.wasmSha256 is ${recorded} but ${wasmPath} actually hashes to ${actual} — re-run `describe` after the latest build
  ```

**Source:** `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` (lines 1992-2060, especially lines 2033-2039)

**Verification command (repo-wide):**
```bash
bun nx run @semio-tech/plugin-registry:check
```

---

## 3. Runtime Descriptor Loading: THE CRUCIAL POINT

### At RUNTIME, the host fetches the descriptor from the dev server, NOT from the committed file.

**Location:** `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` (lines 107-130)

**Function:** `fetchDescriptorManifest(pluginId: string, moduleUrl: string, signal?: AbortSignal)`

**Exact Flow:**
1. Runtime receives plugin's JavaScript module URL (e.g., `/plugin-modules/procedural/semio_s_plugin_procedural.js`)
2. Constructs descriptor URL by replacing filename:
   ```typescript
   const descriptorUrl = path.slice(0, path.lastIndexOf("/") + 1) + "🔣️.json";
   // e.g.: /plugin-modules/procedural/🔣️.json
   ```
3. **Fetches via HTTP** (line 116):
   ```typescript
   const response = await fetch(descriptorUrl, signal ? { signal } : undefined);
   ```
4. Validates response, parses JSON, extracts `manifest` property
5. Verifies `manifest.pluginId` matches the requested pluginId

**Implication:** The dev server must serve the `🔣️.json` file at this URL for the plugin to load. If the committed descriptor is stale, the app will load the stale descriptor from wherever it's served (likely a build artifact or dev server output), not from the committed file directly.

---

## 4. Git Status: Committed, Not Ignored

**File is tracked and committed:**
```bash
git log -1 --date=iso --format='%H %ad' -- '✏️s/🔌️plugins/🌀️procedural/🔣️.json'
7ad363fd1ec91cb0c83cf716bc66522be99a4785 2026-09-03 12:49:41 +0200
```

**Size:** ~969 KB (unusually large for a manifest due to embedded contribution tables)

**.gitignore status:** NOT in `.gitignore` – the file is intentionally committed for build reproducibility and CI/CD verification.

---

## 5. Repo-wide Verification Commands

### Primary Command: Registry Check
```bash
bun nx run @semio-tech/plugin-registry:check
```

**What it does:**
- Runs the `CheckScript` class in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` (lines 2764-2810)
- Validates generated registry files are up-to-date
- Calls `validateDescriptors()` (lines 1992-2060) which:
  - Checks every plugin has a descriptor (warning if missing)
  - Verifies descriptor's packageId matches Cargo manifest
  - Verifies manifest.pluginId is correct
  - **Compares built wasm SHA256 against stored hashes** (HARD FAIL if mismatch)
  - Reports count: `descriptor gate: N/${total} crates have a 🔣️.json`

### Generation Command
```bash
bun nx run @semio-tech/plugin-registry:generate
```

Regenerates the registry from source; does NOT directly regenerate descriptors (each plugin runs its own `describe` command).

### Per-Plugin Regeneration
```bash
cd ✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust
bun ./📜️script.ts describe
```

---

## Key Design Insights

1. **Committed descriptor is a checkpoint, not source-of-truth.** The Rust plugin defines its manifest in code; the descriptor is extracted via `describe()` export.

2. **Hash mechanism is defensive.** It catches stale committed descriptors that would silently change app behavior if the committed file were the only source at runtime.

3. **Runtime isolation.** Plugins load their descriptor via HTTP from the dev server (in dev shell) or from the published artifact URL (in production), not by reading filesystem paths directly. This allows descriptors to be versioned separately from plugin code.

4. **Build-time verification is strict.** The `check` gate runs as part of CI/CD and fails hard if:
   - Descriptor exists but its wasm hash doesn't match the built artifact
   - This forces regeneration before any changes are committed

5. **No silent staleness.** A dev who edits the plugin but forgets to re-run `describe` will see build failures at the registry check stage, not silent behavior changes.

---

## References

- Descriptor emitter Rust code: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/🦀️.rs` (lines 1-50, 422-460)
- Registry validation: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` (lines 1992-2060)
- Runtime fetch: `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` (lines 107-130)
- Descriptor structure: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts` (lines 23-70)
- Procedural plugin descriptor: `✏️s/🔌️plugins/🌀️procedural/🔣️.json` (committed, last updated 2026-09-03)
