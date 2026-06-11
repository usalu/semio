// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@reasoning/mindmap/wires/play`. */
// #endregion 🧲Header

// #region 🔌Adapters
import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../../../ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../../../..");
const threeModule = path.resolve(repoRoot, "node_modules/three/build/three.module.js");
const threePackageRoot = path.resolve(repoRoot, "node_modules/three");

export default createPlaygroundPlayViteConfig({
  playDir,
  repoRoot,
  playEntryKind: "wires",
  extraAliases: [
    { find: "@reasoning/mindmap/wires/react", replacement: path.resolve(playDir, "../react/index.ts") },
    { find: "@reasoning/mindmap/react", replacement: path.resolve(repoRoot, "reasoning/mindmap/react/index.tsx") },
    { find: "@reasoning/mindmap/wires/play", replacement: path.resolve(playDir, "index.ts") },
    { find: "@puzzle/2d/react", replacement: path.resolve(repoRoot, "puzzle/2d/react/index.tsx") },
    { find: /^three$/, replacement: threeModule },
    { find: /^three\/addons\/(.*)$/, replacement: `${threePackageRoot}/example/jsm/$1` },
  ],
  resolveDedupe: ["react", "react-dom", "three", "@puzzle/2d/react", "@reasoning/mindmap/wires/react"],
  optimizeDeps: {
    include: [
      "react",
      "react-dom",
      "react/jsx-runtime",
      "react/jsx-dev-runtime",
      "three",
      "@react-three/fiber",
      "@react-three/drei",
      "lucide-react",
      "@infinite/cavas/react-renderer",
      "@puzzle/2d/react",
      "@reasoning/mindmap/react",
    ],
    exclude: ["@reasoning/mindmap/wires/react", "@reasoning/mindmap/wires/play"],
    esbuildOptions: { target: "esnext" },
  },
  watchIgnored: [
    "../../../../puzzle/2d/rs/lib.rs",
    "../../../../puzzle/2d/rs/target/**",
    "../../../../puzzle/2d/rs/Cargo.toml",
    "../../../../puzzle/2d/rs/script.ts",
  ],
});
