import { createHash } from "node:crypto";
import { basename } from "node:path";
import { updateBuildOwnerRegistryV1 } from "../../🏗️build/🕸️graph/🧾️registry-consistency/🔏️writer/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };
import policy from "./🔣️.json" with { type: "json" };

/** ✍️ Publishes the exact `bytes` a reference edit computed over the file that holds `expected`: the writer of the file refuses a file that is no longer the preimage and refuses invalid text, so a refusal leaves the file byte for byte. */
export type OwnFileWriterV1 = (target: string, expected: Readonly<{ contentHash: string; mode: number }>, bytes: Buffer) => void;

/** 🪪️ The implementations a declared writer id may name; a declaration without one is refused. */
const OWN_FILE_WRITERS_V1: Readonly<Record<string, OwnFileWriterV1>> = {
  "build-owner-registry": (target, expected, bytes) => {
    updateBuildOwnerRegistryV1(target, (source) => {
      if (createHash("sha256").update(source).digest("hex") !== expected.contentHash) throw new Error(`Reference edit preimage changed: ${target}`);
      return { value: undefined, text: bytes.toString("utf8") };
    }, { mode: expected.mode });
  },
};

let declared: ReadonlyMap<string, { id: string; writer: OwnFileWriterV1 }> | undefined;

/** 📖️ Reads and validates the declared mapping of file names to their own writers (schema `🧬️schema/🔣️.json`, data `🔣️.json`); an unknown writer id or a file name declared twice is refused. */
export function readOwnFileWritersV1(value: unknown = policy): ReadonlyMap<string, { id: string; writer: OwnFileWriterV1 }> {
  if (validateJsonSchemaSubset(schema, value).length > 0) throw new Error("Canonical own-file writer declaration required");
  const rows = new Map<string, { id: string; writer: OwnFileWriterV1 }>();
  for (const entry of (value as typeof policy).writers) {
    const writer = OWN_FILE_WRITERS_V1[entry.id];
    if (!writer) throw new Error(`Own-file writer ${entry.id} has no implementation`);
    if (rows.has(entry.fileName)) throw new Error(`Own-file writer declared twice for ${entry.fileName}`);
    rows.set(entry.fileName, { id: entry.id, writer });
  }
  return rows;
}

/** 🔎️ The declared writer of the file a reference edit would rewrite at `path` (matched by file name), or `undefined` when the engine publishes that file itself; see `🔣️.json`. */
export function ownFileWriterFor(path: string): OwnFileWriterV1 | undefined {
  declared ??= readOwnFileWritersV1();
  return declared.get(basename(path))?.writer;
}
