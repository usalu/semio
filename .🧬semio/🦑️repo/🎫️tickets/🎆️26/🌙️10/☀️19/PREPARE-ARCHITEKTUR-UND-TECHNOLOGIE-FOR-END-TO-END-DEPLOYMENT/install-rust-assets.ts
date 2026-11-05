import { realpathSync } from 'node:fs';
import { join } from 'node:path';
import { homedir } from 'node:os';
import { pathToFileURL } from 'node:url';
const root = process.cwd(), ticket = process.env.SEMIO_TICKET_DIR;
if (!ticket) throw Error('Ticket required');
const bootstrap = join(root, '🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap');
const { nativeTemporarySystemPortV1, withNativeTemporaryDirectoryV1 } = await import(pathToFileURL(join(bootstrap, '📁️temporary/🟦️.ts')).href);
const { runCmd, runCmdStatus } = await import(pathToFileURL(join(root, '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts')).href);
const port = nativeTemporarySystemPortV1(args => runCmdStatus('subst.exe', args));
withNativeTemporaryDirectoryV1({ target: realpathSync(join(ticket, '🗑️generated')), candidates: [join(homedir(), '.semio/native-temp')], ticket }, port, temporary => {
  runCmd('powershell.exe', ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', join(ticket, 'install-rust-assets.ps1'), '-RepoRoot', root, '-Bootstrap', join(bootstrap, '🔵️.ps1'), '-Pins', join(bootstrap, '🧬️schema/🔣️.json')], { cwd: root, env: { ...process.env, TEMP: temporary, TMP: temporary } });
});
