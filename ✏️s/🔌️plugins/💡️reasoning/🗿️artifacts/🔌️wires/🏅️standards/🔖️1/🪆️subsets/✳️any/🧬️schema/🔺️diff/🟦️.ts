/** 🧬️ Wires diff schema — sparse field delta. */

export type DslValue = Record<string, unknown>;

export interface WiresDiff {
  /** @state artifact */
  artifact?: WiresArtifact;
  /** @state artifact */
  wiresFixture?: DslValue;
  /** @state artifact */
  boardFixture?: DslValue;
  /** @state artifact */
  dragNodeId?: string | null;
  /** @state artifact */
  dragLastX?: number;
  /** @state artifact */
  dragLastY?: number;
  /** @state config */
  locale?: string;
}

export interface WiresArtifact {
  /** @state artifact */
  wiresFixture: DslValue;
  /** @state artifact */
  boardFixture: DslValue;
  /** @state artifact */
  dragNodeId?: string;
  /** @state artifact */
  dragLastX: number;
  /** @state artifact */
  dragLastY: number;
  /** @state config */
  locale: string;
}
