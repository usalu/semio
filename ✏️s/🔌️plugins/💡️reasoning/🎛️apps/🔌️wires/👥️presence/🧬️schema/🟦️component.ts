/** 🧬️ WiresPresence */
export interface WiresPresence {
  /** @state shared-ui */
  selectedIds: string[];
  /** @state shared-ui */
  dragNodeId?: string;
  /** @state shared-ui */
  dragLastX: number;
  /** @state shared-ui */
  dragLastY: number;
}
