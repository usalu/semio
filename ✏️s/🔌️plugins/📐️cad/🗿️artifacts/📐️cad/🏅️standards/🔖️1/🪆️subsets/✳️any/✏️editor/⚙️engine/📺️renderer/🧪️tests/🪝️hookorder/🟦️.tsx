/** 🪝️ Rules-of-hooks regression suite for the CAD renderer: every component renders across prop changes that used to flip an early return, React must never report a changed hook count, and every render must call the same number of hooks (React itself only notices some of the violations). Registered by the in-source block of {@link ../../🟦️.tsx | the renderer}. */
import { render } from "@testing-library/react";
import { Component, type ReactNode } from "react";
import type { RendererTestDependencies } from "../../🎒️test-dependencies/🟦️.ts";

type TestSource = { readonly url: string };
type Vitest = NonNullable<ImportMeta["vitest"]>;
type HookHost = Record<string, (...args: unknown[]) => unknown>;
type HookTrace = { readonly faults: readonly string[]; readonly hookCalls: readonly number[] };

const HOOK_FAULT = /Rendered (more|fewer) hooks|change in the order of Hooks|accidental early return/i;
const HOOK_NAMES = ["useCallback", "useContext", "useEffect", "useId", "useLayoutEffect", "useMemo", "useReducer", "useRef", "useState", "useSyncExternalStore"] as const;

/** 🪝️ Error boundary that records every render-phase error message instead of unmounting the test tree. */
class FaultCapture extends Component<{ readonly faults: string[]; readonly children: ReactNode }, { readonly failed: boolean }> {
  override state = { failed: false };

  static getDerivedStateFromError(): { readonly failed: boolean } {
    return { failed: true };
  }

  override componentDidCatch(error: Error): void {
    this.props.faults.push(error.message);
  }

  override render(): ReactNode {
    return this.state.failed ? null : this.props.children;
  }
}

function traceHookOrder(vitest: Vitest, reactHostPort: RendererTestDependencies["reactHostPort"], frames: readonly ReactNode[]): HookTrace {
  const faults: string[] = [];
  const host = reactHostPort as unknown as HookHost;
  const errorSpy = vitest.vi.spyOn(console, "error").mockImplementation((...args: unknown[]) => {
    faults.push(args.map(String).join(" "));
  });
  const hookSpies = HOOK_NAMES.filter((name) => typeof host[name] === "function").map((name) => vitest.vi.spyOn(host, name));
  const hookCalls: number[] = [];
  const callsSoFar = () => hookSpies.reduce((sum, spy) => sum + spy.mock.calls.length, 0);
  try {
    let before = callsSoFar();
    const view = render(<FaultCapture faults={faults}>{frames[0]}</FaultCapture>);
    hookCalls.push(callsSoFar() - before);
    for (const frame of frames.slice(1)) {
      before = callsSoFar();
      view.rerender(<FaultCapture faults={faults}>{frame}</FaultCapture>);
      hookCalls.push(callsSoFar() - before);
    }
    view.unmount();
  } finally {
    for (const spy of hookSpies) spy.mockRestore();
    errorSpy.mockRestore();
  }
  return { faults: faults.filter((message) => HOOK_FAULT.test(message)), hookCalls };
}

export async function registerTests2(vitest: Vitest, dependencies: RendererTestDependencies, _source: TestSource): Promise<void> {
  const { PreviewItem, SpatialPickTargetNode, TessellatedCommitMesh, emptyMeshTransfer, reactHostPort } = dependencies;
  const { describe, it, expect } = vitest;

  const trace = (frames: readonly ReactNode[]) => traceHookOrder(vitest, reactHostPort, frames);
  const expectStableHooks = (frames: readonly ReactNode[]) => {
    const { faults, hookCalls } = trace(frames);
    expect(faults).toEqual([]);
    expect(hookCalls.length).toBe(frames.length);
    expect(new Set(hookCalls).size).toBe(1);
  };

  describe("rules of hooks", () => {
    it("the harness sees a component whose hook count depends on its props", () => {
      function ConditionalHookProbe({ skip }: { readonly skip: boolean }): ReactNode {
        if (skip) return null;
        (reactHostPort as unknown as HookHost)["useMemo"]!(() => 0, []);
        return null;
      }
      const { hookCalls } = trace([<ConditionalHookProbe key="probe" skip />, <ConditionalHookProbe key="probe" skip={false} />]);
      expect(new Set(hookCalls).size).toBe(2);
    });

    it("PreviewItem calls the same hooks whether or not the item has params", () => {
      const withoutParams = { id: "preview-1", kind: "preview" } as const;
      const withParams = { id: "preview-1", kind: "preview", params: { previewKind: "hook-order-probe" } } as const;
      expectStableHooks([<PreviewItem key="p" item={withoutParams} />, <PreviewItem key="p" item={withParams} />, <PreviewItem key="p" item={withoutParams} />]);
    });

    it("SpatialPickTargetNode calls the same hooks for vertex and edge targets", () => {
      const vertex = { kind: "vertex", id: "v1", point: [0, 0, 0] } as const;
      const edge = { kind: "edge", id: "e1", point: [0, 0, 0] } as const;
      expectStableHooks([<SpatialPickTargetNode key="n" target={vertex} />, <SpatialPickTargetNode key="n" target={edge} />, <SpatialPickTargetNode key="n" target={vertex} />]);
    });

    it("TessellatedCommitMesh calls the same hooks whether faces and edges are shown or hidden", () => {
      const mesh = emptyMeshTransfer();
      expectStableHooks([<TessellatedCommitMesh key="m" mesh={mesh} />, <TessellatedCommitMesh key="m" mesh={mesh} showFaces={false} showEdges={false} />, <TessellatedCommitMesh key="m" mesh={mesh} />]);
    });
  });
}
