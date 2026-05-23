import { readFileSync, writeFileSync } from "node:fs";

const headPath = "c:/git/semio/.repo/🎫/26/05/19/GRAPH-QL-MUTATION-RESPONSE-TYPES/lib-head.rs";
const libPath = "c:/git/semio/semio/client/lib/rs/lib.rs";

function sliceGapRegion(src) {
  const modStart = src.indexOf("pub mod schema_gap_surfaces");
  if (modStart < 0) throw new Error("pub mod schema_gap_surfaces not found");
  const regionStart = src.lastIndexOf("//#region", modStart);
  if (regionStart < 0) throw new Error("schema_gap_surfaces region start not found");
  let regionEnd = -1;
  for (let i = modStart + 1; ; ) {
    const hit = src.indexOf("//#endregion", i);
    if (hit < 0) break;
    const lineEnd = src.indexOf("\n", hit);
    const line = src.slice(hit, lineEnd < 0 ? undefined : lineEnd);
    if (line.includes("schema_gap_surfaces")) {
      regionEnd = hit;
      break;
    }
    i = hit + 12;
  }
  if (regionEnd < 0) {
    regionEnd = src.indexOf("//#region", modStart + 1);
    if (regionEnd < 0) throw new Error("schema_gap_surfaces region end not found");
  }
  return { regionStart, regionEnd, region: src.slice(regionStart, regionEnd) };
}

const head = readFileSync(headPath, "utf8");
const lib = readFileSync(libPath, "utf8");
const headRegion = sliceGapRegion(head);
const libBounds = sliceGapRegion(lib);
const out = lib.slice(0, libBounds.regionStart) + headRegion.region + lib.slice(libBounds.regionEnd);
writeFileSync(libPath, out, "utf8");
console.log("applied lib-head schema_gap_surfaces", {
  chars: headRegion.region.length,
  libStart: libBounds.regionStart,
  libEnd: libBounds.regionEnd,
});
