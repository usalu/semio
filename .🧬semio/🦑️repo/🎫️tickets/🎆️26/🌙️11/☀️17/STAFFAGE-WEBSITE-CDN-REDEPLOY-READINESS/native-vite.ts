/** 🚑️ Graph-free release build of the staffage site: runs `vite build --mode <SEMIO_BUILD_MODE|release>` with the site config straight from the workspace Vite, for when script admission or the build graph is starved. */
import { join, resolve } from "node:path";

const repo = resolve(import.meta.dir, "../../../../../../..");
const site = join(repo, "🎓️teaching/🏛️architecture/🧍️staffage/🌐️site");
const mode = process.env.SEMIO_BUILD_MODE ?? "release";
const child = Bun.spawn([process.execPath, join(repo, "node_modules/vite/bin/vite.js"), "build", "--config", join(site, "🏗️builder/🌐️vite/🟦️.ts"), "--mode", mode], {
  cwd: site,
  env: { ...process.env, SEMIO_BUILD_MODE: mode, SEMIO_VITE_CACHE_DIRECTORY: join(repo, ".cache/b/p/m/vite-staffage-native") },
  stdout: "inherit", stderr: "inherit",
});
process.exit(await child.exited);
