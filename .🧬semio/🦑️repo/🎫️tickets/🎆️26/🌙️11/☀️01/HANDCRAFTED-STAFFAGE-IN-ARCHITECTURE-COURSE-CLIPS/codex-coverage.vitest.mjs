/** 🎓️ Focused course-coverage verification with ticket-scoped dependencies. */
import { resolve } from "node:path";
const root = process.cwd();
const runtime = resolve(import.meta.dirname, "🗑️generated/t/node_modules");
export default {
  root,
  resolve: { alias: { ajv: resolve(runtime, "ajv/dist/ajv.js"), vitest: resolve(runtime, "vitest/dist/index.js") } },
  test: { include: ["🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🎓️course-coverage/🟦️.ts"], pool: "forks", maxWorkers: 1, fileParallelism: false }
};
