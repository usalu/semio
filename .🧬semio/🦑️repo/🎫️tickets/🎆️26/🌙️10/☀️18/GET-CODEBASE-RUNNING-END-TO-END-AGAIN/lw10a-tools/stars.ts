import { readFileSync } from "node:fs";
import { ROOT } from "./pkgs.ts";
const p = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
for (const s of p.skipped.filter((s: any) => s.reason.startsWith("namespace"))) {
  const text = readFileSync(ROOT + s.from, "utf8");
  const line = text.split("\n").filter((l) => l.includes(s.specifier)).map((l) => l.trim().slice(0, 150));
  console.log(s.from.split("/").slice(-4).join("/"), "|", s.to.replace("🧰️framework/🔨️modules/🏃️process/", ""), "|", line.join(" || "));
}
