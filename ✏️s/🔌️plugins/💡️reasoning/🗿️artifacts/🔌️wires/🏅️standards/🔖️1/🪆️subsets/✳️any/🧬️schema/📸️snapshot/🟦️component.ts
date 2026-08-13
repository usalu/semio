/** 🧬️ Wires snapshot schema — every field with its state class. */

export type DslValue = Record<string, unknown>;

export interface WiresSnapshot {
  /** @state artifact */
  wiresFixture: DslValue;
  /** @state artifact */
  boardFixture: DslValue;
}

export interface WiresStringList {
  values: string[];
}
