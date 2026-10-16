const files = process.argv.slice(2);
let bad = 0;
for (const f of files) {
  try {
    const m = await import(f);
    for (const [k, v] of Object.entries(m)) {
      if (!/SelfTests?$/.test(k) || typeof v !== "function") continue;
      try { await (v as () => unknown)(); console.log(`PASS\t${f.split("/").slice(-2, -1)[0]}\t${k}`); }
      catch (e) { bad++; console.log(`FAIL\t${f.split("/").slice(-2, -1)[0]}\t${k}\t${String((e as Error).message).slice(0, 160)}`); }
    }
  } catch (e) { bad++; console.log(`LOADFAIL\t${f.split("/").slice(-2, -1)[0]}\t${String((e as Error).message).slice(0, 200)}`); }
}
console.log(`DONE bad=${bad}`);
