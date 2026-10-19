const L = "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery";
const tax = await import(L + "/🔐️taxonomy-authority/🟦️.ts");
const proj = await import(L + "/🧲️registry-catalog-projection/🟦️.ts");
const root = process.cwd();
let t = performance.now();
const lap = (n: string) => { const d = performance.now() - t; console.log("[DEBUG]", n, d.toFixed(0), "ms"); t = performance.now(); return d; };
const loc = await import(L + "/🧭️workspace-taxonomy-locator/🟦️.ts"); const taxonomy = loc.readTaxonomyUnchecked(); lap("loadTaxonomy");
const pruned = proj.discoverCatalogPackages(root, taxonomy); lap("pruned");
if (process.argv[2] === "full") {
  const full = proj.discoverCatalogPackages(root, taxonomy, tax.registryCatalogInputView(root, taxonomy)); lap("full");
  const key = (rows: any[]) => JSON.stringify(rows.map((r) => [r.ownerRel, r.lang, r.target ?? null, r.packageRel, r.manifestPath, r.role, r.id]));
  console.log("[DEBUG] packages pruned", pruned.length, "full", full.length, "equal", key(pruned) === key(full));
} else console.log("[DEBUG] packages pruned", pruned.length);
