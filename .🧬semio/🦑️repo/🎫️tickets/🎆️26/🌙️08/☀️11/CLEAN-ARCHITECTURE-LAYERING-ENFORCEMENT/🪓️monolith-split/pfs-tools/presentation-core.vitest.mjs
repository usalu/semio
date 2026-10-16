import { resolve } from "node:path";
const root = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🎤️presentation/📦️packages/🟦️typescript";
export default {
  root,
  resolve: { alias: { "@semio-tech/presentation": resolve(root, "🟦️.ts") } },
  test: { root, name: "@semio-tech/presentation", mode: "test", environment: "node", include: ["../../🧪️tests/🧭️slide-glob-assembly/🟦️.ts", "../../🧪️tests/📽️presentation-core/🟦️.ts"], passWithNoTests: false },
};
