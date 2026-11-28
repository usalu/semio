import {copyFileSync, writeFileSync} from "node:fs";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {matrix, root, session} from "./e2e-harness.ts";

const {detailsControlId} = await import(join(root, "✏️s/🔌️plugins/🗄️stdio/🧪️tests/🎯️artifact-acceptance/🏃️browser/🎯️details/🟦️.ts"));

export type OracleCell = {value: unknown; type: string; cached: unknown; bold: boolean; numberFormat: string; fill: string | null};
export type Oracle = {sheetNames: string[]; calcPr: string | null; recalculated: Record<string, unknown>; sheets: Record<string, {cells: Record<string, OracleCell>; merged: string[]; widths: Record<string, number>}>};

/** 🧾 Reads a saved workbook with openpyxl and recalculates it with the `formulas` library (third-party oracle, `e2e-oracle.py`). */
export function oracle(file: string): Oracle {
  const script = "/tmp/e2e-oracle.py";
  copyFileSync(join(import.meta.dir, "e2e-oracle.py"), script);
  const run = spawnSync("uv", ["run", "-q", "--no-project", "--with", "openpyxl", "--with", "formulas", "python", "-I", "-W", "ignore", script, file], {cwd: "/tmp", encoding: "utf8"});
  writeFileSync(file + ".oracle.json", run.stdout);
  return JSON.parse(run.stdout) as Oracle;
}

export const near = (a: unknown, b: number) => typeof a === "number" ? Math.abs(a - b) < 1e-9 : Number(a) === b;

/** 🧰 Grid, sheet tree, details pane and staged-action helpers of one XLSX editor session. */
export function workbook(s: Awaited<ReturnType<typeof session>>) {
  const {page, record, waitFor} = s;
  const details = '[id*="s.stdio.window.snapshot-details"]';
  const cell = (sheet: string, ref: string) => page.locator(`[role="grid"][aria-label="${sheet}"] input[aria-label="${ref}"]`).first();
  const value = (sheet: string, ref: string) => cell(sheet, ref).inputValue({timeout: 5_000}).catch(() => null);
  const sheetLabels = () => page.evaluate(() => [...document.querySelectorAll('[role="treeitem"][id*="xlsx-sheet-"]')].map(e => ((e as HTMLElement).innerText ?? "").trim().split("\n")[0]!.trim()));
  async function openSheet(name: string) {
    const grid = page.locator(`[role="grid"][aria-label="${name}"]`);
    if ((await grid.count()) > 0) return true;
    const item = page.locator(`[role="treeitem"][id*="xlsx-sheet-"]`, {hasText: name}).first();
    if ((await item.count()) === 0) return false;
    for (const open of await page.locator(`[role="treeitem"][id*="xlsx-sheet-"][aria-expanded="true"]`).all()) await open.dispatchEvent("click");
    await page.waitForTimeout(500);
    if ((await item.getAttribute("aria-expanded")) === "false") await item.dispatchEvent("click");
    return !!(await waitFor(30_000, async () => (await grid.count()) > 0));
  }
  async function setCell(sheet: string, ref: string, text: string, shown = text) {
    const input = cell(sheet, ref), seen = (await s.notices()).length;
    await openSheet(sheet);
    await input.waitFor({state: "visible", timeout: 30_000}).catch(() => undefined);
    if ((await input.count()) === 0) return record(`edit:${sheet}!${ref}`, false, "cell input absent");
    await input.fill(text); await input.press("Enter"); await input.blur().catch(() => undefined);
    const ok = await waitFor(30_000, async () => page.evaluate(({sheet, ref, shown}) => { const c = document.querySelector<HTMLInputElement>(`[role="grid"][aria-label="${sheet}"] input[aria-label="${ref}"]`); return c?.value === shown && !c.hasAttribute("data-draft-dirty"); }, {sheet, ref, shown}));
    const dirty = await input.getAttribute("data-draft-dirty").catch(() => null);
    return record(`edit:${sheet}!${ref}`, !!ok, `typed ${JSON.stringify(text)} shows ${JSON.stringify(await value(sheet, ref))}${ok ? "" : ` dirty=${dirty} notices=${JSON.stringify((await s.notices()).slice(seen, seen + 3))}`}`);
  }
  async function stagedAction(verb: string, args: Record<string, string>) {
    const form = page.locator(`[id$=".action.${verb}.arg.${Object.keys(args)[0]}.input"]`).first();
    for (let attempt = 0; attempt < 3 && !(await form.isVisible().catch(() => false)); attempt += 1) { await s.clickAction(verb); await page.waitForTimeout(800); }
    const filled: string[] = [];
    for (const [key, v] of Object.entries(args)) filled.push(await matrix.fillStagedArgument(page, key, v, "@liveId"));
    return `${filled.join(" ")} submit=${await matrix.submitStagedVerb(page, verb)}`;
  }
  async function detailsText(pointer: string) {
    const input = page.locator(`${details} [data-ui-node-key="${detailsControlId(pointer, "text")}"]`).first();
    return (await input.count()) > 0 ? input.inputValue() : null;
  }
  async function editDetails(pointer: string, next: string) {
    const input = page.locator(`${details} [data-ui-node-key="${detailsControlId(pointer, "text")}"]`).first();
    let via = "details-field";
    if ((await input.count()) > 0) { await input.scrollIntoViewIfNeeded().catch(() => undefined); await input.fill(next); await input.press("Enter"); await input.blur().catch(() => undefined); }
    else via = `rail setSnapshotValue ${await stagedAction("setSnapshotValue", {path: pointer, value: next})}`;
    await page.waitForTimeout(2_500);
    return via;
  }
  async function partIndex(path: string) {
    for (let n = 0; n < 16; n += 1) if ((await detailsText(`/xmlParts/${n}/path`)) === path) return n;
    return -1;
  }
  async function save(name: string, out: string) {
    const saved = await s.download(() => s.clickAction("saveArtifactFile"), join(out, name));
    record(`save:${name}`, saved.ok && saved.notes.some(n => n.startsWith("shell.documentTransfer.file-saved|")), `download=${saved.ok ? saved.name : "none"} notes=${JSON.stringify(saved.notes.slice(0, 2))}`);
    return saved;
  }
  return {cell, value, sheetLabels, openSheet, setCell, stagedAction, detailsText, editDetails, partIndex, save};
}
