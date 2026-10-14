/** 🗂️ Publishes the generated Cargo workspace member list of the given artifact workspaces: `bun 🗂️membership.ts <workspace dir>...`. */
import { discoverCargoWorkspaces, publishCargoWorkspaceMembership } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";
const root = "/Users/ueli/Documents/semio", want = process.argv.slice(2);
for (const owner of discoverCargoWorkspaces(root)) if (want.some(dir => owner.directory === dir)) console.log(owner.directory, publishCargoWorkspaceMembership(root, owner, "write"));
