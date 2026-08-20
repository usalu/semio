/** @module Interface semio:framework/reactor@1.0.0 **/
export function poll(events: Array<Event>, budget: Budget): Promise<TurnResult>;
export type PluginError = import('./semio-framework-types.js').PluginError;
export type Effect = import('./semio-framework-effects.js').Effect;
export type Event = import('./semio-framework-events.js').Event;
export type UiPatch = import('./semio-framework-ui.js').UiPatch;
export interface Budget {
  fuel: bigint,
  deadlineMs: number,
  maxEffects: number,
  maxPatchBytes: number,
  maxFrames: number,
}
export type TurnStatus = TurnStatusIdle | TurnStatusMoreWork | TurnStatusCheckpointReady | TurnStatusFaulted;
export interface TurnStatusIdle {
  tag: 'idle',
}
export interface TurnStatusMoreWork {
  tag: 'more-work',
}
export interface TurnStatusCheckpointReady {
  tag: 'checkpoint-ready',
}
export interface TurnStatusFaulted {
  tag: 'faulted',
  val: Uint8Array,
}
export interface TurnResult {
  uiPatches: Array<UiPatch>,
  effects: Array<Effect>,
  nextWake?: bigint,
  status: TurnStatus,
  fuelUsed: bigint,
}
