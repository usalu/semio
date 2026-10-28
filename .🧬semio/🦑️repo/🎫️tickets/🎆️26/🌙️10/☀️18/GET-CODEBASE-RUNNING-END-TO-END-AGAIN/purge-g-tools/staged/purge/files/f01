import Ajv from "ajv";
import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, truncateSync, utimesSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { contractRoot } from "../🏗️shared-fixtures/🟦️.ts";

type Fixture = {
  version: number;
  dayRoot: string;
  tickets: { slug: string; manifest: string | null; generatedFile: string }[];
  nestedGenerated: { slug: string; path: string };
  images: { directory: string; role: string }[];
  imageBackingBytes: number;
};

const fixture: Fixture = JSON.parse(readFileSync(join(contractRoot, "../🧫️fixtures/🏪️clean-managed-storage/🔣️.json"), "utf8"));
const generated = "🗑️generated";
const closedOracle = new Ajv({ strict: true }).compile({ type: "object", required: ["status"], properties: { status: { const: "closed" } }, additionalProperties: true });
const oracleClosed = (manifest: string | null): boolean => {
  try {
    return manifest !== null && closedOracle(JSON.parse(manifest)) === true;
  } catch {
    return false;
  }
};

function build(): string {
  const root = mkdtempSync(join(tmpdir(), "semio-clean-storage-"));
  const put = (path: string, text: string): void => {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), text);
    const old = new Date(Date.now() - 7_200_000);
    utimesSync(join(root, path), old, old);
  };
  if (spawnSync("git", ["init", "-q"], { cwd: root }).status !== 0) throw new Error("git init failed for the managed storage fixture");
  for (const ticket of fixture.tickets) {
    const base = `${fixture.dayRoot}/${ticket.slug}`;
    if (ticket.manifest !== null) put(`${base}/🎫️ticket.json`, ticket.manifest);
    put(`${base}/${generated}/${ticket.generatedFile}`, "live evidence");
  }
  put(`${fixture.dayRoot}/${fixture.nestedGenerated.slug}/${fixture.nestedGenerated.path}`, "nested evidence");
  for (const image of fixture.images) {
    mkdirSync(join(root, image.directory, "m"), { recursive: true });
    put(`${image.directory}/image.json`, "{}");
    put(`${image.directory}/fixed.dmg`, "");
    truncateSync(join(root, image.directory, "fixed.dmg"), fixture.imageBackingBytes);
  }
  return root;
}

describe("workspace cleanup protects what other owners retire", () => {
  test("never removes native cache images or the generated folder of an open ticket, with manifest oracle parity", async () => {
    const { runWorkspaceClean } = await import("../../../🧼️workspace-cleanup/🗑️removal/🟦️.ts");
    expect(fixture.version).toBe(1);
    const root = build();
    try {
      const dry = runWorkspaceClean(root, true);
      for (const ticket of fixture.tickets) expect(existsSync(join(root, fixture.dayRoot, ticket.slug, generated, ticket.generatedFile))).toBe(true);
      expect(dry.removals.map((row) => row.path).filter((path) => path.includes("fixed.dmg") || path.includes("image.json"))).toEqual([]);

      const applied = runWorkspaceClean(root, false);
      for (const ticket of fixture.tickets) {
        const survives = !oracleClosed(ticket.manifest);
        expect(existsSync(join(root, fixture.dayRoot, ticket.slug, generated, ticket.generatedFile)), ticket.slug).toBe(survives);
        expect(applied.removals.some((row) => row.path === `${fixture.dayRoot}/${ticket.slug}/${generated}`), ticket.slug).toBe(!survives);
      }
      expect(existsSync(join(root, fixture.dayRoot, fixture.nestedGenerated.slug, fixture.nestedGenerated.path))).toBe(true);
      for (const image of fixture.images) {
        for (const name of ["fixed.dmg", "image.json", "m"]) expect(existsSync(join(root, image.directory, name)), `${image.role} ${name}`).toBe(true);
      }
      expect(applied.removals.filter((row) => row.path.startsWith(".cache"))).toEqual([]);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });

  test("candidate gates refuse image files and open-ticket generated outputs directly", async () => {
    const { cleanIsNativeImagePath, cleanOpenTicketGeneratedOutputProtection, cleanProjectRemovals } = await import("../../../🧼️workspace-cleanup/🛡️protection/🟦️.ts");
    const { cleanRemovePath } = await import("../../../🧼️workspace-cleanup/🗑️removal/🟦️.ts");
    const root = build();
    try {
      const image = fixture.images[1]!.directory;
      for (const path of [`${image}/fixed.dmg`, `${image}/image.json`, `${image}/m`, image]) expect(cleanIsNativeImagePath(join(root, path)), path).toBe(true);
      expect(cleanIsNativeImagePath(join(root, fixture.dayRoot))).toBe(false);
      const open = `${fixture.dayRoot}/OPEN`;
      expect(cleanOpenTicketGeneratedOutputProtection(root, `${open}/${generated}`)).toHaveLength(1);
      expect(cleanOpenTicketGeneratedOutputProtection(root, `${open}/${generated}/${fixture.tickets[0]!.generatedFile}`)).toHaveLength(1);
      expect(cleanOpenTicketGeneratedOutputProtection(root, `${open}/sub`)).toHaveLength(1);
      expect(cleanOpenTicketGeneratedOutputProtection(root, `${fixture.dayRoot}/CLOSED/${generated}`)).toEqual([]);
      const rows = [
        { kind: "gitignore" as const, path: `${image}/fixed.dmg`, bytes: fixture.imageBackingBytes },
        { kind: "ticket-generated" as const, path: `${open}/${generated}`, bytes: 1 },
        { kind: "gitignore" as const, path: `${open}/sub`, bytes: 1 },
      ];
      expect(cleanProjectRemovals(root, rows, [])).toEqual([]);
      for (const row of rows) expect(cleanRemovePath(root, join(root, row.path), false, []), row.path).toBe(false);
      expect(existsSync(join(root, image, "fixed.dmg"))).toBe(true);
      expect(existsSync(join(root, open, generated))).toBe(true);
    } finally {
      rmSync(root, { recursive: true, force: true });
    }
  });
});
