/** 🧬️ Wires snapshot schema — every field with its state class. */

export type DslValue = Record<string, unknown>;

export interface WiresSnapshot {
  /** @state persistent */
  wiresFixture: DslValue;
  /** @state persistent */
  boardFixture: DslValue;
}

export interface WiresStringList {
  values: string[];
}
