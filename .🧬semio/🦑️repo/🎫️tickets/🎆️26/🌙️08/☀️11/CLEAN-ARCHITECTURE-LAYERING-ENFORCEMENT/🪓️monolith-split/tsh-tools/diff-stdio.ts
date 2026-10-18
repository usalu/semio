/** 🔍️ Diagnostic for the stdio publication test: prints the factory keys of the catalog beside the source receipts. */
import { readFileSync } from "node:fs";
import { buildTrustedStdioCatalogV1 } from "/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/🗄️stdio/📇️publication/✅️trusted-stdio-catalog/🟦️.ts";
const source = JSON.parse(readFileSync("/Users/ueli/Documents/semio/🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇️catalog/📜️native-codec-factories.json", "utf8"));
const a = source.receipts.map((r: any) => [r.artifact_kind, r.schema, r.factory_id].join("|")).sort();
const b = buildTrustedStdioCatalogV1().nativeCodecs.map((r: any) => [r.artifactKind, r.artifactSchema, r.factoryId].join("|")).sort();
console.log("only source:", a.filter((x: string) => !b.includes(x)));
console.log("only catalog:", b.filter((x: string) => !a.includes(x)));
