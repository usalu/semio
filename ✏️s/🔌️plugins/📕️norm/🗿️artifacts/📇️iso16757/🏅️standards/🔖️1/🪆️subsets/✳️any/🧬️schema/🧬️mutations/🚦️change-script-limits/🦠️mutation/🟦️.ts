/** mutation payload — mirrors `ChangeScriptLimits`. */
export interface ChangeScriptLimits {
  newMaxSteps: number;
  newMaxRecursion: number;
  newTimeoutMs: number;
}
