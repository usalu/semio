type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { demonstratorRuntimeBuildVariants, join } = dependencies;

  const { describe, expect, it } = vitest;

  //#region 🧪️DemonstratorPluginBuildTests
  describe("demonstratorRuntimeBuildVariants", () => {
    it("builds one additional artifact for six pane runtime variants", () => {
      expect(demonstratorRuntimeBuildVariants("generator")).toEqual(["generation3d"]);
    });

    it("validates the authored runtime catalog against its owner schema module", async () => {
      const { readFileSync } = await import("node:fs");
      const { createRequire } = await import("node:module");
      const { dirname } = await import("node:path");
      const { fileURLToPath } = await import("node:url");
      const modulePath = join(dirname(fileURLToPath(source.url)), "🔨️modules/🧩️runtime");
      const schema = JSON.parse(readFileSync(join(modulePath, "🧬️schema/🔣️.json"), "utf8"));
      expect(schema.$schema).toBe("http://json-schema.org/draft-07/schema#");
      expect(schema.$id).toBe("https://semio.tech/schema/mit-bestand/demonstrator/runtime/schema.json");
      const ajv = new (createRequire(source.url)("ajv").default)({ strict: false });
      const validate = ajv.compile(schema);
      expect(validate(JSON.parse(readFileSync(join(modulePath, "🔣️.json"), "utf8")))).toBe(true);
      expect(validate({ schemaVersion: 1, host: "", assetsDirectory: "a", panes: [] })).toBe(false);
      const validatePipeline = ajv.compile({ ...schema, $id: `${schema.$id}#pipeline`, $ref: "#/$defs/DemonstratorPipelineContract" });
      expect(validatePipeline(JSON.parse(readFileSync(join(modulePath, "🧫️pipeline.json"), "utf8")))).toBe(true);
      expect(validatePipeline({ schemaVersion: 2 })).toBe(false);
    });
  });
  //#endregion 🧪️DemonstratorPluginBuildTests

}
