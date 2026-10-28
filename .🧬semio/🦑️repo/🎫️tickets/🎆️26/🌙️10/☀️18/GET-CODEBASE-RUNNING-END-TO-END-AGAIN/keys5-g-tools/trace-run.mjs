// keys5-g trace-run: ONE cached target's action command, graph-free, with the executor's own environment (undeclared-g mkenv) and an fs-tracing preload ([DEBUG], temporary) that names the JS stack of every manifest read.
// usage: bun trace-run.mjs <tag> <closure.jsonl> <task id> [--timeout S]
import {readFileSync, writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
const W = "/Users/ueli/Documents/semio", C = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN";
const [tag, closureFile, id, ...flags] = process.argv.slice(2);
const timeout = Number(flags[flags.indexOf("--timeout") + 1] || 900) * 1000;
const row = readFileSync(closureFile, "utf8").split("\n").filter(Boolean).map((l) => JSON.parse(l)).find((r) => r.id === id);
const split = (text) => { const words = []; let cur = "", q = "", on = false; for (const ch of text) { if (q) { if (ch === q) q = ""; else cur += ch; } else if (ch === '"' || ch === "'") { q = ch; on = true; } else if (/\s/u.test(ch)) { if (on || cur) { words.push(cur); cur = ""; on = false; } } else { cur += ch; on = true; } } if (on || cur) words.push(cur); return words; };
const words = split(row.execution.command), cwd = row.execution?.cwd ?? ".";
const born = spawnSync("bun", [`${W}/${C}/undeclared-g-tools/mkenv.ts`, closureFile, id], {encoding: "utf8", cwd: W});
if (born.status !== 0) throw new Error("mkenv failed: " + born.stderr.slice(0, 400));
const out = `${W}/${C}/🗑️generated/keys5-g/${tag}.trace.jsonl`;
writeFileSync(out, "");
const env = {...process.env, ...JSON.parse(born.stdout), KEYS5_TRACE_OUT: out, BUN_OPTIONS: `--preload=${W}/${C}/keys5-g-tools/probe/trace.ts`};
const run = spawnSync("nice", ["-n", "10", "/Users/ueli/.local/bin/repo", ...words.slice(1)], {cwd: `${W}/${cwd}`, env, encoding: "utf8", timeout, maxBuffer: 64 * 1024 * 1024});
writeFileSync(`${W}/${C}/🗑️generated/keys5-g/${tag}.log`, (run.stdout ?? "") + (run.stderr ?? ""));
console.log(JSON.stringify({tag, id, exit: run.status, signal: run.signal, traceBytes: readFileSync(out).length}));
