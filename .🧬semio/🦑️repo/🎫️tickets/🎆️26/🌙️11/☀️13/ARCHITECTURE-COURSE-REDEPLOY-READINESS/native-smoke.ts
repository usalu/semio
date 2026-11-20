import { chromium } from '/Users/ueli/Documents/semio/node_modules/playwright/index.mjs';
import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const BASE = process.env.SMOKE_BASE ?? 'http://127.0.0.1:8799/';
const OUT = join(dirname(fileURLToPath(import.meta.url)), '🗑️generated', 'smoke');
mkdirSync(OUT, { recursive: true });

const QUIZZES = ['Physical Understanding', 'Ventilation', 'Heating', 'Cooling', 'Energy Demand'];
const LOCKED = ['Parametric Design', 'Grasshopper: Data and Trees', 'Excel: Workbook and Commands', 'BIM Concepts', 'Objectives and Pareto'];
const PDFS = [
  'architecture-parametric-parametrization-en.pdf', 'architecture-parametric-parametrization-de.pdf',
  'architecture-parametric-typology-en.pdf', 'architecture-parametric-typology-de.pdf',
  'architecture-energy-heating-cooling-en.pdf', 'architecture-energy-heating-cooling-de.pdf',
];
const PETS = ['Hello', 'Trick', 'Pet', 'Toss', 'Circle clockwise', 'Circle counterclockwise', 'Stroke', 'Shake'];
const pause = (ms) => new Promise((r) => setTimeout(r, ms));

const visible = (p, name, exact = true) => {
  const l = p.getByRole('button', { name, exact }).locator('visible=true');
  const orig = l.click.bind(l);
  return l;
};
async function press(loc) {
  try { await loc.click({ timeout: 4000 }); } catch { await loc.evaluate((el) => el.click()); }
}

async function skipTour(p) {
  for (let i = 0; i < 3; i++) {
    const s = visible(p, 'Skip');
    if (await s.count()) { await press(s.first()); await pause(300); } else break;
  }
}

async function open(browser, tag, viewport, errors) {
  const ctx = await browser.newContext({ viewport, isMobile: viewport.width < 600, hasTouch: viewport.width < 600 });
  const p = await ctx.newPage();
  p.on('pageerror', (e) => errors.push(`pageerror: ${e.message}`));
  p.on('crash', () => errors.push('page crashed'));
  p.on('console', (m) => { if (m.type() === 'error') errors.push(`console.error: ${m.text()}`); });
  await p.goto(BASE);
  await p.waitForSelector('h1', { timeout: 20000 });
  await pause(2500);
  await skipTour(p);
  return { ctx, p };
}

async function home(p) {
  const b = p.getByRole('button', { name: 'Course overview' });
  if (await b.isEnabled()) { await press(b); await pause(800); }
}
async function exploreEnergy(p, topic = 0) {
  await home(p);
  await press(visible(p, 'Explore').first()); await pause(900);
  await press(visible(p, 'Explore').nth(topic)); await pause(1200);
}
async function startQuiz(p, idx, topic = 0) {
  await exploreEnergy(p, topic);
  await press(visible(p, 'Start (Medium)').nth(idx)); await pause(900);
  const cont = visible(p, 'Continue');
  if (await cont.count()) {
    await press(cont.first()); await pause(1500);
    await exploreEnergy(p, topic);
    await press(visible(p, 'Start (Medium)').nth(idx)); await pause(1500);
  }
  await skipTour(p);
}

async function fillTask(p) {
  return p.evaluate(() => {
    let n = 0;
    const fire = (el) => { el.dispatchEvent(new Event('input', { bubbles: true })); el.dispatchEvent(new Event('change', { bubbles: true })); n++; };
    document.querySelectorAll('select').forEach((s) => {
      if (s.getBoundingClientRect().width && s.options.length > 1) { s.selectedIndex = 1; fire(s); }
    });
    document.querySelectorAll('input[type=text],input[type=number],input:not([type]),textarea').forEach((i) => {
      if (i.getBoundingClientRect().width && !i.readOnly && !i.disabled) {
        const set = Object.getOwnPropertyDescriptor(Object.getPrototypeOf(i), 'value').set;
        set.call(i, '100'); fire(i);
      }
    });
    return n;
  });
}

async function quizCheck(p, name, shot) {
  const info = {};
  const body = () => p.evaluate(() => document.body.innerText);
  const t = await body();
  if (!t.includes(name)) throw new Error(`quiz heading "${name}" not shown`);
  const tabs = p.getByRole('button', { name: /^\S*\s*\d+\. / }).locator('visible=true');
  info.tabs = await tabs.count();
  if (!info.tabs) throw new Error('no task tabs rendered');
  const interactive = await p.evaluate(() => [...document.querySelectorAll('main select, main input, main button, main [draggable=true], select, input:not([type=radio])')].filter((e) => e.getBoundingClientRect().width).length);
  if (!/Task 1 of \d+/.test(t)) throw new Error('first task header "Task 1 of N" missing');
  info.interactive = interactive;
  await p.screenshot({ path: shot('first-task') });
  info.filled = 0;
  for (let i = 0; i < info.tabs; i++) {
    await press(tabs.nth(i)); await pause(600);
    info.filled += await fillTask(p); await pause(300);
  }
  await p.screenshot({ path: shot('answered') });
  const submit = visible(p, 'Submit quiz');
  if (await submit.count()) {
    await press(submit.first()); await pause(1500);
    for (const label of ['Submit', 'Confirm', 'Yes', 'Continue']) {
      const c = visible(p, new RegExp(`^${label}`)).filter({ hasNotText: 'quiz' });
      if (await c.count()) { await c.first().click().catch(() => {}); await pause(1200); break; }
    }
    const after = await body();
    info.afterSubmit = after.slice(0, 160).replace(/\s+/g, ' ');
    info.resultShown = /Points|points|score|Score|Result|result|Your result/.test(after);
    await p.screenshot({ path: shot('submitted') });
  } else info.afterSubmit = 'no Submit quiz button';
  return info;
}

async function featureChecks(p, shot, viewport, results, errsOf) {}

async function runAll(tag, viewport) {
  const browser = await chromium.launch();
  const results = [];
  const check = async (name, fn) => {
    const errors = [];
    const safe = name.replace(/[^a-z0-9]+/gi, '-').toLowerCase();
    const shot = (s) => join(OUT, `${tag}-${safe}-${s}.png`);
    let ctx, p, status = 'PASS', detail = {}, failure = '';
    try {
      ({ ctx, p } = await open(browser, tag, viewport, errors));
      detail = (await fn(p, shot)) ?? {};
    } catch (e) { status = 'FAIL'; failure = String(e.message).split('\n').slice(0, 3).join(' / '); }
    if (!status.startsWith('FAIL') && errors.some((e) => e.startsWith('pageerror') || e.startsWith('page crashed'))) { status = 'FAIL'; failure = errors.find((e) => e.startsWith('pageerror') || e.startsWith('page crashed')); }
    if (status === 'FAIL' && p) await p.screenshot({ path: shot('FAIL') }).catch(() => {});
    results.push({ tag, name, status, failure, detail, errors: [...new Set(errors)] });
    console.log(`[${tag}] ${status} ${name}${failure ? ' :: ' + failure : ''}${errors.length ? ` (errors: ${[...new Set(errors)].slice(0, 2).join(' | ')})` : ''}`);
    await ctx?.close();
  };

  for (let i = 0; i < QUIZZES.length; i++) {
    await check(`quiz ${QUIZZES[i]}`, async (p, shot) => { await startQuiz(p, i); return quizCheck(p, QUIZZES[i], shot); });
  }
  if (process.env.SMOKE_ALL_QUIZZES === '1') {
    for (let i = 0; i < LOCKED.length; i++) {
      await check(`quiz ${LOCKED[i]}`, async (p, shot) => { await startQuiz(p, 0, i + 1); return quizCheck(p, LOCKED[i], shot); });
    }
  }
  await check('locked topics (Parametrics .. Optimization)', async (p, shot) => {
    await home(p); await press(visible(p, 'Explore').first()); await pause(1200);
    const t = await p.evaluate(() => document.body.innerText);
    const locked = (t.match(/Not available yet/g) ?? []).length;
    await p.screenshot({ path: shot('locked') });
    return { lockedTopicCards: locked, note: 'topics with unlocks:null are not reachable; quizzes ' + LOCKED.join(', ') + ' cannot be started from the UI' };
  });
  await check('Explore', async (p, shot) => {
    await home(p); await press(visible(p, 'Explore').first()); await pause(1000);
    if (!(await p.evaluate(() => document.body.innerText)).includes('Energy')) throw new Error('topics not shown');
    await press(visible(p, 'Explore').first()); await pause(1200);
    if (!(await visible(p, 'Start (Medium)').count())) throw new Error('no quiz cards');
    await p.screenshot({ path: shot('ok') });
  });
  await check('Full leaderboard', async (p, shot) => {
    await home(p); await press(visible(p, 'Full leaderboard').first()); await pause(1500);
    for (const b of ['Today', 'This week', 'This month', 'All time']) { const x = visible(p, b); if (await x.count()) { await press(x.first()); await pause(300); } }
    await p.screenshot({ path: shot('ok') });
    return { text: (await p.evaluate(() => document.body.innerText)).slice(0, 120).replace(/\s+/g, ' ') };
  });
  await check('All badges', async (p, shot) => {
    await home(p); await press(visible(p, 'All badges').first()); await pause(1500);
    const t = await p.evaluate(() => document.body.innerText);
    if (!t.includes('Physics Expert')) throw new Error('badge list missing');
    await p.screenshot({ path: shot('ok') });
  });
  await check('Introduction', async (p, shot) => {
    await press(visible(p, 'Introduction').first()); await pause(1500);
    await p.screenshot({ path: shot('ok') });
    return { text: (await p.evaluate(() => document.body.innerText)).slice(0, 120).replace(/\s+/g, ' ') };
  });
  const settings = async (p) => { await home(p); await press(visible(p, 'Open').first()); await pause(1200); };
  await check('Settings: language DE and back', async (p, shot) => {
    await press(visible(p, 'Deutsch').first()); await pause(1200);
    const de = await p.evaluate(() => document.body.innerText);
    if (!/Architektur|Kurs|Einführung/.test(de)) throw new Error('German texts not shown');
    await p.screenshot({ path: shot('de') });
    await exploreEnergyDe(p);
    await p.getByRole('button', { name: /English/ }).first().click(); await pause(1200);
    if (!(await p.evaluate(() => document.body.innerText)).includes('Architecture')) throw new Error('English not restored');
  });
  await check('Settings: pets', async (p, shot) => {
    await settings(p);
    const clicked = [];
    for (const n of PETS) {
      const b = visible(p, n);
      const c = await b.count();
      for (let i = 0; i < c; i++) { await b.nth(i).click({ timeout: 3000 }).catch(() => {}); await pause(250); }
      if (c) clicked.push(`${n}x${c}`);
    }
    for (const n of ['Still', 'Calm', 'Lively', 'Off']) { const b = visible(p, n); if (await b.count()) { await press(b.first()); await pause(300); clicked.push(n); } }
    await p.screenshot({ path: shot('ok') });
    if (!clicked.length) throw new Error('no pet buttons visible');
    return { clicked };
  });
  await check('Settings: dark theme', async (p, shot) => {
    await settings(p);
    const bg = () => p.evaluate(() => getComputedStyle(document.body).backgroundColor);
    const before = await bg();
    await press(visible(p, 'Dark').first()); await pause(800);
    const after = await bg();
    await p.screenshot({ path: shot('dark') });
    await press(visible(p, 'Light').first()); await pause(500);
    return { before, after };
  });
  await check(viewport.width < 768 ? 'Settings: no layout switch on a phone + text sizes' : 'Settings: tablet layout + text sizes', async (p, shot) => {
    await settings(p);
    if (viewport.width < 768) {
      const offered = await p.getByRole('button', { name: /^(Tablet|Desktop)$/ }).locator('visible=true').count();
      if (offered !== 0) throw new Error(`a phone is offered ${offered} layout switches`);
    } else {
      await press(visible(p, 'Tablet').first()); await pause(1000);
      await p.screenshot({ path: shot('tablet') });
    }
    for (const s of ['Large', 'Larger', 'Largest', 'Normal']) { await press(visible(p, s).first()); await pause(400); }
    if (viewport.width >= 768) { await press(visible(p, 'Desktop').first()); await pause(500); }
  });
  await check('Assignment PDFs', async (p) => {
    const hrefs = await p.evaluate(() => [...document.querySelectorAll('a[href$=".pdf"]')].map((a) => a.getAttribute('href')));
    const all = [...new Set([...hrefs, ...PDFS.map((f) => `/assignments/${f}`)])];
    const out = [];
    for (const h of all) {
      const r = await p.request.get(new URL(h, BASE).toString());
      const buf = await r.body();
      const ok = r.status() === 200 && buf.subarray(0, 5).toString() === '%PDF-';
      out.push(`${ok ? 'ok' : 'BAD'} ${r.status()} ${h} (${buf.length} B)`);
      if (!ok) throw new Error(`bad pdf ${h}: ${r.status()}`);
    }
    return { out };
  });
  await browser.close();
  return results;
}

async function exploreEnergyDe(p) {
  await p.getByRole('button', { name: /Kursübersicht|Course overview/ }).first().click().catch(() => {}); await pause(600);
  await p.locator('button:has-text("Erkunden"), button:has-text("Explore")').locator('visible=true').first().click().catch(() => {}); await pause(800);
}

const runs = [['desktop', { width: 1280, height: 900 }], ['phone', { width: 390, height: 844 }]];
const only = process.argv[2];
const all = [];
for (const [tag, vp] of runs) if (!only || only === tag) all.push(...(await runAll(tag, vp)));
writeFileSync(join(OUT, 'results.json'), JSON.stringify(all, null, 2));
const bad = all.filter((r) => r.status !== 'PASS');
console.log(`\n${all.length - bad.length}/${all.length} passed; results: ${join(OUT, 'results.json')}`);
process.exit(bad.length ? 1 : 0);
