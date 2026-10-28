// keys5-g sb-run: ONE cached target's action command, graph-free, with the executor's environment (undeclared-g mkenv) under a Seatbelt profile that refuses the named files; prints the sandbox denial lines (process + path) and the JS error of the failed read.
// usage: bun sb-run.mjs <tag> <closure.jsonl> <task id> <path-to-deny>... [--timeout S]
import {readFileSync, writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
const W = "/Users/ueli/Documents/semio", C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN", G = `${W}/${C}/🗑️generated/keys5-g`;
const args = process.argv.slice(2), [tag, closureFile, id] = args, rest = args.slice(3), at = rest.indexOf("--timeout"), timeout = Number(at < 0 ? 600 : rest[at + 1]) * 1000, denied = (at < 0 ? rest : rest.slice(0, at));
const row = readFileSync(closureFile, "utf8").split("\n").filter(Boolean).map((l) => JSON.parse(l)).find((r) => r.id === id);
const split = (text) => { const words = []; let cur = "", q = "", on = false; for (const ch of text) { if (q) { if (ch === q) q = ""; else cur += ch; } else if (ch === '"' || ch === "'") { q = ch; on = true; } else if (/\s/u.test(ch)) { if (on || cur) { words.push(cur); cur = ""; on = false; } } else { cur += ch; on = true; } } if (on || cur) words.push(cur); return words; };
const words = split(row.execution.command), cwd = row.execution?.cwd ?? ".";
const born = spawnSync("bun", [`${W}/${C}/undeclared-g-tools/mkenv.ts`, closureFile, id], {encoding: "utf8", cwd: W});
if (born.status !== 0) throw new Error("mkenv failed: " + born.stderr.slice(0, 400));
const profile = `(version 1)\n(allow default)\n` + denied.map((p) => `(deny file-read-data (literal ${JSON.stringify(p.startsWith("/") ? p : `${W}/${p}`)}))`).join("\n") + "\n";
writeFileSync(`${G}/${tag}.sb`, profile);
const started = new Date();
const run = spawnSync("nice", ["-n", "10", "sandbox-exec", "-f", `${G}/${tag}.sb`, "/Users/ueli/.local/bin/repo", ...words.slice(1)], {cwd: `${W}/${cwd}`, env: {...process.env, ...JSON.parse(born.stdout)}, encoding: "utf8", timeout, maxBuffer: 64 * 1024 * 1024});
writeFileSync(`${G}/${tag}.log`, (run.stdout ?? "") + (run.stderr ?? ""));
await new Promise((r) => setTimeout(r, 4000));
const seconds = Math.ceil((Date.now() - started.getTime()) / 1000) + 10;
const log = spawnSync("/usr/bin/log", ["show", "--last", `${seconds}s`, "--style", "compact", "--predicate", 'eventMessage CONTAINS "Sandbox" AND eventMessage CONTAINS "deny"'], {encoding: "utf8", maxBuffer: 64 * 1024 * 1024});
const lines = (log.stdout ?? "").split("\n").map((l) => l.replace(/^.*Sandbox: /u, "")).filter((l) => denied.some((p) => l.includes(p.split("/").slice(-2).join("/")) || l.includes(p)));
writeFileSync(`${G}/${tag}.denials`, [...new Set(lines)].join("\n") + "\n");
console.log(JSON.stringify({tag, id, exit: run.status, signal: run.signal, denials: [...new Set(lines)].slice(0, 6)}));
