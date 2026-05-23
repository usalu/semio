/** 🚫BreachRecord is one lint finding emitted by a lint.script.ts (serialized to cache JSON). */
export type BreachPriority = "high" | "medium" | "low";

export type BreachRecord = {
  id: string;
  summary: string;
  /** Statute-style rule id (path or slug) for GraphQL `kindId` / cache round-trip. */
  kind: string;
  scope: string;
  line?: number;
  column?: number;
  excerpt?: string;
  priority?: BreachPriority;
  autofixable?: boolean;
  reason?: string;
  solution?: string;
};
