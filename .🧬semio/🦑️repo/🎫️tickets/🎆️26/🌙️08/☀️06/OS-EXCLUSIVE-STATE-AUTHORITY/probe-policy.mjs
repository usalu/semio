process.env.SEMIO_OS_STATE_AUTHORITY = "1";
const mod = await import(new URL("../../../📜️script.ts", import.meta.url).href).catch(async () => {
  // path from ticket is deep — use absolute
  return import(process.cwd() + "/📜️script.ts");
});
console.log("keys", Object.keys(mod).slice(0,30));
if (typeof mod.policy === "function") {
  const r = mod.policy({ root: process.cwd() });
  console.log("policy typeof", typeof r, Array.isArray(r) ? r.length : r);
  if (Array.isArray(r)) {
    const os = r.filter(b => /os.?state|DocumentApp|authority|Store|LazyLock|HashMap/i.test(JSON.stringify(b)));
    console.log("os-ish", os.length);
    console.log(os.slice(0,5));
    console.log("total", r.length);
  }
}
