import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { mergeConfig } from "vitest/config";
import base from "../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts";
const here = dirname(fileURLToPath(import.meta.url));
export default mergeConfig(base, { resolve: { alias: [
  { find: "@dnd-kit/core", replacement: resolve(here, "dnd-core.js") },
  { find: "@dnd-kit/sortable", replacement: resolve(here, "dnd-sortable.js") },
  { find: "@dnd-kit/utilities", replacement: resolve(here, "dnd-utilities.js") },
  { find: /^katex\/dist\/.*\.css$/, replacement: resolve(here, "empty.css") },
  { find: /^katex$/, replacement: resolve(here, "katex.js") },
  { find: "react-resizable-panels", replacement: resolve(here, "panels.js") },
] } });
