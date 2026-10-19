#!/usr/bin/env bun
/**
 * 🧪️ Captures the frozen Nx authorizer closure and compiles it with the real compiler, without touching any cache partition.
 * Proves a source move (Nx SDK port) keeps the closure capturable and bundle-able under the closed external-module rule.
 */
import { resolve } from "node:path";
import { captureNxAuthorizerSourcesV1 } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import { compileNxAuthorizerSnapshotV1, snapshotNxAuthorizerSourcesV1 } from "../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️snapshot/🟦️.ts";
import { createIssuedResourceControlV1, createScriptProcessEnvelope, withScriptProcessEnvelope } from "../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../../..");
const entry = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📦️runtime/🟦️.ts";
await withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "authorizer-closure-probe", maximumElapsedMilliseconds: 600_000 }, {}, Date.now()), async original => {
  const control = createIssuedResourceControlV1(original, "probe:authorizers", 0);
  const source = await captureNxAuthorizerSourcesV1(root, entry, control.signal);
  const result = await compileNxAuthorizerSnapshotV1(root, snapshotNxAuthorizerSourcesV1(source), control, "bun");
  console.log(JSON.stringify({ sources: source.manifest.sources.length, digest: source.digest, bytes: result.bytes.length, inputs: result.inputs.length, port: result.inputs.filter(path => path.includes("🔌️sdk")) }, null, 2));
});
