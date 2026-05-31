// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@puzzle/2d/play`. */
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
	playEntryKind: "2d",
	extraAliases: [{ find: "@puzzle/2d/react", replacement: path.resolve(playDir, "../react/index.tsx") }],
	// Rebuild wasm writes to `../rs/pkg` — do not ignore pkg or play keeps stale edge rendering after `bun ./script.ts wasm`.
	watchIgnored: ["../rs/lib.rs", "../rs/target/**", "../rs/Cargo.toml", "../rs/Cargo.lock", "../rs/script.ts"],
});
