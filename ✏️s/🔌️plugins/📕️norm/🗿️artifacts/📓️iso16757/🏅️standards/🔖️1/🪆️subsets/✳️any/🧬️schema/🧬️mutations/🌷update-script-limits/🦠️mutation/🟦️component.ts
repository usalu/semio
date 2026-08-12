/** mutation payload — mirrors `UpdateScriptLimits`. */
export interface UpdateScriptLimits {
  newMaxSteps: number;
  newMaxRecursion: number;
  newTimeoutMs: number;
}
