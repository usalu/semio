#!/usr/bin/env bun
/**
 * Lazy-activate probe: hide bridges for 3 plugins, GET module routes, watch SSE, test abort.
 */
import { existsSync, mkdirSync, renameSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const REPO = "/Users/ueli/Documents/semio";
const G = join(REPO, ".tmp-ticket/wp-o2b/generated");
mkdirSync(G, { recursive: true });
const PORT = Number(process.env.O2B_PORT || 6222);
const BASE = `http://127.0.0.1:${PORT}`;

const deployment = await import(`${REPO}/.tmp-ticket/wp-o2b/links/deployment.ts`);
const activation = await import(`${REPO}/.tmp-ticket/wp-o2b/links/activation.ts`);
const catalog = await import(`${REPO}/.tmp-ticket/wp-o2b/links/catalog-view.ts`);

const MODULE_PLUGIN_ROUTE = deployment.MODULE_PLUGIN_ROUTE as string;
const MODULE_BRIDGE_FILE = deployment.MODULE_BRIDGE_FILE as string;
const moduleDirectoryName = deployment.moduleDirectoryName as (id: string) => string;
const moduleRoot = activation.pluginModulesRoot("dev") as string;
const watchPath = `${MODULE_PLUGIN_ROUTE}/watch`;

const entries = catalog.readGeneratedCatalogProjection().entries as { pluginId: string; role: string }[];
// Prefer non-host plugins that currently have a bridge
const host = "space";
const candidates = entries
  .filter((e) => e.pluginId !== host)
  .map((e) => {
    const dir = moduleDirectoryName(e.pluginId);
    const bridge = join(moduleRoot, dir, MODULE_BRIDGE_FILE);
    return { pluginId: e.pluginId, dir, bridge, present: existsSync(bridge) };
  })
  .filter((c) => c.present);

if (candidates.length < 4) throw new Error(`need 4 staged bridges, have ${candidates.length}`);
const selected = candidates.slice(0, 3);
const cancelTarget = candidates[3];

const evidence: Record<string, unknown> = {
  watchPath,
  MODULE_PLUGIN_ROUTE,
  MODULE_BRIDGE_FILE,
  selected: selected.map((s) => s.pluginId),
  cancelTarget: cancelTarget.pluginId,
};

const hidden: { bridge: string; hiddenAs: string }[] = [];
const hide = (bridge: string) => {
  const hiddenAs = `${bridge}.o2b-hidden`;
  renameSync(bridge, hiddenAs);
  hidden.push({ bridge, hiddenAs });
};
const restoreAll = () => {
  for (const h of hidden.splice(0)) {
    if (existsSync(h.hiddenAs) && !existsSync(h.bridge)) renameSync(h.hiddenAs, h.bridge);
  }
};

try {
  for (const s of selected) hide(s.bridge);
  hide(cancelTarget.bridge);

  // SSE client
  const sseAc = new AbortController();
  const sseLines: string[] = [];
  const sseDone = (async () => {
    const res = await fetch(`${BASE}${watchPath}`, { signal: sseAc.signal });
    evidence.sseStatus = res.status;
    if (!res.body) return;
    const reader = res.body.getReader();
    const dec = new TextDecoder();
    let buf = "";
    const deadline = Date.now() + 180_000;
    while (Date.now() < deadline) {
      const { done, value } = await reader.read();
      if (done) break;
      buf += dec.decode(value, { stream: true });
      const parts = buf.split("\n");
      buf = parts.pop() ?? "";
      for (const line of parts) {
        if (line.trim()) sseLines.push(line);
      }
      if (sseLines.some((l) => l.includes("lazy-activate"))) {
        // keep reading a bit more for progress/built
        if (sseLines.filter((l) => l.includes("lazy-activate") || l.includes('"kind":"built"')).length >= 3) break;
      }
    }
  })().catch((e) => {
    evidence.sseError = String(e);
  });

  await new Promise((r) => setTimeout(r, 300));

  const results: { pluginId: string; status: number; bytes: number; secs: number; detail?: string }[] = [];
  for (const s of selected) {
    const url = `${BASE}${MODULE_PLUGIN_ROUTE}/${s.dir}/${MODULE_BRIDGE_FILE}`;
    const t0 = Date.now();
    try {
      const res = await fetch(url, { signal: AbortSignal.timeout(120_000) });
      const buf = await res.arrayBuffer();
      results.push({ pluginId: s.pluginId, status: res.status, bytes: buf.byteLength, secs: (Date.now() - t0) / 1000 });
    } catch (e) {
      results.push({ pluginId: s.pluginId, status: 0, bytes: 0, secs: (Date.now() - t0) / 1000, detail: String(e) });
    }
  }
  evidence.activateResults = results;

  // Cancellation: start fetch for cancelTarget, abort quickly
  const cancelUrl = `${BASE}${MODULE_PLUGIN_ROUTE}/${cancelTarget.dir}/${MODULE_BRIDGE_FILE}`;
  const cancelAc = new AbortController();
  const cancelStarted = Date.now();
  const cancelPromise = fetch(cancelUrl, { signal: cancelAc.signal })
    .then(async (res) => ({ status: res.status, bytes: (await res.arrayBuffer()).byteLength }))
    .catch((e) => ({ error: String(e.name || e) }));
  await new Promise((r) => setTimeout(r, 200));
  cancelAc.abort();
  const cancelResult = await cancelPromise;
  evidence.cancel = { ms: Date.now() - cancelStarted, result: cancelResult };

  await Promise.race([sseDone, new Promise((r) => setTimeout(r, 5000))]);
  sseAc.abort();
  evidence.sseSample = sseLines.slice(0, 40);
  evidence.sseLazyLines = sseLines.filter((l) => /lazy-activate|built/.test(l)).slice(0, 30);

  const pass =
    results.length === 3 &&
    results.every((r) => r.status === 200 && r.bytes > 0) &&
    (String((cancelResult as { error?: string }).error || "").includes("Abort") ||
      (cancelResult as { status?: number }).status === 503);
  evidence.pass = pass;
} finally {
  restoreAll();
}

writeFileSync(join(G, "lazy-activate-evidence.json"), JSON.stringify(evidence, null, 2));
console.log(JSON.stringify(evidence, null, 2));
process.exit(evidence.pass ? 0 : 1);
