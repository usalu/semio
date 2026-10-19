import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
export const root = "/Users/ueli/Documents/semio";
export const C = root + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN";
export const skip = new Set(["node_modules", "target", "pkg", "dist", "📤️dist", ".git", "🗑️generated", ".cache", "bazel-out", "temp", "🎫️tickets", ".nx"]);
export const roots = ["✏️s", "🧰️framework", "🌎️hub", "♻️mit-bestand", "🎓️teaching", "🏢️semio-tech", "👴️leutwiler", "🧪️tests", "🧬️schema"];
export const walkFiles = (pred: (n: string) => boolean): string[] => {
  const out: string[] = [];
  const walk = (d: string) => {
    let es; try { es = readdirSync(d, { withFileTypes: true }); } catch { return; }
    for (const e of es) {
      if (skip.has(e.name)) continue;
      const p = join(d, e.name);
      if (e.isDirectory()) walk(p); else if (pred(e.name)) out.push(p);
    }
  };
  for (const r of roots) if (existsSync(join(root, r))) walk(join(root, r));
  return out;
};
export const renamed = (): string[] => {
  const lines = [...readFileSync(C + "/lay1-tools/renames1.txt", "utf8").split("\n"), ...readFileSync(C + "/lay1-tools/renames2.txt", "utf8").split("\n")].filter(Boolean);
  const ds = lines.map((l) => { const [o, n] = l.split("|"); return join(root, dirname(o), n); });
  ds.push(root + "/✏️s/🔌️plugins/🪐️space/🫀️catalog-documents");
  ds.push(root + "/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧰️check-builders/🧩️part-libraries");
  return ds.filter((d) => existsSync(d));
};
