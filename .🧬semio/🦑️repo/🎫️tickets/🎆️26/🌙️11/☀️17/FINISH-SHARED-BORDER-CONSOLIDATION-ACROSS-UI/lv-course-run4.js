/** 🔭️ Resumable course live gate: pages, quiz runs with results, persisted per viewport in sessionStorage. */
import * as R from "./round3-course-crawl.js";
const D = "lv" + innerWidth + (document.documentElement.lang || "");
const kd = "__lvc_done_" + D;
const done = () => new Set(JSON.parse(sessionStorage.getItem(kd) || "[]"));
const mark = k => { const s = done(); s.add(k); sessionStorage.setItem(kd, JSON.stringify([...s])); };
export const out = () => R.rows.all.join("\n");
export async function pages(hashes) {
  for (const h of hashes) {
    if (done().has("p" + h)) continue;
    try { await R.view(h); } catch (e) { R.rows.push(`ERR page ${h} ${e.message}`); }
    mark("p" + h);
  }
}
async function run(id, level) {
  const Lv = level[0].toUpperCase() + level.slice(1);
  if (document.querySelector(".course-steps")) { R.click(/^Course overview/); await R.sleep(1200); }
  location.hash = "#" + id;
  await R.sleep(2600);
  const scope = () => document.querySelector(`[data-page="quiz:${id}"]`) ?? document;
  scope().querySelector(`input.course-radio[value="${level}"]`)?.click();
  await R.sleep(400);
  await R.rec(`${id}/${level} quiz-page`);
  R.click(new RegExp(`\\(${Lv}\\)`), scope()) || R.click(/^(Start|Resume|Continue|Again)/, scope());
  await R.sleep(1800);
  for (let k = 0; k < 3 && !document.querySelector(".course-steps"); k++) {
    const dlg = [...document.querySelectorAll("dialog[open],[data-course-dialog],[role=dialog]")].filter(d => d.getBoundingClientRect().width > 0 && !d.closest("[inert]"));
    if (!dlg.length) break;
    await R.rec(`${id}/${level} dialog${k}`);
    if (/How do you want to appear/.test(dlg[0].textContent)) { R.click(/^Continue/, dlg[0]); await R.sleep(2500); location.hash = "#" + id; await R.sleep(2200); R.click(new RegExp(`\\(${Lv}\\)`), scope()) || R.click(/^(Start|Resume|Continue|Again)/, scope()); await R.sleep(1800); }
    else { R.click(/^Discard and start|^Start|^Confirm/, dlg[0]); await R.sleep(2000); }
  }
  if (!document.querySelector(".course-steps")) { R.rows.push(`${id}/${level}: no run opened`); return; }
  const n = document.querySelectorAll(".course-steps > li button").length;
  for (let i = 0; i < n; i++) {
    document.querySelectorAll(".course-steps > li button")[i]?.click();
    await R.sleep(700);
    await R.rec(`${id}/${level} step${i + 1}`);
    await R.stateSweep(`${id}/${level} step${i + 1}`);
    const c = await R.fill(); const g = await R.typeGuesses(); await R.sleep(500);
    await R.rec(`${id}/${level} step${i + 1} filled(${c}/${g})`);
  }
}
export async function quizzes(ids, levels, { results = true } = {}) {
  for (const id of ids) for (const lv of levels) {
    const k = `q${id}/${lv}`;
    if (done().has(k)) continue;
    try {
      await run(id, lv);
      if (results && document.querySelector(".course-steps")) {
        const ok = R.click(/Submit quiz/);
        await R.sleep(1500);
        if (document.querySelector("[data-course-dialog]")) { await R.rec(`${id}/${lv} submit-dialog`); R.click(/^Submit|^Confirm|^Yes/); await R.sleep(2500); }
        await R.rec(`${id}/${lv} results(${ok})`);
      }
    } catch (e) { R.rows.push(`ERR quiz ${k} ${e.message}`); }
    mark(k);
  }
}
