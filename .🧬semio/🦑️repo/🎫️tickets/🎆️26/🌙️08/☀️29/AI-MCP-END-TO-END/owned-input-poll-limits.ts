import assert from "node:assert/strict";

const retainedKeys = ["maximumItems", "maximumCopyBytes", "maximumCapacityBytes", "maximumReleaseBytes", "maximumDepth"] as const;
const requestKeys = ["workFuel", "fuel", "deadlineUs", "maxEffects", "maxPatchBytes", "maxFrames"] as const;

export type PollLimitsObject = Record<typeof requestKeys[number], number> & {
  retainedPolicy: Record<typeof retainedKeys[number], number>;
};

export function diagnosticPollLimitsObject(limits: PollLimitsObject | number[]): PollLimitsObject {
  if (!Array.isArray(limits)) return limits;
  assert.equal(limits.length, retainedKeys.length + requestKeys.length);
  return {
    retainedPolicy: Object.fromEntries(retainedKeys.map((key, index) => [key, limits[index]])),
    ...Object.fromEntries(requestKeys.map((key, index) => [key, limits[retainedKeys.length + index]])),
  } as PollLimitsObject;
}

export function diagnosticPollLimitsTuple(limits: PollLimitsObject): number[] {
  return [...retainedKeys.map(key => limits.retainedPolicy[key]), ...requestKeys.map(key => limits[key])];
}
