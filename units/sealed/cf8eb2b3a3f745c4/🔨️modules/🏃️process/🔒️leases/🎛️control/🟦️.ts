import { dirname, isAbsolute, relative, resolve, sep } from "node:path";

/** 🔒️ Resource lease admission control: the access descriptor, the awaited wait progress, the caller control and the materialized path bound every lease owner shares; a leaf, so leases, retirement and routing never import each other for these. */
export type ResourceAccess = { readonly resource: string; readonly mode: "shared" | "exclusive" };
export type LeaseWait = ResourceAccess & { readonly elapsedMs: number };
/** 🎛️ Preserves an admitted caller's existing signal, deadline and awaited wait transitions. */
export interface ResourceLeaseControl {
  readonly signal: AbortSignal;
  remainingMilliseconds(): number | null;
  publish(progress: LeaseWait): void | Promise<void>;
  yieldContinuation(): Promise<void>;
}

/** ⏳️ Refuses expired caller authority without replacing its cancellation signal. */
export function checkResourceLeaseControl(control: ResourceLeaseControl): number | null {
  if (!control || !(control.signal instanceof AbortSignal) || typeof control.remainingMilliseconds !== "function" || typeof control.publish !== "function" || typeof control.yieldContinuation !== "function") throw Error("Complete resource admission control required");
  control.signal.throwIfAborted();
  const remaining = control.remainingMilliseconds();
  if (remaining !== null && (!Number.isFinite(remaining) || remaining <= 0)) throw Error("Original compiler admission deadline exhausted");
  return remaining;
}

/** ⌛️ Calls `expire` once with the refusal when the control's authority ends. The control is read again every time its end is believed due, so a renewable (idle-window) deadline moves the expiry and a fixed one is met exactly; one delay never exceeds the native timer range. Returns the call that retires the watch. */
export function watchResourceLeaseControlDeadlineV1(control: ResourceLeaseControl, expire: (reason: unknown) => void): () => void {
  let active = true, timer: ReturnType<typeof setTimeout> | undefined;
  const arm = (): void => {
    if (!active) return;
    let remaining: number | null;
    try { remaining = checkResourceLeaseControl(control); } catch (error) { active = false; expire(error); return; }
    if (remaining === null) return;
    timer = setTimeout(arm, Math.max(1, Math.min(2147483647, Math.ceil(remaining))));
  };
  arm();
  return () => { active = false; if (timer !== undefined) clearTimeout(timer); };
}

/** 🚧️ The directories this process must never materialize a path in, each with its refusal; one table per process on a registered symbol, so every loaded copy of this leaf shares it. */
const fencedDirectories: Map<string, string> = ((globalThis as Record<symbol, Map<string, string> | undefined>)[Symbol.for("semio.process.materialized-path-fences.v1")] ??= new Map());
/** 🚧️ Fences directories for the rest of this process: every later materialized path at or below one of them is refused with its reason (a test process fences the checkout's live store this way). */
export function fenceMaterializedPathsV1(directories: readonly string[], reason: string): void {
  for (const directory of directories) fencedDirectories.set(resolve(directory), reason);
}

/** 🧲️ The directories a fenced store admits for reading only: installed tool material that a private repository adopts in place; one table per process on a registered symbol. */
const adoptedDirectories: Set<string> = ((globalThis as Record<symbol, Set<string> | undefined>)[Symbol.for("semio.process.materialized-path-adoptions.v1")] ??= new Set());
/** 🧲️ Admits installed tool directories below a fenced store as read-only material: a materialized path inside one passes the fence, while the store's image, lease and partition paths stay fenced and no store root contains an adopted directory, so no writer of a private store can address it. */
export function adoptMaterializedReadOnlyDirectoriesV1(directories: readonly string[]): void {
  for (const directory of directories) adoptedDirectories.add(resolve(directory));
}

/** 🧲️ The adopted read-only directory this exact directory is, or undefined when it is none: identity by path, so a clone source is only accepted when it is the very installed material that was adopted. */
export function adoptedReadOnlyDirectoryV1(directory: string): string | undefined {
  const absolute = resolve(directory);
  return adoptedDirectories.has(absolute) ? absolute : undefined;
}

/** 🧲️ Requires a path to lie inside one adopted read-only directory and returns it resolved; a path that leaves the directory is refused like any path that escapes its enforcing boundary. */
export function requireWithinAdoptedDirectoryV1(directory: string, path: string): string {
  const absolute = requireMaterializedPathV1(path), distance = relative(directory, absolute);
  if (!adoptedDirectories.has(resolve(directory)) || distance === ".." || distance.startsWith(".." + sep) || isAbsolute(distance)) throw Error("Adopted read-only path escapes its directory");
  return absolute;
}

/** 📏️ Checks the complete resolved materialized path in both portable string measures and refuses one inside a fenced directory. */
export function requireMaterializedPathV1(path: string): string {
  const absolute = resolve(path);
  if (absolute.includes("\0") || absolute.length > 256 || [...absolute].length > 256) throw Error("Materialized absolute path exceeds 256 characters");
  for (const [directory, reason] of fencedDirectories) if ((absolute === directory || absolute.startsWith(directory + sep)) && ![...adoptedDirectories].some(adopted => absolute === adopted || absolute.startsWith(adopted + sep))) throw Error(reason + ": " + absolute);
  return absolute;
}

/** 🔗️ Whether an ancestor symlink belongs to the operating system rather than to the store: a link directly below the filesystem root that only root could create (macOS `/var`, `/tmp`, `/etc`), or on Windows any link directly below the drive root. Such a link cannot be substituted by the store's users, so the strict no-symlink rule stands for every other ancestor. */
export function isSystemLeaseLinkV1(current: string, stat: Readonly<{ uid: number }>): boolean {
  const parent = dirname(current);
  if (process.platform === "win32") return dirname(parent) === parent;
  return stat.uid === 0 && (process.getuid?.() !== 0 || dirname(parent) === parent);
}
