/** 🧶 Node graph host, graph slider overlays, graph parameter dispatch and shared Flow runtime. */
import { act as reactAct, createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { type ValidateFunction } from "ajv";
import Ajv2020 from "ajv/dist/2020";
import { uiViewport2dSchema as viewport2dSchema } from "@semio-tech/framework/ui/testing/fixtures";
import { viewportPoseFixture } from "@semio-tech/framework/ui/testing/fixtures";
import { semioSchemaAjvV1 } from "@semio-tech/schema-ts/oracles/validator";
import { validateJsonSchemaSubset } from "@semio-tech/schema-validator-ts";
import { dagSliderOverlayFixture as graphSliderFixture, artifactsDagVcsSchema as dagVcsSchema } from "../../../../♾️infinite/🧪️tests/🧫️fixtures/🟦️.ts";
import { artifactsFlowParameterFixture as graphParameterFixture, artifactsFlowParameterSchema as flowParameterSchema, wasmBrowserRuntimeFixture as flowBrowserRuntimeFixture } from "../../../../🌊️flow/🧪️tests/🧫️fixtures/🟦️.ts";
import graphPickFixture from "@semio-tech/framework-os-node-graph-rs/testing/fixtures/pick-target.json";
import * as flowSessionLoader from "../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import * as infiniteCanvasRenderer from "@semio-tech/canvas-react-renderer";
import { createFlowBrowserRuntime } from "@semio-tech/flow-core/🌐️flow-browser.js";
import { MockFlowBridge } from "../../../../🌊️flow/🕸️wasm/🧪️tests/🎭️mock-flow-bridge/🟦️.ts";
import { cleanup, fireEvent, render, waitFor } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { NodeGraphHost, FlowGraphCanvasHost, nodeGraphHoverActionArgs, nodeGraphSelectionActionArgs, world3dHoverActionArgs, world3dSelectionActionArgs, interactionTargetsForInstances, WORLD3D_DEFAULT_INTERACTION_GRANULARITY, WORLD3D_DEFAULT_MARKER_GRANULARITY, world3dMarkerInteractionTarget, world3dInstanceInteractionTarget, world3dInstanceInteractionTargets, world3dSelectionTargetsActionArgs, nodeGraphViewportActionArgs, parseNodeGraphSessionViewport, nodeGraphPickChannel, parseDagSliderOverlays, dagSliderValueText, GraphSliderOverlays, sceneToSyncPack } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { computeAccessibleName, noopAction, rendererExport } from "../🧰️engine-contract-helpers/🟦️.ts";

/** 🧬️ Compiles one named export of the `os.renderer` schema module. */
/** 🧬️ Compiles one named `$defs` export of a peer scope's `🧬️schema/` module. */
const peerExport = (module: { $id: string }, exportId: string): ValidateFunction => semioSchemaAjvV1({ strict: true, allErrors: true }).addSchema(module).getSchema(`${module.$id}#/$defs/${exportId}`) as ValidateFunction;

describe("physical slider receiving contract",()=>{

  it("checks actual slider rows against the canonical contract and independent validator",()=>{
    const independent=peerExport(dagVcsSchema,"SliderOverlayRow");
    for(const item of graphSliderFixture.cases){expect(independent(item.row),JSON.stringify(independent.errors)).toBe(true);expect(validateJsonSchemaSubset(dagVcsSchema.$defs.SliderOverlayRow,item.row,dagVcsSchema)).toEqual([]);expect(parseDagSliderOverlays(JSON.stringify({sliders:[item.row]}))).toEqual([item.row]);}
  });
});

describe("framework renderer hosts", () => {
  afterEach(() => cleanup());

  it("renders node graph host from workflow scene json", () => {
    const markup = renderToStaticMarkup(
      createElement(NodeGraphHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.workflow",
          controllerId: "s-play",
          componentKind: "node-graph",
          nodeGraph: {
            nodes: [
              {
                id: "node-a",
                instanceId: "app-a",
                label: "Draw",
                x: 10,
                y: 20,
                width: 160,
                height: 80,
                inputs: [{ id: "in", resourceKind: "2d.drawing" }],
                outputs: [{ id: "out", resourceKind: "2d.drawing" }],
              },
            ],
            edges: [],
            viewport: { x: 0, y: 0, zoom: 1 },
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-node-graph-host");
  });

  it("retires a mounted graph gesture without synthesizing an up or graph commit", async () => {
    const attachCanvas = vi.fn(async () => {});
    const pointerDownScreen = vi.fn();
    const pointerUpScreen = vi.fn();
    const pointerCancelScreen = vi.fn();
    const onAction = vi.fn();
    const session = {
      attachCanvas,
      setCaretVisible: vi.fn(),
      setSize: () => {},
      renderFrame: vi.fn(),

      synchronizeScene: () => {},
      setCanvasThemeJson: () => {},
      pointerDownScreen,
      pointerMoveScreen: () => {},
      pointerUpScreen,
      pointerCancelScreen,
      wheelScreen: () => {},
      labelOverlayPaintStateJson: () => '{"labels":[]}',
      sliderOverlayStateJson: () => "{}",
      selectionUnionBoundsScreenJson: () => "{}",
      selectionPreviewPointsJson: () => "[]",
      selectionPreviewCrossing: () => false,
      selectionPreviewMethod: () => "rectangle",
      selectedNodeIdsJson: () => "[]",
      hoveredNodeId: () => null,
      hoveredChannelJson: () => '{"channel":null,"refusal":null}',
      viewport: () => ({ x: 0, y: 0, zoom: 1 }),
      pickTargetsAtScreenJson: () => "[]",
      setHover: () => {},
      setHoverChannel: () => {},
      alignSelection: () => {},
      hostSnapshotJson: () => "{}",
      takePendingOpenInstanceId: () => null,
      free: vi.fn(),
    } as unknown as Awaited<ReturnType<typeof flowSessionLoader.createGraphSession>>;
    const factory = vi.spyOn(flowSessionLoader, "createGraphSession").mockResolvedValue(session);
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
    const canvasContext = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    const view = render(
      createElement(NodeGraphHost, {
        node: {
          type: "componentScene",
          surfaceId: "graph.cancel",
          controllerId: "graph",
          componentKind: "node-graph",
          nodeGraph: {
            nodes: [{ id: "node-a", instanceId: "app-a", label: "Draw", x: 10, y: 20, width: 160, height: 80, inputs: [], outputs: [] }],
            edges: [],
            viewport: { x: 0, y: 0, zoom: 1 },
            editable: true,
            // 🎯️ Hover and selection are published to an interaction DOMAIN: `nodeGraphHoverActionArgs`
            // (`🧱️elements/🕸️NodeGraph/🟦️.ts?:252`) returns `undefined` without one, so a scene that
            // declares none can never publish the retired hover this law measures.
            interactionDomain: { id: "graph.cancel", nodeTargetPrefix: "node:", edgeTargetPrefix: "edge:", handleTargetPrefix: "handle:" },
          },
        },
        onAction,
      }),
    );
    try {
      await waitFor(() => expect(attachCanvas).toHaveBeenCalledOnce());
      const surface = view.container.querySelector('.semio-node-graph-host [class*="z-30"]') as HTMLElement;
      await waitFor(() => {
        fireEvent.pointerDown(surface, { pointerId: 11, button: 0, clientX: 20, clientY: 20 });
        expect(pointerDownScreen).toHaveBeenCalled();
      });
      const downsBeforeCancel = pointerDownScreen.mock.calls.length;
      const actionsBeforeCancel = onAction.mock.calls.length;
      fireEvent.pointerCancel(surface, { pointerId: 11, clientX: 30, clientY: 20 });
      expect(pointerCancelScreen).toHaveBeenCalledOnce();
      expect(pointerUpScreen).not.toHaveBeenCalled();
      expect(onAction.mock.calls.slice(actionsBeforeCancel).map(([action]) => action.action)).toEqual(["interactionHover"]);
      fireEvent.pointerDown(surface, { pointerId: 12, button: 0, clientX: 40, clientY: 20 });
      expect(pointerDownScreen).toHaveBeenCalledTimes(downsBeforeCancel + 1);
    } finally {
      view.unmount();
      factory.mockRestore();
      bounds.mockRestore();
      canvasContext.mockRestore();
    }
  });

  it("renders editable node graph host with find items", () => {
    const markup = renderToStaticMarkup(
      createElement(NodeGraphHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.workflow",
          controllerId: "s-play",
          componentKind: "node-graph",
          nodeGraph: {
            nodes: [
              {
                id: "node-a",
                instanceId: "app-a",
                label: "Draw",
                x: 10,
                y: 20,
                width: 160,
                height: 80,
                inputs: [{ id: "in", resourceKind: "2d.drawing" }],
                outputs: [{ id: "out", resourceKind: "2d.drawing" }],
              },
            ],
            edges: [],
            viewport: { x: 0, y: 0, zoom: 1 },
            editable: true,
            findItems: [{ id: "app-a", label: "Draw", category: "Workflow" }],
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-node-graph-host");
  });

  it("uses the live session viewport for node graph wheel actions", () => {
    expect(parseNodeGraphSessionViewport({ x: 12, y: 24, zoom: 1.75 })).toEqual({ x: 12, y: 24, zoom: 1.75 });
    expect(nodeGraphViewportActionArgs({ x: 12, y: 24, zoom: 1.75 })).toEqual({
      viewport: { x: 12, y: 24, zoom: 1.75 },
    });
    expect(() => parseNodeGraphSessionViewport('{"x":12,"y":24,"zoom":1.75}')).toThrow();
    expect(() => nodeGraphViewportActionArgs({ x: 0, y: 0, zoom: 0 })).toThrow();
    expect(() => nodeGraphViewportActionArgs({ x: 0, y: 0, zoom: Number.NaN })).toThrow();
    expect(() => nodeGraphViewportActionArgs({ x: 0, y: 0, zoom: 1, extra: true } as never)).toThrow();
  });

  it("matches the shared neutral viewport schema at the node graph action boundary", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(viewport2dSchema);
    const cases = viewportPoseFixture.cases.filter((row) => row.dimension === "2d");
    for (const row of cases) {
      expect(validate(row.value), row.name).toBe(row.valid);
      if (row.valid) {
        expect(nodeGraphViewportActionArgs(parseNodeGraphSessionViewport(row.value)), row.name).toEqual({ viewport: row.value });
      } else {
        expect(() => nodeGraphViewportActionArgs(parseNodeGraphSessionViewport(row.value)), row.name).toThrow();
      }
    }
    expect(cases).toHaveLength(8);
  });

  it("encodes node graph selection and hover with its scene-owned framework interaction address", () => {
    const domain = { id: "graph", nodeTargetPrefix: "document.node.", edgeTargetPrefix: "document.edge.", handleTargetPrefix: "document.handle." };
    expect(nodeGraphSelectionActionArgs(domain, { nodeIds: ["node-a"], edgeIds: ["edge-a"], handleIds: ["handle-a"] })).toEqual({
      domainId: "graph",
      targets: JSON.stringify([
        { granularity: "node", id: "document.node.node-a" },
        { granularity: "edge", id: "document.edge.edge-a" },
        { granularity: "handle", id: "document.handle.handle-a" },
      ]),
      merge: "replace",
      method: "pick",
    });
    expect(nodeGraphHoverActionArgs(domain, "node-a")).toEqual({
      domainId: "graph",
      channel: "pointer",
      targets: JSON.stringify([{ granularity: "node", id: "document.node.node-a" }]),
    });
    expect(nodeGraphSelectionActionArgs(undefined, { nodeIds: ["node-a"] })).toBeUndefined();
    expect(nodeGraphHoverActionArgs(undefined, "node-a")).toBeUndefined();
  });

  // 🎯️ A world-3d window bound to an interaction domain must speak the SAME framework verbs the node
  // graph does, so hovering geometry in the 3D view and hovering its node in the graph land on one
  // shared hover state — the two halves of generation3d's bidirectional hover.
  it("encodes world3d interaction dispatch args the same way the node graph does", () => {
    expect(WORLD3D_DEFAULT_INTERACTION_GRANULARITY).toBe("handle");
    expect(world3dHoverActionArgs("graph", "handle", "extrude@solid")).toEqual({
      domainId: "graph",
      channel: "pointer",
      targets: JSON.stringify([{ granularity: "handle", id: "extrude@solid" }]),
    });
    expect(world3dHoverActionArgs("graph", "handle", null)).toEqual({
      domainId: "graph",
      channel: "pointer",
      targets: JSON.stringify([]),
    });
    expect(world3dSelectionActionArgs("graph", "handle", ["extrude@solid"], "replace")).toEqual({
      domainId: "graph",
      targets: JSON.stringify([{ granularity: "handle", id: "extrude@solid" }]),
      merge: "replace",
      method: "pick",
    });
  });

  // 🧿️ A marker hit (vortex / target volume / reference) must reach the SAME generic
  // `interactionSelect`/`interactionHover` verbs an instance hit does, at its own granularity — the
  // host never learns an app's granularity names: the layer name is the default and the scene record
  // overrides it.
  it("resolves world3d marker interaction targets from the scene record, not from host knowledge", () => {
    expect(WORLD3D_DEFAULT_MARKER_GRANULARITY).toEqual({ vortex: "vortex", attraction: "attraction", targetVolume: "targetVolume", reference: "reference" });
    expect(world3dMarkerInteractionTarget("vortex", "seed-left-001:v0")).toEqual({ granularity: "vortex", id: "seed-left-001:v0" });
    expect(world3dMarkerInteractionTarget("vortex", "seed-left-001:v0", {})).toEqual({ granularity: "vortex", id: "seed-left-001:v0" });
    expect(world3dMarkerInteractionTarget("targetVolume", "volume-1")).toEqual({ granularity: "targetVolume", id: "volume-1" });
    expect(world3dMarkerInteractionTarget("reference", "ref-1")).toEqual({ granularity: "reference", id: "ref-1" });
    expect(world3dMarkerInteractionTarget("vortex", "seed-left-001:v0", { interactionGranularityId: "pin", interactionId: "pin-7" })).toEqual({ granularity: "pin", id: "pin-7" });
  });

  // 🎯️ An instance pick, hover and marquee resolve each rendered row onto ITS OWN granularity — a fem3d
  // scene draws nodes, members, solids, supports and load glyphs as one instance lane under one domain,
  // and a member must select an `element`, never the scene default `node`. Rows without a declared
  // granularity keep the scene's default; the load glyph redirects onto its load through both fields.
  it("resolves world3d instance interaction targets per record, falling back to the scene granularity", () => {
    const instances = [{ id: "n1" }, { id: "e1", interactionGranularityId: "element" }, { id: "l2:head", interactionId: "l2", interactionGranularityId: "load" }, { id: "l2:shaft", interactionId: "l2", interactionGranularityId: "load" }];
    expect(world3dInstanceInteractionTarget(instances, "n1", "node")).toEqual({ granularity: "node", id: "n1" });
    expect(world3dInstanceInteractionTarget(instances, "e1", "node")).toEqual({ granularity: "element", id: "e1" });
    expect(world3dInstanceInteractionTarget(instances, "l2:shaft", "node")).toEqual({ granularity: "load", id: "l2" });
    expect(world3dInstanceInteractionTarget(instances, "ghost", "node")).toEqual({ granularity: "node", id: "ghost" });
    expect(world3dInstanceInteractionTargets(instances, ["l2:head", "e1", "l2:shaft", "n1"], "node")).toEqual([
      { granularity: "load", id: "l2" },
      { granularity: "element", id: "e1" },
      { granularity: "node", id: "n1" },
    ]);
    expect(world3dSelectionTargetsActionArgs("fem3d", world3dInstanceInteractionTargets(instances, ["e1", "n1"], "node"), "replace", "rectangle")).toEqual({
      domainId: "fem3d",
      targets: JSON.stringify([
        { granularity: "element", id: "e1" },
        { granularity: "node", id: "n1" },
      ]),
      merge: "replace",
      method: "rectangle",
    });
    expect(world3dSelectionTargetsActionArgs("fem3d", [{ granularity: "node", id: "n1" }], "additive")).toEqual({
      domainId: "fem3d",
      targets: JSON.stringify([{ granularity: "node", id: "n1" }]),
      merge: "additive",
      method: "pick",
    });
  });

  // 🧿️ The exact wire shapes a vortex-marker click and hover now put on the domain path — the same
  // encoder the instance path uses, so one selection authority sees both.
  it("encodes a vortex marker pick and hover as generic domain interaction args", () => {
    const target = world3dMarkerInteractionTarget("vortex", "seed-left-001:v0");
    expect(world3dSelectionActionArgs("vortex", target.granularity, [target.id], "invertive")).toEqual({
      domainId: "vortex",
      targets: JSON.stringify([{ granularity: "vortex", id: "seed-left-001:v0" }]),
      merge: "invertive",
      method: "pick",
    });
    expect(world3dHoverActionArgs("vortex", target.granularity, target.id)).toEqual({
      domainId: "vortex",
      channel: "pointer",
      targets: JSON.stringify([{ granularity: "vortex", id: "seed-left-001:v0" }]),
    });
    expect(world3dHoverActionArgs("vortex", WORLD3D_DEFAULT_MARKER_GRANULARITY.vortex, undefined)).toEqual({
      domainId: "vortex",
      channel: "pointer",
      targets: JSON.stringify([]),
    });
  });

  // 🎯️ Several rendered instances can stand for ONE interaction target — generation3d renders one
  // instance per geometry item of a channel and every one of them resolves to that channel's port,
  // which is the id its interaction topology actually declares.
  it("collapses world instance ids onto the interaction targets they stand for", () => {
    const instances = [{ id: "profile@wire#0", interactionId: "profile@wire" }, { id: "profile@wire#1", interactionId: "profile@wire" }, { id: "extrude@solid#0", interactionId: "extrude@solid" }, { id: "plain-instance" }];
    expect(interactionTargetsForInstances(instances, ["profile@wire#0", "profile@wire#1", "extrude@solid#0"])).toEqual(["profile@wire", "extrude@solid"]);
    expect(interactionTargetsForInstances(instances, ["plain-instance"])).toEqual(["plain-instance"]);
    expect(interactionTargetsForInstances(instances, ["unknown-id"])).toEqual(["unknown-id"]);
  });

  it("encodes node graph scenes as pack bytes for wasm sync", async () => {
    const scene = {
      nodes: [],
      edges: [],
      viewport: { x: 0, y: 0, zoom: 1 },
    };
    const bytes = sceneToSyncPack(scene);
    expect(bytes.length).toBeGreaterThan(8);
  });

  //#region 🎚️GraphSliderAccessibility
  it("decodes graph pick channels from the native handle grammar with strict schema parity", () => {
    const validateTarget = rendererExport("NodeGraphPickTargetV1");
    const validateChannel = rendererExport("NodeGraphInteractionChannelV1");
    for (const row of graphPickFixture) {
      const target = row.target;
      expect(validateTarget(target), JSON.stringify(validateTarget.errors)).toBe(true);
      expect(validateChannel(row.channel), JSON.stringify(validateChannel.errors)).toBe(true);
      const match = target?.domain === "handle" ? /^([^@]+)@(.+)$/u.exec(target.id) : null;
      const oracle = match ? { nodeId: match[1], portId: match[2] } : null;
      expect(oracle).toEqual(row.channel);
      expect(nodeGraphPickChannel(target)).toEqual(row.channel);
    }
    const hostile = structuredClone(graphPickFixture);
    Object.assign(hostile[1]!.target!, { portId: "invented" });
    expect(validateTarget(hostile[1]!.target)).toBe(false);
  });

  it("validates strict language-neutral graph slider labels and rejects unnamed rows", () => {
    const validate = peerExport(dagVcsSchema,"SliderOverlayRow");
    for(const item of graphSliderFixture.cases)expect(validate(item.row),JSON.stringify(validate.errors)).toBe(true);
    for (const label of ["", "   ", null, 42]) {
      const malformed = structuredClone(graphSliderFixture);
      (malformed.cases[0]!.row as Record<string, unknown>).label = label;
      expect(validate(malformed.cases[0]!.row)).toBe(false);
      expect(parseDagSliderOverlays(JSON.stringify({ sliders: [malformed.cases[0]!.row] }))).toEqual([]);
    }
    expect(parseDagSliderOverlays('{"sliders":{}}')).toEqual([]);
  });

  it("names graph slider overlays from exact localized captions and keeps scoped ids stable", () => {
    const first = graphSliderFixture.cases[0]!;
    const second = graphSliderFixture.cases[1]!;
    const props = (item: typeof first) => ({
      scopeId: item.scopeId,
      stateJson: JSON.stringify({ camera: { x: 0, y: 0, zoom: 1 }, sliders: [item.row] }),
      logicalW: 800,
      logicalH: 600,
      editable: true,
      onSliderChange: () => {},
    });
    const view = render(createElement("div", {}, createElement(GraphSliderOverlays, props(first)), createElement(GraphSliderOverlays, props(second))));
    const english = view.getByRole("slider", { name: first.row.label });
    const german = view.getByRole("slider", { name: second.row.label });
    expect(computeAccessibleName(english)).toBe(first.row.label);
    expect(computeAccessibleName(german)).toBe(second.row.label);
    const controlId = english.closest('[data-slot="slider"]')!.id;
    expect(controlId).toBe(`graph-slider-${encodeURIComponent(JSON.stringify([first.scopeId, first.row.widgetId]))}`);
    expect(german.closest('[data-slot="slider"]')!.id).not.toBe(controlId);
    view.rerender(createElement("div", {}, createElement(GraphSliderOverlays, { ...props(first), stateJson: JSON.stringify({ sliders: [{ ...first.row, value: 3 }] }) }), createElement(GraphSliderOverlays, props(second))));
    expect(view.getByRole("slider", { name: first.row.label }).closest('[data-slot="slider"]')!.id).toBe(controlId);
  });

  it("keeps graph slider keyboard changes exact and disabled controls inert", () => {
    const item = graphSliderFixture.cases[1]!;
    const changes = vi.fn();
    const gesture = vi.fn();
    const props = { scopeId: item.scopeId, stateJson: JSON.stringify({ sliders: [item.row] }), logicalW: 800, logicalH: 600, editable: true, onSliderChange: changes, onSliderPointerDown: gesture };
    const view = render(createElement(GraphSliderOverlays, props));
    const slider = view.getByRole("slider", { name: item.row.label });
    slider.focus();
    expect(document.activeElement).toBe(slider);
    for (const action of graphSliderFixture.keyboard) {
      fireEvent.keyDown(slider, { key: action.key });
      fireEvent.keyUp(slider, { key: action.key });
      expect(changes).toHaveBeenLastCalledWith(item.row.widgetId, action.expected);
    }
    expect(gesture).not.toHaveBeenCalled();
    changes.mockClear();
    view.rerender(createElement(GraphSliderOverlays, { ...props, editable: false }));
    const disabled = view.getByRole("slider", { name: item.row.label });
    expect(disabled.getAttribute("aria-disabled")).toBe("true");
    expect(disabled.tabIndex).toBe(-1);
    fireEvent.keyDown(disabled, { key: "ArrowRight" });
    fireEvent.keyUp(disabled, { key: "ArrowRight" });
    expect(changes).not.toHaveBeenCalled();
  });

  /** ⚖️ LAW (scrub protocol, `📓️api-scrub-machine.md`): a graph slider cancels its open press like every continuous
   * control — a lost pointer capture as `captureLost`, a blur as `blur` — so the guest drops the press with zero trace. */
  it("cancels a graph slider press on lost capture and on blur", () => {
    const item = graphSliderFixture.cases[1]!;
    const aborts = vi.fn();
    const view = render(createElement(GraphSliderOverlays, { scopeId: item.scopeId, stateJson: JSON.stringify({ sliders: [item.row] }), logicalW: 800, logicalH: 600, editable: true, onSliderChange: vi.fn(), onSliderAbort: aborts }));
    const slider = view.getByRole("slider", { name: item.row.label });
    const root = slider.closest('[data-slot="slider"]')!;
    fireEvent.pointerDown(root, { pointerId: 1, clientX: 0 });
    fireEvent.pointerCancel(root, { pointerId: 1 });
    expect(aborts).toHaveBeenLastCalledWith(item.row.widgetId, "captureLost");
    slider.focus();
    fireEvent.blur(slider);
    expect(aborts).toHaveBeenLastCalledWith(item.row.widgetId, "blur");
  });

  /** ⚖️ LAW: a knob has ONE value. The readout painted beside the track and the `aria-valuenow` a
   * screen reader announces are two renderings of the same published number, so they can never
   * disagree — the readout used to be painted on the GPU, where it moved only when a frame was
   * drawn, and a released knob read `10` to a screen reader while the canvas said `0.0`
   * (`📓️slider-reevaluation-correctness-2026-09-15.md`). */
  it("renders a graph slider's painted readout and its aria value from one published number", () => {
    const item = graphSliderFixture.cases[0]!;
    const rowAt = (value: number) => ({ ...item.row, value, fontScreenPx: 9, gapScreenPx: 4 });
    const propsAt = (value: number) => ({
      scopeId: item.scopeId,
      stateJson: JSON.stringify({ camera: { x: 0, y: 0, zoom: 1 }, sliders: [rowAt(value)] }),
      logicalW: 800,
      logicalH: 600,
      editable: true,
      onSliderChange: () => {},
    });
    const view = render(createElement(GraphSliderOverlays, propsAt(item.row.min)));
    for (const value of [item.row.min, (item.row.min + item.row.max) / 2, item.row.max]) {
      view.rerender(createElement(GraphSliderOverlays, propsAt(value)));
      const knob = view.getByRole("slider", { name: item.row.label });
      const readout = view.container.querySelector(`[data-graph-slider-value="${item.row.widgetId}"]`);
      expect(readout, "the overlay paints the value beside the track it belongs to").not.toBeNull();
      expect(readout!.textContent).toBe(dagSliderValueText(value));
      expect(readout!.textContent).toBe(dagSliderValueText(Number(knob.getAttribute("aria-valuenow"))));
      expect(readout!.getAttribute("aria-hidden")).toBe("true");
    }
    const parsed = parseDagSliderOverlays(JSON.stringify({ sliders: [rowAt(item.row.max)] }))[0]!;
    expect(parsed.value).toBe(item.row.max);
    expect(dagSliderValueText(parsed.value)).toBe(view.container.querySelector(`[data-graph-slider-value="${item.row.widgetId}"]`)!.textContent);
  });
  //#endregion 🎚️GraphSliderAccessibility


  //#region 🎚️GraphParameterDispatch
  it("validates the strict language-neutral graph parameter contract for all three consumers", () => {
    
    const command = peerExport(flowParameterSchema, "GraphParameterCommand");
    
    for (const extra of ["snapshotJson", "hostSnapshotJson", "operations"]) {
      const malformed = structuredClone(graphParameterFixture);
      (malformed.cases[0] as Record<string, unknown>)[extra] = "{}";
      
      expect(command({ widgetId: "radius", value: 3, [extra]: "{}" })).toBe(false);
    }
    expect(new Set(graphParameterFixture.cases.map((value) => value.app))).toEqual(new Set(["flow", "generation2d", "generation3d"]));
    for (const value of graphParameterFixture.cases) expect(command({ widgetId: value.widgetId, value: value.request, surfaceId: value.surfaceId })).toBe(true);
  });

  it("dispatches graph parameter keyboard and drag events as bounded nodeGraphEdit operations with explicit commits", async () => {
    const task = <T>(value: T) => ({ result: Promise.resolve(value), subscribe: () => () => {}, cancel: vi.fn() });
    const scheduler = { invalidate: vi.fn(), paintNow: vi.fn(), beginContinuous: vi.fn(), endContinuous: vi.fn(), dispose: vi.fn() };
    const schedulerSpy = vi.spyOn(infiniteCanvasRenderer, "createDemandFrameScheduler").mockReturnValue(scheduler);
    const contextSpy = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    const timerSpy = vi.spyOn(globalThis, "setTimeout");
    const createSpy = vi.spyOn(flowSessionLoader, "createFlowSession");
    const originalPointer = globalThis.PointerEvent;
    class TestPointerEvent extends MouseEvent {
      readonly pointerId: number;
      readonly pointerType: string;
      constructor(type: string, init: PointerEventInit = {}) {
        super(type, init);
        this.pointerId = init.pointerId ?? 1;
        this.pointerType = init.pointerType ?? "mouse";
      }
    }
    globalThis.PointerEvent = TestPointerEvent as unknown as typeof PointerEvent;
    try {
      for (const item of graphParameterFixture.cases) {
        const row = { widgetId: item.widgetId, label: item.label, ...item.before, x: 0, y: 0, w: 100, h: 16 };
        const methods: Record<string, ReturnType<typeof vi.fn>> = {
          documentJson: vi.fn(() => {
            throw new Error("slider must not serialize the fixture");
          }),
          sliderOverlayStateJson: vi.fn(() => task(JSON.stringify({ camera: { x: 0, y: 0, zoom: 1 }, sliders: [row] }))),
          labelOverlayPaintStateJson: vi.fn(() => task("{}")),
          setSliderValue: vi.fn((_id: string, value: number) => {
            row.value = value;
            return task(undefined);
          }),
          selectedWidgetIds: vi.fn(() => task("[]")),
          previewOffWidgetIds: vi.fn(() => task("[]")),
          selectionPreviewPointsJson: vi.fn(() => task("[]")),
          selectionPreviewCrossing: vi.fn(() => task(false)),
        };
        const session = new Proxy(methods, {
          get(target, key: string) {
            if (key === "then") return undefined;
            return (target[key] ??= vi.fn(() => task(undefined)));
          },
        }) as unknown as flowSessionLoader.FlowWasmSession;
        createSpy.mockResolvedValueOnce(session);
        const onAction = vi.fn();
        const view = render(
          createElement(FlowGraphCanvasHost, {
            scene: { nodes: [], edges: [], viewport: { x: 0, y: 0, zoom: 1 }, hostSnapshotJson: '{"schema":"flow.host_snapshot","widgets":[]}' },
            controllerId: item.controllerId,
            surfaceId: item.surfaceId,
            editable: true,
            keyboardPort: { current: null },
            onAction,
          }),
        );
        await waitFor(() => expect(view.getByRole("slider", { name: item.label })).toBeTruthy());
        const slider = view.getByRole("slider", { name: item.label });
        const root = slider.closest('[data-slot="slider"]') as HTMLElement;
        const stableId = root.id;
        for (const element of [slider, root]) {
          Object.defineProperties(element, { setPointerCapture: { value: () => {} }, releasePointerCapture: { value: () => {} }, hasPointerCapture: { value: () => true } });
        }
        root.getBoundingClientRect = () => ({ x: 0, y: 0, width: 100, height: 16, left: 0, right: 100, top: 0, bottom: 16, toJSON: () => ({}) });
        (root.querySelector('[data-slot="slider-track"]') as HTMLElement).getBoundingClientRect = root.getBoundingClientRect;
        const gesturePrefix = `${item.surfaceId}:${item.widgetId}`.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
        const gesturePattern = new RegExp(`^${gesturePrefix}:\\d+:\\d+$`);
        slider.focus();
        fireEvent.keyDown(slider, { key: "ArrowRight" });
        fireEvent.keyUp(slider, { key: "ArrowRight" });
        await waitFor(() =>
          expect(onAction).toHaveBeenLastCalledWith({
            controllerId: item.controllerId,
            action: graphParameterFixture.action,
            args: { surfaceId: item.surfaceId, operations: [{ operation: "setSlider", widgetId: item.widgetId, value: 3 }], gesture: expect.stringMatching(gesturePattern), commit: true },
          }),
        );
        expect(document.activeElement).toBe(slider);
        expect(computeAccessibleName(slider)).toBe(item.label);
        expect(root.id).toBe(stableId);
        let staleOverlay: ((value: string) => void) | undefined;
        methods.sliderOverlayStateJson!.mockImplementationOnce(() => ({
          ...task(""),
          result: new Promise<string>((resolve) => {
            staleOverlay = resolve;
          }),
        }));
        fireEvent.pointerDown(root, { pointerId: 1, pointerType: "mouse", button: 0, buttons: 1, clientX: 40, clientY: 8 });
        fireEvent.pointerMove(root, { pointerId: 1, pointerType: "mouse", buttons: 1, clientX: 80, clientY: 8 });
        await waitFor(() => {
          const event = onAction.mock.calls.at(-1)?.[0];
          expect(event).toMatchObject({ controllerId: item.controllerId, action: graphParameterFixture.action, args: { surfaceId: item.surfaceId, operations: [{ operation: "setSlider", widgetId: item.widgetId, value: 8 }], commit: false } });
          expect(event.args.gesture).toMatch(gesturePattern);
        });
        await waitFor(() => expect(slider.getAttribute("aria-valuenow")).toBe("8"));
        await reactAct(async () => {
          staleOverlay?.(JSON.stringify({ sliders: [{ ...row, value: 4 }] }));
        });
        expect(slider.getAttribute("aria-valuenow")).toBe("8");
        const count = onAction.mock.calls.length;
        const gesture = onAction.mock.calls.at(-1)?.[0].args.gesture;
        fireEvent.pointerUp(root, { pointerId: 1, button: 0, clientX: 80, clientY: 8 });
        await waitFor(() => expect(onAction).toHaveBeenCalledTimes(count + 1));
        expect(onAction).toHaveBeenLastCalledWith({
          controllerId: item.controllerId,
          action: graphParameterFixture.action,
          args: { surfaceId: item.surfaceId, operations: [{ operation: "setSlider", widgetId: item.widgetId, value: 8 }], gesture, commit: true },
        });
        fireEvent.pointerCancel(root, { pointerId: 1 });
        expect(onAction).toHaveBeenCalledTimes(count + 1);
        expect(methods.documentJson).not.toHaveBeenCalled();
        expect(
          onAction.mock.calls.every(
            ([event]) =>
              event.action === graphParameterFixture.action &&
              Object.keys(event.args).sort().join() === "commit,gesture,operations,surfaceId" &&
              event.args.operations.every((operation: Record<string, unknown>) => Object.keys(operation).sort().join() === "operation,value,widgetId"),
          ),
        ).toBe(true);
        view.unmount();
      }
      expect(timerSpy.mock.calls.some(([, delay]) => delay === 80)).toBe(false);
      expect(scheduler.beginContinuous).toHaveBeenCalledWith("gesture");
      expect(scheduler.endContinuous).toHaveBeenCalledWith("gesture");
    } finally {
      cleanup();
      createSpy.mockRestore();
      schedulerSpy.mockRestore();
      contextSpy.mockRestore();
      timerSpy.mockRestore();
      globalThis.PointerEvent = originalPointer;
    }
  });

  it("retains one shared Flow browser runtime across two mounted graph hosts and retires only each unmounted session", async () => {
    
    
    const expected = flowBrowserRuntimeFixture.mountedHosts;
    const bridge = new MockFlowBridge(new WebAssembly.Memory({ initial: 400 }));
    const runtime = await createFlowBrowserRuntime({ source: bridge.exports });
    const createSpy = vi.spyOn(flowSessionLoader, "createFlowSession").mockImplementation(async () => runtime.openSession());
    const contextSpy = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    const scene = (revision: number) => ({
      nodes: [],
      edges: [],
      viewport: { x: 0, y: 0, zoom: 1 },
      hostSnapshotJson: JSON.stringify({ schema: "flow.host_snapshot", revision, widgets: [] }),
    });
    const host = (id: "A" | "B", revision: number) =>
      createElement(FlowGraphCanvasHost, {
        key: id,
        scene: scene(revision),
        controllerId: `flow.${id}`,
        surfaceId: `flow.${id}`,
        editable: true,
        keyboardPort: { current: null },
        onAction: vi.fn(),
      });
    const pair = (revision: number) => createElement("div", {}, host("A", revision), host("B", revision));
    const single = (revision: number) => createElement("div", {}, host("B", revision));
    const view = render(pair(0));
    try {
      expect(expected.sharedRuntime).toBe(true);
      await waitFor(() => expect(bridge.openRequestIds).toEqual(flowBrowserRuntimeFixture.openRequestIds));
      await waitFor(() => expect(bridge.operationSessions.some((row) => row.operation === expected.afterUnmountA.command && row.slot === expected.afterUnmountA.liveSlot)).toBe(true));
      view.rerender(single(1));
      await waitFor(() => expect(bridge.closedSessionSlots).toEqual(expected.afterUnmountA.closedSlots));
      expect(bridge.globalCloseCalls).toBe(expected.afterUnmountA.globalCloseCalls);
      const beforeSiblingCommand = bridge.operationSessions.filter((row) => row.operation === expected.afterUnmountA.command && row.slot === expected.afterUnmountA.liveSlot).length;
      view.rerender(single(2));
      await waitFor(() => expect(bridge.operationSessions.filter((row) => row.operation === expected.afterUnmountA.command && row.slot === expected.afterUnmountA.liveSlot)).toHaveLength(beforeSiblingCommand + 1));
      view.unmount();
      await waitFor(() => expect(bridge.closedSessionSlots).toEqual(expected.afterUnmountB.closedSlots));
      expect(bridge.globalCloseCalls).toBe(expected.afterUnmountB.globalCloseCalls);
      await runtime.close();
      expect(bridge.globalCloseCalls).toBe(flowBrowserRuntimeFixture.runtimeClose.globalCloseCalls);
      expect(runtime.terminalIsEmpty()).toBe(flowBrowserRuntimeFixture.runtimeClose.terminal);
    } finally {
      view.unmount();
      await runtime.close();
      createSpy.mockRestore();
      contextSpy.mockRestore();
      cleanup();
    }
  });

  it("retires a graph host unmounted before its open reply while its shared-runtime sibling remains live", async () => {
    const expected = flowBrowserRuntimeFixture.mountedHosts.lateUnmount;
    const bridge = new MockFlowBridge(new WebAssembly.Memory({ initial: 400 }), { heldOpenReplies: 1 });
    const runtime = await createFlowBrowserRuntime({ source: bridge.exports });
    const createSpy = vi.spyOn(flowSessionLoader, "createFlowSession").mockImplementation(async () => runtime.openSession());
    const contextSpy = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    const scene = (revision: number) => ({
      nodes: [],
      edges: [],
      viewport: { x: 0, y: 0, zoom: 1 },
      hostSnapshotJson: JSON.stringify({ schema: "flow.host_snapshot", revision, widgets: [] }),
    });
    const host = (id: "A" | "B", revision: number) =>
      createElement(FlowGraphCanvasHost, {
        scene: scene(revision),
        controllerId: `flow.late.${id}`,
        surfaceId: `flow.late.${id}`,
        editable: true,
        keyboardPort: { current: null },
        onAction: vi.fn(),
      });
    const first = render(host("A", 0));
    let sibling: ReturnType<typeof render> | undefined;
    try {
      await waitFor(() => expect(bridge.openRequestIds).toEqual(["1"]));
      first.unmount();
      sibling = render(host("B", 0));
      await waitFor(() => expect(bridge.openRequestIds).toEqual(flowBrowserRuntimeFixture.openRequestIds));
      await waitFor(() => expect(bridge.operationSessions.some((row) => row.operation === expected.command && row.slot === expected.liveSlot)).toBe(true));
      bridge.releaseOpenReplies();
      await waitFor(() => expect(bridge.closedSessionSlots).toEqual(expected.closedSlots));
      expect(bridge.globalCloseCalls).toBe(0);
      const beforeSiblingCommand = bridge.operationSessions.filter((row) => row.operation === expected.command && row.slot === expected.liveSlot).length;
      sibling.rerender(host("B", 1));
      await waitFor(() => expect(bridge.operationSessions.filter((row) => row.operation === expected.command && row.slot === expected.liveSlot)).toHaveLength(beforeSiblingCommand + 1));
      sibling.unmount();
      await waitFor(() => expect(bridge.closedSessionSlots).toEqual([expected.heldOpenSlot, expected.liveSlot]));
      await runtime.close();
      expect(bridge.globalCloseCalls).toBe(flowBrowserRuntimeFixture.runtimeClose.globalCloseCalls);
      expect(runtime.terminalIsEmpty()).toBe(flowBrowserRuntimeFixture.runtimeClose.terminal);
    } finally {
      bridge.releaseOpenReplies();
      first.unmount();
      sibling?.unmount();
      await runtime.close();
      createSpy.mockRestore();
      contextSpy.mockRestore();
      cleanup();
    }
  });
  //#endregion 🎚️GraphParameterDispatch

  it("parses slider overlay state json for flow graph hosts", () => {
    const sliders = parseDagSliderOverlays(
      JSON.stringify({
        camera: { x: 0, y: 0, zoom: 1 },
        sliders: [
          {
            widgetId: "slider_2",
            label: "Radius",
            value: 2.2,
            min: 0,
            max: 10,
            step: 0.1,
            x: 100,
            y: 50,
            w: 120,
            h: 8,
          },
        ],
      }),
    );
    expect(sliders).toHaveLength(1);
    expect(sliders[0]?.widgetId).toBe("slider_2");
    expect(sliders[0]?.value).toBe(2.2);
  });

  it("renders graph slider overlays as track-only controls without a nested value readout", () => {
    const markup = renderToStaticMarkup(
      createElement(GraphSliderOverlays, {
        scopeId: "track-test",
        stateJson: JSON.stringify({
          camera: { x: 0, y: 0, zoom: 1 },
          sliders: [
            {
              widgetId: "slider_2",
              label: "Radius",
              value: 2.2,
              min: 0,
              max: 10,
              step: 0.1,
              x: 100,
              y: 50,
              w: 120,
              h: 8,
            },
          ],
        }),
        logicalW: 800,
        logicalH: 600,
        editable: true,
        onSliderChange: () => {},
      }),
    );
    expect(markup).toContain('data-slot="slider"');
    expect(markup).toContain('data-slot="slider-thumb"');
    expect(markup).not.toContain('data-slot="slider-value"');
    expect(markup).not.toContain('data-slot="slider-row"');
  });

  it("scales graph slider overlay chrome with canvas zoom so the knob matches other elements", () => {
    const markup = renderToStaticMarkup(
      createElement(GraphSliderOverlays, {
        scopeId: "zoom-test",
        stateJson: JSON.stringify({
          camera: { x: 0, y: 0, zoom: 2 },
          sliders: [
            {
              widgetId: "slider_2",
              label: "Radius",
              value: 2.2,
              min: 0,
              max: 10,
              step: 0.1,
              x: 100,
              y: 50,
              w: 120,
              h: 8,
            },
          ],
        }),
        logicalW: 800,
        logicalH: 600,
        editable: true,
        onSliderChange: () => {},
      }),
    );
    expect(markup).toContain('data-graph-slider-zoom="2"');
    expect(markup).toContain("translate(-50%, -50%) scale(2)");
    expect(markup).toContain("width:120px");
    expect(markup).toContain('data-slot="slider-thumb"');
    expect(markup).toContain("size-tiny");
    expect(markup).not.toMatch(/data-slot="slider-thumb"[^>]*size-small/);
  });
});
