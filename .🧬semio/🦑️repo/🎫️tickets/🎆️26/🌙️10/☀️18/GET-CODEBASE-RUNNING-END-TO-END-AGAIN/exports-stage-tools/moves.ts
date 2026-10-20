/**
 * 🚚 Misplaced glue: a registered glue file of package P0 that re-exports modules owned by another package Pn.
 * The entry moves to Pn (destination glue = same owner-relative path), P0 loses the key, importers of the old key are rewritten.
 *
 *   bun moves.ts   (writes generated/moves.json)
 */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import ts from "typescript";
import { GENERATED, ROOT, posix } from "./lib.ts";
import { abs, entriesOf, lane, model, originsOf, parseFile, rel, resolveRel, siblingsOf } from "./model.ts";
import { scan } from "./scan.ts";
import { placement } from "./build-candidates.ts";

type V = { file: string; spec: string; target: string; owner: string };
const viol: V[] = JSON.parse(readFileSync(resolve(GENERATED, "ncpr-now.json"), "utf8")).filter((v: V) => !v.spec.includes("?"));
const entryByFile = new Map<string, { pkg: string; key: string }[]>();
for (const p of model.all) for (const e of entriesOf(p)) (entryByFile.get(e.file) ?? entryByFile.set(e.file, []).get(e.file)!).push({ pkg: p.name, key: e.sub });

export type Move = {
  glue: string; pkg0: string; keys0: string[]; owner: string; pkg: string; pkgJson: string;
  dest: string; destExists: boolean; destKey: string; destKeyExists: boolean; destValue: string;
  content: string; namesOfGlue: string[]; edges: number; sources: string[]; rewrites: { file: string; from: string; to: string }[]; flags: string[];
};

const isPureGlue = (file: string): boolean => {
  const sf = parseFile(file);
  return sf.statements.length > 0 && sf.statements.every(st => ts.isExportDeclaration(st) && !!st.moduleSpecifier && ts.isStringLiteral(st.moduleSpecifier) && st.moduleSpecifier.text.startsWith(".") && (!st.exportClause || (ts.isNamedExports(st.exportClause) && st.exportClause.elements.every(e => !e.propertyName))));
};
export const stripOwnerPrefix = (gluePathInP0: string, P0: ReturnType<typeof lane> & {}, Pn: ReturnType<typeof lane> & {}): string | null => {
  let relGlue = gluePathInP0.slice(P0.dir.length + 1);
  const ownerRel = Pn.sourceOwner.startsWith(P0.sourceOwner + "/") ? Pn.sourceOwner.slice(P0.sourceOwner.length + 1) : null;
  if (ownerRel === null) return null;
  let prefix = ownerRel;
  if (P0.name === "@semio-tech/framework") prefix = prefix.replace(/^🔨️modules\//u, "");
  if (!relGlue.startsWith(prefix + "/")) return null;
  relGlue = relGlue.slice(prefix.length + 1);
  return relGlue;
};

export function computeMoves(): Move[] {
  const { bare } = scan();
  const byFile = new Map<string, V[]>();
  for (const v of viol) if (entryByFile.has(v.file)) (byFile.get(v.file) ?? byFile.set(v.file, []).get(v.file)!).push(v);
  const out: Move[] = [];
  for (const [file, edges] of byFile) {
    const regs = entryByFile.get(file)!;
    const pkg0 = model.byName.get(regs[0]!.pkg)!;
    if (!file.startsWith(pkg0.dir + "/") || pkg0.dir === pkg0.sourceOwner || !isPureGlue(file)) continue;
    const owners = [...new Set(edges.map(e => e.owner))];
    const flags: string[] = [];
    if (owners.length !== 1) { flags.push("mixed-owners:" + owners.join("|")); }
    const owner = owners[0]!;
    const Pn = lane(owner);
    if (!Pn || Pn.name === pkg0.name) continue;
    const sfG0 = parseFile(file);
    const specs0 = sfG0.statements.filter(ts.isExportDeclaration).map(st => (st.moduleSpecifier as ts.StringLiteral).text);
    const targets0 = [...new Set(specs0.map(s => resolveRel(file, s)).filter((x): x is string => !!x))];
    let dest: string, dest0: string | null, destKeyPlanned: string | null = null;
    if (targets0.length === 1) {
      const pl = placement(Pn, targets0[0]!);
      dest0 = pl.glueRel; dest = pl.glueRel ? posix(`${Pn.dir}/${pl.glueRel}`) : targets0[0]!; destKeyPlanned = pl.key;
      if (!pl.glueRel) flags.push("destination-is-the-source-file");
    } else {
      dest0 = stripOwnerPrefix(file, pkg0, Pn);
      if (dest0 === null) flags.push("no-owner-relative-destination");
      dest = dest0 === null ? posix(`${Pn.dir}/${file.slice(pkg0.dir.length + 1)}`) : posix(`${Pn.dir}/${dest0}`);
    }
    const destExists = existsSync(abs(dest));
    const sfG = parseFile(file);
    const specs = sfG.statements.filter(ts.isExportDeclaration).map(st => (st.moduleSpecifier as ts.StringLiteral).text);
    const allTargets = specs.map(s => resolveRel(file, s)).filter((x): x is string => !!x);
    const ownerOf = (t: string): string => lane(model.all.filter(p => t.startsWith(p.sourceOwner + "/")).sort((a, b) => b.sourceOwner.length - a.sourceOwner.length)[0]?.sourceOwner ?? "")?.sourceOwner ?? "";
    const targetOwners = [...new Set(allTargets.map(ownerOf))];
    if (targetOwners.length !== 1 || targetOwners[0] !== owner) flags.push("targets-in-several-owners:" + targetOwners.join("|"));
    const text = readFileSync(abs(file), "utf8");
    const rebased = text.replace(/(from\s*)(["'])(\.[^"']*)\2/gu, (_m, a: string, q: string, s: string) => {
      const t = resolveRel(file, s); if (!t) return _m;
      let r = posix(relative(dirname(abs(dest)), abs(t))); if (!r.startsWith(".")) r = "./" + r;
      return a + q + r + q;
    });
    const eo = originsOf(file);
    const namesOfGlue = eo ? [...eo.keys()].filter(k => !k.startsWith("*bare:")) : [];
    const destKeyRel = (() => { const segs = (dest0 ?? "").split("/").map(s => s.replace(/\.(ts|tsx|mts)$/u, "").replace(/[^\x00-\x7f]/gu, "")).filter(Boolean); return segs; })();
    const existingKey = Object.entries(Pn.exports).find(([, v]) => typeof v === "string" && abs(posix(`${Pn.dir}/${(v as string).slice(2)}`)) === abs(dest))?.[0];
    const destKey = existingKey ?? destKeyPlanned ?? (destKeyRel.length ? "./" + destKeyRel.join("/") : ".");
    const rewrites: Move["rewrites"] = [];
    for (const r of regs) for (const b of bare) if (b.pkg === r.pkg && b.sub === r.key) rewrites.push({ file: b.file, from: b.spec, to: Pn.name + (destKey === "." ? "" : destKey.slice(1)) });
    out.push({ glue: file, pkg0: pkg0.name, keys0: regs.map(r => r.key), owner, pkg: Pn.name, pkgJson: Pn.packageJson, dest, destExists, destKey, destKeyExists: existingKey !== undefined, destValue: "./" + (dest0 ?? ""), content: rebased, namesOfGlue, edges: edges.length, sources: [...new Set(allTargets)], rewrites, flags });
    void rel; void siblingsOf; void ROOT;
  }
  return out;
}

if (import.meta.main) {
  const moves = computeMoves();
  writeFileSync(resolve(GENERATED, "moves.json"), JSON.stringify(moves));
  console.log("moves", moves.length, "dest exists", moves.filter(m => m.destExists).length, "dest key exists", moves.filter(m => m.destKeyExists).length, "flagged", moves.filter(m => m.flags.length).length, "rewrites", moves.reduce((n, m) => n + m.rewrites.length, 0));
  for (const m of moves) console.log(m.pkg0 + m.keys0[0]!.slice(1), "=>", m.pkg + m.destKey.slice(1), m.destExists ? "[dest glue exists]" : "[create]", m.destKeyExists ? "[key exists]" : "[key new]", m.flags.join(";"), "rw", m.rewrites.length);
}
