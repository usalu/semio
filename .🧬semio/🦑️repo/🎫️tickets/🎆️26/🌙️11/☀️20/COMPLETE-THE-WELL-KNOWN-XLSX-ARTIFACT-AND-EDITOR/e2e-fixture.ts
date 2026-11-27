import {createRequire} from "node:module";
import {join} from "node:path";

const root = "/Users/ueli/Documents/semio";
const {zipSync, strToU8} = createRequire(join(root, "🌎️hub/package.json"))("fflate") as typeof import("fflate");

const main = "http://schemas.openxmlformats.org/spreadsheetml/2006/main";
const rel = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const pkg = "http://schemas.openxmlformats.org/package/2006";
const ct = "application/vnd.openxmlformats-officedocument.spreadsheetml";
const head = '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>\n';

/** 📒 A formatted two-sheet Transitional budget workbook: bold filled headers (style 1), a `#,##0.00` amount format (style 2), a bold total with that format (style 3), booleans, shared and inline strings, a cached SUM formula, column widths and a merged range. */
export function budgetWorkbook(): Uint8Array {
  const strings = ["Item", "Amount", "Paid", "Rent", "Food", "Total", "Budget 2026"];
  const parts: Record<string, string> = {
    "[Content_Types].xml": `<Types xmlns="${pkg}/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/xl/workbook.xml" ContentType="${ct}.sheet.main+xml"/><Override PartName="/xl/worksheets/sheet1.xml" ContentType="${ct}.worksheet+xml"/><Override PartName="/xl/worksheets/sheet2.xml" ContentType="${ct}.worksheet+xml"/><Override PartName="/xl/styles.xml" ContentType="${ct}.styles+xml"/><Override PartName="/xl/sharedStrings.xml" ContentType="${ct}.sharedStrings+xml"/></Types>`,
    "_rels/.rels": `<Relationships xmlns="${pkg}/relationships"><Relationship Id="rId1" Type="${rel}/officeDocument" Target="xl/workbook.xml"/></Relationships>`,
    "xl/workbook.xml": `<workbook xmlns="${main}" xmlns:r="${rel}"><sheets><sheet name="Data" sheetId="1" r:id="rId1"/><sheet name="Notes" sheetId="2" r:id="rId2"/></sheets><calcPr calcId="191029"/></workbook>`,
    "xl/_rels/workbook.xml.rels": `<Relationships xmlns="${pkg}/relationships"><Relationship Id="rId1" Type="${rel}/worksheet" Target="worksheets/sheet1.xml"/><Relationship Id="rId2" Type="${rel}/worksheet" Target="worksheets/sheet2.xml"/><Relationship Id="rId3" Type="${rel}/styles" Target="styles.xml"/><Relationship Id="rId4" Type="${rel}/sharedStrings" Target="sharedStrings.xml"/></Relationships>`,
    "xl/styles.xml": `<styleSheet xmlns="${main}"><numFmts count="1"><numFmt numFmtId="164" formatCode="#,##0.00"/></numFmts><fonts count="2"><font><sz val="11"/><name val="Calibri"/></font><font><b/><sz val="11"/><name val="Calibri"/></font></fonts><fills count="3"><fill><patternFill patternType="none"/></fill><fill><patternFill patternType="gray125"/></fill><fill><patternFill patternType="solid"><fgColor rgb="FFFFE699"/><bgColor indexed="64"/></patternFill></fill></fills><borders count="1"><border><left/><right/><top/><bottom/><diagonal/></border></borders><cellStyleXfs count="1"><xf numFmtId="0" fontId="0" fillId="0" borderId="0"/></cellStyleXfs><cellXfs count="4"><xf numFmtId="0" fontId="0" fillId="0" borderId="0" xfId="0"/><xf numFmtId="0" fontId="1" fillId="2" borderId="0" xfId="0" applyFont="1" applyFill="1"/><xf numFmtId="164" fontId="0" fillId="0" borderId="0" xfId="0" applyNumberFormat="1"/><xf numFmtId="164" fontId="1" fillId="0" borderId="0" xfId="0" applyFont="1" applyNumberFormat="1"/></cellXfs><cellStyles count="1"><cellStyle name="Normal" xfId="0" builtinId="0"/></cellStyles></styleSheet>`,
    "xl/sharedStrings.xml": `<sst xmlns="${main}" count="${strings.length}" uniqueCount="${strings.length}">${strings.map(s => `<si><t>${s}</t></si>`).join("")}</sst>`,
    "xl/worksheets/sheet1.xml": `<worksheet xmlns="${main}"><dimension ref="A1:E4"/><cols><col min="1" max="1" width="18" customWidth="1"/><col min="2" max="2" width="14" customWidth="1"/></cols><sheetData><row r="1"><c r="A1" s="1" t="s"><v>0</v></c><c r="B1" s="1" t="s"><v>1</v></c><c r="C1" s="1" t="s"><v>2</v></c><c r="D1" t="s"><v>6</v></c></row><row r="2"><c r="A2" t="s"><v>3</v></c><c r="B2" s="2"><v>1200.5</v></c><c r="C2" t="b"><v>1</v></c></row><row r="3"><c r="A3" t="s"><v>4</v></c><c r="B3" s="2"><v>350.25</v></c><c r="C3" t="b"><v>0</v></c></row><row r="4"><c r="A4" s="1" t="s"><v>5</v></c><c r="B4" s="3"><f>SUM(B2:B3)</f><v>1550.75</v></c></row></sheetData><mergeCells count="1"><mergeCell ref="D1:E1"/></mergeCells></worksheet>`,
    "xl/worksheets/sheet2.xml": `<worksheet xmlns="${main}"><sheetData><row r="1"><c r="A1" t="inlineStr"><is><t>Keep receipts</t></is></c></row></sheetData></worksheet>`,
  };
  return zipSync(Object.fromEntries(Object.entries(parts).map(([k, v]) => [k, strToU8(head + v)])), {level: 6, mtime: new Date(1980, 0, 1)});
}

if (import.meta.main) await Bun.write(process.argv[2] ?? join(import.meta.dir, "🗑️generated/e2e/Budget.xlsx"), budgetWorkbook());
