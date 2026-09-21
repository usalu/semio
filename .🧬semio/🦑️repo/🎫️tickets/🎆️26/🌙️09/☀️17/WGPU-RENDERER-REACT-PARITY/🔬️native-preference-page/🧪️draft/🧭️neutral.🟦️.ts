import { expect, test } from "bun:test";
import { appendFileSync, existsSync, mkdtempSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";

import Ajv from "ajv";

type WriteCase = {
  readonly id: "exact-document-round-trip" | "one-byte-over-document" | "cancel-after-two-steps" | "supersede-after-one-step";
  readonly bytes: number;
  readonly writeSteps: number;
  readonly replacementWriteSteps?: number;
  readonly write: string;
  readonly publish: string;
  readonly read: string;
};

const root = resolve(
  process.cwd(),
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🔬️native-preference-page",
);
const fixture = JSON.parse(readFileSync(resolve(root, "🧫️fixtures/🔣️.json"), "utf8")) as {
  readonly authority: {
    readonly serviceStepBytes: number;
    readonly configDocumentMaxBytes: number;
    readonly documentMaxWriteSteps: number;
    readonly storedFieldMaxBytes: number;
  };
  readonly ownership: {
    readonly tokenFields: readonly string[];
    readonly exactTokenRequired: boolean;
    readonly supersedingWriteInvalidatesPriorToken: boolean;
    readonly cancelledWriteMayPublish: boolean;
  };
  readonly publication: {
    readonly maximumBytesPerStep: number;
    readonly visibility: string;
    readonly readDuringWrite: string;
  };
  readonly cases: readonly WriteCase[];
  readonly integrityCases: readonly { readonly id: string; readonly publish: string; readonly read: string }[];
};
const schema = JSON.parse(readFileSync(resolve(root, "🧬️schema/🔣️.json"), "utf8")) as object;
const validate = new Ajv({ allErrors: true, strict: true }).compile(schema);

test("the retained native preference document fixture satisfies its strict schema", () => {
  expect(validate(fixture), JSON.stringify(validate.errors)).toBeTrue();
});

test("the schema refuses unknown, substituted, duplicated, and capacity-drifted contracts", () => {
  const rows = fixture.cases;
  const integrity = fixture.integrityCases;
  const invalid = [
    { ...fixture, inventedFallback: true },
    { ...fixture, cases: [rows[0], rows[0], rows[2], rows[3]] },
    { ...fixture, cases: rows.map((row) => row.id === "exact-document-round-trip" ? { ...row, bytes: 16384 } : row) },
    { ...fixture, authority: { ...fixture.authority, storedFieldMaxBytes: 16384 } },
    { ...fixture, integrityCases: [integrity[0], integrity[0], integrity[2]] },
  ];
  for (const candidate of invalid) expect(validate(candidate)).toBeFalse();
});

test("one 64 KiB document advances in at most four 16 KiB retained steps", () => {
  const authority = fixture.authority;
  expect(authority.configDocumentMaxBytes).toBe(64 * 1024);
  expect(authority.serviceStepBytes).toBe(16 * 1024);
  expect(authority.documentMaxWriteSteps).toBe(4);
  expect(authority.configDocumentMaxBytes).toBe(authority.serviceStepBytes * authority.documentMaxWriteSteps);
  expect(fixture.publication.maximumBytesPerStep).toBe(authority.serviceStepBytes);
  expect(authority.storedFieldMaxBytes).toBe(4 * 1024);
});

test("exact ownership gates cancellation, supersession, and atomic publication", () => {
  expect(fixture.ownership.tokenFields).toEqual(["owner", "generation"]);
  expect(fixture.ownership.exactTokenRequired).toBeTrue();
  expect(fixture.ownership.supersedingWriteInvalidatesPriorToken).toBeTrue();
  expect(fixture.ownership.cancelledWriteMayPublish).toBeFalse();
  expect([fixture.publication.visibility, fixture.publication.readDuringWrite]).toEqual(["atomic-replace", "last-committed-document"]);
  expect(fixture.cases).toEqual([
    { id: "exact-document-round-trip", bytes: 65536, writeSteps: 4, write: "admit", publish: "admit", read: "exact" },
    { id: "one-byte-over-document", bytes: 65537, writeSteps: 0, write: "refuse", publish: "not-started", read: "prior-exact" },
    { id: "cancel-after-two-steps", bytes: 65536, writeSteps: 2, write: "cancel", publish: "refuse", read: "prior-exact" },
    { id: "supersede-after-one-step", bytes: 65536, writeSteps: 1, replacementWriteSteps: 4, write: "supersede", publish: "replacement-only", read: "replacement-exact" },
  ]);
  expect(fixture.integrityCases.map(({ id }) => id)).toEqual(["truncated-temporary-file", "trailing-temporary-byte", "non-owner-token"]);
});

test("the Node filesystem oracle replaces one exact 64 KiB value and plus one preserves it", () => {
  const directory = mkdtempSync(join(tmpdir(), "semio-native-preference-oracle-"));
  const destination = join(directory, "ui-prefs.json");
  const temporary = join(directory, "ui-prefs.json.pending");
  const prior = Buffer.from('{"version":1,"preferences":{}}');
  const exact = Buffer.allocUnsafe(fixture.authority.configDocumentMaxBytes);
  for (let index = 0; index < exact.length; index += 1) exact[index] = (index * 31 + 17) & 0xff;

  const publish = (bytes: Buffer): boolean => {
    if (bytes.length > fixture.authority.configDocumentMaxBytes) return false;
    writeFileSync(temporary, Buffer.alloc(0), { flag: "wx" });
    for (let offset = 0; offset < bytes.length; offset += fixture.authority.serviceStepBytes) {
      appendFileSync(temporary, bytes.subarray(offset, offset + fixture.authority.serviceStepBytes));
      expect(readFileSync(destination)).toEqual(prior);
    }
    renameSync(temporary, destination);
    return true;
  };

  try {
    writeFileSync(destination, prior);
    expect(publish(exact)).toBeTrue();
    expect(readFileSync(destination)).toEqual(exact);
    expect(publish(Buffer.concat([exact, Buffer.from([0xff])]))).toBeFalse();
    expect(existsSync(temporary)).toBeFalse();
    expect(readFileSync(destination)).toEqual(exact);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});
