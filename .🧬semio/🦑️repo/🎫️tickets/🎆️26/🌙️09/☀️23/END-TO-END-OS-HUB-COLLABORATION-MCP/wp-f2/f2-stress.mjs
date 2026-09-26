#!/usr/bin/env bun
/** 🏋️ F2 — stress of the ONE stream channel: in a booted `s` page, opens N `backbone.folder` streams (distinct folders under the
 * slice's data root) and M `plugin-modules.activation` jobs on the page's own channel (the same module instance the shell uses),
 * fires a burst of K same-origin fetches meanwhile, then touches every watched folder from outside and counts the change
 * notices that arrive, cancels everything and reads the channel census. The NetLog answers what held HTTP/1.1 connections.
 *
 * usage: bun f2-stress.mjs <baseUrl> <tag> [--streams 64] [--jobs 8] [--fetches 300] */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { settleMain, sleep } from "../wp-f1/f1-lib.mjs";

const argv = process.argv.slice(2);
const baseUrl = argv[0] ?? "http://127.0.0.1:6580/";
const tag = argv[1] ?? "stress";
const valueOf = (flag, fallback) => (argv.includes(flag) ? argv[argv.indexOf(flag) + 1] : fallback);
const streams = Number(valueOf("--streams", "64"));
const jobs = Number(valueOf("--jobs", "8"));
const fetches = Number(valueOf("--fetches", "300"));
const generated = fileURLToPath(new URL("./generated/", import.meta.url));
const netlog = `${generated}f2-netlog-${tag}.json`;
const root = "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-f2-space/stress";
const muxModule = "/@fs/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🚪️io/🔀️stream-mux/🟦️.ts";
const log = (...parts) => console.log(`[f2-stress ${new Date().toISOString().slice(11, 19)}]`, ...parts);
const sweep = await import("/Users/ueli/Documents/semio/.tmp-ticket-0918/🐍️s6-all-kinds-sweep.mjs");

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", `--log-net-log=${netlog}`, "--net-log-capture-mode=Default"] });
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
page.setDefaultNavigationTimeout(300_000);
const cdp = await page.context().newCDPSession(page);
await cdp.send("Performance.enable", { timeDomain: "timeTicks" });
await page.goto(baseUrl, { waitUntil: "commit" });
log("beacon", await sweep.awaitBeacon(page, Date.now() + 300_000));
await sweep.dismissIntroduction(page);
await settleMain(cdp, 20_000);

const folders = Array.from({ length: streams }, (_, index) => join(root, `folder-${String(index).padStart(3, "0")}`));
for (const folder of folders) mkdirSync(join(folder, ".semio"), { recursive: true });
const pluginIds = await page.evaluate(() => (window.__semioOsCatalogProbe?.plugins ?? []).map((row) => row.pluginId));
const opened = await page.evaluate(
  async ({ muxModule, folders, jobIds, fetches }) => {
    const mux = await import(muxModule);
    const channel = mux.pageStreamMuxChannelV1("/semio-stream-mux");
    const state = { folders: {}, jobs: {}, fetch: [] };
    window.__f2Stress = { channel, state, subscriptions: [] };
    for (const folder of folders) {
      const row = (state.folders[folder] = { opened: [], notices: 0, end: null });
      window.__f2Stress.subscriptions.push(channel.open("backbone.folder", `folder://${folder}`, { opened: (mode) => row.opened.push(mode), data: () => void (row.notices += 1), end: (reason, detail) => (row.end = `${reason}:${detail}`) }));
    }
    for (const pluginId of jobIds) {
      const row = (state.jobs[pluginId] = { opened: [], progress: 0, end: null });
      window.__f2Stress.subscriptions.push(channel.open("plugin-modules.activation", pluginId, { opened: (mode) => row.opened.push(mode), progress: () => void (row.progress += 1), end: (reason, detail) => (row.end = `${reason}:${detail}`) }));
    }
    const started = performance.now();
    const results = await Promise.all(
      Array.from({ length: fetches }, async (_, index) => {
        const begin = performance.now();
        const response = await fetch(`/favicon.ico?f2-stress=${index}`, { cache: "no-store" });
        await response.arrayBuffer();
        return { status: response.status, ms: performance.now() - begin };
      }),
    );
    const sorted = results.map((row) => row.ms).sort((a, b) => a - b);
    state.fetch = { count: results.length, ok: results.filter((row) => row.status === 200).length, p50: +sorted[Math.floor(sorted.length / 2)].toFixed(1), p95: +sorted[Math.floor(sorted.length * 0.95)].toFixed(1), max: +sorted.at(-1).toFixed(1), wallMs: +(performance.now() - started).toFixed(1) };
    await new Promise((resolve) => setTimeout(resolve, 3_000));
    return { census: channel.census(), fetch: state.fetch };
  },
  { muxModule, folders, jobIds: pluginIds.slice(0, jobs), fetches },
);
log("opened", JSON.stringify(opened));
for (const folder of folders) writeFileSync(join(folder, ".semio", "touch.json"), JSON.stringify({ at: Date.now() }));
await sleep(3_000);
for (const folder of folders.filter((_, index) => index % 2 === 0)) writeFileSync(join(folder, ".semio", "touch.json"), JSON.stringify({ at: Date.now() }));
await sleep(3_000);
const observed = await page.evaluate(() => {
  const { state } = window.__f2Stress;
  const folderRows = Object.values(state.folders);
  return {
    folders: folderRows.length,
    freshOpens: folderRows.filter((row) => row.opened.includes("fresh")).length,
    notices: folderRows.map((row) => row.notices),
    folderEnds: folderRows.filter((row) => row.end !== null).map((row) => row.end),
    jobs: state.jobs,
  };
});
const noticeTotal = observed.notices.reduce((sum, value) => sum + value, 0);
log("observed", JSON.stringify({ folders: observed.folders, freshOpens: observed.freshOpens, noticesTotal: noticeTotal, withAtLeastOne: observed.notices.filter((value) => value >= 1).length, withTwo: observed.notices.filter((value) => value >= 2).length, folderEnds: observed.folderEnds.slice(0, 4), jobs: observed.jobs }));
const closed = await page.evaluate(async () => {
  for (const subscription of window.__f2Stress.subscriptions) subscription.close();
  await new Promise((resolve) => setTimeout(resolve, 1_000));
  return window.__f2Stress.channel.census();
});
log("after close", JSON.stringify(closed));
await browser.close();
const out = `${generated}f2-stress-${tag}.json`;
writeFileSync(out, JSON.stringify({ baseUrl, tag, streams, jobs, fetches, opened, observed, closed, netlog }, null, 1));
log("wrote", out);
