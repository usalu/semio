import { defineConfig } from "vitest/config";

const root = process.env.CLS_TEST_ROOT!;
const include = (process.env.CLS_TEST_INCLUDE ?? "🟦️.ts").split(",");
const cache = "/Users/ueli/Documents/semio/.cache/b/d/m/bun-cache";

export default defineConfig({
  root,
  resolve: { alias: [
    { find: /^immer$/, replacement: `${cache}/immer@10.1.1@@@1/dist/immer.mjs` },
    { find: /^@webassemblyjs\/leb128\/lib\/leb\.js$/, replacement: `${cache}/@webassemblyjs/leb128@1.13.2@@@1/lib/leb.js` },
    { find: /^@webassemblyjs\/leb128$/, replacement: `${cache}/@webassemblyjs/leb128@1.13.2@@@1/lib/index.js` },
  ] },
  test: { include, environment: "node", pool: "forks", testTimeout: 60000, hookTimeout: 60000 },
  server: { fs: { strict: false } },
});
