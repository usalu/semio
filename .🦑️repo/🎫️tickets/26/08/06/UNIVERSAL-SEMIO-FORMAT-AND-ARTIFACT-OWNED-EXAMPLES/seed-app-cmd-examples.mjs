/**
 * @emoji 🎮 Seed `⚙️engine/📚️examples/♻️reuse/*.cmd.semio` for every plugin app.
 */
import { existsSync, mkdirSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const PLUGINS = join(REPO, "✏️s/🔌️plugins");

const slug = (name) => name.replace(/\uFE0F/g, "").replace(/^[^\p{L}\p{N}]+/u, "");

for (const pluginDir of readdirSync(PLUGINS)) {
  if (pluginDir.startsWith(".")) continue;
  const pluginRoot = join(PLUGINS, pluginDir);
  if (!statSync(pluginRoot).isDirectory()) continue;
  const pluginId = slug(pluginDir);
  const appsRoot = join(pluginRoot, "🎛️apps");
  if (!existsSync(appsRoot)) continue;
  for (const appDir of readdirSync(appsRoot)) {
    const appRoot = join(appsRoot, appDir);
    if (!statSync(appRoot).isDirectory()) continue;
    const appId = slug(appDir);
    const envelope = `${pluginId}.${appId}`;
    const exampleDir = join(appRoot, "⚙️engine", "📚️examples", "♻️reuse");
    const cmdFile = join(exampleDir, `🧬️component.${envelope}.cmd.semio`);
    if (existsSync(cmdFile)) continue;
    mkdirSync(exampleDir, { recursive: true });
    writeFileSync(cmdFile, `semio ${envelope}.cmd v1\naction=demo\n`, "utf8");
    const leaf = join(exampleDir, "🦀️component.rs");
    if (!existsSync(leaf)) {
      writeFileSync(
        leaf,
        `pub const FIXTURE_CMD: &str = include_str!("./🧬️component.${envelope}.cmd.semio");\n`,
        "utf8",
      );
    }
  }
}
console.log("[seed-app-cmd-examples] done");
