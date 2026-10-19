import { join } from "node:path";
const workspace = process.cwd();
const owner = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧬️generator-ownership/🟦️.ts");
const laws = await import(owner);
const only = process.argv.slice(2);
for (const name of ["testBrowserTransportOwnership", "testBrowserInputClosure", "testWgpuGeneratorOwnership"]) {
  if (only.length && !only.includes(name)) continue;
  try { await laws[name](workspace); console.log("PASS", name); } catch (error) { console.log("FAIL", name, String((error as Error).message).slice(0, 1500)); }
}
