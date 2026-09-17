type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: { readonly demonstratorPaneDescriptionParagraphs: (description: string) => readonly string[]; readonly DEMONSTRATOR_PANES: readonly { readonly description: string }[] }, _source: TestSource): Promise<void> {
  const { demonstratorPaneDescriptionParagraphs, DEMONSTRATOR_PANES } = dependencies;
  const { describe, expect, it } = vitest;

  describe("demonstratorPaneDescriptionParagraphs", () => {
    it("splits blank-line separated copy into trimmed paragraphs", () => {
      expect(demonstratorPaneDescriptionParagraphs("First.\n\nSecond.")).toEqual(["First.", "Second."]);
      expect(demonstratorPaneDescriptionParagraphs("Only one.")).toEqual(["Only one."]);
    });

    it("gives every pane two overview paragraphs", () => {
      for (const pane of DEMONSTRATOR_PANES) {
        expect(demonstratorPaneDescriptionParagraphs(pane.description).length).toBe(2);
      }
    });
  });
}
