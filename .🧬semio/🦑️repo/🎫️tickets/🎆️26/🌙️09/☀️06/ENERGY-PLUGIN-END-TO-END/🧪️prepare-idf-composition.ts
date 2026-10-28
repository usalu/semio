import { plugin } from "bun";
import { resolve } from "node:path";
const root = process.cwd();
await plugin({ name: "energy-composition-native-build-port", setup(build) {
  build.onResolve({ filter: /^@semio-tech\/process\/native-build$/ }, () => ({ path: resolve(root, "🧰️framework/🔨️modules/🏃️process/🦀️cargo-workspace/🟦️.ts") }));
} });
const { prepareStdioComposition } = await import(resolve(root, "🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🟦️.ts"));
console.log(prepareStdioComposition(root, resolve(root, "🌎️hub/🧩️compositions/🗄️stdio")));
