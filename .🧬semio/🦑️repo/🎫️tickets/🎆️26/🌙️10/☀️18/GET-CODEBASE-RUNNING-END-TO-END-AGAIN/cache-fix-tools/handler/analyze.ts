import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";
import { classifyAll } from "./classify.ts";
import { TICKET } from "./targets.ts";
const recs = classifyAll();
const dir = join(TICKET, "🗑️generated", "cachefix-handler"); mkdirSync(dir, { recursive: true });
writeFileSync(join(dir, "records.json"), JSON.stringify(recs));
for (const r of recs) {
  const ev = r.evidence;
  const strong = ev?.writes.filter((w) => w.strong).length ?? 0, other = (ev?.writes.length ?? 0) - strong;
  const spawns: Record<string, number> = {}; for (const s of ev?.spawns ?? []) spawns[s.name] = (spawns[s.name] ?? 0) + 1;
  console.log(r.project.replace("@semio-tech/", "") + ":" + r.target, "|", r.handler.kind, r.handler.label.split("/").slice(-2).join("/"), "| fns", ev?.owned.length, "| w", strong + "/" + other, "| sp", JSON.stringify(spawns), "|", r.flags.join(" ").slice(0, 160));
  for (const w of ev?.writes ?? []) console.log("    W", w.strong ? "STRONG" : "other", w.prim, w.arg.slice(0, 100), w.fn.split("#")[1], ":" + w.line);
}
