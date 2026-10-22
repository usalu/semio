class OffBrowserProgressEvent extends Event {
  readonly lengthComputable: boolean;
  readonly loaded: number;
  readonly total: number;
  constructor(type: string, init: { lengthComputable?: boolean; loaded?: number; total?: number } = {}) {
    super(type);
    this.lengthComputable = init.lengthComputable ?? false;
    this.loaded = init.loaded ?? 0;
    this.total = init.total ?? 0;
  }
}

class OffBrowserFileReader {
  result: string | ArrayBuffer | null = null;
  onload: (() => void) | null = null;
  onloadend: (() => void) | null = null;
  onerror: ((error: unknown) => void) | null = null;
  readAsArrayBuffer(blob: Blob): void {
    blob.arrayBuffer().then((buffer) => this.finish(buffer), (error) => this.onerror?.(error));
  }
  readAsDataURL(blob: Blob): void {
    blob.arrayBuffer().then((buffer) => this.finish(`data:${blob.type || "application/octet-stream"};base64,${Buffer.from(buffer).toString("base64")}`), (error) => this.onerror?.(error));
  }
  private finish(result: string | ArrayBuffer): void {
    this.result = result;
    this.onloadend?.();
    this.onload?.();
  }
}

/** 🌐️ Off-browser host of the three.js oracles: gives `globalThis` the `requestAnimationFrame`, `ProgressEvent` and `FileReader` that `PLYExporter`, `GLTFExporter` and `GLTFLoader` reach for when the runtime has none, and never replaces a host-provided one. `GLTFExporter` assigns `onloadend` rather than `onload` and `GLTFLoader` dispatches a `ProgressEvent`; a host missing either does not throw, the completion callback never fires and the run hangs, so a caller that waits on them also carries a watchdog. */
export function installOffBrowserGlobals(): void {
  const host = globalThis as { requestAnimationFrame?: unknown; ProgressEvent?: unknown; FileReader?: unknown };
  host.requestAnimationFrame ??= (callback: () => void): number => {
    callback();
    return 0;
  };
  host.ProgressEvent ??= OffBrowserProgressEvent;
  host.FileReader ??= OffBrowserFileReader;
}
