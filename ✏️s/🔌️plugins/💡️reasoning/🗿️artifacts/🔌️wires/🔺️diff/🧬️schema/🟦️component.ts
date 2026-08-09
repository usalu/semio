/** 🧬️ Wires diff schema — sparse field delta. */

export type DslValue = Record<string, unknown>;

export interface WiresDiff {
  /** @state persistent */
  artifact?: WiresArtifact;
  /** @state persistent */
  wiresFixture?: DslValue;
  /** @state persistent */
  boardFixture?: DslValue;
  /** @state shared-ui */
  selectedIds?: WiresStringList;
  /** @state preview */
  dragNodeId?: string | null;
  /** @state preview */
  dragLastX?: number;
  /** @state preview */
  dragLastY?: number;
  /** @state local-ui */
  locale?: string;
}

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
