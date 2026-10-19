import { browserBundleTestOwners } from "./bb-exec-copy.ts";
const owners = browserBundleTestOwners();
const which = process.argv[2];
if (which === "host") await owners.testBrowserHostActivation();
else if (which === "manifest") await owners.testBrowserActorCodegenManifest();
else throw new Error("which");
console.log("[DEBUG] bb-owners done", which);
