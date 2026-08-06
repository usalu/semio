import { readdirSync, existsSync } from "fs";
import { join } from "path";

const fw = readdirSync(".").find((n) => n.includes("framework") && !n.startsWith("."));
const pkgDir = join(fw, "🛍️products", "🦑️repo", "🔨️modules", "📚️lib", "📦️packages");
const pkg = readdirSync(pkgDir).find((n) => n.includes("typescript"));
const indexPath = join(pkgDir, pkg, "📦️index.ts");
console.log("loading", indexPath, existsSync(indexPath));

const { loadFrameworkOsPlaygroundCatalog, resolveFrameworkOsPlaygroundPlugin } = await import(join(process.cwd(), indexPath));

const catalog = loadFrameworkOsPlaygroundCatalog();
console.log("catalog size", catalog.length);
console.log(
  "variants",
  catalog
    .map((r) => r.variant)
    .sort()
    .join(", "),
);

for (const segs of [
  ["process", "3d"],
  ["cad"],
  ["puzzle", "3d"],
  ["3d"],
  ["s"],
  ["gis", "2d"],
  ["animate"],
  ["flow"],
  ["procedural", "3d"],
  ["trinity", "jack"],
  ["sourcing"],
  ["playbook"],
]) {
  console.log(JSON.stringify(segs), "=>", resolveFrameworkOsPlaygroundPlugin(catalog, segs));
}

const processRows = catalog.filter((r) => r.variant.includes("process") || r.aliases.some((a) => a.includes("process")));
console.log("process rows", JSON.stringify(processRows, null, 2));
