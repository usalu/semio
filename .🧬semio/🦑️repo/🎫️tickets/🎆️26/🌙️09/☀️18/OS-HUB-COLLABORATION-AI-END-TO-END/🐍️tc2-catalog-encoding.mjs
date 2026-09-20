// 🔎️ Recomputes the neutral trusted open-target catalog encoding of the document-open-plan fixture
// exactly as `proveDocumentOpenPlanFixture` does, to locate a drift between the fixture's pinned
// `catalogEncoding.expectedHex` and the rows it pins them for.
import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../../../../../../../", import.meta.url));
const fixture = JSON.parse(readFileSync(root + "🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json", "utf8"));
const prefix = (bytes) => {
  const length = Buffer.alloc(8);
  length.writeBigUInt64BE(BigInt(bytes.byteLength));
  return Buffer.concat([length, bytes]);
};
const encode = (rows) => {
  const count = Buffer.alloc(4);
  count.writeUInt32BE(rows.length);
  const encoded = rows.map((row) => {
    const version = Buffer.alloc(4);
    version.writeUInt32BE(row.package.executionProtocol.appChannelVersion);
    const fields = [
      Buffer.from(row.package.pluginId, "utf8"), Buffer.from(row.package.packageId, "utf8"), Buffer.from(row.package.version, "utf8"),
      Buffer.from(row.package.componentSha256, "hex"), Buffer.from(row.package.componentBlake3, "hex"), Buffer.from(row.package.descriptorByteSha256, "hex"), version,
      Buffer.from(row.artifact.kind, "utf8"), Buffer.from(row.artifact.schema, "utf8"), Buffer.from(row.artifact.packSchemaHash, "hex"),
      Buffer.from(row.parentDialect.artifactKind, "utf8"), Buffer.from(row.parentDialect.standard, "utf8"), Buffer.from(row.parentDialect.subset, "utf8"),
      Buffer.from(row.surface.surfaceId, "utf8"), Buffer.from(row.surface.appId, "utf8"), Buffer.from(row.surface.windowKindId, "utf8"),
      Buffer.from(row.surface.role, "utf8"), Buffer.from(row.surface.rendererTarget, "utf8"),
      Buffer.from([row.grant.read ? 1 : 0, row.grant.write ? 1 : 0, row.grant.observe ? 1 : 0]),
    ];
    return Buffer.concat(fields.map(prefix));
  });
  return Buffer.concat([Buffer.from("semio/hub/openable-document-catalog/v1\0"), count, ...encoded]);
};
const actual = encode(fixture.catalogRows);
console.log("rows", fixture.catalogRows.length);
console.log("actualHex     ", actual.toString("hex").slice(0, 120));
console.log("expectedHex   ", fixture.catalogEncoding.expectedHex.slice(0, 120));
console.log("hex equal", actual.toString("hex") === fixture.catalogEncoding.expectedHex);
console.log("actualGen  ", createHash("sha256").update(actual).digest("hex"));
console.log("expectedGen", fixture.catalogEncoding.expectedGenerationId);
console.log("lengths", actual.toString("hex").length, fixture.catalogEncoding.expectedHex.length);
console.log("fieldOrder", JSON.stringify(fixture.catalogEncoding.fieldOrder));
