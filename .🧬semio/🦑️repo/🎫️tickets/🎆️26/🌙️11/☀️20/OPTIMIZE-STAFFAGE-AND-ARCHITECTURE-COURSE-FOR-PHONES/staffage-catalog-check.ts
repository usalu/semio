import { readdirSync, readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { join } from "node:path";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import type { Asset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";

export function summarizeCatalog(catalog: string) {
  const failures: { asset: string; category: string; findings: number; codes: readonly string[] }[] = [];
  let validated = 0;
  for (const category of readdirSync(catalog, { withFileTypes: true })) {
    if (!category.isDirectory()) continue;
    const categoryDir = join(catalog, category.name);
    for (const member of readdirSync(categoryDir, { withFileTypes: true })) {
      if (!member.isDirectory()) continue;
      let document;
      try {
        document = JSON.parse(readFileSync(join(categoryDir, member.name, "🔣️.json"), "utf8")) as { readonly asset: Asset };
      } catch (error) {
        if ((error as NodeJS.ErrnoException).code === "ENOENT") continue;
        throw error;
      }
      validated++;
      const { asset } = document;
      const issues = validateAsset(asset);
      if (issues.length > 0) failures.push({
        asset: asset.id,
        category: asset.category,
        findings: issues.length,
        codes: [...new Set(issues.map(({ code }) => code))],
      });
    }
  }
  return { validated, ready: validated - failures.length, failures };
}

if (import.meta.main) {
  const catalog = fileURLToPath(new URL("../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog", import.meta.url));
  const result = summarizeCatalog(catalog);
  console.log("[DEBUG]", JSON.stringify(result));
  if (result.failures.length > 0) process.exitCode = 1;
}
