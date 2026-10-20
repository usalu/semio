/** 🔸️ The carries one mirror activation tests of the wgpu accessibility interaction contract suite. */
import { fixtureOriginalWorkStock } from "@semio-tech/framework-actor/retained-turn/bootstrap/testing/fixtures";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { expect, it, vi } from "vitest";
import { createAccessibilityMirror, type AccessibilityProjectionWindow } from "@semio-tech/framework-renderer-wgpu/accessibility-mirror";
import { BrowserFrameTransport } from "@semio-tech/framework-renderer-wgpu/browser-frame-transport";
import { resolveWgpuBootDescriptor } from "@semio-tech/framework-renderer-wgpu/boot-descriptor";
import { AccessibilityWorker, computeAccessibleDescription, computeAccessibleName, fixture, getRole } from "../🟣️shared/🟦️.tsx";
import type { WireEvent } from "../🟣️shared/🟦️.tsx";

/** 🔹 Registers the carries one mirror activation tests of the wgpu accessibility interaction contract suite. */
export function registerCarriesOneMirrorActivationTests(): void {
  it("carries one mirror activation through the real transport and replaces it with the next accepted epoch", async () => {
    vi.useFakeTimers();
    const root = document.createElement("div");
    document.body.append(root);
    const worker = new AccessibilityWorker();
    const frameTransport = new BrowserFrameTransport({ originalWorkStock: fixtureOriginalWorkStock(),
      worker,
      boot: {
        bindingsModuleUrl: "renderer.js",
        bindingsWasmUrl: "renderer_bg.wasm",
        canvas: {} as OffscreenCanvas,
        width: 800,
        height: 600,
        dpr: 1,
        locale: "en",
        descriptor: resolveWgpuBootDescriptor({ defaultVariant: "s" }),
        appearance: { preference: "", systemDark: false },
        platform: "MacIntel",
        storage: {},
        catalog: { version: 1 as const, targets: [], hosts: [], playgrounds: [] },
      },
      setTimer: () => 1,
      clearTimer: () => {},
    });
    worker.reply({ kind: "booted", lifecycle: 1 });
    const chrome = fixture.presentedChrome;
    let generation = chrome.firstEpoch;
    let displayChecked = false;
    const projection = (): AccessibilityProjectionWindow => ({
      windowId: "shell.chrome",
      windowGeneration: generation,
      nodes: chrome.controls.map((control, index) => ({ nodeId: index + 1, key: control.id, role: "switch", depth: 0, label: control.label, live: "off", focusable: true, tabbable: true, actionable: true, checked: control.id === "ui.panelToggle.display" && displayChecked })),
    });
    const mirror = createAccessibilityMirror(root, {
      introspect: async () => JSON.stringify({ windows: [projection()] }),
      enqueueLossless: (event) => frameTransport.enqueueLossless(event),
    }, "en");
    mirror.refresh();
    await vi.advanceTimersByTimeAsync(400);
    const staleDisplay = root.querySelector<HTMLButtonElement>('[data-node-key="ui.panelToggle.display"]')!;
    staleDisplay.click();
    expect(frameTransport.flush(1)).toBe(true);
    const batch = worker.messages.at(-1);
    if (batch?.kind !== "batch") throw new Error("accessibility activation batch");
    expect(batch.lossless).toEqual([expect.objectContaining({ kind: "accessibility-activate", windowGeneration: chrome.firstEpoch, nodeKey: "ui.panelToggle.display" })]);
    worker.reply({ kind: "batch-accepted", lifecycle: 1, inputSequence: batch.inputSequence, generation: batch.generation });
    displayChecked = true;
    generation = chrome.successorEpoch;
    mirror.refresh();
    await vi.advanceTimersByTimeAsync(400);
    expect(root.querySelector('[data-node-key="ui.panelToggle.display"]')?.getAttribute("aria-checked")).toBe("true");
    root.querySelector<HTMLButtonElement>('[data-node-key="ui.panelToggle.settings"]')!.click();
    expect(frameTransport.flush(2)).toBe(true);
    const successor = worker.messages.at(-1);
    if (successor?.kind !== "batch") throw new Error("successor accessibility activation batch");
    expect(successor.lossless).toEqual([expect.objectContaining({ kind: "accessibility-activate", windowGeneration: chrome.successorEpoch, nodeKey: "ui.panelToggle.settings" })]);
    staleDisplay.click();
    worker.reply({ kind: "batch-accepted", lifecycle: 1, inputSequence: successor.inputSequence, generation: successor.generation });
    expect(frameTransport.flush(3)).toBe(true);
    const stale = worker.messages.at(-1);
    if (stale?.kind !== "batch") throw new Error("stale accessibility activation batch");
    expect(stale.lossless).toEqual([expect.objectContaining({ kind: "accessibility-activate", windowGeneration: chrome.firstEpoch })]);
    mirror.dispose();
    frameTransport.close();
  });
  it("mounts the production mirror with sibling descriptions, semantic states, and paragraph text", async () => {
    vi.useFakeTimers();
    const root = document.createElement("div");
    document.body.append(root);
    const sent: WireEvent[] = [];
    let introspections = 0;
    const json = JSON.stringify({ windows: [fixture.mirrorProjection] });
    const mirror = createAccessibilityMirror(root, {
      introspect: async () => { introspections++; return json; },
      enqueueLossless: (event) => { sent.push(event as WireEvent); return true; },
    }, "en");
    mirror.refresh();
    await vi.advanceTimersByTimeAsync(400);
    const input = root.querySelector<HTMLInputElement>('[data-node-key="settings.iterations"]')!;
    const description = document.getElementById(input.getAttribute("aria-describedby")!)!;
    expect(input.tagName).toBe("INPUT");
    expect(description.parentElement).toBe(input.parentElement);
    expect(input.contains(description)).toBe(false);
    expect(computeAccessibleName(input)).toBe("Iterations");
    expect(computeAccessibleDescription(input)).toBe("Whole-number iterations");
    expect(root.querySelector<HTMLElement>('[data-node-key="settings.status"]')?.textContent).toBe("Ready");
    for (const key of ["framework.settings", "framework.settings.general"]) {
      const button = root.querySelector<HTMLElement>(`[data-node-key="${key}"]`)!;
      expect(getRole(button)).toBe(fixture.settingsTabStrip.role);
      expect(button.getAttribute(fixture.settingsTabStrip.stateAttribute)).toBe("true");
      expect(button.hasAttribute("aria-selected")).toBe(false);
      expect(button.hasAttribute("aria-checked")).toBe(false);
    }
    expect(document.activeElement?.getAttribute("data-node-key")).toBe("framework.settings.general");
    mirror.refresh();
    await vi.advanceTimersByTimeAsync(400);
    expect(introspections).toBe(2);
    expect(root.querySelectorAll("[data-node-key]")).toHaveLength(fixture.mirrorProjection.nodes.length);
    expect(sent).toEqual([]);
    mirror.dispose();
  });
  it("mirrors every transient notice as the polite status React renders, named by its message and described by its code", async () => {
    type NoticeFixture = { readonly node: { readonly key: string; readonly role: string; readonly live: string }; readonly notices: readonly { readonly code: string | null; readonly message: string }[] };
    const notices = JSON.parse(readFileSync(join(dirname(fileURLToPath(import.meta.url)), "../../🧫️fixtures/🧯️wgpu-transient-notice/🔣️.json"), "utf8")) as NoticeFixture;
    vi.useFakeTimers();
    for (const notice of notices.notices) {
      const root = document.createElement("div");
      document.body.append(root);
      const sent: WireEvent[] = [];
      const node = { nodeId: 1, key: notices.node.key, role: notices.node.role, depth: 0, label: notice.message, ...(notice.code === null ? {} : { description: notice.code }), live: notices.node.live, focusable: false, tabbable: false, actionable: false };
      const json = JSON.stringify({ windows: [{ windowId: "shell.chrome", windowGeneration: 1, nodes: [node] }] });
      const mirror = createAccessibilityMirror(root, { introspect: async () => json, enqueueLossless: (event) => { sent.push(event as WireEvent); return true; } }, "en");
      mirror.refresh();
      await vi.advanceTimersByTimeAsync(400);
      const status = root.querySelector<HTMLElement>(`[data-node-key="${notices.node.key}"]`)!;
      expect([getRole(status), status.getAttribute("aria-live"), computeAccessibleName(status), computeAccessibleDescription(status), status.tabIndex], notice.message).toEqual([notices.node.role, notices.node.live, notice.message, notice.code ?? "", -1]);
      status.click();
      expect(sent, "a status is announced, never activated").toEqual([]);
      mirror.dispose();
      root.remove();
    }
  });
  it("mounts an open retained Select as a read-only combobox with live listbox options", async () => {
    vi.useFakeTimers();
    const root = document.createElement("div");
    document.body.append(root);
    const sent: WireEvent[] = [];
    const nodes = [
      { nodeId: 11, key: "appearance", role: "combobox", depth: 0, label: "Appearance", live: "off", focusable: true, tabbable: true, actionable: true, expanded: true, valueText: "system" },
      { nodeId: 11, key: "appearance::listbox", role: "listbox", depth: 0, label: "Appearance", live: "off" },
      { nodeId: 11, key: "appearance::option::system", role: "option", depth: 1, label: "System", live: "off", actionable: true, selected: true },
      { nodeId: 11, key: "appearance::option::dark", role: "option", depth: 1, label: "Dark", live: "off", actionable: true, selected: false },
    ];
    const mirror = createAccessibilityMirror(root, {
      introspect: async () => JSON.stringify({ windows: [{ windowId: "settings", windowGeneration: 4, nodes }] }),
      enqueueLossless: (event) => { sent.push(event as WireEvent); return true; },
    }, "en");
    mirror.refresh();
    await vi.advanceTimersByTimeAsync(400);
    const combobox = root.querySelector<HTMLElement>('[role="combobox"]')!;
    const listbox = root.querySelector<HTMLElement>('[role="listbox"]')!;
    const options = Array.from(root.querySelectorAll<HTMLElement>('[role="option"]'));
    expect(combobox.tagName).toBe("BUTTON");
    expect(combobox).not.toBeInstanceOf(HTMLInputElement);
    expect(computeAccessibleName(combobox)).toBe("Appearance");
    expect(listbox.parentElement).toBe(combobox.parentElement);
    expect(options.every((option) => option.parentElement === listbox)).toBe(true);
    options[1]!.click();
    expect(sent.filter((event) => event.kind === "accessibility-activate")).toEqual([{ kind: "accessibility-activate", windowId: "settings", windowGeneration: 4, nodeId: 11, nodeKey: "appearance::option::dark" }]);
    mirror.dispose();
  });
}
