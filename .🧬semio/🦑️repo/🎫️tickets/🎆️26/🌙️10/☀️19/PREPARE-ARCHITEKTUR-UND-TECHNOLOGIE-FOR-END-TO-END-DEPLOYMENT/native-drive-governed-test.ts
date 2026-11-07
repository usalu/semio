import { join } from 'node:path';
const output=join(import.meta.dir,'🗑️generated','native-drive-governed-rerun');
try { process.kill(19648,0); throw Error('Previous PID 19648 still exists; refusing duplicate'); } catch (error) { if ((error as NodeJS.ErrnoException).code !== 'ESRCH') throw error; }
const repo=Bun.which('repo');
if (!repo) throw Error('repo executable unavailable');
await Bun.write(output+'.state.json',JSON.stringify({state:'started',pid:process.pid}));
const child=Bun.spawn([repo,'tools','exec','bun','--','test','D:/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/📁️temporary/🧪️tests/🟦️.ts'],{stdout:'pipe',stderr:'pipe'});
const [stdout,stderr,exit]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
await Promise.all([Bun.write(output+'.stdout.txt',stdout),Bun.write(output+'.stderr.txt',stderr),Bun.write(output+'.state.json',JSON.stringify({state:'finished',pid:process.pid,childPid:child.pid,exit}))]);
process.stdout.write(stdout); process.stderr.write(stderr); console.log('INNER_EXIT='+exit);
process.exitCode=exit;
