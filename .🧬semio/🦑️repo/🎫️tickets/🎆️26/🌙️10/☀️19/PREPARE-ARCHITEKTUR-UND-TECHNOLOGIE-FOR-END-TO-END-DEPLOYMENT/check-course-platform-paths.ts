import {readFileSync,mkdirSync,mkdtempSync} from 'node:fs';
import {join,posix,win32,dirname,sep} from 'node:path';
import {strict as assert} from 'node:assert';
const root=process.cwd(), course=join(root,'🎓️teaching/🏛️architecture/📚️course');
const source=readFileSync(join(course,'🎭️e2e/🟦️.ts'),'utf8');
const corpus=JSON.parse(readFileSync(join(course,'🧫️fixtures/🎭️e2e-output/🔣️.json'),'utf8'));
const names=['endToEndWorkDirectory','endToEndReportDirectory','endToEndRunDirectory'];
const functions=names.map(name=>{const start=source.indexOf('export function '+name);return source.slice(start,source.indexOf('\n/**',start+1)).replace(/^export /,'');}).join('\n');
const javascript=new Bun.Transpiler({loader:'ts'}).transformSync(functions);
const results=[];
const runFunction=functions.slice(functions.indexOf('function endToEndRunDirectory'));
const makeRun=new Function('join','sep','mkdirSync','mkdtempSync',new Bun.Transpiler({loader:'ts'}).transformSync(runFunction)+';return endToEndRunDirectory')(join,sep,mkdirSync,mkdtempSync);
const generated=join(process.env.SEMIO_TICKET_DIR,'🗑️generated');
const first=makeRun(generated),second=makeRun(generated);
assert.equal(dirname(first),generated);assert.equal(dirname(second),generated);assert.notEqual(first,second);assert.equal(first.slice(generated.length+1).length,corpus.pathBudget.run.length);
for(const host of corpus.pathBudget.repositories){
 const paths=host.platform==='win32'?win32:posix;
 const ticket=paths.join(host.root,corpus.pathBudget.ticket);
 const ticketOutput=()=>paths.join(ticket,'🗑️generated','nx');
 const helpers=new Function('join','dirname','ticketOutput','COURSE_E2E_DIRECTORY','mkdirSync','mkdtempSync','sep',javascript+';return {endToEndWorkDirectory,endToEndReportDirectory,endToEndRunDirectory}')(paths.join,paths.dirname,ticketOutput,[],mkdirSync,mkdtempSync,paths.sep);
 const outputRoot=helpers.endToEndWorkDirectory(host.root,{SEMIO_TICKET_DIR:ticket});
 let maximum=0; const errors=[];
 for(const report of corpus.reports){try{const output=helpers.endToEndReportDirectory(paths.join(outputRoot,corpus.pathBudget.run),report.stack,report.phase);for(const file of corpus.pathBudget.files)maximum=Math.max(maximum,paths.join(output,corpus.pathBudget.testDirectory,file).length);}catch(error){errors.push(String(error));}}
 results.push({platform:host.platform,outputRoot,maximum,errors});
}
console.log(JSON.stringify(results));
assert.ok(results.every(row=>row.errors.length===0&&row.maximum<=corpus.pathBudget.maximum),'Supported native/container prefixes must pass the actual report guard');
