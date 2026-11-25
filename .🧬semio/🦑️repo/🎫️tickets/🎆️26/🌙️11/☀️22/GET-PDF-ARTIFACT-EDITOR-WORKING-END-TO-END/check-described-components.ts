import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { isDeepStrictEqual } from "node:util";

/** 🧩️ Checks the pair the native `committed_descriptor` test described for `stdio-pdf` the way `stage-pdf-component.ts` takes it: pack and JSON agree, the components partition the app roster (the kernel's own `readPackageComponents`), and every app of the 1.7 editor shard selects that shard's component.
 * Usage: `bun check-described-components.ts <described directory>` from the workspace root. */
const root = process.cwd();
const [directory] = process.argv.slice(2) as [string];
const { finalizePluginDescriptor } = await import(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts"));
const { packageComponentForApp, readPackageComponents } = await import(join(root, "🧰️framework/🔨️modules/🎠️kernel/🧩️package-components/🟦️.ts"));
const pack = await readFile(join(directory, "🛂️.descriptor.semio")), json = await readFile(join(directory, "🔣️.json"), "utf8");
const final = finalizePluginDescriptor(pack, "stdio-pdf");
if (!isDeepStrictEqual(JSON.parse(json), JSON.parse(final.json))) throw new Error("JSON is not the pack form");
const descriptor = JSON.parse(final.json) as { manifest: { apps: { id: string }[] } };
const components = readPackageComponents(descriptor, descriptor.manifest.apps.map(app => app.id));
if (components === null) throw new Error("no components");
for (const app of descriptor.manifest.apps) console.log(app.id.padEnd(34), packageComponentForApp(components, app.id));
console.log("no app:", packageComponentForApp(components, null), `| ${json.length} JSON bytes`);
