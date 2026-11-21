import { readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';

const [worktree, code] = process.argv.slice(2);
const { originId, groupId, storeRoot } = JSON.parse(await readFile(join(worktree, '.cache', 'f', 'machine.json'), 'utf8'));
const { verifyV1 } = await import(pathToFileURL(join(code, '🔁️replica', '🟦️.ts')).href);
const controller = new AbortController();
for (const name of ['SIGINT', 'SIGTERM'] as const) process.once(name, () => controller.abort());
const report = await verifyV1({ worktree, originId, groupId, storeRoot }, controller.signal, (event: { event: string; stage?: string; done?: number }) => {
  if (event.stage === 'fold' || event.done !== undefined && event.done % 10000 === 0) console.error(JSON.stringify(event));
});
console.log(JSON.stringify(report));
