import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";
import { authoredScales, catalog, consistency, sweep } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";

const [out, ...only] = process.argv.slice(2);
const wanted = new Set(only);
const status: Record<string, Record<string, string>> = {};
for (const asset of catalog()) {
  if (wanted.size > 0 && !wanted.has(asset.id)) continue;
  status[asset.id] = {};
  for (const scale of authoredScales(asset)) {
    const findings = [...sweep(asset, scale), ...consistency(asset, scale)];
    status[asset.id]![String(scale)] = findings.length === 0 ? "ok" : [...new Set(findings.map((finding) => finding.check))].join(",");
    for (const finding of findings) console.log(`[${finding.check}] ${finding.message}`);
  }
}
if (out !== undefined && out !== "") {
  mkdirSync(dirname(out), { recursive: true });
  const previous: Record<string, Record<string, string>> = existsSync(out) && wanted.size > 0 ? JSON.parse(readFileSync(out, "utf8")) : {};
  writeFileSync(out, JSON.stringify({ ...previous, ...status }, null, 2), "utf8");
}
