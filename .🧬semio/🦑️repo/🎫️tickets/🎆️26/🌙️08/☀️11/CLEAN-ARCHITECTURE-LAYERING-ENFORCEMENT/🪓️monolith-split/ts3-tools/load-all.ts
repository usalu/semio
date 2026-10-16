const files = process.argv.slice(2);
let ok = 0;
for (const f of files) {
  try { const mod = await import(f); ok++; console.log("loaded", f.split("/").slice(-2).join("/"), Object.keys(mod).length, "exports"); }
  catch (e) { console.log("FAILED", f, String(e).slice(0, 300)); }
}
console.log(`loaded ${ok}/${files.length}`);
