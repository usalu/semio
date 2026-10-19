import { Glob } from "bun";
import { resolve } from "node:path";
import { TICKET, readJson } from "./lib.ts";

export type SpecRow = {
  pkg: string;
  targetFile: string;
  subpath: string;
  glueFile?: string;
  action: "new-entry" | "extend-existing-glue";
  symbols: string[] | "*";
  importers: string[];
  source: string;
};

/** Every exports spec any layering executor wrote: `<prefix>-tools/exports-needed.json`, lay6 format. */
export function loadSpecs(): { rows: SpecRow[]; files: string[] } {
  const files = [...new Glob("*-tools/exports-needed.json").scanSync(TICKET)].sort();
  const rows: SpecRow[] = [];
  for (const f of files) {
    const j = readJson(resolve(TICKET, f));
    const groups: [string, Record<string, any>][] = Object.entries<any>(j.byPackage ?? {});
    if (j.uiContract) groups.push(["@semio-tech/framework", j.uiContract]);
    for (const [pkg, m] of groups) {
      for (const [key, e] of Object.entries<any>(m)) {
        rows.push({
          pkg,
          targetFile: e.targetFile ?? key,
          subpath: e.proposedSubpath,
          glueFile: e.proposedGlueFile,
          action: e.action === "extend-existing-glue" ? "extend-existing-glue" : "new-entry",
          symbols: e.symbols === "*" || (Array.isArray(e.symbols) && e.symbols.includes("*")) ? "*" : [...(e.symbols ?? [])],
          importers: [...(e.importers ?? [])],
          source: f.split("/")[0]!,
        });
      }
    }
  }
  return { rows, files };
}
