import { existsSync, lstatSync, readFileSync, readdirSync, mkdirSync, writeFileSync, rmSync, mkdtempSync, chmodSync, renameSync } from "node:fs";
import { copyFile, open } from "node:fs/promises";
import { dirname, resolve, join, isAbsolute, relative } from "node:path";
import { acquireResourceLease, type LeaseWait } from "../../🔒️leases/🟦️.ts";

export type ArtifactPublicationOptions = { readonly signal?: AbortSignal; readonly leaseDirectory: string; readonly onWait?: (progress: LeaseWait) => void };

/** 🧮️ Compares file bytes in bounded async chunks while honoring publication cancellation. */
async function haveIdenticalBytes(source: string, destination: string, signal: AbortSignal): Promise<boolean> {
  const left = await open(source, "r");
  try {
    const right = await open(destination, "r");
    try {
      const leftBytes = Buffer.allocUnsafe(65536), rightBytes = Buffer.allocUnsafe(65536);
      for (;;) {
        signal.throwIfAborted();
        const [a, b] = await Promise.all([left.read(leftBytes, 0, leftBytes.length, null), right.read(rightBytes, 0, rightBytes.length, null)]);
        signal.throwIfAborted();
        if (a.bytesRead !== b.bytesRead || !leftBytes.subarray(0, a.bytesRead).equals(rightBytes.subarray(0, b.bytesRead))) return false;
        if (!a.bytesRead) return true;
      }
    } finally { await right.close(); }
  } finally { await left.close(); }
}

/** 🔍️ Recognizes identical owned files without replacing resources already open in consumers. */
async function hasIdenticalArtifacts(staging: string, marker: string, owner: string, files: ReadonlyMap<string, string>, signal: AbortSignal): Promise<boolean> {
  if (!existsSync(join(staging, marker))) return false;
  const names = [...files.keys()].sort(), metadata = JSON.parse(readFileSync(join(staging, marker), "utf8"));
  if (metadata.version !== 1 || metadata.owner !== owner || JSON.stringify(metadata.files) !== JSON.stringify(names)) return false;
  const paths = new Set([marker]);
  for (const name of names) {
    const destination = resolve(staging, name);
    for (let path = destination; path !== resolve(staging); path = dirname(path)) paths.add(relative(staging, path).replaceAll("\\", "/"));
  }
  const inspect = (directory: string): boolean => readdirSync(directory, { withFileTypes: true }).every(entry => {
    signal.throwIfAborted();
    const path = join(directory, entry.name), name = relative(staging, path).replaceAll("\\", "/");
    return paths.has(name) && !entry.isSymbolicLink() && (entry.isDirectory() ? inspect(path) : entry.isFile());
  });
  if (!inspect(staging)) return false;
  for (const [name, source] of files) {
    signal.throwIfAborted();
    const destination = join(staging, name);
    if (!existsSync(destination)) return false;
    const published = lstatSync(destination), authored = lstatSync(source);
    if (!published.isFile() || published.size !== authored.size || (published.mode & 0o777) !== (authored.mode & 0o777) || !await haveIdenticalBytes(source, destination, signal)) return false;
  }
  return true;
}

/** 🧱️ Reuses identical owned artifacts or replaces their tree with rollback on publication failure. */
export async function stageArtifacts(staging: string, owner: string, files: ReadonlyMap<string, string>, options: ArtifactPublicationOptions): Promise<void> {
  if(!options||!isAbsolute(options.leaseDirectory))throw Error("Explicit absolute publication lease directory required");
  const signal = options.signal ?? new AbortController().signal;
  const lease = await acquireResourceLease({ directory: options.leaseDirectory, resource: `artifact:${resolve(staging)}`, mode: "exclusive", signal, onWait: options.onWait });
  let temporary: string | undefined;
  try {
    const marker = ".nx-artifact.json";
    for (let parent = resolve(staging); dirname(parent) !== parent; parent = dirname(parent)) if (existsSync(parent) && lstatSync(parent).isSymbolicLink()) throw new Error(`Symlink artifact destination: ${parent}`);
    for (const name of files.keys()) if (isAbsolute(name) || name.split(/[\\/]/).includes("..")) throw new Error(`Invalid artifact path ${name}`);
    if (existsSync(staging)) {
      const markerPath = join(staging, marker);
      if (!existsSync(markerPath)) rmSync(staging, { recursive: true, force: true });
      else if (JSON.parse(readFileSync(markerPath, "utf8")).owner !== owner) throw new Error(`Unowned artifact directory: ${staging}`);
    }
    const identical = await hasIdenticalArtifacts(staging, marker, owner, files, signal);
    signal.throwIfAborted();
    if (identical) return;
    mkdirSync(dirname(staging), { recursive: true });
    temporary = mkdtempSync(`${staging}.stage-`);
    const previous = `${temporary}.previous`;
    for (const [name, source] of files) {
      signal.throwIfAborted();
      const destination = join(temporary, name);
      mkdirSync(dirname(destination), { recursive: true });
      await copyFile(source, destination);
      chmodSync(destination, lstatSync(source).mode & 0o777);
    }
    writeFileSync(join(temporary, marker), JSON.stringify({ version: 1, owner, files: [...files.keys()].sort() }) + "\n");
    signal.throwIfAborted();
    if (existsSync(staging)) renameSync(staging, previous);
    try { renameSync(temporary, staging); }
    catch (error) { if (existsSync(previous)) renameSync(previous, staging); throw error; }
    rmSync(previous, { recursive: true, force: true });
  } finally { try { if (temporary) rmSync(temporary, { recursive: true, force: true }); } finally { lease.release(); } }
}
