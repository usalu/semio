import { readFileSync } from "node:fs";
const s = JSON.parse(readFileSync(`../🗑️generated/go-all-logs/summary-${process.argv[2]}-${process.argv[3]}.json`, "utf8"));
console.log("| module | pkgs | build | vet | short pass/fail/skip | full pass/fail/skip |\n|---|---|---|---|---|---|");
const f = (x?: any) => x ? `${x.exit ? "FAIL(" + x.exit + ") " : ""}${x.counts ? `${x.counts.topPass}/${x.counts.topFail}/${x.counts.topSkip} (sub ${x.counts.pass}/${x.counts.fail}/${x.counts.skip})` : ""}` : "-";
const name = (m: string) => m.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/", "M/").replace("🧰️framework/🔨️modules/", "F/").replace("/📦️packages/🐹️go", "");
for (const r of s.results) { const g = (n: string) => r.steps.find((x: any) => x.step === n); console.log(`| ${name(r.module)} | ${r.packages} | ${g("build")?.exit ?? "-"}${r.error ? " " + r.error : ""} | ${g("vet")?.exit ?? "-"} | ${f(g("short"))} | ${f(g("full"))} |`); }
