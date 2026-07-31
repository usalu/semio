// #region 🧲️Header
// 💻️ .storybook/stories/framework/hosts/NodeGraphHost.stories.tsx
// Specs: Host the framework renderer's `NodeGraphHost` against real prebuilt WASM engines — the workflow
// `GraphSession` (`framework/surface/node-graph/rs`) for the default DAG variant, and the `FlowSession`
// (`flow/core/rs`) for the flow-graph variant `isFlowGraphScene`/`fixtureJson` routes to.
// Summary: Two stories share one debug-readout host component; each sets `parameters.wasm` to the loader id
// `NodeGraphHost`'s active branch needs so the `withWasm` decorator gates first paint until that engine's
// wasm-bindgen module has booted.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { Meta, StoryObj } from "@storybook/react";
import { useCallback, useState, type ReactElement } from "react";

import { NodeGraphHost } from "@semio-tech/framework-renderer-react";
import type { ActionDescriptor, NodeGraphScene, UiComponentSceneNode } from "@semio-tech/framework-core";

//#region SceneFixtures
/** 🕸️ Two connected workflow nodes, matching the shape `framework/os/renderer/js/react/index.test.ts` exercises for `NodeGraphHost`. */
const WORKFLOW_SCENE: NodeGraphScene = {
  nodesJson: JSON.stringify([
    { id: "node-a", instanceId: "app-a", label: "Draw", x: 10, y: 20, inputs: [{ id: "in", resourceKind: "2d.drawing" }], outputs: [{ id: "out", resourceKind: "2d.drawing" }] },
    { id: "node-b", instanceId: "app-b", label: "Composite", x: 260, y: 60, inputs: [{ id: "in", resourceKind: "2d.drawing" }], outputs: [] },
  ]),
  edgesJson: JSON.stringify([{ id: "edge-1", sourceNodeId: "node-a", sourcePortId: "out", targetNodeId: "node-b", targetPortId: "in" }]),
  viewportJson: '{"x":0,"y":0,"zoom":1}',
  editable: true,
  findItemsJson: JSON.stringify([{ id: "app-a", label: "Draw", category: "Workflow" }]),
};

/** 🌊️ A `FlowFixture`-shaped `fixtureJson` (mirrors `FlowFixture::default()` in `flow/core/rs/lib.rs`) — its presence alone routes `NodeGraphHost` to `FlowGraphCanvasHost`/`createFlowSession` (see `isFlowGraphScene`). */
const FLOW_FIXTURE_JSON = JSON.stringify({
  schema: "flow.fixture",
  camera: { x: 0, y: 0, zoom: 1 },
  widgets: [
    { kind: "inputSlider", id: "slider", value: 3, min: 0, max: 10, step: 0.1 },
    { kind: "neuron", id: "add", neuronKind: "math.add", params: {}, inputPorts: [], outputPorts: [], preview: true },
    { kind: "outputPreview", id: "preview", preview: {}, expanded: [] },
  ],
  synapses: [
    { id: "s1", from: "slider", to: "add", fromPort: "number", toPort: "a" },
    { id: "s2", from: "add", to: "preview", fromPort: "sum", toPort: "" },
  ],
  layout: {},
});

const FLOW_GRAPH_SCENE: NodeGraphScene = {
  nodesJson: "[]",
  edgesJson: "[]",
  viewportJson: '{"x":0,"y":0,"zoom":1}',
  editable: true,
  capabilitiesJson: '{"engine":"flow"}',
  fixtureJson: FLOW_FIXTURE_JSON,
};
//#endregion SceneFixtures

//#region StoryHost
function NodeGraphStoryHost({ scene, controllerId, surfaceId }: { readonly scene: NodeGraphScene; readonly controllerId: string; readonly surfaceId: string }): ReactElement {
  const [lastAction, setLastAction] = useState<ActionDescriptor | null>(null);

  const onAction = useCallback((action: ActionDescriptor): void => {
    setLastAction(action);
  }, []);

  const node: UiComponentSceneNode = { type: "componentScene", surfaceId, controllerId, componentKind: "node-graph", nodeGraph: scene };

  return (
    <div style={{ display: "flex", height: "100%", width: "100%", flexDirection: "column" }}>
      <div style={{ position: "relative", flex: "1 1 auto", minHeight: 0 }}>
        <NodeGraphHost node={node} onAction={onAction} />
      </div>
      <pre data-testid="node-graph-host-debug" style={{ margin: 0, padding: 4, fontSize: 11 }}>
        {JSON.stringify({ lastAction })}
      </pre>
    </div>
  );
}
//#endregion StoryHost

const meta = {
  title: "🛠️framework🔌️hosts/NodeGraphHost",
  component: NodeGraphStoryHost,
  parameters: { layout: "fullscreen" },
  tags: ["autodocs"],
} satisfies Meta<typeof NodeGraphStoryHost>;

export default meta;

type Story = StoryObj<typeof meta>;

/** 🕸️ Default DAG engine — `WasmGraphSurface` against the real `framework_surface_node_graph` `GraphSession`. */
export const Workflow: Story = {
  args: { scene: WORKFLOW_SCENE, controllerId: "s-play", surfaceId: "s.play.workflow" },
  parameters: { wasm: ["node-graph"] },
};

/** 🌊️ Flow engine — `FlowGraphCanvasHost` against the real `flow_core` `FlowSession`, routed by `capabilitiesJson.engine === "flow"` (`isFlowGraphScene`). */
export const FlowGraph: Story = {
  args: { scene: FLOW_GRAPH_SCENE, controllerId: "flow-play", surfaceId: "flow.play.canvas" },
  parameters: { wasm: ["flow"] },
};
