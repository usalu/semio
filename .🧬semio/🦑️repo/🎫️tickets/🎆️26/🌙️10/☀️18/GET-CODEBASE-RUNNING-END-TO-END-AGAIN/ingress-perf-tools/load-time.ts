const target = process.argv[2]!;
const t0 = performance.now();
const mod = await import(target);
const t1 = performance.now();
console.log(JSON.stringify({ target: target.split("/").slice(-3).join("/"), loadMs: Math.round(t1 - t0), exports: Object.keys(mod).length }));
