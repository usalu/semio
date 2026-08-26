import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { loadTaxonomy, semanticDirectoryKindId, semanticOwnedFileProjectionAuthority, validateTaxonomy } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts";

//#region 🧪️TicketImportantOwnerAuthority
const fixtureRoot = join(resolve(import.meta.dir, "../../../../../../../"), "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "📦️packages", "🟦️typescript", "🧫️fixtures");
const golden = JSON.parse(readFileSync(join(fixtureRoot, "🧪️ticket-important-owner-authority", "🔣️.json"), "utf8")) as { schemaVersion: number; cases: { id: string; ownerClaimed: boolean; manifestClaimed: boolean; manifestContent: string; sourceByteLength: number; expected: { contentState: string; disposition: string; hasDestination: boolean; status?: string; problems: string[] } }[] };
const prefix = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17";

describe("ticket important owner authority", () => {
  const taxonomy = loadTaxonomy();

  test("keeps the strict owner projection schema canonical", () => {
    expect(validateTaxonomy(taxonomy)).toEqual([]);
    expect(taxonomy.semanticDirectoryKinds["ticket-important"]?.projectionOnly).toBe(true);
    expect(semanticDirectoryKindId("📌️important", taxonomy, { parentKindId: "ticket-slug" })).toBe("ticket-important");
    expect(Object.keys(taxonomy.semanticOwnedFileProjectionContracts)).toEqual(["ticket-important-markdown-v1"]);
  });

  test("matches every language-neutral owner and lifecycle vector", () => {
    expect(golden.schemaVersion).toBe(1);
    for (const vector of golden.cases) {
      const ownerPath = `${prefix}/${vector.id.toUpperCase()}`;
      const result = semanticOwnedFileProjectionAuthority({
        ownerPath,
        ownerFixedDirectoryContractIds: vector.ownerClaimed ? ["ticket-slug"] : [],
        manifestPath: `${ownerPath}/🎫️ticket.json`,
        manifestFixedFilenameContractIds: vector.manifestClaimed ? ["ticket-manifest"] : [],
        manifestContent: vector.manifestContent,
        sourcePath: `${ownerPath}/📌️important.md`,
        sourceFileKindId: "markdown",
        sourceByteLength: vector.sourceByteLength,
      }, taxonomy);
      expect({ contentState: result.contentState, disposition: result.disposition, hasDestination: result.destinationPath !== undefined, status: result.status, problems: result.problems }).toEqual(vector.expected);
      if (result.destinationPath) expect(result.destinationPath).toBe(`${ownerPath}/📌️important/📝️.md`);
    }
  });

  test("matches an independent JSON Schema status candidate", () => {
    const ownsStatus = new Ajv({ allErrors: true, strict: true }).compile({ type: "object", required: ["status"], properties: { status: { enum: ["closed", "open"] } }, additionalProperties: true });
    for (const vector of golden.cases.filter((entry) => entry.ownerClaimed && entry.manifestClaimed)) {
      const candidate = (() => { try { return ownsStatus(JSON.parse(vector.manifestContent)); } catch { return false; } })();
      expect(candidate).toBe(vector.expected.status !== undefined);
    }
  });

  test("rejects forged contracts and projection-only drift", () => {
    const missing = structuredClone(taxonomy) as typeof taxonomy;
    delete (missing as { semanticOwnedFileProjectionContracts?: unknown }).semanticOwnedFileProjectionContracts;
    expect(validateTaxonomy(missing).some((problem) => problem.includes("semanticOwnedFileProjectionContracts"))).toBe(true);
    const forged = structuredClone(taxonomy) as typeof taxonomy;
    (forged.semanticDirectoryKinds["ticket-important"] as { projectionOnly?: boolean }).projectionOnly = false;
    expect(validateTaxonomy(forged).some((problem) => problem.includes("projectionOnly"))).toBe(true);
    const malformed = structuredClone(taxonomy) as typeof taxonomy;
    delete (malformed.semanticOwnedFileProjectionContracts["ticket-important-markdown-v1"] as { statusDispositions?: unknown }).statusDispositions;
    expect(() => validateTaxonomy(malformed)).not.toThrow();
    expect(validateTaxonomy(malformed).some((problem) => problem.includes("statusDispositions"))).toBe(true);
  });
});
//#endregion 🧪️TicketImportantOwnerAuthority
