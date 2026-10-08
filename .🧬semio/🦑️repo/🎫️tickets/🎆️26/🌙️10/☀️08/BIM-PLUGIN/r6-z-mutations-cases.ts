/**
 * 🧪️ Case operations on existing mutation leaves: read the committed fixtures of a leaf, add a case (fixture quintet, generated test
 * file and root mount) or remove one. Applied cases get placeholder `after`/`diff` documents that `BIM_BLESS=1 cargo test` fills.
 */
import { existsSync, readdirSync, readFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { emitCase, relRoot } from "./r3-f1-gen-leaf.ts";
import { em, fixtures, mutations } from "./r3-f1-paths.ts";
import { addMounts, removeCaseMount } from "./r6-z-mutations-root.ts";

const POOL = [0x1f9ed, 0x1f9ee, 0x1f9ef, 0x1f9f0, 0x1f9f2, 0x1f9f3, 0x1f9f4, 0x1f9f6, 0x1f9f7, 0x1f9f8, 0x1f9fa, 0x1f9fb, 0x1f9fc, 0x1fa90, 0x1fa92, 0x1fa93, 0x1fa94, 0x1fa95, 0x1fa96, 0x1fa97, 0x1fa98, 0x1fa99, 0x1fa9a, 0x1fa9b, 0x1fa9c, 0x1fa9d, 0x1fa9e, 0x1fa9f, 0x1faa0, 0x1faa1, 0x1faa2, 0x1faa3];

const mutationsFixtures = () => join(fixtures, em(0x1f9ec) + "mutations");
export const leafDirName = (kind: string) => readdirSync(mutations).find((name) => name.endsWith(kind) && !name.slice(0, -kind.length).includes("-") && name.length > kind.length)!;
export const snake = (kind: string) => kind.replaceAll("-", "_");
export const variantOf = (kind: string) => kind.split("-").map((word) => word[0].toUpperCase() + word.slice(1)).join("");
const fixtureLeaf = (kind: string) => join(mutationsFixtures(), leafDirName(kind));
const testsDir = (kind: string) => join(mutations, leafDirName(kind), em(0x1f9ea) + "tests");

export const caseDirs = (kind: string): string[] => (existsSync(fixtureLeaf(kind)) ? readdirSync(fixtureLeaf(kind)) : []);

export type Part = "before" | "after" | "mutation" | "diff" | "outcome";
export function read(kind: string, caseDir: string, part: Part): string {
  const base = join(fixtureLeaf(kind), caseDir);
  const path =
    part === "before" || part === "after"
      ? join(base, em(0x1f4f8) + "snapshot", part === "before" ? em(0x2b05) + "before" : em(0x27a1) + "after")
      : join(base, part === "mutation" ? em(0x1f9a0) + "mutation" : part === "diff" ? em(0x1f53a) + "diff" : em(0x1f3af) + "outcome");
  return readFileSync(join(path, em(0x1f523) + ".json"), "utf8").replaceAll("\r\n", "\n");
}

export const statusOf = (kind: string, caseDir: string): string => JSON.parse(read(kind, caseDir, "outcome")).status;
export const appliedCases = (kind: string): string[] => caseDirs(kind).filter((name) => statusOf(kind, name) === "applied");

export function freshEmoji(kind: string): number {
  const used = new Set<string>();
  for (const dir of [fixtureLeaf(kind), testsDir(kind)]) if (existsSync(dir)) for (const name of readdirSync(dir)) used.add(String.fromCodePoint(name.codePointAt(0)!));
  const pick = POOL.find((point) => !used.has(String.fromCodePoint(point)));
  if (pick === undefined) throw new Error(`no free case emoji for ${kind}`);
  return pick;
}

export type Outcome = { status: "applied" } | { status: "rejected"; code: string; path: string[] };

export function addCase(kind: string, name: string, documents: { beforeText: string; mutationText: string; outcome: Outcome }, emoji = freshEmoji(kind)) {
  if (caseDirs(kind).some((dir) => dir.endsWith(name))) throw new Error(`${kind} already has a case ${name}`);
  const mounts = emitCase(kind, leafDirName(kind), variantOf(kind), { name, emoji, before: null, mutation: {}, outcome: documents.outcome, beforeText: documents.beforeText, mutationText: documents.mutationText });
  addMounts(snake(kind), mounts);
}

export function removeCase(kind: string, caseDir: string) {
  removeCaseMount(snake(kind), caseDir);
  rmSync(join(fixtureLeaf(kind), caseDir), { recursive: true, force: true });
  rmSync(join(testsDir(kind), caseDir), { recursive: true, force: true });
}

export { relRoot };
