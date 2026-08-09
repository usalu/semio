/** 🧬️ WiresConfig */
export interface WiresConfig {
  /** @state local-ui */
  selectedIds: string[];
  /** @state local-ui */
  dragNodeId?: string;
  /** @state local-ui */
  dragLastX: number;
  /** @state local-ui */
  dragLastY: number;
  /** @state local-ui */
  locale: string;
}
