import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { BundleScript, runExactCargoLaws } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";


export class NativeOpenableIdentityCheckScript extends BundleScript {
  async run(): Promise<void> {
    const owner = join(this.root, "..", "..");
    const fixtureRoot = join(owner, "🧫️fixtures/🪪️native-openable-identity/🧬️v1");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const module = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ allErrors: true, strict: true });
    ajv.addKeyword({ keyword: "x-semio-formats", metaSchema: { type: "array", items: { type: "string" } } });
    ajv.addSchema(module);
    const validate = ajv.compile({ $ref: `${module.$id}#/$defs/VcsNativeOpenableIdentity` });
    if (!validate(fixture)) throw new Error(`VCS identity fixture is invalid: ${ajv.errorsText(validate.errors)}`);
    const identities = new Set<string>();
    for (const hostile of fixture.hostileCases) {
      if (identities.has(hostile.name)) throw new Error("duplicate hostile identity case");
      identities.add(hostile.name);
      const candidate = structuredClone(fixture);
      if (hostile.value === null) delete candidate.authority[hostile.field];
      else candidate.authority[hostile.field] = hostile.value;
      if (validate(candidate)) throw new Error(`VCS hostile identity admitted: ${hostile.name}`);
    }
    const protocol = readFileSync(join(owner, "🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/📡️.protocol.semio"));
    const webDigest = Buffer.from(await crypto.subtle.digest("SHA-256", protocol)).toString("hex");
    if (webDigest !== fixture.snapshotProtocolSha256 || webDigest !== createHash("sha256").update(protocol).digest("hex")) throw new Error("VCS protocol identity differs from its pinned neutral vector");
    console.log(`vcs-native-openable-identity-oracle: positive=1 hostile-denied=${identities.size} protocol-webcrypto=1`);
    const root = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const artifact = readFileSync(join(owner, "🗿️artifacts/🌿️vcs/🦀️.rs"), "utf8");
    const manifest = readFileSync(join(this.root, "Cargo.toml"), "utf8");
    if (!root.includes('.package_id("semio:vcs")') || !manifest.includes('package = "semio:vcs"') || artifact.includes('"vcs.document"')) throw new Error("VCS guest package/artifact authority is not canonical");
    if (process.argv.includes("--oracle-only")) return;
    const receipts = await runExactCargoLaws({
      cwd: this.root,
      env: { ...process.env, RUST_MIN_STACK: "268435456" },
      groups: [{ package: "semio-s-plugin-vcs", target: { kind: "test", name: "native_openable_identity" }, laws: ["vcs_guest_descriptor_has_one_canonical_native_openable_identity"] }],
      progress(event) { console.log(`vcs-native-openable-identity ${event.stage}: artifacts=${event.artifactDir}`); },
    });
    console.log(`vcs-native-openable-identity-laws: ${JSON.stringify(receipts)}`);
  }
}
