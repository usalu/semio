/** 🧮️ Unit cache mode of one release build attempt: on unless the dev opts out (`SEMIO_RELEASE_UNIT_CACHE=off`) or an earlier attempt failed early with it on (demotion marker). */
export function releaseUnitCacheModeV1(input: Readonly<{ optOut: string | undefined; demoted: boolean }>): "on" | "off" {
  return input.optOut === "off" || input.demoted ? "off" : "on";
}

/** 🛟️ Whether a failed attempt with the cache on failed so early (chain problem, not a compile problem) that it must be demoted and rerun with the cache off. */
export function releaseUnitCacheDemotesV1(input: Readonly<{ mode: "on" | "off"; status: number; elapsedMs: number; earlyMs: number }>): boolean {
  return input.mode === "on" && input.status !== 0 && input.elapsedMs < input.earlyMs;
}
