// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@dag/play`. */
// #endregion 🧲Header

import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../../../../../../");

export default createPlaygroundPlayViteConfig({
  playDir,
  repoRoot,
  playEntryKind: "dag",
  extraAliases: [{ find: "@dag/react", replacement: path.resolve(playDir, "../react/index.tsx") }],
  resolveDedupe: ["react", "react-dom", "@dag/react"],
  optimizeDeps: {
    include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime", "@dag/react"],
    esbuildOptions: { target: "esnext" },
  },
  watchIgnored: ["../lib.rs", "../target/**", "../Cargo.toml", "../Cargo.lock", "../script.ts"],
});
