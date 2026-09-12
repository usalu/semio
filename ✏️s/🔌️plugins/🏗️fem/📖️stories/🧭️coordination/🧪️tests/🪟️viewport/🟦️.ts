import { expect, test } from "vitest";
import Ajv from "ajv/dist/2020.js";
import { applyPatch } from "fast-json-patch";
import orbitSchema from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🪟️viewport/🧊️3d/🧬️schema/🔣️.json";
import rendererGestures from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json";
import { buildFem3dSceneNode, fem2dStoryStateFor, fem3dStoryStateFor, reduceFem2dStoryAction, reduceFem3dStoryAction } from "../../🧫️fixtures/🧫️scene/🟦️.ts";

test("story window ownership admits and echoes the renderer's closed orbit pose", () => {
  const initial = fem3dStoryStateFor("demo", "de-DE");
  const validate = new Ajv({ strict: true }).compile(orbitSchema);
  expect(validate(initial.camera)).toBe(true);
  const camera = rendererGestures.gestures.find((row) => row.id === "orbit-completes-into-one-setcamera")!.expect.camera!;
  const next = reduceFem3dStoryAction(initial, "setCamera", { windowId: rendererGestures.scene.windowInstanceId, camera });
  const independent = applyPatch(structuredClone(initial), [{ op: "replace", path: "/camera", value: camera }], true).newDocument;
  expect(next).toEqual(independent);
  const scene = buildFem3dSceneNode([], next.camera, "model-left", "fem3d-editor");
  expect(JSON.parse(scene.world3d!.cameraJson!)).toEqual(camera);
  expect(next.snapshot).toBe(initial.snapshot);
  for (const invalid of ["{}", { json: "{}" }, { position: [1, 2], target: [0, 0, 0], zoom: 1 }, { ...camera, zoom: 0 }, { ...camera, projection: "orthographic" }]) {
    expect(validate(invalid)).toBe(false);
    expect(() => reduceFem3dStoryAction(initial, "setCamera", { camera: invalid })).toThrow(TypeError);
  }
  console.log("[DEBUG] FEM story camera schema, renderer payload and exact window echo agree with Ajv and fast-json-patch");
});

test("story document replacement preserves window preferences and OS locale", () => {
  for (const dimension of ["2d", "3d"] as const) {
    if (dimension === "2d") {
      const initial = { ...fem2dStoryStateFor("demo", "de-DE"), camera: { x: 4, y: -3, zoom: 2 } };
      const cleared = reduceFem2dStoryAction(initial, "setActiveExample", { exampleId: "cleared" });
      expect(cleared.snapshot.nodes).toHaveLength(0);
      expect(cleared.camera).toBe(initial.camera);
      expect(cleared.resultDisplay).toBe(initial.resultDisplay);
      expect(cleared.locale).toBe("de-DE");
      expect(reduceFem2dStoryAction(initial, "setLocale", { value: "en-US" })).toBe(initial);
      const moved = reduceFem2dStoryAction(initial, "setCamera", { windowId: "model-left", x: 8, y: 9, zoom: 1.5 });
      expect(moved.camera).toEqual({ x: 8, y: 9, zoom: 1.5 });
      expect(reduceFem2dStoryAction(cleared, "setActiveExample", { exampleId: "demo" }).snapshot.nodes.length).toBeGreaterThan(0);
    } else {
      const initial = fem3dStoryStateFor("demo", "de-DE");
      const cleared = reduceFem3dStoryAction(initial, "setActiveExample", { exampleId: "cleared" });
      expect(cleared.snapshot.nodes).toHaveLength(0);
      expect(cleared.camera).toBe(initial.camera);
      expect(cleared.resultDisplay).toBe(initial.resultDisplay);
      expect(cleared.locale).toBe("de-DE");
      expect(reduceFem3dStoryAction(initial, "setLocale", { value: "en-US" })).toBe(initial);
      expect(reduceFem3dStoryAction(cleared, "setActiveExample", { exampleId: "demo" }).snapshot.nodes.length).toBeGreaterThan(0);
    }
  }
  console.log("[DEBUG] FEM story replacement preserves exact window preferences and externally supplied OS locale");
});
