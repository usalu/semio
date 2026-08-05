// 🧪️Task step-3 sanity check: policyDiscoverPluginCrateDirs is module-private in 📜️script.ts, so this
// re-runs the exact same logic (copied verbatim from the shipped file) standalone to eyeball that it
// actually lists 🌊️flow's real crate directories now that the discoverer is revived. Ticket-scratch only.
import { existsSync, readdirSync } from "node:fs";
import { join } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const POLICY_SKIP_DIRS = new Set(["node_modules", ".git", ".🦑️repo", "target", "dist", ".claude", "vendor", ".venv", ".turbo", ".nx", ".storybook", "storybook-static"]);
const POLICY_PLUGINS_ROOT = "✏️s/🔌️plugins";

type PolicyCrateRef = { dir: string; libRelPath: string; shape: "legacy" | "taxonomy" };

function policyDiscoverPluginCrateDirs(root: string): PolicyCrateRef[] {
  const found: PolicyCrateRef[] = [];
  let pluginNames: string[];
  try {
    pluginNames = readdirSync(join(root, POLICY_PLUGINS_ROOT), { withFileTypes: true })
      .filter((e) => e.isDirectory() && !POLICY_SKIP_DIRS.has(e.name))
      .map((e) => e.name);
  } catch {
    pluginNames = [];
  }

  for (const plugin of pluginNames) {
    const pluginRel = `${POLICY_PLUGINS_ROOT}/${plugin}`;

    const walkLegacy = (relDir: string): void => {
      const abs = join(root, relDir);
      let entries: ReturnType<typeof readdirSync>;
      try {
        entries = readdirSync(abs, { withFileTypes: true });
      } catch {
        return;
      }
      for (const ent of entries) {
        if (!ent.isDirectory() || POLICY_SKIP_DIRS.has(ent.name)) continue;
        const childRel = `${relDir}/${ent.name}`;
        if (ent.name === "🦀️rust" && relDir.endsWith("/⚡️implementations")) {
          if (existsSync(join(root, childRel, "📦️lib.rs")) && existsSync(join(root, childRel, "Cargo.toml"))) {
            found.push({ dir: childRel, libRelPath: `${childRel}/📦️lib.rs`, shape: "legacy" });
          }
          continue;
        }
        walkLegacy(childRel);
      }
    };
    walkLegacy(pluginRel);

    const taxonomyCargoDir = `${pluginRel}/📦️packages/🦀️rust`;
    if (existsSync(join(root, taxonomyCargoDir, "Cargo.toml")) && existsSync(join(root, pluginRel, "📦️lib.rs"))) {
      found.push({ dir: taxonomyCargoDir, libRelPath: `${pluginRel}/📦️lib.rs`, shape: "taxonomy" });
    }
  }
  return found.sort((a, b) => a.dir.localeCompare(b.dir));
}

const all = policyDiscoverPluginCrateDirs(repoRoot);
console.log(`[sanity] total discovered plugin crates: ${all.length}`);
const flow = all.filter((c) => c.dir.startsWith("✏️s/🔌️plugins/🌊️flow/"));
console.log(`[sanity] 🌊️flow crates (${flow.length}):`);
for (const c of flow) console.log(`  ${c.shape.padEnd(8)} ${c.dir}`);
console.log(`[sanity] distinct plugins represented: ${new Set(all.map((c) => c.dir.split("/")[2])).size}`);
