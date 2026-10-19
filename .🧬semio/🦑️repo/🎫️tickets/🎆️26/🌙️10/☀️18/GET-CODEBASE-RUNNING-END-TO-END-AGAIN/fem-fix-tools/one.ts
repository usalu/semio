const [file, fn] = process.argv.slice(2) as [string, string];
const m = await import(process.cwd() + "/" + file);
await m[fn]();
console.log("OK", fn);
