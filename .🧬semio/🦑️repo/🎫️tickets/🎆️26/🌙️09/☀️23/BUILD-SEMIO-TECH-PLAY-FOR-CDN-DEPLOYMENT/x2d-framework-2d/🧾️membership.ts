#!/usr/bin/env bun
/**
 * 📣️ Publishes the canonical Cargo member arrays of the named workspace dirs (repository-relative) in `check` or `write` mode,
 * or prints the member additions and removals the canonical array would make in `diff` mode (copy of `split-p4/🧾️membership.ts`).
 *
 * usage: bun 🧾️membership.ts check|write|diff <workspace dir>...
 */
import { readFileSync } from "node:fs";
import { relative, resolve } from "node:path";
import { cargoWorkspaceMembers, discoverCargoWorkspaces, publishSelectedCargoWorkspaceMembershipV1 } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";

const REPO = resolve(import.meta.dir, "../../../../../../../..");
const [mode, ...dirs] = process.argv.slice(2) as ["check" | "write" | "diff", ...string[]];
const owners = discoverCargoWorkspaces(REPO);
for (const dir of dirs) {
  const owner = owners.find(row => resolve(REPO, row.directory) === resolve(REPO, dir));
  if (!owner) throw Error("No workspace at " + dir);
  if (mode === "diff") {
    const current = new Set<string>((Bun.TOML.parse(readFileSync(resolve(REPO, owner.manifest), "utf8")) as { workspace: { members: string[] } }).workspace.members);
    const canonical = new Set(cargoWorkspaceMembers(REPO, owner).map(row => relative(resolve(REPO, owner.directory), resolve(REPO, row.directory)).replaceAll("\\", "/") || "."));
    for (const member of canonical) if (!current.has(member)) console.log(`${dir} + ${member}`);
    for (const member of current) if (!canonical.has(member)) console.log(`${dir} - ${member}`);
    continue;
  }
  console.log(dir, publishSelectedCargoWorkspaceMembershipV1(REPO, owner, mode, owners) ? "changed" : "unchanged");
}
