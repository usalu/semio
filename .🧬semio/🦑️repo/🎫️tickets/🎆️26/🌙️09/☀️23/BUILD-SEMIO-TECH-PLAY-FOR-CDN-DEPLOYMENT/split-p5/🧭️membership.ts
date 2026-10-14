#!/usr/bin/env bun
/**
 * 🧭️ Publishes the canonical Cargo workspace membership (members and child exclusions) of the named workspace
 * manifests through the repository's own membership generator.
 *
 * usage: bun 🧭️membership.ts check|write <workspace Cargo.toml relative to the repo>...
 */
import { resolve } from "node:path";
import { cargoWorkspaceForManifest, publishCargoWorkspaceMembership } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";

const REPO = resolve(import.meta.dir, "../../../../../../../..");
const [mode, ...manifests] = process.argv.slice(2) as ["check" | "write", ...string[]];
for (const manifest of manifests) {
  const owner = cargoWorkspaceForManifest(REPO, manifest);
  console.log(`${manifest}: ${publishCargoWorkspaceMembership(REPO, owner, mode) ? "changed" : "unchanged"}`);
}
