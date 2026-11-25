/** 🗝️ The derived-view findings of assets with the allowlists of the fixture as they are and with the entries proposed in `allowlist.json` beside this script added, per authored scale, without loading the catalog: `bun derived-with.ts <id> [<id> …]`. Nothing is written. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { S } from "../MR/lib.ts";

const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const proposed = JSON.parse(readFileSync(join(import.meta.dir, "allowlist.json"), "utf8"));
const now = engine.allowlists();
const merged = { derived: [...now.derived, ...(proposed.derivedViewAllowlist ?? [])], bandCopy: [...now.bandCopy, ...(proposed.bandCopyAllowlist ?? [])] };
for (const id of process.argv.slice(2)) {
  const asset = engine.loadAsset(id);
  for (const key of Object.keys(asset.representations).filter((candidate) => engine.authoredOf(asset, candidate) !== undefined)) {
    const line = (allow: unknown): string => JSON.stringify(engine.derivedFindings(asset, key, Number(key), undefined, allow).map((finding: any) => `${finding.check} ${finding.view} ${finding.message}`));
    console.log(`${id} 1:${key}: fixture ${line(now)}; with the proposed entries ${line(merged)}`);
  }
}
