/** 🔬 Slices the census rows: bun slice.ts <census.json> <group-by: origin|project|target-shape|decision|family> [reason] */
import { readFileSync } from "node:fs";
const rows = JSON.parse(readFileSync(process.argv[2]!, "utf8")).rows as any[];
const by = process.argv[3]!, only = process.argv[4];
const shape = (r: any) => r.target.replace(/-(react|wgpu|native)-(prototype|release)$/, "-<engine>-<profile>").replace(/^(prepare|activate|serve|dev|run|smoke|mcp|build)-[a-z0-9-]+?-(?=<engine>)/, "$1-<lane>-").replace(/^(build)-[a-z0-9-]+-site$/, "build-<lane>-site");
const key = (r: any) => by === "origin" ? r.origin : by === "project" ? r.project : by === "target-shape" ? shape(r) : by === "decision" ? `${r.reason}|${r.decision}` : r.family;
const m = new Map<string, number>();
for (const r of rows) if (r.disposition === "uncached" && (!only || r.reason === only)) m.set(key(r), (m.get(key(r)) ?? 0) + 1);
for (const [k, v] of [...m].sort((a, b) => b[1] - a[1]).slice(0, 80)) console.log(v, k);
