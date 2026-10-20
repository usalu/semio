/** 🔁 Lists the exports-stage batch entries whose keys or glue the cycle-break delta supersedes (key moved to a carved package, glue deleted), for the re-stage after the sync. */
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { keyMap } from "./refs-map.ts";
const C = resolve(import.meta.dir, "..");
const batch = JSON.parse(readFileSync(resolve(C, "exports-stage-tools/batch.json"), "utf8")) as { entries: { id: string; op: string; field: string; package: string; key: string; packageJson: string }[]; glue: { id: string; path: string; entry: string }[] };
const delta = JSON.parse(readFileSync(resolve(C, "cycle-break-delta.json"), "utf8")) as { manifestOps: { op: string; field: string; packageJson: string; key: string; package: string; why: string }[]; fileOps: { op: string; path: string }[] };
const removed = new Set(delta.manifestOps.filter(o => o.op === "remove" && o.field === "exports").map(o => o.packageJson + "\0" + o.key));
const deleted = new Set(delta.fileOps.filter(o => o.op === "delete").map(o => o.path));
const entries = batch.entries.filter(e => e.field === "exports" && e.op === "add" && removed.has(e.packageJson + "\0" + e.key));
const glue = batch.glue.filter(g => deleted.has(g.path));
const out = { schema: "cycle-break/supersedes@1", note: "Batch entries already applied live whose key the delta moves to the carved package, and batch glue files the delta deletes. Re-run exports-stage-tools/stage.ts after the delta and the sync; these entries then no longer exist as candidates.", entries: entries.map(e => ({ id: e.id, package: e.package, key: e.key, movedTo: keyMap.get(e.package + e.key.slice(1)) ?? /key moves to (\S+)/u.exec(delta.manifestOps.find(o => o.op === "remove" && o.packageJson === e.packageJson && o.key === e.key)?.why ?? "")?.[1] ?? null })), glue: glue.map(g => ({ id: g.id, path: g.path, entry: g.entry })) };
writeFileSync(resolve(C, "cycle-break-supersedes.json"), JSON.stringify(out, null, 1));
console.log("superseded entries", out.entries.length, "of which keyed", out.entries.filter(e => e.movedTo).length, "glue files", out.glue.length);
