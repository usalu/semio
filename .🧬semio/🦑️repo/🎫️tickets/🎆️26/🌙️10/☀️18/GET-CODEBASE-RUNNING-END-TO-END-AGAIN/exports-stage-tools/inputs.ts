/**
 * 📥 Provenance index of the earlier export proposals (lay13, lay14, lay15, tax2b): target file -> proposals.
 */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { TICKET } from "./lib.ts";

export type Proposal = { source: string; pkg?: string; subpath?: string; glue?: string; symbols?: string[] | "*"; importers?: number; status?: string };
const read = (p: string): any => JSON.parse(readFileSync(resolve(TICKET, p), "utf8"));
const out = new Map<string, Proposal[]>();
const add = (t: string, p: Proposal): void => { (out.get(t) ?? out.set(t, []).get(t)!).push(p); };

const l13 = read("lay13-tools/exports-needed.json");
for (const [pkg, rows] of Object.entries<any>(l13.byPackage)) for (const [t, r] of Object.entries<any>(rows)) add(t, { source: "lay13", pkg, subpath: r.proposedSubpath, glue: r.glueFile || undefined, symbols: r.symbols, importers: r.importers.length, status: r.status });
const l14 = read("lay14-tools/exports-needed.json");
for (const [pkg, rows] of Object.entries<any>(l14.byPackage)) for (const [t, r] of Object.entries<any>(rows)) add(t, { source: "lay14", pkg, subpath: r.proposedSubpath, glue: r.glueFile || undefined, symbols: r.symbols, importers: r.importers.length, status: r.status });
const l15 = read("lay15-tools/narrow-keys.json");
for (const k of l15.keys) add(k.target, { source: "lay15", pkg: "@semio-tech/repo-lib", subpath: k.key, glue: k.glue, symbols: k.names });
export const proposals = out;
export const lay13NotExportable: { from: string; to: string; specifier: string; reason: string; pkg: string }[] = l13.notExportable;
export const lay15Narrow: { key: string; glue: string; target: string; names: string[] }[] = l15.keys;
