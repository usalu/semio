type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { PdfCanvasResourceOwner, pdfCanvasBitmapSize, pdfCanvasStatusAnnouncement } = dependencies;
  type PdfCanvasDocument = any;
  type PdfCanvasLoadingTask = any;
  type PdfCanvasPage = any;
  type PdfCanvasRenderTask = any;

  const { describe, expect, it, vi } = vitest;

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
