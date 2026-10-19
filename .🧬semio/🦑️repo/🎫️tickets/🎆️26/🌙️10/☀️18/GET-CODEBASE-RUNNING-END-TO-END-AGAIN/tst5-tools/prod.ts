import { readModuleTreeSource } from "../../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { readFileSync } from "node:fs";
const law = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
for (const p of law.producerPaths) {
  const s = readModuleTreeSource("/Users/ueli/Documents/semio/" + p);
  const a = s.indexOf("fn advance(&mut self, grant:"), e = s.indexOf("fn checkpoint(", a);
  const act = s.slice(a, e);
  console.log(p.slice(-80), a, e, act.includes("Result<store::os_store::one_item::ArtifactStoreOneItemPreparationStep, semio_framework_value::ValueError>"));
}
