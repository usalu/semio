import {join} from 'node:path';
import {pathToFileURL} from 'node:url';
const {runCmd}=await import(pathToFileURL(join(process.cwd(),'🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts')).href);
const ticket=process.env.SEMIO_TICKET_DIR;
if(!ticket) throw Error('Ticket required');
runCmd('powershell.exe',['-NoProfile','-ExecutionPolicy','Bypass','-File',join(ticket,'inspect-writer.ps1'),'-WriterId','25716','-Output',join(ticket,'🗑️generated/writer-process.json')]);
