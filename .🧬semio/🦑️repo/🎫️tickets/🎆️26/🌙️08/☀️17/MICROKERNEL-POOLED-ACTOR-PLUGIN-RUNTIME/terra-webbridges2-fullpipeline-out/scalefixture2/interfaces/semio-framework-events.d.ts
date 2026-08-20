/** @module Interface semio:framework/events@1.0.0 **/
export type InstanceId = import('./semio-framework-types.js').InstanceId;
export type Pack = import('./semio-framework-types.js').Pack;
export type CapabilityGrant = import('./semio-framework-capabilities.js').CapabilityGrant;
export interface InstanceOpenEvent {
  instance: InstanceId,
  appId: string,
  actor: string,
  config: Pack,
  assets: Array<[string, Pack]>,
  capabilities: Array<CapabilityGrant>,
  quotas: Pack,
}
export interface InstanceCloseEvent {
  instance: InstanceId,
}
export type ActivationEvent = ActivationEventOnCommand | ActivationEventOnViewVisible | ActivationEventOnFileType | ActivationEventOnArtifactKind | ActivationEventOnExtensionRequest | ActivationEventOnStartupFinished;
export interface ActivationEventOnCommand {
  tag: 'on-command',
  val: string,
}
export interface ActivationEventOnViewVisible {
  tag: 'on-view-visible',
  val: string,
}
export interface ActivationEventOnFileType {
  tag: 'on-file-type',
  val: string,
}
export interface ActivationEventOnArtifactKind {
  tag: 'on-artifact-kind',
  val: string,
}
export interface ActivationEventOnExtensionRequest {
  tag: 'on-extension-request',
  val: string,
}
export interface ActivationEventOnStartupFinished {
  tag: 'on-startup-finished',
}
export interface ActivateEvent {
  instance: InstanceId,
  reason: ActivationEvent,
}
export interface SuspendRequestEvent {
  instance: InstanceId,
}
export type CapabilityChange = import('./semio-framework-capabilities.js').CapabilityChange;
export interface CapabilityChangedEvent {
  instance: InstanceId,
  change: CapabilityChange,
}
export interface QuotaChangedEvent {
  instance: InstanceId,
  quotas: Pack,
}
export interface AppCommandEvent {
  instance: InstanceId,
  seq: bigint,
  command: Pack,
}
export type SurfaceRef = import('./semio-framework-ui.js').SurfaceRef;
export interface SurfaceVisibleEvent {
  surface: SurfaceRef,
}
export interface SurfaceHiddenEvent {
  surface: SurfaceRef,
}
export interface SurfaceResizedEvent {
  surface: SurfaceRef,
  width: number,
  height: number,
}
export type Revision = import('./semio-framework-types.js').Revision;
export interface PatchAckEvent {
  surface: SurfaceRef,
  revision: Revision,
}
export interface PatchRejectedEvent {
  surface: SurfaceRef,
  revision: Revision,
  reason: string,
}
export type RequestId = import('./semio-framework-types.js').RequestId;
export type CompletionResult = CompletionResultOk | CompletionResultFault;
export interface CompletionResultOk {
  tag: 'ok',
  val: Pack,
}
export interface CompletionResultFault {
  tag: 'fault',
  val: Pack,
}
export interface CompletedEvent {
  req: RequestId,
  outcome: CompletionResult,
}
export interface HttpChunkParams {
  bytes: Pack,
  done: boolean,
}
export interface HttpChunkEvent {
  req: RequestId,
  params: HttpChunkParams,
}
export interface JobProgressEvent {
  job: bigint,
  progress: Pack,
}
export interface JobCompletedEvent {
  job: bigint,
  outcome: CompletionResult,
}
export type MessageEndpoint = import('./semio-framework-types.js').MessageEndpoint;
export interface MessageEvent {
  source: MessageEndpoint,
  payload: Pack,
}
export interface TimerEvent {
  id: bigint,
}
export interface RequestParams {
  origin: MessageEndpoint,
  capability: string,
  payload: Pack,
}
export interface RequestEvent {
  req: RequestId,
  params: RequestParams,
}
export type Event = EventInstanceOpen | EventInstanceClose | EventActivate | EventSuspendRequest | EventCapabilityChanged | EventQuotaChanged | EventAppCommand | EventSurfaceVisible | EventSurfaceHidden | EventSurfaceResized | EventPatchAck | EventPatchRejected | EventCompleted | EventHttpChunk | EventJobProgress | EventJobCompleted | EventMessage | EventTimer | EventWake | EventRequest;
export interface EventInstanceOpen {
  tag: 'instance-open',
  val: InstanceOpenEvent,
}
export interface EventInstanceClose {
  tag: 'instance-close',
  val: InstanceCloseEvent,
}
export interface EventActivate {
  tag: 'activate',
  val: ActivateEvent,
}
export interface EventSuspendRequest {
  tag: 'suspend-request',
  val: SuspendRequestEvent,
}
export interface EventCapabilityChanged {
  tag: 'capability-changed',
  val: CapabilityChangedEvent,
}
export interface EventQuotaChanged {
  tag: 'quota-changed',
  val: QuotaChangedEvent,
}
export interface EventAppCommand {
  tag: 'app-command',
  val: AppCommandEvent,
}
export interface EventSurfaceVisible {
  tag: 'surface-visible',
  val: SurfaceVisibleEvent,
}
export interface EventSurfaceHidden {
  tag: 'surface-hidden',
  val: SurfaceHiddenEvent,
}
export interface EventSurfaceResized {
  tag: 'surface-resized',
  val: SurfaceResizedEvent,
}
export interface EventPatchAck {
  tag: 'patch-ack',
  val: PatchAckEvent,
}
export interface EventPatchRejected {
  tag: 'patch-rejected',
  val: PatchRejectedEvent,
}
export interface EventCompleted {
  tag: 'completed',
  val: CompletedEvent,
}
export interface EventHttpChunk {
  tag: 'http-chunk',
  val: HttpChunkEvent,
}
export interface EventJobProgress {
  tag: 'job-progress',
  val: JobProgressEvent,
}
export interface EventJobCompleted {
  tag: 'job-completed',
  val: JobCompletedEvent,
}
export interface EventMessage {
  tag: 'message',
  val: MessageEvent,
}
export interface EventTimer {
  tag: 'timer',
  val: TimerEvent,
}
export interface EventWake {
  tag: 'wake',
}
export interface EventRequest {
  tag: 'request',
  val: RequestEvent,
}
