#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
/** 🗜️ shooting TypeScript package */
import { join } from "node:path";
import { runCmd } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
class TestScript extends BundleScript {
  run(): void {
    const subset = join(this.repoRoot, "✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any");
    runCmd(process.execPath, [
      "test",
      join(this.repoRoot, "✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🎥️shooting-route/🔬️unit/🟦️.ts"),
      join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖼️IconRenderHost/🧪️export/🟦️.ts"),
      join(this.repoRoot, "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🖼️icon-transform/🟦️.ts"),
      join(subset, "🧬️schema/🧪️tests/🛂️parsing/🟦️.ts"),
      join(subset, "🧬️schema/🧪️tests/☀️sun/🟦️.ts"),
      join(subset, "✏️editor/🧪️tests/📷️shot-camera/🟦️.ts"),
      join(subset, "📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts"),
      join(subset, "📚️examples/🌲️hexagonal-cut-concrete-forest-left/🧪️tests/🧩️example/🟦️.ts"),
      join(subset, "✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts"),
    ]);
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
