/** 🧪️ Runs the logic of the suite test "a manifest row may name the lower crate module that already owns its generated Rust module" and the three new invalid-fixture rows without `bun test` (bun test segfaults at start in this checkout). */
import Ajv from "ajv";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { parseGraphOutputCatalog } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📇️catalog/🟦️.ts";
import { renderGraphArtifacts } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📽️projection/🟦️.ts";
import fixture from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🕸️graph/🧪️tests/🧫️fixtures/🔣️outputs.json";
import schema from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🧬️schema/🔣️.json";

const must = (ok: boolean, what: string) => { if (!ok) { console.error("FAIL", what); process.exit(1); } console.log("ok  ", what); };
const validate = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/Outputs`)!;
const sandbox = mkdtempSync("/Users/ueli/.cache/tad/art/graph-rust-owner-");
try {
  mkdirSync(join(sandbox, "owners"), { recursive: true });
  for (const doc of fixture.emission) writeFileSync(join(sandbox, "owners", `${doc.id}.manifest.json`), JSON.stringify(doc));
  const input = { ...fixture.catalog, manifests: fixture.catalog.manifests.map((row) => row.id === "chronology" ? { ...row, rustOwner: "lower_crate::standards::graph_manifest::chronology" } : row) };
  must(validate(input) === true, "schema admits rustOwner");
  const registry = renderGraphArtifacts(sandbox, join(sandbox, "output"), parseGraphOutputCatalog(input, fixture.manifestIds), false).artifacts.find((a) => a.path.endsWith("📇️registry/🦀️.rs"))!.content;
  must(registry.includes("pub use lower_crate::standards::graph_manifest::chronology;"), "registry re-exports the owner module");
  must(!registry.includes("pub mod chronology;"), "registry does not mount the owned module");
  must(registry.includes("pub mod measurement;"), "registry still mounts the other module");
  must(registry.includes("chronology::chronology_manifest()"), "registry still calls the manifest function");
  const fixtureCatalog = parseGraphOutputCatalog(fixture.catalog, fixture.manifestIds);
  const plain = renderGraphArtifacts(sandbox, join(sandbox, "plain"), fixtureCatalog, false).artifacts.find((a) => a.path.endsWith("📇️registry/🦀️.rs"))!.content;
  must(plain.includes("pub mod chronology;") && !plain.includes("pub use"), "no owner: unchanged mount form");
  for (const row of fixture.invalid.filter((r) => r.name.startsWith("rust-owner"))) {
    const invalid = structuredClone(fixture.catalog) as unknown as Record<string | number, unknown>;
    let owner = invalid;
    for (const key of row.path.slice(0, -1)) owner = owner[key] as Record<string | number, unknown>;
    owner[row.path.at(-1)!] = row.value;
    let threw = false;
    try { parseGraphOutputCatalog(invalid, fixture.manifestIds); } catch { threw = true; }
    must(validate(invalid) === !row.schemaInvalid && threw, `invalid row ${row.name}: schemaInvalid=${row.schemaInvalid}, parse throws`);
  }
} finally { rmSync(sandbox, { recursive: true, force: true }); }
