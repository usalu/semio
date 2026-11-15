import { entryOf, loadSource, merged } from "./volumes-lib.ts";
import { rulingMeasurements } from "./volumes-rulings.ts";
const { entry, asset } = entryOf(process.argv[2]!);
const source = loadSource(entry)!;
const a = merged(asset, source);
const probe = { ...a, thermal: { ...a.thermal, regions: a.thermal.regions.map((r: any) => ({ ...r, areaM2: 1000 })) } };
const ms = rulingMeasurements(probe, source, { rules: ["volumes-thermal-coverage"] });
const out: Record<string, Record<string, number>> = {};
for (const m of ms) if (m.name.startsWith("areaDeviation")) { const x = /exposed ([0-9.]+) m2/.exec(m.note ?? ""); if (x) (out[m.scale!] ??= {})[m.name.split(":")[1]!] = Number(x[1]); }
const want = Object.fromEntries((a.thermal.regions as any[]).map((r) => [r.id, r.areaM2]));
for (const [s, rs] of Object.entries(out)) console.log(s, Object.entries(rs).map(([k, v]) => `${k}=${v}(${want[k]})`).join("  "));
