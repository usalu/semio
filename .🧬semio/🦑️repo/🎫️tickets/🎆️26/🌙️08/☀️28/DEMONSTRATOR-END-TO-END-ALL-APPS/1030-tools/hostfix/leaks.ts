import { findTestkitLeaks } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🚧️testkit-leaks/🟦️.ts";
import { spawnSync } from "bun";
import { existsSync, readFileSync } from "node:fs";
const root = "/Users/ueli/Documents/semio";
const listing = spawnSync(["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard", "--", "*Cargo.toml"], { cwd: root, stdout: "pipe" });
const out: unknown[] = [];
for (const path of new TextDecoder().decode(listing.stdout).split("\0").filter((entry) => entry.endsWith("Cargo.toml"))) {
  if (!path.startsWith(".") && existsSync(root + "/" + path)) out.push(...findTestkitLeaks(path, readFileSync(root + "/" + path, "utf8")));
}
console.log(JSON.stringify(out));
