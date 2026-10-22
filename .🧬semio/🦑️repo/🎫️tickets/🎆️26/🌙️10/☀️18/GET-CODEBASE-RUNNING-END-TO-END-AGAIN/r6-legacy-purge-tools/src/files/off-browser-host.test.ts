import { describe, expect, test } from "bun:test";
import { installOffBrowserGlobals } from "../../🟦️.ts";

type Host = { requestAnimationFrame?: unknown; ProgressEvent?: unknown; FileReader?: unknown };

const names = ["requestAnimationFrame", "ProgressEvent", "FileReader"] as const;

function withoutGlobals<T>(run: (host: Host) => T): T {
  const host = globalThis as Host;
  const saved = names.map((name) => [name, host[name]] as const);
  for (const name of names) delete host[name];
  try {
    return run(host);
  } finally {
    for (const [name, value] of saved) host[name] = value;
  }
}

describe("off-browser host", () => {
  test("installs the three globals three.js reaches for when the runtime has none", () => {
    withoutGlobals((host) => {
      installOffBrowserGlobals();
      for (const name of names) expect(typeof host[name]).toBe("function");
    });
  });

  test("never replaces a global the host provides", () => {
    withoutGlobals((host) => {
      const own = () => 1;
      host.FileReader = own;
      installOffBrowserGlobals();
      expect(host.FileReader).toBe(own);
    });
  });

  test("is idempotent", () => {
    withoutGlobals((host) => {
      installOffBrowserGlobals();
      const installed = names.map((name) => host[name]);
      installOffBrowserGlobals();
      expect(names.map((name) => host[name])).toEqual(installed);
    });
  });

  test("runs requestAnimationFrame callbacks synchronously", () => {
    withoutGlobals((host) => {
      installOffBrowserGlobals();
      let ran = 0;
      (host.requestAnimationFrame as (callback: () => void) => number)(() => { ran += 1; });
      expect(ran).toBe(1);
    });
  });

  test("reads a blob as an ArrayBuffer and as a data URL and fires loadend before load", async () => {
    await withoutGlobals(async (host) => {
      installOffBrowserGlobals();
      const Reader = host.FileReader as new () => { result: unknown; onload: (() => void) | null; onloadend: (() => void) | null; readAsArrayBuffer(blob: Blob): void; readAsDataURL(blob: Blob): void };
      const blob = new Blob([new Uint8Array([1, 2, 3])], { type: "application/test" });
      const order: string[] = [];
      const buffer = await new Promise<unknown>((resolve) => {
        const reader = new Reader();
        reader.onloadend = () => order.push("loadend");
        reader.onload = () => { order.push("load"); resolve(reader.result); };
        reader.readAsArrayBuffer(blob);
      });
      expect([...new Uint8Array(buffer as ArrayBuffer)]).toEqual([1, 2, 3]);
      expect(order).toEqual(["loadend", "load"]);
      const url = await new Promise<unknown>((resolve) => {
        const reader = new Reader();
        reader.onload = () => resolve(reader.result);
        reader.readAsDataURL(blob);
      });
      expect(url).toBe(`data:application/test;base64,${Buffer.from([1, 2, 3]).toString("base64")}`);
    });
  });

  test("builds a progress event that carries its fields and is an Event", () => {
    withoutGlobals((host) => {
      installOffBrowserGlobals();
      const Progress = host.ProgressEvent as new (type: string, init?: { lengthComputable?: boolean; loaded?: number; total?: number }) => Event & { loaded: number; total: number; lengthComputable: boolean };
      const event = new Progress("progress", { lengthComputable: true, loaded: 3, total: 9 });
      expect(event).toBeInstanceOf(Event);
      expect([event.type, event.loaded, event.total, event.lengthComputable]).toEqual(["progress", 3, 9, true]);
      expect(new Progress("progress").total).toBe(0);
    });
  });
});
