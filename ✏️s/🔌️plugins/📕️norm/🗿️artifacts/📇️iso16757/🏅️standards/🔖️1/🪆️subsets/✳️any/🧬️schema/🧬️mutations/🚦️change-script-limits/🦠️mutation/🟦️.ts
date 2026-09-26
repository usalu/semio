/** mutation payload — mirrors `ChangeScriptLimits`. */
export interface ChangeScriptLimits {
  new_max_steps: number;
  new_max_recursion: number;
  new_timeout_ms: number;
}
