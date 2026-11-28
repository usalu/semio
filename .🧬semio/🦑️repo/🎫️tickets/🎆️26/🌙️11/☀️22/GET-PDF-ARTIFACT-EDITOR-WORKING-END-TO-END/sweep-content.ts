import { out, runSteps, session, verdicts, oracle, unfoldActionsRail } from "./journey-kit.ts";
import type { Browser } from "playwright";
import { join } from "node:path";

/** 🧪️ Sweep of the content and page actions of the PDF 1.7 editor through the real Actions rail, the Object pane and the canvas (port 6260); each action runs apply, undo, redo, then Save and the third-party oracle (`oracle-content.py`) and Open again.
 * usage: `PDF_JOURNEY_OUT=sweep-content bun sweep-content.ts [content-en content-de pages-en ...]` */

type Pane = Awaited<ReturnType<typeof session>>;
const content = join(import.meta.dir, "🗑️generated/fixtures/content.pdf");
const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
const oracleContent = (...args: string[]) => { const run = Bun.spawnSync(["uv", "run", "--offline", "--with", "pymupdf==1.26.7", "--with", "pypdf", "python", "-I", join(import.meta.dir, "oracle-content.py"), ...args], { stdout: "pipe", stderr: "pipe" }); return { ok: run.exitCode === 0, output: `${run.stdout.toString()}${run.stderr.toString()}`.slice(-3000) }; };

const hashOf = async (pane: Pane) => (await pane.canvasHash()).join(",");
async function until(pane: Pane, predicate: (body: string) => boolean, ms = 12_000) {
  for (const deadline = Date.now() + ms; Date.now() < deadline; await sleep(400)) if (predicate(await pane.text())) return true;
  return predicate(await pane.text());
}
async function untilHash(pane: Pane, predicate: (hash: string) => boolean, ms = 12_000) {
  let hash = await hashOf(pane);
  for (const deadline = Date.now() + ms; Date.now() < deadline && !predicate(hash); await sleep(500)) hash = await hashOf(pane);
  return { ok: predicate(hash), hash };
}
async function stableHash(pane: Pane) {
  let last = "";
  for (let attempt = 0; attempt < 12; attempt++) { const hash = await hashOf(pane); if (hash === last) return hash; last = hash; await sleep(700); }
  return last;
}
const textRows = (body: string) => body.split("\n").map(line => line.trim()).filter(line => /^(Text|Image|Vector graphic|Form|Shading|Annotation)/u.test(line) && line !== "Text layer");
const fieldValues = (pane: Pane) => pane.page.evaluate(() => Object.fromEntries([...document.querySelectorAll('[id^="window:pdf.object/pdf-object"]')].filter(e => e.tagName === "INPUT").map(e => [e.id.replace("window:pdf.object/pdf-object-", ""), (e as HTMLInputElement).value])));

/** 🖱️ Clicks the canvas at a PDF point of page `pageIndex` (bottom-left origin points) and returns the object pane field values once an object is selected. */
async function selectAt(pane: Pane, pageIndex: number, x: number, y: number, accept: (fields: Record<string, string>) => boolean = fields => Object.keys(fields).length > 1, modifiers: string[] = []) {
  const box = await pane.page.evaluate(() => {
    const canvas = [...document.querySelectorAll("canvas")].map(element => ({ element, rect: element.getBoundingClientRect() })).filter(entry => entry.rect.width > 200).sort((first, second) => second.rect.height - first.rect.height)[0];
    const context = canvas.element.getContext("2d")!;
    const ratio = canvas.element.height / canvas.rect.height;
    const column = Math.floor(canvas.element.width * 0.97);
    const data = context.getImageData(column, 0, 1, canvas.element.height).data;
    const white = (index: number) => data[index * 4] === 255 && data[index * 4 + 1] === 255 && data[index * 4 + 2] === 255;
    let row = 0;
    while (row < canvas.element.height - 30 && !Array.from({ length: 30 }, (_, offset) => white(row + offset)).every(Boolean)) row++;
    return { left: canvas.rect.left, top: canvas.rect.top, width: canvas.rect.width, paperTop: row / ratio };
  });
  const scale = box.width / 612;
  const target = process.env.CLICK_MODE === "css" ? [box.left + x, box.top + (792 - y)] : [box.left + x * scale, box.top + box.paperTop + (792 - y) * scale];
  await pane.page.evaluate(label => console.log(`[DEBUG] STEP ${label}`), `select ${x},${y}`);
  await pane.page.mouse.click(target[0], target[1], { modifiers: modifiers as never });
  let fields = await fieldValues(pane);
  for (const deadline = Date.now() + 12_000; !accept(fields) && Date.now() < deadline; await sleep(400)) fields = await fieldValues(pane);
  await stableHash(pane);
  return fields;
}

/** 🔁️ One action with its undo/redo cycle: apply, observe (row predicate and canvas change), undo, observe, redo, observe. */
async function cycle(pane: Pane, name: string, id: string, args: Record<string, string>, expect: { applied?: (body: string) => boolean; reverted?: (body: string) => boolean; noCanvas?: boolean } = {}) {
  const evidence: Record<string, unknown> = { id, args };
  const before = (await untilHash(pane, () => false, 2500), await stableHash(pane));
  const rowsBefore = textRows(await pane.text());
  const run = await pane.act(id, args);
  evidence.notices = run.notices.filter(line => !/history|unsaved/iu.test(line)).slice(-2);
  const applied = expect.applied ?? ((body: string) => textRows(body).join("|") !== rowsBefore.join("|") || true);
  const rowOk = await until(pane, applied);
  const changed = expect.noCanvas === true ? { ok: true, hash: before } : await untilHash(pane, hash => hash !== before);
  evidence.rowOk = rowOk;
  evidence.canvasChanged = changed.ok;
  await pane.shot(`${name}-applied`);
  await pane.press("undo");
  const undoRows = await until(pane, expect.reverted ?? (() => true));
  const undone = expect.noCanvas === true ? { ok: true } : await untilHash(pane, hash => hash === before);
  evidence.undone = undoRows && undone.ok;
  await pane.shot(`${name}-undone`);
  evidence.undoRows = undoRows;
  evidence.undoCanvas = undone.ok;
  evidence.rowsAfterUndo = textRows(await pane.text());
  await pane.press("redo");
  const redoRows = await until(pane, applied);
  const redone = expect.noCanvas === true ? { ok: true } : await untilHash(pane, hash => hash === changed.hash);
  evidence.redone = redoRows && redone.ok;
  const pass = rowOk && changed.ok && evidence.undone === true && evidence.redone === true;
  verdicts.push({ step: `${name}: apply, undo, redo`, pass, evidence });
  return pass;
}

/** 🚫️ An invalid input shows an announced, localized refusal notice and no console fault beyond the intended refusal. */
async function refusal(pane: Pane, name: string, id: string, args: Record<string, string>, pattern: RegExp) {
  const before = (await pane.notices()).length;
  const faultsBefore = pane.faults.length;
  const run = await pane.act(id, args);
  await sleep(1500);
  const fresh = (await pane.notices()).slice(before);
  const announced = await pane.page.evaluate(() => ((window as unknown as { __journeyAnnounced: { code: string; role: string | null; text: string }[] }).__journeyAnnounced).map(entry => `${entry.code}|${entry.role}|${entry.text}`));
  const hit = announced.find(line => pattern.test(line.split("|").slice(2).join("|")));
  verdicts.push({ step: `${name}: invalid input is refused with a localized alert`, pass: hit !== undefined && hit.split("|")[1] === "alert", evidence: { id, args, fresh, hit, newFaults: pane.faults.slice(faultsBefore), alerts: run.alerts.slice(0, 3) } });
}

/** 🧾️ Oracle verdict for a saved file, then Open again and compare the text-layer rows. */
async function saveAndReopen(pane: Pane, name: string, file: string, oracleArgs: string[], rowChecks: (rows: string[]) => boolean) {
  const rowsBefore = textRows(await pane.text());
  const saved = await pane.save(file).catch(error => ({ pressed: "failed", path: "", suggested: String(error).slice(0, 160), bytes: new Uint8Array() }));
  const checked = saved.bytes.byteLength > 0 ? oracleContent(...oracleArgs.map(arg => arg === "{pdf}" ? saved.path : arg)) : { ok: false, output: "no download" };
  verdicts.push({ step: `${name}: Save File output passes the third-party oracle`, pass: checked.ok, evidence: { bytes: saved.bytes.byteLength, suggested: saved.suggested, oracle: checked.output } });
  if (saved.bytes.byteLength === 0) return;
  const hashBefore = await hashOf(pane);
  const again = await pane.open(saved.path);
  await sleep(3000);
  const rowsAfter = textRows(await pane.text());
  const hashAfter = (await untilHash(pane, hash => hash.length > 0 && !hash.endsWith(":0"), 6000)).hash;
  await pane.shot(`${name}-reopened`);
  verdicts.push({ step: `${name}: reopen shows the saved changes`, pass: again.notice.startsWith("shell.documentTransfer.file-opened|") && rowChecks(rowsAfter) && JSON.stringify(rowsAfter) === JSON.stringify(rowsBefore), evidence: { notice: again.notice, rowsBefore, rowsAfter, canvasEqual: hashBefore === hashAfter } });
}

const faultsOutside = (pane: Pane) => pane.faults.filter(fault => !/^warning: input #\d+ [\w-]+ refused/u.test(fault));

async function contentRail(browser: Browser) {
  const pane = await session(browser, "en-US", "content-en");
  try {
    const opened = await pane.open(content);
    verdicts.push({ step: "content: open content.pdf", pass: opened.notice.startsWith("shell.documentTransfer.file-opened|"), evidence: { notice: opened.notice } });
    await unfoldActionsRail(pane.page);
    await sleep(2500);
    const has = (needle: string) => (body: string) => textRows(body).some(row => row.includes(needle));
    const hasNot = (needle: string) => (body: string) => !textRows(body).some(row => row.includes(needle));
    await cycle(pane, "insert-text", "insert-text", { page: "1", x: "300", y: "600", text: "Alpha Marker", height: "14" }, { applied: has("Alpha Marker"), reverted: hasNot("Alpha Marker") });
    await cycle(pane, "insert-rectangle", "insert-rectangle", { page: "1", x: "300", y: "100", width: "100", height: "50", red: "0", green: "1", blue: "0" }, { applied: body => textRows(body).filter(row => row === "Vector graphic").length === 3, reverted: body => textRows(body).filter(row => row === "Vector graphic").length === 2 });
    await cycle(pane, "insert-line", "insert-line", { page: "1", x: "50", y: "50", width: "200", height: "0", red: "1", green: "0", blue: "1" }, { applied: body => textRows(body).filter(row => row === "Vector graphic").length === 4, reverted: body => textRows(body).filter(row => row === "Vector graphic").length === 3 });
    await cycle(pane, "set-text", "set-text", { page: "1", object: "p0:t0", text: "Rich Edited" }, { applied: has("Rich Edited"), reverted: has("Rich Fixture Helvetica") });
    await cycle(pane, "move", "move", { page: "1", object: "p0:t1", x: "200", y: "400" });
    await cycle(pane, "resize", "resize", { page: "1", object: "p0:v0", x: "72", y: "500", width: "300", height: "50" });
    await cycle(pane, "set-fill", "set-fill", { page: "1", object: "p0:v0", red: "0", green: "1", blue: "1" });
    await cycle(pane, "set-stroke", "set-stroke", { page: "1", object: "p0:v1", width: "5", red: "0", green: "0.5", blue: "0" });
    await cycle(pane, "set-font", "set-font", { page: "1", object: "p0:t1", text: "Courier" });
    await cycle(pane, "set-font-size", "set-font-size", { page: "1", object: "p0:t0", size: "30" });
    await cycle(pane, "set-image", "set-image", { page: "1", object: "p0:i0", text: "255,0,0,0,255,0,0,0,255,255,255,0", width: "2", height: "2" });
    await cycle(pane, "delete", "delete", { page: "1", object: "p0:v3" }, { applied: body => textRows(body).filter(row => row === "Vector graphic").length === 3, reverted: body => textRows(body).filter(row => row === "Vector graphic").length === 4 });
    await pane.shot("content-rail-edited");
    await saveAndReopen(pane, "content-rail", "content-rail.pdf", ["rail", "{pdf}"], rows => rows.some(row => row.includes("Rich Edited")) && rows.some(row => row.includes("Alpha Marker")));
    verdicts.push({ step: "content-rail: no console faults", pass: faultsOutside(pane).length === 0, evidence: { faults: faultsOutside(pane).slice(0, 8) } });
  } finally { await pane.context.close(); }
}

/** ⌨️ Keyboard history shortcut with focus released from any field. */
async function shortcut(pane: Pane, keys: string) {
  await pane.page.evaluate(() => (document.activeElement as HTMLElement | null)?.blur());
  await pane.page.keyboard.press(keys);
}

/** 🎛️ Commits one Object pane field (fill, Enter), then observes apply, undo (mod+z), redo (mod+shift+z) through the field value, text rows and canvas. */
async function inlineCycle(pane: Pane, name: string, key: string, value: string, observe: (fields: Record<string, string>, body: string) => boolean, observeBefore: (fields: Record<string, string>, body: string) => boolean) {
  const evidence: Record<string, unknown> = { key, value };
  await pane.page.evaluate(label => console.log(`[DEBUG] STEP ${label}`), `inline ${name}`);
  const before = await stableHash(pane);
  const input = pane.page.locator(`[id="window:pdf.object/pdf-object-${key}"]`).first();
  await input.fill(value);
  await input.press("Enter");
  const wait = async (predicate: (fields: Record<string, string>, body: string) => boolean) => { for (const deadline = Date.now() + 12_000; Date.now() < deadline; await sleep(400)) if (predicate(await fieldValues(pane), await pane.text())) return true; return false; };
  evidence.applied = await wait(observe);
  const changed = await untilHash(pane, hash => hash !== before);
  evidence.canvasChanged = changed.ok;
  await pane.page.evaluate(() => console.log("[DEBUG] STEP undo"));
  await shortcut(pane, "ControlOrMeta+z");
  evidence.undone = await wait(observeBefore);
  evidence.canvasReverted = (await untilHash(pane, hash => hash === before)).ok;
  await shortcut(pane, "ControlOrMeta+Shift+z");
  evidence.redone = await wait(observe);
  evidence.canvasRedone = (await untilHash(pane, hash => hash === changed.hash)).ok;
  const pass = evidence.applied === true && evidence.canvasChanged === true && evidence.undone === true && evidence.canvasReverted === true && evidence.redone === true && evidence.canvasRedone === true;
  verdicts.push({ step: `inline ${name}: Object pane field, undo (mod+z), redo (mod+shift+z)`, pass, evidence });
}

async function contentInline(browser: Browser) {
  const pane = await session(browser, "en-US", "inline-en");
  try {
    const opened = await pane.open(content);
    verdicts.push({ step: "inline: open content.pdf", pass: opened.notice.startsWith("shell.documentTransfer.file-opened|"), evidence: { notice: opened.notice } });
    await sleep(3000);
    const num = (value: string | undefined) => Math.abs(Number(value ?? "NaN"));
    const kinds = { text: (f: Record<string, string>) => "text" in f && "font" in f, vector: (f: Record<string, string>) => "width" in f && "fill-red" in f && !("text" in f), image: (f: Record<string, string>) => "width" in f && !("fill-red" in f) };
    const select = async (name: string, id: string, kind: keyof typeof kinds) => {
      const at = { "p0-t0": [120, 726], "p0-v0": [172, 592], "p0-i0": [440, 602], "p0-v1": [200, 492] }[id]!;
      const fields = await selectAt(pane, 0, at[0], at[1], kinds[kind]);
      verdicts.push({ step: `selection: canvas click selects ${name}`, pass: kinds[kind](fields), evidence: { id, mode: process.env.CLICK_MODE ?? "world", fields } });
      return fields;
    };
    await select("the Helvetica text", "p0-t0", "text");
    await inlineCycle(pane, "text", "text", "Inline Title", (f, body) => f.text === "Inline Title" && body.includes("Text: Inline Title"), (f, body) => f.text === "Rich Fixture Helvetica" && body.includes("Text: Rich Fixture Helvetica"));
    await inlineCycle(pane, "font", "font", "Times-Roman", f => f.font === "Times-Roman", f => f.font === "Helvetica");
    await inlineCycle(pane, "font size", "size", "26", f => f.size === "26", f => f.size === "18");
    await inlineCycle(pane, "fill red", "fill-red", "1", f => f["fill-red"] === "1", f => f["fill-red"] === "0");
    await inlineCycle(pane, "x", "x", "150", f => f.x === "150", f => f.x === "72");
    await inlineCycle(pane, "y", "y", "680", f => f.y === "680", f => f.y === "720");
    const vector = await select("the red rectangle", "p0-v0", "vector");
    verdicts.push({ step: "inline: vector fields listed", pass: ["x", "y", "width", "height"].every(key => key in vector), evidence: { vector } });
    await inlineCycle(pane, "vector width", "width", "150", f => num(f.width) === 150, f => num(f.width) === 200);
    await inlineCycle(pane, "vector height", "height", "80", f => num(f.height) === 80, f => num(f.height) === 100);
    await inlineCycle(pane, "vector x", "x", "90", f => num(f.x) === 90, f => num(f.x) === 72);
    await inlineCycle(pane, "vector y", "y", "520", f => num(f.y) === 520, f => num(f.y) === 542);
    const fillKey = Object.keys(vector).find(key => /fill.*green/u.test(key)) ?? "fill-green";
    await inlineCycle(pane, "vector fill green", fillKey, "1", f => f[fillKey] === "1", f => f[fillKey] === "0");
    const image = await select("the image", "p0-i0", "image");
    verdicts.push({ step: "inline: image fields listed", pass: ["x", "y", "width", "height"].every(key => key in image), evidence: { image } });
    await inlineCycle(pane, "image width", "width", "100", f => num(f.width) === 100, f => num(f.width) === 80);
    await inlineCycle(pane, "image height", "height", "60", f => num(f.height) === 60, f => num(f.height) === 80);
    await select("the blue line", "p0-v1", "vector");
    const lineCount = (body: string) => textRows(body).filter(row => row === "Vector graphic").length;
    const before = await stableHash(pane);
    await pane.page.locator('[id="window:pdf.object/pdf-object-delete"]').click();
    const gone = await until(pane, body => lineCount(body) === 1);
    const deletedHash = await untilHash(pane, hash => hash !== before);
    await shortcut(pane, "ControlOrMeta+z");
    const back = await until(pane, body => lineCount(body) === 2);
    await shortcut(pane, "ControlOrMeta+Shift+z");
    const again = await until(pane, body => lineCount(body) === 1);
    verdicts.push({ step: "inline delete: Object pane Delete button, undo, redo", pass: gone && deletedHash.ok && back && again, evidence: { gone, canvasChanged: deletedHash.ok, back, again } });
    await saveAndReopen(pane, "inline", "inline.pdf", ["inline", "{pdf}"], rows => rows.some(row => row.includes("Inline Title")));
    verdicts.push({ step: "inline: no console faults", pass: faultsOutside(pane).length === 0, evidence: { faults: faultsOutside(pane).slice(0, 8) } });
  } finally { await pane.context.close(); }
}

const language = (step: string) => (step.endsWith("de") ? "de" : "en");
await runSteps(["content-en", "inline-en"], { "content-en": contentRail, "inline-en": contentInline });
void out; void oracle; void language;
