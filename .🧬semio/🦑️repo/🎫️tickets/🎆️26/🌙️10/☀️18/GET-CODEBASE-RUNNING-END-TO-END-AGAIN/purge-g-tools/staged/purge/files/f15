import { existsSync, lstatSync, readFileSync, readdirSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { getRepoMetaDir, getSemioRoot, HUB_DATA_DIR_NAME, MAP_CACHE_DIR_NAME, REPO_META_DIR_NAME, SPACE_DATA_DIR_NAME } from "../../🗂️workspaces/🧬️metadata/🟦️.ts";
import { repoCacheDirectory } from "../../⚡️caching/🟦️.ts";
import { cargoDirectories } from "../../⚡️caching/🦀️cargo/🗂️directories/🟦️.ts";
import { TICKET_GENERATED_OUTPUT_DIRECTORY } from "../../🗂️workspaces/🧬️metadata/🟦️.ts";
import {repositoryCacheBudgetV1} from "../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";

export const CLEAN_CANONICAL_REPO_DIR = REPO_META_DIR_NAME;
export const CLEAN_CANONICAL_TICKETS_DIR = "🎫️tickets";
export const CLEAN_BUILD_DIR_NAMES = new Set(["target", "dist", "build", "out"]);
export const CLEAN_CACHE_DIR_NAME = "⚡️cache";
export const CLEAN_TICKET_GENERATED_OUTPUT_DIRS = new Set(["🗑️generated", TICKET_GENERATED_OUTPUT_DIRECTORY, "🧾️runs", "🧪️runs", "🧾️taxonomy-transaction"]);
export const CLEAN_TICKET_GENERATED_PROBE_PREFIXES = ["🧪️purity-", "🧪️cli-", "🧪️inventory-"];

export type CleanRemovalKind = "misplaced" | "gitignore" | "build-artifact" | "ticket-generated" | "windows-illegal" | "root-transient" | "marker-only-folder" | "empty-folder" | "cargo-incremental";

export type CleanRemoval = {
  kind: CleanRemovalKind;
  path: string;
  bytes: number;
};
export function cleanEndsWithAscii(name: string, ascii: string): boolean {
  return name.endsWith(ascii);
}

export function cleanIsCanonicalRepoDir(name: string): boolean {
  return name === CLEAN_CANONICAL_REPO_DIR;
}

export function cleanIsCanonicalTicketsDir(name: string): boolean {
  return name === CLEAN_CANONICAL_TICKETS_DIR;
}

export function cleanIsMisplacedRepoDir(name: string): boolean {
  return cleanEndsWithAscii(name, "repo") && !cleanIsCanonicalRepoDir(name);
}

export function cleanIsMisplacedTicketsDir(name: string): boolean {
  return cleanEndsWithAscii(name, "tickets") && !cleanIsCanonicalTicketsDir(name);
}

export function cleanIsBuildArtifactDirName(name: string): boolean {
  if (CLEAN_BUILD_DIR_NAMES.has(name)) return true;
  if (cleanIsCargoTargetDirName(name)) return true;
  return false;
}

/** 🦀️ Classifies raw Cargo output names independently of emoji domain Source owners. */
export function cleanIsCargoTargetDirName(name: string): boolean {
  return name === "target" || name.startsWith("target-");
}

/** 🏷️Cargo stamps every target and build dir with `CACHEDIR.TAG`; only such dirs are provably Cargo output. https://bford.info/cachedir/ */
export function cleanIsCargoTargetDir(abs: string, name: string): boolean {
  return cleanIsCargoTargetDirName(name) && existsSync(join(abs, "CACHEDIR.TAG"));
}

/** 🧹 Rustc incremental profile caches larger than this are removed by `clean`. Smaller ones stay for the cache pruner. */
export const CLEAN_RUST_INCREMENTAL_MAX_BYTES = 1024 * 1024 * 1024;

/** 🧼️ One rustc incremental crate directory: `incremental/<crate>-<hash>/`. https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation-in-detail.html */
export interface CleanRustIncrementalCrate {
  readonly name: string;
  readonly bytes: number;
  readonly working: boolean;
}

/** 🗂️ One profile-level rustc `incremental` directory and the crate caches inside it. */
export interface CleanRustIncrementalDirectory {
  readonly path: string;
  readonly crates: readonly CleanRustIncrementalCrate[];
}

/** 📂️ Selects disposable rustc incremental directories once their profile cache exceeds `maxBytes`. A crate with a `-working` session stays. */
export function cleanSelectHugeRustIncremental(directory: CleanRustIncrementalDirectory, maxBytes: number): readonly string[] {
  if (!Number.isSafeInteger(maxBytes) || maxBytes < 0) throw new Error("Rust incremental limit must be a non-negative safe integer");
  const base = directory.path.replaceAll("\\", "/").replace(/\/+$/u, "");
  if (!base || base.split("/").some((part) => !part || part === "." || part === "..")) throw new Error("Rust incremental path must stay inside the workspace");
  const total = directory.crates.reduce((sum, crate) => {
    if (!crate.name || crate.name.includes("/") || crate.name.includes("\\") || crate.name === "." || crate.name === "..") throw new Error("Rust incremental crate name must be one path segment");
    if (!Number.isSafeInteger(crate.bytes) || crate.bytes < 0) throw new Error("Rust incremental size must be a non-negative safe integer");
    return sum + crate.bytes;
  }, 0);
  if (!Number.isSafeInteger(total)) throw new Error("Rust incremental size must be a non-negative safe integer");
  if (total <= maxBytes) return [];
  const idle = directory.crates.filter((crate) => !crate.working);
  if (idle.length === 0) return [];
  if (idle.length === directory.crates.length) return [base];
  return idle.map((crate) => `${base}/${crate.name}`).sort((left, right) => (left < right ? -1 : left > right ? 1 : 0));
}

/** 🔎️ Cargo output roots whose `incremental` directories clean may reclaim: the shared cargo cache, plus any configured build or target directory inside the workspace. */
export function cleanRustIncrementalSearchRoots(root: string): string[] {
  const workspace = resolve(root);
  const candidates = [repoCacheDirectory(workspace, "cargo"), cargoDirectories(workspace).build, cargoDirectories(workspace).target];
  const inside: string[] = [];
  for (const candidate of candidates) {
    const abs = resolve(candidate);
    const rel = relative(workspace, abs);
    if (rel === "" || rel.startsWith(".." + sep) || rel === ".." || isAbsolute(rel)) continue;
    try {
      const state = lstatSync(abs);
      if (!state.isDirectory() || state.isSymbolicLink()) continue;
    } catch {
      continue;
    }
    inside.push(abs);
  }
  const unique = [...new Set(inside)];
  return unique.filter((path) => !unique.some((other) => other !== path && (path === other || path.startsWith(other + sep))));
}

/** 🛡️ True when a removal is exactly one rustc `incremental` directory, or one crate directory inside it, under a cargo output root. */
export function cleanIsRustIncrementalRemovalPath(root: string, abs: string, searchRoots: readonly string[]): boolean {
  const workspace = resolve(root);
  const resolved = resolve(abs);
  const local = relative(workspace, resolved);
  if (local === "" || local === ".." || local.startsWith(".." + sep) || isAbsolute(local)) return false;
  const parts = local.split(sep);
  const at = parts.lastIndexOf("incremental");
  if (at < 0 || (parts.length !== at + 1 && parts.length !== at + 2)) return false;
  if (parts.some((part) => !part || part === "." || part === "..")) return false;
  return searchRoots.some((base) => {
    const rootAbs = resolve(base);
    return resolved === rootAbs || resolved.startsWith(rootAbs + sep);
  });
}

/** 🔒️ True when rustc still has a `-working` session under this incremental directory or crate directory. */
export function cleanRustIncrementalHasWorkingSession(abs: string): boolean {
  const own = abs.split(sep).pop() ?? "";
  const crates = own === "incremental" ? cleanChildDirectories(abs) : [abs];
  if (own === "incremental" && crates === undefined) return true;
  for (const crate of crates ?? []) {
    const children = cleanChildDirectories(crate);
    if (children === undefined) return true;
    if (children.some((path) => (path.split(sep).pop() ?? "").endsWith("-working"))) return true;
  }
  return false;
}

function cleanChildDirectories(abs: string): string[] | undefined {
  let names: string[];
  try {
    names = readdirSync(abs);
  } catch {
    return undefined;
  }
  const children: string[] = [];
  for (const name of names) {
    if (!name || name === "." || name === "..") return undefined;
    const child = join(abs, name);
    try {
      const state = lstatSync(child);
      if (state.isSymbolicLink() || !state.isDirectory()) continue;
    } catch {
      return undefined;
    }
    children.push(child);
  }
  return children;
}

export function cleanIsSemioRootName(name: string): boolean {
  return name === ".🧬semio" || (name.startsWith(".🧬") && name.endsWith("semio"));
}

export function cleanIsRootTransientName(name: string): boolean {
  return /^\.generation3d-.+-link$/u.test(name) || name === ".w-g3-ticket";
}

export const CLEAN_WINDOWS_RESERVED_DEVICE_NAMES = /^(con|prn|aux|nul|com[1-9¹²³]|lpt[1-9¹²³])(\..*)?$/iu;
export const CLEAN_WINDOWS_FORBIDDEN_CHARS = /[<>:"|?*\x00-\x1f]/;

/** 🪟️ Checks whether a path component is forbidden on Windows filesystems (NTFS/Win32). */
export function cleanIsWindowsIllegalName(name: string): boolean {
  if (!name) return true;
  if (name === "." || name === "..") return false;
  if (name.trim().length === 0 || name.endsWith(" ") || name.endsWith(".") || name.startsWith(" ")) return true;
  if (CLEAN_WINDOWS_FORBIDDEN_CHARS.test(name)) return true;
  if (CLEAN_WINDOWS_RESERVED_DEVICE_NAMES.test(name)) return true;
  return false;
}

/** 🏪️ Reserves authored enforcing stores for their original physical retirement owners. */
export function cleanManagedStoragePrefixes(root:string):string[]{const budget=repositoryCacheBudgetV1(root);return [...Object.values(budget.partitions).map(partition=>partition.directory),budget.controlDirectory];}

/** 🖼️ Native image backing names owned by the storage layer: only its canonical retirement may remove them. */
export const CLEAN_NATIVE_IMAGE_BACKING_NAMES: ReadonlySet<string> = new Set(["fixed.dmg", "fixed.dir"]);

/** 🧲️ True when a path is a native image backing file, its receipt or mount directory, or any directory holding that pair, wherever it lives. */
export function cleanIsNativeImagePath(abs: string): boolean {
  const resolved = resolve(abs);
  const holdsImage = (directory: string): boolean => existsSync(join(directory, "image.json")) && [...CLEAN_NATIVE_IMAGE_BACKING_NAMES].some((name) => existsSync(join(directory, name)));
  const name = resolved.split(sep).pop() ?? "";
  if (CLEAN_NATIVE_IMAGE_BACKING_NAMES.has(name)) return true;
  if (holdsImage(resolved)) return true;
  const parent = dirname(resolved);
  return parent !== resolved && holdsImage(parent);
}

/** 🎫️ A generated-output directory of an open ticket holds live evidence: protects the candidate when it is, lies inside, or contains one. Closed tickets stay removable. */
export function cleanOpenTicketGeneratedOutputProtection(root: string, candidate: string, view: CleanProtectionView = CLEAN_PROTECTION_VIEW): string[] {
  const target = resolve(root, candidate);
  const ticket = cleanTicketFolderForPath(root, target);
  if (ticket && !cleanTicketManifestIsClosed(ticket, view)) {
    const inside = relative(ticket, target).split(sep);
    if (inside.some((segment) => cleanIsTicketGeneratedOutputDir(segment))) return [target];
  }
  if (view.kind(target) !== "directory") return [];
  const stack = [target];
  while (stack.length > 0) {
    const directory = stack.pop()!;
    const names = view.children(directory);
    if (!names) return [directory];
    for (const name of names) {
      const child = join(directory, name);
      if (view.kind(child) !== "directory") continue;
      if (cleanIsTicketGeneratedOutputDir(name)) {
        const owner = cleanTicketFolderForPath(root, child);
        if (owner && !cleanTicketManifestIsClosed(owner, view)) return [child];
        continue;
      }
      stack.push(child);
    }
  }
  return [];
}

export function cleanProtectedPrefixes(root: string): string[] {
  const semio = getSemioRoot(root);
  return [join(semio, MAP_CACHE_DIR_NAME), join(semio, HUB_DATA_DIR_NAME), join(semio, SPACE_DATA_DIR_NAME), join(getRepoMetaDir(root), CLEAN_CACHE_DIR_NAME),...cleanManagedStoragePrefixes(root)].map((p) => resolve(p));
}

export function cleanIsProtected(abs: string, protectedPrefixes: readonly string[]): boolean {
  const resolved = resolve(abs);
  return protectedPrefixes.some((prefix) => resolved === prefix || resolved.startsWith(prefix + sep));
}

//#region 🛡️TicketProtection
export type CleanProtectionNodeKind = "directory" | "file" | "symlink" | "missing" | "unreadable";

/** 👁️ Read-only filesystem projection used by the deletion gate and synthetic safety laws. */
export interface CleanProtectionView {
  kind(path: string): CleanProtectionNodeKind;
  children(path: string): readonly string[] | undefined;
  read(path: string): string | undefined;
}

export const CLEAN_PROTECTION_VIEW: CleanProtectionView = {
  kind(path) {
    try {
      const state = lstatSync(path);
      return state.isSymbolicLink() ? "symlink" : state.isDirectory() ? "directory" : state.isFile() ? "file" : "unreadable";
    } catch (error) {
      return (error as NodeJS.ErrnoException).code === "ENOENT" ? "missing" : "unreadable";
    }
  },
  children(path) {
    try {
      return readdirSync(path);
    } catch {
      return undefined;
    }
  },
  read(path) {
    try {
      return new TextDecoder("utf-8", { fatal: true }).decode(readFileSync(path));
    } catch {
      return undefined;
    }
  },
};

export function cleanIntersectsProtected(abs: string, protectedPrefixes: readonly string[]): boolean {
  const candidate = resolve(abs);
  return protectedPrefixes.some((value) => {
    const prefix = resolve(value);
    return candidate === prefix || candidate.startsWith(prefix + sep) || prefix.startsWith(candidate + sep);
  });
}

export function cleanTicketManifestIsClosed(directory: string, view: CleanProtectionView): boolean {
  const path = join(directory, "🎫️ticket.json");
  try {
    if (view.kind(path) !== "file") return false;
    const text = view.read(path);
    if (text === undefined) return false;
    const manifest: unknown = JSON.parse(text);
    return manifest !== null && typeof manifest === "object" && !Array.isArray(manifest) && Object.hasOwn(manifest, "status") && (manifest as { status: unknown }).status === "closed";
  } catch {
    return false;
  }
}

export function cleanIsTicketFolderBoundary(root: string, directory: string): boolean {
  const segments = relative(root, directory).split(sep).filter(Boolean);
  return segments.some((name, index) => cleanEndsWithAscii(name, "tickets") && segments.length - index - 1 === 4);
}

export function cleanTicketFolderForPath(root: string, abs: string): string | undefined {
  let ancestor = dirname(abs);
  while (true) {
    const local = relative(root, ancestor);
    if (local === "" || local === ".." || local.startsWith(".." + sep) || isAbsolute(local)) return undefined;
    if (cleanIsTicketFolderBoundary(root, ancestor)) return resolve(ancestor);
    const parent = dirname(ancestor);
    if (parent === ancestor) return undefined;
    ancestor = parent;
  }
}

/** 🗑️ Rejects a removal intersecting any non-closed ticket, unsafe path, or unreadable subtree without following symlinks. */
export function cleanRemovalProtection(root: string, candidate: string, view: CleanProtectionView = CLEAN_PROTECTION_VIEW, nestedLinks: "protect" | "unlink" = "protect"): string[] {
  const workspace = resolve(root),
    target = resolve(workspace, candidate);
  const local = relative(workspace, target);
  if (local === "" || local === ".." || local.startsWith(".." + sep) || isAbsolute(local)) return [target];
  if(cleanIntersectsProtected(target,cleanManagedStoragePrefixes(workspace)))return [target];
  if (cleanIsNativeImagePath(target)) return [target];
  const protectedPaths = new Set<string>(),
    closedTickets = new Set<string>();
  const ancestors: { path: string; kind: CleanProtectionNodeKind }[] = [{ path: workspace, kind: "directory" }];
  const inspectDirectory = (directory: string): void => {
    const manifestKind = view.kind(join(directory, "🎫️ticket.json"));
    if (!cleanIsTicketFolderBoundary(workspace, directory) && manifestKind === "missing") return;
    if (cleanTicketManifestIsClosed(directory, view)) closedTickets.add(directory);
    else protectedPaths.add(directory);
  };
  let ancestor = workspace;
  try {
    if (view.kind(workspace) !== "directory") return [workspace];
    inspectDirectory(workspace);
    for (const segment of local.split(sep)) {
      ancestor = join(ancestor, segment);
      const kind = view.kind(ancestor);
      if (kind === "symlink" || kind === "missing" || kind === "unreadable" || (ancestor !== target && kind !== "directory")) return [ancestor];
      ancestors.push({ path: ancestor, kind });
      if (kind === "directory") inspectDirectory(ancestor);
    }
    if (protectedPaths.size > 0) return [...protectedPaths];
    const stack = view.kind(target) === "directory" ? [target] : [];
    while (stack.length > 0) {
      const directory = stack.pop()!;
      inspectDirectory(directory);
      if (protectedPaths.has(directory)) continue;
      const names = view.children(directory);
      if (!names) {
        protectedPaths.add(directory);
        continue;
      }
      for (const name of names) {
        if (!name || name === "." || name === ".." || name.includes("/") || name.includes("\\")) {
          protectedPaths.add(directory);
          continue;
        }
        const child = join(directory, name),
          kind = view.kind(child);
        if (kind === "directory") stack.push(child);
        else if (kind !== "file" && !(kind === "symlink" && nestedLinks === "unlink")) protectedPaths.add(child);
      }
    }
    for (const entry of ancestors) if (view.kind(entry.path) !== entry.kind) protectedPaths.add(entry.path);
    for (const directory of closedTickets) if (view.kind(directory) !== "directory" || !cleanTicketManifestIsClosed(directory, view)) protectedPaths.add(directory);
  } catch {
    protectedPaths.add(ancestor);
  }
  return [...protectedPaths];
}

/** 🔗️ True when a candidate lies inside a generated-output folder of a ticket, where a nested link is unlinked with its folder and never followed. */
export function cleanIsInsideTicketGeneratedOutput(root: string, abs: string): boolean {
  const ticket = cleanTicketFolderForPath(root, abs);
  return ticket !== undefined && relative(ticket, abs).split(sep).some((segment) => cleanIsTicketGeneratedOutputDir(segment));
}

/** 🧮️ Removes unsafe candidates before shallow deduplication so a protected parent cannot absorb eligible siblings. */
export function cleanProjectRemovals(root: string, removals: readonly CleanRemoval[], protectedPrefixes: readonly string[] = [], view: CleanProtectionView = CLEAN_PROTECTION_VIEW, onProtected?: (path: string) => void): CleanRemoval[] {
  return cleanDedupePreferShallowest(
    removals.filter((row) => {
      const absolute = resolve(root, row.path);
      if (row.kind === "cargo-incremental") {
        const allowed = cleanIsRustIncrementalRemovalPath(root, absolute, cleanRustIncrementalSearchRoots(root));
        if (!allowed) onProtected?.(absolute);
        return allowed;
      }
      if (cleanIsNativeImagePath(absolute)) {
        onProtected?.(absolute);
        return false;
      }
      const openGenerated = cleanOpenTicketGeneratedOutputProtection(root, absolute, view);
      if (openGenerated.length !== 0) {
        for (const path of openGenerated) onProtected?.(path);
        return false;
      }
      const protectedPaths = cleanIntersectsProtected(absolute, protectedPrefixes) ? [absolute] : cleanRemovalProtection(root, row.path, view, cleanIsInsideTicketGeneratedOutput(root, absolute) ? "unlink" : "protect");
      for (const path of protectedPaths) onProtected?.(path);
      return protectedPaths.length === 0;
    }),
  );
}
//#endregion 🛡️TicketProtection

export function cleanIsTicketGeneratedOutputDir(name: string): boolean {
  return CLEAN_TICKET_GENERATED_OUTPUT_DIRS.has(name) || CLEAN_TICKET_GENERATED_PROBE_PREFIXES.some((prefix) => name.startsWith(prefix));
}

export function cleanTicketGeneratedOutputTicketRoot(root: string, abs: string): string | undefined {
  const name = relative(dirname(abs), abs);
  if (name.includes(sep) || !cleanIsTicketGeneratedOutputDir(name)) return undefined;
  return cleanTicketFolderForPath(root, abs);
}
export function cleanDedupePreferShallowest(removals: readonly CleanRemoval[]): CleanRemoval[] {
  const sorted = [...removals].sort((a, b) => a.path.length - b.path.length || a.path.localeCompare(b.path));
  const kept: CleanRemoval[] = [];
  for (const row of sorted) {
    if (kept.some((k) => row.path === k.path || row.path.startsWith(k.path + "/"))) continue;
    kept.push(row);
  }
  return kept;
}
