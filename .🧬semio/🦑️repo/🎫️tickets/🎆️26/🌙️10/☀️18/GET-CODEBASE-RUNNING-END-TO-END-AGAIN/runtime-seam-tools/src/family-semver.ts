import { requireRuntimeMemberV1 } from "../🧭️selection/🟦️.ts";

/** 🔢️ Tells whether a version satisfies a range. */
export function runtimeSemverSatisfiesV1(version: string, range: string): boolean { return requireRuntimeMemberV1("semverSatisfies")(version, range); }
/** 🔢️ Orders two versions. */
export function runtimeSemverOrderV1(left: string, right: string): -1 | 0 | 1 { return requireRuntimeMemberV1("semverOrder")(left, right); }
