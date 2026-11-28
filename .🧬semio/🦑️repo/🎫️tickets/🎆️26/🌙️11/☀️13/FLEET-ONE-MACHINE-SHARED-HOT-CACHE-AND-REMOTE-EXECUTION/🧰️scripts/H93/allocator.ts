export const GIB = 2 ** 30;
export const BUCKET_AGE_MS = [300000, 2100000, 12600000, 54000000, 345600000, 1209600000] as const;

export type Policy = {
  reserveFloorBytes: number;
  reserveFractionPercent: number;
  maxSharePercent: number;
  headroomNum: number;
  headroomDen: number;
  imageCapBytes: number;
  shardFillNum: number;
  shardFillDen: number;
};
export type ImageIn = { id: string; capacityBytes: number; usedBytes: number; physicalBytes: number; unmappable: boolean };
export type PurposeIn = {
  name: string;
  floorBytes: number;
  alignBytes: number;
  costMsPerGiB: number;
  peakBytes: number;
  imageless: boolean;
  buckets: number[];
  images: ImageIn[];
};
export type Input = { host: { totalBytes: number; freeBytes: number }; policy: Policy; purposes: PurposeIn[] };
export type Action = { kind: string; purpose: string; image: string; fromBytes: number; toBytes: number };
export type Output = {
  budgetBytes: number;
  reserveBytes: number;
  physicalNowBytes: number;
  unmet: boolean;
  deficitBytes: number;
  evictions: { purpose: string; bucket: number; bytes: number }[];
  purposes: { name: string; usedBeforeBytes: number; usedAfterBytes: number; commitBytes: number; targetBytes: number; shards: number[] }[];
  actions: Action[];
};

const ceilDiv = (a: number, b: number) => Math.floor((a + b - 1) / b);
const alignUp = (v: number, a: number) => ceilDiv(v, a) * a;
const alignDown = (v: number, a: number) => Math.floor(v / a) * a;

export function allocateV1(input: Input): Output {
  const { host, policy: p } = input;
  const ps = input.purposes;
  const commit = (u: number, q: PurposeIn) =>
    alignUp(Math.max(q.floorBytes, u + Math.floor((u * p.headroomNum) / p.headroomDen)), q.alignBytes);
  const physicalNowBytes = ps.reduce((s, q) => s + q.images.reduce((t, i) => t + i.physicalBytes, 0), 0);
  const reserveBytes = Math.max(p.reserveFloorBytes, Math.floor((host.totalBytes * p.reserveFractionPercent) / 100));
  const maxShare = Math.floor((host.totalBytes * p.maxSharePercent) / 100);
  const budgetBytes = Math.max(0, Math.min(maxShare, physicalNowBytes + host.freeBytes - reserveBytes));

  const used = ps.map((q) => q.buckets.reduce((s, b) => s + b, 0));
  const usedBefore = used.slice();
  const rem = ps.map((q) => q.buckets.slice());
  const total = () => ps.reduce((s, q, i) => s + commit(used[i], q), 0);

  const cand: { pi: number; b: number }[] = [];
  ps.forEach((q, pi) => q.buckets.forEach((_, b) => b > 0 && cand.push({ pi, b })));
  cand.sort((x, y) => {
    const l = BUCKET_AGE_MS[x.b] * ps[y.pi].costMsPerGiB;
    const r = BUCKET_AGE_MS[y.b] * ps[x.pi].costMsPerGiB;
    if (l !== r) return r - l;
    if (ps[x.pi].name !== ps[y.pi].name) return ps[x.pi].name < ps[y.pi].name ? -1 : 1;
    return y.b - x.b;
  });

  const evictions: Output["evictions"] = [];
  for (const c of cand) {
    while (total() > budgetBytes && rem[c.pi][c.b] > 0) {
      const deficit = total() - budgetBytes;
      const e = Math.min(rem[c.pi][c.b], Math.max(ps[c.pi].alignBytes, ceilDiv(deficit * p.headroomDen, p.headroomDen + p.headroomNum)));
      rem[c.pi][c.b] -= e;
      used[c.pi] -= e;
      const last = evictions[evictions.length - 1];
      if (last && last.purpose === ps[c.pi].name && last.bucket === c.b) last.bytes += e;
      else evictions.push({ purpose: ps[c.pi].name, bucket: c.b, bytes: e });
    }
  }
  const after = total();
  const unmet = after > budgetBytes;
  const deficitBytes = unmet ? after - budgetBytes : 0;

  const commits = ps.map((q, i) => commit(used[i], q));
  const extra = ps.map((q, i) => Math.max(0, commit(Math.max(q.peakBytes, used[i]), q) - commits[i]));
  const sumExtra = extra.reduce((s, v) => s + v, 0);
  const spare = Math.max(0, budgetBytes - after);
  const targets = ps.map((q, i) => {
    if (sumExtra === 0 || spare === 0) return commits[i];
    const share = sumExtra <= spare ? extra[i] : Math.floor((spare * extra[i]) / sumExtra);
    return commits[i] + alignDown(Math.min(share, extra[i]), q.alignBytes);
  });

  const actions: Action[] = [];
  const outPurposes: Output["purposes"] = [];
  ps.forEach((q, i) => {
    if (q.imageless) {
      actions.push({ kind: "quota", purpose: q.name, image: "-", fromBytes: usedBefore[i], toBytes: targets[i] });
      outPurposes.push({ name: q.name, usedBeforeBytes: usedBefore[i], usedAfterBytes: used[i], commitBytes: commits[i], targetBytes: targets[i], shards: [] });
      return;
    }
    const shardCap = alignDown(Math.floor((p.imageCapBytes * p.shardFillNum) / p.shardFillDen), q.alignBytes);
    const n = Math.max(1, ceilDiv(targets[i], shardCap));
    const units = targets[i] / q.alignBytes;
    const shards = Array.from({ length: n }, (_, k) => (Math.floor(units / n) + (k < units % n ? 1 : 0)) * q.alignBytes);
    outPurposes.push({ name: q.name, usedBeforeBytes: usedBefore[i], usedAfterBytes: used[i], commitBytes: commits[i], targetBytes: targets[i], shards });
    const imgs = q.images.slice().sort((a, b) => (a.id < b.id ? -1 : 1));
    if (imgs.some((im) => im.capacityBytes > p.imageCapBytes)) {
      imgs.forEach((im) => actions.push({ kind: "retire", purpose: q.name, image: im.id, fromBytes: im.capacityBytes, toBytes: 0 }));
      shards.forEach((s, k) => actions.push({ kind: "birth", purpose: q.name, image: `${q.name}#${k}`, fromBytes: 0, toBytes: s }));
      return;
    }
    shards.forEach((s, k) => {
      const im = imgs[k];
      if (!im) return actions.push({ kind: "birth", purpose: q.name, image: `${q.name}#${k}`, fromBytes: 0, toBytes: s });
      if (s > im.capacityBytes) actions.push({ kind: "grow", purpose: q.name, image: im.id, fromBytes: im.capacityBytes, toBytes: s });
      else if (im.unmappable && im.capacityBytes * 2 > s * 3 && im.capacityBytes - s >= 2 * q.alignBytes)
        actions.push({ kind: "shrink", purpose: q.name, image: im.id, fromBytes: im.capacityBytes, toBytes: s });
      else actions.push({ kind: "keep", purpose: q.name, image: im.id, fromBytes: im.capacityBytes, toBytes: im.capacityBytes });
    });
    imgs.slice(shards.length).forEach((im) => actions.push({ kind: "retire", purpose: q.name, image: im.id, fromBytes: im.capacityBytes, toBytes: 0 }));
  });

  return { budgetBytes, reserveBytes, physicalNowBytes, unmet, deficitBytes, evictions, purposes: outPurposes, actions };
}
