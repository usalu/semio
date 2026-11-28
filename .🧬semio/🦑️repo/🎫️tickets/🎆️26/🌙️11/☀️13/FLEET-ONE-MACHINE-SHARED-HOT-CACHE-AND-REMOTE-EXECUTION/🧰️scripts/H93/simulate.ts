import { readdirSync, readFileSync } from "node:fs";
import { allocateV1, GIB, type Input, type PurposeIn } from "./allocator.ts";

const STORE = process.argv[2] ?? "/Users/Shared/semio-store/v1/origins";
const COSTS: Record<string, number> = { r: 900000, d: 120000, g: 300000, p: 240000, t: 60000, n: 400000, o: 30000, l: 1000 };
const FLOORS: Record<string, number> = { r: 4, d: 2, g: 1, p: 2, t: 1, n: 0.25, o: 0.25, l: 0.0625 };
const ALIGN: Record<string, number> = { r: GIB, d: GIB, g: GIB, p: GIB, t: GIB, n: GIB / 4, o: GIB / 4, l: GIB / 16 };
const PROFILES: Record<string, Record<string, number[]>> = {
  stale: { r: [5, 5, 10, 15, 25, 40], d: [2, 3, 5, 10, 30, 50], g: [10, 10, 10, 20, 25, 25], p: [5, 5, 10, 20, 30, 30], t: [1, 2, 5, 12, 30, 50], x: [10, 15, 25, 25, 15, 10] },
  hot: { r: [30, 25, 20, 10, 10, 5], d: [20, 20, 20, 15, 15, 10], g: [30, 25, 15, 10, 10, 10], p: [25, 25, 20, 10, 10, 10], t: [20, 20, 20, 15, 15, 10], x: [30, 25, 20, 10, 10, 5] },
};
const CAP: Record<string, number> = { darwin: 32 * GIB, linux: 128 * GIB, win32: 64 * GIB };
const HOST_TOTAL = 228 * GIB;
const FREES = [12, 25, 40].map((v) => v * GIB);
const g = (b: number) => (b / GIB).toFixed(1);

type Row = { origin: string; platform: string; purposes: { name: string; capacity: number; content: number; partial: boolean }[] };
const rows: Row[] = [];
for (const o of readdirSync(STORE).sort()) {
  let d: any;
  try { d = JSON.parse(readFileSync(`${STORE}/${o}/partitions.json`, "utf8")); } catch { continue; }
  const purposes = d.partitions
    .filter((p: any) => /^[dglnoprt]$/.test(p.name))
    .map((p: any) => {
      const fixed = (p.top ?? []).find((t: any) => t.path === "fixed.dmg")?.bytes ?? 0;
      return { name: p.name, capacity: p.capacityBytes ?? 0, content: Math.max(0, p.bytes - fixed), partial: !!p.partial };
    });
  rows.push({ origin: d.origin, platform: d.platform, purposes });
}

function build(row: Row, profile: string, free: number): Input {
  const cap = CAP[row.platform] ?? 32 * GIB;
  const ps: PurposeIn[] = row.purposes.map((p) => {
    const frac = (PROFILES[profile][p.name] ?? PROFILES[profile].x);
    const sum = frac.reduce((s, v) => s + v, 0);
    const buckets = frac.map((v) => Math.floor((p.content * v) / sum));
    const oversize = p.capacity > cap;
    return {
      name: p.name, floorBytes: Math.floor(FLOORS[p.name] * GIB), alignBytes: ALIGN[p.name], costMsPerGiB: COSTS[p.name],
      peakBytes: Math.floor(p.content * (p.partial ? 1.3 : 1.1)), imageless: false, buckets,
      images: p.capacity > 0 ? [{ id: p.name, capacityBytes: p.capacity, usedBytes: p.content, physicalBytes: Math.floor(p.content * (oversize ? 1.3 : 1.05)), unmappable: !oversize && row.platform !== "win32" || row.platform === "win32" }] : [],
    };
  });
  return {
    host: { totalBytes: HOST_TOTAL, freeBytes: free },
    policy: { reserveFloorBytes: 10 * GIB, reserveFractionPercent: 8, maxSharePercent: 60, headroomNum: 1, headroomDen: 4, imageCapBytes: cap, shardFillNum: 7, shardFillDen: 8 },
    purposes: ps,
  };
}

const out: string[] = [];
out.push("| origin | platform | declared Σcap GiB | content GiB | profile | host free GiB | budget GiB | evicted GiB | Σtarget GiB | images before→after | reshard (oversize) | grow | shrink | unmet |");
out.push("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|");
const detail: string[] = [];
for (const row of rows) {
  const declared = row.purposes.reduce((s, p) => s + p.capacity, 0);
  const content = row.purposes.reduce((s, p) => s + p.content, 0);
  for (const profile of ["stale", "hot"]) {
    for (const free of FREES) {
      const inp = build(row, profile, free);
      const o = allocateV1(inp);
      const ev = o.evictions.reduce((s, e) => s + e.bytes, 0);
      const k = (kind: string) => o.actions.filter((a) => a.kind === kind).length;
      const before = inp.purposes.reduce((s, p) => s + p.images.length, 0);
      const afterImgs = o.purposes.reduce((s, p) => s + p.shards.length, 0);
      out.push(`| ${row.origin} | ${row.platform} | ${g(declared)} | ${g(content)} | ${profile} | ${g(free)} | ${g(o.budgetBytes)} | ${g(ev)} | ${g(o.purposes.reduce((s, p) => s + p.targetBytes, 0))} | ${before}→${afterImgs} | ${k("birth")} | ${k("grow")} | ${k("shrink")} | ${o.unmet ? "yes " + g(o.deficitBytes) : "no"} |`);
      if (row.origin === "g1-m01" && profile === "stale" && free === 25 * GIB) {
        detail.push("m01, stale profile, 25 GiB free: per purpose");
        detail.push("| purpose | capacity GiB today | used GiB | used after | target GiB | shards (GiB) | action |");
        detail.push("|---|---|---|---|---|---|---|");
        o.purposes.forEach((p) => {
          const acts = o.actions.filter((a) => a.purpose === p.name).map((a) => a.kind).join("+");
          detail.push(`| ${p.name} | ${g(row.purposes.find((q) => q.name === p.name)!.capacity)} | ${g(p.usedBeforeBytes)} | ${g(p.usedAfterBytes)} | ${g(p.targetBytes)} | ${p.shards.map(g).join(", ")} | ${acts} |`);
        });
        detail.push("evictions: " + o.evictions.map((e) => `${e.purpose}/b${e.bucket} ${g(e.bytes)} GiB`).join("; "));
      }
    }
  }
}
console.log(out.join("\n") + "\n\n" + detail.join("\n"));
