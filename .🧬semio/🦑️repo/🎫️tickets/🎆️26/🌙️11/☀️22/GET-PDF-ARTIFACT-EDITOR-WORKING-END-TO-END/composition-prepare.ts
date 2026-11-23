import { resolve } from "node:path";
const root = process.cwd();
const { prepareStdioComposition } = await import(resolve(root, "🌎️hub/🧩️compositions/🗄️stdio/🧩️composition/🟦️.ts"));
console.log(JSON.stringify(prepareStdioComposition(root, resolve(root, "🌎️hub/🧩️compositions/🗄️stdio"))));
