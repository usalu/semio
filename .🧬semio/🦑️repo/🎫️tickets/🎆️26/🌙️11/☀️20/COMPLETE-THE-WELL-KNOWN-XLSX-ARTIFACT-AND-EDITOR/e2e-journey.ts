import {writeFileSync} from "node:fs";
import {join} from "node:path";
import {matrix, session} from "./e2e-harness.ts";
import {near, oracle, workbook} from "./e2e-workbook.ts";
import {budgetWorkbook} from "./e2e-fixture.ts";

const [locale = "en", serve = "http://127.0.0.1:6072/"] = process.argv.slice(2) as ["en" | "de", string];
const out = join(import.meta.dir, "🗑️generated/e2e/journey-" + locale);
const s = await session(locale, serve, out);
const {page, record, waitFor, shot} = s;
const de = locale === "de";
const num = (text: string) => de ? text.replace(".", ",") : text;
const {value, sheetLabels, openSheet, setCell, stagedAction, editDetails, partIndex} = workbook(s);

function judge(tag: string, file: string, expect: {sheets: string[]; summary: boolean}) {
  const o = oracle(file), d = o.sheets.Data?.cells ?? {};
  const c = (ref: string) => d[ref] ?? {value: undefined, type: "", cached: undefined, bold: false, numberFormat: "", fill: null};
  record(`${tag}:structure`, JSON.stringify(o.sheetNames) === JSON.stringify(expect.sheets), `sheets=${JSON.stringify(o.sheetNames)}`);
  record(`${tag}:types`, near(c("B3").value, 400) && c("B3").type === "n" && c("C3").value === true && c("C3").type === "b" && c("A5").value === "Tax" && c("A5").type === "s" && near(c("D2").value, 1.5) && c("D2").type === "n" && c("A6").value === "007" && c("A6").type === "s", `B3=${JSON.stringify(c("B3"))} C3=${c("C3").value}/${c("C3").type} A5=${c("A5").value}/${c("A5").type} D2=${c("D2").value}/${c("D2").type} A6=${JSON.stringify(c("A6").value)}/${c("A6").type}`);
  record(`${tag}:formulas`, c("B4").value === "=SUM(B2:B3)" && c("B5").value === "=B4*0.1" && (!expect.summary || o.sheets.Summary?.cells.B1?.value === "=SUM(Data!B2:B3)"), `B4=${c("B4").value} B5=${c("B5").value} Summary!B1=${o.sheets.Summary?.cells.B1?.value}`);
  record(`${tag}:recalculated-third-party`, near(o.recalculated["DATA!B4"], 1600.5) && near(o.recalculated["DATA!B5"], 160.05), `formulas lib: B4=${o.recalculated["DATA!B4"]} B5=${o.recalculated["DATA!B5"]} Summary!B1=${o.recalculated["SUMMARY!B1"]}`);
  record(`${tag}:cached-values`, near(c("B4").cached, 1600.5) && near(c("B5").cached, 160.05) && (!expect.summary || near(o.sheets.Summary?.cells.B1?.cached, 1600.5)), `stored caches: B4=${c("B4").cached} B5=${c("B5").cached} Summary!B1=${o.sheets.Summary?.cells.B1?.cached} calcPr=${o.calcPr}`);
  record(`${tag}:formatting`, c("A1").bold && c("A1").fill === "FFFFE699" && c("B3").numberFormat === "#,##0.00" && c("B4").bold && c("B4").numberFormat === "#,##0.00" && c("B2").bold && c("B2").numberFormat === "#,##0.00" && JSON.stringify(o.sheets.Data?.merged) === '["D1:E1"]' && o.sheets.Data?.widths.A === 18 && o.sheets.Data?.widths.B === 14, `A1 bold=${c("A1").bold} fill=${c("A1").fill}; B2 bold=${c("B2").bold} fmt=${c("B2").numberFormat}; B3 fmt=${c("B3").numberFormat}; B4 bold=${c("B4").bold}; merged=${JSON.stringify(o.sheets.Data?.merged)} widths=${JSON.stringify(o.sheets.Data?.widths)}`);
  record(`${tag}:other-sheets`, o.sheets.Memo?.cells.A1?.value === "Keep receipts" && (!expect.summary || o.sheets.Summary?.cells.A1?.value === "Overview"), `Memo!A1=${o.sheets.Memo?.cells.A1?.value} Summary!A1=${o.sheets.Summary?.cells.A1?.value}`);
}

async function checkGrid(tag: string) {
  await openSheet("Data");
  const got = {B3: await value("Data", "B3"), C3: await value("Data", "C3"), A5: await value("Data", "A5"), B5: await value("Data", "B5"), D2: await value("Data", "D2"), B4: await value("Data", "B4")};
  const labels = await sheetLabels();
  record(`${tag}:grid`, got.B3 === "400" && got.C3 === "true" && got.A5 === "Tax" && got.B5 === "=B4*0.1" && got.D2 === num("1.5") && got.B4 === "=SUM(B2:B3)", JSON.stringify(got));
  record(`${tag}:sheets`, JSON.stringify(labels) === JSON.stringify(["Data", "Memo", "Summary"]), JSON.stringify(labels));
}

try {
  await s.boot();
  const opened = await s.openViaPicker(budgetWorkbook(), "Budget.xlsx");
  const owner = await page.evaluate(() => (window as unknown as {__semioOsCatalogProbe?: {spawned: {appId: string}[]}}).__semioOsCatalogProbe?.spawned.map(x => x.appId) ?? []);
  record("open", opened.opened.length > 0 && owner.includes("s.stdio.xlsx@ecma-376/transitional#editor"), `windows=${opened.opened.length} owners=${JSON.stringify(owner)} notes=${JSON.stringify(opened.notes.slice(0, 1))}`);
  await waitFor(60_000, async () => (await value("Data", "A1")) !== null);
  record("render:values", (await value("Data", "A1")) === "Item" && (await value("Data", "B2")) === num("1200.5") && (await value("Data", "C2")) === "true" && (await value("Data", "B4")) === "=SUM(B2:B3)", `A1=${await value("Data", "A1")} B2=${await value("Data", "B2")} C2=${await value("Data", "C2")} B4=${await value("Data", "B4")}`);
  const initialSheets = await sheetLabels();
  const notesOpen = await openSheet("Notes");
  record("render:sheets", JSON.stringify(initialSheets) === '["Data","Notes"]' && notesOpen && (await value("Notes", "A1")) === "Keep receipts", `labels=${JSON.stringify(initialSheets)} Notes opened=${notesOpen} Notes!A1=${await value("Notes", "A1")}`);
  await shot("01-opened");

  await setCell("Data", "B3", num("400"), "400");
  await setCell("Data", "C3", "true");
  await setCell("Data", "A5", "Tax");
  await setCell("Data", "B5", "=B4*0.1");
  await setCell("Data", "D2", num("1.5"));
  await setCell("Data", "A6", "'007");
  await setCell("Data", "E5", "temporary");
  await s.clickAction("undo");
  const undone = await waitFor(20_000, async () => (await value("Data", "E5")) === "");
  await s.clickAction("redo");
  const redone = await waitFor(20_000, async () => (await value("Data", "E5")) === "temporary");
  record("undo-redo", !!undone && !!redone, `after undo E5 empty=${!!undone}, after redo E5=${JSON.stringify(await value("Data", "E5"))}`);
  await setCell("Data", "E5", "", "");
  await shot("02-edited");

  const wb = await partIndex("xl/workbook.xml"), ws = await partIndex("xl/worksheets/sheet1.xml");
  record("details:parts", wb >= 0 && ws >= 0, `workbook=xmlParts/${wb} sheet1=xmlParts/${ws}`);
  const styleVia = await editDetails(`/xmlParts/${ws}/document/root/children/2/children/1/children/1/attrs/1/value`, "3");
  record("format:B2-bold", true, `style s=2→3 on B2 via ${styleVia}`);
  const renameVia = await editDetails(`/xmlParts/${wb}/document/root/children/0/children/1/attrs/0/value`, "Memo");
  await waitFor(20_000, async () => (await sheetLabels()).includes("Memo"));
  record("sheet:rename", JSON.stringify(await sheetLabels()) === '["Data","Memo"]', `via ${renameVia}: ${JSON.stringify(await sheetLabels())}`);
  const sheetJson = (name: string) => JSON.stringify({kind: "element", name: "sheet", attrs: [{name: "name", value: name}], children: []});
  const insertA = await stagedAction("insertSnapshotValue", {path: `/xmlParts/${wb}/document/root/children/0/children/-`, value: sheetJson("Scratch"), valueEncoding: "json"});
  await waitFor(30_000, async () => (await sheetLabels()).includes("Scratch"));
  record("sheet:insert", JSON.stringify(await sheetLabels()) === '["Data","Memo","Scratch"]', `${insertA} → ${JSON.stringify(await sheetLabels())}`);
  const removeVia = await stagedAction("removeSnapshotValue", {path: `/xmlParts/${wb}/document/root/children/0/children/2`});
  await waitFor(30_000, async () => !(await sheetLabels()).includes("Scratch"));
  record("sheet:remove", JSON.stringify(await sheetLabels()) === '["Data","Memo"]', `${removeVia} → ${JSON.stringify(await sheetLabels())}`);
  const insertB = await stagedAction("insertSnapshotValue", {path: `/xmlParts/${wb}/document/root/children/0/children/-`, value: sheetJson("Summary"), valueEncoding: "json"});
  await waitFor(30_000, async () => (await sheetLabels()).includes("Summary"));
  record("sheet:insert-summary", (await sheetLabels()).includes("Summary") && await openSheet("Summary"), `${insertB} → ${JSON.stringify(await sheetLabels())}`);
  await setCell("Summary", "A1", "Overview");
  await setCell("Summary", "B1", "=SUM(Data!B2:B3)");
  await shot("03-sheets");

  const saved = await s.download(() => s.clickAction("saveArtifactFile"), join(out, "saved.xlsx"));
  record("save", saved.ok && saved.name === "Budget.xlsx" && saved.notes.some(n => n.startsWith("shell.documentTransfer.file-saved|")), `name=${saved.ok ? saved.name : "none"} notes=${JSON.stringify(saved.notes.slice(0, 2))}`);
  if (saved.ok) judge("saved", saved.path, {sheets: ["Data", "Memo", "Summary"], summary: true});

  const kept = await page.evaluate(() => document.querySelector('[data-semio-local-document-id][aria-current="true"]')?.getAttribute("data-semio-local-document-id") ?? null);
  await s.closeWindows(opened.opened);
  for (const phase of ["reopen", "reload-reopen"] as const) {
    if (phase === "reload-reopen") { await page.goto(serve); await matrix.awaitBeacon(page, Date.now() + 300_000); await matrix.dismissIntroduction(page); }
    const button = page.locator(`[data-semio-local-document-id=${JSON.stringify(kept)}]`).first();
    await button.waitFor({state: "visible", timeout: 120_000}).catch(() => undefined);
    const before = await s.windowIds(), seen = (await s.notices()).length;
    await button.click({timeout: 15_000}).catch(() => undefined);
    const again = await waitFor(120_000, async () => { const w = (await s.windowIds()).filter(id => !before.includes(id)); return w.length > 0 && (await s.notices()).slice(seen).some(n => n.startsWith("shell.documentTransfer.file-opened|")) ? w : null; });
    record(`${phase}:open`, !!again, `kept=${kept} windows=${again?.length ?? 0}`);
    await waitFor(60_000, async () => (await value("Data", "B3")) !== null);
    await checkGrid(phase);
    await shot(`04-${phase}`);
    if (phase === "reload-reopen") {
      const resaved = await s.download(() => s.clickAction("saveArtifactFile"), join(out, "resaved.xlsx"));
      record("reload-reopen:save", resaved.ok, resaved.ok ? resaved.name : "no download");
      if (resaved.ok) judge("resaved", resaved.path, {sheets: ["Data", "Memo", "Summary"], summary: true});
      const archive = await s.download(() => s.clickAction("exportArtifactDocument"), join(out, "document-archive.bin"));
      record("export-document", archive.ok && archive.notes.every(n => !n.includes("export-failed")), `${archive.ok ? archive.name : "no download"} notes=${JSON.stringify(archive.notes.slice(0, 2))}`);
      if (again) await s.closeWindows(again);
      const exportClosed = await s.download(() => s.clickAction("exportArtifactDocument"), join(out, "document-archive-after-close.bin"));
      record("export-after-close", !exportClosed.notes.some(n => n.includes("export-failed")), `download=${exportClosed.ok} notes=${JSON.stringify(exportClosed.notes.slice(0, 2))}`);
    } else if (again) await s.closeWindows(again);
  }
  record("faults", s.faults.length === 0, JSON.stringify([...new Set(s.faults)].slice(0, 5)));
} catch (error) {
  record("fatal", false, String(error).split("\n")[0]!.slice(0, 300));
  await shot("fatal");
} finally {
  writeFileSync(join(out, "verdicts.json"), JSON.stringify({locale, serve, verdicts: s.verdicts}, null, 1));
  writeFileSync(join(out, "table.md"), ["| step | verdict | detail |", "| --- | --- | --- |", ...s.verdicts.map(v => `| ${v.step} | ${v.status} | ${v.detail.replace(/\|/g, "/")} |`)].join("\n") + "\n");
  await s.browser.close();
  process.exitCode = s.verdicts.every(v => v.status === "pass") ? 0 : 1;
}
