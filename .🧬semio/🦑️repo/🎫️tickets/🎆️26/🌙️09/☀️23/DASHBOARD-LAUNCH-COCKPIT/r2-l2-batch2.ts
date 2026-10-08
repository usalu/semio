import { readFileSync, writeFileSync } from "node:fs";
import { edit } from "./r2-l2-edit.ts";
const r = "C:/git/semio/";
const live = r + "✏️s/🧑‍💻dev/💡️services/🧪️tests/🤖️live-agent-loop/🟦️.ts";
let s = readFileSync(live, "utf8");
const pattern = /\(launch row "[^"]*\$\{PLUGIN\}[^"]*", or \\`bun nx run workspace:dev -- \$\{PLUGIN\}\\`\)/;
if (!pattern.test(s)) throw new Error("live pattern");
s = s.replace(pattern, "(dashboard command \\`playground:${PLUGIN}\\`, or \\`semio run playground:${PLUGIN} --detach --wait-ready\\`)");
if (!s.includes("the gate drives the shipped launch line, never")) throw new Error("live2");
s = s.replace("the gate drives the shipped launch line, never", "the gate drives the shipped server command line, never");
writeFileSync(live, s);
edit(r + "✏️s/🧑‍💻dev/💡️services/🧪️tests/💬️agent-reply/🟦️.ts", [
  ["the gate drives the shipped launch line, never", "the gate drives the shipped server command line, never"],
  ["/** 🔐️ The same launch line with", "/** 🔐️ The same server command line with"],
]);
const pm = r + "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts";
const p = readFileSync(pm, "utf8");
const q = /\(launch row [^,]*local-only, S_LOCAL_ONLY=1\)/;
if (!q.test(p)) throw new Error("pm");
writeFileSync(pm, p.replace(q, "(dashboard command playground:s with S_LOCAL_ONLY=1)"));
