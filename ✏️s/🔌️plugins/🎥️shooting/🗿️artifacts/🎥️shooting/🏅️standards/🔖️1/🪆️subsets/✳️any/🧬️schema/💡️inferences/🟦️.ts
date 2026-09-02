/** 💡️ Shooting inference schema — topology derived from the shot→camera reference graph. */

export interface ShootingTopology {
  topoOrder: string[];
  depth: Record<string, number>;
  cycleFree: boolean;
  nodeCount: number;
}

export interface ShootingInference {
  /** @derived */
  topology: ShootingTopology;
}
