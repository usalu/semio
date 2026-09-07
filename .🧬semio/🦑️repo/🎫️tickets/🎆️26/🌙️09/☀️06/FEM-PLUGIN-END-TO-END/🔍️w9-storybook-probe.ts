// #region 🧲️Header
// 💻️ 🔍️w9-storybook-probe.ts
// Specs: W9 verification probe for the generated `🏗️fem` Storybook scope — runs the story-local DSL reader
// and scene projections against the REAL shipped example documents, with no Storybook/Vite/cargo involved.
// Summary: `bun .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️06/FEM-PLUGIN-END-TO-END/🔍️w9-storybook-probe.ts`
// from the repo root. Imports only `.storybook/stories/fem/{dsl,scene}.ts` (their single framework import
// is `import type`, so nothing of the renderer module graph is loaded) and prints the parsed document
// shape plus the projected `canvas-2d` layers / `world-3d` instances for both artifacts.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import { parseFem2dDsl, parseFem3dDsl } from "../../../../../../../.storybook/stories/fem/dsl.ts";
import {
  buildFem2dSceneNode,
  buildFem3dSceneNode,
  fem2dStructureLayers,
  fem2dStoryStateFor,
  fem2dSummaryLines,
  fem3dStoryStateFor,
  fem3dStructuralInstances,
  fem3dSummaryLines,
  femNextId,
  reduceFem2dStoryAction,
  reduceFem3dStoryAction,
  FEM2D_DEFAULT_CAMERA,
  FEM2D_DEMO_SNAPSHOT,
  FEM2D_STORY_EXAMPLE_ID,
  FEM3D_DEFAULT_CAMERA_JSON,
  FEM3D_DEMO_SNAPSHOT,
  FEM3D_STORY_EXAMPLE_ID,
  FEM3D_STORY_SHIPPED_EXAMPLE_ID,
} from "../../../../../../../.storybook/stories/fem/scene.ts";

const line = (label: string, value: unknown): void => console.log(label, typeof value === "string" ? value : JSON.stringify(value));

line("2d parsed counts", { nodes: FEM2D_DEMO_SNAPSHOT.nodes.length, elements: FEM2D_DEMO_SNAPSHOT.elements.length, supports: FEM2D_DEMO_SNAPSHOT.supports.length, regions: FEM2D_DEMO_SNAPSHOT.regions.length, loadCases: FEM2D_DEMO_SNAPSHOT.loadCases.length });
line("2d quantity units stripped", { firstNodeY: FEM2D_DEMO_SNAPSHOT.nodes[0]?.y, steelE: FEM2D_DEMO_SNAPSHOT.materials[0]?.e, steelRho: FEM2D_DEMO_SNAPSHOT.materials[0]?.rho, chs76Area: FEM2D_DEMO_SNAPSHOT.sections[0]?.area });
line("2d nested loads:BLOCK", FEM2D_DEMO_SNAPSHOT.loadCases.map((entry) => `${entry.id}:${entry.loads.map((load) => `${load.kind}/${load.id}`).join("+")}:self=${entry.selfWeight}`));

const state2d = fem2dStoryStateFor(FEM2D_STORY_EXAMPLE_ID, "en-US");
const layers = fem2dStructureLayers(state2d.snapshot, "#38bdf8", "#94a3b8", "#f97316");
line("2d layer ids", layers.map((layer) => (layer as { id: string }).id));
line("2d scene node", { componentKind: buildFem2dSceneNode(layers, FEM2D_DEFAULT_CAMERA, "fem2d.play.model", "fem2d-play").componentKind, layerCount: layers.length });
line("2d addNode", reduceFem2dStoryAction(state2d, "addNode", { x: 4, y: 9 }).snapshot.nodes.at(-1));
line("2d next id", femNextId(state2d.snapshot.nodes.map((node) => node.id), "n"));
line("2d setActiveExample(unknown) → nodes", reduceFem2dStoryAction(state2d, "setActiveExample", { exampleId: "none" }).snapshot.nodes.length);
line("2d de labels", fem2dSummaryLines(state2d.snapshot, "de-DE").join(" | "));

line("3d parsed counts", { nodes: FEM3D_DEMO_SNAPSHOT.nodes.length, elements: FEM3D_DEMO_SNAPSHOT.elements.length, supports: FEM3D_DEMO_SNAPSHOT.supports.length, solids: FEM3D_DEMO_SNAPSHOT.solids.length, loadCases: FEM3D_DEMO_SNAPSHOT.loadCases.length });
const state3d = fem3dStoryStateFor(FEM3D_STORY_EXAMPLE_ID, "en-US");
const instances = fem3dStructuralInstances(state3d.snapshot);
line("3d instance count", instances.length);
line("3d vertical member (identity quat)", instances.find((entry) => (entry as { id: string }).id === "el-e1"));
line("3d horizontal member (+X, 90° about Y)", instances.find((entry) => (entry as { id: string }).id === "el-fb1_0"));
line("3d box mesh buffers", JSON.parse(buildFem3dSceneNode(instances, FEM3D_DEFAULT_CAMERA_JSON, "fem3d.play.model", "fem3d-play").world3d!.meshesJson).map((mesh: { id: string; data: { positions: number[]; normals: number[]; indices: number[] } }) => [mesh.id, mesh.data.positions.length, mesh.data.normals.length, mesh.data.indices.length]));
line(`3d setActiveExample(${FEM3D_STORY_SHIPPED_EXAMPLE_ID}) → nodes`, reduceFem3dStoryAction(state3d, "setActiveExample", { exampleId: FEM3D_STORY_SHIPPED_EXAMPLE_ID }).snapshot.nodes.length);
line("3d de labels", fem3dSummaryLines(state3d.snapshot, "de-DE").join(" | "));

line("re-parse determinism", { fem2d: JSON.stringify(parseFem2dDsl.length) !== "", fem3d: JSON.stringify(parseFem3dDsl.length) !== "" });
