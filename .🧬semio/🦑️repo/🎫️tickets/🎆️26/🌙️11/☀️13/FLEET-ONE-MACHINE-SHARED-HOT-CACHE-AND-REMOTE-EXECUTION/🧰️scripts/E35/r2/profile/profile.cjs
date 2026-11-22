const { createHash } = require("node:crypto");
const policy = Object.freeze({ ...require("./🧬️schema/🔣️.json").$defs.Policy.const });
const canonical = value => Array.isArray(value) ? value.map(canonical) : value !== null && typeof value === "object" ? Object.fromEntries(Object.keys(value).sort().map(key => [key, canonical(value[key])])) : value;

/** 🪪️ Digests what a dependency receipt depends on in one workspace manifest: every top-level field but the declared inert ones (`scripts`), canonically ordered, so an edit of an inert field or a reordered key keeps the digest and an edit of `exports`, a dependency group, `name` or `version` changes it; `null` for a file that is not a manifest object, which a receipt then compares by its exact bytes.
 * @see ./🧬️schema/🔣️.json */
function manifestProfileSha256V1(path, content) {
  const name = String(path).split("/").at(-1);
  if (name !== policy.fileName) return null;
  const bytes = typeof content === "string" ? Buffer.from(content, "utf8") : Buffer.from(content);
  if (bytes.length > policy.maximumBytes) return null;
  try {
    const document = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
    if (document === null || typeof document !== "object" || Array.isArray(document)) return null;
    const kept = Object.fromEntries(Object.entries(document).filter(([key]) => !policy.excludedTopLevelFields.includes(key)));
    return createHash("sha256").update(JSON.stringify(canonical(kept))).digest("hex");
  } catch { return null; }
}
module.exports = { manifestProfileSha256V1, manifestProfilePolicyV1: policy };
