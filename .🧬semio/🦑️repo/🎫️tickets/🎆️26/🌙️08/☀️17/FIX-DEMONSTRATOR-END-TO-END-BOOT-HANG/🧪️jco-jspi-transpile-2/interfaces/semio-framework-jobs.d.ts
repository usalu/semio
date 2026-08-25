/** @module Interface semio:framework/jobs@1.0.0 **/
export function startJob(job: bigint, kind: string, input: Uint8Array): Promise<void>;
export function stepJob(job: bigint, budget: JobBudget): Promise<JobStep>;
export function cancelJob(job: bigint): Promise<void>;
export function takeSegmentedDownloadChunk(instanceId: number, operationId: bigint): Promise<Uint8Array | undefined>;
export type PluginError = import('./semio-framework-types.js').PluginError;
export interface JobBudget {
  fuel: bigint,
  deadlineMs: number,
}
export type JobStep = JobStepRunning | JobStepDone | JobStepFailed;
export interface JobStepRunning {
  tag: 'running',
  val: Uint8Array | undefined,
}
export interface JobStepDone {
  tag: 'done',
  val: Uint8Array,
}
export interface JobStepFailed {
  tag: 'failed',
  val: Uint8Array,
}
