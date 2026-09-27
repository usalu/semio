/** 🚪️ WG11 ticket entry of the (window-3) renderer-wgpu verb `hub-collaboration-acceptance`: the same CLI, run from the ticket's
 * staged harness copy. Usage: bun harness/wg11-verb.ts --journey <j> --hub <url> [...] (credentials from the environment). */
import { runHubCollaborationCli } from "./🤝️hub-collaboration/🟦️.ts";

await runHubCollaborationCli("/Users/ueli/Documents/semio", "/Users/ueli/Documents/semio/.tmp-ticket/wp-wg11/generated/hub-collaboration", process.argv.slice(2), {
  nativeModules: () => "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/native/release/block2d",
});
