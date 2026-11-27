/** 🧮️ The budget facts of catalog assets without the file audit: per asset one JSON line with the triangles of every scene (the count of `auditGeometry`), the bytes of the GLB of the default pose and the budget findings they give (triangles above 400 000 at the finest scale, triangles growing towards a coarser scale, a GLB above 8 MB). Appends to <out.jsonl> and skips the ids it already holds. budgets.ts <out.jsonl> [--glb] [--from <n>] [--to <n>] [ids… | file.json…] */
import { appendFileSync, existsSync, readFileSync } from "node:fs";
import { scenesOf, tessellateSolid } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { modelOfAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts";
import { qualityOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import { writeGlb } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🧊️glb/🟦️.ts";

const [out, ...rest] = process.argv.slice(2);
const flag = (name: string): string | undefined => (rest.includes(name) ? rest[rest.indexOf(name) + 1] : undefined);
const glb = rest.includes("--glb");
const named = rest.filter((a, i) => !a.startsWith("--") && !["--from", "--to"].includes(rest[i - 1] ?? ""));
const entries = await assetPathsFromFiles();
const done = new Set(existsSync(out!) ? readFileSync(out!, "utf8").split("\n").filter((l) => l.length > 0).map((l) => JSON.parse(l).id as string) : []);
const targets = named.length > 0 ? named : entries.slice(Number(flag("--from") ?? 0), Number(flag("--to") ?? entries.length)).map((e) => e.folder);
for (const target of targets) {
  const started = Date.now();
  const asset = target.endsWith(".json") ? JSON.parse(readFileSync(target, "utf8")).asset : (await readStreamedAsset(entries.find((e) => e.folder === target)!)).asset;
  if (done.has(asset.id) && named.length === 0) continue;
  try {
    const document = await modelOfAsset(asset, { poses: "all", clips: [] });
    const scenes: Record<string, number> = {};
    for (const scene of scenesOf(document)) {
      const quality = qualityOf(asset, Number(/^s(\d+)-/u.exec(scene.id)![1]));
      const members = new Set(scene.nodes);
      scenes[scene.id] = document.solids.filter((s) => members.has(s.node)).reduce((n, s) => n + tessellateSolid(s, quality).indices.length / 3, 0);
    }
    const findings: string[] = [];
    for (const mode of ["body", "thermal", "structure"]) {
      const row = Object.entries(scenes).filter(([id]) => id.endsWith(`-${mode}`)).map(([id, triangles]) => ({ scale: Number(/^s(\d+)-/u.exec(id)![1]), triangles })).sort((a, b) => a.scale - b.scale);
      for (let i = 1; i < row.length; i++) if (row[i]!.triangles > row[i - 1]!.triangles) findings.push(`${mode} grows 1:${row[i - 1]!.scale} ${row[i - 1]!.triangles} -> 1:${row[i]!.scale} ${row[i]!.triangles}`);
      if (row[0] !== undefined && row[0].triangles > 400_000) findings.push(`${row[0].triangles} ${mode} triangles at 1:${row[0].scale}`);
    }
    const glbBytes = glb ? (await writeGlb(asset, { pose: asset.presentation.defaultPose })).length : null;
    if (glbBytes !== null && glbBytes > 8 * 1024 * 1024) findings.push(`GLB ${(glbBytes / 1048576).toFixed(2)} MB`);
    appendFileSync(out!, JSON.stringify({ id: asset.id, category: asset.category, scenes, glbBytes, findings, seconds: Math.round((Date.now() - started) / 1000) }) + "\n");
    console.log(`${asset.id} ${findings.length === 0 ? "ok" : findings.join("; ")} (${glbBytes === null ? "" : `${(glbBytes / 1048576).toFixed(2)} MB, `}${Math.round((Date.now() - started) / 1000)} s)`);
  } catch (error) {
    appendFileSync(out!, JSON.stringify({ id: asset.id, error: String(error) }) + "\n");
    console.log(`${asset.id} ERROR ${String(error).slice(0, 200)}`);
  }
  (globalThis as { Bun?: { gc(force: boolean): void } }).Bun?.gc(true);
}
