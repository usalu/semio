// [scratch CVF-A] long-lived worker: waits once for machine capacity, then answers request files by running a fresh copy of the handler module
import { readFileSync, writeFileSync, readdirSync, mkdirSync, renameSync, copyFileSync } from "node:fs";
const D = process.argv[2]!;
const H = process.argv[3]!;
mkdirSync(`${D}/req`, { recursive: true });
mkdirSync(`${D}/res`, { recursive: true });
writeFileSync(`${D}/server.ready`, String(Date.now()));
for (;;) {
  for (const name of readdirSync(`${D}/req`)) {
    if (!name.endsWith(".json")) continue;
    const path = `${D}/req/${name}`;
    let req: any;
    try { req = JSON.parse(readFileSync(path, "utf8")); } catch { continue; }
    renameSync(path, `${path}.done`);
    const t0 = Date.now();
    let out: string[];
    try {
      const copy = `${D}/h-${Date.now()}.ts`;
      copyFileSync(H, copy);
      const handler = await import(copy);
      out = await handler.handle(req);
    } catch (e: any) { out = [`ERROR ${e?.stack ?? e}`]; }
    out.push(`(${Date.now() - t0} ms)`);
    const res = `${D}/res/${name.replace(/\.json$/, ".txt")}`;
    writeFileSync(`${res}.tmp`, out.join("\n") + "\n");
    renameSync(`${res}.tmp`, res);
  }
  await new Promise((r) => setTimeout(r, 300));
}
