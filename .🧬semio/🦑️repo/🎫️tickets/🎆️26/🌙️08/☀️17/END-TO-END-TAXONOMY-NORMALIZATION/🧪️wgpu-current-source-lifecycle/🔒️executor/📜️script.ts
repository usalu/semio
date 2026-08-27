import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { lstatSync, readFileSync } from "node:fs";
import { isAbsolute, join, parse, posix, relative, resolve, sep } from "node:path";
import * as implementation from "fixture-normalizer";

type Binding = Readonly<{ root: string; ticketDir: string; planArtifactPath: string; cancelPath: string; taxonomyPath: string; baselineCommit: string; scopes: readonly string[]; exclusions: readonly string[]; pins: readonly Readonly<{ path: string; hash: string }>[] }>;
declare const __WGPU_REHEARSAL_BINDING__: Binding;
const binding = __WGPU_REHEARSAL_BINDING__;
export const boundaryClass = "fixture-rehearsal-only";
const pathFields = new Set(["path", "sourcePath", "destinationPath", "normalizedPath", "targetPath", "catalogPath", "ownerPath", "relativeRoot", "sourceMetadataRoot", "sourceTicketRoot", "canonicalTicketRoot", "relativeEvidencePath"]);

function confinedPath(value: unknown, exact?: string): string {
  assert.equal(typeof value, "string");
  const input = value as string;
  assert.ok(input.length > 0);
  if (!isAbsolute(input)) assert.ok(!input.includes("\\") && posix.normalize(input) === input && !input.split("/").includes(".."));
  const absolute = isAbsolute(input) ? input : join(binding.root, ...input.split("/"));
  assert.equal(resolve(absolute), absolute);
  const child = relative(binding.root, absolute).split(sep).join("/");
  assert.ok(child !== ".." && !child.startsWith("../") && !isAbsolute(child));
  assert.ok(!binding.exclusions.some((root) => child === root || child.startsWith(root + "/")));
  if (exact !== undefined) assert.equal(absolute, exact);
  let current = parse(absolute).root;
  const parts = relative(current, absolute).split(sep);
  for (let index = 0; index < parts.length; index++) {
    current = join(current, parts[index]!);
    try {
      const stat = lstatSync(current);
      assert.ok(!stat.isSymbolicLink() && (index === parts.length - 1 ? stat.isDirectory() || stat.isFile() : stat.isDirectory()), current);
    } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") break; throw error; }
  }
  return absolute;
}

function identities(): void {
  confinedPath(binding.root, binding.root);
  for (const pin of binding.pins) {
    const path = confinedPath(pin.path);
    assert.ok(lstatSync(path).isFile());
    assert.equal(createHash("sha256").update(readFileSync(path)).digest("hex"), pin.hash, pin.path);
  }
}

function scope(value: unknown): void {
  assert.equal(typeof value, "string");
  assert.ok(binding.scopes.includes(value as string));
  confinedPath(value);
}

function coordinates(value: unknown): void {
  if (Array.isArray(value)) { value.forEach(coordinates); return; }
  if (!value || typeof value !== "object") return;
  for (const [key, child] of Object.entries(value)) {
    if (key === "repoRoot") confinedPath(child, binding.root);
    else if (key === "taxonomyPath") confinedPath(child, join(binding.root, binding.taxonomyPath));
    else if (key === "scope" && child !== undefined) scope(child);
    else if (pathFields.has(key) && typeof child === "string" && child.length) confinedPath(child);
    else if (["inputPaths", "outputPaths"].includes(key) && Array.isArray(child)) child.forEach((path) => confinedPath(path));
    else coordinates(child);
  }
}

function options(value: Record<string, unknown>, operation: "inventory" | "plan" | "apply"): void {
  const allowed = operation === "inventory" ? ["repoRoot", "scope", "ticketDir", "cancelFile", "workers", "progress", "taxonomyPath", "baselineCommit", "excludedTreeDigests"] : operation === "plan" ? ["baselineCommit", "excludedTreeDigests", "cancelFile", "progress"] : ["repoRoot", "ticketDir", "expectedBaselineCommit", "planArtifactPath", "expectedPlanDigest", "cancelFile", "resumeJournal", "injectFailureAt", "workers", "progress", "taxonomyPath"];
  for (const key of Object.keys(value)) assert.ok(allowed.includes(key), "Unregistered option: " + key);
  if (operation !== "plan") confinedPath(value.repoRoot, binding.root);
  if (operation === "inventory") { scope(value.scope); confinedPath(value.ticketDir, binding.ticketDir); }
  if (operation === "apply") confinedPath(value.ticketDir, binding.ticketDir);
  if (value.taxonomyPath !== undefined) confinedPath(value.taxonomyPath, join(binding.root, binding.taxonomyPath));
  if (value.cancelFile !== undefined) confinedPath(value.cancelFile, binding.cancelPath);
  if (value.planArtifactPath !== undefined) confinedPath(value.planArtifactPath, binding.planArtifactPath);
  if (value.resumeJournal !== undefined) {
    const path = confinedPath(value.resumeJournal), prefix = join(binding.ticketDir, "🧾️taxonomy-transaction") + sep;
    assert.ok(path.startsWith(prefix));
    assert.match(relative(prefix, path).split(sep).join("/"), /^🔖️[0-9a-f]{64}\/🔂️attempts\/🔢[\ufe0f]?\d{6}\/🔣️\.json$/u);
  }
  if (value.baselineCommit !== undefined) assert.equal(value.baselineCommit, binding.baselineCommit);
  if (value.expectedBaselineCommit !== undefined) assert.equal(value.expectedBaselineCommit, binding.baselineCommit);
  if (value.excludedTreeDigests !== undefined) assert.deepEqual(value.excludedTreeDigests, []);
}

export function inventoryTaxonomy(value: Record<string, unknown>): unknown {
  options(value, "inventory"); identities();
  return implementation.inventoryTaxonomy(value);
}

export function planTaxonomy(inventory: unknown, value: Record<string, unknown>): unknown {
  options(value, "plan"); coordinates(inventory); identities();
  return implementation.planTaxonomy(inventory, value);
}

export function applyTaxonomyPlan(plan: unknown, value: Record<string, unknown>): unknown {
  options(value, "apply"); coordinates(plan); identities();
  const parsed = implementation.parseTaxonomyPlan(plan);
  coordinates(parsed);
  return implementation.applyTaxonomyPlan(parsed, value);
}

export const canonicalJson = implementation.canonicalJson;
export const parseTaxonomyPlan = implementation.parseTaxonomyPlan;
export const taxonomyPlanDigest = implementation.taxonomyPlanDigest;
