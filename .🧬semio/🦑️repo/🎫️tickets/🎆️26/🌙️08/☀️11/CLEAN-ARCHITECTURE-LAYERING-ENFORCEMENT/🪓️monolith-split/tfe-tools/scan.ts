/** 🔍 Scan test files for cross-module fixture/corpus reads by relative or framework-root literal. */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, normalize, resolve } from "node:path";

const ROOT = "/Users/ueli/Documents/semio";
const FW = join(ROOT, "🧰️framework");
const [listFile, outFile] = process.argv.slice(2);

function moduleOf(abs: string): string {
  const rel = abs.slice(ROOT.length + 1).split("/");
  const at = (i: number) => rel.slice(0, i).join("/");
  if (rel[0] === "🧰️framework") {
    if (rel[1] === "🔨️modules") return at(3);
    if (rel[1] === "🛍️products") {
      if (rel[3] === "🔨️modules") return at(5);
      return at(4);
    }
    if (rel[1] === "📦️packages") return at(3);
    return at(2);
  }
  if (rel[0] === "✏️s") return at(4);
  if (rel[0] === "🌎️hub") return at(3);
  return at(2);
}

const litRe = /(["'`])((?:\\.|(?!\1)[^\\\n])*)\1/g;
const rows: string[] = [];
const files = readFileSync(listFile, "utf8").split("\n").filter(Boolean);
const dataLike = /🧫️fixtures|corpus|\.json$|\.jsonc$|\.rs$|\.md$|\.bin$|\.txt$|\.wasm$|\.csv$|\.toml$/;
for (const f of files) {
  if (!/\/(🧪️tests|🧪️testing)\/|\.(test|spec)\.tsx?$/.test(f)) continue;
  const abs = join(ROOT, f);
  if (!existsSync(abs)) continue;
  const text = readFileSync(abs, "utf8").split("\n");
  const me = moduleOf(abs);
  text.forEach((line, i) => {
    litRe.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = litRe.exec(line))) {
      const lit = m[2];
      if (lit.includes("${")) continue;
      let target: string | null = null;
      let kind = "";
      if (lit.startsWith("./") || lit.startsWith("../")) {
        target = normalize(resolve(dirname(abs), lit));
        kind = "rel";
      } else if (/frameworkSourcePath|frameworkSourceUrl|readFrameworkSource/.test(line) && !lit.startsWith("@") && lit.includes("/")) {
        target = normalize(join(FW, lit));
        kind = "fw";
      } else if (lit.startsWith("🧰️framework/") || lit.startsWith("✏️s/") || lit.startsWith("🌎️hub/")) {
        target = normalize(join(ROOT, lit));
        kind = "root";
      } else continue;
      if (!target.startsWith(ROOT)) continue;
      const tm = moduleOf(target);
      if (tm === me) continue;
      const isTs = /\.(ts|tsx)$/.test(target);
      const data = dataLike.test(target) || /🧫️fixtures/.test(target);
      if (!data) continue;
      if (isTs && !/🧫️fixtures/.test(target)) continue;
      if (/^import |from "|import\(/.test(line.trim()) === false && kind === "rel" && !/new URL|readFileSync|Bun\.file|resolve|join|fixture|import\(/.test(line)) continue;
      rows.push([f, i + 1, me, tm, kind, target.slice(ROOT.length + 1), existsSync(target) ? "exists" : "MISSING"].join("\t"));
    }
  });
}
writeFileSync(outFile, rows.join("\n") + "\n");
console.log(rows.length);
