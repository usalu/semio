import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { verifySemanticDescriptorPairBytesV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🧬️descriptor-pair/🟦️.ts";
import { DESCRIPTOR_JSON_FILENAME, DESCRIPTOR_PACK_FILENAME } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts";
import { componentStageTargetsV1, type ComponentStageTargetV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧩️component-stage/🟦️.ts";

export const workspace = resolve(import.meta.dirname, "../../../../../../..");

/** 🧾️ Checks an authored pair using the same identity and semantic digest as a release component. */
export function authoredReleaseDigestV1(target: Pick<ComponentStageTargetV1, "pluginId" | "packageId" | "role">, json: Uint8Array, pack: Uint8Array): string {
  const descriptor = verifySemanticDescriptorPairBytesV1(json, pack);
  if (descriptor.pluginId !== target.pluginId || descriptor.packageId !== target.packageId || descriptor.role !== target.role) {
    throw new Error(`${target.pluginId}: descriptor pair does not match its release owner`);
  }
  return descriptor.semanticSha256;
}

/** 🧭️ Maps the requested Demonstrator roster to canonical release descriptor owners. */
export function authoredReleaseDigestsV1(roots: readonly string[]): Record<string, string> {
  const targets = componentStageTargetsV1(workspace);
  const errors: string[] = [];
  const digests = new Map<string, string>();
  for (const pluginId of roots) {
    try {
      const matches = targets.filter(target => target.pluginId === pluginId);
      const [target] = matches;
      if (!target || matches.length !== 1) throw new Error(`expected one release component owner, found ${matches.length}`);
      const digest = authoredReleaseDigestV1(target, readFileSync(join(target.ownerRoot, DESCRIPTOR_JSON_FILENAME)), readFileSync(join(target.ownerRoot, DESCRIPTOR_PACK_FILENAME)));
      digests.set(pluginId, digest);
    } catch (error) {
      errors.push(`${pluginId}: ${error instanceof Error ? error.message : error}`);
    }
  }
  if (errors.length) throw new Error(errors.join("\n"));
  return Object.fromEntries(digests);
}

if (import.meta.main) {
  try {
    console.log(JSON.stringify(authoredReleaseDigestsV1(process.argv.slice(2))));
  } catch (error) {
    console.error(error instanceof Error ? error.message : error);
    process.exitCode = 1;
  }
}
