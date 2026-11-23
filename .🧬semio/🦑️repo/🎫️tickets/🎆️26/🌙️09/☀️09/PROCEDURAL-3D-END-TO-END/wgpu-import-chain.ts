import { dirname, resolve } from "node:path";
/** 🔎️ [DEBUG] Prints the importer chain that reaches a module containing process.argv[3] from the entry process.argv[2]. */
const [entry, needle] = process.argv.slice(2);
const parent = new Map<string, string>();
await Bun.build({ entrypoints: [entry!], target: "browser", format: "esm", plugins: [{ name: "chain", setup(b) {
  b.onResolve({ filter: /.*/ }, r => { try { const p = Bun.resolveSync(r.path, dirname(r.importer)); if (!parent.has(p)) parent.set(p, r.importer); } catch {} return undefined; });
  b.onLoad({ filter: /.*/ }, r => { if (r.path.includes(needle!)) { let c = r.path; const out = [c]; for (let i = 0; i < 40 && parent.get(c); i++) { c = parent.get(c)!; out.push(c); } console.log(out.map(x => x.replace(process.cwd() + "/", "")).join("\n  <- ")); } return undefined; });
} }] }).catch(() => {});
