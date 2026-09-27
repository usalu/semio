const { createContinuationScheduler } = await import("/Users/ueli/Documents/semio/🧰️framework/🔨️modules/⏳️async/🪃️continuation/🟦️.ts");
const scheduler = createContinuationScheduler();
let timer = false;
const start = performance.now();
const timeout = setTimeout(() => { timer = true; }, 5);
for (let i = 0; i < 50000 && !timer; i += 1) await scheduler.yieldContinuation();
console.log(JSON.stringify({ timer, elapsedMs: performance.now() - start }));
clearTimeout(timeout);
scheduler.dispose();
