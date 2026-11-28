/** 🧰 Page helpers of the v3 matrix: boot gate, panels, tree selection, canvas gestures, field edits, palette actions, pixel probes. */
import { createHash } from "node:crypto";
import { mkdirSync } from "node:fs";
import { VIEWPORT } from "./apps.mjs";

const SIDE = ["Results", "History", "Settings", "Command", "Display", "Marketplace", "Tasks", "Chat"];
export class BootHang extends Error {}
export const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
export const esc = s => String(s).replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
export const words = id => id.replace(/([A-Z])/g, " $1").replace(/^./, c => c.toUpperCase()).replace(/\bUdl\b/, "UDL").trim();
export const same = (a, b) => {
  const x = Number(a), y = Number(b);
  if (String(a).trim() !== "" && String(b).trim() !== "" && Number.isFinite(x) && Number.isFinite(y)) return Math.abs(x - y) <= 1e-6 * Math.max(1, Math.abs(y));
  return String(a).trim().toLowerCase() === String(b).trim().toLowerCase();
};

export class Ctx {
  constructor(app, browser, dirs) { this.app = app; this.browser = browser; this.dirs = dirs; this.logs = []; this.t0 = Date.now(); this.dialogs = []; mkdirSync(dirs.shots, { recursive: true }); }
  async init() {
    await this.context?.close().catch(() => {});
    this.context = await this.browser.newContext({ locale: "en-US", viewport: VIEWPORT, acceptDownloads: true });
    const page = this.page = await this.context.newPage();
    page.on("console", m => this.logs.push({ t: Date.now(), type: m.type(), text: m.text().slice(0, 600) }));
    page.on("pageerror", e => this.logs.push({ t: Date.now(), type: "pageerror", text: String(e.message ?? e).slice(0, 600) }));
    page.on("dialog", d => { this.dialogs.push(d.message()); d.dismiss().catch(() => {}); });
  }
  mark() { return this.logs.length; }
  consoleSince(mark) {
    const rows = this.logs.slice(mark);
    const noise = /\[DEBUG\]|React DevTools|\[vite\]|follow unit|semio dev ·/;
    const errors = rows.filter(r => /error|pageerror/.test(r.type) && !noise.test(r.text));
    const fatal = errors.filter(r => /panic|wasm trap|unreachable|RuntimeError|pageerror|stale|native-return-refused|cleanup faulted|Unsupported/i.test(r.text) || r.type === "pageerror");
    return { errors: errors.length, fatal: fatal.length, noise: rows.length - errors.length, first: errors.slice(0, 4).map(r => r.text.slice(0, 220)), fatalFirst: fatal.slice(0, 3).map(r => r.text.slice(0, 220)) };
  }
  async text() { return this.page.locator("body").innerText().catch(() => ""); }
  async aria() { return this.page.locator("body").ariaSnapshot().catch(() => ""); }
  async until(fn, ms = 30000, step = 400) {
    const t = Date.now();
    while (Date.now() - t < ms) { try { const r = await fn(); if (r) return Date.now() - t; } catch {} await sleep(step); }
    return -1;
  }
  btn(name, exact = true, scope = this.page) { return scope.getByRole("button", { name, exact }); }
  async isOn(loc) { try { return await loc.isChecked(); } catch { return (await loc.getAttribute("aria-checked")) === "true"; } }
  async tap(loc, timeout = 2500) { try { await loc.click({ timeout }); } catch { await loc.dispatchEvent("click"); } }
  async visible(loc) { return (await loc.count().catch(() => 0)) > 0 && await loc.first().isVisible().catch(() => false); }

  async bootState() {
    const t = await this.text();
    const loading = /Loading plugins/.test(t);
    if (this.app.kind === "viewer") {
      const view = await this.page.locator(`[aria-label="${this.app.viewLabel}"]`).count();
      return { ready: !loading && view > 0, loading, detail: `loading=${loading} view=${view} len=${t.length}` };
    }
    const nodes = Number((/Nodes \((\d+)\)/.exec(t) ?? /Nodes\s+(\d+)/.exec(t) ?? [])[1] ?? (/\S · .*\bNode\b/.test(t) ? 1 : 0));
    const caption = await this.caption();
    const solved = this.legacyCaption || (this.app.bootCaption.test(caption) && !this.app.pending.test(caption));
    const view = await this.page.locator(`[aria-label="${this.app.viewLabel}"]`).count();
    return { ready: nodes > 0 && solved && view > 0, loading, detail: `loading=${loading} modelView=${view} nodes=${nodes} caption=${JSON.stringify(caption)} len=${t.length}` };
  }

  async calibrate() {
    await this.canvasLayout(); await this.page.waitForTimeout(1800);
    const clip = { ...this.app.modelClip, y: 100, height: 660 }, png = (await this.screenshotBytes(clip)).toString("base64"), dim = this.app.dim;
    const f = await this.page.evaluate(async ({ png, dim }) => {
      const bmp = await createImageBitmap(await (await fetch(`data:image/png;base64,${png}`)).blob());
      const cv = new OffscreenCanvas(bmp.width, bmp.height), cx = cv.getContext("2d"); cx.drawImage(bmp, 0, 0);
      const { data, width, height } = cx.getImageData(0, 0, bmp.width, bmp.height);
      const px = (x, y) => { const i = (y * width + x) * 4; return [data[i], data[i + 1], data[i + 2]]; };
      const line = dim === 2 ? ([r, g, b]) => Math.abs(r - 148) < 14 && Math.abs(g - 163) < 14 && Math.abs(b - 184) < 14 : ([r, g, b]) => r > 95 && r < 175 && g > 105 && g < 180 && b > 90 && b < 170 && g >= r - 8 && Math.max(r, g, b) - Math.min(r, g, b) < 45;
      const blue = ([r, g, b]) => b > 195 && r < 150 && g > 140 && g < 215;
      const mask = (fn) => { const m = new Uint8Array(width * height); for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) if (fn(px(x, y))) m[y * width + x] = 1; return m; };
      const L = mask(line); let minx = width, maxx = 0, miny = height, maxy = 0, cnt = 0;
      for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) if (L[y * width + x]) { cnt++; if (x < minx) minx = x; if (x > maxx) maxx = x; if (y < miny) miny = y; if (y > maxy) maxy = y; }
      const out = { bbox: [minx, miny, maxx, maxy], lineCount: cnt };
      if (dim === 3) {
        const runs = [];
        for (let x = 2; x < width - 2; x++) { let best = 0, bs = 0, run = 0, gap = 0, st = 0; for (let y = 0; y < height; y++) { const on = L[y * width + x] || L[y * width + x - 1] || L[y * width + x + 1]; if (on) { if (!run) st = y; run++; gap = 0; if (run > best) { best = run; bs = st; } } else if (run && ++gap <= 2) { run++; } else { run = 0; gap = 0; } } if (best >= 50) runs.push({ x, len: best, y: bs + best / 2 }); }
        runs.sort((a, b) => b.len - a.len); const picked = []; for (const r of runs) if (picked.every(p => Math.abs(p.x - r.x) > 45)) picked.push(r);
        out.points = picked.slice(0, 3).map(r => [r.x, Math.round(r.y)]);
      } else {
        const B = mask(blue), seen = new Uint8Array(width * height), comps = [];
        for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) if (B[y * width + x] && !seen[y * width + x]) { const q = [[x, y]]; seen[y * width + x] = 1; let sx = 0, sy = 0, n = 0; while (q.length) { const [cx2, cy2] = q.pop(); sx += cx2; sy += cy2; n++; for (const [dx, dy] of [[1, 0], [-1, 0], [0, 1], [0, -1], [1, 1], [-1, -1], [1, -1], [-1, 1]]) { const nx = cx2 + dx, ny = cy2 + dy; if (nx >= 0 && ny >= 0 && nx < width && ny < height && B[ny * width + nx] && !seen[ny * width + nx]) { seen[ny * width + nx] = 1; q.push([nx, ny]); } } } if (n >= 3) comps.push({ x: sx / n, y: sy / n, n }); }
        const merged = []; for (const c of comps) { const m = merged.find(o => Math.hypot(o.x - c.x, o.y - c.y) < 9); if (m) { m.x = (m.x * m.n + c.x * c.n) / (m.n + c.n); m.y = (m.y * m.n + c.y * c.n) / (m.n + c.n); m.n += c.n; } else merged.push({ ...c }); }
        comps.length = 0; comps.push(...merged.filter(c => c.n >= 20));
        comps.sort((a, b) => a.y - b.y); out.comps = comps.length;
        if (comps.length >= 2) { const y1 = comps[1].y; const row = comps.filter(c => Math.abs(c.y - y1) < 12).sort((a, b) => a.x - b.x); out.points = [[comps[0].x, comps[0].y], [row[0].x, row[0].y], [row[row.length - 1].x, row[row.length - 1].y]].map(p => p.map(Math.round)); } else out.points = [];
      }
      return out;
    }, { png, dim });
    const a = this.app, P = (p, k) => [clip.x + p[0], clip.y + p[1]];
    if (!f.points?.length) { this.calibration = { ok: false, ...f }; return this.calibration; }
    const pts = f.points.map(P); const A = pts[0], B = pts[1] ?? pts[0], C = pts[2] ?? pts[1] ?? pts[0];
    a.nodeA = { re: null, at: A.map(Math.round) }; a.nodeB = { re: null, at: B.map(Math.round) }; a.nodeC = { re: null, at: C.map(Math.round) };
    const [x0, y0, x1, y1] = f.bbox; const bx0 = clip.x + x0 - 8, by0 = clip.y + y0 - 8, bx1 = clip.x + x1 + 8, by1 = clip.y + y1 + 8;
    a.marquee = [Math.max(bx0, clip.x + 5), Math.max(by0, clip.y + 40), Math.min(bx1, clip.x + clip.width - 5), Math.min(by1, clip.y + clip.height - 20)];
    const [m0, m1, m2, m3] = a.marquee; a.lasso = [[m0, m1], [m2, m1], [m2, m3], [m0, m3], [m0, m1]];
    a.empty = [clip.x + 300, clip.y + (a.dim === 3 ? 120 : 110)]; a.wheel = a.nodeA.at;
    this.calibration = { ok: true, A: a.nodeA.at, B: a.nodeB.at, C: a.nodeC.at, marquee: a.marquee, bbox: f.bbox, lineCount: f.lineCount };
    return this.calibration;
  }
  async healthy() {
    const t = await this.text();
    if (/Loading plugins/.test(t)) return false;
    return (await this.page.locator(`[aria-label="${this.app.viewLabel}"]`).count()) > 0;
  }
  async boot(maxMs = 180000, url = this.app.url) {
    const t = Date.now(); let last = { detail: "no state" };
    await this.page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 }).catch(e => { last = { detail: `goto failed ${e.message.slice(0, 120)}` }; });
    while (Date.now() - t < maxMs) {
      await this.page.waitForTimeout(3000);
      if (this.app.kind === "editor") await this.openWorkbench().catch(() => {});
      last = await this.bootState().catch(e => ({ ready: false, detail: String(e).slice(0, 100) }));
      if (last.ready) { await this.page.waitForTimeout(1500); if (this.app.kind === "editor") await this.calibrate().catch(e => { this.calibration = { ok: false, err: String(e).slice(0, 120) }; }); return Date.now() - t; }
    }
    const err = new BootHang(`boot-hang after ${Math.round((Date.now() - t) / 1000)} s: ${last.detail}`);
    err.detail = last.detail; throw err;
  }
  async reload(maxMs = 180000) {
    await this.init(); this.treeHasMember = undefined;
    return this.boot(maxMs);
  }

  async isPressed(name) {
    const b = this.btn(name).first();
    if (!(await b.count())) return null;
    const v = await b.getAttribute("aria-pressed"); const e = await b.getAttribute("aria-expanded");
    return v === "true" || (v === null && e === "true");
  }
  async settled(name, want, ms = 8000) { return this.until(async () => (await this.isPressed(name)) === want, ms, 250); }
  async panel(name, want = true) {
    if (want && SIDE.includes(name)) for (const o of SIDE) if (o !== name && (await this.isPressed(o)) === true) { await this.btn(o).first().click({ timeout: 8000 }); await this.settled(o, false); }
    const st = await this.isPressed(name);
    if (st === null) return false;
    if (st !== want) { await this.btn(name).first().click({ timeout: 8000 }); await this.settled(name, want); await this.page.waitForTimeout(600); }
    return true;
  }
  async openWorkbench() {
    for (const o of SIDE) if ((await this.isPressed(o)) === true) { await this.btn(o).first().click({ timeout: 8000 }); await this.page.waitForTimeout(600); }
    await this.panel("Artifact", true); await this.panel("Inspection", true);
  }
  artTree() { return this.page.getByRole("tree").first(); }
  inspTree() { return this.page.locator('[id="framework.panelTab.framework.panel.inspection"]').getByRole("tree").last(); }
  async sideOpen() { for (const o of SIDE) if ((await this.isPressed(o)) === true) return true; return false; }
  async ensureInsp() { if (this.app.kind === "editor" && ((await this.sideOpen()) || (await this.isPressed("Inspection")) !== true)) await this.inspOnly(); }
  async inspOnly() { for (const o of SIDE) if ((await this.isPressed(o)) === true) { await this.btn(o).first().click({ timeout: 8000 }); await this.settled(o, false); } await this.panel("Inspection", true); }
  async optionsLayout() {
    await this.ensureInsp();
    if ((await this.isPressed("Artifact")) === true) { await this.btn("Artifact").first().click({ timeout: 8000 }); await this.page.waitForTimeout(2500); }
    this.freeLayout = true;
  }
  async canvasLayout() {
    if (this.app.kind !== "editor" || this.freeLayout) return;
    await this.ensureInsp();
    if ((await this.isPressed("Artifact")) !== true) { await this.btn("Artifact").first().click({ timeout: 8000 }); await this.page.waitForTimeout(1500); }
    if (await this.paletteOpen().catch(() => false)) { await this.tap(this.modelPanel().getByRole("button", { name: /^Actions/ }).first()); await this.page.waitForTimeout(1200); }
  }
  async insp() { await this.ensureInsp(); return (await this.inspTree().innerText({ timeout: 4000 }).catch(() => "")).replace(/\n+/g, " | "); }
  async selIds() {
    const t = await this.insp(); const head = t.split(/\|\s*(?:NODE|ELEMENT|SOLID|SUPPORT|LOAD|REGION|COMBINATION|MATERIAL|SECTION)\s*\|/)[0];
    const many = /\d+\s+selected/i.test(t);
    if (!many) { const m = /\bID\s+(\S+)/.exec(t); return m ? [m[1]] : []; }
    return [...head.matchAll(/\|\s*(?:Node|Element|Solid|Support|Load Case|Load|Combination|Material|Section|Region):?\s+([A-Za-z0-9_.\-]+)/g)].map(m => m[1]);
  }
  async selection() {
    const t = await this.insp();
    const many = /(\d+)\s+selected/i.exec(t), id = /\bID\s+(\S+)/.exec(t);
    const summary = /Schema/i.test(t) && !many && !id;
    return { summary, n: many ? Number(many[1]) : id ? 1 : summary ? 0 : t.trim() ? 1 : 0, id: id?.[1] ?? null, text: t.slice(0, 140) };
  }
  async summaryCounts() {
    const t = await this.insp(); const o = {};
    for (const m of t.matchAll(/(Nodes|Elements|Solids|Regions|Supports|Load Cases|Combinations|Materials|Sections)\s+(\d+)/g)) o[m[1]] = Number(m[2]);
    return o;
  }
  async toSummary() {
    await this.ensureInsp();
    if ((await this.selection()).summary) return true;
    const [x, y] = this.app.empty; await this.hoverClick(x, y, { hover: 1800 });
    return (await this.until(async () => (await this.selection()).summary, 30000)) >= 0;
  }
  async counts() {
    if (!(await this.toSummary())) return null;
    let k = {}; await this.until(async () => { k = await this.summaryCounts(); return Object.keys(k).length >= 3; }, 20000, 500);
    return Object.keys(k).length ? k : null;
  }
  async nodePositions() {
    return this.page.evaluate(() => { const o = {}; for (const e of document.querySelectorAll("[role=treeitem],[role=button]")) { const m = /^(\S+)\s·\s[(\[]([^)\]]+)[)\]]\s*Node/.exec((e.innerText ?? "").replace(/−/g, "-").replace(/\s+/g, " ").trim()); if (m) o[m[1]] = m[2].split(",").map(Number); } return o; });
  }
  async treeHeadCounts() {
    return this.page.evaluate(() => { const o = {}; for (const e of document.querySelectorAll("[role=tree] *")) { const m = /^([A-Za-z ]+?)\s*[(\[](\d+)[)\]]$/.exec((e.textContent ?? "").trim()); if (m) o[m[1].trim().toUpperCase()] = Number(m[2]); } return o; });
  }
  async pickPair(A, B) {
    await this.toSummary(); const r = await this.pickAt(A, { re: null }); await this.hoverClick(...B.at, { mod: "Shift" });
    const ms = await this.until(async () => (await this.selIds()).length >= 2, 12000, 300);
    return { ok: r.ok && ms >= 0, ids: await this.selIds() };
  }
  async workbench() { await this.openWorkbench(); }

  async scrollTree(d) { const t = this.artTree(); const b = await t.boundingBox().catch(() => null); await this.page.mouse.move(b ? b.x + 60 : 150, b ? b.y + Math.min(b.height / 2, 400) : 500); await this.page.mouse.wheel(0, d); await this.page.waitForTimeout(600); }
  rowLoc(re) { const t = this.artTree(); return t.getByRole("button", { name: re }).or(t.getByRole("treeitem", { name: re })); }
  async findRow(re) {
    let loc = this.rowLoc(re);
    if (await loc.count()) return loc.first();
    for (const d of [-9000, 3000, 3000, 3000, 3000]) { await this.scrollTree(d); loc = this.rowLoc(re); if (await loc.count()) return loc.first(); }
    return null;
  }
  async expandGroup(name) {
    const hdr = this.artTree().getByRole("button", { name: new RegExp(`^${esc(name)}( \\(\\d+\\))?$`) }).first();
    if (!(await hdr.count())) { await this.findRow(new RegExp(`^${esc(name)}( \\(\\d+\\))?$`)); }
    if (!(await hdr.count())) return false;
    if ((await hdr.getAttribute("aria-expanded")) === "true") return true;
    await hdr.scrollIntoViewIfNeeded().catch(() => {}); await hdr.click({ timeout: 5000 }); await this.page.waitForTimeout(900);
    if ((await hdr.getAttribute("aria-expanded")) !== "true") { const b = await hdr.boundingBox(); if (b) { await this.page.mouse.click(9, b.y + b.height / 2); await this.page.waitForTimeout(900); } }
    return (await hdr.getAttribute("aria-expanded")) === "true";
  }
  async openCases() {
    const cases = this.artTree().getByRole("treeitem", { name: / Load Case$/ }); let n = 0;
    for (let i = 0, c = await cases.count(); i < c; i++) { const it = cases.nth(i); if ((await it.getAttribute("aria-expanded")) === "false") { await it.click({ timeout: 6000 }); await this.page.waitForTimeout(700); n++; } }
    return n > 0;
  }
  async selectRow(re, { group } = {}) {
    await this.workbench();
    if (group) await this.expandGroup(group);
    let row = await this.findRow(re);
    if (!row && group === "Load Cases" && (await this.openCases())) row = await this.findRow(re);
    if (!row) return { ok: false, why: `no tree row ${re}` };
    const before = await this.insp();
    await row.scrollIntoViewIfNeeded().catch(() => {});
    await row.click({ timeout: 8000 });
    const t0 = Date.now();
    const ms = await this.until(async () => { const s = await this.selection(); if (s.summary) return false; return (await this.insp()) !== before || Date.now() - t0 > 4000; }, 20000, 300);
    await this.page.waitForTimeout(900);
    return { ok: ms >= 0, ms, sel: await this.selection() };
  }
  async entityIds(group) {
    await this.panel("Artifact", true); await this.expandGroup(group);
    const lines = (await this.artTree().innerText().catch(() => "")).split("\n").map(l => l.trim());
    const i = lines.findIndex(l => new RegExp(`^${esc(group.toUpperCase())} \\(\\d+\\)$`).test(l));
    const out = [];
    if (i >= 0) for (const l of lines.slice(i + 1)) { if (/^[A-Z][A-Z ]+( \(\d+\))?$/.test(l)) break; const m = /^(\S+) ·/.exec(l); if (m) out.push(m[1]); }
    return out;
  }
  async refId(group, n = 0) {
    const ids = await this.entityIds(group); if (ids[n]) return ids[n];
    const re = { Materials: / Material$/, Sections: / Section$/, "Load Cases": / Load Case$/ }[group]; if (!re) return undefined;
    await this.expandGroup(group);
    const texts = [...new Set((await this.rowLoc(re).allInnerTexts()).map(t => t.trim()).filter(Boolean))];
    if (!texts[n]) return undefined;
    const row = this.rowLoc(re).filter({ hasText: texts[n] }).first();
    await row.scrollIntoViewIfNeeded().catch(() => {}); await row.click({ timeout: 8000 }); const head = { Materials: /MATERIAL/, Sections: /SECTION/, "Load Cases": /LOAD CASE/ }[group]; await this.until(async () => head.test(await this.insp()), 30000, 400); await this.page.waitForTimeout(600);
    return (await this.selection()).id ?? undefined;
  }
  async caseIds() {
    await this.panel("Artifact", true); await this.expandGroup("Combinations");
    const t = await this.artTree().innerText().catch(() => "");
    return [...new Set([...t.matchAll(/^\s*[\d.]+ ([A-Za-z][\w-]*) Term$/gm)].map(m => m[1]))];
  }
  historyPanel() { return this.page.locator('[id="framework.panelTab.framework.panel.history"]'); }
  async groupCount(name) {
    if ((await this.isPressed("Artifact")) !== true) await this.panel("Artifact", true);
    const find = () => this.page.evaluate(n => { const b = [...document.querySelectorAll("[role=tree] button,[role=tree] [role=button]")].map(e => e.textContent.trim()).find(t => new RegExp(`^${n} \\((\\d+)\\)$`).test(t)); return b ? Number(/\((\d+)\)/.exec(b)[1]) : null; }, name);
    let r = await find(); if (r !== null) return r;
    for (const d of [-9000, 3000, 3000, 3000]) { await this.scrollTree(d); r = await find(); if (r !== null) return r; }
    return null;
  }

  async caption() {
    const t = await this.text();
    return (/(Case: [^\n]*|Mode \d+[^\n]*|Buckling mode[^\n]*|Converged[^\n]*|Pending[^\n]*|Solving[^\n]*|Diverged[^\n]*|Analysis failed[^\n]*|Preparing[^\n]*)/.exec(t) ?? [""])[0].trim();
  }
  async solved(max = 150000) { if (this.legacyCaption) { await sleep(15000); return 15000; } return this.until(async () => { const c = await this.caption(); return this.app.bootCaption.test(c) && !this.app.pending.test(c); }, max, 600); }
  async idle(max = 150000) { const ms = await this.solved(max); return ms >= 0; }

  async hover(x, y, ms = 1500) { await this.canvasLayout(); await this.page.mouse.move(x, y); await this.page.waitForTimeout(ms); }
  async hoverClick(x, y, { mod, button = "left", hover = 1500 } = {}) {
    await this.hover(x, y, hover);
    for (const m of [mod].flat().filter(Boolean)) await this.page.keyboard.down(m);
    await this.page.mouse.down({ button }); await this.page.mouse.up({ button });
    for (const m of [mod].flat().filter(Boolean).reverse()) await this.page.keyboard.up(m);
  }
  async drag(a, b, c, d, { n = 8, dt = 150, button = "left", mod, hover = 800 } = {}) {
    await this.hover(a, b, hover);
    for (const m of [mod].flat().filter(Boolean)) await this.page.keyboard.down(m);
    await this.page.mouse.down({ button }); await this.page.waitForTimeout(300);
    for (let i = 1; i <= n; i++) { await this.page.mouse.move(a + (c - a) * i / n, b + (d - b) * i / n); await this.page.waitForTimeout(dt); }
    await this.page.waitForTimeout(500); await this.page.mouse.up({ button });
    for (const m of [mod].flat().filter(Boolean).reverse()) await this.page.keyboard.up(m);
    await this.page.waitForTimeout(400);
  }
  async rawDrag(a, b, c, d, { n = 10, dt = 100 } = {}) {
    const m = this.page.mouse; await m.move(a, b); await this.page.waitForTimeout(400); await m.down(); await this.page.waitForTimeout(300);
    for (let i = 1; i <= n; i++) { await m.move(a + (c - a) * i / n, b + (d - b) * i / n); await this.page.waitForTimeout(dt); }
    await this.page.waitForTimeout(600); await m.up(); await this.page.waitForTimeout(400);
  }
  async lasso(pts) {
    await this.hover(pts[0][0], pts[0][1], 800); await this.page.mouse.down(); await this.page.waitForTimeout(400);
    for (const p of pts.slice(1)) { await this.page.mouse.move(p[0], p[1], { steps: 3 }); await this.page.waitForTimeout(200); }
    await this.page.waitForTimeout(800); await this.page.mouse.up();
  }
  async pickAt(spec, { mod, re = spec.re, differs = false } = {}) {
    const [x, y] = spec.at; const was = differs ? (await this.selIds()).join(",") : null;
    for (const [dx, dy] of [[0, 0], [3, 0], [-3, 0], [0, 3], [6, 6]]) {
      await this.hoverClick(x + dx, y + dy, { mod });
      const ms = await this.until(async () => { const s = await this.selection(); return !s.summary && (!re || (s.id ? re.test(s.id) : true)) && s.n >= 1 && (await this.selIds()).length >= 1 && (was === null || (await this.selIds()).join(",") !== was); }, 16000, 400);
      if (ms >= 0) { await this.page.waitForTimeout(600); return { ok: true, ms, at: [x + dx, y + dy] }; }
    }
    return { ok: false, at: [x, y] };
  }
  async util(name) {
    const b = this.btn(name).first();
    if (!(await b.isVisible().catch(() => false))) { await this.tap(this.btn(/^Utilities/, false).first()); await this.page.waitForTimeout(900); }
    if ((await b.getAttribute("aria-pressed").catch(() => null)) === "true") return;
    await this.tap(b); await this.until(async () => (await b.getAttribute("aria-pressed")) === "true", 6000, 200); await this.page.waitForTimeout(2500);
  }
  async utilsOpen() {
    if (!(await this.btn("Marquee Select").first().isVisible().catch(() => false)) && !(await this.btn("Transform").first().isVisible().catch(() => false))) { await this.tap(this.btn(/^Utilities/, false).first()); await this.page.waitForTimeout(900); }
  }
  async pressedTools() { return ((await this.aria()).match(/button "[A-Za-z ]+" \[pressed\]/g) ?? []).map(s => /"(.*)"/.exec(s)[1]); }
  async radio(name) { await this.utilsOpen(); const r = this.page.getByRole("radio", { name, exact: true }); await this.tap(r.first()); await this.page.waitForTimeout(500); }
  async radioChecked() { return this.page.evaluate(() => [...document.querySelectorAll("[role=radio]")].filter(r => r.getAttribute("aria-checked") === "true" || r.checked).map(r => r.getAttribute("aria-label") ?? r.textContent.trim())); }

  modelPanel() { return this.page.getByRole("tabpanel", { name: "Model" }); }
  resultsPanel() { return this.page.getByRole("tabpanel", { name: "Results" }); }
  tabs() { return this.page.evaluate(() => [...document.querySelectorAll("[role=tab]")].map(e => e.textContent.trim() + "@" + Math.round(e.getBoundingClientRect().x))); }
  winButtons(win) { return this.page.getByRole("tablist").filter({ has: this.page.getByRole("tab", { name: win }) }).first(); }

  async screenshotBytes(clip) { return this.page.screenshot(clip ? { clip } : {}); }
  async hash(clip = this.app.modelClip) { return createHash("md5").update(await this.screenshotBytes(clip)).digest("hex").slice(0, 10); }
  async stable(clip = this.app.modelClip, ms = 25000) { const t = Date.now(); let h = await this.hash(clip); while (Date.now() - t < ms) { await this.page.waitForTimeout(1300); const n = await this.hash(clip); if (n === h) return h; h = n; } return h; }
  async changed(h0, fn, ms = 15000, clip) { const t = Date.now(); while (Date.now() - t < ms) { if ((await this.hash(clip)) !== h0) return Date.now() - t; await this.page.waitForTimeout(300); } return -1; }
  async analyze(clip, rule) {
    const png = (await this.screenshotBytes(clip)).toString("base64");
    return this.page.evaluate(async ({ png, rule }) => {
      const bmp = await createImageBitmap(await (await fetch(`data:image/png;base64,${png}`)).blob());
      const cv = new OffscreenCanvas(bmp.width, bmp.height), cx = cv.getContext("2d"); cx.drawImage(bmp, 0, 0);
      const { data, width, height } = cx.getImageData(0, 0, bmp.width, bmp.height);
      const f = new Function("r", "g", "b", "a", `return (${rule});`);
      let n = 0, sx = 0, sy = 0, run = 0, best = 0; const colors = new Set();
      for (let y = 0; y < height; y++) { run = 0; for (let x = 0; x < width; x++) { const i = (y * width + x) * 4; const r = data[i], g = data[i + 1], b = data[i + 2]; colors.add((r >> 4) << 8 | (g >> 4) << 4 | (b >> 4)); if (f(r, g, b, data[i + 3])) { n++; sx += x; sy += y; run++; if (run > best) best = run; } else run = 0; } }
      return { n, cx: n ? sx / n : null, cy: n ? sy / n : null, run: best, colors: colors.size, width, height };
    }, { png, rule });
  }
  async diffShots(a64, b64) {
    return this.page.evaluate(async ({ a64, b64 }) => {
      const load = async p => { const bmp = await createImageBitmap(await (await fetch(`data:image/png;base64,${p}`)).blob()); const cv = new OffscreenCanvas(bmp.width, bmp.height), cx = cv.getContext("2d"); cx.drawImage(bmp, 0, 0); return cx.getImageData(0, 0, bmp.width, bmp.height); };
      const A = await load(a64), B = await load(b64); const pts = [];
      for (let y = 0; y < A.height; y++) for (let x = 0; x < A.width; x++) { const i = (y * A.width + x) * 4; if (Math.abs(A.data[i] - B.data[i]) + Math.abs(A.data[i + 1] - B.data[i + 1]) + Math.abs(A.data[i + 2] - B.data[i + 2]) > 90) pts.push([x, y]); }
      if (!pts.length) return { n: 0 };
      const cx = pts.reduce((s, p) => s + p[0], 0) / pts.length, cy = pts.reduce((s, p) => s + p[1], 0) / pts.length;
      const right = pts.reduce((m, p) => p[0] > m[0] ? p : m, pts[0]), bottom = pts.reduce((m, p) => p[1] > m[1] ? p : m, pts[0]), top = pts.reduce((m, p) => p[1] < m[1] ? p : m, pts[0]);
      const quad = pts.filter(p => p[0] > cx && p[1] < cy); const q = quad.length ? quad[Math.floor(quad.length / 2)] : pts[0];
      return { n: pts.length, cx, cy, right, bottom, top, quad: q };
    }, { a64, b64 });
  }
  async handleOf(option, clip = this.app.modelClip) {
    const box = this.page.getByRole("checkbox", { name: option, exact: true }).first();
    if (!(await box.count())) return null;
    const checked = await this.isOn(box);
    const snap = async () => (await this.screenshotBytes(clip)).toString("base64");
    if (!checked) { const h = await this.hash(clip); await this.tap(box); await this.changed(h, clip, 15000); }
    const hOn = await this.stable(clip); const on = await snap();
    await this.tap(box); const ms = await this.changed(hOn, clip, 15000); const hOff = await this.stable(clip); const off = await snap();
    await this.tap(box); await this.changed(hOff, clip, 15000); await this.stable(clip);
    if (!checked) { const h = await this.hash(clip); await this.tap(box); await this.changed(h, clip, 15000); }
    const d = ms >= 0 ? await this.diffShots(on, off) : { n: 0 };
    return d.n ? { ...d, clip } : null;
  }
  async soloOption(option, all) {
    for (const o of all) { const b = this.page.getByRole("checkbox", { name: o, exact: true }).first(); if (!(await b.count())) continue; const on = await this.isOn(b); if (on !== (o === option)) { await this.tap(b); await this.page.waitForTimeout(3000); } }
  }
  async drawn(clip = this.app.modelClip) { return (await this.analyze(clip, "false")).colors; }
  async shot(id) { const file = `${this.dirs.shots}/${id.replace(/[^\w.-]+/g, "_")}.png`; await this.page.screenshot({ path: file }).catch(() => {}); return file; }

  async field(label, keys = []) {
    const t = this.inspTree();
    for (const role of ["spinbutton", "textbox", "combobox", "switch", "checkbox", "slider", "button"]) {
      const l = t.getByRole(role, { name: label, exact: typeof label === "string" }).first();
      if (await l.count()) return { loc: l, role, label };
    }
    for (const k of keys) {
      const l = this.page.locator(`[id$="-inspection.${k}.input"],[id$="-inspection.${k}.select"],[id$="-inspection.${k}.toggle"]`).first();
      if (await l.count()) { const id = await l.getAttribute("id"); const role = await l.getAttribute("role") ?? (/\.select$/.test(id) ? "combobox" : /\.toggle$/.test(id) ? "switch" : "textbox"); const tag = await l.evaluate(e => e.tagName + ":" + (e.type ?? "")); return { loc: l, role: /number/.test(tag) ? "spinbutton" : role, label: k }; }
    }
    return null;
  }
  async readField(f) {
    const l = f.loc;
    if (["spinbutton", "textbox"].includes(f.role)) return (await l.inputValue().catch(async () => (await l.textContent()) ?? "")).trim();
    if (f.role === "slider") return (await l.inputValue().catch(async () => (await l.getAttribute("aria-valuenow")) ?? "")).trim();
    if (f.role === "combobox") return ((await l.textContent()) ?? "").trim();
    if (f.role === "button") return (await l.getAttribute("aria-pressed")) ?? "";
    return String(await l.isChecked().catch(async () => (await l.getAttribute("aria-checked")) ?? ""));
  }
  async options(loc) { await loc.click({ timeout: 6000 }); await this.page.waitForTimeout(600); const o = (await this.page.getByRole("option").allTextContents()).map(s => s.trim()).filter(Boolean); return o; }
  async importFile(path) {
    await this.openPalette();
    const item = this.modelPanel().getByRole("button", { name: /^Import Document/ }).first();
    if (!(await item.count())) return { ok: false, why: "no Import Document palette entry" };
    const chooser = this.page.waitForEvent("filechooser", { timeout: 15000 }).catch(() => null);
    await item.click(); const fc = await chooser;
    if (!fc) return { ok: false, why: "Import Document opened no file chooser" };
    await fc.setFiles(path); return { ok: true };
  }
  async chooseOption(loc, want, { other } = {}) {
    const o = await this.options(loc); let pick = want;
    if (typeof want === "number") pick = o[want];
    else if (other !== undefined) pick = o.find(x => x && x !== other && !/^no example$/i.test(x)) ?? o[0];
    else if (want instanceof RegExp) pick = o.find(x => want.test(x));
    if (!pick) { await this.page.keyboard.press("Escape"); return { ok: false, options: o }; }
    await this.page.getByRole("option", { name: pick, exact: true }).first().click({ timeout: 6000 });
    await this.page.waitForTimeout(500);
    return { ok: true, picked: pick, options: o };
  }
  async setField(f, v) {
    if (["spinbutton", "textbox"].includes(f.role)) { await f.loc.fill(String(v)); await f.loc.press("Enter"); return v; }
    if (f.role === "slider") { const tag = await f.loc.evaluate(e => e.tagName); if (tag === "INPUT") { await f.loc.fill(String(v)); await f.loc.press("Enter").catch(() => {}); } else { const cur = Number(await this.readField(f)); await f.loc.focus(); await f.loc.press(Number(v) <= cur ? "Home" : "End"); } return v; }
    if (f.role === "combobox") { const r = await this.chooseOption(f.loc, typeof v === "string" && v ? new RegExp(esc(v)) : undefined, v === undefined ? { other: await this.readField(f) } : {}); return r.picked; }
    const cur = (await this.readField(f)) === "true"; if (v === undefined || v !== cur) await f.loc.click({ timeout: 6000 }); return !cur;
  }
  async editField(rowRe, label, { keys = [], value, group, role } = {}) {
    const s = await this.selectRow(rowRe, { group });
    if (!s.ok) return { pass: false, observed: s.why ?? `row ${rowRe} not selectable` };
    let f = await this.field(label, keys);
    if (!f) return { pass: false, observed: `inspection field "${label}" missing; inspection="${(await this.insp()).slice(0, 160)}"` };
    const before = await this.readField(f);
    const want = await this.setField(f, value);
    const exp = typeof want === "boolean" ? String(want) : String(want);
    const ms = await this.until(async () => { const g = (await this.field(label, keys)) ?? f; return same(await this.readField(g), exp) || (["button", "slider"].includes(f.role) && (await this.readField(g)) !== before); }, 40000, 500);
    f = (await this.field(label, keys)) ?? f;
    const after = await this.readField(f);
    return { pass: ms >= 0, observed: `"${label}" ${JSON.stringify(before)} -> ${JSON.stringify(after)} (wanted ${JSON.stringify(exp)}) in ${ms} ms` };
  }

  actionLabel(id) { return new RegExp(`^${esc(words(id))}…?$`); }
  async paletteOpen() { return (await this.modelPanel().getByRole("button", { name: /^Clear Selection$/ }).count()) > 0; }
  windowPanel(win) { return win === "results" ? this.resultsPanel() : this.modelPanel(); }
  async openPalette(win = "model", probe = /^Clear Selection$/) {
    const panel = this.windowPanel(win), has = async () => (await panel.getByRole("button", { name: probe }).count()) > 0;
    if (await has()) return true;
    await this.tap(panel.getByRole("button", { name: /^Actions/ }).first(), 8000); await this.page.waitForTimeout(1500);
    return has();
  }
  async actionPfx(id) { return (await this.page.evaluate(id => { const m = [...document.querySelectorAll("[id^='framework.window.']")].map(e => e.id.split(".")).find(p => p[3] === "action" && p[4] === id); return m ? m[2] : null; }, id)) ?? this.app.pfx; }
  async argIds(id) {
    return this.page.evaluate(({ pfx, id }) => [...document.querySelectorAll(`[id^="framework.window.${pfx}.action.${id}.arg."][id$=".input"]`)].map(e => ({ k: e.id.split(".arg.")[1].replace(/\.input$/, ""), tag: e.tagName, role: e.getAttribute("role") ?? "", type: e.type ?? "", value: e.value ?? "" })), { pfx: this.pfxNow ?? this.app.pfx, id });
  }
  argLoc(id, k) { return this.page.locator(`[id="framework.window.${this.pfxNow ?? this.app.pfx}.action.${id}.arg.${k}.input"]`); }
  execLoc(id) { return this.page.locator(`[id="framework.window.${this.pfxNow ?? this.app.pfx}.action.${id}.execute"]`); }
  async runAction(id, args = {}, { verify, timeout = 70000, after = 0, win = "model" } = {}) {
    const note = []; this.pfxNow = null;
    if (!(await this.openPalette(win, win === "model" ? undefined : this.actionLabel(id)))) return { pass: false, observed: `Actions palette of the ${win} window did not open` };
    const item = this.windowPanel(win).getByRole("button", { name: this.actionLabel(id) }).first();
    if (!(await item.count())) return { pass: false, observed: `palette item ${words(id)} missing in the ${win} window` };
    const pre = verify?.pre ? await verify.pre() : null;
    if (!(await this.execLoc(id).count())) { await item.click({ timeout: 8000 }); await this.page.waitForTimeout(1200); }
    this.pfxNow = await this.actionPfx(id); const found = await this.argIds(id); note.push(`args=[${found.map(a => a.k).join(",")}]`);
    const hasForm = (await this.execLoc(id).count()) > 0;
    for (const [wants, given] of Object.entries(args)) {
      const v = typeof given === "function" ? await given(this) : given;
      if (v === undefined) { note.push(`arg ${wants} has no value`); continue; }
      const cands = wants.split("|"); let m, want = cands[0];
      for (const c of cands) { m = found.find(a => a.k === c) ?? found.find(a => a.k.toLowerCase() === c.toLowerCase()) ?? (c.length >= 3 ? found.find(a => a.k.toLowerCase().includes(c.toLowerCase())) : undefined); if (m) { want = c; break; } }
      if (!m) { note.push(`arg ${wants} missing`); continue; }
      const loc = this.argLoc(id, m.k); const idx = typeof v === "string" && /^#\d+$/.test(v) ? Number(v.slice(1)) - 1 : null;
      try {
        if (m.role === "combobox" || m.tag === "SELECT") { const r = await this.chooseOption(loc, idx !== null ? idx : v === "" || v === true ? undefined : new RegExp(esc(v), "i"), v === "" || v === true ? { other: "" } : {}); if (!r.ok) note.push(`option ${v} not in [${r.options}]`); else note.push(`${m.k}=${r.picked}`); }
        else if (m.tag === "BUTTON") { const on = (await loc.getAttribute("aria-checked")) ?? (await loc.getAttribute("aria-pressed")); if (on === null ? Boolean(v) : (on === "true") !== Boolean(v)) await loc.click(); note.push(`${m.k}=${Boolean(v)} (was ${on})`); }
        else if (m.type === "checkbox") { if ((await loc.isChecked()) !== Boolean(v)) await loc.click(); }
        else await loc.fill(String(v));
      } catch (e) { note.push(`fill ${want}: ${String(e).slice(0, 80)}`); }
    }
    for (const a of found) if ((a.role === "combobox" || a.tag === "SELECT") && !Object.keys(args).some(ks => ks.split("|").some(k => a.k.toLowerCase() === k.toLowerCase() || (k.length >= 3 && a.k.toLowerCase().includes(k.toLowerCase()))))) {
      const txt = ((await this.argLoc(id, a.k).textContent().catch(() => "")) ?? "").trim();
      if (!txt || /select|choose|^$/i.test(txt)) { const r = await this.chooseOption(this.argLoc(id, a.k), undefined, { other: "" }); note.push(`auto ${a.k}=${r.picked}`); }
    }
    const mark = this.mark(); const t = Date.now();
    if (hasForm) {
      const ex = this.execLoc(id); const dis = await ex.getAttribute("aria-disabled");
      if (dis === "true") { note.push("execute disabled"); await this.until(async () => (await ex.getAttribute("aria-disabled")) !== "true", 8000, 300); }
      await ex.click({ timeout: 8000 });
    }
    let ok = true, why = "";
    if (verify) { const ms = await this.until(async () => verify.post(pre), timeout, 600); ok = ms >= 0; why = ok ? `effect after ${ms} ms` : "no effect before timeout"; }
    else await this.page.waitForTimeout(1500 + after);
    const con = this.consoleSince(mark);
    if (hasForm && (await this.execLoc(id).count())) await item.click().catch(() => {});
    return { pass: ok && con.fatal === 0, observed: `${why}; ${note.join("; ")}; execute ${Date.now() - t} ms; console errors ${con.errors}${con.fatal ? ` fatal ${con.fatalFirst.join(" || ")}` : ""}`, pre };
  }

  async commands() {
    await this.panel("History", true);
    const tree = this.historyPanel().getByRole("tree").filter({ hasText: /Commit Checkpoint/ }).last();
    await this.until(async () => (await tree.count()) > 0 && /Commands/i.test(await tree.innerText()), 8000, 300);
    const t = (await tree.innerText().catch(() => ""));
    const i = t.search(/Commands/i);
    return i >= 0 ? t.slice(i).split("\n").map(s => s.trim()).filter(s => s && !/^Commands$/i.test(s)) : [];
  }
  settingsPanel() { return this.page.locator("body"); }
  async setLanguage(name) {
    const cbs = () => this.page.getByRole("combobox", { name: /^(Language|Sprache)$/ }).first();
    if (!(await cbs().count())) { await this.btn(/^(Settings|Einstellungen)$/, false).first().click({ timeout: 8000 }); await this.page.waitForTimeout(2000); }
    await cbs().click({ timeout: 8000 }); await this.page.waitForTimeout(700);
    await this.page.getByRole("option", { name: new RegExp(name) }).first().click({ timeout: 8000 }); await this.page.waitForTimeout(3500);
  }
  async alerts() { return this.page.evaluate(() => [...document.querySelectorAll("[role=alert]")].map(e => (e.textContent ?? "").trim()).filter(Boolean)); }
  async waitAlert(re, ms = 20000) { let hit = null; await this.until(async () => { hit = (await this.alerts()).find(t => re.test(t)) ?? null; return hit; }, ms, 150); return hit; }
  async dispatchKey(key, mods = []) { for (const m of mods) await this.page.keyboard.down(m); await this.page.keyboard.press(key); for (const m of mods.reverse()) await this.page.keyboard.up(m); }
}

