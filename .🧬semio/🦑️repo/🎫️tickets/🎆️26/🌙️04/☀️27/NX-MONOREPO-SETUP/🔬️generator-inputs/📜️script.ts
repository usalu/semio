import { resolve } from "node:path";
const root = process.cwd();
const entry = resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📜️script.ts");
const started = performance.now();
const child = Bun.spawn([process.execPath, entry, "invalid-generator-startup-probe"], { cwd: root, stdout: "pipe", stderr: "pipe" });
const [status, stdout, stderr] = await Promise.all([child.exited, new Response(child.stdout).text(), new Response(child.stderr).text()]);
console.log(JSON.stringify({ status, milliseconds: performance.now() - started, stdout, stderr: stderr.slice(-2000) }));
