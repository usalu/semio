/** ⌨️ H13: the DB CLI process-entry wait census that `verify interactivity` reaches only after its whole-repo walk — the
 * self-tests plus the live CLI source (production + its test evidence, read exactly as the audit reads it).
 * usage: bun h13-dbcli-probe.ts */
import { interactivityDbCliFailures, policyReadRustPolicySource } from "/Users/ueli/Documents/semio/📜️script.ts";
import { interactivityDbCliSelfTests } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-db-cli/🟦️.ts";

interactivityDbCliSelfTests();
const failures = interactivityDbCliFailures(policyReadRustPolicySource("/Users/ueli/Documents/semio", "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️.rs"));
console.log(`[h13 db-cli census] self-tests=4 live-failures=${failures.length}${failures.length ? `: ${failures.join("; ")}` : ""}`);
process.exit(failures.length ? 1 : 0);
