import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { ROOT, projectFiles, readJson } from "./lib.ts";
import { apply, parse, prop, setProps } from "./jsonspan.ts";
for (const f of projectFiles) { const j = readJson(f); if (j.name !== "@semio-tech/framework-os-node-graph-rs") continue; const text = readFileSync(join(ROOT, f), "utf8"); const node = prop(prop(parse(text), "targets")!.value, "wasm")!.value; if (prop(node, "cache")) continue; const next = apply(text, setProps(text, node, [["cache", false]], ["executor"])); JSON.parse(next); writeFileSync(join(ROOT, f), next); console.log("set", f.slice(-60)); }
