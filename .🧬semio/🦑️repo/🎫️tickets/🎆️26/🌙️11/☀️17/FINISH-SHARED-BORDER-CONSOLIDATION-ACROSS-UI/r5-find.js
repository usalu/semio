/** 🔎️ Opens a quiz run at a level and stops on the first step that shows the given selector. */
import * as R from "./round3-course-crawl.js";
export async function find(id, level, selector) {
  const de = document.documentElement.lang === "de"; const Lv = de ? { easy: "Leicht", medium: "Mittel", hard: "Schwer", expert: "Experte" }[level] : level[0].toUpperCase() + level.slice(1);
  if (document.querySelector(".course-steps")) { R.click(/^(Course overview|Kursübersicht)/); await R.sleep(1200); }
  location.hash = "#" + id; await R.sleep(2600);
  const scope = () => document.querySelector(`[data-page="quiz:${id}"]`) ?? document;
  scope().querySelector(`input.course-radio[value="${level}"]`)?.click(); await R.sleep(400);
  R.click(new RegExp(`\\(${Lv}\\)`), scope()) || R.click(/^(Start|Resume|Continue|Again|Starten|Fortsetzen|Weiter|Nochmal)/, scope());
  await R.sleep(1800);
  for (let k = 0; k < 3 && !document.querySelector(".course-steps"); k++) {
    const dlg = [...document.querySelectorAll("dialog[open],[data-course-dialog],[role=dialog]")].filter(d => d.getBoundingClientRect().width > 0 && !d.closest("[inert]"));
    if (!dlg.length) break;
    if (/How do you want to appear|Wie möchtest du/.test(dlg[0].textContent)) { R.click(/^(Continue|Weiter)/, dlg[0]); await R.sleep(2500); location.hash = "#" + id; await R.sleep(2200); R.click(new RegExp(`\\(${Lv}\\)`), scope()) || R.click(/^(Start|Resume|Continue|Again|Starten|Fortsetzen|Weiter|Nochmal)/, scope()); await R.sleep(1800); }
    else { R.click(/^Discard and start|^Start|^Confirm|^Verwerfen|^Starten|^Bestätigen/, dlg[0]); await R.sleep(2000); }
  }
  const n = document.querySelectorAll(".course-steps > li button").length;
  for (let i = 0; i < n; i++) {
    document.querySelectorAll(".course-steps > li button")[i]?.click(); await R.sleep(700);
    const hit = [...document.querySelectorAll("#course-main " + selector)].filter(e => !e.closest("[inert]") && e.getBoundingClientRect().width > 0);
    if (hit.length) return `${id}/${level} step${i + 1} ${hit.length}x ${selector}`;
  }
  return `${id}/${level} none in ${n} steps`;
}
