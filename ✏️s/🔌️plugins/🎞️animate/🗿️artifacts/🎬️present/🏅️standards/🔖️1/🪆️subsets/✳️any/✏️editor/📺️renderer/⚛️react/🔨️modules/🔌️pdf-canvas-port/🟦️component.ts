// #region 🔌️Contracts
/** @emoji 📐️ Workspace-owned PDF viewport contract. */
export interface PdfCanvasViewport {
  readonly width: number;
  readonly height: number;
}

/** @emoji ⏹️ Workspace-owned cancellable PDF render contract. */
export interface PdfCanvasRenderTask {
  readonly promise: Promise<void>;
  cancel(): void;
}

/** @emoji 📄️ Workspace-owned PDF page contract. */
export interface PdfCanvasPage {
  getViewport(options: { readonly scale: number }): PdfCanvasViewport;
  render(options: { readonly canvas: HTMLCanvasElement; readonly canvasContext: CanvasRenderingContext2D; readonly viewport: PdfCanvasViewport }): PdfCanvasRenderTask;
  cleanup(): void;
}

/** @emoji 📑️ Workspace-owned loaded PDF document contract. */
export interface PdfCanvasDocument {
  readonly numPages: number;
  getPage(pageNumber: number): Promise<PdfCanvasPage>;
  destroy(): void | Promise<void>;
}

/** @emoji ⏳️ Workspace-owned cancellable PDF load contract. */
export interface PdfCanvasLoadingTask {
  readonly promise: Promise<PdfCanvasDocument>;
  destroy(): void | Promise<void>;
}

/** @emoji 🔌️ PDF document loader boundary used by the presentation renderer. */
export interface PdfCanvasPort {
  load(source: string): PdfCanvasLoadingTask;
}

/** @emoji 🚦️ Renderer-owned PDF canvas status. */
export type PdfCanvasStatus = "loading" | "ready" | "error";

/** @emoji 📣️ Accessible loading/error announcement for a PDF canvas status. */
export function pdfCanvasStatusAnnouncement(status: PdfCanvasStatus): { readonly role: "status" | "alert"; readonly text: string } | null {
  if (status === "loading") {
    return { role: "status", text: "…" };
  }
  if (status === "error") {
    return { role: "alert", text: "PDF" };
  }
  return null;
}

/** @emoji 🖼️ Device-pixel bitmap size for a logical PDF viewport. */
export function pdfCanvasBitmapSize(viewport: PdfCanvasViewport, pixelRatio: number): { readonly width: number; readonly height: number } {
  const ratio = Math.max(1, pixelRatio);
  return {
    width: Math.max(1, Math.ceil(viewport.width * ratio)),
    height: Math.max(1, Math.ceil(viewport.height * ratio)),
  };
}
// #endregion 🔌️Contracts

// #region 🧹️Lifecycle
/** @emoji 🧹️ Owns disposal ordering for one PDF canvas lifecycle. */
export class PdfCanvasResourceOwner {
  private loadingTask: PdfCanvasLoadingTask | null = null;
  private document: PdfCanvasDocument | null = null;
  private page: PdfCanvasPage | null = null;
  private renderTask: PdfCanvasRenderTask | null = null;

  beginLoad(task: PdfCanvasLoadingTask): void {
    this.disposeDocument();
    this.loadingTask = task;
  }

  acceptDocument(task: PdfCanvasLoadingTask, document: PdfCanvasDocument): boolean {
    if (this.loadingTask !== task) {
      return false;
    }
    this.loadingTask = null;
    this.document = document;
    return true;
  }

  beginPage(): void {
    this.disposePage();
  }

  acceptPage(page: PdfCanvasPage): void {
    this.page = page;
  }

  acceptRender(task: PdfCanvasRenderTask): void {
    this.renderTask = task;
  }

  disposePage(): void {
    this.renderTask?.cancel();
    this.renderTask = null;
    this.page?.cleanup();
    this.page = null;
  }

  disposeDocument(): void {
    this.disposePage();
    if (this.document) {
      void this.document.destroy();
    } else if (this.loadingTask) {
      void this.loadingTask.destroy();
    }
    this.document = null;
    this.loadingTask = null;
  }
}
// #endregion 🧹️Lifecycle

// #region 🧪️Tests
if (import.meta.vitest) {
  const { describe, expect, it, vi } = import.meta.vitest;

  describe("PdfCanvasResourceOwner", () => {
    it("cancels the render before cleaning the page and destroying the document", () => {
      const calls: string[] = [];
      const owner = new PdfCanvasResourceOwner();
      const loading: PdfCanvasLoadingTask = {
        promise: new Promise(() => undefined),
        destroy: () => calls.push("loading"),
      };
      const document: PdfCanvasDocument = {
        numPages: 1,
        getPage: vi.fn(),
        destroy: () => calls.push("document"),
      };
      const page: PdfCanvasPage = {
        getViewport: () => ({ width: 1, height: 1 }),
        render: vi.fn(),
        cleanup: () => calls.push("page"),
      };
      const render: PdfCanvasRenderTask = {
        promise: new Promise(() => undefined),
        cancel: () => calls.push("render"),
      };
      owner.beginLoad(loading);
      expect(owner.acceptDocument(loading, document)).toBe(true);
      owner.acceptPage(page);
      owner.acceptRender(render);
      owner.disposeDocument();
      expect(calls).toEqual(["render", "page", "document"]);
    });

    it("aborts an unresolved load and rejects documents from superseded loads", () => {
      const destroy = vi.fn();
      const owner = new PdfCanvasResourceOwner();
      const first: PdfCanvasLoadingTask = {
        promise: new Promise(() => undefined),
        destroy,
      };
      const second: PdfCanvasLoadingTask = {
        promise: new Promise(() => undefined),
        destroy: vi.fn(),
      };
      owner.beginLoad(first);
      owner.beginLoad(second);
      expect(destroy).toHaveBeenCalledOnce();
      expect(owner.acceptDocument(first, { numPages: 1, getPage: vi.fn(), destroy: vi.fn() })).toBe(false);
    });
  });

  describe("PDF canvas presentation", () => {
    it("owns accessible loading, ready and error states", () => {
      expect(pdfCanvasStatusAnnouncement("loading")).toEqual({ role: "status", text: "…" });
      expect(pdfCanvasStatusAnnouncement("ready")).toBeNull();
      expect(pdfCanvasStatusAnnouncement("error")).toEqual({ role: "alert", text: "PDF" });
    });

    it("sizes the bitmap for device pixels while preserving positive dimensions", () => {
      expect(pdfCanvasBitmapSize({ width: 595.2, height: 841.8 }, 2)).toEqual({ width: 1191, height: 1684 });
      expect(pdfCanvasBitmapSize({ width: 0, height: 0 }, 0)).toEqual({ width: 1, height: 1 });
    });
  });
}
// #endregion 🧪️Tests
