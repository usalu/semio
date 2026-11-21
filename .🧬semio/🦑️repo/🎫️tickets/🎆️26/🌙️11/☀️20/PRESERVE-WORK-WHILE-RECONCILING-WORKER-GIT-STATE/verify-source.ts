import { readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';

type Entry = { path: string; kind: string; bytes: number; target?: string; executable: boolean };
const [worktree, code] = process.argv.slice(2);
const { storeRoot } = JSON.parse(await readFile(join(worktree, '.cache', 'f', 'machine.json'), 'utf8'));
const load = (directory: string) => import(pathToFileURL(join(code, directory, '🟦️.ts')).href);
const { foldStoreV1 } = await load('🔁️replica');
const { compileIgnoreV1, scanSharedV1, sharedV1, hashFileV1 } = await load('🚧️boundary');
const controller = new AbortController();
for (const name of ['SIGINT', 'SIGTERM'] as const) process.once(name, () => controller.abort());
const { signal } = controller;
const folded = await foldStoreV1(storeRoot, signal, (event: { event: string }) => {
  if (event.event === 'rejoin-required' || event.event === 'clock-skew') throw Error(JSON.stringify(event));
});
const ignore = compileIgnoreV1(await readFile(join(worktree, '.gitignore'), 'utf8'));
const sourcePath = (path: string) => !path.startsWith('.git/') && !path.startsWith('.cache/');
const entries: Entry[] = [];
await scanSharedV1(worktree, ignore, signal, (entry: Entry) => {
  if (sourcePath(entry.path)) entries.push(entry);
});
const seen = new Set(entries.map(entry => entry.path));
const differences: { path: string; kind: string }[] = [];
let next = 0;
await Promise.all(Array.from({ length: 16 }, async () => {
  while (next < entries.length) {
    signal.throwIfAborted();
    const entry = entries[next++];
    const expected = folded.manifest.get(entry.path);
    if (expected === undefined || expected.operation === 'delete') {
      differences.push({ path: entry.path, kind: expected === undefined ? 'unregistered' : 'fleet-deleted' });
      continue;
    }
    if (entry.kind === 'link' && expected.operation === 'link' && expected.target === entry.target) continue;
    if (entry.kind === 'file' && expected.operation === 'put' && expected.bytes === entry.bytes && expected.executable === entry.executable) {
      const actual = await hashFileV1(join(worktree, entry.path)).catch((error: NodeJS.ErrnoException) => {
        if (error.code !== 'ENOENT') throw error;
      });
      if (actual?.stable && actual.sha256 === expected.sha256) continue;
    }
    differences.push({ path: entry.path, kind: 'different-content' });
  }
}));
for (const [path, entry] of folded.manifest.entries()) {
  if (sourcePath(path) && entry.operation !== 'delete' && !seen.has(path) && sharedV1(path, false, ignore)) {
    differences.push({ path, kind: 'missing-local' });
  }
}
differences.sort((left, right) => left.path.localeCompare(right.path));
console.log(JSON.stringify({ checked: entries.length, differences }));
