import{afterEach,beforeEach,afterAll}from"bun:test";
let peak=0;
const mb=()=>Math.round(process.memoryUsage().rss/1048576);
const timer=setInterval(()=>{peak=Math.max(peak,mb());},100);timer.unref();
afterEach(()=>{const before=mb();peak=Math.max(peak,before);Bun.gc(true);process.stderr.write(`[RSS] end ${before} MB -> after gc ${mb()} MB (peak ${peak}) heap=${Math.round(process.memoryUsage().heapUsed/1048576)}\n`);});
afterAll(()=>{process.stderr.write(`[RSS] all done ${mb()} MB peak ${peak}\n`);});
