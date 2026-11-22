import { join } from "node:path";
const root = "/Users/ueli/Documents/semio";
const dev = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev");
const { playgroundCatalog, resolvePlaygroundFilter } = await import("@semio-tech/framework-plugin/build/plan");
const { loadDevContribution, resolveDevContributionFile } = await import(join(dev, "🧩️contribution/📥️loading/🟦️.ts"));
const r = resolvePlaygroundFilter("stdio-xlsx");
console.log(JSON.stringify(r), JSON.stringify(playgroundCatalog().find((x: any) => x.variant === "stdio-xlsx")));
const c = loadDevContribution(root, r);
console.log(JSON.stringify(c));
