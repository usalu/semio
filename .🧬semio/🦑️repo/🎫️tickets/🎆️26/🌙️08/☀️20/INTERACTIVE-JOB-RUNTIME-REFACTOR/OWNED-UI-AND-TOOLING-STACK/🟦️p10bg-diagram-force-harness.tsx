//#region 🔌️Adapters
import * as React from "react";
import { createRoot } from "react-dom/client";
import "@xyflow/react/dist/style.css";
import { Diagram, type DiagramHandoffStatus } from "/@fs/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Diagram/🟦️component.tsx";
//#endregion 🔌️Adapters

//#region 📊️Telemetry
interface HarnessTelemetry {
  callbackInAnimationFrame: boolean;
  callbackInPointerStack: boolean;
  datasetFrameMaxMs: number;
  datasetReady: boolean;
  dragCalls: number;
  dragSelectionLength: number;
  dragStopCalls: number;
  firstPublicationReady: boolean;
  handoffViolations: ReturnType<DiagramHandoffStatus["violations"]>;
  hostEdgeCount: number;
  hostNodeCount: number;
  hostReady: boolean;
  lastPublicationLength: number;
  maximumAnimationFrameGapMs: number;
  maximumConsumerMs: number;
  maximumPointerConsumerMs: number;
  mounted: boolean;
  pointerCaptureCallbacks: number;
  publicationCalls: number;
  publicationReads: number;
  slowConsumerArmed: boolean;
  slowPointerArmed: boolean;
}

interface HarnessApi {
  armSlowConsumer(): void;
  armSlowPointer(): void;
  snapshot(): HarnessTelemetry;
}

declare global {
  interface Window {
    __P10_DIAGRAM_FORCE__: HarnessApi;
  }
}

const telemetry: HarnessTelemetry = {
  callbackInAnimationFrame: false,
  callbackInPointerStack: false,
  datasetFrameMaxMs: 0,
  datasetReady: false,
  dragCalls: 0,
  dragSelectionLength: 0,
  dragStopCalls: 0,
  firstPublicationReady: false,
  handoffViolations: [],
  hostEdgeCount: 0,
  hostNodeCount: 0,
  hostReady: false,
  lastPublicationLength: 0,
  maximumAnimationFrameGapMs: 0,
  maximumConsumerMs: 0,
  maximumPointerConsumerMs: 0,
  mounted: false,
  pointerCaptureCallbacks: 0,
  publicationCalls: 0,
  publicationReads: 0,
  slowConsumerArmed: false,
  slowPointerArmed: false,
};

let animationFrameStack = false;
let pointerStack = false;
const nativeAnimationFrame = window.requestAnimationFrame.bind(window);
window.requestAnimationFrame = (callback: FrameRequestCallback): number =>
  nativeAnimationFrame((time) => {
    animationFrameStack = true;
    try {
      callback(time);
    } finally {
      animationFrameStack = false;
    }
  });

for (const eventName of ["mousedown", "mousemove", "mouseup", "pointerdown", "pointermove", "pointerup"]) {
  document.addEventListener(eventName, () => (pointerStack = true), true);
  document.addEventListener(eventName, () => (pointerStack = false));
}

window.__P10_DIAGRAM_FORCE__ = {
  armSlowConsumer() {
    telemetry.slowConsumerArmed = true;
  },
  armSlowPointer() {
    telemetry.slowPointerArmed = true;
  },
  snapshot() {
    return structuredClone(telemetry);
  },
};
//#endregion 📊️Telemetry

//#region 🧪️Harness
type HarnessNode = { data: Record<string, never>; id: string; position: { x: number; y: number }; selected: boolean };
type HarnessEdge = { id: string; source: string; target: string };

function block(milliseconds: number): void {
  const until = performance.now() + milliseconds;
  while (performance.now() < until) {}
}

function nextFrame(): Promise<number> {
  return new Promise((resolve) => nativeAnimationFrame(resolve));
}

async function createDataset(): Promise<{ edges: HarnessEdge[]; nodes: HarnessNode[] }> {
  const nodes: HarnessNode[] = [];
  const edges: HarnessEdge[] = [];
  for (let start = 0; start < 20_000; start += 500) {
    const frameStarted = performance.now();
    const end = Math.min(20_000, start + 500);
    for (let index = start; index < end; index++) {
      const id = `node-${index.toString().padStart(5, "0")}`;
      nodes.push({ data: {}, id, position: { x: (index % 500) * 80, y: Math.floor(index / 500) * 80 }, selected: index < 3_001 });
      edges.push({ id: `edge-${index.toString().padStart(5, "0")}`, source: id, target: `node-${((index + 1) % 20_000).toString().padStart(5, "0")}` });
    }
    telemetry.datasetFrameMaxMs = Math.max(telemetry.datasetFrameMaxMs, performance.now() - frameStarted);
    await nextFrame();
  }
  telemetry.datasetReady = true;
  return { edges, nodes };
}

const TelemetryPanel: React.FC<{ statusRef: React.RefObject<DiagramHandoffStatus | null> }> = ({ statusRef }) => {
  const [, refresh] = React.useReducer((value) => value + 1, 0);
  React.useEffect(() => {
    const handle = window.setInterval(() => {
      telemetry.handoffViolations = statusRef.current?.violations() ?? [];
      telemetry.hostNodeCount = document.querySelectorAll(".react-flow__node").length;
      telemetry.hostEdgeCount = document.querySelectorAll(".react-flow__edge").length;
      telemetry.hostReady = document.querySelector(".react-flow") !== null;
      refresh();
    }, 100);
    return () => window.clearInterval(handle);
  }, [statusRef]);
  return (
    <aside style={{ position: "absolute", zIndex: 20, right: 8, top: 8, width: 440, maxHeight: "44%", overflow: "auto", padding: 10, borderRadius: 6, background: "rgba(3, 17, 22, .92)" }}>
      <div style={{ display: "flex", gap: 8, marginBottom: 8 }}>
        <button id="arm-slow-consumer" onClick={() => window.__P10_DIAGRAM_FORCE__.armSlowConsumer()}>
          Arm slow publication
        </button>
        <button id="arm-slow-pointer" onClick={() => window.__P10_DIAGRAM_FORCE__.armSlowPointer()}>
          Arm slow drag
        </button>
      </div>
      <pre id="p10-telemetry" style={{ whiteSpace: "pre-wrap", margin: 0 }}>
        {JSON.stringify(telemetry, null, 2)}
      </pre>
    </aside>
  );
};

const Harness: React.FC<{ edges: HarnessEdge[]; nodes: HarnessNode[] }> = ({ edges, nodes }) => {
  const statusRef = React.useRef<DiagramHandoffStatus | null>(null);
  React.useEffect(() => {
    telemetry.mounted = true;
    let previous = performance.now();
    let active = true;
    const observe = (time: number) => {
      telemetry.maximumAnimationFrameGapMs = Math.max(telemetry.maximumAnimationFrameGapMs, time - previous);
      previous = time;
      if (active) nativeAnimationFrame(observe);
    };
    nativeAnimationFrame(observe);
    return () => {
      active = false;
    };
  }, []);
  return (
    <main style={{ position: "relative", width: "100%", height: "100%" }}>
      <TelemetryPanel statusRef={statusRef} />
      <section id="p10-diagram-surface" style={{ position: "absolute", inset: 0 }}>
        <Diagram
          nodeTypes={{}}
          nodes={nodes}
          edges={edges}
          handoffStatusRef={statusRef}
          forceConfig={{ enabled: true, chargeStrength: 0, linkDistance: 60, collideRadius: 0, centerStrength: 0.02, updateIntervalMs: 50 }}
          onNodesChange={(proposal) => {
            const started = performance.now();
            telemetry.callbackInAnimationFrame ||= animationFrameStack;
            telemetry.callbackInPointerStack ||= pointerStack;
            telemetry.publicationCalls += 1;
            telemetry.lastPublicationLength = proposal.length;
            for (let index = 0; index < proposal.length; index++) if (proposal[index]) telemetry.publicationReads += 1;
            telemetry.firstPublicationReady ||= proposal.length === 20_000;
            if (telemetry.slowConsumerArmed) {
              telemetry.slowConsumerArmed = false;
              block(12);
            }
            telemetry.maximumConsumerMs = Math.max(telemetry.maximumConsumerMs, performance.now() - started);
          }}
          onNodeDragStart={() => {
            telemetry.pointerCaptureCallbacks += 1;
            telemetry.callbackInPointerStack ||= pointerStack;
          }}
          onNodeDrag={(_event, _node, selection) => {
            const started = performance.now();
            telemetry.callbackInAnimationFrame ||= animationFrameStack;
            telemetry.callbackInPointerStack ||= pointerStack;
            telemetry.dragCalls += 1;
            telemetry.dragSelectionLength = selection.length;
            for (let index = 0; index < selection.length; index++) selection[index];
            if (telemetry.slowPointerArmed) {
              telemetry.slowPointerArmed = false;
              block(12);
            }
            telemetry.maximumPointerConsumerMs = Math.max(telemetry.maximumPointerConsumerMs, performance.now() - started);
          }}
          onNodeDragStop={() => {
            telemetry.dragStopCalls += 1;
            telemetry.callbackInPointerStack ||= pointerStack;
          }}
        />
      </section>
    </main>
  );
};

void createDataset().then(({ edges, nodes }) => createRoot(document.getElementById("root")!).render(<Harness edges={edges} nodes={nodes} />));
//#endregion 🧪️Harness
