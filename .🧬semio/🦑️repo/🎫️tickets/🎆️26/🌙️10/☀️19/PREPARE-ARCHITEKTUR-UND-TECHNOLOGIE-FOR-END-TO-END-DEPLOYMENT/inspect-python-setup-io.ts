import {readFileSync} from 'node:fs';
import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
const {runCmd}=await import(pathToFileURL(join(process.cwd(),'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts')).href);
const ticket=process.env.SEMIO_TICKET_DIR;
if(!ticket)throw Error('Ticket required');
const source=readFileSync(join(ticket,'inspect-current-sync-io.ps1'),'utf8');
runCmd('powershell.exe',['-NoProfile','-Command',`& { ${source} } -WriterId 21596 -Output '${join(ticket,'🗑️generated/python-setup-io.json').replaceAll("'","''")}'`]);
