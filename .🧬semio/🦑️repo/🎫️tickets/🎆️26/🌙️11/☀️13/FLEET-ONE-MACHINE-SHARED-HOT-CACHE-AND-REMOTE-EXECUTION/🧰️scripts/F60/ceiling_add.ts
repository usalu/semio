
/** 📈️ The memory facts of one machine that decide whether it can hold a unit: its total, the free-memory floor and the interactive reserve it keeps back (`hostFloorMegabytesV1`, `hostInteractiveReserveMegabytesV1` of the host-capacity policy), the share of the rest the policy lets reservations take, and, for the machine the unit runs on, what is available now. */
export type GrowthHostV1 = Readonly<{ origin: string; totalMegabytes: number; floorMegabytes: number; interactiveReserveMegabytes: number; budgetBasisPoints: number; availableMegabytes?: number }>;

/** 📈️ What a unit needing more than its memory grant is told: `fits` (the grant carries it), `grow` (this host has the room now: ask for `extraMegabytes` more), `wait` (this host can hold it but the free memory beside the reserves is short by now: wait for growth), `beyond-host` (no memory this host can ever hold carries it, a larger machine of the fleet does: queue it there), `refused` (no machine of the fleet can hold it; `largestMegabytes` is the most the largest holds). */
export type GrantGrowthVerdictV1 = Readonly<{ kind: "fits" } | { kind: "grow"; extraMegabytes: number; roomMegabytes: number; holdableMegabytes: number } | { kind: "wait"; extraMegabytes: number; roomMegabytes: number; holdableMegabytes: number } | { kind: "beyond-host"; holdableMegabytes: number; largestMegabytes: number; largestOrigin: string } | { kind: "refused"; largestMegabytes: number; largestOrigin: string }>;

/** 🧱️ The most memory reservations on one machine can ever hold: its total less the floor and the interactive reserve, times the share the policy lets reservations take (the same arithmetic as the host-capacity decision).
 * @see ../../../../🏗️build/🌡️host-capacity/📏️decision/🟦️.ts (`decideHostCapacityV1`) */
export function hostHoldableMegabytesV1(host: Pick<GrowthHostV1, "totalMegabytes" | "floorMegabytes" | "interactiveReserveMegabytes" | "budgetBasisPoints">): number {
  return Math.floor(Math.max(0, host.totalMegabytes - host.floorMegabytes - host.interactiveReserveMegabytes) * host.budgetBasisPoints / 10000);
}

/** 📈️ The law of grant growth: a unit that needs `needMegabytes` under a grant of `grantMegabytes` is carried (`fits`), grown into when this host has the free room above its floor and interactive reserve (`grow`), waited for while the room is short (`wait`), queued for a larger machine when this host can never hold it (`beyond-host`) and refused only when no machine of the fleet can (`refused`). `fleet` lists the other machines (`undefined` when the fleet is unknown: the unit is then queued, never refused).
 * @see ../🧬️schema/🔣️.json (`Policy.growth`) */
export function decideGrantGrowthV1(input: Readonly<{ needMegabytes: number; grantMegabytes: number; local: GrowthHostV1; fleet: readonly GrowthHostV1[] | undefined }>): GrantGrowthVerdictV1 {
  const { needMegabytes, grantMegabytes, local } = input;
  if (needMegabytes <= grantMegabytes) return { kind: "fits" };
  const holdable = hostHoldableMegabytesV1(local), extra = needMegabytes - grantMegabytes;
  if (needMegabytes <= holdable) {
    const room = Math.max(0, (local.availableMegabytes ?? Infinity) - local.floorMegabytes - local.interactiveReserveMegabytes);
    return { kind: extra <= room ? "grow" : "wait", extraMegabytes: extra, roomMegabytes: Number.isFinite(room) ? room : holdable, holdableMegabytes: holdable };
  }
  const others = (input.fleet ?? []).filter((host) => host.origin !== local.origin).map((host) => ({ origin: host.origin, megabytes: hostHoldableMegabytesV1(host) })), largest = others.reduce((best, host) => host.megabytes > best.megabytes ? host : best, { origin: local.origin, megabytes: holdable });
  if (input.fleet === undefined || largest.megabytes >= needMegabytes) return { kind: "beyond-host", holdableMegabytes: holdable, largestMegabytes: largest.megabytes, largestOrigin: largest.origin };
  return { kind: "refused", largestMegabytes: largest.megabytes, largestOrigin: largest.origin };
}
