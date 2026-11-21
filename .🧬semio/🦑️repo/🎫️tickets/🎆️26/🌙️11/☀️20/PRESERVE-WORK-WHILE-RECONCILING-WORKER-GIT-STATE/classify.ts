import { readFile } from 'node:fs/promises';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';

const [saved, code, report] = process.argv.slice(2);
const json = async (file: string) => JSON.parse(await readFile(file, 'utf8'));
const { originId, groupId, storeRoot } = await json(join(saved, '.cache', 'f', 'machine.json'));
const { openReplicaV1, closeReplicaV1, foldStoreV1 } = await import(pathToFileURL(join(code, '🔁️replica', '🟦️.ts')).href);
const savedReplica = await openReplicaV1({ worktree: saved, storeRoot, originId, groupId, lock: false });
const format = (entry: { operation: string; sha256?: string; executable?: boolean; target?: string } | undefined) => {
  if (entry === undefined || entry.operation === 'delete') return 'absent';
  return entry.operation === 'link' ? `link ${entry.target}` : `put ${entry.sha256} ${entry.executable}`;
};
try {
  const { manifest } = await foldStoreV1(storeRoot, undefined, (event: { event: string }) => {
    if (event.event === 'rejoin-required' || event.event === 'clock-skew') throw Error(JSON.stringify(event));
  });
  const { differences } = await json(report);
  const items = [];
  for (const { path, kind } of differences) {
    const base = format(savedReplica.files.get(path));
    const remote = format(manifest.get(path));
    const classification = base === remote ? 'local-only' : 'needs-reconciliation';
    items.push({ path, kind, classification, base, remote });
  }
  console.log(JSON.stringify({ items, shallow: format(manifest.get('.git/shallow')), gitHead: format(manifest.get('.git/HEAD')) }));
} finally {
  await closeReplicaV1(savedReplica);
}
