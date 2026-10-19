import { readFileSync } from "node:fs";
const r = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const strip = (s: string) => s.split("/").slice(-5).join("/");
for (const x of r.summary.violations) if (x.rule.name === "no-circular") console.log(x.from.split("/").slice(-4).join("/"), "|", (x.cycle ?? []).map((c: any) => strip(c.name)).join("  ->  "));
