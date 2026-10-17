/** 🔥️ Imports each `spec:name,name` argument in a fresh bun process (from the repository root) and reports the value types. Usage: bun load-check.ts <spec:names>... */
const ROOT = "/Users/ueli/Documents/semio";
let bad = 0;
for (const arg of process.argv.slice(2)) {
  const [spec, names = ""] = arg.split(":");
  const code = `const m = await import(${JSON.stringify(spec)}); const want = ${JSON.stringify(names.split(",").filter(Boolean))}; const miss = want.filter(n => !(n in m)); console.log("LOADED " + want.map(n => n + ":" + typeof m[n]).join(" ") + (miss.length ? " MISSING " + miss.join(",") : ""));`;
  const p = Bun.spawn(["bun", "-e", code], { cwd: ROOT, stdout: "pipe", stderr: "pipe" });
  const [o, e] = await Promise.all([new Response(p.stdout).text(), new Response(p.stderr).text()]);
  await p.exited;
  const ok = /LOADED/u.test(o) && !/MISSING/u.test(o);
  if (!ok) bad++;
  console.log(ok ? "ok  " : "FAIL", spec, ok ? o.trim() : (e.split("\n")[0] || o).slice(0, 200));
}
process.exit(bad ? 1 : 0);
