import systemPolicy from "../../../🗂️workspaces/🦀️cargo/🎛️control/🚪️cli/🧩️entrypoint/🧩️system/🎛️policy/🔣️.json";
import { readFileSync } from "node:fs";
import { join, resolve, dirname } from "node:path";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import {prepareRepositoryCargoCommand,runCargoSystemOperation,cargoSystemMilliseconds,cargoCommandOwner} from "../../../🗂️workspaces/🦀️cargo/🟦️.ts";
import { cargoDirectories } from "../../🦀️cargo/🟦️.ts";
import { repoCacheDirectory } from "../../🟦️.ts";
import { buildBudgetMs } from "../../../../../../../🔨️modules/🏃️process/⏱️budget/🟦️.ts";
import { buildCargoArtifacts, type CargoArtifactBuildPolicyV1, type CargoArtifactBuildOptionsV1 } from "../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";

/** 🏷️ The one version every crate in this workspace carries (`[workspace.package] version`). */
export function workspaceCargoVersion(repoRoot = getWorkspaceRoot()): string {
  const manifest = readFileSync(join(repoRoot, "Cargo.toml"), "utf8");
  const heading = "[workspace.package]";
  const start = manifest.indexOf(heading);
  if (start < 0) throw new Error("Cargo.toml declares no [workspace.package] section");
  const rest = manifest.slice(start + heading.length);
  const end = rest.indexOf("\n[");
  const version = /^\s*version\s*=\s*"([^"]+)"/m.exec(end < 0 ? rest : rest.slice(0, end));
  if (!version) throw new Error("Cargo.toml declares no [workspace.package] version");
  return version[1]!;
}

/** 🏗️ Authors explicit repository compiler storage for one source owner. */
export function repositoryCargoArtifactBuildPolicyV1(cwd: string, root = getWorkspaceRoot()): CargoArtifactBuildPolicyV1 {
  return { version: 1, cwd: resolve(cwd), buildDirectory: cargoDirectories(root).build, leaseDirectory: repoCacheDirectory(root, "agents", "resource-leases"), captureDirectory: process.env.SEMIO_TEST_ARTIFACT_DIR ? resolve(process.env.SEMIO_TEST_ARTIFACT_DIR) : resolve(cwd, "dist"), budgetMs: buildBudgetMs() };
}
/** 🦀️ Composes selected repository preparation with the neutral compiler capture port. */
export async function buildRepositoryCargoArtifacts(manifest: string, args: string[] = [], root = getWorkspaceRoot(), options: CargoArtifactBuildOptionsV1 = {}): Promise<void> {
  return runCargoSystemOperation(root,cargoSystemMilliseconds(systemPolicy),async()=>{const owner=cargoCommandOwner(),abort=()=>owner.cancelDiscovery();options.signal?.addEventListener("abort",abort,{once:true});try{if(options.signal?.aborted)owner.cancelDiscovery();
  const path = resolve(root, manifest);
  await prepareRepositoryCargoCommand(root,[options.command??"build","--manifest-path",path,...args],root,{...process.env,...options.environment});
  await buildCargoArtifacts(path, args, { ...repositoryCargoArtifactBuildPolicyV1(root, root), captureDirectory: process.env.SEMIO_TEST_ARTIFACT_DIR ? resolve(process.env.SEMIO_TEST_ARTIFACT_DIR) : resolve(dirname(path), "dist") }, options);
  }finally{options.signal?.removeEventListener("abort",abort);}});
}
