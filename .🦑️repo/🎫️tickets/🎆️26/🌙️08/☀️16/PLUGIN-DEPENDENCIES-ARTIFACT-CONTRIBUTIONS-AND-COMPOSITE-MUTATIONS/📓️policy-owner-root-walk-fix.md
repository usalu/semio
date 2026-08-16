# 🔗️ Policy Owner-Root Walk: Nested Extension Double-Counting Fix

## 📌️ Context & Problem Statement

During the landing of ticket `26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS` lane `W3-B`, `policyPluginDependencyParityBreaches` reported a spurious HIGH-priority breach for `✏️s/🔌️plugins/📐️cad`:

```
plugin-dependency/parity  ✏️s/🔌️plugins/📐️cad  "✏️s/🔌️plugins/📐️cad" declares .depends_on("cad") with no Cargo dependency on semio-s-plugin-cad
```

### Root Cause
1. `policyDependencyOwnerRoots` builds the list of roots containing both parent plugins (`✏️s/🔌️plugins/<plugin>`) and each nested extension (`✏️s/🔌️plugins/<plugin>/🧩️extensions/<ext>`).
2. `policyWalkRelFiles` performed a recursive filesystem walk without directory-level pruning for nested root owners.
3. Consequently, the walk for a parent plugin like `✏️s/🔌️plugins/📐️cad` descended into `🧩️extensions/🏢️aec-building/` and collected its `🦀️component.rs` file.
4. When `🏢️aec-building` declared `.depends_on("cad")` backed by `semio-s-plugin-cad` in its own `Cargo.toml`, the parent plugin `📐️cad` was falsely credited with declaring `.depends_on("cad")` while having no self-referencing Cargo dependency in `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml`.

## 🛠️ Changes Implemented

### 1. `policyWalkRelFiles` Directory-Level Pruning ([`📜️script.ts`](file:///Users/ueli/Documents/semio/📜️script.ts))
Added an optional `skipDir?: (relDir: string, name: string) => boolean` predicate to `policyWalkRelFiles`. When iterating directories during the recursive walk, `skipDir` is checked before recursing, preventing subtree descent:

```typescript
function policyWalkRelFiles(
  repoRoot: string,
  relRoots: readonly string[],
  pred: (relPath: string, name: string) => boolean,
  skipDir?: (relDir: string, name: string) => boolean,
): string[]
```

### 2. `policyOwnerOwnComponentFiles` Isolation ([`📜️script.ts`](file:///Users/ueli/Documents/semio/📜️script.ts))
Updated `policyOwnerOwnComponentFiles` to pass `(_relDir, name) => name === "🧩️extensions"` to `policyWalkRelFiles`.
When evaluating a parent plugin root, `policyWalkRelFiles` skips the `🧩️extensions/` directory entirely, while extension roots (which start inside `🧩️extensions/<ext>`) are evaluated with their own isolated files.

### 3. `policyContributedSurfaceTargetBreaches` Alignment ([`📜️script.ts`](file:///Users/ueli/Documents/semio/📜️script.ts))
Updated `policyContributedSurfaceTargetBreaches` to call `policyOwnerOwnComponentFiles(repoRoot, pluginRoot)` rather than unpruned `policyWalkRelFiles`.

### 4. Unit Test Coverage ([`🧰️framework/.../🧪️index.test.ts`](file:///Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts))
Added automated tests under `describe("policyPluginDependencyParityBreaches")`:
- Verified that parent plugin root walks exclude nested extensions and do not produce spurious breaches when an extension depends on the parent plugin.
- Verified that an extension with a missing Cargo dependency is still correctly flagged at its own extension scope.

## 🧪️ Verification

1. **Automated Unit Tests**:
   ```bash
   bun test -t "policyPluginDependencyParityBreaches" 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts
   ```
   Output: `2 pass, 0 fail`.

2. **Parity Breach Query**:
   - `✏️s/🔌️plugins/📐️cad`: 0 high-priority breaches (only medium-priority pending migration tracked for stdio).
   - `✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building`: 0 breaches.
   - Total high-priority `plugin-dependency/parity` breaches repo-wide: `0`.
   - Total `plugin-dependency/contribution-target` breaches repo-wide: `0`.
   - Total `plugin-dependency/contributed-surface-target` breaches repo-wide: `0`.
