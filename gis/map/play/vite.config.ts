// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@semio-tech/gis-map-play`. */
// #endregion 🧲Header

// #region 🔌Adapters
import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../../ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../../..");

export default createPlaygroundPlayViteConfig({
  playDir,
  repoRoot,
  playEntryKind: "map",
  extraAliases: [{ find: "@semio-tech/gis-map-react", replacement: path.resolve(playDir, "../react/index.tsx") }],
  resolveDedupe: ["react", "react-dom", "@semio-tech/gis-map-react", "three"],
  optimizeDeps: {
    include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime", "@semio-tech/infinite-cavas-react-renderer", "@semio-tech/gis-map-react"],
    esbuildOptions: { target: "esnext" },
  },
  watchIgnored: ["../rs/lib.rs", "../rs/target/**", "../rs/Cargo.toml", "../rs/script.ts"],
});
