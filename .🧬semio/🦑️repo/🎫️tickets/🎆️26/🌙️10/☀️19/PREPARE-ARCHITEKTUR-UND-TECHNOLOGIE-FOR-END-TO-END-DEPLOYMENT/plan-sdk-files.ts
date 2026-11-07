import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { msiDestination } from "./msi-paths.ts";

const generated = join(process.env.SEMIO_TICKET_DIR!, "🗑️generated"), result = [], destinations = new Map<string, string>();
let maximum = 0, duplicates = 0;
for (let index = 0; index < 8; index++) {
  const data = JSON.parse(readFileSync(join(generated, `sdk-files-${index}.json`), "utf8"));
  const components = new Map<string, string>(data.components);
  const media = data.media.map(([disk, last, cabinet]: string[]) => ({ disk: Number(disk), last: Number(last), cabinet })).sort((a: any, b: any) => a.disk - b.disk);
  for (const [id, component, file, rawSize, rawSequence] of data.files) {
    const directory = components.get(component), size = Number(rawSize), sequence = Number(rawSequence);
    if (!directory || !/^[a-z0-9_]+$/iu.test(id) || !Number.isSafeInteger(size) || size < 0 || !Number.isSafeInteger(sequence) || sequence < 1) throw Error(`Invalid MSI file: ${id}`);
    const cabinet = media.find((item: any) => item.last >= sequence)?.cabinet;
    if (!/^[a-f0-9]{32}\.cab$/u.test(cabinet ?? "")) throw Error(`Invalid cabinet for ${id}`);
    const path = msiDestination(data.directories, directory, file), key = path.toLowerCase();
    if (destinations.has(key)) { duplicates++; } else destinations.set(key, id);
    maximum = Math.max(maximum, path.length);
    result.push({ index, id, path, size, cabinet });
  }
}
writeFileSync(join(generated, "sdk-file-plan.json"), JSON.stringify(result), { flag: "wx" });
console.log(JSON.stringify({ files: result.length, maximumRelativeLength: maximum, duplicateDestinations: duplicates }));
