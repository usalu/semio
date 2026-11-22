/** 🧾️ Reconciles the build owner registry for the given owner roots; pass `--write` to persist.
 * @see ../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts */
import {reconcileBuildOwnerRegistry} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts";
const write = process.argv.includes("--write");
const roots = process.argv.slice(2).filter(argument => argument !== "--write");
const result = reconcileBuildOwnerRegistry(process.cwd(), {scope: roots, write});
console.log(JSON.stringify({write, added: result.added, removed: result.removed, changed: result.changed, findings: result.findings}, null, 1));
