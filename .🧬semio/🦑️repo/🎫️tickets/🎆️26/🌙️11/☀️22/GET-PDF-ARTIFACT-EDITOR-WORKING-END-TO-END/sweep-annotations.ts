import { runSteps, session, verdicts } from "./journey-kit.ts";
import type { Browser } from "playwright";
import { join } from "node:path";
const doc = join(import.meta.dir, "🗑️generated/fixtures/ann-doc.pdf");
const oracle = (...args: string[]) => { const run = Bun.spawnSync(["uv", "run", "--offline", "--with", "pymupdf==1.26.7", "--with", "pypdf", "python", "-I", join(import.meta.dir, "oracle-annotations.py"), ...args], { stdout: "pipe", stderr: "pipe" }); return { ok: run.exitCode === 0, output: `${run.stdout.toString()}${run.stderr.toString()}`.slice(-1500) }; };
type Pane = Awaited<ReturnType<typeof session>>;
const rows = async (pane: Pane) => (await pane.text()).split("\n").filter(l => /^(Annotation|Text|Page \d)/.test(l.trim()));
const annRows = async (pane: Pane) => (await rows(pane)).filter(l => l.startsWith("Annotation")).length;
const wait = (pane: Pane, ms = 1500) => pane.page.waitForTimeout(ms);
const refusal = async (pane: Pane, id: string, args: Record<string, string>) => {
  const cursor = (await pane.notices()).length;
  const r = await pane.act(id, args);
  await wait(pane, 1500);
  const alerts = await pane.page.evaluate(() => [...document.querySelectorAll('[data-semio-transient-notice]')].map(e => `${e.getAttribute("role")}|${e.getAttribute("data-notice-code")}|${(e.firstChild?.textContent ?? "").trim()}`));
  return { alerts, notices: (await pane.notices()).slice(cursor), covered: r.covered };
};
async function create(browser: Browser) {
  const pane = await session(browser, "en-US", "create");
  try {
    await pane.open(doc);
    await wait(pane, 2000);
    const before = await annRows(pane);
    const specs: [string, Record<string, string>][] = [
      ["note", { page: "1", object: "note", text: "Rail note", x: "100", y: "600", width: "20", height: "20", red: "0", green: "0", blue: "0" }],
      ["highlight", { page: "1", object: "highlight", text: "Rail hl", x: "72", y: "650", width: "200", height: "14" }],
      ["underline", { page: "1", object: "underline", text: "Rail ul", x: "72", y: "630", width: "200", height: "14" }],
      ["strikeOut", { page: "1", object: "strikeOut", text: "Rail so", x: "72", y: "610", width: "200", height: "14" }],
      ["squiggly", { page: "1", object: "squiggly", text: "Rail sq", x: "72", y: "590", width: "200", height: "14" }],
      ["ink", { page: "1", object: "ink", extra: "300,300,350,350;350,350,400,300", width: "3" }],
      ["linkUri", { page: "1", object: "link", text: "https://example.test/rail", x: "300", y: "100", width: "100", height: "20" }],
      ["linkPage", { page: "1", object: "link", text: "page:2", x: "300", y: "140", width: "100", height: "20" }],
    ];
    const log: Record<string, unknown>[] = [];
    let count = before;
    for (const [name, args] of specs) {
      const r = await pane.act("create-annotation", args);
      await wait(pane, 1200);
      const now = await annRows(pane);
      log.push({ name, grew: now - count, covered: r.covered });
      count = now;
    }
    await pane.shot("create-1-all");
    verdicts.push({ step: "create-annotation rail: all 8 kinds add a text-layer row", pass: log.every(l => l.grew === 1 && !l.covered), evidence: { before, log } });
    const undo = await pane.press("undo"); await wait(pane); const afterUndo = await annRows(pane);
    const redo = await pane.press("redo"); await wait(pane); const afterRedo = await annRows(pane);
    verdicts.push({ step: "create-annotation: undo removes, redo restores", pass: afterUndo === count - 1 && afterRedo === count, evidence: { count, afterUndo, afterRedo } });
    const bad = await refusal(pane, "create-annotation", { page: "1", object: "bogus", text: "x", x: "1", y: "1", width: "5", height: "5" });
    verdicts.push({ step: "create-annotation: invalid kind refused with alert", pass: bad.alerts.length > 0, evidence: bad });
    const saved = await pane.save("create.pdf");
    const o = oracle("create", saved.path);
    verdicts.push({ step: "create-annotation: saved file in pymupdf/pypdf", pass: o.ok, evidence: { oracle: o.output } });
    const again = await pane.open(saved.path); await wait(pane, 2500);
    verdicts.push({ step: "create-annotation: reopen shows annotations", pass: again.notice.includes("file-opened") && (await annRows(pane)) >= count, evidence: { notice: again.notice, rows: await annRows(pane) } });
  } finally { await pane.context.close(); }
}
await runSteps(["create"], { create });
