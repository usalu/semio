/** mutation payload — mirrors `UpdateScriptLimits`. */
export interface UpdateScriptLimits {
  new_max_steps: number;
  new_max_recursion: number;
  new_timeout_ms: number;
}
