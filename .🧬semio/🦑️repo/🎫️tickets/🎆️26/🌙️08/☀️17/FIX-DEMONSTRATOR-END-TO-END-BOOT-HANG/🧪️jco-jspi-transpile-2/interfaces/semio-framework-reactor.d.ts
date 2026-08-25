/** @module Interface semio:framework/reactor@1.0.0 **/
export function poll(events: Array<Event>, commandPage: CommandIngressPage | undefined, budget: Budget): Promise<TurnResult>;
export type PluginError = import('./semio-framework-types.js').PluginError;
export type Pack = import('./semio-framework-types.js').Pack;
export type InstanceId = import('./semio-framework-types.js').InstanceId;
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
export interface JobCheckpoint {
  state: Uint8Array,
  appliedProgress: bigint,
}
export interface CommandPageCursor {
  owner: bigint,
  generation: bigint,
  commandIndex: number,
  commandCount: number,
  instance: InstanceId,
  seq: bigint,
  kind: number,
  pageIndex: number,
  pageCount: number,
  itemCount: number,
  metadata: number,
}
export interface CommandPageBlock {
  word0: bigint,
  word1: bigint,
  word2: bigint,
  word3: bigint,
  word4: bigint,
  word5: bigint,
  word6: bigint,
  word7: bigint,
}
export interface CommandIngressPage {
  cursor: CommandPageCursor,
  length: number,
  block00: CommandPageBlock,
  block01: CommandPageBlock,
  block02: CommandPageBlock,
  block03: CommandPageBlock,
  block04: CommandPageBlock,
  block05: CommandPageBlock,
  block06: CommandPageBlock,
  block07: CommandPageBlock,
  block08: CommandPageBlock,
  block09: CommandPageBlock,
  block10: CommandPageBlock,
  block11: CommandPageBlock,
  block12: CommandPageBlock,
  block13: CommandPageBlock,
  block14: CommandPageBlock,
  block15: CommandPageBlock,
  block16: CommandPageBlock,
  block17: CommandPageBlock,
  block18: CommandPageBlock,
  block19: CommandPageBlock,
  block20: CommandPageBlock,
  block21: CommandPageBlock,
  block22: CommandPageBlock,
  block23: CommandPageBlock,
  block24: CommandPageBlock,
  block25: CommandPageBlock,
  block26: CommandPageBlock,
  block27: CommandPageBlock,
  block28: CommandPageBlock,
  block29: CommandPageBlock,
  block30: CommandPageBlock,
  block31: CommandPageBlock,
  block32: CommandPageBlock,
  block33: CommandPageBlock,
  block34: CommandPageBlock,
  block35: CommandPageBlock,
  block36: CommandPageBlock,
  block37: CommandPageBlock,
  block38: CommandPageBlock,
  block39: CommandPageBlock,
  block40: CommandPageBlock,
  block41: CommandPageBlock,
  block42: CommandPageBlock,
  block43: CommandPageBlock,
  block44: CommandPageBlock,
  block45: CommandPageBlock,
  block46: CommandPageBlock,
  block47: CommandPageBlock,
  block48: CommandPageBlock,
  block49: CommandPageBlock,
  block50: CommandPageBlock,
  block51: CommandPageBlock,
  block52: CommandPageBlock,
  block53: CommandPageBlock,
  block54: CommandPageBlock,
  block55: CommandPageBlock,
  block56: CommandPageBlock,
  block57: CommandPageBlock,
  block58: CommandPageBlock,
  block59: CommandPageBlock,
  block60: CommandPageBlock,
  block61: CommandPageBlock,
  block62: CommandPageBlock,
  block63: CommandPageBlock,
}
export interface CommandIngressFault {
  cursor: CommandPageCursor,
  fault: PluginError,
}
export type CommandIngressStatus = CommandIngressStatusIdle | CommandIngressStatusPageAccepted | CommandIngressStatusBackpressure | CommandIngressStatusCommandPending | CommandIngressStatusCommandComplete | CommandIngressStatusFault;
export interface CommandIngressStatusIdle {
  tag: 'idle',
}
export interface CommandIngressStatusPageAccepted {
  tag: 'page-accepted',
  val: CommandPageCursor,
}
export interface CommandIngressStatusBackpressure {
  tag: 'backpressure',
  val: CommandPageCursor,
}
export interface CommandIngressStatusCommandPending {
  tag: 'command-pending',
  val: CommandPageCursor,
}
export interface CommandIngressStatusCommandComplete {
  tag: 'command-complete',
  val: CommandPageCursor,
}
export interface CommandIngressStatusFault {
  tag: 'fault',
  val: CommandIngressFault,
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
  val: JobCheckpoint,
}
export interface TurnStatusFaulted {
  tag: 'faulted',
  val: Uint8Array,
}
export interface PresenceUpdate {
  update: Pack,
}
export interface TurnResult {
  uiPatches: Array<UiPatch>,
  effects: Array<Effect>,
  presence: Array<PresenceUpdate>,
  nextWake?: bigint,
  status: TurnStatus,
  fuelUsed: bigint,
  commandIngress: CommandIngressStatus,
}
