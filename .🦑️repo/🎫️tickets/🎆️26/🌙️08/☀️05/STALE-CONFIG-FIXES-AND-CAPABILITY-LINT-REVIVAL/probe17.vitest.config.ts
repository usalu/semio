import { defineConfig, loadConfigFromFile, type UserWorkspaceConfig } from "vite";
import { globSync } from "node:fs";
import { dirname } from "node:path";

const root = process.cwd();
function isDiscoverable(relPath: string): boolean {
  return (
    !relPath.includes("node_modules") &&
    !relPath.startsWith(".🦑️repo/") &&
    !relPath.startsWith("♻️mit-bestand/") &&
    relPath !== "🧪️vitest.config.ts"
  );
}
const ONLY = process.env.PROBE_ONLY;
let discoveredConfigPaths = globSync("**/🧪️vitest.config.ts", { cwd: root }).filter(isDiscoverable);
if (ONLY) discoveredConfigPaths = discoveredConfigPaths.filter((p) => p === ONLY);
console.error("[probe17] using", discoveredConfigPaths);

const discoveredProjects = (
  await Promise.all(
    discoveredConfigPaths.map(async (relPath) => {
      const absPath = `${root}/${relPath}`;
      try {
        const loaded = await loadConfigFromFile({ command: "serve", mode: "test" }, absPath, dirname(absPath));
        return loaded ? (loaded.config as UserWorkspaceConfig) : null;
      } catch (e) {
        console.error("[probe17] load fail", relPath, (e as Error).message.split("\n")[0]);
        return null;
      }
    }),
  )
).filter((p): p is UserWorkspaceConfig => p !== null);
console.error("[probe17] loaded", discoveredProjects.length, "root values:", discoveredProjects.map((p:any)=>p.root));

export default defineConfig({
  test: { include: [], projects: discoveredProjects },
});
