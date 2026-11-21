import { createHash } from 'node:crypto';
import { lstat, mkdir, readFile, rename } from 'node:fs/promises';
import { dirname, isAbsolute, join, relative, resolve } from 'node:path';
import { pathToFileURL } from 'node:url';

const [root, code, backup, plan] = process.argv.slice(2);
if (![root, code, backup].every(path => path && isAbsolute(path))) throw Error('Expected absolute paths');
const readJson = async (file: string) => JSON.parse(await readFile(file, 'utf8'));
const metadata = join(root, '.cache', 'f');
const requirePaused = async () => {
  if ((await readJson(join(metadata, 'sync.json'))).enabled !== false) throw Error('Local publishing must be paused');
};
await requirePaused();
const machine = await readJson(join(metadata, 'machine.json'));
if (machine.originId !== 'g1-m07') throw Error('Unexpected replica origin');

const core = await import(pathToFileURL(join(code, '🔁️replica', '🟦️.ts')).href);
const controller = new AbortController();
for (const name of ['SIGINT', 'SIGTERM'] as const) process.once(name, () => controller.abort());
const { signal } = controller;
const log = (value: unknown) => console.log(JSON.stringify(value));
const replica = await core.openReplicaV1({ worktree: root, storeRoot: machine.storeRoot,
  originId: machine.originId, groupId: machine.groupId, concurrency: 24 }, log);
const started = Date.now().toString();

try {
  const repaired = await core.repairV1(replica, signal);
  const git = await core.healGitV1(replica, signal);
  if (plan) {
    for (const { path, classification, remote } of (await readJson(plan)).items) {
      const id = replica.book.find(path);
      if (id === undefined || !replica.manifest.live(id)) continue;
      const entry = replica.manifest.entryAt(id);
      const current = entry.operation === 'delete' ? 'absent' : entry.operation === 'link' ?
        `link ${entry.target}` : `put ${entry.sha256} ${entry.executable}`;
      if (classification === 'needs-reconciliation' || current !== remote) core.enqueueId(replica, id);
    }
  }
  log({ event: 'ready-to-receive', repaired, git, incoming: replica.queue.size });
  let previous = Infinity;
  for (let attempt = 0; attempt < 8; attempt++) {
    await requirePaused();
    const result = await core.applyQueue(replica, signal, 'receive');
    log({ event: 'receive-pass', attempt, ...result, incoming: replica.queue.size });
    for (const path of result.kept as string[]) {
      if (path.startsWith('.git/')) throw Error(`Git ref differs locally and was preserved: ${path}`);
      const source = resolve(root, path);
      if (relative(root, source) !== path) throw Error(`Unexpected recovery path: ${path}`);
      const key = createHash('sha256').update(path).digest('hex');
      const destination = join(backup, 'incoming-conflicts', started, String(attempt), key);
      await mkdir(dirname(destination), { recursive: true, mode: 0o700 });
      const occupied = await lstat(destination).then(() => true, error => {
        if (error.code !== 'ENOENT') throw error;
        return false;
      });
      if (occupied) throw Error(`Recovery file already exists: ${path}`);
      await rename(source, destination);
      core.enqueueId(replica, replica.book.intern(path));
      log({ event: 'local-variant-saved', path, destination });
    }
    if (replica.queue.size === 0) break;
    if (result.kept.length === 0 && replica.queue.size >= previous && attempt > 2) break;
    previous = replica.queue.size;
    await Bun.sleep(Math.min(4000, 500 * 2 ** attempt));
  }
  const remaining = replica.queue.snapshot().map((id: number) => replica.book.name(id));
  log({ event: 'remaining', count: remaining.length, paths: remaining.slice(0, 75) });
  if (remaining.length > 0) process.exitCode = 1;
} finally {
  await core.closeReplicaV1(replica);
}
