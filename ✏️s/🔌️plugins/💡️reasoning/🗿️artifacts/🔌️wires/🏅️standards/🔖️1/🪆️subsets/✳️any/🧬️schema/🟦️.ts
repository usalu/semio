/** 🧬️ Wires artifact schema — every field with its state class. */

export type DslValue = Record<string, unknown>;

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
