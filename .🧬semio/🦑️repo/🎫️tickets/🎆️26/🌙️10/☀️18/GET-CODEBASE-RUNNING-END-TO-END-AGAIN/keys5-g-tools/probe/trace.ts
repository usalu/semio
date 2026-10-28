// [DEBUG] keys5-g probe: logs the JS stack of every fs access to a package.json / Cargo.toml / README.md of a path outside the cwd chain
const fs: any = require("node:fs");
const fsp: any = require("node:fs/promises");
const out = process.env.KEYS5_TRACE_OUT!;
const orig = { append: fs.appendFileSync };
const seen = new Set<string>();
const want = /(package\.json|Cargo\.toml|README\.md)$/u;
function note(kind: string, path: unknown) {
  if (typeof path !== "string" || !want.test(path)) return;
  const stack = (new Error().stack ?? "").split("\n").slice(2, 9).map((l) => l.trim().replace(/^at /u, "").replace("/Users/ueli/Documents/semio/", "")).filter((l) => !l.includes("trace.ts"));
  const key = kind + "|" + stack.slice(0, 4).join("|");
  if (seen.has(key)) return;
  seen.add(key);
  orig.append(out, JSON.stringify({ pid: process.pid, argv: process.argv.slice(1, 4), kind, path: path.replace("/Users/ueli/Documents/semio/", ""), stack }) + "\n");
}
for (const name of ["readFileSync", "openSync", "existsSync", "statSync", "lstatSync", "accessSync", "readdirSync"] as const) {
  const f = (fs as any)[name];
  (fs as any)[name] = function (...a: unknown[]) { note(name, a[0]); return f.apply(this, a); };
}
for (const name of ["readFile", "open", "stat", "lstat", "access", "readdir"] as const) {
  const f = (fsp as any)[name];
  try { (fsp as any)[name] = function (...a: unknown[]) { note("p." + name, a[0]); return f.apply(this, a); }; } catch {}
}
