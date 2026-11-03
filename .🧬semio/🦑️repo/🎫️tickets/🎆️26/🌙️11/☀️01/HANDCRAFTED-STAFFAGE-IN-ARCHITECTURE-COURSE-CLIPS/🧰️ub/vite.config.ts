import react from "@vitejs/plugin-react";
import tailwind from "@tailwindcss/vite";
import { resolve } from "node:path";
import { defineConfig } from "vite";

const repository = resolve(import.meta.dirname, "../../../../../../../..");
const staffage = resolve(repository, "🎓️teaching/🏛️architecture/🧍️staffage");
const react_ = resolve(staffage, "🎯️targets/⚛️react/📦️packages/🟦️typescript");

export default defineConfig({
  root: import.meta.dirname,
  plugins: [
    {
      name: "node-stub",
      enforce: "pre",
      resolveId: (id) => (id.startsWith("node:") ? `\0stub:${id}` : null),
      load: async (id) => {
        if (!id.startsWith("\0stub:")) return null;
        const names = Object.keys(await import(id.slice(6))).filter((name) => name !== "default" && /^[A-Za-z_$][\w$]*$/u.test(name));
        return `const f = new Proxy(function () {}, { get: (_, key) => (key === Symbol.toPrimitive ? () => "/" : key === "then" ? undefined : f), apply: () => f, construct: () => f });\n${names.map((name) => `export const ${name} = f;`).join("\n")}\nexport default {};`;
      },
    },
    react(),
    tailwind(),
  ],
  resolve: {
    alias: [
      { find: /^@teaching\/staffage-react\/app$/u, replacement: resolve(react_, "📚️app/🟦️.tsx") },
      { find: /^@teaching\/staffage-react$/u, replacement: resolve(react_, "🟦️.tsx") },
      { find: /^@teaching\/staffage$/u, replacement: resolve(staffage, "📦️packages/🟦️typescript/🟦️.ts") },
      { find: /^@semio-tech\/framework-transient$/u, replacement: resolve(repository, "🧰️framework/🔨️modules/🫧️transient/🟦️.ts") },
      { find: /^d3-format$/u, replacement: resolve(repository, "node_modules/.bun/d3-format@3.1.0/node_modules/d3-format/src/index.js") },
    ],
    dedupe: ["react", "react-dom"],
  },
  cacheDir: resolve(import.meta.dirname, "../🗑️generated/ub/vitecache"),
  server: { host: "127.0.0.1", port: 6291, strictPort: true, fs: { strict: false }, watch: null },
});
