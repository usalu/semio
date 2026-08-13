/** 🧩️ AssemblySnapshot schema — real facet mirror of the Rust `🦀️component.rs` sibling. `modules`
 * compose `kit` content (owned `ArtifactChild` handles — no snapshot content ever embedded here);
 * `rules.params` is `value`-shaped structured data, never a bespoke per-constraint-kind type. */
export interface ArtifactChildHandle { childId: string; target: string; }
export type SemioValue =
  | { kind: "null" }
  | { kind: "bool"; value: boolean }
  | { kind: "int"; lexeme: string }
  | { kind: "float"; lexeme: string }
  | { kind: "str"; value: string }
  | { kind: "bytes"; value: number[] }
  | { kind: "list"; items: SemioValue[] }
  | { kind: "map"; entries: { key: string; value: SemioValue }[] }
  | { kind: "ref"; id: { value: string } };

export interface AssemblySlot {
  id: string;
  x: number;
  y: number;
  z: number;
  pinnedModuleId?: string;
}

export interface AssemblySlotEdge {
  id: string;
  fromSlotId: string;
  toSlotId: string;
}

export interface AssemblyModuleWeight {
  moduleId: string;
  weight: number;
}

export interface AssemblyRule {
  id: string;
  moduleAId: string;
  moduleBId: string;
  allowed: boolean;
  params: SemioValue;
}

export interface AssemblySnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ seed: number;
  /** @state persistent */ slots: AssemblySlot[];
  /** @state persistent */ edges: AssemblySlotEdge[];
  /** @state persistent @child kind=s.stdio.semio.kit many */ modules: ArtifactChildHandle[];
  /** @state persistent */ weights: AssemblyModuleWeight[];
  /** @state persistent */ rules: AssemblyRule[];
}
