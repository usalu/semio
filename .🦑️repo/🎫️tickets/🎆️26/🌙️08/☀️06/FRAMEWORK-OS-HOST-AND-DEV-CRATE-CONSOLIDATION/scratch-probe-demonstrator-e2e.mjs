const base = "http://127.0.0.1:6029";
const ticketDir = process.argv[2];

async function probe(path) {
  try {
    const res = await fetch(base + path);
    const text = await res.text();
    const head = text.slice(0, 2500);
    const bad =
      /Internal Server Error|Failed to resolve|does not provide an export named|Cannot find module|SyntaxError:/.test(head) ||
      (res.status >= 400);
    return { path, status: res.status, bytes: text.length, bad, head: head.slice(0, 220).replace(/\n/g, " ") };
  } catch (e) {
    return { path, error: String(e), bad: true };
  }
}

async function waitReady(maxMs = 60000) {
  const start = Date.now();
  while (Date.now() - start < maxMs) {
    try {
      const res = await fetch(base + "/");
      if (res.ok) return true;
    } catch {}
    await Bun.sleep(500);
  }
  return false;
}

const ready = await waitReady();
if (!ready) {
  const out = { ready: false };
  await Bun.write(`${ticketDir}/🧪demonstrator-e2e-probe.json`, JSON.stringify(out, null, 2));
  console.log(JSON.stringify(out));
  process.exit(1);
}

const index = await probe("/");
const entryPath = "/📦️index.tsx";
const entry = await probe(entryPath);
const results = { ready: true, index, entry, failed: [] };

if (entry.status === 200) {
  const full = await (await fetch(base + entryPath)).text();
  const imports = [...full.matchAll(/from\s+["']([^"']+)["']/g)].map((m) => m[1]);
  results.importCount = imports.length;
  results.importsSample = imports.slice(0, 40);
  for (const imp of imports) {
    if (imp.startsWith("/@fs/") || (imp.startsWith("/") && !imp.startsWith("//") && !imp.startsWith("/node_modules"))) {
      const r = await probe(imp);
      if (r.bad) results.failed.push(r);
    }
  }
  // Also pull vite-transformed relative deps that show as absolute after transform
  const absImports = [...full.matchAll(/from\s+["'](\/@id\/[^"']+|\/@fs\/[^"']+)["']/g)].map((m) => m[1]);
  for (const imp of absImports) {
    const r = await probe(imp);
    if (r.bad) results.failed.push(r);
  }
}

// Hit a few known critical modules via Vite transform if entry worked
const critical = [
  "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx",
  "/@fs/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHelpers/🟦️component.tsx",
];
results.critical = [];
for (const p of critical) {
  const r = await probe(p);
  results.critical.push(r);
  if (r.bad) results.failed.push(r);
}

await Bun.write(`${ticketDir}/🧪demonstrator-e2e-probe.json`, JSON.stringify(results, null, 2));
console.log(JSON.stringify({ ready: results.ready, indexStatus: results.index?.status, entryStatus: results.entry?.status, failed: results.failed.length, failedPaths: results.failed.map((f) => f.path) }, null, 2));
