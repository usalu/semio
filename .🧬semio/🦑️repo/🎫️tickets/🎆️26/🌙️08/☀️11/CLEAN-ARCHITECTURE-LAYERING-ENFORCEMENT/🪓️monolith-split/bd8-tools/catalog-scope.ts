import { composeSpecificOsCatalogV1 } from "/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️catalog/🟦️.ts";
import { readGeneratedCatalogProjection } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📖️catalog-view/🟦️.ts";
const composition = composeSpecificOsCatalogV1("http://localhost/", { maxBytes: 2097152, maxRows: 512, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => false, progress: () => {} });
const deployed = readGeneratedCatalogProjection("/Users/ueli/Documents/semio/✏️s/🧑‍💻dev/🧩️catalog/🤖️generated");
const targets = composition.rows.targets.map(row => row.pluginId).sort();
const variants = composition.rows.playgrounds.map(row => row.variant).sort();
const dv = deployed.playgrounds.map(row => row.variant).sort();
console.log(JSON.stringify({ targets: targets.length, variants: variants.length, deployedVariants: dv.length, sameVariants: JSON.stringify(variants) === JSON.stringify(dv), onlyComposition: variants.filter(v => !dv.includes(v)), onlyDeployed: dv.filter(v => !variants.includes(v)) }));
await Bun.write(process.argv[2]!, JSON.stringify({ targets, variants, dv }, null, 1));
