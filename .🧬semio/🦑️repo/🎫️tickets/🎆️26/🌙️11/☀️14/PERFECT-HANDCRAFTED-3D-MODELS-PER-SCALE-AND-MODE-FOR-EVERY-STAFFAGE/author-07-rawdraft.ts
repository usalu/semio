import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { entryOf, GENERATED } from "./volumes-lib.ts";
import { draftBody } from "./volumes-draft.ts";

const [spec, ...rest] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
for (const scale of [50, 100, 200, 500]) {
  if (asset.representations?.[String(scale)]?.status !== "authored") continue;
  const d = draftBody(asset, scale);
  const path = join(GENERATED, "author-07", `${entry.slug}.s${scale}.raw.json`);
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, JSON.stringify({ solids: d.raw, flags: d.flags }));
}
console.log("ok", entry.slug);
