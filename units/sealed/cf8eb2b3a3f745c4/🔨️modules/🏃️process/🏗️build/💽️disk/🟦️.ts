import { diskFullOfV1, isDiskFullV1, type DiskFullV1 } from "../../🆓️free-space/💽️disk-full/🟦️.ts";
import { PROCESS_TEXTS_V1, refuseProcessV1 } from "../../📖️texts/🟦️.ts";
import { handleTextV1 } from "../🗣️language/📖️catalog/🟦️.ts";
import type { RepoErrorV1 } from "../🗣️language/🚨️error/🟦️.ts";

/** 🧹️ The command that frees space on a full disk; it needs no free byte of its own. */
export const DISK_RECLAIM_COMMAND_V1 = "repo cache cas gc";

const UNITS = ["B", "KiB", "MiB", "GiB", "TiB"] as const;

/** 🔢️ Writes a byte count with the largest binary unit that keeps it at one or more (`0 B`, `512 KiB`, `1.25 GiB`); the one form of every disk refusal. */
export function formatDiskBytesV1(bytes: number): string {
  let value = Math.max(0, bytes), unit = 0;
  while (value >= 1024 && unit < UNITS.length - 1) { value /= 1024; unit++; }
  return `${unit === 0 ? Math.round(value) : Math.round(value * 100) / 100} ${UNITS[unit]}`;
}

/** 💽️ The refusal of a failure that says the disk is full: it names the partition, the free bytes and the command that frees space, in both languages. @see ../../🆓️free-space/💽️disk-full/🟦️.ts */
export const refuseDiskFullFactsV1 = (facts: DiskFullV1, cause?: unknown): RepoErrorV1 => facts.freeBytes === undefined
  ? refuseProcessV1("failed", "disk.full-unmeasured", { partition: facts.partition, command: DISK_RECLAIM_COMMAND_V1 }, { cause })
  : refuseProcessV1("failed", "disk.full", { partition: facts.partition, free: formatDiskBytesV1(facts.freeBytes), command: DISK_RECLAIM_COMMAND_V1 }, { cause });

/** 💽️ Turns a full-disk failure (`ENOSPC`, `EDQUOT`, `database or disk is full`) into the typed refusal; any other error answers `undefined`. */
export function refuseDiskFullV1(error: unknown, options: Parameters<typeof diskFullOfV1>[1] = {}): RepoErrorV1 | undefined {
  const facts = isDiskFullV1(error) ? diskFullOfV1(error, options) : undefined;
  return facts === undefined ? undefined : refuseDiskFullFactsV1(facts, error);
}

/** 🚧️ What a partition holds against the used share a write or a build may start at. */
export type DiskLowSpaceV1 = Readonly<{ partition: string; freeBytes: number; totalBytes: number; limitFraction: number }>;

/** 🚧️ The refusal of a build or cache write that would start on a partition above its used share: it names the partition, the free bytes, the used and the allowed percent and the command that frees space. */
export const refuseLowSpaceV1 = (facts: DiskLowSpaceV1): RepoErrorV1 => refuseProcessV1("refused", "disk.low-space", {
  partition: facts.partition,
  free: formatDiskBytesV1(facts.freeBytes),
  usedPercent: Math.round((1 - facts.freeBytes / Math.max(1, facts.totalBytes)) * 1000) / 10,
  limitPercent: Math.round(facts.limitFraction * 1000) / 10,
  command: DISK_RECLAIM_COMMAND_V1,
});

/** 📣️ The progress line of a command that waits for the cache collection because its partition is above the used share it may start at: it names the partition, how full it is and the limit, in the language asked for. */
export const diskCollectingLineV1 = (facts: Readonly<{ partition: string; usedFraction: number; limitFraction: number }>, language: string): string => handleTextV1(PROCESS_TEXTS_V1, "disk.collecting", language, {
  partition: facts.partition,
  usedPercent: Math.round(facts.usedFraction * 1000) / 10,
  limitPercent: Math.round(facts.limitFraction * 1000) / 10,
});
