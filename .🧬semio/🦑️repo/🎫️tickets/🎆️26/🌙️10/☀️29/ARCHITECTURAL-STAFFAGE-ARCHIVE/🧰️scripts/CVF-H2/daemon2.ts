import { existsSync, mkdirSync, readFileSync, readdirSync, renameSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const T = "/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE";
const JOBS = `${T}/🗑️generated/CVF-H2/jobs`;
const HANDLER = `${T}/🧰️scripts/CVF-H2/handler.ts`;
mkdirSync(JOBS, { recursive: true });
let loaded: any = null, loadedAt = 0;
async function handler() {
  const m = statSync(HANDLER).mtimeMs;
  if (m !== loadedAt) { loaded = await import(`${HANDLER}?v=${m}`); loadedAt = m; }
  return loaded;
}
writeFileSync(join(JOBS, "ready2"), String(Date.now()));
console.log("[DEBUG] CVF-H2 daemon2 ready", JOBS);
for (;;) {
  if (existsSync(join(JOBS, "stop2"))) break;
  const pending = readdirSync(JOBS).filter((f) => f.endsWith(".job2.json")).sort();
  for (const f of pending) {
    const path = join(JOBS, f);
    const id = f.replace(".job2.json", "");
    let text: string;
    try {
      const t0 = Date.now();
      const job = JSON.parse(readFileSync(path, "utf8"));
      renameSync(path, join(JOBS, `${id}.running`));
      text = await (await handler()).run(job);
      text += `\n[${((Date.now() - t0) / 1000).toFixed(1)} s]`;
    } catch (e) {
      text = `ERROR ${(e as Error).stack}`;
    }
    writeFileSync(join(JOBS, `${id}.out.tmp`), text);
    renameSync(join(JOBS, `${id}.out.tmp`), join(JOBS, `${id}.out.txt`));
  }
  await Bun.sleep(300);
}
console.log("[DEBUG] CVF-H2 daemon2 stopped");
