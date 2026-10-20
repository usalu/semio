import {plugin} from "bun";
import {existsSync, readFileSync} from "node:fs";
import {join} from "node:path";
type Row = {dir: string; exports: unknown; main?: string};
const map = JSON.parse(readFileSync(process.env.TST8_PKGMAP!, "utf8")) as Record<string, Row>;
const pick = (value: unknown): string | undefined => typeof value === "string" ? value : value && typeof value === "object" ? pick((value as Record<string, unknown>).bun ?? (value as Record<string, unknown>).import ?? (value as Record<string, unknown>).default) : undefined;
const transpiler = new Bun.Transpiler({loader: "ts"});
const facade = (path: string): {contents: string; loader: "js"} => {
  const quoted = JSON.stringify(path);
  if (path.endsWith(".json")) return {contents: `import document from ${quoted} with {type:"json"}; export default document;`, loader: "js"};
  const names = transpiler.scan(readFileSync(path, "utf8")).exports.filter(name => /^[A-Za-z_$][\w$]*$/u.test(name));
  return {contents: names.length ? `export {${names.join(",")}} from ${quoted};` : `export * from ${quoted};`, loader: "js"};
};
plugin({name: "tst8-unlinked-workspace", setup(build) {
  for (const [name, row] of Object.entries(map)) {
    const exp = row.exports;
    const entries: [string, string][] = typeof exp === "string" ? [[".", exp]] : exp && typeof exp === "object" ? Object.entries(exp as Record<string, unknown>).flatMap(([k, v]) => { const t = pick(v); return t === undefined || k.includes("*") ? [] : [[k, t] as [string, string]]; }) : row.main ? [[".", row.main]] : [];
    for (const [key, rel] of entries) {
      const path = join(row.dir, rel);
      if (!existsSync(path)) continue;
      build.module(key === "." ? name : name + key.slice(1), () => facade(path));
    }
  }
}});
