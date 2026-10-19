import { receiveScriptProcessInvocation } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { testClosedBrowserActorProducerV1, testClosedBrowserComponentFactory } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏃️execution/🟦️.ts";
const which = process.argv[2] ?? "factory";
await receiveScriptProcessInvocation(process.env, async (invocation: any) => {
  const root = "/Users/ueli/Documents/semio";
  if (which === "factory") await testClosedBrowserComponentFactory(root, invocation);
  else await testClosedBrowserActorProducerV1(root, invocation);
  console.log("[DEBUG] bb-run done", which);
});
