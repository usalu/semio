#!/usr/bin/env bun
/**
 * 📣️ Publishes the canonical Cargo member arrays of the named workspace dirs (repository-relative) in `check` or `write` mode.
 *
 * usage: bun 🧾️membership.ts check|write <workspace dir>...
 */
import { resolve } from "node:path";
import { discoverCargoWorkspaces, publishSelectedCargoWorkspaceMembershipV1 } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";

const REPO = resolve(import.meta.dir, "../../../../../../../..");
const [mode, ...dirs] = process.argv.slice(2) as ["check" | "write", ...string[]];
const owners = discoverCargoWorkspaces(REPO);
for (const dir of dirs) {
  const owner = owners.find(row => resolve(REPO, row.directory) === resolve(REPO, dir));
  if (!owner) throw Error("No workspace at " + dir);
  console.log(dir, publishSelectedCargoWorkspaceMembershipV1(REPO, owner, mode, owners) ? "changed" : "unchanged");
}
