import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const root = resolve(here, "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript");

/** 🧮️ Vitest for the one-off regeneration of the model digests: the staffage package as root, this folder's script as the only case. */
export default {
  root,
  resolve: { alias: [{ find: /^@teaching\/staffage$/u, replacement: resolve(root, "🟦️.ts") }] },
  test: { root, name: "regenerate-digests", environment: "node", include: [resolve(here, "regenerate-digests.ts")], testTimeout: 3_600_000 },
};
