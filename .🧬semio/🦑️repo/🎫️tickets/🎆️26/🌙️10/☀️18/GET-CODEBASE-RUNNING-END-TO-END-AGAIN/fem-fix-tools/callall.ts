import { readFileSync } from "node:fs";
const files = readFileSync(process.argv[2]!, "utf8").split("\n").filter(Boolean);
const root = process.cwd();
for (const f of files) {
  try {
    const m = await import(root + "/" + f);
    const fns = Object.entries(m).filter(([, v]) => typeof v === "function");
    if (fns.length === 0) { console.log("NOEXPORT", f.slice(-80)); continue; }
    for (const [name, fn] of fns) {
      try { await (fn as () => unknown)(); console.log("OK   ", name, f.slice(-70)); }
      catch (e) { console.log("FAIL ", name, f.slice(-70), String((e as Error).message).slice(0, 160)); }
    }
  } catch (e) { console.log("IMPORT-FAIL", f.slice(-80), String((e as Error).message).slice(0, 160)); }
}
