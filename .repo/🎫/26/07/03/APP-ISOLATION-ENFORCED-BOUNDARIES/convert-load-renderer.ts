#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";

const CORE_UPDATES: Array<{ path: string; pkg: string; exportName: string }> = [
  { path: "flow/core/js/index.ts", pkg: "@semio-tech/flow-react", exportName: "flowAppRenderer" },
  { path: "draw/core/js/index.ts", pkg: "@semio-tech/draw-react", exportName: "drawAppRenderer" },
  { path: "note/core/js/index.ts", pkg: "@semio-tech/note-react", exportName: "noteAppRenderer" },
  { path: "s/core/js/index.ts", pkg: "@semio-tech/s-react", exportName: "sAppRenderer" },
  { path: "puzzle/2d/core/js/index.ts", pkg: "@semio-tech/puzzle-2d-react", exportName: "puzzle2dAppRenderer" },
  { path: "reasoning/mindmap/wires/core/js/index.ts", pkg: "@semio-tech/puzzle-2d-react", exportName: "wiresAppRenderer" },
  { path: "puzzle/5d/core/js/index.ts", pkg: "@semio-tech/puzzle-5d-react", exportName: "puzzle5dAppRenderer" },
  { path: "puzzle/3d/core/js/index.ts", pkg: "@semio-tech/puzzle-3d-react", exportName: "puzzle3dAppRenderer" },
  { path: "gis/2d/core/js/index.ts", pkg: "@semio-tech/gis-2d-react", exportName: "mapAppRenderer" },
  { path: "mathematical/graph/port/directed/dag/core/js/index.ts", pkg: "@semio-tech/dag-react", exportName: "dagAppRenderer" },
  { path: "imperative/core/js/index.ts", pkg: "@semio-tech/imperative-react", exportName: "imperativeAppRenderer" },
  { path: "sequence/core/js/index.ts", pkg: "@semio-tech/sequence-react", exportName: "sequenceAppRenderer" },
  { path: "lowpoly/core/js/index.ts", pkg: "@semio-tech/lowpoly-react", exportName: "lowpolyAppRenderer" },
  { path: "trinity/rewrite/core/js/index.ts", pkg: "@semio-tech/trinity-react", exportName: "trinityRewriteAppRenderer" },
  { path: "trinity/jack/host-core/js/index.ts", pkg: "@semio-tech/trinity-react", exportName: "trinityJackAppRenderer" },
  { path: "procedural/3d/core/js/index.ts", pkg: "@semio-tech/procedural-3d-react", exportName: "proceduralAppRenderer" },
  { path: "procedural/2d/core/js/index.ts", pkg: "@semio-tech/procedural-2d-react", exportName: "procedural2dAppRenderer" },
  { path: "shooting/core/js/index.ts", pkg: "@semio-tech/shooting-react", exportName: "shootingAppRenderer" },
  { path: "forms/core/js/index.ts", pkg: "@semio-tech/forms-react", exportName: "formsAppRenderer" },
  { path: "raster/core/js/index.ts", pkg: "@semio-tech/raster-react", exportName: "rasterAppRenderer" },
  { path: "cad/renderer/core/js/index.ts", pkg: "@semio-tech/cad-js-renderer-react", exportName: "cadAppRenderer" },
  { path: "vcs/core/js/index.ts", pkg: "@semio-tech/vcs-react", exportName: "vcsAppRenderer" },
  { path: "writer/core/js/index.ts", pkg: "@semio-tech/writer-react", exportName: "writerAppRenderer" },
  { path: "framework/product/presentation/core/js/index.ts", pkg: "@semio-tech/framework-presentation-renderer-react", exportName: "presentationAppRenderer" },
];

for (const { path, pkg, exportName } of CORE_UPDATES) {
  const fullPath = join(REPO, path);
  let content = readFileSync(fullPath, "utf8");
  const replaced = content.replace(
    /\tbootRenderer: async \(pg\) => \{[\s\S]*?\n\t\},/,
    `\tloadRenderer: async () => (await import("${pkg}/play")).${exportName},`,
  );
  if (replaced === content) {
    console.log(`SKIP ${path}`);
    continue;
  }
  writeFileSync(fullPath, replaced);
  console.log(`Updated ${path}`);
}

console.log("DONE core loadRenderer");
