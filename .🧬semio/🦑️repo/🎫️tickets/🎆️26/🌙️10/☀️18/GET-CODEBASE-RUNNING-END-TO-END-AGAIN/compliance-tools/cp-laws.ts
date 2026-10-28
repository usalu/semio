/** ⚖️ Compliance laws runner (lane compliance, read-only): bun cp-laws.ts <list file> <out json>. Runs the repo's own laws and reports the findings that touch the listed files. */
import { readFileSync, writeFileSync, lstatSync } from "node:fs";
const LIVE = "/Users/ueli/Documents/semio";
const ROOT = process.env.CP_ROOT ?? LIVE;
const [listFile, out] = process.argv.slice(2) as [string, string];
const mine = readFileSync(listFile, "utf8").split("\n").filter(Boolean);
const mineSet = new Set(mine);
const rss = (): number => Math.round(process.memoryUsage().rss / 1048576);
const result: Record<string, unknown> = {};

const L = `${LIVE}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`;
const LA = `${ROOT}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`;
const language = await import(`${L}/⚖️laws/🗣️language-catalogs/🟦️.ts`);
const allowlist = JSON.parse(readFileSync(`${LA}/⚖️laws/🗣️language-catalogs/🔒️allowlist/🔣️.json`, "utf8"));
const LAW_ROOTS = ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", "🧰️framework/🔨️modules/🏃️process"];
const SKIP = new Set(["node_modules", "🧪️tests", "🧫️fixtures", ".git", "target", "dist", ".cache", "🗑️generated"]);
const lawCovered = (p: string): boolean => LAW_ROOTS.some((r) => p.startsWith(r + "/")) && !p.split("/").some((c) => SKIP.has(c)) && /\.(?:json|ts|mts|mjs|cjs)$/u.test(p);
const own = mine.filter(lawCovered);
const sources = own.map((path) => ({ path, text: readFileSync(`${ROOT}/${path}`, "utf8") }));
const all = [] as { rule: string; path: string; detail: string }[];
for (const source of sources) {
  if (source.path.endsWith(".json")) all.push(...language.catalogFindingsV1(source));
  else {
    const actual = language.proseSitesV1(source.text), allowed = allowlist.files[source.path] ?? 0;
    if (actual > allowed) all.push({ rule: "prose-literal", path: source.path, detail: `allowed=${allowed} actual=${actual}` });
    else if (actual < allowed) all.push({ rule: "allowance-stale", path: source.path, detail: `allowed=${allowed} actual=${actual}` });
    if (!source.path.includes("⚖️laws/🗣️language-catalogs/")) {
      const assumed = language.defaultLanguageFindingsV1(source), tolerated = allowlist.defaults[source.path] ?? 0;
      if (assumed.length > tolerated) all.push(...assumed);
      else if (assumed.length < tolerated) all.push({ rule: "allowance-stale", path: source.path, detail: `defaults allowed=${tolerated} actual=${assumed.length}` });
    }
  }
}
result.languageLawTotal = all.length;
result.languageLawMine = all;
result.languageLawInRoots = own.length;
const prose: Record<string, { prose: number; defaults: number; allowed: number }> = {};
for (const p of mine) {
  if (!/\.(m?ts|mjs|cjs)$/u.test(p)) continue;
  const text = readFileSync(`${ROOT}/${p}`, "utf8");
  prose[p] = { prose: language.proseSitesV1(text), defaults: language.defaultLanguageFindingsV1({ path: p, text }).length, allowed: allowlist.files[p] ?? 0 };
}
result.prose = prose;
const catalogs: unknown[] = [];
for (const p of mine) if (p.endsWith(".json")) catalogs.push(...language.catalogFindingsV1({ path: p, text: readFileSync(`${ROOT}/${p}`, "utf8") }));
result.catalogFindings = catalogs;
console.error("[DEBUG] language done rss", rss());

const runtime = await import(`${LIVE}/🧰️framework/🔨️modules/⚙️runtime/⚖️law/🟦️.ts`);
const files = mine.filter((p) => /\.(?:[cm]?[jt]sx?)$/u.test(p)).map((path) => ({ path, source: readFileSync(`${ROOT}/${path}`, "utf8") }));
const all2 = files.filter((f) => !runtime.isRuntimeLawExemptPathV1(f.path));
result.runtimeLawScanned = all2.length;
result.runtimeLaw = runtime.checkRuntimeLawV1(all2).violations;
result.runtimeLawIncludingTests = runtime.checkRuntimeLawV1(files).violations.filter((v: string) => !result.runtimeLaw || !(result.runtimeLaw as string[]).includes(v));

const docs = await import(`${LIVE}/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts`);
const docHits: unknown[] = [];
for (const p of mine) if (/\.(m?ts|mjs|rs)$/u.test(p)) docHits.push(...docs.docstringHitsOfText(p, readFileSync(`${ROOT}/${p}`, "utf8")));
result.docstringLaw = docHits;
console.error("[DEBUG] docstring done rss", rss(), Object.keys(docs).join(","));

const pathBudget = mine.map((p) => ({ p, units: p.length, absUnits: (ROOT + "/" + p).length, maxComponentBytes: Math.max(...p.split("/").map((c) => Buffer.byteLength(c))) })).filter((r) => r.units > 256 || r.absUnits > 256 || r.maxComponentBytes > 255);
result.pathBudget = pathBudget;
result.pathMax = Math.max(...mine.map((p) => p.length));
result.pathAbsMax = Math.max(...mine.map((p) => (ROOT + "/" + p).length));

if (process.env.CP_NAMING) try {
  const naming = await import(`${L}/⚖️laws/🧩️plugin-consistency/🗺️taxonomy-shape/🏷️naming/🟦️.ts`);
  const touch = (b: { scope?: string; summary: string; id: string }): boolean => mine.some((p) => (b.scope ?? "").includes(p) || b.summary.includes(p) || b.id.includes(p));
  const stems = naming.policyBannedNameStemBreaches(ROOT) as { scope: string; summary: string; id: string }[];
  result.bannedStemTotal = stems.length;
  result.bannedStem = stems.filter(touch);
  console.error("[DEBUG] stems done rss", rss());
  const prefix = naming.policyEmojiPrefixBreaches(ROOT) as { scope: string; summary: string; id: string }[];
  result.emojiPrefixTotal = prefix.length;
  result.emojiPrefix = prefix.filter(touch);
  console.error("[DEBUG] prefix done rss", rss());
} catch (error) {
  result.namingError = String(error).slice(0, 400);
}
writeFileSync(out, JSON.stringify(result, null, 1));
console.error("[DEBUG] done rss", rss());
