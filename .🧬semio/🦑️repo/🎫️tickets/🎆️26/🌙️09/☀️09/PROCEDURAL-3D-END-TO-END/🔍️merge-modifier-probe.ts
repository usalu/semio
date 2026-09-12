#!/usr/bin/env bun
/** 🎯️ Runtime proof of the ONE selection merge vocabulary on the generation3d preview (a DOMAIN-BOUND
 * world scene): a plain click, then a shift-click, a ctrl-click and a shift+ctrl-click on the SAME
 * column. Before ticket 26/09/09/PROCEDURAL-3D-END-TO-END's merge-vocabulary wave the host translated
 * the resolved `MergeMode` into `add`/`remove`/`toggle`, which the guest's `parse_merge_mode` rejects
 * (`interactionSelect: unknown merge '…'`), so only the plain click worked and every modified click
 * left the selection untouched.
 *
 * 🔦️ The observable is the pane's own `data-selection-json` (what the pane PAINTS) plus every
 * `leftover InteractionView` console line — the merge word never crosses the in-realm bridge as
 * readable bytes, but the four modes are behaviourally distinguishable on ONE id: `replace` selects
 * it, `additive` keeps it (idempotent), `subtractive` CLEARS it, `invertive` brings it back. A
 * rejected merge cannot produce that sequence — it freezes the selection at the first pick.
 *
 * bun 🔍️merge-modifier-probe.ts [--url=…] [--settle=180] [--predelay=30] [--label=…] [--x=180] [--y=180]
 * Output: 🗑️generated/merge-vocabulary/merge-modifier-<label>-<stamp>/
 */
import { chromium, type ConsoleMessage, type Page } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const arg = (name: string): string | undefined => process.argv.slice(2).find((entry) => entry.startsWith(`--${name}=`))?.split("=").slice(1).join("=");
const url = arg("url") ?? "http://127.0.0.1:6018/?plugin=generation3d";
const settleSec = Number(arg("settle") ?? "180");
const preDelaySec = Number(arg("predelay") ?? "30");
const label = arg("label") ?? "merge-modifier";
const pickAt = { x: Number(arg("x") ?? "180"), y: Number(arg("y") ?? "180") };
const stamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, 19);
const outDir = join(import.meta.dir, "🗑️generated", "merge-vocabulary", `merge-modifier-${label}-${stamp}`);
mkdirSync(outDir, { recursive: true });

const t0 = Date.now();
const rows: { elapsedMs: number; level: string; text: string }[] = [];
const log = (message: string): void => console.log(`[${((Date.now() - t0) / 1000).toFixed(1)}s] ${message}`);

/** 🖱️ The four chords of `🕹️interaction/🧫️fixtures/🎯️merge-modes.json`'s `modifierPolicy`, played on ONE
 * id so each mode's effect on the painted selection is unambiguous. */
const GESTURES = [
  { id: "plain-click", modifiers: [] as ("Shift" | "Control")[], mode: "replace", selects: true },
  { id: "shift-click", modifiers: ["Shift"] as ("Shift" | "Control")[], mode: "additive", selects: true },
  { id: "control-click", modifiers: ["Control"] as ("Shift" | "Control")[], mode: "subtractive", selects: false },
  { id: "shift-control-click", modifiers: ["Shift", "Control"] as ("Shift" | "Control")[], mode: "invertive", selects: true },
];

/** 🧮️ Extracts every `leftover InteractionView {…}` payload from one console line by brace matching —
 * the payload nests (`locked`, `hoverTarget`), so a non-greedy match truncates it. */
const interactionViews = (text: string): string[] => {
  const found: string[] = [];
  const needle = "leftover InteractionView ";
  let at = text.indexOf(needle);
  while (at >= 0) {
    let depth = 0;
    let end = at + needle.length;
    for (; end < text.length; end += 1) {
      if (text[end] === "{") depth += 1;
      else if (text[end] === "}") {
        depth -= 1;
        if (depth === 0) break;
      }
    }
    found.push(text.slice(at + needle.length, end + 1));
    at = text.indexOf(needle, end + 1);
  }
  return found;
};

const previewCanvas = (page: Page) => page.locator(".semio-world-3d-host canvas, [data-meshes-json] canvas, #framework\\.window\\.proceduralPreview canvas").last();

/** 🔦️ The pane's own published state: how many meshes it holds, and the selection it PAINTS. */
const paneState = async (page: Page): Promise<{ meshes: number; instances: number; selection: unknown }> =>
  page.evaluate(() => {
    const parse = (raw: string | null): unknown => {
      if (!raw) return null;
      try {
        return JSON.parse(raw);
      } catch {
        return raw.slice(0, 200);
      }
    };
    const hosts = [...document.querySelectorAll<HTMLElement>(".semio-world-3d-host, [data-meshes-json]")];
    const len = (raw: string | null): number => {
      const parsed = parse(raw);
      return Array.isArray(parsed) ? parsed.length : 0;
    };
    return {
      meshes: hosts.reduce((total, host) => total + len(host.getAttribute("data-meshes-json")), 0),
      instances: hosts.reduce((total, host) => total + len(host.getAttribute("data-instances-json")), 0),
      selection: hosts.map((host) => parse(host.getAttribute("data-selection-json"))).find((entry) => entry !== null) ?? null,
    };
  });

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("console", (message: ConsoleMessage) => rows.push({ elapsedMs: Date.now() - t0, level: message.type(), text: message.text() }));
page.on("pageerror", (error) => rows.push({ elapsedMs: Date.now() - t0, level: "pageerror", text: String(error?.stack ?? error) }));

await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120_000 });
log(`navigated title="${await page.title()}"`);

const deadline = Date.now() + settleSec * 1000;
let pane = await paneState(page);
while (pane.meshes === 0 && Date.now() < deadline) {
  await page.waitForTimeout(2000);
  pane = await paneState(page);
}
log(`meshes ready: meshes=${pane.meshes} instances=${pane.instances}`);
if (preDelaySec > 0) {
  log(`predelay ${preDelaySec}s before the first gesture`);
  await page.waitForTimeout(preDelaySec * 1000);
}
pane = await paneState(page);
log(`before gestures: meshes=${pane.meshes} instances=${pane.instances} selection=${JSON.stringify(pane.selection)}`);

const canvas = previewCanvas(page);
if ((await canvas.count()) === 0) throw new Error("[DEBUG] no preview canvas");
log(`preview canvas found; meshes=${pane.meshes}`);
const box = await canvas.boundingBox();
if (!box) throw new Error("[DEBUG] preview canvas has no box");
/** 🖱️ Raw mouse events, not locator clicks: a portal overlay covers the canvas for Playwright's
 * actionability check, while the real pointer events still reach r3f. */
const point = { x: box.x + Math.min(pickAt.x, box.width - 4), y: box.y + Math.min(pickAt.y, box.height - 4) };
await page.mouse.move(point.x, point.y);
await page.waitForTimeout(1500);

const firstGestureRow = rows.length;
const perGesture: { id: string; mode: string; expectsSelection: boolean; selection: unknown; views: string[]; unknownMergeFaults: string[]; selectDispatches: number; selectSettles: number }[] = [];
for (const gesture of GESTURES) {
  const from = rows.length;
  log(`gesture ${gesture.id} (${gesture.mode}): start`);
  /** 🖱️ A portal overlay sits above the preview canvas, so the real pointer never reaches r3f under
   * automation. The gesture is therefore dispatched ON the canvas element itself — the same
   * bubbling `pointerdown`/`pointerup`/`click` sequence a user's pointer produces, carrying the
   * modifier flags the host's `resolveWorldMergeMode` reads. */
  await canvas.evaluate(
    (element, gesture) => {
      const rect = (element as HTMLElement).getBoundingClientRect();
      const init: PointerEventInit = {
        bubbles: true,
        cancelable: true,
        composed: true,
        pointerId: 1,
        pointerType: "mouse",
        button: 0,
        buttons: 1,
        isPrimary: true,
        clientX: rect.left + gesture.x,
        clientY: rect.top + gesture.y,
        shiftKey: gesture.shift,
        ctrlKey: gesture.control,
        metaKey: false,
      };
      element.dispatchEvent(new PointerEvent("pointermove", init));
      element.dispatchEvent(new PointerEvent("pointerdown", init));
      element.dispatchEvent(new PointerEvent("pointerup", { ...init, buttons: 0 }));
      element.dispatchEvent(new MouseEvent("click", { ...init, buttons: 0 } as MouseEventInit));
    },
    { x: pickAt.x, y: pickAt.y, shift: gesture.modifiers.includes("Shift"), control: gesture.modifiers.includes("Control") },
  );
  await page.waitForTimeout(6000);
  const fresh = rows.slice(from);
  const state = await paneState(page);
  const views = fresh.flatMap((row) => interactionViews(row.text));
  const unknownMergeFaults = fresh.filter((row) => row.text.includes("unknown merge")).map((row) => row.text.slice(0, 400));
  /** 🧯 A merge word the guest rejects never SETTLES: `parse_merge_mode` faults the reserved job, so the
   * invocation is dispatched and then dropped. dispatched-vs-settled is therefore the discriminator
   * that survives a preview with no evaluated geometry (where every click is a background clear). */
  const count = (needle: string): number => fresh.reduce((total, row) => total + row.text.split(needle).length - 1, 0);
  const dispatchedSelects = count('[DEBUG] performInvocation {"invocationKind":"action","instanceId":1,"actionId":"interactionSelect"}');
  const settledSelects = count('[DEBUG] performInvocation settled {"invocationKind":"action","instanceId":1,"actionId":"interactionSelect"');
  const selects = dispatchedSelects;
  perGesture.push({ id: gesture.id, mode: gesture.mode, expectsSelection: gesture.selects, selection: state.selection, views, unknownMergeFaults, selectDispatches: selects, selectSettles: settledSelects });
  log(`gesture ${gesture.id} (${gesture.mode}): dispatched=${dispatchedSelects} settled=${settledSelects} views=${views.length} unknownMergeFaults=${unknownMergeFaults.length} selection=${JSON.stringify((state.selection as { selectedIds?: string[] } | null)?.selectedIds ?? null)}`);
  for (const view of views.slice(-1)) log(`  InteractionView ${view}`);
}

/** ⏳️ Settles lag their dispatch by seconds, so the totals are only honest after a drain window. */
const gestureWindowFrom = firstGestureRow;
await page.waitForTimeout(15_000);
const tail = rows.slice(gestureWindowFrom);
const totalCount = (needle: string): number => tail.reduce((total, row) => total + row.text.split(needle).length - 1, 0);
const totalDispatched = totalCount('[DEBUG] performInvocation {"invocationKind":"action","instanceId":1,"actionId":"interactionSelect"}');
const totalSettled = totalCount('[DEBUG] performInvocation settled {"invocationKind":"action","instanceId":1,"actionId":"interactionSelect"');
const totalUnknownMerge = tail.filter((row) => row.text.includes("unknown merge")).length;
log(`after a 15s drain: interactionSelect dispatched=${totalDispatched} settled=${totalSettled} unknownMergeFaults=${totalUnknownMerge}`);

writeFileSync(join(outDir, "console.jsonl"), rows.map((row) => JSON.stringify(row)).join("\n"), "utf8");
writeFileSync(join(outDir, "gestures.json"), JSON.stringify(perGesture, null, 2), "utf8");
await page.screenshot({ path: join(outDir, "final.png"), fullPage: false });
await browser.close();

const ids = (entry: (typeof perGesture)[number]): string[] => {
  const selection = entry.selection as { ids?: string[] } | null;
  return selection?.ids ?? [];
};
const faults = totalUnknownMerge;
// 🧯 The painted-selection law only means something when the preview actually evaluated geometry —
// with `meshes=0` every click is a BACKGROUND click, which still carries the resolved merge word and
// so still proves the vocabulary (a rejected word faults there too), but proves no set algebra.
const wrong = pane.meshes === 0 ? [] : perGesture.filter((entry) => (ids(entry).length > 0) !== entry.expectsSelection);
if (pane.meshes === 0) console.log("NOTE: the preview evaluated NO geometry in this run — every gesture was a background click, so only the vocabulary (no `unknown merge` fault) is proved, not the set algebra.");
console.log(`\nwrote ${outDir}`);
const dispatched = totalDispatched;
const settled = totalSettled;
console.log(`DIAGNOSIS: interactionSelect dispatched=${dispatched} settled=${settled} (a rejected merge word never settles); unknownMergeFaults=${faults}; gestures whose painted selection contradicts their mode: ${wrong.length === 0 ? "none" : wrong.map((entry) => `${entry.id}/${entry.mode} -> ${JSON.stringify(ids(entry))}`).join(", ")}`);
process.exit(faults === 0 && wrong.length === 0 && dispatched > 0 && settled === dispatched ? 0 : 1);
