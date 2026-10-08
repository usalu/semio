import { buildBoxInteractionSpec } from "../../../🗿️artifact/🟦️.ts";
import { preciseSpatialKernelMath } from "../../../../../../../../../../../../../🔨️modules/🌐️spatial-kernel/⚙️engine/🧮️preview/🟦️.ts";
import type { ActionsTestDependencies } from "../../🟦️.ts";
import type { Vec3 } from "@semio-tech/framework-3d-js";

type TestSource = { readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: ActionsTestDependencies, source: TestSource): Promise<void> {
  const { __actionsTestRuntime, resolveDisplay } = dependencies;

  __actionsTestRuntime!.bootstrapCadModules();
  const M = preciseSpatialKernelMath;
  const { describe, expect, it } = vitest;

  describe("@semio-tech/cad-js/core box display committed", () => {
    it("keeps box-preview visible for committed state", () => {
      const spec = buildBoxInteractionSpec();
      const ctx: Record<string, unknown> = {
        origin: [0, 0, 0] as Vec3,
        corner: [2, 3, 0] as Vec3,
        height: 4,
      };
      const d = resolveDisplay(spec, "committed", ctx, M);
      const prev = d.items.find((i) => i.kind === "box-preview" && i.id === "preview-committed");
      expect(prev?.params?.cornerA).toEqual([0, 0, 0]);
      expect(prev?.params?.cornerB).toEqual([2, 3, 0]);
      expect(prev?.params?.height).toBe(4);
    });
  });

}
