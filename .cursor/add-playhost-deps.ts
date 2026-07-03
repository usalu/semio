#!/usr/bin/env bun
import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";

const REPO = "/Users/ueli/Documents/semio";

const TARGETS = [
  "puzzle/2d/react",
  "puzzle/3d/react",
  "puzzle/5d/react",
  "gis/2d/react",
  "flow/react",
  "mathematical/graph/port/directed/dag/react",
  "imperative/react",
  "sequence/react",
  "layout/react",
  "lowpoly/react",
  "trinity/react",
  "procedural/3d/react",
  "procedural/2d/react",
  "shooting/react",
  "forms/react",
  "raster/react",
  "draw/react",
  "note/react",
  "cad/renderer/react",
  "vcs/react",
  "writer/react",
  "framework/product/presentation/renderer/react",
  "s/react",
];

for (const dir of TARGETS) {
  const indexPath = join(REPO, dir, "index.tsx");
  const pkgPath = join(REPO, dir, "package.json");
  if (!existsSync(indexPath) || !existsSync(pkgPath)) continue;
  const content = readFileSync(indexPath, "utf8");
  const start = content.lastIndexOf("//#region 🔖");
  const playHostStart = content.indexOf("PlayHost", start);
  if (playHostStart < 0) continue;
  const regionStart = content.lastIndexOf("//#region 🔖", playHostStart);
  const region = content.slice(regionStart);
  const pkg = JSON.parse(readFileSync(pkgPath, "utf8"));
  const ownName = pkg.name as string;
  pkg.dependencies ??= {};
  const importRe = /from\s+["'](@semio-tech\/[^"']+)["']/g;
  let m: RegExpExecArray | null;
  const deps = new Set<string>();
  while ((m = importRe.exec(region))) {
    if (m[1] !== ownName) deps.add(m[1]);
  }
  deps.add("@semio-tech/framework-playground-renderer-react");
  for (const dep of deps) {
    if (!pkg.dependencies[dep]) {
      pkg.dependencies[dep] = "workspace:*";
    }
  }
  writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + "\n");
  console.log(`${dir}: added ${[...deps].filter((d) => !JSON.parse(readFileSync(pkgPath, "utf8")).dependencies?.[d]).length || deps.size} deps`);
}
