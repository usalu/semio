import { realpathSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const root = process.cwd(), ticket = process.env.SEMIO_TICKET_DIR, repo = process.argv[2];
if (!ticket || !repo) throw Error('Ticket and repository executable required');
const { nativeTemporarySystemPortV1, withNativeTemporaryDirectoryV1 } = await import(pathToFileURL(join(root, '🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/📁️temporary/🟦️.ts')).href);
const { runCmd, runCmdStatus } = await import(pathToFileURL(join(root, '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts')).href);
const port = nativeTemporarySystemPortV1(args => runCmdStatus('subst.exe', args));
withNativeTemporaryDirectoryV1({ target: realpathSync(join(ticket, '🗑️generated')), candidates: [], ticket }, port, temporary => {
  console.log('[DEBUG] Using owned native DOS temporary alias ' + temporary);
  runCmd(repo, ['./📜️script.ts', 'setup', 'native', 'rust'], { cwd: root, env: { ...process.env, TEMP: temporary, TMP: temporary } });
});
