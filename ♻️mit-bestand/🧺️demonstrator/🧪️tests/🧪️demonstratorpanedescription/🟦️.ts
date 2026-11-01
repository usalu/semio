type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(
  vitest: NonNullable<ImportMeta["vitest"]>,
  dependencies: {
    readonly demonstratorPaneDescriptionParagraphs: (description: string) => readonly string[];
    readonly demonstratorPaneViews: (locale: "en" | "de") => readonly { readonly description: string; readonly windowTitle: string; readonly shortWindowTitle: string }[];
    readonly DEMONSTRATOR_LOCALES: readonly ("en" | "de")[];
  },
  _source: TestSource,
): Promise<void> {
  const { demonstratorPaneDescriptionParagraphs, demonstratorPaneViews, DEMONSTRATOR_LOCALES } = dependencies;
  const { describe, expect, it } = vitest;

  describe("demonstratorPaneDescriptionParagraphs", () => {
    it("splits blank-line separated copy into trimmed paragraphs", () => {
      expect(demonstratorPaneDescriptionParagraphs("First.\n\nSecond.")).toEqual(["First.", "Second."]);
      expect(demonstratorPaneDescriptionParagraphs("Only one.")).toEqual(["Only one."]);
    });

    it("gives every pane two overview paragraphs in every locale", () => {
      for (const locale of DEMONSTRATOR_LOCALES) for (const pane of demonstratorPaneViews(locale)) expect(demonstratorPaneDescriptionParagraphs(pane.description).length, `${locale} ${pane.windowTitle}`).toBe(2);
    });

    it("titles every pane window from the catalog in every locale", () => {
      for (const locale of DEMONSTRATOR_LOCALES) {
        const panes = demonstratorPaneViews(locale);
        expect(new Set(panes.map((pane) => pane.windowTitle)).size, locale).toBe(panes.length);
        expect(new Set(panes.map((pane) => pane.shortWindowTitle)).size, locale).toBe(panes.length);
        for (const pane of panes) expect(pane.windowTitle.startsWith("Entwerfen mit Bestand · "), pane.windowTitle).toBe(true);
      }
      expect(demonstratorPaneViews("en").map((pane) => pane.windowTitle)).not.toEqual(demonstratorPaneViews("de").map((pane) => pane.windowTitle));
    });
  });
}
