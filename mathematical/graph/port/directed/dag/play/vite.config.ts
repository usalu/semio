// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@semio-tech/dag-play`. */
// #endregion 🧲Header

import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../../../../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../../../../../../");

export default createPlaygroundPlayViteConfig({
  playDir,
  repoRoot,
  playEntryKind: "dag",
  extraAliases: [],
  resolveDedupe: ["react", "react-dom"],
  optimizeDeps: {
    include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime"],
    exclude: ["@semio-tech/framework-playground-renderer-react/dag", "@semio-tech/dag-react"],
    esbuildOptions: { target: "esnext" },
  },
  watchIgnored: ["../lib.rs", "../target/**", "../Cargo.toml", "../Cargo.lock", "../script.ts"],
});
