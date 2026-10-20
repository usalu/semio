import { readFileSync } from "node:fs";
const WS = "/Users/ueli/Documents/semio";
const C = process.argv[2];
const list = readFileSync(`${C}/🗑️generated/cachefix-x/pj-list.txt`, "utf8").split("\n").filter(Boolean);
const policy = JSON.parse(readFileSync(`${WS}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🔣️policy.json`, "utf8"));
const matchesCommand = (name: string, cmds: string[]) => cmds.some((c) => name === c || name.startsWith(`${c}-`));
const cacheableFamily = (name: string) =>
  /^(test(?:-|$)|check(?:-|$)|lint(?:-|$)|typecheck(?:-|$)|format-check(?:-|$)|generate(?:-|$)|schema(?:-|$)|verify(?:-|$)|stdio(?:-|$)|build(?:-|$)|wasm$|native-build$|package$|extension-package$|cpp-(?:configure|build|test)$|graph-check$|policy-check$|artifact-check$|artifact-package-contract$|toolchain$)/.test(name) ||
  /(?:^|-)contract(?:-|$)/.test(name) || (/^generator(?:-|$)/.test(name) && !name.includes("generator-inputs"));
const mutatingName = (n: string) => /(?:^|-)(?:clean|gc|prune|setup|fuzz)(?:-|$)/.test(n);
const liveName = (n: string) => /(?:^|-)(?:e2e|live)(?:-|$)/.test(n);
const verifyCommand = (cmd: string) => /(?:^|[\s"'])verify(?:[\s"']|$)/.test(cmd);
const rows: any[] = [];
for (const file of list) {
  let json: any; try { json = JSON.parse(readFileSync(`${WS}/${file}`, "utf8")); } catch { continue; }
  for (const [name, t] of Object.entries<any>(json.targets ?? {})) {
    const cmd = String(t.command ?? t.options?.command ?? (Array.isArray(t.options?.commands) ? t.options.commands.join(" && ") : ""));
    if (t.cache === false || matchesCommand(name, policy.continuous) || policy.uncachedExact.includes(name) || matchesCommand(name, policy.uncached) || policy.cachedExact.includes(name) || mutatingName(name) || liveName(name) || cacheableFamily(name) || verifyCommand(cmd)) continue;
    rows.push({ file, project: json.name, name, cache: t.cache, outputs: t.outputs ?? null, inputs: t.inputs ? "explicit" : null, executor: t.executor, exec: t.metadata?.semio?.execution?.kind, cmd: cmd.slice(0, 200), dependsOn: t.dependsOn });
  }
}
await Bun.write(`${C}/🗑️generated/cachefix-x/fallthrough.json`, JSON.stringify(rows, null, 1));
const byName: Record<string, number> = {}; for (const r of rows) byName[r.name] = (byName[r.name] ?? 0) + 1;
console.log(rows.length, Object.keys(byName).length);
console.log(Object.entries(byName).sort((a, b) => b[1] - a[1]).map(([k, v]) => `${v}\t${k}`).join("\n"));
