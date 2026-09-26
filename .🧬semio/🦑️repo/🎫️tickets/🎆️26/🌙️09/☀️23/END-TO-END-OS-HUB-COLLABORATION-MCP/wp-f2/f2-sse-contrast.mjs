#!/usr/bin/env bun
/** ⚖️ F2 — the same stress shape through per-stream HTTP streams, for contrast with `f2-stress.mjs`: a throwaway Bun server on
 * the slice port serves a page, an endless SSE endpoint and a tiny file; Chromium opens N `EventSource`s (what the shell did per
 * watch and per folder-bound document before the stream channel) and then K fetches of the tiny file, each with a 20 s budget.
 *
 * usage: bun f2-sse-contrast.mjs <port> [--streams 64] [--fetches 300] */
import { chromium } from "playwright";

const argv = process.argv.slice(2);
const port = Number(argv[0] ?? "6581");
const valueOf = (flag, fallback) => (argv.includes(flag) ? argv[argv.indexOf(flag) + 1] : fallback);
const streams = Number(valueOf("--streams", "64"));
const fetches = Number(valueOf("--fetches", "300"));
const server = Bun.serve({
  port,
  hostname: "127.0.0.1",
  idleTimeout: 0,
  fetch(request) {
    const path = new URL(request.url).pathname;
    if (path === "/sse") {
      const body = new ReadableStream({ start(controller) { controller.enqueue(new TextEncoder().encode(": connected\n\n")); } });
      return new Response(body, { headers: { "content-type": "text/event-stream", "cache-control": "no-cache" } });
    }
    if (path === "/tiny") return new Response("x", { headers: { "cache-control": "no-store" } });
    return new Response("<!doctype html><title>f2</title>", { headers: { "content-type": "text/html" } });
  },
});
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage();
await page.goto(`http://127.0.0.1:${port}/`);
const result = await page.evaluate(
  async ({ streams, fetches }) => {
    const sources = Array.from({ length: streams }, (_, index) => new EventSource(`/sse?stream=${index}`));
    await new Promise((resolve) => setTimeout(resolve, 2_000));
    const open = sources.filter((source) => source.readyState === EventSource.OPEN).length;
    const outcomes = await Promise.all(
      Array.from({ length: fetches }, async (_, index) => {
        const controller = new AbortController();
        const timer = setTimeout(() => controller.abort(), 20_000);
        const begin = performance.now();
        try {
          await (await fetch(`/tiny?i=${index}`, { signal: controller.signal })).text();
          return performance.now() - begin;
        } catch {
          return null;
        } finally {
          clearTimeout(timer);
        }
      }),
    );
    for (const source of sources) source.close();
    const done = outcomes.filter((value) => value !== null);
    return { streams, eventSourcesOpen: open, fetches, completedWithin20s: done.length, queuedPast20s: outcomes.length - done.length };
  },
  { streams, fetches },
);
console.log(JSON.stringify(result));
await browser.close();
server.stop(true);
