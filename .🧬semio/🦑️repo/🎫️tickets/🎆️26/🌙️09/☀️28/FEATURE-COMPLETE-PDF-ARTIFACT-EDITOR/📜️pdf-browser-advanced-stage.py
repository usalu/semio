"""🧭️ Stages mounted cases for mixed resources and exact font reassignment."""

from pathlib import Path

ticket = Path(__file__).resolve().parent
source = (ticket / "📜️pdf-browser-smoke.ts").read_text()

def replace(old, new):
    global source
    assert source.count(old) == 1, old
    source = source.replace(old, new)

replace('mode === "cancel-large",', 'mode === "cancel-large" || mode === "large-mixed" || mode === "font-reassignment",')
replace('Expected optional large-vector, large-text, large-images, edit-content or cancel-large case', 'Expected a PDF smoke case')
replace('const controlledImages: ControlledImages | undefined = mode === "large-images"', 'const controlledImages: ControlledImages | undefined = mode === "large-images" || mode === "large-mixed"')
replace('const large: {readonly byteLength:', '''const mixedLines: string[] | undefined = mode === "large-mixed"
  ? JSON.parse(readFileSync(new URL("../🧫️mixed-resources.json", import.meta.url), "utf8")).lines
  : undefined;
const fontEdit: {sourceFont: string; targetFont: string; firstLine: string; secondLine: string} | undefined = mode === "font-reassignment"
  ? JSON.parse(readFileSync(new URL("./font-reassignment.json", import.meta.url), "utf8"))
  : undefined;
const large: {readonly byteLength:''')
replace('{byteLength: 38_096, pageCount:', '{byteLength: mixedLines ? 38_459 : 38_096, pageCount:')
replace('const reportName = cancelLarge', 'const reportName = fontEdit ? "pdf-mounted-font-reassignment" : mixedLines ? "pdf-mounted-mixed" : cancelLarge')
replace('"./🧪️mounted-cancel.json"', '"../🧪️mounted-cancel.json"')
replace('"./🔣️mounted-content-edit.json"', '"../🔣️mounted-content-edit.json"')
replace('"./🎞️fixture.json"', '"../🎞️fixture.json"')
replace('await openFile(page, open, source, large?.pageCount ?? 2);', 'await openFile(page, open, source, large?.pageCount ?? (fontEdit ? 1 : 2));')
replace('    if (contentEdit) {\n      await verifyOrEditContent', '''    if (fontEdit) {
      const first = "Text: " + fontEdit.firstLine;
      const second = "Text: " + fontEdit.secondLine;
      await selectInspector(owned, first, "Text");
      await commitInspectorField(owned, "pdf-object-font", fontEdit.sourceFont, fontEdit.targetFont);
      await selectInspector(owned, second, "Text");
      await inspectorField(owned, "pdf-object-font", fontEdit.sourceFont);
      const reopened = await saveAndReopen(page, 1);
      await selectInspector(reopened, first, "Text");
      await inspectorField(reopened, "pdf-object-font", fontEdit.targetFont);
      await selectInspector(reopened, second, "Text");
      await inspectorField(reopened, "pdf-object-font", fontEdit.sourceFont);
      await page.screenshot({path: join(output, `${reportName}.png`)});
      console.log("[DEBUG] Mounted PDF font change retained both lines after Save and reopen");
    } else if (contentEdit) {
      await verifyOrEditContent''')
replace('async function verifyControlledImages(owned: Locator, spec: ControlledImages) {',
        'async function verifyControlledImages(owned: Locator, spec: ControlledImages, lines?: readonly string[]) {')
replace('    const name = spec.images[page.image].name;', '''    const line = lines?.[pageIndex];
    if (line) await owned.locator(`[data-ui-node-key^="pdf-text-layer-p${pageIndex}-"]`)
      .and(owned.getByRole("button", {name: "Text: " + line, exact: true})).waitFor({timeout: 60_000});
    const name = spec.images[page.image].name;''')
replace('await verifyControlledImages(owned, controlledImages);','await verifyControlledImages(owned, controlledImages, mixedLines);')
replace('await verifyControlledImages(reopened, controlledImages);','await verifyControlledImages(reopened, controlledImages, mixedLines);')
(ticket / "🗑️generated/pdf-browser-advanced.ts").write_text(source)
