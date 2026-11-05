import { strict as assert } from 'node:assert';
import { existsSync, realpathSync, writeFileSync, unlinkSync } from 'node:fs';
import { join } from 'node:path';
import { homedir } from 'node:os';
import { pathToFileURL } from 'node:url';
const root = process.cwd(), ticket = process.env.SEMIO_TICKET_DIR;
if (!ticket) throw Error('Ticket required');
const native = join(root, '🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap');
const { nativeTemporaryPlanV1, withNativeTemporaryDirectoryV1, nativeTemporarySystemPortV1 } = await import(pathToFileURL(join(native, '📁️temporary/🟦️.ts')).href);
const corpus = JSON.parse(await Bun.file(join(native, '🧪️tests/🧰️toolchain/🔣️.json')).text());
for (const row of corpus.temporary) {
  if (row.expected === null) assert.throws(() => nativeTemporaryPlanV1(row.target, row.candidates));
  else assert.deepEqual(nativeTemporaryPlanV1(row.target, row.candidates), row.expected);
}
const { runCmdStatus } = await import(pathToFileURL(join(root, '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts')).href);
const target = realpathSync(join(ticket, '🗑️generated'));
const port = nativeTemporarySystemPortV1(args => runCmdStatus('subst.exe', args));
const outputs = [];
for (const candidates of [[join(homedir(), '.semio/native-temp')], ['C:/Users/Ä/semio']]) {
  let alias;
  withNativeTemporaryDirectoryV1({ target, ticket, candidates }, port, path => {
    alias = path;
    assert.ok(/^[\x00-\x7f]+$/.test(path));
    assert.ok(port.sameDirectory(path, target));
    const nested = nativeTemporarySystemPortV1(args => runCmdStatus('subst.exe', args));
    withNativeTemporaryDirectoryV1({ target: path, ticket, candidates: [] }, nested, selected => assert.equal(selected, path));
    const file = 'native-alias-probe.txt';
    writeFileSync(join(path, file), 'owned');
    assert.ok(existsSync(join(target, file)));
    unlinkSync(join(path, file));
    outputs.push({ alias: path, physical: target });
  });
  assert.ok(!existsSync(alias));
  assert.ok(existsSync(target));
}
console.log(JSON.stringify({ cases: corpus.temporary.length, aliases: outputs }));
