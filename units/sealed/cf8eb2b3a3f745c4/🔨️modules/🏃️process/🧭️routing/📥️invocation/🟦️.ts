import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import { compileJsonSchemaSubset, compileExactKeysV1, requireRecord } from "../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {types} from "node:util";

const scriptPolicyContract = compileJsonSchemaSubset(schema);
const immutablePolicies = new WeakSet<object>();
const admittedInvocations = new WeakSet<object>();
const invocationKeys=compileExactKeysV1(schema.definitions.ExpectedKeys.const.invocation),controlKeys=compileExactKeysV1(schema.definitions.ExpectedKeys.const.control);

/** 🧬️ Names the original caller's finite or ordinary no-deadline command authority. */
export type ScriptPolicy = Readonly<{ version: 1; owner: string; maximumElapsedMilliseconds: number }>;
/** 📣️ Identifies actual neutral command ownership transitions. */
export type ScriptProgress = Readonly<{ owner: string; command: string; stage: "contribution" | "loading" | "running" | "complete"; activity?: Readonly<{ resource: string; mode: "shared" | "exclusive"; elapsedMs: number }> }>;
/** 🎛️ Receives the caller's existing cancellation, time, progress and continuation ports. */
export interface ScriptControl {
  readonly signal: AbortSignal;
  remainingMilliseconds(): number | null;
  publish(progress: ScriptProgress): void | Promise<void>;
  yieldContinuation(): Promise<void>;
}
/** 📥️ Carries exact incoming control and application capabilities without creating authority. */
export interface ScriptInvocation<Capabilities extends object = object> {
  readonly policy: ScriptPolicy;
  readonly control: ScriptControl;
  readonly capabilities: Capabilities;
}

/** 🔐️ Admits a complete authored wire policy including explicit zero for no deadline. */
export function readScriptPolicy(value: unknown): ScriptPolicy {
  if (value !== null && typeof value === "object" && immutablePolicies.has(value)) return value as ScriptPolicy;
  if (scriptPolicyContract.validate(value).length) throw Error("Complete Script policy required");
  if (value !== null && typeof value === "object" && !types.isProxy(value) && Object.isFrozen(value) && [null,Object.prototype].includes(Object.getPrototypeOf(value)) && Reflect.ownKeys(value).length === 3 && Reflect.ownKeys(value).every(key=>{const descriptor=Object.getOwnPropertyDescriptor(value,key)!;return typeof key==="string"&&"value"in descriptor&&!descriptor.writable&&!descriptor.configurable&&["string","number"].includes(typeof descriptor.value);})) immutablePolicies.add(value);
  return value as ScriptPolicy;
}

/** ⏳️ Checks original incoming authority before acquiring or executing an owner. */
export function checkScriptInvocation<Capabilities extends object>(invocation: ScriptInvocation<Capabilities>): void {
  const row = requireRecord(invocation, "Script invocation");
  if (!admittedInvocations.has(row)) {
    invocationKeys(row, "Script invocation");
    const admitted = requireRecord(row.control, "Script control");
    controlKeys(admitted, "Script control");
    requireRecord(row.capabilities, "Script capabilities");
    if (!(admitted.signal instanceof AbortSignal) || typeof admitted.remainingMilliseconds !== "function" || typeof admitted.publish !== "function" || typeof admitted.yieldContinuation !== "function") throw Error("Original Script control ports required");
    if (!types.isProxy(row) && Object.isFrozen(row) && !types.isProxy(admitted) && Object.isFrozen(admitted)) admittedInvocations.add(row);
  }
  const policy = readScriptPolicy(row.policy);
  invocation.control.signal.throwIfAborted();
  const remaining = invocation.control.remainingMilliseconds();
  if (policy.maximumElapsedMilliseconds === 0 ? remaining !== null : remaining === null || !Number.isFinite(remaining) || remaining <= 0 || remaining > policy.maximumElapsedMilliseconds) throw Error("Original Script deadline refused");
}

/** 🔄️ Publishes and yields through the same caller-owned operation. */
export async function advanceScriptInvocation<Capabilities extends object>(invocation: ScriptInvocation<Capabilities>, command: string, stage: ScriptProgress["stage"]): Promise<void> {
  checkScriptInvocation(invocation);
  await invocation.control.publish({ owner: invocation.policy.owner, command, stage });
  checkScriptInvocation(invocation);
  await invocation.control.yieldContinuation();
  checkScriptInvocation(invocation);
}

/** ⏱️ Intersects a supplied child budget with the same original operation authority. */
export function scriptInvocationBudget<Capabilities extends object>(invocation: ScriptInvocation<Capabilities>, requestedMilliseconds: number): number {
  checkScriptInvocation(invocation);
  if (!Number.isSafeInteger(requestedMilliseconds) || requestedMilliseconds < 0) throw Error("Original child process budget refused");
  const remaining = invocation.control.remainingMilliseconds();
  if (remaining === null) return requestedMilliseconds;
  const budget = Math.floor(requestedMilliseconds === 0 ? remaining : Math.min(requestedMilliseconds, remaining));
  if (budget < 1) throw Error("Original child process deadline exhausted");
  return budget;
}
