#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";

const FIXES: { path: string; handler: string }[] = [
  { path: "draw/core/js/index.ts", handler: "createDrawAppVcsHandler" },
  { path: "note/core/js/index.ts", handler: "createNoteAppVcsHandler" },
  { path: "writer/core/js/index.ts", handler: "createWriterAppVcsHandler" },
  { path: "raster/core/js/index.ts", handler: "createRasterAppVcsHandler" },
  { path: "forms/core/js/index.ts", handler: "createFormsAppVcsHandler" },
  { path: "framework/product/presentation/core/js/index.ts", handler: "createPresentationAppVcsHandler" },
];

for (const { path, handler } of FIXES) {
  const full = join(REPO, path);
  let content = readFileSync(full, "utf8");
  const needle = `import type { OsProgramContribution } from "@semio-tech/framework-platform-core";`;
  const insert = `${needle}\nimport { ${handler} } from "./internal.ts";`;
  if (!content.includes(`import { ${handler} } from "./internal.ts"`)) {
    content = content.replace(needle, insert);
    writeFileSync(full, content);
    console.log(`fixed ${path}`);
  }
}
