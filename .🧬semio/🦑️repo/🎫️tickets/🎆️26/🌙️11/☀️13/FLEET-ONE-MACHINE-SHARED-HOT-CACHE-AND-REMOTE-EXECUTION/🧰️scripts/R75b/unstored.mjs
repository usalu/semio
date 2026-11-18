import { readFileSync, readdirSync, statSync } from "node:fs";
import { join } from "node:path";
const R = "/Users/ueli/Documents/semio", tmp = process.argv[2], sample = Number(process.argv[3] ?? 80);
const waves = readdirSync(tmp).filter(n => n.startsWith("semio-owner-pool-")).flatMap(n => readdirSync(join(tmp, n)).map(w => join(tmp, n, w))).filter(d => { try { statSync(join(d, "head.json")); return true; } catch { return false; } });
const newest = waves.map(d => ({ d, t: JSON.parse(readFileSync(join(d, "head.json"), "utf8")).createdMs })).sort((a, b) => b.t - a.t)[0].d;
const head = JSON.parse(readFileSync(join(newest, "head.json"), "utf8")), units = JSON.parse(readFileSync(join(newest, "units.json"), "utf8")), session = readFileSync(join(newest, "session.bin"));
const L = R + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/";
const { openOwnerPoolInferenceV1 } = await import(L + "🔬️inference/🟨️.mjs");
const environment = { ...process.env };
const open = await openOwnerPoolInferenceV1({ root: R, fleetRoot: R, environment, session, globals: head.globals, units });
const counts = { stored: 0, held: 0, granted: 0, fault: 0, refused: 0 }, picks = [];
for (let i = 0; i < sample; i++) picks.push(units[Math.floor(Math.random() * units.length)]);
for (const unit of picks) {
  const a = await open.claim(unit); counts[a.state] = (counts[a.state] ?? 0) + 1;
  if (a.state === "granted") await a.abandon();
}
open.close?.();
console.log(JSON.stringify({ wave: head.id.slice(0, 8), units: units.length, sample, ...counts, unstoredShare: Math.round((counts.held + counts.granted) / sample * 1000) / 10, unstoredEstimate: Math.round((counts.held + counts.granted) / sample * units.length) }));
process.exit(0);
