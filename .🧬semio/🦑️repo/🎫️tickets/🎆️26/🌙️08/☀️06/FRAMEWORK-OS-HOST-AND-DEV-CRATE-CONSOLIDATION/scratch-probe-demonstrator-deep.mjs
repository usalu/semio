const base = "http://127.0.0.1:6029";
const ticketDir = process.argv[2];
const badRe = /Internal Server Error|Failed to resolve import|does not provide an export named|Cannot find module|SyntaxError:|The plugin "vite:esbuild" failed/;

async function probe(path) {
  try {
    const res = await fetch(base + encodeURI(path).replace(/^(https?%3A\/\/[^/]+)/, (m)=>decodeURIComponent(m)).replace(/%2F/g,'/'));
    // encodeURI breaks path - use raw URL
  } catch {}
  try {
    const res = await fetch(new URL(path, base));
    const text = await res.text();
    const head = text.slice(0, 4000);
    const bad = badRe.test(head) || res.status >= 400;
    return { path, status: res.status, bytes: text.length, bad, text: bad ? head : undefined, imports: bad ? [] : [...text.matchAll(/from\s+["']([^"']+)["']/g)].map(m=>m[1]) };
  } catch (e) {
    return { path, error: String(e), bad: true, imports: [] };
  }
}

const seeds = [
  "/",
  "/📦️index.tsx",
  "/favicon.svg",
];
// staged plugin wasm/js
const pluginRoots = [
  "/plugin-modules/demonstrator/",
];

const visited = new Set();
const failed = [];
const ok = [];
const queue = [...seeds];

while (queue.length && visited.size < 80) {
  const path = queue.shift();
  if (visited.has(path)) continue;
  visited.add(path);
  const r = await probe(path);
  if (r.bad) failed.push({ path: r.path, status: r.status, error: r.error, head: (r.text||'').slice(0,500) });
  else ok.push({ path: r.path, status: r.status, bytes: r.bytes });
  for (const imp of r.imports || []) {
    if (imp.startsWith("/@fs/") || (imp.startsWith("/") && !imp.startsWith("//") && !imp.includes("node_modules") && !imp.startsWith("/@vite") && !imp.startsWith("/@react-refresh") && !imp.startsWith("/@id/"))) {
      if (!visited.has(imp) && queue.length < 200) queue.push(imp);
    }
  }
}

// explicit critical + plugin listing
for (const p of [
  "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
  "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
  "/plugin-modules/demonstrator/semio_s_plugin_demonstrator_component.js",
  "/plugin-modules/demonstrator/semio_s_plugin_demonstrator_component.core.wasm",
]) {
  if (!visited.has(p)) {
    const r = await probe(p);
    visited.add(p);
    if (r.bad) failed.push({ path: r.path, status: r.status, error: r.error, head: (r.text||'').slice(0,500) });
    else ok.push({ path: r.path, status: r.status, bytes: r.bytes });
  }
}

const out = { visited: visited.size, okCount: ok.length, failedCount: failed.length, failed, okSample: ok.slice(0, 30) };
await Bun.write(`${ticketDir}/🧪demonstrator-e2e-deep.json`, JSON.stringify(out, null, 2));
console.log(JSON.stringify({ visited: out.visited, okCount: out.okCount, failedCount: out.failedCount, failed: out.failed }, null, 2));
