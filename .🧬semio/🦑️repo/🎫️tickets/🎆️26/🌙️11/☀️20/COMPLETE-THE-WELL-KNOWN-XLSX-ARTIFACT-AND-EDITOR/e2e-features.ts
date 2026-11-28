import {readFileSync, writeFileSync} from "node:fs";
import {join} from "node:path";
import {root, session} from "./e2e-harness.ts";
import {near, oracle, workbook} from "./e2e-workbook.ts";
import {foreignZip, formulaWorkbook} from "./e2e-fixture.ts";

const [locale = "en", serve = "http://127.0.0.1:6072/"] = process.argv.slice(2) as ["en" | "de", string];
const out = join(import.meta.dir, "🗑️generated/e2e/features-" + locale);
const s = await session(locale, serve, out);
const {page, record, waitFor, shot} = s;
const {value, sheetLabels, openSheet, setCell, editDetails, partIndex, save} = workbook(s);
const num = (text: string) => locale === "de" ? text.replace(".", ",") : text;
const strict = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict/🧪️tests/🧫️fixtures/🔒️reuse-marketplaces-strict.xlsx");
const owners = () => page.evaluate(() => (window as unknown as {__semioOsCatalogProbe?: {spawned: {appId: string}[]}}).__semioOsCatalogProbe?.spawned.map(x => x.appId) ?? []);

const formulas: [string, string, unknown][] = [
  ["C1", "=AVERAGE(A1:A3)", 4], ["C2", '=IF(A1>1,"big","small")', "big"], ["C3", "=1/0", "#DIV/0!"], ["C4", "=FOO(1)", "#NAME?"],
  ["C5", "=ROUND(2/3,2)", 0.67], ["C6", '=CONCATENATE("a",A1)', "a4"], ["C7", "=MAX(A1:A3)-MIN(A1:A3)", 2], ["C8", "=COUNTA(A1:A3,E1)", 4],
  ["C9", '=IFERROR(1/0,"err")', "err"], ["C10", "=AND(TRUE,NOT(FALSE))", true], ["C11", '=LEN(UPPER("abc"))&LOWER("X")', "3x"], ["C12", "=ABS(-3)^2", 9],
  ["C13", "=PRODUCT(A1:A2)", 12], ["C14", "=COUNT(A1:A3)", 3], ["C15", "=OR(A1>5,A2>5)", false], ["C16", "=Other!A1*2", 42], ["C17", "=SUM(B1:B3)", 24],
];
const same = (a: unknown, b: unknown) => typeof b === "number" ? near(a, b) : typeof b === "boolean" ? a === b || String(a).toUpperCase() === String(b).toUpperCase() : String(a) === String(b);

try {
  await s.boot();
  const opened = await s.openViaPicker(formulaWorkbook(), "Formulas.xlsx");
  record("open:formulas", opened.opened.length > 0 && (await owners()).includes("s.stdio.xlsx@ecma-376/transitional#editor"), `windows=${opened.opened.length} notes=${JSON.stringify(opened.notes.slice(0, 1))}`);
  await waitFor(60_000, async () => (await value("Calc", "A1")) !== null);
  record("shared-formula:display", (await value("Calc", "B1")) === "=A1*2" && (await value("Calc", "B2")) === "=A2*2" && (await value("Calc", "B3")) === "=A3*2", `B1=${await value("Calc", "B1")} B2=${await value("Calc", "B2")} B3=${await value("Calc", "B3")}`);
  record("render:shared-string", (await value("Calc", "E1")) === "Label", `E1=${await value("Calc", "E1")}`);

  for (const [ref, text] of formulas) await setCell("Calc", ref, text);
  await setCell("Calc", "D1", "TRUE", "true");
  await setCell("Calc", "D2", "007", "7");
  await setCell("Calc", "D3", "'007");
  await setCell("Calc", "D4", num("0.1"), num("0.1"));
  await setCell("Calc", "A1", "4");
  await shot("01-formulas");

  await s.clickAction("undo");
  const undone = await waitFor(20_000, async () => (await value("Calc", "A1")) === "2");
  await s.clickAction("redo");
  const redone = await waitFor(20_000, async () => (await value("Calc", "A1")) === "4");
  record("undo-redo:A1", !!undone && !!redone, `undo→2 ${!!undone}, redo→4 ${!!redone}`);

  const sst = await partIndex("xl/sharedStrings.xml"), wb = await partIndex("xl/workbook.xml");
  record("details:parts", sst >= 0 && wb >= 0, `sst=${sst} workbook=${wb}`);
  const sstVia = await editDetails(`/xmlParts/${sst}/document/root/children/0/children/0/children/0/text`, "Renamed label");
  const relabeled = await waitFor(20_000, async () => (await value("Calc", "E1")) === "Renamed label");
  record("shared-string:edit", !!relabeled, `via ${sstVia}: E1=${await value("Calc", "E1")}`);
  const renameVia = await editDetails(`/xmlParts/${wb}/document/root/children/0/children/1/attrs/0/value`, "Other Data");
  await waitFor(20_000, async () => (await sheetLabels()).includes("Other Data"));
  record("sheet:rename", JSON.stringify(await sheetLabels()) === '["Calc","Other Data"]', `via ${renameVia}: ${JSON.stringify(await sheetLabels())}`);
  await openSheet("Calc");
  const c16 = await waitFor(20_000, async () => (await value("Calc", "C16")) === "='Other Data'!A1*2");
  record("sheet:rename-rewrites-references", !!c16, `C16=${await value("Calc", "C16")}`);
  await shot("02-renamed");

  const saved = await save("formulas.xlsx", out);
  if (saved.ok) {
    const o = oracle(saved.path), c = o.sheets.Calc?.cells ?? {};
    record("saved:structure", JSON.stringify(o.sheetNames) === '["Calc","Other Data"]', JSON.stringify(o.sheetNames));
    const wrong = formulas.filter(([ref, , want]) => !same(c[ref]?.cached, want)).map(([ref, , want]) => `${ref}: cached ${JSON.stringify(c[ref]?.cached)} want ${JSON.stringify(want)}`);
    record("saved:cached-values-expected", wrong.length === 0, wrong.join("; ") || `${formulas.length} formulas`);
    const formulaCells = Object.entries(o.sheets).flatMap(([sheet, v]) => Object.entries(v.cells).filter(([, x]) => x.type === "f").map(([ref, x]) => [sheet, ref, x] as const));
    const disagree = formulaCells.filter(([sheet, ref, x]) => { const r = o.recalculated[`${sheet.toUpperCase()}!${ref}`]; return r === undefined || !same(x.cached, typeof x.cached === "number" ? Number(r) : r); }).map(([sheet, ref, x]) => `${sheet}!${ref} cached=${JSON.stringify(x.cached)} formulas=${JSON.stringify(o.recalculated[`${sheet.toUpperCase()}!${ref}`])}`);
    record("saved:cached-equals-third-party-recalculation", formulaCells.length >= 20 && disagree.length === 0, disagree.join("; ") || `${formulaCells.length} formula cells agree`);
    record("saved:shared-formulas", c.B2?.value === "=A2*2" && near(c.B1?.cached, 8) && near(c.B2?.cached, 6) && near(c.B3?.cached, 10), `B1=${c.B1?.value}/${c.B1?.cached} B2=${c.B2?.value}/${c.B2?.cached} B3=${c.B3?.value}/${c.B3?.cached}`);
    record("saved:rename-reference", c.C16?.value === "='Other Data'!A1*2", `C16=${c.C16?.value}`);
    record("saved:types", c.D1?.value === true && c.D1?.type === "b" && near(c.D2?.value, 7) && c.D2?.type === "n" && c.D3?.value === "007" && c.D3?.type === "s" && near(c.D4?.value, 0.1) && c.E1?.value === "Renamed label", `D1=${c.D1?.value}/${c.D1?.type} D2=${c.D2?.value}/${c.D2?.type} D3=${JSON.stringify(c.D3?.value)}/${c.D3?.type} D4=${c.D4?.value} E1=${c.E1?.value}`);
  }
  await s.closeWindows(opened.opened);

  const strictOpen = await s.openViaPicker(new Uint8Array(readFileSync(strict)), "Strict.xlsx");
  await waitFor(60_000, async () => page.evaluate(() => [...document.querySelectorAll('[role="grid"] input[aria-label="A1"]')].some(e => (e as HTMLInputElement).value === "ID")));
  const strictA1 = await page.evaluate(() => [...document.querySelectorAll('[role="grid"] input[aria-label="A1"]')].map(e => (e as HTMLInputElement).value));
  record("open:strict", strictOpen.opened.length > 0 && (await owners()).includes("s.stdio.xlsx@ecma-376/strict#editor") && strictA1.includes("ID"), `windows=${strictOpen.opened.length} A1=${JSON.stringify(strictA1)} owners=${JSON.stringify(await owners())}`);
  await shot("03-strict");
  await s.closeWindows(strictOpen.opened);

  for (const [name, bytes] of [["Broken.xlsx", new Uint8Array([0xde, 0xad, 0xbe, 0xef])], ["Foreign.xlsx", foreignZip()]] as const) {
    const refused = await s.openViaPicker(bytes, name);
    const notice = refused.notes.find(n => n.startsWith("shell.openFile.") || n.startsWith("shell.documentTransfer.file-open-failed|"));
    record(`refuse:${name}`, refused.opened.length === 0 && !!notice, `notice=${notice ?? JSON.stringify(refused.notes.slice(0, 2))}`);
    await s.clearNotices();
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
