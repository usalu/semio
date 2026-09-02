/** 🧭 `topology` — one named inference: the shot→camera reference graph (saved cameras are roots
 * at depth 0, a shot resolving to a real saved camera sits at depth 1, always cycleFree). */

export interface ShootingTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}
