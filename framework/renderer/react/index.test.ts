import { describe, expect, it } from "vitest";
import type { UiNode } from "./types.ts";

describe("framework renderer types", () => {
	it("accepts component scene nodes", () => {
		const node: UiNode = {
			type: "componentScene",
			surfaceId: "draw.play.composite",
			controllerId: "draw-play",
			componentKind: "canvas-2d",
			canvas2d: {
				cameraX: 0,
				cameraY: 0,
				zoom: 1,
				layersJson: "[]",
			},
		};
		expect(node.componentKind).toBe("canvas-2d");
	});
});
