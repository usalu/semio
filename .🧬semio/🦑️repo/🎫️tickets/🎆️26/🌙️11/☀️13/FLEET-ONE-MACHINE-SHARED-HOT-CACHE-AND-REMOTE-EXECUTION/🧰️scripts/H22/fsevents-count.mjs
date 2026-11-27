import fs from "node:fs";
const [seconds, ...dirs] = process.argv.slice(2);
const counts = new Map(dirs.map((d) => [d, 0]));
const streams = dirs.map((d) => fs.watch(d, { recursive: true }, () => counts.set(d, counts.get(d) + 1)));
setTimeout(() => {
  for (const s of streams) s.close();
  const out = Object.fromEntries(counts);
  console.log(JSON.stringify({ seconds: Number(seconds), events: out, perTenMinutes: Object.fromEntries([...counts].map(([d, n]) => [d, Math.round((n * 600) / Number(seconds))])) }, null, 1));
  process.exit(0);
}, Number(seconds) * 1000);
