/** 🧱️ The REPL aside is one vertical shared-border row: the compositor paints its edges, every section is a cell, no native border, gap-separated box or `border-t` remains. */
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import type { RendererTestDependencies } from "../../🎒️test-dependencies/🟦️.ts";

type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerTests3(vitest: Vitest, _dependencies: RendererTestDependencies, source: { readonly url: string }): Promise<void> {
  const { describe, it, expect } = vitest;
  const repl = readFileSync(fileURLToPath(new URL("./🎛️repl/🟦️.tsx", source.url)), "utf8");
  const panesSource = readFileSync(fileURLToPath(new URL("./📋️repl-panes/🟦️.tsx", source.url)), "utf8");
  const panes = panesSource.slice(0, panesSource.indexOf("export interface SelectionPropertiesPaneProps"));
  const aside = repl.slice(repl.indexOf("<SharedBorderRow as=\"aside\""), repl.indexOf(">", repl.indexOf("<SharedBorderRow as=\"aside\"")) + 1);

  describe("REPL aside shared border", () => {
    it("renders the aside as a vertical SharedBorderRow owner", () => {
      expect(aside).toContain('orientation="vertical"');
      expect(repl).not.toContain("<aside");
      expect(repl).not.toContain("useSharedBorderFrameRef");
    });
    it("never overrides single sides of the aside frame", () => {
      expect(aside).not.toMatch(/border-[lrtb]-0|border-[lrtb]\b/);
    });
    it("uses no bordered tags, field surfaces, gap stacks or border-t separators", () => {
      for (const name of ["floatingTagClass", "floatingFieldSurfaceClass", "floatingMenuSurfaceClass", "border-t", "gap-half", "gap-single text-xs", "border-0"]) expect(repl).not.toContain(name);
      expect(repl).toContain('kind="multiple"');
      expect(repl).toContain("wrap");
    });
    it("makes the attribute pane a vertical row without gap stacks", () => {
      expect(panes).not.toMatch(/flex flex-col gap-/);
      expect(panes).toContain('orientation="vertical"');
    });
  });
}
