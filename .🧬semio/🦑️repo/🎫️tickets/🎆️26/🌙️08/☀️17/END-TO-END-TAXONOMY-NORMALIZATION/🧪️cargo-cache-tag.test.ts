import { describe, expect, test } from "bun:test";
import {
  fixedFilenameContractIdsForPath,
  loadTaxonomy,
  semanticDirectoryKindId,
  validateTaxonomy,
} from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts";

//#region 🧪️CargoCacheTag
const embeddedCacheTags = [
  "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-ONE-POOL-WORKER-RUNTIME/🧪️target-os-process-pool/CACHEDIR.TAG",
  "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-os-errors/CACHEDIR.TAG",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-os-errors/CACHEDIR.TAG",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-shell-owned-schema/CACHEDIR.TAG",
  "🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-os-errors/CACHEDIR.TAG",
  "🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-owned-wasm-core/CACHEDIR.TAG",
] as const;

describe("cargo cache tag fixed authority", () => {
  const taxonomy = loadTaxonomy();

  test("keeps strict taxonomy valid", () => {
    expect(validateTaxonomy(taxonomy)).toEqual([]);
  });

  test("resolves the six evidenced embedded ticket target roots", () => {
    for (const path of embeddedCacheTags) {
      const parent = path.split("/").at(-2)!;
      expect(semanticDirectoryKindId(parent, taxonomy)).toBe("ticket-cargo-target-evidence");
      expect(fixedFilenameContractIdsForPath(path, taxonomy, { parentDirectoryKindId: "ticket-cargo-target-evidence" })).toEqual(["cargo-cache-tag"]);
    }
  });

  test("rejects basename-only and noncanonical-parent matches", () => {
    const path = embeddedCacheTags[0]!;
    expect(fixedFilenameContractIdsForPath(path, taxonomy)).toEqual([]);
    expect(fixedFilenameContractIdsForPath(path, taxonomy, { parentDirectoryKindId: "test-case" })).toEqual([]);
    expect(semanticDirectoryKindId("🧪️target_OS", taxonomy)).toBeNull();
  });

  test("rejects production lookalikes and malformed ticket prefixes", () => {
    const context = { parentDirectoryKindId: "ticket-cargo-target-evidence" } as const;
    expect(fixedFilenameContractIdsForPath("🧰️framework/🧪️target-os-errors/CACHEDIR.TAG", taxonomy, context)).toEqual([]);
    expect(fixedFilenameContractIdsForPath("🎫️tickets/🎆️26/🌙️08/☀️20/TICKET/🧪️target-os-errors/CACHEDIR.TAG", taxonomy, context)).toEqual([]);
    expect(fixedFilenameContractIdsForPath(".🧬semio/🦑️repo/🎫️tickets/🎆️2x/🌙️08/☀️20/TICKET/🧪️target-os-errors/CACHEDIR.TAG", taxonomy, context)).toEqual([]);
  });
});
//#endregion 🧪️CargoCacheTag
