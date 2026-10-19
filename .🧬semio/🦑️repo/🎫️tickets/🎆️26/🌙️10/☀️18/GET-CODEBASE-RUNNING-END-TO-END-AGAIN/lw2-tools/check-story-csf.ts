import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { loadCsf } from "storybook/internal/csf-tools";
const repoRoot = resolve(import.meta.dir, "../../../../../../../..");
const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🧫️storybook-discovery/🔣️.json"), "utf8"));
let ok = 0, bad = 0;
for (const e of fixture.stories.filter((r: any) => r.path.startsWith("✏️s/🧑‍💻dev/"))) {
  const parsed = loadCsf(readFileSync(join(repoRoot, e.path), "utf8"), { fileName: e.path, makeTitle: (t: string) => t }).parse();
  const stories = Object.entries(parsed._stories).map(([exportName, s]: any) => ({ exportName, id: s.id, name: s.name }));
  const same = parsed.meta?.title === e.title && JSON.stringify(stories) === JSON.stringify(e.stories);
  same ? ok++ : (bad++, console.log("DIFF", e.path));
}
console.log("csf ok", ok, "bad", bad);
