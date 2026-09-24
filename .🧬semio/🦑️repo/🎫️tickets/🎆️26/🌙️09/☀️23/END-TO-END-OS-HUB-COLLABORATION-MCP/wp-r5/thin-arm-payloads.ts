import { readFileSync, readdirSync, writeFileSync, existsSync } from "node:fs";
import { join } from "node:path";
const SUBSETS = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets";
const MUT = join(SUBSETS, "✉️base/🧬️schema/🧬️mutations");
const slugOf = (dir: string) => dir.replace(/^[^a-z]+/u, "");
const armIds = new Map<string, string>();
for (const subset of readdirSync(SUBSETS)) {
  const p = join(SUBSETS, subset, "🧬️schema/🧬️mutations/🔣️.json");
  if (subset === "✉️base" || !existsSync(p)) continue;
  armIds.set(slugOf(subset), JSON.parse(readFileSync(p, "utf8")).$id);
}
for (const dir of readdirSync(MUT).filter((d) => /apply-/.test(d))) {
  const path = join(MUT, dir, "🧬️schema/🔣️.json");
  const old = JSON.parse(readFileSync(path, "utf8"));
  const arm = slugOf(dir).replace(/^apply-/, "");
  const target = armIds.get(arm);
  if (!target) { console.log("NO ARM", dir); continue; }
  const next = { $schema: old.$schema, $id: old.$id, title: old.title, description: `🔀️ The ${arm} arm's own mutation, carried whole: its vocabulary is the ${arm} subset's published union, referenced and never restated.`, type: "object", additionalProperties: false, required: ["mutation"], properties: { mutation: { $ref: target } } };
  if (process.argv[2] === "apply") writeFileSync(path, JSON.stringify(next, null, 2) + "\n");
  console.log(dir, "->", target, `${JSON.stringify(old).length}B -> ${JSON.stringify(next).length}B`);
}
