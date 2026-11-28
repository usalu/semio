import { runSteps, session, verdicts, root, out } from "./journey-kit.ts";
import type { Browser } from "playwright";
import { join } from "node:path";
import { writeFile } from "node:fs/promises";

/** 📑️ Document-level action sweep (port 6260): every document action through the Actions rail with visible diff, undo, redo, Save, third-party oracle and reopen; invalid inputs in en and de; Object pane inline fields. Evidence under `🗑️generated/<PDF_JOURNEY_OUT>`; oracle checks live in `oracle-document.py`.
 * @see ./🧭️sweep-document.md */

const fx = join(import.meta.dir, "🗑️generated/fixtures");
type Pane = Awaited<ReturnType<typeof session>>;
type Checks = Record<string, unknown>;
type Case = { feature: string; id: string; args: Record<string, string>; visible?: RegExp; hidden?: boolean; rebuilt?: boolean; before: Checks; after: Checks };

const snap = (pane: Pane) => pane.page.evaluate(() => {
  const inputs = [...document.querySelectorAll("input,select,textarea")].filter(element => !element.id.includes("action.") && !element.id.includes("snapshot-details")).map(element => `${element.getAttribute("aria-label") ?? element.id.slice(-30)}=${(element as HTMLInputElement).value}`);
  const text = [...document.querySelectorAll('[id^="window:pdf.text-layer/"],[id^="window:pdf.object/"]')].filter(element => !(element instanceof HTMLInputElement)).map(element => (element as HTMLElement).innerText.replace(/\s+/gu, " ").trim()).filter(line => line.length > 0);
  return [...inputs.map(line => `input: ${line}`), ...text.map(line => `text: ${line}`)];
});
const delta = (before: string[], after: string[]) => ({ added: after.filter(line => !before.includes(line)), removed: before.filter(line => !after.includes(line)) });
const flakes: string[] = [];
async function openFile(pane: Pane, path: string, tries = 3) {
  let last = await pane.open(path);
  for (let attempt = 1; attempt < tries && !last.notice.startsWith("shell.documentTransfer.file-opened|"); attempt++) { flakes.push(`${path.split("/").pop()}: ${last.notice}`); last = await pane.open(path); }
  return last;
}
const settle = (pane: Pane, ms = 1500) => pane.page.waitForTimeout(ms);
const shown = (change: { added: string[]; removed: string[] }) => [...change.added.map(line => `+ ${line}`), ...change.removed.map(line => `- ${line}`)];

async function cycle(pane: Pane, language: string, item: Case) {
  const before = await snap(pane);
  const result = await pane.act(item.id, item.args);
  await settle(pane);
  const applied = await snap(pane);
  const change = delta(before, applied);
  await pane.press("undo");
  await settle(pane);
  const undone = await snap(pane);
  const undoGap = delta(before, undone);
  await pane.press("redo");
  await settle(pane);
  const redone = await snap(pane);
  const redoGap = delta(applied, redone);
  const visible = item.hidden === true || (item.visible === undefined ? shown(change).length > 0 : shown(change).some(line => item.visible!.test(line)));
  const undoOk = undoGap.added.length === 0 && undoGap.removed.length === 0;
  const redoOk = redoGap.added.length === 0 && redoGap.removed.length === 0;
  verdicts.push({ step: `${language}: ${item.feature}: applies visibly, undo reverts, redo reapplies`, pass: visible && undoOk && redoOk && !result.covered, evidence: { id: item.id, args: item.args, noUiReadback: item.hidden === true && shown(change).length === 0, change: shown(change).slice(0, 12), undoGap: shown(undoGap).slice(0, 8), redoGap: shown(redoGap).slice(0, 8), alerts: result.alerts.slice(0, 4), notices: result.notices.slice(-2) } });
}

const baseline: Checks = { info: { title: "Doc Fixture", author: "Sweep", subject: "Document level", keywords: "pdf,sweep" }, lang: null, layout: null, mode: null, toc: [[1, "Start", 1], [1, "Middle", 2], [2, "End", 3]], markinfo: null, xmp: null, viewer: { FitWindow: null, NumCopies: null, PrintPageRange: null, Direction: null }, openaction: null, dests: { "chapter-two": 2 }, embedded: { "attached.txt": "embedded file body" }, outputintents: null, pages: 3, text: { 0: "Doc Fixture Page One", 1: "Doc Fixture Page Two", 2: "Doc Fixture Page Three" } };

function runOracle(path: string, checks: Checks, password = "") {
  const spec = `${path}.checks.json`;
  Bun.write(spec, JSON.stringify(checks));
  const run = Bun.spawnSync(["uv", "run", "--offline", "--with", "pymupdf==1.26.7", "--with", "pypdf", "python", "-I", join(import.meta.dir, "oracle-document.py"), "check", path, spec, password], { stdout: "pipe", stderr: "pipe" });
  return { ok: run.exitCode === 0, output: `${run.stdout.toString()}${run.stderr.toString()}`.slice(-1500) };
}

async function saved(pane: Pane, language: string, name: string, checks: Checks, password = "") {
  const file = await pane.save(`${name}.pdf`);
  const checked = runOracle(file.path, checks, password);
  verdicts.push({ step: `${language}: ${name} saved file passes the third-party oracle`, pass: checked.ok, evidence: { bytes: file.bytes.byteLength, suggested: file.suggested, checks: Object.keys(checks), output: checked.output } });
  return { ...file, ok: checked.ok };
}

async function byteCycle(pane: Pane, language: string, item: Case, index: number) {
  const slug = `${language}-${index}-${item.id}`;
  const before = await snap(pane);
  const result = await pane.act(item.id, item.args);
  await settle(pane);
  const applied = await snap(pane);
  const change = shown(delta(before, applied));
  const one = runOracle((await pane.save(`${slug}-applied.pdf`)).path, { ...item.after, pages: 3 });
  await pane.press("undo"); await settle(pane);
  const undone = runOracle((await pane.save(`${slug}-undone.pdf`)).path, { ...item.before, pages: 3 });
  await pane.press("redo"); await settle(pane);
  const redone = runOracle((await pane.save(`${slug}-redone.pdf`)).path, { ...item.after, pages: 3 });
  verdicts.push({ step: `${language}: ${item.feature}: Save shows it, undo+Save removes it, redo+Save restores it (oracle)`, pass: one.ok && undone.ok && redone.ok && !result.covered, evidence: { id: item.id, args: item.args, uiChange: change.slice(0, 6), applied: one.output, undone: undone.output, redone: redone.output, alerts: result.alerts.slice(0, 3) } });
}

const lang = (code: "en" | "de") => code === "en" ? "en-US" as const : "de-DE" as const;
const xmp = `<?xpacket begin="" id="W5M0MpCehiHzreSzNTczkc9d"?><x:xmpmeta xmlns:x="adobe:ns:meta/"><rdf:RDF xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#"><rdf:Description rdf:about="" xmlns:dc="http://purl.org/dc/elements/1.1/"><dc:title><rdf:Alt><rdf:li xml:lang="x-default">Swept XMP</rdf:li></rdf:Alt></dc:title></rdf:Description></rdf:RDF></x:xmpmeta><?xpacket end="w"?>`;
const prefs = (value: Record<string, string | null>) => ({ viewer: value });
const field = (feature: string, key: string, text: string, was: string | null, now: string, visible: RegExp, oracleKey = key): Case => ({ feature: `set-info-field ${feature}`, id: "set-info-field", args: { object: key, text }, visible, before: { info: { [oracleKey]: was } }, after: { info: { [oracleKey]: now } } });

const batchA: Case[] = [
  { feature: "set-info", id: "set-info", args: { object: "Swept Subject", text: "Swept Title", extra: "Swept Author" }, visible: /Swept Title/u, before: { info: { title: "Doc Fixture", author: "Sweep", subject: "Document level" } }, after: { info: { title: "Swept Title", author: "Swept Author", subject: "Swept Subject" } } },
  field("keywords", "keywords", "alpha, beta", "pdf,sweep", "alpha, beta", /alpha, beta/u),
  field("creator", "creator", "SweepCreator", null, "SweepCreator", /SweepCreator/u),
  field("producer", "producer", "SweepProducer", null, "SweepProducer", /SweepProducer/u),
  field("trapped", "trapped", "True", null, "/True", /True/u),
  field("creationDate", "creationDate", "D:20240102030405", null, "20240102030405", /2024/u),
  field("modificationDate", "modificationDate", "D:20250607080910", null, "20250607080910", /2025/u, "modDate"),
  field("author", "author", "Field Author", "Swept Author", "Field Author", /Field Author/u),
  field("subject", "subject", "Field Subject", "Swept Subject", "Field Subject", /Field Subject/u),
  field("title", "title", "Field Title", "Swept Title", "Field Title", /Field Title/u),
  { feature: "set-language", id: "set-language", args: { text: "de-CH" }, visible: /de-CH/u, before: { lang: null }, after: { lang: "de-CH" } },
  { feature: "set-page-layout", id: "set-page-layout", args: { text: "twoColumnLeft" }, visible: /twoColumnLeft/u, before: { layout: null }, after: { layout: "TwoColumnLeft" } },
  { feature: "set-page-mode", id: "set-page-mode", args: { text: "useOutlines" }, visible: /useOutlines/u, before: { mode: null }, after: { mode: "UseOutlines" } },
  { feature: "set-outline", id: "set-outline", args: { page: "2", text: "Added Chapter", x: "-1" }, hidden: true, before: { toc: baseline.toc }, after: { toc: [...(baseline.toc as unknown[]), [1, "Added Chapter", 2]] } },
  { feature: "set-mark-info", id: "set-mark-info", args: { x: "1", y: "1", width: "0" }, visible: /Tagged/u, before: { markinfo: null }, after: { markinfo: { Marked: true, UserProperties: true, Suspects: false } } },
  { feature: "set-metadata", id: "set-metadata", args: { text: xmp }, visible: /Swept XMP/u, before: { xmp: null }, after: { xmp: "Swept XMP" } },
  { feature: "set-viewer-preferences fitWindow", id: "set-viewer-preferences", args: { object: "fitWindow", x: "1" }, hidden: true, before: prefs({ FitWindow: null }), after: prefs({ FitWindow: "true" }) },
  { feature: "set-viewer-preferences numCopies", id: "set-viewer-preferences", args: { object: "numCopies", x: "2" }, hidden: true, before: prefs({ NumCopies: null }), after: prefs({ NumCopies: "2" }) },
  { feature: "set-viewer-preferences printPageRange", id: "set-viewer-preferences", rebuilt: true, args: { object: "printPageRange", text: "1,2", x: "0" }, hidden: true, before: prefs({ PrintPageRange: null }), after: prefs({ PrintPageRange: "\\[\\s*1\\s+2\\s*\\]" }) },
  { feature: "set-viewer-preferences direction", id: "set-viewer-preferences", rebuilt: true, args: { object: "direction", text: "R2L", x: "0" }, hidden: true, before: prefs({ Direction: null }), after: prefs({ Direction: "/R2L" }) },
  { feature: "set-open-action page", id: "set-open-action", args: { object: "page", page: "2" }, hidden: true, before: { openaction: null }, after: { openaction: { kind: "page", page: 2 } } },
  { feature: "set-document-id", id: "set-document-id", args: { text: "abc123", extra: "def456" }, before: { docid: null }, after: { docid: ["abc123", "def456"] } },
  { feature: "set-named-destination", id: "set-named-destination", args: { page: "2", object: "intro" }, hidden: true, before: { dests: { "chapter-two": 2 } }, after: { dests: { "chapter-two": 2, intro: 2 } } },
  { feature: "remove-named-destination", id: "remove-named-destination", args: { object: "chapter-two" }, hidden: true, before: { dests: { "chapter-two": 2, intro: 2 } }, after: { dests: { intro: 2 } } },
  { feature: "set-embedded-file", id: "set-embedded-file", args: { object: "note.txt", text: "note.txt", extra: "hello note" }, hidden: true, before: { embedded: { "attached.txt": "embedded file body" } }, after: { embedded: { "attached.txt": "embedded file body", "note.txt": "hello note" } } },
  { feature: "remove-embedded-file", id: "remove-embedded-file", args: { object: "attached.txt" }, hidden: true, before: { embedded: { "attached.txt": "embedded file body", "note.txt": "hello note" } }, after: { embedded: { "note.txt": "hello note" } } },
  { feature: "set-output-intent", id: "set-output-intent", args: { object: "GTS_PDFA1", text: "sRGB IEC61966-2.1", extra: "sweep intent" }, hidden: true, before: { outputintents: null }, after: { outputintents: [{ subtype: "GTS_PDFA1", identifier: "sRGB IEC61966-2.1", info: "sweep intent" }] } },
  { feature: "set-catalog-entry", id: "set-catalog-entry", args: { object: "Marker", text: "Yes" }, before: { catalog: { Marker: null } }, after: { catalog: { Marker: "/Yes" } } },
  { feature: "set-trailer-entry", id: "set-trailer-entry", args: { object: "Foo", text: "Bar" }, before: { trailer: { Foo: null } }, after: { trailer: { Foo: "/Bar" } } },
  { feature: "set-entry catalog", id: "set-entry", args: { object: "catalog", text: "Marker2=Yes" }, before: { catalog: { Marker2: null } }, after: { catalog: { Marker2: "/Yes" } } },
  { feature: "set-entry info", id: "set-entry", args: { object: "info", extra: "Custom", text: "Kept" }, before: { infoextra: { Custom: null } }, after: { infoextra: { Custom: "/Kept" } } },
  { feature: "set-extra-entry info", id: "set-extra-entry", args: { object: "info", extra: "Custom2", text: "Kept2" }, before: { infoextra: { Custom2: null } }, after: { infoextra: { Custom2: "/Kept2" } } },
  { feature: "set-extra-entry viewer", id: "set-extra-entry", args: { object: "viewer", extra: "Custom3", text: "Kept3" }, hidden: true, before: prefs({ Custom3: null }), after: prefs({ Custom3: "/Kept3" }) },
  { feature: "set-optional-content", id: "set-optional-content", args: { object: "OCG1", text: "Renamed Layer", x: "0" }, hidden: true, before: { ocg: { names: ["Layer One"], on: ["Layer One"] } }, after: { ocg: { names: ["Renamed Layer"], off: ["Renamed Layer"] } } },
  { feature: "set-indirect-object", id: "set-indirect-object", args: { x: "900", y: "0", text: "Hello" }, hidden: true, before: { indirect: { 900: null } }, after: { indirect: { 900: "/Hello" } } },
];

const merge = (items: Case[]) => items.reduce<Checks>((all, item) => {
  for (const [key, value] of Object.entries(item.after)) all[key] = typeof value === "object" && value !== null && !Array.isArray(value) && typeof all[key] === "object" && all[key] !== null && !Array.isArray(all[key]) && key !== "embedded" && key !== "dests" ? { ...(all[key] as object), ...value } : value;
  return all;
}, { pages: 3, text: baseline.text });

const unwind = (items: Case[]) => items.reduce<Checks>((all, item) => {
  for (const [key, value] of Object.entries(item.before)) {
    if (typeof value === "object" && value !== null && !Array.isArray(value) && key !== "embedded" && key !== "dests") all[key] = { ...(value as object), ...(all[key] as object | undefined) };
    else if (!(key in all)) all[key] = value;
  }
  return all;
}, { pages: 3, text: baseline.text });

async function docA(browser: Browser, code: "en" | "de") {
  const pane = await session(browser, lang(code), `docA-${code}`);
  try {
    const opened = await openFile(pane, join(fx, "doc.pdf"));
    verdicts.push({ step: `${code}: open doc.pdf`, pass: opened.notice.startsWith("shell.documentTransfer.file-opened|"), evidence: { ...opened, flakes } });
    await settle(pane, 3000);
    const only = process.env.ONLY?.split(",");
    const items = batchA.filter(item => (only === undefined || only.includes(item.feature)) && (process.env.REBUILT === "1" || item.rebuilt !== true));
    let index = 0;
    for (const item of items) {
      await cycle(pane, code, item);
    }
    await pane.shot(`${code}-docA-applied`);
    const final = merge(items);
    const file = await saved(pane, code, `${code}-docA`, final);
    for (let step = 0; step < items.length; step++) { await pane.press("undo"); await settle(pane, 500); }
    await settle(pane);
    await saved(pane, code, `${code}-docA-all-undone`, unwind(items));
    for (let step = 0; step < items.length; step++) { await pane.press("redo"); await settle(pane, 500); }
    await settle(pane);
    await saved(pane, code, `${code}-docA-all-redone`, final);
    const again = await openFile(pane, file.path);
    await settle(pane, 3000);
    const reopened = (await snap(pane)).filter(line => line.startsWith("input: "));
    verdicts.push({ step: `${code}: reopen of the saved docA file shows the edits again`, pass: again.notice.startsWith("shell.documentTransfer.file-opened|"), evidence: { notice: again.notice, inputs: reopened.filter(line => !/detail|Rotation|Page (width|height)|User unit|box|Add /u.test(line)) } });
    await saved(pane, code, `${code}-docA-second-generation`, final);
  } finally { await pane.context.close(); }
}

const announced = (pane: Pane) => pane.page.evaluate(() => [...((window as unknown as { __journeyAnnounced?: { code: string; role: string | null; live: string | null; text: string }[] }).__journeyAnnounced ?? [])]);

type Invalid = { feature: string; id: string; args: Record<string, string>; unchanged?: string };
const invalids: Invalid[] = [
  { feature: "set-info-field unknown field", id: "set-info-field", args: { object: "nonexistent", text: "x" } },
  { feature: "set-info-field bad date", id: "set-info-field", args: { object: "creationDate", text: "yesterday" } },
  { feature: "set-page-layout unknown value", id: "set-page-layout", args: { text: "bogus" } },
  { feature: "set-page-mode unknown value", id: "set-page-mode", args: { text: "bogus" } },
  { feature: "set-outline page out of range", id: "set-outline", args: { page: "99", text: "Nope", x: "-1" } },
  { feature: "set-viewer-preferences unknown key", id: "set-viewer-preferences", args: { object: "bogus", x: "1" } },
  { feature: "set-open-action uri without address", id: "set-open-action", args: { object: "uri", text: "" } },
  { feature: "set-open-action unknown kind", id: "set-open-action", args: { object: "bogus", text: "" } },
  { feature: "set-open-action page out of range", id: "set-open-action", args: { object: "page", page: "99" } },
  { feature: "set-named-destination page out of range", id: "set-named-destination", args: { page: "99", object: "x" } },
  { feature: "remove-named-destination unknown name", id: "remove-named-destination", args: { object: "does-not-exist" } },
  { feature: "remove-embedded-file unknown id", id: "remove-embedded-file", args: { object: "does-not-exist" } },
  { feature: "set-encryption unknown algorithm", id: "set-encryption", args: { object: "owner", text: "bogus", extra: "user" } },
  { feature: "set-output-intent without identifier", id: "set-output-intent", args: { object: "GTS_PDFA1", text: "", extra: "" } },
  { feature: "set-catalog-entry structural key Pages", id: "set-catalog-entry", args: { object: "Pages", text: "Foo" } },
  { feature: "set-trailer-entry structural key Root", id: "set-trailer-entry", args: { object: "Root", text: "Foo" } },
  { feature: "set-trailer-entry structural key Size", id: "set-trailer-entry", args: { object: "Size", text: "Foo" } },
  { feature: "set-entry unknown family", id: "set-entry", args: { object: "bogus", text: "k=v" } },
  { feature: "set-entry without key=value", id: "set-entry", args: { object: "catalog", text: "novalue" } },
  { feature: "set-extra-entry unknown owner", id: "set-extra-entry", args: { object: "bogus", extra: "K", text: "v" } },
  { feature: "set-optional-content empty name", id: "set-optional-content", args: { object: "OCG1", text: "", x: "1" } },
];

async function invalid(browser: Browser, code: "en" | "de") {
  const pane = await session(browser, lang(code), `invalid-${code}`);
  try {
    const opened = await openFile(pane, join(fx, "doc.pdf"));
    verdicts.push({ step: `${code}: open doc.pdf for invalid inputs`, pass: opened.notice.startsWith("shell.documentTransfer.file-opened|"), evidence: { ...opened, flakes } });
    await settle(pane, 3000);
    const only = process.env.ONLY?.split(",");
    for (const item of invalids.filter(entry => only === undefined || only.includes(entry.feature))) {
      await pane.page.evaluate(() => { for (const element of document.querySelectorAll("[data-semio-transient-notice]")) (element.querySelector("button") as HTMLButtonElement | null)?.click(); });
      await settle(pane, 400);
      const before = await snap(pane);
      const result = await pane.act(item.id, item.args);
      const live = () => pane.page.evaluate(() => [...document.querySelectorAll("[data-semio-transient-notice]")].map(element => ({ code: element.getAttribute("data-notice-code") ?? "", role: element.getAttribute("role"), live: element.getAttribute("aria-live"), text: (element.firstChild?.textContent ?? "").trim() })));
      let shownNow: Awaited<ReturnType<typeof live>> = [];
      for (const deadline = Date.now() + 6000; shownNow.length === 0 && Date.now() < deadline; await settle(pane, 400)) shownNow = await live();
      const after = await snap(pane);
      const changed = shown(delta(before, after));
      const refusal = shownNow.find(entry => entry.role === "alert");
      const coded = refusal !== undefined && refusal.code.startsWith("stdio.pdf.");
      verdicts.push({ step: `${code}: invalid ${item.feature} shows a localized alert and leaves the document unchanged`, pass: refusal !== undefined && coded && changed.length === 0, evidence: { args: item.args, shownNow, changed: changed.slice(0, 4), coded, covered: result.covered, form: result.form, executeState: result.executeState, notices: result.notices.slice(-2) } });
    }
    await pane.shot(`${code}-invalid`);
  } finally { await pane.context.close(); }
}

const inline: { feature: string; slug: string; value: string; visible: string; after: Checks }[] = [
  { feature: "Object pane Page layout", slug: "pdf-doc-layout", value: "oneColumn", visible: "oneColumn", after: { layout: "OneColumn" } },
  { feature: "Object pane Page mode", slug: "pdf-doc-mode", value: "useThumbs", visible: "useThumbs", after: { mode: "UseThumbs" } },
  { feature: "Object pane XMP metadata", slug: "pdf-doc-metadata", value: xmp, visible: "Swept XMP", after: { xmp: "Swept XMP" } },
  { feature: "Object pane Tagged", slug: "pdf-doc-marked", value: "1", visible: "1", after: { markinfo: { Marked: true, UserProperties: false, Suspects: false } } },
  { feature: "Object pane permanent document id", slug: "pdf-doc-id-permanent", value: "inlineA", visible: "inlineA", after: {} },
  { feature: "Object pane changing document id", slug: "pdf-doc-id-changing", value: "inlineB", visible: "inlineB", after: { docid: ["inlineA", "inlineB"] } },
  { feature: "Object pane Open action link address", slug: "pdf-doc-open-link", value: "https://example.org/inline", visible: "https://example.org/inline", after: { openaction: { kind: "uri", uri: "https://example.org/inline" } } },
  { feature: "Object pane Add bookmark", slug: "pdf-page-bookmark", value: "Inline Bookmark", visible: "", after: { toc: [[1, "Start", 1], [1, "Middle", 2], [2, "End", 3], [1, "Inline Bookmark", 1]] } },
  { feature: "Object pane Named destination", slug: "pdf-page-destination", value: "inline-dest", visible: "", after: { dests: { "chapter-two": 2, "inline-dest": 1 } } },
  { feature: "Object pane Page label prefix", slug: "pdf-page-label-prefix", value: "P-", visible: "P-", after: { labels: [{ startpage: 0, prefix: "P-" }] } },
  { feature: "Object pane Catalog entry add", slug: "pdf-entry-catalog-add", value: "InlineC=Yes", visible: "InlineC", after: { catalog: { InlineC: "/Yes" } } },
  { feature: "Object pane Trailer entry add", slug: "pdf-entry-trailer-add", value: "InlineT=Yes", visible: "InlineT", after: { trailer: { InlineT: "/Yes" } } },
  { feature: "Object pane Info entry add", slug: "pdf-entry-info-add", value: "InlineI=Yes", visible: "InlineI", after: { infoextra: { InlineI: "/Yes" } } },
  { feature: "Object pane Viewer entry add", slug: "pdf-entry-viewer-add", value: "InlineV=Yes", visible: "", after: { viewer: { InlineV: "/Yes" } } },
  { feature: "Text layer Language", slug: "pdf-page-language", value: "fr-CH", visible: "fr-CH", after: { lang: "fr-CH" } },
];

async function inlineFields(browser: Browser, code: "en" | "de") {
  const pane = await session(browser, lang(code), `inline-${code}`);
  try {
    const opened = await openFile(pane, join(fx, "doc.pdf"));
    verdicts.push({ step: `${code}: open doc.pdf for inline fields`, pass: opened.notice.startsWith("shell.documentTransfer.file-opened|"), evidence: { ...opened, flakes } });
    await settle(pane, 3000);
    const merged: Checks = { pages: 3, text: baseline.text };
    for (const item of inline) {
      const input = pane.page.locator(`[id$="/${item.slug}"]`).first();
      await input.waitFor({ state: "visible", timeout: 20_000 });
      const before = await snap(pane);
      await input.fill(item.value);
      await input.press("Enter");
      await settle(pane, 1500);
      const after = await snap(pane);
      const change = shown(delta(before, after));
      const field = await input.inputValue().catch(() => "");
      const pass = item.visible === "" ? true : field.includes(item.visible) || change.some(line => line.includes(item.visible));
      verdicts.push({ step: `${code}: ${item.feature} commits on Enter and shows the value`, pass, evidence: { value: item.value.slice(0, 60), field: field.slice(0, 60), change: change.slice(0, 4), noUiReadback: item.visible === "" } });
      for (const [key, value] of Object.entries(item.after)) merged[key] = typeof value === "object" && !Array.isArray(value) && value !== null && typeof merged[key] === "object" && merged[key] !== null && !Array.isArray(merged[key]) && key !== "dests" ? { ...(merged[key] as object), ...value } : value;
    }
    await pane.press("undo"); await settle(pane);
    const undone = await snap(pane);
    await pane.press("redo"); await settle(pane);
    const redone = await snap(pane);
    const language = redone.find(line => line.startsWith("input: Language=")) ?? "";
    verdicts.push({ step: `${code}: undo reverts and redo restores the last inline edit`, pass: !(undone.find(line => line.startsWith("input: Language=")) ?? "").includes("fr-CH") && language.includes("fr-CH"), evidence: { undone: undone.find(line => line.startsWith("input: Language=")), language } });
    const file = await saved(pane, code, `${code}-inline`, merged);
    const again = await openFile(pane, file.path);
    await settle(pane, 3000);
    const inputs = (await snap(pane)).filter(line => /^input: (Language|Page layout|Page mode|XMP|Tagged|Permanent|Changing|Open action|Page label)/u.test(line));
    verdicts.push({ step: `${code}: reopen of the inline-edited file shows the fields again`, pass: again.notice.startsWith("shell.documentTransfer.file-opened|") && inputs.some(line => line.includes("fr-CH")) && inputs.some(line => line.includes("oneColumn")) && inputs.some(line => line.includes("useThumbs")), evidence: { inputs: inputs.map(line => line.slice(0, 120)) } });
    const cursor = (await announced(pane)).length;
    const input = pane.page.locator('[id$="/pdf-doc-layout"]').first();
    await input.fill("bogus"); await input.press("Enter");
    let fresh: Awaited<ReturnType<typeof announced>> = [];
    for (const deadline = Date.now() + 6000; fresh.length === 0 && Date.now() < deadline; await settle(pane, 400)) fresh = (await announced(pane)).slice(cursor);
    verdicts.push({ step: `${code}: an invalid inline Page layout shows a localized alert`, pass: fresh.some(entry => entry.role === "alert" && entry.code.startsWith("stdio.pdf.")), evidence: { fresh } });
    await pane.shot(`${code}-inline`);
  } finally { await pane.context.close(); }
}

const OPENED = /^shell\.documentTransfer\.file-opened\|/u;
const titleLabel = { en: "Title", de: "Titel" } as const;
const inputByLabel = (pane: Pane, label: string) => pane.page.getByRole("textbox", { name: label, exact: true }).first();

async function transfer(browser: Browser, code: "en" | "de") {
  const pane = await session(browser, lang(code), `transfer-${code}`);
  try {
    const opened = await openFile(pane, join(fx, "plain.pdf"));
    verdicts.push({ step: `${code}: Open File opens plain.pdf`, pass: OPENED.test(opened.notice), evidence: { ...opened, flakes } });
    await settle(pane, 2500);
    const title = code === "en" ? "Transfer Title" : "Übertragungstitel";
    await pane.setTitle(code, title);
    const first = await saved(pane, code, `${code}-transfer-save`, { pages: 3 - 2, info: { title }, text: { 0: "Plain Page" } });
    const again = await openFile(pane, first.path);
    await settle(pane, 2500);
    verdicts.push({ step: `${code}: Save File then Open File shows the title again`, pass: OPENED.test(again.notice) && (await pane.titleValue(code)) === title, evidence: { notice: again.notice, title: await pane.titleValue(code) } });

    const cursor = (await announced(pane)).length;
    const garbage = await pane.open(join(fx, "garbage.pdf"));
    const fresh = (await announced(pane)).slice(cursor);
    await settle(pane, 1000);
    verdicts.push({ step: `${code}: opening a non-PDF file is refused with a localized alert and keeps the current document`, pass: /file-open-failed/u.test(garbage.notice) && (await pane.titleValue(code)) === title && fresh.some(entry => entry.role === "alert"), evidence: { notice: garbage.notice, fresh, titleKept: await pane.titleValue(code) } });
    await pane.shot(`${code}-transfer-garbage`);

    for (const name of ["encrypted-aes256", "encrypted-rc4"]) {
      const cursorEncrypted = (await announced(pane)).length;
      const locked = await pane.open(join(fx, `${name}.pdf`));
      await settle(pane, 2500);
      verdicts.push({ step: `${code}: opening ${name}.pdf gives a localized outcome (opened or refused with an alert)`, pass: /file-opened|file-open-failed/u.test(locked.notice) && (/file-opened/u.test(locked.notice) || (await announced(pane)).slice(cursorEncrypted).some(entry => entry.role === "alert")), evidence: { notice: locked.notice, fresh: (await announced(pane)).slice(cursorEncrypted), text: (await snap(pane)).slice(0, 8) } });
      await pane.shot(`${code}-transfer-${name}`);
    }

    const again2 = await openFile(pane, first.path);
    await settle(pane, 2500);
    const body = code === "en" ? "Exported Body" : "Exportierter Text";
    await pane.insertText(body);
    await settle(pane);
    const [download, pressed] = await Promise.all([pane.page.waitForEvent("download", { timeout: 90_000 }), pane.press("exportArtifactDocument")]);
    const exported = join(out, `${code}-exported-document`);
    await download.saveAs(exported);
    const size = (await Bun.file(exported).arrayBuffer()).byteLength;
    await openFile(pane, join(fx, "doc.pdf"));
    await settle(pane, 2500);
    const importCursor = (await pane.notices()).length;
    const [chooser] = await Promise.all([pane.page.waitForEvent("filechooser", { timeout: 30_000 }), pane.press("importArtifactDocument")]);
    await chooser.setFiles(exported);
    await settle(pane, 8000);
    const afterImport = await pane.text();
    verdicts.push({ step: `${code}: Export Document then Import Document restores the exported document`, pass: size > 0 && afterImport.includes(body) && !afterImport.includes("Doc Fixture Page One"), evidence: { suggested: download.suggestedFilename(), size, pressed, notices: (await pane.notices()).slice(importCursor), hasBody: afterImport.includes(body), openedPlain: again2.notice } });
    const garbageCursor = (await announced(pane)).length;
    const [chooser2] = await Promise.all([pane.page.waitForEvent("filechooser", { timeout: 30_000 }), pane.press("importArtifactDocument")]);
    await chooser2.setFiles(join(fx, "garbage.pdf"));
    let refused: Awaited<ReturnType<typeof announced>> = [];
    for (const deadline = Date.now() + 15_000; refused.length === 0 && Date.now() < deadline; await settle(pane, 500)) refused = (await announced(pane)).slice(garbageCursor);
    verdicts.push({ step: `${code}: Import Document of a garbage file is refused with an alert and keeps the document`, pass: refused.some(entry => entry.role === "alert") && (await pane.text()).includes(body), evidence: { refused, notices: (await pane.notices()).slice(-3) } });
    await pane.shot(`${code}-transfer-import-garbage`);

    const palette = await pane.page.evaluate(() => [...document.querySelectorAll('[id="action.exportMediaFile"],[id$="exportMediaFile"]')].map(element => element.id));
    verdicts.push({ step: `${code}: the exportMediaFile "Export PDF File" action is reachable from the UI`, pass: palette.length > 0, evidence: { ids: palette, note: "declared with in_palette=false, so the Actions rail does not list it" } });
  } finally { await pane.context.close(); }
}

const base = join(root, "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base");
const thesis = join(base, "🖼️assets/🎓️bachelor-thesis/🎓️bachelor-thesis.pdf");
const repoFixtures = [join(base, "🧪️tests/🧫️fixtures/🔗️indirect-length.pdf"), join(base, "🧪️tests/🧫️fixtures/🩹️repair-stream.pdf")];

function compareOracle(original: string, saved: string, title?: string) {
  const run = Bun.spawnSync(["uv", "run", "--offline", "--with", "pymupdf==1.26.7", "--with", "pypdf", "python", "-I", join(import.meta.dir, "oracle-document.py"), "compare", original, saved, ...(title === undefined ? [] : [title])], { stdout: "pipe", stderr: "pipe" });
  return { ok: run.exitCode === 0, output: `${run.stdout.toString()}${run.stderr.toString()}`.slice(-1200) };
}

async function large(browser: Browser, code: "en" | "de") {
  const pane = await session(browser, lang(code), `large-${code}`);
  try {
    const started = Date.now();
    const opened = await openFile(pane, thesis);
    verdicts.push({ step: `${code}: open the bachelor thesis`, pass: OPENED.test(opened.notice), evidence: { ...opened, flakes, seconds: Math.round((Date.now() - started) / 1000) } });
    await settle(pane, 5000);
    const title = code === "en" ? "Thesis Retitled" : "Arbeit umbenannt";
    await pane.setTitle(code, title);
    const file = await pane.save(`${code}-thesis.pdf`);
    const original = new Uint8Array(await Bun.file(thesis).arrayBuffer());
    const prefix = file.bytes.byteLength > original.byteLength && original.every((value, index) => file.bytes[index] === value);
    const checked = compareOracle(thesis, file.path, title);
    verdicts.push({ step: `${code}: thesis title edit saves incrementally and every page matches the original in the oracle`, pass: checked.ok && prefix, evidence: { bytes: file.bytes.byteLength, original: original.byteLength, incrementalPrefix: prefix, output: checked.output } });
    const again = await openFile(pane, file.path);
    await settle(pane, 5000);
    verdicts.push({ step: `${code}: the saved thesis reopens with the new title`, pass: OPENED.test(again.notice) && (await pane.titleValue(code)) === title, evidence: { notice: again.notice, title: await pane.titleValue(code) } });
  } finally { await pane.context.close(); }
}

async function fixtures(browser: Browser, code: "en" | "de") {
  const pane = await session(browser, lang(code), `fixtures-${code}`);
  try {
    for (const path of repoFixtures) {
      const name = path.split("/").pop()!;
      const opened = await openFile(pane, path);
      await settle(pane, 2500);
      const title = `Saved ${name.slice(0, 6)}`;
      let saved: Awaited<ReturnType<Pane["save"]>> | undefined;
      let checked: ReturnType<typeof compareOracle> | undefined;
      if (OPENED.test(opened.notice)) {
        await pane.setTitle(code, title);
        saved = await pane.save(`${code}-${name}`);
        checked = compareOracle(path, saved.path, title);
      }
      verdicts.push({ step: `${code}: ${name} opens, saves and passes the oracle`, pass: OPENED.test(opened.notice) && checked?.ok === true, evidence: { opened, bytes: saved?.bytes.byteLength, output: checked?.output, flakes } });
    }
  } finally { await pane.context.close(); }
}

const encryptions = [{ alg: "aes128", v: 4, r: 4 }, { alg: "aes256", v: 5, r: 6 }, { alg: "rc4-128", v: 2, r: 3 }, { alg: "rc4-40", v: 1, r: 2 }];

async function encryption(browser: Browser, code: "en" | "de") {
  const pane = await session(browser, lang(code), `encrypt-${code}`);
  try {
    const opened = await openFile(pane, join(fx, "doc.pdf"));
    verdicts.push({ step: `${code}: open doc.pdf for encryption`, pass: OPENED.test(opened.notice), evidence: opened });
    await settle(pane, 3000);
    const plain: Checks = { pages: 3, text: baseline.text, encrypt: null };
    for (const [index, item] of encryptions.entries()) {
      const result = await pane.act("set-encryption", { object: "owner", text: item.alg, extra: "user" });
      await settle(pane);
      const applied = await saved(pane, code, `${code}-encrypt-${item.alg}`, { pages: 3, text: baseline.text, encrypt: { v: item.v, r: item.r } }, "user");
      await pane.press("undo"); await settle(pane);
      const undone = await saved(pane, code, `${code}-encrypt-${item.alg}-undone`, plain);
      if (index === 0) {
        await pane.press("redo"); await settle(pane);
        await saved(pane, code, `${code}-encrypt-${item.alg}-redone`, { pages: 3, text: baseline.text, encrypt: { v: item.v, r: item.r } }, "user");
        await pane.act("set-encryption", { object: "", text: "", extra: "" });
        await settle(pane);
        await saved(pane, code, `${code}-encrypt-removed`, plain);
        await pane.press("undo"); await settle(pane);
        await pane.press("undo"); await settle(pane);
      }
      verdicts.push({ step: `${code}: set-encryption ${item.alg} applied, undone`, pass: applied.ok && undone.ok && !result.covered, evidence: { alerts: result.alerts.slice(0, 3) } });
    }
    const encrypted = join(out, `${code}-encrypt-aes256.pdf`);
    const again = await pane.open(encrypted);
    await settle(pane, 3000);
    verdicts.push({ step: `${code}: the editor opens a file it encrypted (outcome recorded)`, pass: /file-opened|file-open-failed/u.test(again.notice), evidence: { notice: again.notice, alerts: (await pane.alerts()).slice(0, 4) } });
  } finally { await pane.context.close(); }
}

const handlers: Record<string, (browser: Browser) => Promise<void>> = {};
for (const code of ["en", "de"] as const) { handlers[`docA-${code}`] = browser => docA(browser, code); handlers[`invalid-${code}`] = browser => invalid(browser, code); handlers[`inline-${code}`] = browser => inlineFields(browser, code); handlers[`transfer-${code}`] = browser => transfer(browser, code); handlers[`large-${code}`] = browser => large(browser, code); handlers[`fixtures-${code}`] = browser => fixtures(browser, code); handlers[`encrypt-${code}`] = browser => encryption(browser, code); }
await runSteps(["docA-en"], handlers);
for (const verdict of verdicts) console.log(verdict.pass ? "ok" : "FAILED", verdict.step, JSON.stringify(verdict.evidence).slice(0, process.env.WIDE ? 3000 : 600));
