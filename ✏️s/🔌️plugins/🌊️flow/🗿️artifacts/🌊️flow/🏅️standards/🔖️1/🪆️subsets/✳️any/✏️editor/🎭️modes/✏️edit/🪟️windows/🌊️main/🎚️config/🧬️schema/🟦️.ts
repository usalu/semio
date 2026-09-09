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
export interface FlowMainWindowConfigMutation { kind: "snapshot"; config: FlowMainWindowConfig }

export const applyFlowMainWindowConfigMutation = (_base: FlowMainWindowConfig, mutation: FlowMainWindowConfigMutation): FlowMainWindowConfig => structuredClone(mutation.config);
