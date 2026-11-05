import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
const root = process.cwd(), ticket = process.env.SEMIO_TICKET_DIR;
if (!ticket) throw Error('Ticket required');
const { runCmd } = await import(pathToFileURL(join(root, '🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts')).href);
runCmd('powershell.exe', ['-NoProfile', '-NonInteractive', '-ExecutionPolicy', 'Bypass', '-File', join(ticket, 'inspect-native-temp.ps1'), '-Target', join(ticket, '🗑️generated/native')]);
