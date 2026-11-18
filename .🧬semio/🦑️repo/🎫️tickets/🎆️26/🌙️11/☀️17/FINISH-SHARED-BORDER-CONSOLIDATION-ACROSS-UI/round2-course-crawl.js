/** Round 2 course crawler: drives the course dev server and probes every view. Import in the page via /@fs. */
const DIR = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI";
export const sleep = ms => new Promise(r => setTimeout(r, ms));
export const root = () => {
  const c = [...document.querySelectorAll("[data-page]")].filter(e => {
    if (e.closest("[inert]")) return false;
    const r = e.getBoundingClientRect();
    return r.left >= -1 && r.left < innerWidth && r.top < innerHeight && r.width > 0;
  });
  const dialog = document.querySelector("[data-course-dialog]");
  const under = document.elementFromPoint(innerWidth / 2, innerHeight / 2)?.closest("[data-page]");
  return dialog || under || c[c.length - 1] || document.querySelector("#course-main") || document.body;
};
export const click = (re, scope = document) => {
  const b = [...scope.querySelectorAll("button,a,[role=button],summary")].filter(e => !e.closest("[inert]") && re.test(e.textContent.trim()) && e.getBoundingClientRect().width > 0)[0];
  if (!b) return false;
  b.click();
  return true;
};
export async function probe(mark = false, rootEl) {
  const p = await import("/@fs" + encodeURI(DIR + "/border-gap-probe.js") + "?t=" + Date.now());
  const el = rootEl || window.__rootOverride || root();
  const r = p.probe({ mark, root: el });
  const stack = s => /(course-card-stack|slot=navbar).*svg:[hv]$/.test(s.trim()) || /^footer\.course-footer/.test(s);
  const inside = [], mixed = [];
  for (const v of r.violations) {
    const o = v.distance + " " + v.a.slice(0, 110) + " || " + v.b.slice(0, 110);
    (stack(v.a) && stack(v.b) ? mixed : inside).push(o);
  }
  return { root: el.dataset?.page || el.id || el.tagName, count: r.count, inside, mixed };
}
export async function pageRec(rows, name, id, overlay = false) {
  for (let k = 0; k < 20; k++) {
    const el = document.querySelector(`[data-page="${id}"]`);
    if (el && !el.closest("[inert]") && Math.abs(el.getBoundingClientRect().left) < 2) break;
    await sleep(500);
  }
  await sleep(800);
  const page = document.querySelector(`[data-page="${id}"]`);
  window.__rootOverride = overlay ? page.querySelector("[data-layered-overlay]") || page : page;
  try { return await rec(rows, name); } finally { window.__rootOverride = undefined; }
}
export async function rec(rows, name) {
  await Promise.race([new Promise(r => requestAnimationFrame(() => requestAnimationFrame(r))), sleep(600)]);
  await sleep(350);
  let r = await probe();
  for (let k = 0; k < 5 && r.inside.length; k++) { await sleep(1300); r = await probe(); }
  rows.push(`${name} => inside=${r.inside.length} mixed=${r.mixed.length}` + (r.inside.length ? "\n    " + r.inside.slice(0, 6).join("\n    ") : ""));
  return r;
}
export async function fill() {
  const setter = Object.getOwnPropertyDescriptor(HTMLSelectElement.prototype, "value").set;
  let n = 0;
  for (const s of [...document.querySelectorAll("#course-main select")]) {
    if (s.closest("[inert]")) continue;
    const opts = [...s.options];
    if (opts.length < 2) continue;
    setter.call(s, opts[1 + (n % (opts.length - 1))].value);
    s.dispatchEvent(new Event("change", { bubbles: true }));
    n++;
    await sleep(60);
  }
  return n;
}
export async function typeGuesses() {
  const setter = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value").set;
  let n = 0;
  for (const i of [...document.querySelectorAll("#course-main input.course-input")]) {
    if (i.closest("[inert]")) continue;
    setter.call(i, String(2 + (n % 7)));
    i.dispatchEvent(new Event("input", { bubbles: true }));
    i.blur();
    n++;
    await sleep(60);
  }
  return n;
}
export async function crawlRun(rows, name, { doFill = true } = {}) {
  const n = document.querySelectorAll(".course-steps > li button").length;
  for (let i = 0; i < n; i++) {
    const b = document.querySelectorAll(".course-steps > li button")[i];
    if (!b) break;
    b.click();
    await sleep(500);
    await rec(rows, `${name} step${i + 1}`);
    if (doFill) {
      const c = await fill();
      const g = await typeGuesses();
      await sleep(500);
      await rec(rows, `${name} step${i + 1} filled(${c}/${g})`);
    }
  }
  return n;
}
export async function quiz(rows, id, level, { doFill = true } = {}) {
  if (document.querySelector(".course-steps") || location.hash === "") { click(/^Course overview/); await sleep(1200); }
  location.hash = "#" + id;
  await sleep(1400);
  const radio = document.querySelector(`input.course-radio[value="${level}"]`);
  if (radio) { radio.click(); await sleep(300); }
  await rec(rows, `${id}/${level} quiz-page`);
  const Lv = level[0].toUpperCase() + level.slice(1);
  const ok = click(new RegExp(`\\(${Lv}\\)`)) || click(/^(Start|Resume|Continue|Again)/);
  await sleep(2000);
  if (document.querySelector("[data-course-dialog]")) {
    await rec(rows, `${id}/${level} dialog`);
    click(/^Discard and start|^Start/);
    await sleep(2000);
  }
  if (!document.querySelector(".course-steps")) { rows.push(`${id}/${level}: no run opened (${ok})`); return; }
  await crawlRun(rows, `${id}/${level} run`, { doFill });
}

const KEY = "__crawlRows";
export const log = {
  get rows() { try { return JSON.parse(sessionStorage.getItem(KEY) || "[]"); } catch { return []; } },
  push(x) { const r = this.rows; r.push(x); sessionStorage.setItem(KEY, JSON.stringify(r)); },
  clear() { sessionStorage.removeItem(KEY); sessionStorage.removeItem(KEY + "Done"); },
};
export async function crawl(ids, levels, opts) {
  const done = new Set(JSON.parse(sessionStorage.getItem(KEY + "Done") || "[]"));
  for (const id of ids) for (const lv of levels) {
    const k = id + "/" + lv;
    if (done.has(k)) continue;
    await quiz(log, id, lv, opts);
    done.add(k);
    sessionStorage.setItem(KEY + "Done", JSON.stringify([...done]));
  }
  return log.rows.join("\n");
}

export async function pages(hashes, tag = "page") {
  const done = new Set(JSON.parse(sessionStorage.getItem(KEY + "PDone") || "[]"));
  for (const hs of hashes) {
    const k = tag + hs;
    if (done.has(k)) continue;
    if (document.querySelector(".course-steps")) { click(/^Course overview/); await sleep(1200); }
    if (location.hash !== "#" + hs) { location.hash = "#" + hs; await sleep(2200); }
    await sleep(1000);
    await rec(log, `${tag} #${hs}`);
    done.add(k);
    sessionStorage.setItem(KEY + "PDone", JSON.stringify([...done]));
  }
  return log.rows.join("\n");
}

export async function cards(rows, tag, scope = document) {
  const list = [...scope.querySelectorAll("[data-card]")].filter(e => {
    const r = e.getBoundingClientRect();
    return r.width > 0 && r.height > 0 && r.left < innerWidth && r.right > 0 && r.top < innerHeight * 3 && !e.closest("[inert]");
  });
  for (const c of list) {
    window.__rootOverride = c;
    try { await rec(rows, `${tag} card ${c.dataset.card}`); } finally { window.__rootOverride = undefined; }
  }
  return list.length;
}

export async function quizRun(id, level, { doFill = true, results = false } = {}) {
  const rows = [];
  const Lv = level[0].toUpperCase() + level.slice(1);
  await sleep(3500);
  const radio = document.querySelector(`input.course-radio[value="${level}"]`);
  if (radio) { radio.click(); await sleep(300); }
  await pageRec(rows, `${id}/${level} quiz-page`, `quiz:${id}`);
  click(new RegExp(`\\(${Lv}\\)`)) || click(/^(Start|Resume|Continue|Again)/);
  await sleep(1800);
  if (document.querySelector("[data-course-dialog]")) {
    await rec(rows, `${id}/${level} dialog`);
    click(/^Discard and start|^Start/);
    await sleep(2000);
  }
  if (!document.querySelector(".course-steps")) { rows.push(`${id}/${level}: no run opened`); return rows.join("\n"); }
  await crawlRun(rows, `${id}/${level} run`, { doFill });
  return rows.join("\n");
}
