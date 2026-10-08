import { readFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";
const root = "C:/git/semio";
const { resolveNxInvocation } = await import(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts"));
for (const input of [["run", "workspace:dev", "--", "s"], ["run", "workspace:dev", "--", "mcp", "http", "os"], ["run", "@semio-tech/framework-os-dev:dev"]]) {
  const got = resolveNxInvocation(input).args;
  console.log(JSON.stringify(input), "->", JSON.stringify(got), JSON.stringify(got) === JSON.stringify(input) ? "PASSTHROUGH" : "REWRITTEN");
}
const project = JSON.parse(readFileSync(join(root, "📋️project.json"), "utf8"));
console.log("workspace targets dev/dev-mcp-engine:", project.targets.dev, project.targets["dev-mcp-engine"]);
const osDev = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json"), "utf8"));
console.log("os-dev dev:", osDev.targets.dev);
for (const args of [["dev"], ["dev", "s"], ["dev", "mcp", "http", "os"], ["dev", "mcp", "engine"]]) {
  const r = spawnSync("bun", [join(root, "📜️script.ts"), ...args], { cwd: root, encoding: "utf8", timeout: 60000 });
  console.log(args.join(" "), "=> exit", r.status, (r.stderr || r.stdout).trim().split("\n").slice(-1)[0].slice(0, 230));
}
