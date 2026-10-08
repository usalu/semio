/** 🎚️ Persisted local settings for one concrete Flow main window. */
export interface CameraJson { x: number; y: number; zoom: number }
export interface FlowMainWindowConfig {
  previewOffNodeIds: string[];
  camera: CameraJson;
  lodMode: string;
  proximityDistance: number;
  gridVisible: boolean;
  gridSnapEnabled: boolean;
  gridFactor: number;
  catalogueSectionsJson: string;
  automationEnabledJson: string;
}
export type FlowMainWindowConfigMutation =
  | { kind: "set-preview-off-node-ids"; value: string[] }
  | { kind: "set-camera"; value: CameraJson }
  | { kind: "set-lod-mode"; value: string }
  | { kind: "set-proximity-distance"; value: number }
  | { kind: "set-grid-visible"; value: boolean }
  | { kind: "set-grid-snap-enabled"; value: boolean }
  | { kind: "set-grid-factor"; value: number }
  | { kind: "set-catalogue-sections-json"; value: string }
  | { kind: "set-automation-enabled-json"; value: string };

export const applyFlowMainWindowConfigMutation = (base: FlowMainWindowConfig, mutation: FlowMainWindowConfigMutation): FlowMainWindowConfig => {
  const next = structuredClone(base);
  switch (mutation.kind) {
    case "set-preview-off-node-ids":
      next.previewOffNodeIds = structuredClone(mutation.value);
      break;
    case "set-camera":
      next.camera = structuredClone(mutation.value);
      break;
    case "set-lod-mode":
      next.lodMode = structuredClone(mutation.value);
      break;
    case "set-proximity-distance":
      next.proximityDistance = structuredClone(mutation.value);
      break;
    case "set-grid-visible":
      next.gridVisible = structuredClone(mutation.value);
      break;
    case "set-grid-snap-enabled":
      next.gridSnapEnabled = structuredClone(mutation.value);
      break;
    case "set-grid-factor":
      next.gridFactor = structuredClone(mutation.value);
      break;
    case "set-catalogue-sections-json":
      next.catalogueSectionsJson = structuredClone(mutation.value);
      break;
    case "set-automation-enabled-json":
      next.automationEnabledJson = structuredClone(mutation.value);
      break;
  }
  return next;
};
