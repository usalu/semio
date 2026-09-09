/** 🫧️ Ephemeral local Flow state for one concrete invoking window. */
export interface FlowWindowTransient { generationJson: string; duplicateWidgetProgressJson: string }
export interface FlowWindowTransientMutation { kind: "snapshot"; transient: FlowWindowTransient }
export const applyFlowWindowTransientMutation = (_base: FlowWindowTransient, mutation: FlowWindowTransientMutation): FlowWindowTransient => structuredClone(mutation.transient);
