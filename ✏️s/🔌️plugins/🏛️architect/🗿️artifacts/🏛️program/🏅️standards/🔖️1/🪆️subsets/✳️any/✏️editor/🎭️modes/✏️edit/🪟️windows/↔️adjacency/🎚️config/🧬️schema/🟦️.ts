/** ↔️ Architect adjacency kind identity. */
export type ArchitectAdjacencyKind = "required" | "preferred" | "optional" | "prohibited";

/** 🔍️ Persisted filter for one exact Architect Adjacency window. */
export interface ArchitectAdjacencyWindowConfig { adjacencyKindFilter?: ArchitectAdjacencyKind }

/** 🔁️ Exact Adjacency-window filter mutation. */
export type ArchitectAdjacencyWindowConfigMutation = { kind: "set-adjacency-kind-filter"; adjacencyKindFilter?: ArchitectAdjacencyKind };

const kinds = new Set<ArchitectAdjacencyKind>(["required", "preferred", "optional", "prohibited"]);

/** 🚪️ Parses one exact Adjacency-window configuration. */
export function parseArchitectAdjacencyWindowConfig(value: unknown): ArchitectAdjacencyWindowConfig {
  if (value === null || typeof value !== "object" || Array.isArray(value)) throw new TypeError("$ must be an object");
  const row = value as Record<string, unknown>;
  if (Object.keys(row).some((key) => key !== "adjacencyKindFilter")) throw new TypeError("$ contains an unknown field");
  if (row.adjacencyKindFilter === undefined) return {};
  if (!kinds.has(row.adjacencyKindFilter as ArchitectAdjacencyKind)) throw new TypeError("$.adjacencyKindFilter is invalid");
  return { adjacencyKindFilter: row.adjacencyKindFilter as ArchitectAdjacencyKind };
}

/** 🧬️ Applies one Adjacency-window filter mutation. */
export function applyArchitectAdjacencyWindowConfigMutation(_base: ArchitectAdjacencyWindowConfig, mutation: ArchitectAdjacencyWindowConfigMutation): ArchitectAdjacencyWindowConfig {
  return parseArchitectAdjacencyWindowConfig(mutation.adjacencyKindFilter === undefined ? {} : { adjacencyKindFilter: mutation.adjacencyKindFilter });
}
