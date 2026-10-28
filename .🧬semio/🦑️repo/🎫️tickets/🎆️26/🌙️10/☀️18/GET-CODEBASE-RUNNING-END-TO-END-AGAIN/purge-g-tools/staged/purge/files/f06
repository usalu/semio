import { generatedOutputHasLiveLease } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { existsSync, lstatSync, readdirSync, realpathSync, type Dirent } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";
import {HUB_DATA_DIR_NAME,MAP_CACHE_DIR_NAME,SPACE_DATA_DIR_NAME} from "../../🗂️workspaces/🧬️metadata/🟦️.ts";
import {runProbe} from "../../🏃️process/🔍️probe/🟦️.ts";
import {
  CLEAN_CACHE_DIR_NAME,
  CLEAN_CANONICAL_REPO_DIR,
  CLEAN_PROTECTION_VIEW,
  type CleanRemoval,
  cleanIntersectsProtected,
  cleanIsBuildArtifactDirName,
  cleanIsCanonicalRepoDir,
  cleanIsCanonicalTicketsDir,
  cleanIsCargoTargetDir,
  CLEAN_RUST_INCREMENTAL_MAX_BYTES,
  cleanSelectHugeRustIncremental,
  cleanRustIncrementalSearchRoots,
  type CleanRustIncrementalCrate,
  cleanIsMisplacedRepoDir,
  cleanIsMisplacedTicketsDir,
  cleanIsNativeImagePath,
  cleanIsProtected,
  cleanIsRootTransientName,
  cleanIsSemioRootName,
  cleanIsTicketGeneratedOutputDir,
  cleanOpenTicketGeneratedOutputProtection,
  cleanIsWindowsIllegalName,
  cleanTicketFolderForPath,
  cleanTicketManifestIsClosed,
} from "../🛡️protection/🟦️.ts";

export const CLEAN_BUILD_ARTIFACT_MAX_BYTES = 10 * 1024 * 1024 * 1024;
export const CLEAN_OVERSIZED_IGNORED_FILE_MAX_BYTES = 100 * 1024 * 1024;
/** 🧹️ The one purge law of ticket generated output, identical in Go, Rust and TypeScript. @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/🧬️schema/🔣️.json (PurgePolicy) */
export const CLEAN_TICKET_GENERATED_DIRECTORY = "🗑️generated";
export const CLEAN_GENERATED_OVERSIZED_FILE_BYTES = 5 * 1024 * 1024;
export const CLEAN_GENERATED_OVERSIZED_FOLDER_BYTES = 10 * 1024 * 1024;
export const CLEAN_GENERATED_LIVE_WRITER_MS = 600_000;
export function cleanPathBytes(abs: string): number {
  try {
    const st = lstatSync(abs);
    if (st.isSymbolicLink() || st.isFile()) return st.size;
    if (!st.isDirectory()) return 0;
    let total = 0;
    for (const name of readdirSync(abs)) total += cleanPathBytes(join(abs, name));
    return total;
  } catch {
    return 0;
  }
}

export function cleanWalkDirs(root: string, visit: (abs: string, name: string) => "enter" | "skip" | "stop"): void {
  const stack = [root];
  while (stack.length > 0) {
    const dir = stack.pop()!;
    let entries: string[];
    try {
      entries = readdirSync(dir);
    } catch {
      continue;
    }
    for (const name of entries) {
      const abs = join(dir, name);
      let st;
      try {
        st = lstatSync(abs);
      } catch {
        continue;
      }
      if (!st.isDirectory() || st.isSymbolicLink()) continue;
      const action = visit(abs, name);
      if (action === "stop") return;
      if (action === "enter") stack.push(abs);
    }
  }
}

export function cleanCollectMisplaced(root: string, protectedPrefixes: readonly string[]): CleanRemoval[] {
  const out: CleanRemoval[] = [];
  const seen = new Set<string>();
  const push = (abs: string): void => {
    if (cleanIntersectsProtected(abs, protectedPrefixes) || seen.has(abs)) return;
    seen.add(abs);
    out.push({ kind: "misplaced", path: relative(root, abs) || ".", bytes: cleanPathBytes(abs) });
  };
  const rootMisplaced = join(root, CLEAN_CANONICAL_REPO_DIR);
  if (existsSync(rootMisplaced)) push(rootMisplaced);
  cleanWalkDirs(root, (abs, name) => {
    if (name === "node_modules" || name === ".git" || name === CLEAN_CACHE_DIR_NAME) return "skip";
    if (name === MAP_CACHE_DIR_NAME || name === HUB_DATA_DIR_NAME || name === SPACE_DATA_DIR_NAME) return "skip";
    if (cleanIsBuildArtifactDirName(name)) return "skip";
    if (cleanIsSemioRootName(name)) {
      let children: string[];
      try {
        children = readdirSync(abs);
      } catch {
        return "skip";
      }
      for (const child of children) {
        const childAbs = join(abs, child);
        try {
          if (!lstatSync(childAbs).isDirectory()) continue;
        } catch {
          continue;
        }
        if (cleanIsMisplacedRepoDir(child)) push(childAbs);
        if (cleanIsCanonicalRepoDir(child)) {
          let ticketsKids: string[];
          try {
            ticketsKids = readdirSync(childAbs);
          } catch {
            continue;
          }
          for (const ticketsName of ticketsKids) {
            if (!cleanIsMisplacedTicketsDir(ticketsName)) continue;
            const ticketsAbs = join(childAbs, ticketsName);
            try {
              if (!lstatSync(ticketsAbs).isDirectory()) continue;
            } catch {
              continue;
            }
            push(ticketsAbs);
          }
        }
      }
      return "skip";
    }
    return "enter";
  });
  return out;
}

export function cleanCollectRootTransient(root: string, protectedPrefixes: readonly string[]): CleanRemoval[] {
  const out: CleanRemoval[] = [];
  let names: string[];
  try {
    names = readdirSync(root);
  } catch {
    return out;
  }
  for (const name of names) {
    if (!cleanIsRootTransientName(name)) continue;
    const abs = join(root, name);
    if (cleanIntersectsProtected(abs, protectedPrefixes)) continue;
    out.push({ kind: "root-transient", path: name, bytes: cleanPathBytes(abs) });
  }
  return out;
}

export function cleanDiscoverTicketRoots(root: string): string[] {
  const roots: string[] = [];
  cleanWalkDirs(root, (abs, name) => {
    if (name === "node_modules" || name === ".git" || name === CLEAN_CACHE_DIR_NAME) return "skip";
    if (name === MAP_CACHE_DIR_NAME || name === HUB_DATA_DIR_NAME || name === SPACE_DATA_DIR_NAME) return "skip";
    if (cleanIsBuildArtifactDirName(name)) return "skip";
    if (cleanIsCanonicalTicketsDir(name)) {
      roots.push(abs);
      return "skip";
    }
    return "enter";
  });
  return roots;
}

/** 📂Individual ticket slug folders (`…/🎆️YY/🌙️MM/☀️DD/TICKETSLUG`) — never year/month/day parents. */
export function cleanDiscoverTicketFolders(ticketsRoot: string): string[] {
  const folders: string[] = [];
  cleanWalkDirs(ticketsRoot, (abs, _name) => {
    const depth = relative(ticketsRoot, abs).split(sep).filter(Boolean).length;
    if (depth === 4) {
      folders.push(abs);
      return "skip";
    }
    if (depth > 4) return "skip";
    return "enter";
  });
  return folders;
}

export function cleanTicketGeneratedOutputRemovals(root: string, ticketFolder: string, protectedPrefixes: readonly string[], now = Date.now()): CleanRemoval[] {
  const out: CleanRemoval[] = [];
  cleanWalkDirs(ticketFolder, (abs, name) => {
    if (!cleanIsTicketGeneratedOutputDir(name)) return "enter";
    const applicablePrefixes = protectedPrefixes.filter((prefix) => resolve(prefix) !== resolve(ticketFolder));
    if (cleanIntersectsProtected(abs, applicablePrefixes) || cleanIsProtected(abs, applicablePrefixes)) return "skip";
    if (!cleanTicketManifestIsClosed(ticketFolder, CLEAN_PROTECTION_VIEW)) return "skip";
    if (generatedOutputHasLiveLease(abs)) return "skip";
    const plan = cleanPlanGeneratedDirectory(root, abs, "generated", now);
    if (plan.skip === undefined) for (const removal of plan.removals) out.push({ kind: "ticket-generated", path: relative(root, removal.path), bytes: removal.bytes });
    return "skip";
  });
  return out;
}

/** 🎚️ Which part of an eligible generated folder is removed: files above 5 MiB and folders above 10 MiB, or everything below it. */
export type CleanGeneratedSelection = "oversized" | "generated";

/** 🗑️ One planned removal below a generated folder. */
export type CleanGeneratedRemoval = { path: string; kind: "file" | "directory"; bytes: number };

/** ⏭️ Why a whole generated folder or ticket is left alone. */
export type CleanGeneratedSkip = "not-closed" | "live-writer" | "symlink-root" | "not-directory" | "tracked-unknown";

/** 🔎️ Answers which files below a repository-relative folder the git index tracks; undefined means unknown. */
export type CleanTrackedFiles = (root: string, folder: string) => ReadonlySet<string> | undefined;

/** 🌿️ Read-only `git ls-files` that cannot climb above the root and ignores any GIT_ redirection of the caller. */
export const cleanGitTrackedFiles: CleanTrackedFiles = (root, folder) => {
  const env: NodeJS.ProcessEnv = Object.fromEntries(Object.entries(process.env).filter(([name]) => !name.toUpperCase().startsWith("GIT_")));
  env["GIT_OPTIONAL_LOCKS"] = "0";
  env["GIT_CEILING_DIRECTORIES"] = dirname(resolve(root));
  try {
    const probe = runProbe("git", ["-c", "core.quotepath=off", "ls-files", "-z", "--cached", "--", folder.split(sep).join("/")], { cwd: root, env, budgetMs: 120_000 });
    return probe.status === 0 ? new Set(probe.stdout.split("\0").filter(Boolean)) : undefined;
  } catch {
    return undefined;
  }
};

const cleanByteOrder = (left: string, right: string): number => Buffer.compare(Buffer.from(left), Buffer.from(right));

/** 🧮️ Plans the removal below one generated folder: never a link, never a tracked file, never a folder holding one, and nothing at all while a file is younger than the live-writer window. */
export function cleanPlanGeneratedDirectory(root: string, generated: string, selection: CleanGeneratedSelection, now: number, tracked: CleanTrackedFiles = cleanGitTrackedFiles): { removals: CleanGeneratedRemoval[]; skip?: CleanGeneratedSkip } {
  let state;
  try {
    state = lstatSync(generated);
  } catch (error) {
    if ((error as NodeJS.ErrnoException).code === "ENOENT") return { removals: [] };
    throw error;
  }
  if (state.isSymbolicLink()) return { removals: [], skip: "symlink-root" };
  if (!state.isDirectory()) return { removals: [], skip: "not-directory" };
  const known = tracked(root, relative(root, generated));
  if (known === undefined) return { removals: [], skip: "tracked-unknown" };
  const directories = [generated];
  const files: { path: string; size: number; tracked: boolean }[] = [];
  let newest = Number.NEGATIVE_INFINITY;
  for (let index = 0; index < directories.length; index += 1) {
    for (const entry of readdirSync(directories[index]!, { withFileTypes: true })) {
      const path = join(directories[index]!, entry.name);
      let node;
      try {
        node = lstatSync(path);
      } catch (error) {
        if ((error as NodeJS.ErrnoException).code === "ENOENT") continue;
        throw error;
      }
      if (node.isSymbolicLink()) continue;
      if (node.isDirectory()) directories.push(path);
      else if (node.isFile()) {
        files.push({ path, size: node.size, tracked: known.has(relative(root, path).split(sep).join("/")) });
        newest = Math.max(newest, node.mtimeMs);
      }
    }
  }
  if (files.length > 0 && now - newest < CLEAN_GENERATED_LIVE_WRITER_MS) return { removals: [], skip: "live-writer" };
  const everything = selection === "generated";
  const bytes = new Map<string, number>();
  const holdsTracked = new Set<string>();
  for (const file of files) {
    for (let folder = dirname(file.path); ; folder = dirname(folder)) {
      bytes.set(folder, (bytes.get(folder) ?? 0) + file.size);
      if (file.tracked) holdsTracked.add(folder);
      if (folder === generated) break;
    }
  }
  const removals: CleanGeneratedRemoval[] = [];
  const removed: string[] = [];
  const covered = (path: string): boolean => removed.some((folder) => path === folder || path.startsWith(folder + sep));
  for (const folder of [...directories].sort((left, right) => left.split(sep).length - right.split(sep).length || cleanByteOrder(left, right))) {
    if ((folder === generated && !everything) || covered(folder) || holdsTracked.has(folder)) continue;
    if (everything || (bytes.get(folder) ?? 0) > CLEAN_GENERATED_OVERSIZED_FOLDER_BYTES) {
      removed.push(folder);
      removals.push({ path: folder, kind: "directory", bytes: bytes.get(folder) ?? 0 });
    }
  }
  for (const file of files) {
    if (covered(file.path) || file.tracked) continue;
    if (everything || file.size > CLEAN_GENERATED_OVERSIZED_FILE_BYTES) removals.push({ path: file.path, kind: "file", bytes: file.size });
  }
  return { removals };
}

/** 🧹️ A purge plan: ticket-relative removal paths and the tickets left whole, both in byte order. */
export type CleanGeneratedPlan = { removals: CleanGeneratedRemoval[]; skips: { ticket: string; reason: CleanGeneratedSkip }[] };

/** 🧹️ Plans the generated-output purge of the given ticket folders, or of every closed ticket below `ticketsDir`. Pure: it reads the filesystem and git, and removes nothing. */
export function cleanPlanTicketGeneratedPurge(request: { root: string; ticketsDir: string; ticketDirs?: readonly string[]; selection: CleanGeneratedSelection; now: number; tracked?: CleanTrackedFiles }): CleanGeneratedPlan {
  const root = realpathSync(request.root);
  const ticketsDir = realpathSync(request.ticketsDir);
  const folders = request.ticketDirs ?? cleanDiscoverTicketFolders(ticketsDir).filter((folder) => !folder.split(sep).pop()!.startsWith("."));
  const plan: CleanGeneratedPlan = { removals: [], skips: [] };
  const identity = (path: string): string => relative(ticketsDir, path).split(sep).join("/");
  for (const input of folders) {
    const folder = join(realpathSync(dirname(resolve(input))), resolve(input).split(sep).pop()!);
    const ticket = identity(folder);
    if (!cleanTicketManifestIsClosed(folder, CLEAN_PROTECTION_VIEW)) {
      plan.skips.push({ ticket, reason: "not-closed" });
      continue;
    }
    const outcome = cleanPlanGeneratedDirectory(root, join(folder, CLEAN_TICKET_GENERATED_DIRECTORY), request.selection, request.now, request.tracked);
    if (outcome.skip !== undefined) plan.skips.push({ ticket, reason: outcome.skip });
    for (const removal of outcome.removals) plan.removals.push({ ...removal, path: identity(removal.path) });
  }
  plan.removals.sort((left, right) => cleanByteOrder(left.path, right.path));
  plan.skips.sort((left, right) => cleanByteOrder(left.ticket, right.ticket));
  return plan;
}

export function cleanGitignoredMapForTicketRoots(root: string, ticketRoots: readonly string[]): Map<string, string[]> {
  const map = new Map<string, string[]>();
  for (const ticketRoot of ticketRoots) {
    const rel = relative(root, ticketRoot);
    const probe = runProbe("git", ["ls-files", "--others", "-i", "--exclude-standard", "--directory", "--", rel], {
      cwd: root,
      budgetMs: 120_000,
    });
    if ((probe.status ?? 1) !== 0) continue;
    for (const line of probe.stdout
      .split("\n")
      .map((l) => l.trim())
      .filter(Boolean)) {
      const abs = join(root, line.replace(/\/$/, ""));
      const folder = cleanTicketFolderForPath(root, abs);
      if (folder) {
        let list = map.get(folder);
        if (!list) {
          list = [];
          map.set(folder, list);
        }
        list.push(abs);
      }
    }
  }
  return map;
}

export function cleanOversizedIgnoredRemovals(root: string, protectedPrefixes: readonly string[]): CleanRemoval[] {
  const probe = runProbe("git", ["ls-files", "--others", "-i", "--exclude-standard", "--directory", "-z"], { cwd: root, budgetMs: 900_000 });
  if ((probe.status ?? 1) !== 0) return [];
  const out: CleanRemoval[] = [];
  const seen = new Set<string>();
  for (const raw of probe.stdout.split("\0")) {
    const rel = raw.trim().replace(/\/$/, "");
    if (!rel || seen.has(rel)) continue;
    if ((rel.startsWith("node_modules/") || rel.includes("/node_modules/")) && !rel.startsWith("node_modules/.cache/")) continue;
    const abs = join(root, rel);
    const ticket = cleanTicketFolderForPath(root, abs);
    if (cleanIntersectsProtected(abs, protectedPrefixes) && !ticket) continue;
    if (cleanIsNativeImagePath(abs) || cleanOpenTicketGeneratedOutputProtection(root, abs).length !== 0) continue;
    let st;
    try {
      st = lstatSync(abs);
    } catch {
      continue;
    }
    if (st.isDirectory() || st.size <= CLEAN_OVERSIZED_IGNORED_FILE_MAX_BYTES) continue;
    seen.add(rel);
    out.push({ kind: ticket ? "ticket-generated" : "gitignore", path: rel, bytes: st.size });
  }
  return out;
}

export function cleanCollectWindowsIllegal(root: string, protectedPrefixes: readonly string[]): CleanRemoval[] {
  const out: CleanRemoval[] = [];
  const seen = new Set<string>();
  const stack = [root];
  while (stack.length > 0) {
    const dir = stack.pop()!;
    let entries: string[];
    try {
      entries = readdirSync(dir);
    } catch {
      continue;
    }
    for (const name of entries) {
      if (name === ".git" || name === "node_modules") continue;
      if (name === CLEAN_CACHE_DIR_NAME || name === MAP_CACHE_DIR_NAME || name === HUB_DATA_DIR_NAME || name === SPACE_DATA_DIR_NAME) continue;
      if (cleanIsBuildArtifactDirName(name)) continue;
      const abs = join(dir, name);
      if (cleanIntersectsProtected(abs, protectedPrefixes)) continue;
      if (cleanIsWindowsIllegalName(name)) {
        if (!seen.has(abs)) {
          seen.add(abs);
          out.push({ kind: "windows-illegal", path: relative(root, abs), bytes: cleanPathBytes(abs) });
        }
        continue;
      }
      let st;
      try {
        st = lstatSync(abs);
      } catch {
        continue;
      }
      if (st.isDirectory() && !st.isSymbolicLink()) {
        stack.push(abs);
      }
    }
  }
  const probe = runProbe("git", ["ls-files", "-z"], { cwd: root, budgetMs: 60_000 });
  if ((probe.status ?? 1) === 0) {
    for (const file of probe.stdout.split("\0").filter(Boolean)) {
      const parts = file.split("/");
      if (parts.some((part) => cleanIsWindowsIllegalName(part))) {
        const abs = join(root, file);
        if (!cleanIntersectsProtected(abs, protectedPrefixes) && !seen.has(abs)) {
          seen.add(abs);
          out.push({ kind: "windows-illegal", path: file, bytes: cleanPathBytes(abs) });
        }
      }
    }
  }
  return out;
}


const CLEAN_RUST_INCREMENTAL_HEAVY_DIRS = new Set(["build", "deps", ".fingerprint"]);

/** 🦀️ Reclaims rustc `incremental` directories larger than `maxBytes` under cargo output roots. In-progress `-working` sessions stay; dependency units stay. */
export function cleanCollectRustIncrementalRemovals(root: string, maxBytes = CLEAN_RUST_INCREMENTAL_MAX_BYTES): CleanRemoval[] {
  const out: CleanRemoval[] = [];
  const seen = new Set<string>();
  for (const searchRoot of cleanRustIncrementalSearchRoots(root)) {
    const stack: { dir: string; depth: number }[] = [{ dir: searchRoot, depth: 0 }];
    while (stack.length > 0) {
      const current = stack.pop()!;
      let names: string[];
      try {
        names = readdirSync(current.dir);
      } catch {
        continue;
      }
      for (const name of names) {
        if (name === "node_modules" || name === ".git") continue;
        const abs = join(current.dir, name);
        let state;
        try {
          state = lstatSync(abs);
        } catch {
          continue;
        }
        if (!state.isDirectory() || state.isSymbolicLink()) continue;
        if (name === "incremental") {
          const removal = cleanRustIncrementalDirectoryRemoval(root, abs, maxBytes);
          if (removal) {
            for (const row of removal.rows) {
              const key = resolve(root, row.path);
              if (seen.has(key)) continue;
              seen.add(key);
              out.push(row);
            }
          }
          continue;
        }
        if (current.depth >= 8) continue;
        if (CLEAN_RUST_INCREMENTAL_HEAVY_DIRS.has(name) && dirname(abs) !== searchRoot) continue;
        stack.push({ dir: abs, depth: current.depth + 1 });
      }
    }
  }
  return out;
}

function cleanRustIncrementalDirectoryRemoval(root: string, incrementalAbs: string, maxBytes: number): { abs: string; rows: CleanRemoval[] } | undefined {
  let entries: Dirent[];
  try {
    entries = readdirSync(incrementalAbs, { withFileTypes: true });
  } catch {
    return undefined;
  }
  const crates: CleanRustIncrementalCrate[] = [];
  for (const entry of entries) {
    if (!entry.isDirectory() || entry.isSymbolicLink()) continue;
    const crateAbs = join(incrementalAbs, entry.name);
    let children: Dirent[];
    try {
      children = readdirSync(crateAbs, { withFileTypes: true });
    } catch {
      continue;
    }
    let session = false;
    let working = false;
    for (const child of children) {
      if (!child.isDirectory() || child.isSymbolicLink()) continue;
      if (child.name.endsWith("-working")) working = true;
      if (child.name.startsWith("s-") && !child.name.endsWith(".lock")) session = true;
    }
    if (!session) continue;
    crates.push({ name: entry.name, bytes: 0, working });
  }
  if (crates.length === 0) return undefined;
  const path = relative(root, incrementalAbs).split(sep).join("/");
  const measured = crates.some((crate) => crate.working)
    ? crates.map((crate) => ({ ...crate, bytes: cleanRustIncrementalBytes(join(incrementalAbs, crate.name)) }))
    : [{ name: "profile", bytes: cleanRustIncrementalBytes(incrementalAbs), working: false }];
  const selected = cleanSelectHugeRustIncremental({ path, crates: measured }, maxBytes);
  const rows = selected.map((item): CleanRemoval => {
    const abs = join(root, ...item.split("/"));
    const bytes = item === path ? measured.reduce((sum, crate) => sum + crate.bytes, 0) : measured.find((crate) => crate.name === item.split("/").at(-1))?.bytes ?? 0;
    return { kind: "cargo-incremental", path: relative(root, abs), bytes };
  });
  return { abs: incrementalAbs, rows };
}

/** 📏 Allocated bytes of one incremental directory. `du` keeps a multi-gigabyte profile cache from being walked file by file. */
function cleanRustIncrementalBytes(abs: string): number {
  if (process.platform !== "win32") {
    try {
      const probe = runProbe("du", ["-sk", abs], { budgetMs: 600_000 });
      if ((probe.status ?? 1) === 0) {
        const kb = Number(probe.stdout.trim().split(/\s+/u)[0]);
        if (Number.isSafeInteger(kb) && kb >= 0) return kb * 1024;
      }
    } catch {}
  }
  return cleanPathBytes(abs);
}

export function cleanBuildArtifactRemovals(root: string, protectedPrefixes: readonly string[]): CleanRemoval[] {
  const out: CleanRemoval[] = [];
  cleanWalkDirs(root, (abs, name) => {
    if (name === "node_modules" || name === ".git" || name === CLEAN_CACHE_DIR_NAME) return "skip";
    if (name === MAP_CACHE_DIR_NAME || name === HUB_DATA_DIR_NAME || name === SPACE_DATA_DIR_NAME) return "skip";
    if (cleanIsCanonicalTicketsDir(name)) return "skip";
    if (cleanIsProtected(abs, protectedPrefixes)) return "skip";
    if (cleanIsBuildArtifactDirName(name)) {
      if (cleanIntersectsProtected(abs, protectedPrefixes)) return "skip";
      const bytes = cleanPathBytes(abs);
      if (cleanIsCargoTargetDir(abs, name) || bytes > CLEAN_BUILD_ARTIFACT_MAX_BYTES) out.push({ kind: "build-artifact", path: relative(root, abs), bytes });
      return "skip";
    }
    return "enter";
  });
  return out;
}

/** ➖️ Ticket size hits keep the deepest path so `target/` wins over year/month parents. */
export function cleanDedupePreferDeepest(removals: readonly CleanRemoval[]): CleanRemoval[] {
  const sorted = [...removals].sort((a, b) => b.path.length - a.path.length || a.path.localeCompare(b.path));
  const kept: CleanRemoval[] = [];
  for (const row of sorted) {
    if (kept.some((k) => k.path === row.path || k.path.startsWith(row.path + "/"))) continue;
    kept.push(row);
  }
  return kept;
}

/** 👪️ Prefer shallow removals across kinds so a parent delete absorbs nested hits. */
