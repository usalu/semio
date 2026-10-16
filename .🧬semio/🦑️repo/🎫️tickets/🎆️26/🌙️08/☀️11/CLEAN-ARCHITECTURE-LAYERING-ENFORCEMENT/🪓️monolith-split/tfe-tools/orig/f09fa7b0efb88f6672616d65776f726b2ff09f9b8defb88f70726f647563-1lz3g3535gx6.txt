/** 🛡 Runtime diagnostics switch, descriptor load admission, tutorial document wire and plugin session ownership. */
import { RUNTIME_DIAGNOSTICS_KEY, runtimeDiagnosticsEnabled, setRuntimeDiagnostics } from "../../🧱️elements/🏛️ShellHost/🟦️.tsx";
import { SHARD_RUNTIME_DIAGNOSTICS_KEY, SHARD_WORKER_DIAGNOSTICS_PARAM, SHARD_WORKER_URL, shardWorkerUrl } from "../../../../../../../🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts";
import descriptorLoadFixture from "../../../../../../../🔨️modules/🎠️kernel/🧪️tests/🧫️fixtures/📇️descriptor-load/🔣️.json";
import pluginAvailabilityRouteFixture from "../../🧱️elements/🛠️ShellHelpers/🧪️tests/🧫️fixtures/🔁️plugin-availability-route/🔣️.json";
import { stubFetch } from "../../../../../🧪️tests/🌐️fetch-stub/🟦️.ts";
import tutorialDocumentFixture from "../../../../../../../🔨️modules/🎓️onboarding/🧪️tests/🧫️fixtures/🎞️tutorial-document-track.json";
import { tutorialSlice, validateTutorial } from "@semio-tech/ui-react";
import type { TutorialDefinition } from "@semio-tech/framework";
import { afterEach, describe, expect, it, vi } from "vitest";
import { fetchDescriptorManifest, resolveDescriptorBeforeRuntime, pluginAvailabilityRouteV1, pluginShouldReceiveContributions, pluginShouldEstablishSession } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { readFileSync, existsSync } from "node:fs";
import "../🧰️engine-contract-helpers/🟦️.ts";

//#region 🩺️RuntimeDiagnostics
/** 🩺️ The module that GENERATES the shard worker, read as text. `import.meta.url` is not a file URL
 * under this suite's transform and the module itself cannot be imported here (it reaches the repo
 * library's `bun:sqlite` lease store, which a jsdom bundle refuses), so it is located by walking up
 * from the runner's cwd — the same shape the world3d-host law above uses. */
function readGeneratedShardWorkerOwnerSource(): string {
  const relative = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
  let root = process.cwd();
  for (let hop = 0; hop < 12 && !existsSync(`${root}/${relative}`); hop += 1) root = `${root}/..`;
  expect(existsSync(`${root}/${relative}`)).toBe(true);
  return readFileSync(`${root}/${relative}`, "utf8");
}

describe("shell runtime diagnostics switch", () => {
  afterEach(() => {
    setRuntimeDiagnostics(undefined);
  });

  it("is off by default and names the same key the guest switch does", () => {
    // 🪞️ `RUNTIME_DIAGNOSTICS_ENV` in `🧰️framework/🔨️modules/⏱️trace/🦀️.rs` — one name, two languages.
    expect(RUNTIME_DIAGNOSTICS_KEY).toBe("SEMIO_RUNTIME_DIAGNOSTICS");
    expect(runtimeDiagnosticsEnabled()).toBe(false);
  });

  // ⚖️ LAW: the key is spelled once per realm that has to resolve it — the shell, the shard-worker
  // bootstrap, and the generated worker/host-shim source — and all of them must agree, because the
  // guest's own `RUNTIME_DIAGNOSTICS_ENV` is the only reader and it matches on the exact string
  // (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️audit-guest-tick-cost-2026-09-12.md` §4 rank 1).
  it("spells the same diagnostics key in every realm that resolves it", () => {
    expect(SHARD_RUNTIME_DIAGNOSTICS_KEY).toBe(RUNTIME_DIAGNOSTICS_KEY);
    // 🔌️ The generated shard worker's own copy lives in `🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`,
    // which this jsdom suite cannot import (it reaches the repo library's `bun:sqlite` lease store),
    // so it is read as text — the same shape `⏱️wgpu-ui-turn-budget` uses for `ShellHost`.
    const pluginPackage = readGeneratedShardWorkerOwnerSource();
    expect(pluginPackage).toContain(`export const SHARD_RUNTIME_DIAGNOSTICS_KEY = "${RUNTIME_DIAGNOSTICS_KEY}"`);
    expect(pluginPackage).toContain(`export const SHARD_WORKER_DIAGNOSTICS_PARAM = "${SHARD_WORKER_DIAGNOSTICS_PARAM}"`);
  });

  // ⚖️ LAW: an unarmed page boots the shard worker at the bare url, and an armed one stamps the
  // parameter the worker reads back — the ONLY channel the switch has into a Worker realm, which
  // owns no `localStorage`. The generated worker must read that same parameter and seed
  // `wasi:cli/environment` from it, which is what makes the guest's `[TRACE]` sites reachable.
  it("carries the armed switch into the worker realm on the worker url", () => {
    const stored = new Map<string, string>();
    const original = globalThis.localStorage;
    Object.defineProperty(globalThis, "localStorage", {
      configurable: true,
      value: { getItem: (key: string) => stored.get(key) ?? null, setItem: (key: string, value: string) => void stored.set(key, value), removeItem: (key: string) => void stored.delete(key) },
    });
    try {
      expect(shardWorkerUrl()).toBe(SHARD_WORKER_URL);
      stored.set(SHARD_RUNTIME_DIAGNOSTICS_KEY, "1");
      expect(shardWorkerUrl()).toBe(`${SHARD_WORKER_URL}?${SHARD_WORKER_DIAGNOSTICS_PARAM}=1`);
    } finally {
      if (original === undefined) Reflect.deleteProperty(globalThis, "localStorage");
      else Object.defineProperty(globalThis, "localStorage", { configurable: true, value: original });
    }
    const pluginPackage = readGeneratedShardWorkerOwnerSource();
    expect(pluginPackage).toContain('searchParams.get("${SHARD_WORKER_DIAGNOSTICS_PARAM}")');
    expect(pluginPackage).toContain('_setEnv({ "${SHARD_RUNTIME_DIAGNOSTICS_KEY}": "1" })');
    expect(pluginPackage.indexOf("await armGuestRuntimeDiagnostics()")).toBeLessThan(pluginPackage.indexOf("const bridge = await import("));
  });

  it("an explicit host override outranks every resolved source, in both directions", () => {
    setRuntimeDiagnostics(true);
    expect(runtimeDiagnosticsEnabled()).toBe(true);
    setRuntimeDiagnostics(false);
    expect(runtimeDiagnosticsEnabled()).toBe(false);
    setRuntimeDiagnostics(undefined);
    expect(runtimeDiagnosticsEnabled()).toBe(false);
  });
});
//#endregion 🩺️RuntimeDiagnostics


//#region 📇️DescriptorAdmission
describe("descriptor load admission", () => {
  

  it.each(descriptorLoadFixture.rejected)("refuses $name before starting the actor runtime", async (vector) => {
    const originalFetch = globalThis.fetch;
    const initialize = vi.fn();
    const fetch = vi.fn(async () => new Response(vector.body, { status: vector.status, headers: { "content-type": vector.contentType } }));
    globalThis.fetch = stubFetch(fetch);
    try {
      await expect(resolveDescriptorBeforeRuntime(() => fetchDescriptorManifest(descriptorLoadFixture.pluginId, descriptorLoadFixture.moduleUrl), initialize)).rejects.toMatchObject({
        fault: { code: vector.fault, scope: { pluginId: descriptorLoadFixture.pluginId } },
      });
      expect(fetch).toHaveBeenCalledExactlyOnceWith(descriptorLoadFixture.descriptorUrl, undefined);
      expect(initialize).not.toHaveBeenCalled();
    } finally {
      globalThis.fetch = originalFetch;
    }
  });

  it("admits an actual descriptor including an extension's empty app roster", async () => {
    const originalFetch = globalThis.fetch;
    try {
      for (const apps of [descriptorLoadFixture.descriptor.manifest.apps, []]) {
        const descriptor = { manifest: { ...descriptorLoadFixture.descriptor.manifest, apps } };
        globalThis.fetch = stubFetch(async () => new Response(JSON.stringify(descriptor), { headers: { "content-type": "application/json" } }));
        expect(await fetchDescriptorManifest(descriptorLoadFixture.pluginId, descriptorLoadFixture.moduleUrl)).toEqual(descriptor.manifest);
      }
    } finally {
      globalThis.fetch = originalFetch;
    }
  });
});
//#endregion 📇️DescriptorAdmission


//#region 🎞️TutorialDocumentTrack
describe("tutorial document wire contract", () => {
  it("keeps native document-track names and bidirectional event order", () => {
    
    
    
    const document = tutorialDocumentFixture.document.map(({ at, kind }) => ({ at, kind: { ...kind, kind: "load" as const } }));
    const definition: TutorialDefinition = {
      id: "document-wire",
      title: "Document Wire",
      durationMs: 250,
      chapters: [],
      base: { ui: { activeUtilityByWindowId: {}, activePanelTabByGroup: {}, interactionSelection: {}, expandedTreeIds: [], commandPanelOpen: false }, cameras: [] },
      tracks: { narration: [], video: [], events: [], ui: [], document, camera: [], gestures: [] },
    };
    expect(validateTutorial(definition)).toBeNull();
    for (const vector of tutorialDocumentFixture.cases) {
      const slice = tutorialSlice(definition, vector.from, vector.to);
      expect(slice.forward).toBe(vector.forward);
      expect(slice.document.map(({ at }) => at)).toEqual(vector.expectedAt);
      expect(slice.document).toEqual(vector.expectedAt.map((at) => document.find((event) => event.at === at)));
    }
  });
});
//#endregion 🎞️TutorialDocumentTrack

//#region 🔌️PluginSessionOwnership
describe("plugin session ownership", () => {
  it("lets only the configured primary plugin establish a missing session", () => {
    expect(pluginShouldEstablishSession("demonstrator", "demonstrator", false)).toBe(true);
    expect(pluginShouldEstablishSession("cad", "demonstrator", false)).toBe(false);
    expect(pluginShouldEstablishSession("demonstrator", "demonstrator", true)).toBe(false);
  });

  it("configures only the active aggregate outside studio mode", () => {
    expect(pluginShouldReceiveContributions("demonstrator", "demonstrator", false)).toBe(true);
    expect(pluginShouldReceiveContributions("cad", "demonstrator", false)).toBe(false);
    expect(pluginShouldReceiveContributions("cad", "demonstrator", true)).toBe(true);
  });

  it("never hot-swaps a live plugin for a replayed availability snapshot", () => {
    for (const row of pluginAvailabilityRouteFixture.rows) {
      expect(pluginAvailabilityRouteV1(row.alreadyLoaded, row.loadedRebuiltAt ?? undefined, row.eventRebuiltAt ?? undefined), row.id).toBe(row.route);
    }
    const loaded = new Map<string, number | undefined>();
    const tally = { install: 0, "hot-swap": 0, drop: 0 };
    for (let connect = 0; connect < pluginAvailabilityRouteFixture.replay.connects; connect += 1) {
      for (const plugin of pluginAvailabilityRouteFixture.replay.snapshot) {
        const route = pluginAvailabilityRouteV1(loaded.has(plugin.pluginId), loaded.get(plugin.pluginId), plugin.rebuiltAt);
        tally[route] += 1;
        if (route !== "drop") loaded.set(plugin.pluginId, plugin.rebuiltAt);
      }
    }
    expect(tally.install).toBe(pluginAvailabilityRouteFixture.replay.expected.installs);
    expect(tally["hot-swap"]).toBe(pluginAvailabilityRouteFixture.replay.expected.hotSwaps);
    expect(tally.drop).toBe(pluginAvailabilityRouteFixture.replay.expected.drops);
  });
});
//#endregion 🔌️PluginSessionOwnership
