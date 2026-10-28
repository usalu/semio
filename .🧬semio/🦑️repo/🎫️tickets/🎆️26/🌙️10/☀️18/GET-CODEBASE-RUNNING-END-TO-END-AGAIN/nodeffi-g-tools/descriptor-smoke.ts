import { mkdtempSync, writeFileSync, mkdirSync, realpathSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
const mod = await import("../../../../../../../../🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts");
const root = realpathSync(mkdtempSync(join(tmpdir(), "nffi-")));
writeFileSync(join(root, "a.txt"), "hello"); mkdirSync(join(root, "sub"));
const out = await mod.withArtifactRelativeDirectoryV1(root, () => {}, async (directory: any) => { const names = await directory.names(); const st = await directory.stat(); const e = await directory.entry('a.txt'); return JSON.stringify({names, st: Object.keys(st ?? {}), e: e && Object.keys(e)}, (_k, v) => typeof v === 'bigint' ? String(v) : v); });
console.log(process.versions.node ?? process.versions.bun, out);
