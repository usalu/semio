
import { createHash } from "node:crypto";
import { lstatSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";
import { cargoBuildRunLeafV1, cargoProvenanceMerkleRootV1, cargoProvenanceRowsDigestV1, cargoProvenanceSourceV1, cargoUnitLeafV1, type CargoArtifactObservationV1, type CargoInputObservationV1, type CargoProvenanceRootsV1, type CargoResourceWitnessV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";
import { runToolSyncV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
export type { CargoProvenanceRootsV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧾️receipt/🟦️.ts";
import { COMPONENT_PROFILE_POLICIES, COMPONENT_RECEIPT_FILE, componentToolchainV1, encodeComponentReceiptV1, parseComponentReceiptV1 } from "./🧬️schema/🟦️.ts";
import { type ComponentProfileV1, type ComponentReceiptV1 } from "@semio-tech/framework/component-receipt";
export {
  COMPONENT_PROFILE_POLICIES,
  COMPONENT_RECEIPT_FILE,
  COMPONENT_RECEIPT_MAX_BYTES,
  COMPONENT_RECEIPT_SCHEMA_ID,
  COMPONENT_TARGET,
  ITERATE_COMPONENT_MAX_BYTES,
  ITERATE_COMPONENT_PROFILE,
  SHIPPED_COMPONENT_MAX_BYTES,
  SHIPPED_COMPONENT_PROFILE,
  componentProfileV1,
  componentReceiptBindsV1,
  componentReceiptValueV1,
  componentToolchainV1,
  encodeComponentReceiptV1,
  parseComponentReceiptV1,
} from "./🧬️schema/🟦️.ts";
export type { ComponentProfilePolicyV1, ComponentReceiptBindingV1 } from "./🧬️schema/🟦️.ts";
export type { ComponentProfileV1, ComponentReceiptV1 } from "@semio-tech/framework/component-receipt";

/** 📂️ Where one profile's deliverable of one component crate lives: the component bytes and the receipt naming them. */
export type ComponentDeliverableV1 = Readonly<{ profile: ComponentProfileV1; directory: string; wasm: string; receipt: string }>;

/** 📍️ The deliverable of `packageName` (a Cargo package name) under its crate root for one profile. */
export function componentDeliverableV1(crateRoot: string, packageName: string, profile: ComponentProfileV1): ComponentDeliverableV1 {
  const directory = join(crateRoot, COMPONENT_PROFILE_POLICIES[profile].directory);
  return Object.freeze({ profile, directory, wasm: join(directory, `${packageName.replaceAll("-", "_")}.wasm`), receipt: join(directory, COMPONENT_RECEIPT_FILE) });
}

/** 📖️ Reads one receipt file, refusing a symlink or a non-file. */
export function readComponentReceiptV1(path: string): ComponentReceiptV1 {
  const info = lstatSync(path);
  if (info.isSymbolicLink() || !info.isFile() || info.size > 4096) throw new Error(`component receipt ${path} must be a regular file of at most 4096 bytes`);
  return parseComponentReceiptV1(readFileSync(path, "utf8"));
}

/** 💾️ Writes one receipt atomically beside the bytes it names. */
export function writeComponentReceiptV1(path: string, receipt: ComponentReceiptV1): void {
  const text = encodeComponentReceiptV1(receipt);
  parseComponentReceiptV1(text);
  const temporary = `${path}.${process.pid}.tmp`;
  try {
    writeFileSync(temporary, text, { flag: "wx" });
    renameSync(temporary, path);
  } finally {
    rmSync(temporary, { force: true });
  }
}

/** 🦀️ Projects `rustc -vV` onto the receipt's toolchain identity (`rustc <release> (<commit-hash>)`). */
export function rustToolchainIdentityFromVersionV1(verbose: string): string {
  const release = /^release: (\S+)$/mu.exec(verbose)?.[1], commit = /^commit-hash: ([0-9a-f]{40})$/mu.exec(verbose)?.[1];
  const identity = `rustc ${release} (${commit})`;
  if (!release || !commit || !componentToolchainV1(identity)) throw new Error("rustc -vV names no release and commit hash");
  return identity;
}

const toolchains = new Map<string, string>();

/** 🔧️ The toolchain the repository's pinned `rust-toolchain.toml` resolves to, asked once per process. */
export function rustToolchainIdentityV1(repoRoot: string): string {
  const known = toolchains.get(repoRoot);
  if (known) return known;
  const result = runToolSyncV1("cargo", ["-vV"], { command: "rustc", cwd: repoRoot, encoding: "utf8" });
  if (result.status !== 0) throw new Error(`rustc -vV failed: ${result.stderr}`);
  const identity = rustToolchainIdentityFromVersionV1(result.stdout);
  toolchains.set(repoRoot, identity);
  return identity;
}

/** 🧬️ The observed units and build-script runs a component's provenance is derived from: exactly the units of its build (a narrow build's whole receipt, or the
 * unit-graph closure of the component inside a release pass). */
export type ComponentProvenanceObservationV1 = Readonly<{ units: readonly Readonly<{ inputs: readonly CargoInputObservationV1[]; artifacts: readonly CargoArtifactObservationV1[] }>[]; buildResources: readonly Readonly<{ path: string; sha256: string; resources: readonly CargoResourceWitnessV1[] }>[] }>;

/** #️⃣ Derives a receipt's `sourceTreeDigest` and `provenanceSha256` from the units of one component's build: the first is every in-repository file input
 * outside the build directory as sorted `[repo-relative path, sha256]` rows; the second is the RFC 6962 Merkle root over one leaf per unit (its path-normalized
 * input and artifact rows) and one per build-script run (its capture and resource rows) — both free of time and of the machine's absolute layout, so equal
 * sources and outputs give equal digests in every implementation, and a release driver that observed each unit once derives every component's root from
 * the same leaves. */
export function componentProvenanceDigestsV1(observed: ComponentProvenanceObservationV1, roots: CargoProvenanceRootsV1): Readonly<{ sourceTreeDigest: string; provenanceSha256: string }> {
  const sources = new Map<string, string>();
  for (const unit of observed.units) for (const input of unit.inputs) { const path = input.kind === "file" && input.sha256 !== null ? cargoProvenanceSourceV1(input.path, roots) : null; if (path) sources.set(path, input.sha256!); }
  if (sources.size === 0) throw new Error("cargo provenance names no in-repository source");
  return Object.freeze({ sourceTreeDigest: cargoProvenanceRowsDigestV1([...sources]), provenanceSha256: cargoProvenanceMerkleRootV1([...observed.units.map((unit) => cargoUnitLeafV1(unit, roots)), ...observed.buildResources.map((run) => cargoBuildRunLeafV1(run, roots))]) });
}

/** 🔎️ Every way one receipt disagrees with the bytes it is read beside: identity, profile, component bytes and the committed
 * semantic descriptor pack. An empty list is a receipt that names exactly these bytes. */
export function componentReceiptFindingsV1(receipt: ComponentReceiptV1, expected: Readonly<{ pluginId: string; packageId: string; profile: ComponentProfileV1 }>, wasm: Uint8Array, semanticPack: Uint8Array): readonly string[] {
  const findings: string[] = [];
  if (receipt.pluginId !== expected.pluginId || receipt.packageId !== expected.packageId) findings.push(`receipt names ${receipt.pluginId} (${receipt.packageId}), not ${expected.pluginId} (${expected.packageId})`);
  if (receipt.profile !== expected.profile) findings.push(`receipt is a ${receipt.profile} receipt, not ${expected.profile}`);
  if (receipt.wasmBytes !== wasm.byteLength || createHash("sha256").update(wasm).digest("hex") !== receipt.wasmSha256) findings.push("component bytes are not the bytes the receipt names");
  if (createHash("sha256").update(semanticPack).digest("hex") !== receipt.semanticDescriptorSha256) findings.push("committed semantic descriptor is not the descriptor the receipt names");
  return findings;
}
