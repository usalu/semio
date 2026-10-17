/** 🧮 Build the re-point work set: cross-module fixture reads by free-area consumers against free-area owners. */
import { existsSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { dirname, join, normalize, resolve } from "node:path";

export const ROOT = "/Users/ueli/Documents/semio";
const FW = "🧰️framework";
const OS = "🧰️framework/🛍️products/💻️os";

export function moduleRoot(rel: string): string | null {
  const s = rel.split("/");
  if (s[0] !== "🧰️framework") return null;
  if (s[1] === "🔨️modules" && s[2]) return s.slice(0, 3).join("/");
  if (s[1] === "🛍️products" && s[2] === "💻️os" && s[3] === "🔨️modules" && s[4]) return s.slice(0, 5).join("/");
  return null;
}
export function consumerRoot(rel: string): string | null {
  const module = moduleRoot(rel);
  if (module) return module;
  const s = rel.split("/");
  if (s[0] === "🧰️framework" && s[1] === "🛍️products" && s[2] === "💻️os" && s[3] && s[3] !== "🔨️modules" && s[3] !== "node_modules") return s.slice(0, 4).join("/");
  if (s[0] === "🧰️framework" && s[1] === "🧪️tests") return s.slice(0, 2).join("/");
  return null;
}
export const layerOf = (root: string) => (root.startsWith(OS) ? "os" : "fw");
const stripRaw = (t: string) => t.replace(/\?raw$/, "");

export interface Hit { file: string; start: number; end: number; lit: string; quote: string; target: string; cons: string; owner: string; text: string }

const litRe = /(["'`])((?:\\.|(?!\1)[^\\\n])*)\1/g;
const SKIP_CONSUMER = [/\/📚️library\//, /^✏️s\//, /^🌎️hub\//];
const SKIP_OWNER = [/\/🔌️plugin$/, /\/📚️library$/, /\/📺️renderer$/];
const isTestFile = (f: string) => /\/(🧪️tests|🧪️testing)\/|\.(test|spec)\.tsx?$/.test(f);
const fixtureLike = (t: string) => /🧫️fixtures|corpus/.test(t) || /\.(json|jsonc)$/.test(t);

export function collect(files: string[], opts: { includeSkipped?: boolean; textOf?: (file: string) => string | undefined } = {}): { hits: Hit[]; skipped: Hit[] } {
  const hits: Hit[] = [];
  const skipped: Hit[] = [];
  for (const f of files) {
    if (!isTestFile(f) || !f.startsWith("🧰️framework/")) continue;
    const abs = join(ROOT, f);
    if (!existsSync(abs)) continue;
    const text = opts.textOf?.(f) ?? readFileSync(abs, "utf8");
    const cons = consumerRoot(f);
    litRe.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = litRe.exec(text))) {
      const lit = m[2]!;
      if (lit.includes("${")) continue;
      const rooted = lit.startsWith("🧰️framework/");
      if (!(rooted || lit.startsWith("./") || lit.startsWith("../"))) continue;
      const bare = stripRaw(lit);
      const target = rooted ? normalize(join(ROOT, bare)) : normalize(resolve(dirname(abs), bare));
      if (!target.startsWith(ROOT + "/")) continue;
      const rel = target.slice(ROOT.length + 1);
      if (!existsSync(target)) continue;
      if (statSync(target).isDirectory()) continue;
      if (!fixtureLike(rel) || /\.(ts|tsx|rs|toml|md|txt)$/.test(rel)) continue;
      const owner = moduleRoot(rel);
      if (!owner || owner === cons) continue;
      if (rel.endsWith("🧫️fixtures/🟦️.ts")) continue;
      const h: Hit = { file: f, start: m.index, end: m.index + m[0].length, lit, quote: m[1]!, target: rel, cons: cons ?? "", owner, text };
      let skip = false;
      if (SKIP_CONSUMER.some(r => r.test(f))) skip = true;
      if (SKIP_OWNER.some(r => r.test(owner))) skip = true;
      if (cons && moduleRoot(cons) && layerOf(cons) === "fw" && layerOf(owner) === "os") skip = true;
      if (!cons) skip = true;
      (skip ? skipped : hits).push(h);
    }
  }
  return { hits, skipped };
}

if (import.meta.main) {
  const [listFile, outFile] = process.argv.slice(2) as [string, string];
  const files = readFileSync(listFile, "utf8").split("\n").filter(Boolean);
  const { hits, skipped } = collect(files);
  const row = (h: Hit) => [h.file, h.cons, h.owner, h.target, h.lit.endsWith("?raw") ? "raw" : ""].join("\t");
  writeFileSync(outFile, hits.map(row).join("\n") + "\n");
  writeFileSync(outFile + ".skipped", skipped.map(row).join("\n") + "\n");
  console.log("hits", hits.length, "skipped", skipped.length);
}
