import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const [produced, fixtures, twinPath, parser] = process.argv.slice(2);
const twin = (await import(twinPath))[parser];
const out: Record<string, unknown> = {};
for (const name of readdirSync(produced)) {
  const [leaf, caseName] = name.slice(0, -5).split("__");
  const path = join(fixtures, leaf!, caseName!, "🔣️.diff.json");
  out[path] = twin(JSON.parse(readFileSync(path, "utf8")));
}
writeFileSync(process.env.ORDER_OUT!, JSON.stringify(out));
