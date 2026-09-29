
describe("🗂️ local catalog vocabulary", () => {
  const hex = (bytes: Uint8Array): string => Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
  const fromHex = (text: string): Uint8Array => Uint8Array.from(text.match(/../gu) ?? [], (pair) => Number.parseInt(pair, 16));

  it("speaks every notice in both tongues under the code the wgpu shell's lane uses", () => {
    for (const [key, copy] of Object.entries(localCatalogVocabulary.notices)) {
      const notice = key as LocalCatalogNoticeV1;
      const [en, de] = [copy.en.replace("{name}", "Plan"), copy.de.replace("{name}", "Plan")];
      expect([localCatalogNoticeTextV1(notice, "en", "Plan"), localCatalogNoticeTextV1(notice, "fr", "Plan"), localCatalogNoticeTextV1(notice, "de", "Plan"), localCatalogNoticeCodeV1(notice)]).toEqual([en, en, de, `shell.localCatalog.${key}`]);
      expect(en !== de).toBe(true);
    }
  });

  it("answers the shared admission vectors the wgpu shell's law answers", () => {
    expect(localCatalogVocabulary.admissions.length).toBeGreaterThan(0);
    for (const vector of localCatalogVocabulary.admissions) {
      const admission = localCatalogAdmissionV1((vector.args ?? undefined) as Readonly<Record<string, unknown>> | undefined, vector.dataDir ?? undefined, vector.nowMs);
      const actual = "refusal" in admission ? { refusal: admission.refusal } : { document: admission.document, folder: admission.binding.path, archiveHex: hex(admission.archive) };
      expect({ name: vector.name, ...actual }).toEqual({ name: vector.name, ...vector.expect });
    }
  });

  it("persists the catalog as the shared archive bytes, each admittedAtMs an exact uint, and reads them back", () => {
    expect(localCatalogVocabulary.catalogArchives.length).toBeGreaterThan(0);
    for (const vector of localCatalogVocabulary.catalogArchives) {
      const catalog = vector.catalog as Parameters<typeof localCatalogArchiveV1>[0];
      expect(hex(localCatalogArchiveV1(catalog))).toBe(vector.archiveHex);
      expect(decodeLocalCatalogArchiveV1(fromHex(vector.archiveHex))).toEqual(catalog);
    }
  });
});
