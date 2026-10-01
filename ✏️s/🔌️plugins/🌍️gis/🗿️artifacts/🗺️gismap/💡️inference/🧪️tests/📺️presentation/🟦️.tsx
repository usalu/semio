import { cleanup, fireEvent, render, screen } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { InferencePortPanel } from "../../🪟️presentation/🟦️.tsx";
import type { GisMapInferencePortStatusV1 } from "../../🧬️schema/🟦️.ts";
afterEach(cleanup);

const inferencePreview = {
  schema: "semio.hub.gis-map-inference-preview/v1",
  jobId: "0123456789abcdef0123456789abcdef",
  proposalHash: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
  regionId: "inference-0123456789abcdef0123456789abcdef",
  ring: [[-73.99, 40.71], [-73.97, 40.71], [-73.97, 40.73], [-73.99, 40.73], [-73.99, 40.71]],
} as const;

function offeredInferenceStatus(preview: typeof inferencePreview | undefined): GisMapInferencePortStatusV1 {
  return {
    phase: "offered",
    jobId: inferencePreview.jobId,
    cursor: 4,
    completed: 3,
    total: 3,
    proposalHash: inferencePreview.proposalHash,
    ...(preview === undefined ? {} : { preview }),
    cancelRequested: false,
    code: null,
  };
}

describe("GIS inference presentation", () => {
  it("renders only the Hub-validated bounds preview and withholds blind approval", () => {
    const onAction = vi.fn();
    const { container, rerender } = render(<InferencePortPanel status={offeredInferenceStatus(undefined)} locale="en" onAction={onAction} />);
    expect(Array.from(container.querySelectorAll("button"), (button) => button.textContent)).not.toContain("Approve proposal");
    expect(Array.from(container.querySelectorAll("button"), (button) => button.textContent)).not.toContain("Reject proposal");
    expect(container.querySelector("[data-semio-inference-preview]")).toBeNull();
    expect(container.querySelector("[data-semio-inference-overlay]")).toBeNull();

    rerender(<InferencePortPanel status={offeredInferenceStatus(inferencePreview)} locale="de" onAction={onAction} />);
    expect(screen.getByText("Gebiet")).toBeTruthy();
    expect(screen.getByText(inferencePreview.regionId)).toBeTruthy();
    expect(Array.from(container.querySelectorAll("dd"), (cell) => cell.textContent)).toEqual([
      inferencePreview.regionId,
      "-73.99–-73.97",
      "40.71–40.73",
    ]);
    expect(container.querySelector("[data-semio-inference-overlay]")?.getAttribute("aria-label")).toBe("Vorgeschlagene Grenzen auf der Karte");
    expect(container.querySelector("[data-semio-inference-overlay] path")?.getAttribute("d")).toBe("M 0 100 L 100 100 L 100 0 L 0 0 L 0 100 Z");
    fireEvent.click(screen.getByRole("button", { name: "Vorschlag ablehnen" }));
    expect(onAction).toHaveBeenCalledWith({ kind: "cancel" });
    fireEvent.click(screen.getByRole("button", { name: "Vorschlag freigeben" }));
    expect(onAction).toHaveBeenCalledWith({ kind: "approve" });
  });

  it("lets a map document request inference and keeps cancel on the in-flight job", () => {
    const onAction = vi.fn();
    const { container, rerender } = render(
      <InferencePortPanel
        status={{ phase: "idle", jobId: null, cursor: 0, completed: 0, total: 0, proposalHash: null, cancelRequested: false, code: null }}
        locale="en"
        onAction={onAction}
      />,
    );
    fireEvent.click(screen.getByRole("button", { name: "Request bounds proposal" }));
    expect(onAction).toHaveBeenCalledWith({ kind: "propose", payload: { requestId: expect.any(String) } });
    rerender(
      <InferencePortPanel
        status={{ phase: "running", jobId: inferencePreview.jobId, cursor: 1, completed: 1, total: 4, proposalHash: null, cancelRequested: false, code: null }}
        locale="de"
        onAction={onAction}
      />,
    );
    expect(container.querySelector("[data-semio-inference-overlay]")).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: "Vorschlag abbrechen" }));
    expect(onAction).toHaveBeenCalledWith({ kind: "cancel" });
  });

});
