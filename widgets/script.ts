#!/usr/bin/env bun
/** 🧭 `@widgets/react` task router: `bun ./script.ts <test|typecheck>`. */
import { strict as assert } from "node:assert";
import { BundleScript, ScriptRouter, runBundleScriptMain, runBunx } from "../repo/lib/js/src/index.ts";
import { graphWidgetDataFromSemioLanguageGraph } from "./index.tsx";
import { semioLanguageGraphFixture } from "./fixtures/index.ts";

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    const data = graphWidgetDataFromSemioLanguageGraph(semioLanguageGraphFixture);
    assert.equal(data.nodes.length, 5);
    assert.equal(data.edges.length, 5);
    assert.deepEqual(
      data.edges.map((edge) => `${edge.source}->${edge.target}`),
      ["brief->rules", "rules->parts", "brief->layout", "layout->eval", "parts->eval"],
    );
    assert.throws(() =>
      graphWidgetDataFromSemioLanguageGraph({
        kind: "semio.graph",
        statements: [
          { kind: "node", id: "kit", label: "Kit", at: [24, 32] },
          { kind: "edge", source: "kit", target: "ghost" },
        ],
      }),
    );
    console.log("[widgets] graph fixture smoke test passed.");
  }
}

class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["tsc", "--noEmit", "-p", "tsconfig.json", ...segments], this.root);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("typecheck", TypecheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "typecheck" });
