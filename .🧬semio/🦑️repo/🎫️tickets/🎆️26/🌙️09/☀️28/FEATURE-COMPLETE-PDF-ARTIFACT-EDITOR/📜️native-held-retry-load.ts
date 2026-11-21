/** 🧭 Check native recovery module resolution against the actual repository tool graph. */
import assert from "node:assert/strict";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";

const stage = join(import.meta.dir, "🗑️generated", "native-held-retry");
const entries = await Bun.file(join(stage, "overlay.json")).json() as {path: string; after: string}[];
assert.equal(entries.length, 2);
const targets = new Map(entries.map((row, index) => [resolve(row.path), join(stage, `load-${index}.ts`)]));
for (const {path, after} of entries) {
    const body = after.replace(/(?:\bfrom\s*|\bimport\s*\()"([.]{1,2}\/[^"]+)"/gu, (whole: string, relative: string) => {
        const absolute = resolve(dirname(path), relative);
        return whole.replace(relative, pathToFileURL(targets.get(absolute) ?? absolute).href);
    });
    await Bun.write(targets.get(resolve(path))!, body);
}
const launcher = entries.find(row => row.path.includes("/🚪️launcher/"));
assert.ok(launcher);
const loaded = await import(pathToFileURL(targets.get(resolve(launcher.path))!).href);
assert.equal(typeof loaded.launchRepositoryCargoV1, "function");
assert.equal(typeof loaded.withRepositoryCargoLaneV1, "function");
console.log("[DEBUG] The staged native launcher resolved its real repository imports");
