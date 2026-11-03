/** 📌 Re-records the pinned SHA-256 digests of the export-performance fixture after the intentional compact frame form: `bun repin.ts <fixture> <cast-export fixture> <out>`; prints which casts changed. */
import { createHash } from "node:crypto";
import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { exportCast } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts";

const [fixturePath, castPath, out] = process.argv.slice(2);
const vectors = JSON.parse(readFileSync(fixturePath!, "utf8"));
const manifest = JSON.parse(readFileSync(castPath!, "utf8")).manifest;
const engineIndependent = (svg: string): string => svg.replace(/data-value="([^"]*)"/gu, (_, v: string) => `data-value="${Number(Number(v).toPrecision(4))}"`);
const ids = Object.keys(vectors.castDigests);
const indexes = await exportCast(manifest, ids, out!);
const changed: string[] = [];
for (const index of indexes) {
  const pinned = vectors.castDigests[index.cast];
  const files = readdirSync(join(out!, index.cast)).sort();
  if (JSON.stringify(files) !== JSON.stringify(Object.keys(pinned.sha256).sort())) throw new Error(`${index.cast}: frame files differ`);
  for (const name of files) {
    const digest = createHash("sha256").update(engineIndependent(readFileSync(join(out!, index.cast, name), "utf8"))).digest("hex");
    if (digest !== pinned.sha256[name]) {
      pinned.sha256[name] = digest;
      if (!changed.includes(index.cast)) changed.push(index.cast);
    }
  }
}
writeFileSync(fixturePath!, JSON.stringify(vectors, null, 2) + "\n", "utf8");
console.log("changed", changed.join(", "));
