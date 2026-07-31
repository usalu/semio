import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const ticketRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(ticketRoot, "../../../../../..");
const rendererRoot = resolve(repoRoot, "🧰️framework/🛍️product/💻️os/🔨️module/📺️renderer/🧑️‍🎨️engine/⚛️react/⚡️implementation/🟦️typescript");

export default defineConfig({
  root: rendererRoot,
  resolve: {
    alias: [
      { find: /^@semio-tech\/.*\/pkg\/.*\.js$/, replacement: resolve(ticketRoot, "🟦️wasm-stub.ts") },
      { find: "@semio-tech/ui-react", replacement: resolve(repoRoot, "🧰️framework/🔨️module/🖱️ui/⚛️react/⚡️implementation/🟦️typescript/📦️index.tsx") },
      { find: "@semio-tech/ui-asset", replacement: resolve(repoRoot, "🧰️framework/🔨️module/🖱️ui/🖼️asset/⚡️implementation/🟦️typescript/📦️index.ts") },
      { find: "@semio-tech/ui-styling", replacement: resolve(repoRoot, "🧰️framework/🔨️module/🖱️ui/🎨️styling/⚡️implementation/🟦️typescript") },
      { find: "@semio-tech/framework-core", replacement: resolve(repoRoot, "🧰️framework/⚡️implementation/🟦️typescript/📦️index.ts") },
      {
        find: "@semio-tech/infinite-cavas-react-renderer",
        replacement: resolve(repoRoot, "🧰️framework/🛍️product/💻️os/🔨️module/♾️infinite/🖼️canvas/🎨️react-renderer/⚡️implementation/🟦️typescript/📦️index.tsx"),
      },
      {
        find: "@semio-tech/infinite-world-r3f",
        replacement: resolve(repoRoot, "🧰️framework/🛍️product/💻️os/🔨️module/♾️infinite/🌍️world/🎨️r3f/⚡️implementation/🟦️typescript/📦️index.tsx"),
      },
    ],
  },
  test: { environment: "jsdom" },
});
