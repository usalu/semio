import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../../..");

export default createPlaygroundPlayViteConfig({
  playDir,
  repoRoot,
  playEntryKind: "trinity-jack",
  extraAliases: [{ find: "@semio-tech/trinity-react", replacement: path.resolve(playDir, "../../react/index.tsx") }],
  resolveDedupe: ["react", "react-dom", "three"],
  optimizeDeps: {
    include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime"],
    exclude: ["@semio-tech/framework-playground-renderer-react/trinity-jack", "@semio-tech/trinity-react"],
    esbuildOptions: { target: "esnext" },
  },
  watchIgnored: ["../../rewrite/engine/lib.rs", "../../rewrite/engine/target/**", "../../rewrite/engine/Cargo.toml"],
});
