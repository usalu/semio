import { dependencyFreezeCheck } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/📇️inventory/🟦️.ts";
const t = Date.now();
const r = dependencyFreezeCheck(process.cwd());
console.error("ms", Date.now() - t);
console.log(JSON.stringify({ newDeps: r.newDeps, removed: r.removedDeps }, null, 1));
