import { resolve } from "node:path";
import base from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🎚️config/🟦️.ts";

const repository = resolve(import.meta.dirname, "../../../../../../..");
const store = (name: string, file: string) => resolve(repository, "node_modules/.bun", name, "node_modules", file);

export default {
  ...base,
  resolve: {
    alias: [
      ...base.resolve.alias,
      { find: /^d3-format$/u, replacement: store("d3-format@3.1.0", "d3-format/src/index.js") },
    ],
  },
};
