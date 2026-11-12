/** 🌿️ Ticket-local aids for authoring the vegetation volumes: `inspect <slug>` prints bones in the world rest frame and the 1:50 shapes, `viz <slug> [volume.json]` writes overlay PNGs of the authored silhouette against the volume. */
import { readFileSync, writeFileSync } from "node:fs";
import { deflateSync } from "node:zlib";
import { join } from "node:path";
import { VIEWS, GENERATED, Mask, bboxOf, frames, loadAsset, meshes, projectWorld, resolveAsset, restState, type Ring } from "./volume-lib.ts";

const [cmd, slug, volumeFile] = process.argv.slice(2);
const entry = resolveAsset(slug!);
const asset = loadAsset(entry.path);
const f3 = (n: number) => (Math.round(n * 1000) / 1000).toString();

if (cmd === "inspect") {
  const fr = frames(asset);
  console.log(JSON.stringify(asset.dimensions));
  console.log("BONES id parent | origin(x,y,z) u(x,y,z) w(x,y,z) len");
  for (const b of asset.rig.bones) {
    const f = fr[b.id]!;
    console.log(`${b.id} <${b.parent ?? "-"}> o(${f.o.map(f3)}) u(${f.u.map(f3)}) w(${f.w.map(f3)}) L${b.lengthM} ${b.joint?.type}`);
  }
  console.log("MATERIALS", asset.materials.map((m: any) => m.id).join(" "));
  console.log("REGIONS", asset.thermal.regions.map((r: any) => `${r.id}:${r.material}`).join(" "));
  console.log("POSES", asset.poses.map((p: any) => p.id).join(" "), "CLIPS", asset.clips.map((c: any) => c.id).join(" "));
  const st = restState(asset, 50);
  for (const v of VIEWS) {
    console.log(`VIEW ${v} (paper coords)`);
    const rows: string[] = [];
    for (const s of st[v]) {
      if (!["body", "detail", "glazing"].includes(s.shape.role)) continue;
      const pts = s.rings.flat();
      const b = bboxOf(pts);
      rows.push(`${s.shape.id}|${s.shape.bone}|${s.shape.role}|${s.shape.kind}${s.closed ? "" : "(open)"}|x${f3(b.minX)}..${f3(b.maxX)} y${f3(b.minY)}..${f3(b.maxY)}|tone${s.shape.tone}|${s.shape.region ?? s.shape.regions ?? ""}|${s.shape.material ?? ""}`);
    }
    console.log(rows.join("\n"));
  }
} else if (cmd === "viz") {
  const volume = JSON.parse(readFileSync(volumeFile!, "utf8"));
  const solids = (volume.volume ?? volume).solids;
  const st = restState(asset, 50);
  const list = meshes(asset, solids, Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM));
  for (const v of VIEWS) {
    const shapes = st[v].filter((s) => s.closed && ["body", "detail", "glazing"].includes(s.shape.role));
    const pts = shapes.flatMap((s) => s.rings.flat());
    const vp: { x: number; y: number }[] = [];
    const proj = list.map((m) => { const o: { x: number; y: number }[] = []; for (let i = 0; i < m.world.length; i += 3) { const p = projectWorld(v, [m.world[i]!, m.world[i + 1]!, m.world[i + 2]!]); o.push(p); vp.push(p); } return o; });
    const b = bboxOf([...pts, ...vp]);
    const cell = Math.max(b.maxX - b.minX, b.maxY - b.minY) / 560;
    const box = { minX: b.minX - 4 * cell, minY: b.minY - 4 * cell, maxX: b.maxX + 4 * cell, maxY: b.maxY + 4 * cell };
    const A = new Mask(box, cell);
    const V = new Mask(box, cell);
    for (const s of shapes) A.fill(s.rings);
    list.forEach((m, n) => { for (let t = 0; t < m.indices.length; t += 3) V.fill([[0, 1, 2].map((k) => proj[n]![m.indices[t + k]!]!) as Ring]); });
    const W = A.width, H = A.height;
    const raw = Buffer.alloc((W * 3 + 1) * H);
    for (let j = 0; j < H; j++) {
      raw[j * (W * 3 + 1)] = 0;
      for (let i = 0; i < W; i++) {
        const a = A.cells[(H - 1 - j) * W + i]!, c = V.cells[(H - 1 - j) * W + i]!;
        const col = a && c ? [120, 120, 120] : a ? [80, 120, 255] : c ? [255, 90, 70] : [250, 250, 250];
        raw.set(col, j * (W * 3 + 1) + 1 + i * 3);
      }
    }
    const crc = (buf: Buffer) => { let c = ~0; for (const x of buf) { c ^= x; for (let k = 0; k < 8; k++) c = (c >>> 1) ^ (0xedb88320 & -(c & 1)); } return ~c >>> 0; };
    const chunk = (t: string, d: Buffer) => { const len = Buffer.alloc(4); len.writeUInt32BE(d.length); const td = Buffer.concat([Buffer.from(t), d]); const c = Buffer.alloc(4); c.writeUInt32BE(crc(td)); return Buffer.concat([len, td, c]); };
    const ihdr = Buffer.alloc(13); ihdr.writeUInt32BE(W, 0); ihdr.writeUInt32BE(H, 4); ihdr[8] = 8; ihdr[9] = 2;
    const png = Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), chunk("IHDR", ihdr), chunk("IDAT", deflateSync(raw)), chunk("IEND", Buffer.alloc(0))]);
    const out = join(GENERATED, "authoring-vegetation", `${slug}-${v}.png`);
    writeFileSync(out, png);
    console.log(out, `${W}x${H}`, `x ${f3(b.minX)}..${f3(b.maxX)} y ${f3(b.minY)}..${f3(b.maxY)}`);
  }
}
