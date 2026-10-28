/** 🔭️ Which staged batches of other lanes rewrite the given files and how many prose sites their new bytes hold: bun peers.ts <C> <suffix>... */
import { readdirSync, readFileSync, existsSync } from "node:fs";
const LIVE = "/Users/ueli/Documents/semio";
const language = await import(`${LIVE}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🗣️language-catalogs/🟦️.ts`);
const [C, ...suffixes] = process.argv.slice(2);
for (const tool of readdirSync(C)) {
  const staged = `${C}/${tool}/staged`;
  if (!tool.endsWith("-tools") || !existsSync(staged)) continue;
  for (const batch of readdirSync(staged)) {
    const mf = `${staged}/${batch}/manifest.json`;
    if (!existsSync(mf)) continue;
    let rows: Record<string, { file: string }>;
    try { rows = JSON.parse(readFileSync(mf, "utf8")); } catch { continue; }
    for (const [rel, row] of Object.entries(rows)) {
      if (!suffixes.some((s) => rel.endsWith(s))) continue;
      const p = `${staged}/${batch}/files/${row.file}`;
      if (existsSync(p)) console.log(`${tool}/${batch}\t${rel.slice(-48)}\tsites=${language.proseSitesV1(readFileSync(p, "utf8"))}`);
    }
  }
}
