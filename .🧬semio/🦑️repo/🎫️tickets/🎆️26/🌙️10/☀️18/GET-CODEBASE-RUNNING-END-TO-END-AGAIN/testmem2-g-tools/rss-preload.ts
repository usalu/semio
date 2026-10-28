import{afterEach,beforeEach,afterAll}from"bun:test";
let name="",peak=0;
const mb=()=>Math.round(process.memoryUsage().rss/1048576);
const timer=setInterval(()=>{peak=Math.max(peak,mb());},100);timer.unref();
beforeEach(()=>{process.stderr.write(`[RSS] start ${mb()} MB\n`);});
afterEach(()=>{const r=mb();peak=Math.max(peak,r);process.stderr.write(`[RSS] end ${r} MB (peak since start ${peak}) heap=${Math.round(process.memoryUsage().heapUsed/1048576)}\n`);});
afterAll(()=>{process.stderr.write(`[RSS] all done ${mb()} MB peak ${peak}\n`);});
