import { defineConfig, loadConfigFromFile, type UserWorkspaceConfig } from "vite";
import { globSync } from "node:fs";

const root = process.cwd();
function isDiscoverable(relPath: string): boolean {
  return (
    !relPath.includes("node_modules") &&
    !relPath.startsWith(".🦑️repo/") &&
    !relPath.startsWith("♻️mit-bestand/") &&
    relPath !== "🧪️vitest.config.ts"
  );
}
const N = Number(process.env.PROBE_N ?? "24");
const discoveredConfigPaths = globSync("**/🧪️vitest.config.ts", { cwd: root })
  .filter(isDiscoverable)
  .slice(0, N);
console.error("[probe16] using", discoveredConfigPaths.length, "projects");

const discoveredProjects = (
  await Promise.all(
    discoveredConfigPaths.map(async (relPath) => {
      const absPath = `${root}/${relPath}`;
      try {
        const loaded = await loadConfigFromFile({ command: "serve", mode: "test" }, absPath, root);
        return loaded ? (loaded.config as UserWorkspaceConfig) : null;
      } catch {
        return null;
      }
    }),
  )
).filter((p): p is UserWorkspaceConfig => p !== null);
console.error("[probe16] loaded", discoveredProjects.length, "projects");

export default defineConfig({
  test: { projects: discoveredProjects },
});
