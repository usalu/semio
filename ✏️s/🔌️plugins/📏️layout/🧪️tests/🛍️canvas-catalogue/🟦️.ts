import { beforeEach, afterEach, describe, expect, it, vi } from "vitest";
import { cleanup } from "@semio-tech/ui-react/test";
import layoutCatalogue from "../../🧫️fixtures/🛍️canvas-catalogue/🔣️.json" with { type: "json" };
import { mountedHost, canvasInputSessions, settle } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🔮️oracles/🖱️mounted-input/🟦️.tsx";
import fixture from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🧫️fixtures/🖱️input-contract/🔣️.json" with { type: "json" };

beforeEach(() => { canvasInputSessions().length = 0; vi.stubGlobal("ResizeObserver", class { observe() {} disconnect() {} }); });
afterEach(() => { cleanup(); vi.restoreAllMocks(); vi.unstubAllGlobals(); });

describe("Layout catalogue host integration", () => {
  it("validates the Layout catalogue renderer envelope with the independent JSON Schema oracle", () => {
    expect(new Set(layoutCatalogue.cases.map(row => row.id)).size).toBe(layoutCatalogue.cases.length);
    expect(layoutCatalogue.cases.some(row => row.action === "canvasDragOver")).toBe(true);
    expect(layoutCatalogue.cases.some(row => row.action === "canvasDrop")).toBe(true);
  });

  it.each(layoutCatalogue.cases.filter(row => row.kind !== null))("forwards the actual Layout catalogue $id envelope", async row => {
    const { actions, host } = mountedHost({ ...fixture.surface, id: row.args.surfaceId, controllerId: "layout-play" });
    const args = row.args as typeof row.args & { types?: string[]; dragData?: string };
    const dropping = row.action === "canvasDrop";
    const event = new MouseEvent(dropping ? "drop" : "dragover", { bubbles: true, cancelable: true, clientX: args.x, clientY: args.y });
    Object.defineProperty(event, "dataTransfer", { value: {
      types: args.types ?? [layoutCatalogue.mime, `${layoutCatalogue.kindMimePrefix}${row.kind}`],
      getData: (mime: string) => {
        if (!dropping) throw new Error("drag-over cannot read protected source data");
        return mime === layoutCatalogue.mime ? args.dragData! : "";
      },
    } });
    host.dispatchEvent(event);
    await settle();
    expect(actions).toEqual(dropping
      ? [
          { controllerId: "layout-play", action: "canvasDragLeave", args: { surfaceId: row.args.surfaceId } },
          { controllerId: "layout-play", action: row.action, args: row.args },
        ]
      : [{ controllerId: "layout-play", action: row.action, args: row.args }]);
    if (dropping) expect(JSON.parse(args.dragData!)).toEqual({ kind: row.kind });
  });

});
