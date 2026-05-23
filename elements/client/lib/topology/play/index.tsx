// #region 🧲Header
// 💻 elements/client/lib/topology/play/index.tsx — Topology play bootstrap: non-React entry that defers to the React runtime module.
// #endregion 🧲Header

import { mountAsyncReactApp } from "@elements/ui";

import nakaginBoardJson from "../../board/play/fixtures/nakagin-capsule-tower.board.json";
import { parseBoardFixtureV1 } from "../../board/index.ts";
import nakaginSceneJson from "../../scene/play/fixtures/nakagin-capsule-tower.scene.json";
import { parseFixtureV1 } from "../../scene/index.tsx";
import { parseTopologyFixtureV1, topologySharedKindsFromPairedMetas } from "../react/index.tsx";
import topologyManifestJson from "./fixtures/nakagin-capsule-tower.topology.json";

void mountAsyncReactApp(async () => (await import("./react.tsx")).createTopologyPlayElement());

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	describe("topology play fixtures", () => {
		it("parses nakagin board and scene", () => {
			const b = parseBoardFixtureV1(nakaginBoardJson as unknown);
			const s = parseFixtureV1(nakaginSceneJson as unknown);
			expect(b?.nodes.length).toBeGreaterThan(0);
			expect(s?.objects.length).toBeGreaterThan(0);
		});
		it("parses topology manifest", () => {
			const t = parseTopologyFixtureV1(topologyManifestJson as unknown);
			expect(t?.schema).toBe("elements.topology.fixture/v1");
		});
		it("shared kinds merge metas like the play harness", () => {
			const sk = topologySharedKindsFromPairedMetas({
				boardMeta: undefined,
				sceneMeta: { kindCompatibility: [{ source: "u", target: "v" }] },
			});
			expect(sk.kindCompatibility?.length).toBeGreaterThan(0);
		});
	});
}