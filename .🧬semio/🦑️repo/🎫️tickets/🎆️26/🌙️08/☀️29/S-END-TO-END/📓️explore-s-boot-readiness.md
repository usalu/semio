# S App Boot Readiness Investigation

**Date:** 2026-08-30

## 1. Generated Plugin Catalog & Playgrounds

**Status:** Both files exist and recently regenerated.

- **File locations:**
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔣️plugins.json` (41.4 KB)
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🟦️playgrounds.ts` (19.3 KB)

- **Last regenerated:** 2026-08-30 09:13 (both updated together)

- **Staleness check:** Both are in `.gitignore` (`**/🤖️generated/`), confirming they're generated artifacts, not source-controlled. Timestamps show they were regenerated at 09:13 today, indicating a recent build. **NOT stale.**

## 2. Plugin Modules Directory (`plugin-modules/`)

**Location:** `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/`

**Contents:** 58 materialized plugin modules across multiple plugin IDs:
- Most recent: Aug 30 09:14 (`cad`, `demonstrator`, `flow`, `gis`, `procedural`, `process`, `puzzle`, `sourcing`)
- Oldest available: Aug 6-7 (framework engine shims `_vendor`, `_shard`)
- `.hot-swap` marker: 2026-08-30 02:42 (last build notification)

**Status:** Fully materialized and current, no staleness.

## 3. Plugin Build & Dev Server Error Handling

**Code location:** `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts:1093-1131`

**Behavior on plugin failure:**

```typescript
async function buildPluginCatalog(...): Promise<{ readonly failedPluginIds: readonly string[] }> {
  for (const target of orderedTargets) {
    try {
      cargoResult = await cargoFn(target);
    } catch (error) {
      failed.push(target.pluginId);
      console.error(`plugin build failed, continuing with remaining targets: ${target.pluginId}`, error);
      continue;  // ← Does NOT abort
    }
    // Materialize...
  }
  return { failedPluginIds: failed };
}
```

**Dev server does NOT abort on plugin failures:**
- Line 1971: `runViteBunxDev()` spawned without await (dev server starts immediately)
- Line 1971: `await buildPluginsStreaming()` called AFTER dev server is live
- Plugins stream in via `.hot-swap` SSE channel (`🟦️component.tsx:400-441`)
- Failed plugins are logged and continue; previously materialized modules remain available
- **A broken Rust plugin does not crash the shell — it comes up serving the last-good build.**

## 4. Shell Readiness Beacon

**Signal:** `document.documentElement.dataset.semioOsReady === <pluginId>` (or `.semioOsError` on fault)

**Set by:** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx:6921` (ready) and `:6912` (error)

```typescript
// Line 6921 (ready state)
root.dataset.semioOsReady = beaconId;

// Line 6912 (error state)
root.dataset.semioOsError = beaconId;
```

**Test usage:** `.storybook/os-plugins.spec.ts:38-39` waits for `semioOsReady` or `semioOsError` with 60s timeout per plugin.

## 5. TypeScript Mutation Regeneration After Rust Migration

**No automatic regeneration needed:**

- Mutation TypeScript (`🟦️component.ts`) files under `🧬️mutations/` are **hand-authored**, not generated.
- Example: `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️component.ts`

- **Schema-parity gate exists** but is a verification-only policy (`mutation/schema-parity`):
  - Line 28382, 28648-28650: Defines schema-parity rules for GraphQL, Protobuf, and JSON Schema formats
  - Line 19654, 19660: `stdioRunStructuralGate("schema-parity")` enforces parity during `verify` runs
  - Does NOT regenerate TypeScript files; asserts hand-authored Rust and TypeScript mutation shapes match

- **Mutation-outcome-law verification** (line 10495-10510) gate bundle (`policyMutationOutcomeMergePolicyBreaches` × 7 rules) is runnable in isolation via `bun ./📜️script.ts verify mutation-outcome-law`, run before `verify-gate`.

---

## Summary

✓ Plugin registry and playgrounds regenerated 2026-08-30 09:13  
✓ 58 plugin modules materialized, most current as of 09:14  
✓ Dev server starts immediately; plugin builds stream in without blocking  
✓ Readiness beacon set in ShellHost at line 6921 (or error at 6912)  
✓ TypeScript mutations are hand-authored; no post-Rust-migration regeneration required
