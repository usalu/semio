/** 🧾️ Matches the published Draw browser receipt to the checked-in semantic descriptor. */
import { createHash } from "node:crypto";
import { join } from "node:path";
export async function drawPublicationHasCurrentDescriptor(): Promise<boolean> {
  const receipt = Bun.file(join(process.cwd(), ".cache/b/p/m/browser/prototype/modules/🖍️draw/🧾️receipt.json"));
  if (!await receipt.exists()) return false;
  const pack = Bun.file(join(process.cwd(), "🌎️hub/🧩️compositions/🖍️draw/🧩️component/🛂️.descriptor.semio"));
  const expected = createHash("sha256").update(await pack.bytes()).digest("hex");
  return (await receipt.json()).semanticDescriptorSha256 === expected;
}
