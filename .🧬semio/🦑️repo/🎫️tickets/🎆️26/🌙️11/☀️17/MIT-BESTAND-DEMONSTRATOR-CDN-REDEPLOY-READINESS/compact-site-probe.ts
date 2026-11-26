import { constants } from "node:fs";
import { copyFile, mkdir, readdir, readFile, stat, writeFile } from "node:fs/promises";
import { join, resolve, sep } from "node:path";
import { browserCoreName, compressBrowserCore, gzipCoreLoader, gzipCoreSuffix } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/🗜️cores/🟦️.ts";

const [sourceArgument, destinationArgument] = process.argv.slice(2);
if (!sourceArgument || !destinationArgument) throw new Error("Usage: compact-site-probe.ts <built site> <probe destination>");
const source = resolve(sourceArgument), destination = resolve(destinationArgument);
if (destination === source || destination.startsWith(source + sep)) throw new Error("Probe output must be outside the captured site");
await stat(join(source, "index.html"));
await mkdir(destination);
let bytes = 0, cores = 0;

async function copyDirectory(parts: string[]): Promise<void> {
  const input = join(source, ...parts), output = join(destination, ...parts);
  const entries = await readdir(input, { withFileTypes: true });
  const module = parts.length >= 2 && ["🔌️plugin-modules", "🧩️extension-modules"].includes(parts[0]);
  const bindings = new Map<string, string[]>();
  for (const entry of entries) {
    if (!module || !entry.isFile() || !browserCoreName.test(entry.name)) continue;
    const binding = entry.name.replace(browserCoreName, ".js");
    if (!entries.some(item => item.isFile() && item.name === binding)) throw new Error(`Missing browser binding for ${join(input, entry.name)}`);
    bindings.set(binding, [...bindings.get(binding) ?? [], entry.name]);
  }
  for (const entry of entries) {
    const original = join(input, entry.name);
    const packed = module && entry.isFile() && browserCoreName.test(entry.name);
    const target = join(output, packed ? entry.name + gzipCoreSuffix : entry.name);
    if (target.length > 256) throw new Error(`Probe path exceeds 256 characters: ${target}`);
    if (parts.length === 0 && entry.name === ".semio-artifact.json") continue;
    if (entry.isDirectory()) {
      await mkdir(target);
      await copyDirectory([...parts, entry.name]);
      continue;
    }
    if (!entry.isFile()) throw new Error(`Unsupported captured entry: ${original}`);
    const siblings = bindings.get(entry.name);
    if (packed) {
      await compressBrowserCore(original, target);
      cores++;
    } else if (siblings !== undefined) {
      const binding = gzipCoreLoader(await readFile(original, "utf8"), siblings);
      await writeFile(target, binding, { flag: "wx" });
    } else {
      await copyFile(original, target, constants.COPYFILE_EXCL);
    }
    bytes += (await stat(target)).size;
  }
}

await copyDirectory([]);
console.log(`Packed ${cores} browser cores. Static probe: ${destination} (${bytes} bytes).`);
