import { readFileSync } from "node:fs";
import { join } from "node:path";
import { TICKET } from "./targets.ts";
const recs = JSON.parse(readFileSync(join(TICKET, "🗑️generated", "uncached-outputs", "records.json"), "utf8"));
for (const r of recs) if (r.flags.length) console.log(r.project.replace("@semio-tech/", "") + ":" + r.target, "|", r.handler.kind, r.handler.std, "|", [...new Set(r.flags)].map((f: string) => f.replace("suspect-infra:", "S:").replace("strong-write:", "W:")).join(" ").slice(0, 200));
