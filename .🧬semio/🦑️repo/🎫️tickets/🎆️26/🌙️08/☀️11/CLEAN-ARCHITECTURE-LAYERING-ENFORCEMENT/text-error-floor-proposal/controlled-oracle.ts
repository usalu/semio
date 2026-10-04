import assert from "node:assert/strict";
import Ajv from "ajv/dist/2020.js";
import ts from "typescript";
import { BundleScript } from "../../../🏃️process/🧭️routing/🟦️.ts";
import valueSchema from "../../../🌱️value/⚠️refusal/🧬️schema/🔣️.json" with { type: "json" };
import schema from "../../🧬️schema/🎛️controlled/🔣️.json" with { type: "json" };
import fixture from "../../🧫️fixtures/🎛️controlled/🔣️.json" with { type: "json" };

/** 🎛️ Validates the closed neutral Diagnostic cases with an independent schema engine. */
export class TestScript extends BundleScript {
  async run(): Promise<void> {
    const program = ts.createProgram([import.meta.filename], { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, strict: true, noUncheckedIndexedAccess: true, resolveJsonModule: true, allowImportingTsExtensions: true, esModuleInterop: true, skipLibCheck: true, noEmit: true, types: ["bun-types"] });
    assert.deepEqual(ts.getPreEmitDiagnostics(program).map(item => ts.flattenDiagnosticMessageText(item.messageText, "\n")), []);
    const ajv = new Ajv({ strict: true }).addSchema(valueSchema);
    assert(ajv.validate(schema, fixture), JSON.stringify(ajv.errors));
    for (const row of fixture.cases) {
      const validate = ajv.getSchema(`${schema.$id}#/$defs/${row.owner}`);
      assert(validate);
      assert.equal(validate(row.input), row.accepted, row.id);
      if ("expected" in row) assert(validate(row.expected), `${row.id} output`);
    }
    const uniqueKeys = ajv.compile({ type: "array", items: { type: "string" }, uniqueItems: true });
    for (const row of fixture.duplicateCases) assert.equal(uniqueKeys(row.entries.map(entry => entry[0])), false, row.id);
    console.log(`[DEBUG] Diagnostic controlled strict TypeScript and independent Ajv oracle: ${fixture.cases.length} cases`);
  }
}
