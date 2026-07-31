import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..", "..", "..", "..", "..", "..");
const fixturePath = join(repoRoot, ".storybook", "fixtures", "nakagin-capsule-tower.board.json");

const stripLabelFromCatalogRows = (rows) => {
  if (!Array.isArray(rows)) return rows;
  return rows.map((row) => {
    if (!row || typeof row !== "object") return row;
    const { label: _label, ...rest } = row;
    return rest;
  });
};

const fixture = JSON.parse(readFileSync(fixturePath, "utf8"));
const catalogs = fixture.meta?.kindCatalogs;
if (catalogs && typeof catalogs === "object") {
  for (const key of ["handles", "wires", "nodes", "edges"]) {
    if (Array.isArray(catalogs[key])) {
      catalogs[key] = stripLabelFromCatalogRows(catalogs[key]);
    }
  }
}

writeFileSync(fixturePath, `${JSON.stringify(fixture, null, 2)}\n`, "utf8");
console.log(`[DEBUG] stripped kind-catalog label keys from ${fixturePath}`);
