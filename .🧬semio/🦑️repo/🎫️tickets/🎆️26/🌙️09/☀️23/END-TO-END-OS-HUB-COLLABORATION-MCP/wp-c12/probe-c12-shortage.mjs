/** 🔌️ C12 item 4 probe — short connection shortages DURING typing on both clients. A (locale A) and B (locale B) each reach
 * the hub through their own `c10-link-proxy.ts` (a real transport cut: every relayed connection destroyed, new ones refused).
 * For every cut length both links are cut at once while both humans keep typing distinct markers; the probe samples each
 * page's frame time (rAF) and DOM read latency every 250 ms, reads the hub badge (`[data-semio-hub-connection]`), the sync
 * pill and every notice in the human's own language, measures local echo per keystroke burst, and after the links return
 * waits for both editors to agree and records which markers survived and the hub head.
 * usage: source env.sh; bun probe-c12-shortage.mjs <tag> <urlA> <urlB> <controlA> <controlB> <spaceId> <cutsMs=5000,15000,60000> [kindId=text.document]
 * env: C12_LOCALE_A (en-US), C12_LOCALE_B (de-DE), C12_TYPISTS (AB: both type during a cut; A or B: only that human types, which
 *      isolates the outbox drain from the whole-text SET race of item 2) */
import { boot, openSessions, recorder, signIn, shot } from "./c12-lib.mjs";
import { awaitMounted, createArtifact, creatableKinds, hubHead, openRow, openSpace, pause } from "./c12-journey.mjs";

const [tag = "c12shortage", urlA, urlB, controlA, controlB, spaceId, cutList = "5000,15000,60000", kindId = "text.document"] = process.argv.slice(2);
const locales = [process.env.C12_LOCALE_A ?? "en-US", process.env.C12_LOCALE_B ?? "de-DE"];
const typists = process.env.C12_TYPISTS ?? "AB";
const rebuildNotice = /fresh authoritative restore|autoritative Wiederherstellung/u;
const { browser, sessions } = await openSessions([urlA, urlB], { locale: locales[0] });
const [A, B] = sessions;
await B.context.close();
{
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: locales[1] });
  const page = await context.newPage();
  const lines = [];
  page.on("console", (message) => {
    const text = message.text();
    if (/Download the React DevTools|\[vite\]|agent-bridge|status of 404/.test(text)) return;
    lines.push(`${Date.now()} ${message.type()} ${text.slice(0, 900)}`);
  });
  page.on("pageerror", (error) => lines.push(`${Date.now()} pageerror ${String(error).slice(0, 900)}`));
  page.on("worker", (worker) => worker.on("console", (message) => { if (message.type() === "warning" || message.type() === "error") lines.push(`${Date.now()} worker-${message.type()} ${message.text().slice(0, 3000)}`); }));
  Object.assign(B, { context, page, lines, sockets: [] });
}
const { record, save } = recorder(tag, sessions);
const editorOf = (session) => session.page.locator(".semio-text-editor-host textarea").first();
const textOf = async (session) => (await editorOf(session).inputValue().catch(() => "")) ?? "";
const linkState = (session) =>
  session.page.evaluate(() => {
    const text = (element) => (element?.textContent ?? "").replace(/\s+/gu, " ").trim();
    const badge = document.querySelector("[data-semio-hub-connection]");
    return {
      badge: badge ? `${badge.getAttribute("data-semio-hub-connection")}|${badge.getAttribute("aria-label") ?? ""}` : null,
      pill: text(document.querySelector('[id="s-sync-status"]')).slice(0, 80),
      notices: [...document.querySelectorAll('[role="alert"], [data-slot="toast"], [data-semio-execution-target-status]')].map((element) => text(element).slice(0, 160)).filter(Boolean).slice(0, 6),
      lang: document.documentElement.lang,
    };
  });
const frameMs = (session) => session.page.evaluate(() => new Promise((resolve) => { const t0 = performance.now(); requestAnimationFrame(() => resolve(performance.now() - t0)); }));
const cut = (control, ms) => fetch(`${control}/cut?mode=close&ms=${ms}`, { method: "POST" }).then((response) => response.json());

async function typeBurst(session, marker) {
  const started = Date.now();
  await editorOf(session).focus();
  await session.page.keyboard.press("End");
  await session.page.keyboard.type(marker, { delay: 35 });
  let echoMs = null;
  while (Date.now() - started < 10_000) {
    if ((await textOf(session)).includes(marker)) { echoMs = Date.now() - started; break; }
    await pause(session, 50);
  }
  return { marker, echoMs, typedMs: Date.now() - started };
}

try {
  await boot(A); await boot(B); await signIn(A); await signIn(B); await pause(A, 10_000);
  await openSpace(A, spaceId); await openSpace(B, spaceId);
  const kind = (await creatableKinds(A)).find((candidate) => candidate.kindId === kindId);
  if (!kind) throw new Error(`kind ${kindId} is not creatable here`);
  const artifactId = await createArtifact(A, `Shortage ${Date.now() % 100000}`, kind);
  await awaitMounted(A, 300_000);
  await openRow(B, spaceId, artifactId);
  await awaitMounted(B, 300_000);
  const base = await typeBurst(A, "base ");
  record("open + baseline", base.echoMs !== null, { artifactId, locales, base, head: await hubHead(artifactId), linkA: await linkState(A), linkB: await linkState(B) });
  const allMarkers = [];
  for (const [round, outageMs] of cutList.split(",").map(Number).entries()) {
    const settleBefore = Date.now();
    while (Date.now() - settleBefore < 20_000 && (await textOf(A)) !== (await textOf(B))) await pause(A, 250);
    const headBefore = await hubHead(artifactId);
    const cuts = [await cut(controlA, outageMs), await cut(controlB, outageMs)];
    const cutAt = Date.now();
    const samples = { A: [], B: [] };
    const states = { A: new Map(), B: new Map() };
    let sampling = true;
    const sampler = (async () => {
      while (sampling) {
        for (const [label, session] of [["A", A], ["B", B]]) {
          const t0 = Date.now();
          const frame = await frameMs(session).catch(() => -1);
          const state = await linkState(session).catch(() => null);
          samples[label].push({ at: Date.now() - cutAt, frameMs: Math.round(frame), readMs: Date.now() - t0 });
          const key = JSON.stringify(state);
          if (!states[label].has(key)) states[label].set(key, Date.now() - cutAt);
        }
        await new Promise((resolve) => setTimeout(resolve, 250));
      }
    })();
    const bursts = [];
    const typingEnd = cutAt + Math.min(outageMs - 1_000, 20_000);
    let index = 0;
    while (Date.now() < typingEnd) {
      if (typists.includes("A")) bursts.push({ who: "A", at: Date.now() - cutAt, ...(await typeBurst(A, `a${round}k${index} `)) });
      if (typists.includes("B")) bursts.push({ who: "B", at: Date.now() - cutAt, ...(await typeBurst(B, `b${round}k${index} `)) });
      index += 1;
    }
    const remaining = cutAt + outageMs - Date.now();
    if (remaining > 0) await pause(A, remaining);
    const restoredAt = Date.now();
    let texts = ["", ""];
    let convergedMs = null;
    while (Date.now() - restoredAt < 120_000) {
      texts = [await textOf(A), await textOf(B)];
      if (texts[0] === texts[1]) { convergedMs = Date.now() - restoredAt; break; }
      await pause(A, 500);
    }
    await pause(A, 3_000);
    sampling = false;
    await sampler;
    texts = [await textOf(A), await textOf(B)];
    const survived = bursts.map((burst) => ({ who: burst.who, marker: burst.marker.trim(), inA: texts[0].includes(burst.marker.trim()), inB: texts[1].includes(burst.marker.trim()), echoMs: burst.echoMs }));
    const worst = (label) => Math.max(...samples[label].map((sample) => sample.frameMs));
    const worstRead = (label) => Math.max(...samples[label].map((sample) => sample.readMs));
    record(`cut ${outageMs} ms: no freeze`, worst("A") < 1_000 && worst("B") < 1_000 && worstRead("A") < 2_000 && worstRead("B") < 2_000, { outageMs, cuts, samples: [samples.A.length, samples.B.length], worstFrameMs: [worst("A"), worst("B")], worstReadMs: [worstRead("A"), worstRead("B")], localEchoMs: bursts.map((burst) => burst.echoMs) });
    record(`cut ${outageMs} ms: link state in each human's language`, true, { A: [...states.A.entries()].map(([state, at]) => ({ at, ...JSON.parse(state) })), B: [...states.B.entries()].map(([state, at]) => ({ at, ...JSON.parse(state) })) });
    record(`cut ${outageMs} ms: convergence`, convergedMs !== null, { convergedMsAfterRestore: convergedMs, a: texts[0].slice(-200), b: texts[1].slice(-200), headBefore, headAfter: await hubHead(artifactId) });
    record(`cut ${outageMs} ms: typed markers kept`, survived.every((row) => row.inA && row.inB), { typists, survived });
    const rebuilds = Object.fromEntries(Object.entries(states).map(([label, map]) => [label, [...map.keys()].filter((key) => rebuildNotice.test(key)).length]));
    record(`cut ${outageMs} ms: no rebuild (the outbox was admitted, nothing rolled back)`, rebuilds.A === 0 && rebuilds.B === 0, { rebuilds });
    allMarkers.push(...survived.map((row) => row.marker));
    await shot(A, tag, `cut${outageMs}`);
    await shot(B, tag, `cut${outageMs}`);
  }
  await B.page.reload({ waitUntil: "domcontentloaded" });
  await openRow(B, spaceId, artifactId);
  await awaitMounted(B, 300_000);
  const reopenedAt = Date.now();
  let reopened = "";
  while (Date.now() - reopenedAt < 60_000) {
    reopened = await textOf(B);
    if (allMarkers.every((marker) => reopened.includes(marker))) break;
    await pause(B, 500);
  }
  record("the hub holds every marker (B reloaded and re-opened the document)", allMarkers.every((marker) => reopened.includes(marker)), { missing: allMarkers.filter((marker) => !reopened.includes(marker)), markers: allMarkers.length, head: await hubHead(artifactId) });
} catch (error) {
  record("probe", false, String(error?.stack ?? error).slice(0, 1500));
  await shot(A, tag, "fail");
  await shot(B, tag, "fail");
} finally {
  save();
  await browser.close();
}
