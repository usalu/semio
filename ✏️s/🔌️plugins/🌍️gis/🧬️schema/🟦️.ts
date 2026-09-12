import { join } from "node:path";
import { readFileSync } from "node:fs";

export type GisSchemaValidator = ((value: unknown) => boolean) & { errors?: unknown };

/** 🧬️ Compiles one PascalCase export while preserving every registered GIS schema annotation. */
export async function compileGisScopeExport(repoRoot: string, moduleRel: string, exportId: string | null, dependencyModules: readonly string[] = []): Promise<GisSchemaValidator> {
  const module = JSON.parse(readFileSync(join(repoRoot, moduleRel), "utf8"));
  const Ajv = (await import("ajv")).default;
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
  ajv.addKeyword({ keyword: "x-semio-child-kind", metaSchema: { type: "string", minLength: 1 } });
  ajv.addKeyword("x-semio-state").addFormat("double", true);
  for (const dependency of dependencyModules) ajv.addSchema(JSON.parse(readFileSync(join(repoRoot, dependency), "utf8")));
  ajv.addSchema(module);
  return ajv.compile({ $ref: exportId === null ? module.$id : `${module.$id}#/$defs/${exportId}` }) as GisSchemaValidator;
}
