/** 🔎 Non-source references (taxonomy, fixtures, project inputs, configs) to paths and specifiers that the direct parts moved or the delta removes. */
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { ROOT, GENERATED } from "./lib.ts";
const dir = resolve(import.meta.dir, "delta-parts");
const direct = ["d1", "d1b", "d3", "d5", "d5b", "d6", "d7"].map(n => JSON.parse(readFileSync(resolve(dir, n + ".json"), "utf8")));
const staged = JSON.parse(readFileSync(resolve(import.meta.dir, "../cycle-break-delta.json"), "utf8"));
const needles = new Map<string, string>();
for (const p of direct) for (const o of p.fileOps) if (o.op === "move") needles.set(o.path.replace(/\/[^/]+$/u, ""), "moved by a direct part -> " + o.to.replace(/\/[^/]+$/u, ""));
for (const o of staged.fileOps) if (o.op === "delete" && !/\.(ts|tsx|mts)$/u.test(o.path) === false) needles.set(o.path, "deleted glue/dead file");
for (const e of staged.edits) if (e.mode === "specifier" && e.from.startsWith("@semio-tech/")) needles.set(e.from, "specifier -> " + e.to);
for (const o of staged.manifestOps) if (o.op === "remove" && o.field === "exports") needles.set(o.package + o.key.slice(1), "key removed from " + o.package);
const list = [...needles.keys()];
writeFileSync(resolve(GENERATED, "needles.txt"), list.join("\n") + "\n");
const out = spawnSync("git", ["grep", "-n", "-F", "-f", resolve(GENERATED, "needles.txt"), "--", ".", ":(exclude).🧬semio", ":(exclude)*bun.lock", ":(exclude)🔒️dependencies.json", ":(exclude)*.ts", ":(exclude)*.tsx", ":(exclude)*.mts", ":(exclude)*.mjs", ":(exclude)*.md"], { cwd: ROOT, encoding: "utf8", maxBuffer: 1 << 28 });
const hits = out.stdout.split("\n").filter(Boolean);
const byFile = new Map<string, number>();
for (const h of hits) { const f = h.split(":")[0]!; byFile.set(f, (byFile.get(f) ?? 0) + 1); }
console.log("needles", list.length, "hits", hits.length, "files", byFile.size);
for (const [f, n] of [...byFile].sort((a, b) => b[1] - a[1]).slice(0, 40)) console.log(String(n).padStart(4), f.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", "LIB"));
writeFileSync(resolve(GENERATED, "ref-hits.txt"), hits.join("\n"));
