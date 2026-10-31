import { existsSync, readdirSync, readFileSync, writeFileSync, renameSync, mkdirSync } from "node:fs";
import { spawn } from "node:child_process";
const dir = process.argv[2]!;
const S = "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage";
mkdirSync(dir, { recursive: true });
const t0 = Date.now();
let running = 0;
const run = (name: string): void => {
  const path = `${dir}/${name}`;
  const job = JSON.parse(readFileSync(path, "utf8")) as { argv: string[]; cwd?: string };
  renameSync(path, path.replace(".job.json", ".running"));
  running++;
  const out: string[] = [];
  const child = spawn(process.execPath, job.argv, { cwd: job.cwd ?? S, env: process.env });
  child.stdout.on("data", (d) => out.push(String(d)));
  child.stderr.on("data", (d) => out.push(String(d)));
  child.on("close", (code) => {
    running--;
    writeFileSync(`${dir}/${name.replace(".job.json", ".out.txt")}`, out.join("") + `\n[exit ${code}]\n`);
    renameSync(path.replace(".job.json", ".running"), path.replace(".job.json", ".done"));
  });
};
while (Date.now() - t0 < 110 * 60 * 1000) {
  for (const name of readdirSync(dir)) if (name.endsWith(".job.json") && running < 2) run(name);
  await new Promise((r) => setTimeout(r, 400));
}
