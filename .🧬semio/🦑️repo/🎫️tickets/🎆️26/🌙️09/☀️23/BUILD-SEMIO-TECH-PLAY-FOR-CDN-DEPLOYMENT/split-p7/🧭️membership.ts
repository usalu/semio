#!/usr/bin/env bun
/**
 * 🧭️ Publishes the Cargo workspace membership of the given workspace manifests with the repo's own generator
 * (`publishCargoWorkspaceMembership`, `🗂️workspaces/🦀️cargo/🟦️.ts`), so new family crates become members.
 *
 * usage: bun 🧭️membership.ts <workspace Cargo.toml>...
 */
import { resolve } from "node:path";
import { cargoWorkspaceForManifest, publishCargoWorkspaceMembership } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../../..");
for (const manifest of process.argv.slice(2)) {
  const scope = cargoWorkspaceForManifest(root, resolve(manifest));
  console.log(`${manifest}: changed=${publishCargoWorkspaceMembership(root, scope, "write")}`);
}
