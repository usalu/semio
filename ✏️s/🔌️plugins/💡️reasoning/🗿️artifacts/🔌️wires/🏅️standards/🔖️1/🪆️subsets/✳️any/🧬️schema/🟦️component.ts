/** 🧬️ Wires artifact schema — every field with its state class. */

export type DslValue = Record<string, unknown>;

export interface WiresArtifact {
  /** @state persistent */
  wiresFixture: DslValue;
  /** @state persistent */
  boardFixture: DslValue;
  /** @state shared-ui */
  selectedIds: string[];
  /** @state preview */
  dragNodeId?: string;
  /** @state preview */
  dragLastX: number;
  /** @state preview */
  dragLastY: number;
  /** @state local-ui */
  locale: string;
}

export interface WiresStringList {
  values: string[];
}
