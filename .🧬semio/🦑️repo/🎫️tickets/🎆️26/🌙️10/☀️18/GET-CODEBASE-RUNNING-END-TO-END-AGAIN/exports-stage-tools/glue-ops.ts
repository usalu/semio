/**
 * 🧩 Glue operations: extension lines for a glue file that lacks names, re-export statements of a glue file.
 */
import { dirname, relative } from "node:path";
import ts from "typescript";
import { posix } from "./lib.ts";
import { abs, originsOf, parseFile, resolveRel } from "./model.ts";

export const relSpec = (fromFile: string, toFile: string): string => { const p = posix(relative(dirname(abs(fromFile)), abs(toFile))); return p.startsWith(".") ? p : "./" + p; };

export type Reexport = { spec: string; target: string | null; names: string[]; typeOnly: boolean; star: boolean };
export function reexportsOf(file: string): Reexport[] {
  const sf = parseFile(file);
  const out: Reexport[] = [];
  for (const st of sf.statements) {
    if (!ts.isExportDeclaration(st) || !st.moduleSpecifier || !ts.isStringLiteral(st.moduleSpecifier)) continue;
    const spec = st.moduleSpecifier.text;
    const target = resolveRel(file, spec);
    if (st.exportClause && ts.isNamedExports(st.exportClause)) out.push({ spec, target, names: st.exportClause.elements.map(e => e.name.text === (e.propertyName ?? e.name).text ? e.name.text : (e.propertyName!.text + " as " + e.name.text)), typeOnly: st.isTypeOnly, star: false });
    else out.push({ spec, target, names: [], typeOnly: st.isTypeOnly, star: true });
  }
  return out;
}

/** Lines to append to `glue` so that it exports every name of `wanted` (name -> {target, type}); skips names it already exports. */
export function extensionLines(glue: string, wanted: Map<string, { target: string; type: boolean }>): { lines: string[]; values: string[]; types: string[] } {
  const have = originsOf(glue) ?? new Map();
  const byTarget = new Map<string, { values: string[]; types: string[] }>();
  for (const [name, w] of [...wanted].sort((a, b) => (a[0] < b[0] ? -1 : 1))) {
    if (have.has(name)) continue;
    const row = byTarget.get(w.target) ?? byTarget.set(w.target, { values: [], types: [] }).get(w.target)!;
    (w.type ? row.types : row.values).push(name);
  }
  const lines: string[] = [], values: string[] = [], types: string[] = [];
  for (const [target, row] of byTarget) {
    const spec = relSpec(glue, target);
    if (row.values.length) { lines.push(`export { ${row.values.join(", ")} } from ${JSON.stringify(spec)};`); values.push(...row.values); }
    if (row.types.length) { lines.push(`export type { ${row.types.join(", ")} } from ${JSON.stringify(spec)};`); types.push(...row.types); }
  }
  return { lines, values, types };
}
