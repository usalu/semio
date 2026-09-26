#!/usr/bin/env bun
/** 🌩️ S15: fetches every file of hub plugin module bundles CONCURRENTLY and prints each failing status, byte count and time,
 * so a hub/proxy that refuses under a burst shows which file and why. `all` bursts every bundle of the index at once.
 * usage: bun s15-module-burst.ts <base (hub origin, or serve origin + /_semio/hub)> <pluginId|all> [rounds] [concurrency (0 = every file at once)] */
const [base = "http://127.0.0.1:8040", pluginId = "draw", rounds = "1", concurrencyArg = "0"] = process.argv.slice(2);
const root = `${base}/trusted-catalog/plugin-modules`;
let concurrency = Number(concurrencyArg);
const index = (await (await fetch(root)).json()) as { modules: { pluginId: string; bundleSha256: string }[] };
const bundles = index.modules.filter((row) => pluginId === "all" || row.pluginId === pluginId).map((row) => row.bundleSha256);
const files: { bundle: string; path: string; byteLength: number }[] = [];
for (const bundle of bundles) for (const file of ((await (await fetch(`${root}/${bundle}`)).json()) as { files: { path: string; byteLength: number }[] }).files) files.push({ bundle, ...file });
if (concurrency <= 0) concurrency = files.length;
for (let round = 1; round <= Number(rounds); round += 1) {
  const started = performance.now();
  const queue = [...files];
  const fetchOne = async (file: (typeof files)[number]) => {
      const t0 = performance.now();
      try {
        const response = await fetch(`${root}/${file.bundle}/${file.path.split("/").map(encodeURIComponent).join("/")}`, { cache: "no-store" });
        const bytes = (await response.arrayBuffer()).byteLength;
        return { path: file.path, status: response.status, bytes, expected: file.byteLength, ms: Math.round(performance.now() - t0), body: response.status >= 400 ? "" : "" };
      } catch (error) {
        return { path: file.path, status: -1, bytes: 0, expected: file.byteLength, ms: Math.round(performance.now() - t0), body: String(error).slice(0, 120) };
      }
  };
  const rows: Awaited<ReturnType<typeof fetchOne>>[] = [];
  await Promise.all(Array.from({ length: Math.min(concurrency, queue.length) }, async () => {
    for (let file = queue.shift(); file !== undefined; file = queue.shift()) rows.push(await fetchOne(file));
  }));
  const bad = rows.filter((row) => row.status !== 200 || row.bytes !== row.expected);
  console.log(`round ${round}: ${rows.length} files, ${rows.reduce((sum, row) => sum + row.bytes, 0)} bytes, ${bad.length} bad, slowest ${Math.max(...rows.map((row) => row.ms))} ms, ${Math.round(performance.now() - started)} ms`);
  for (const row of bad) console.log(`  BAD ${row.status} ${row.bytes}/${row.expected} ${row.ms}ms ${row.path} ${row.body}`);
}
