# Plugin and Hub Wrapper Refactor

## Modified Consumers

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission-io/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧾️cargo-provenance/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️canonical-go-discovery/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📸️source-index-capture/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️workspace-publication-source/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️canonical-json/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧰️framework-root-source-topology/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌳️kind-only-basename/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎫️ticket-role-routing/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥾️cross-platform-bootstrap/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️mutation-leaf-identity/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️source-file-facts/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️cargo-transaction-command-source/🟦️.ts`

## Follow-Up Expressions

    🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-caps/🟦️.ts:20 remaining ajv.addSchema(schema);
    🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-caps/🟦️.ts:12 unhandled readonly schema: Record<string, unknown>
    🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-caps/🟦️.ts:18 unhandled schema = read("../../🧬️schema/🔣️mutation-caps/🔣️.json") as Record<string, unknown> & { readonly $id: string }
    🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-caps/🟦️.ts:20 unhandled ajv.addSchema(schema)
    🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-caps/🟦️.ts:41 unhandled schema.$id
    🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-input-declarations/🟦️.ts:36 remaining ajv.addSchema(schema);
    🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-input-declarations/🟦️.ts:19 unhandled readonly schema: Json;
    🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-input-declarations/🟦️.ts:26 unhandled schema = read("🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧬️schema/🔣️mutation-input-declarations/🔣️.json") as Json & { readonly $id: string }
    🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-input-declarations/🟦️.ts:36 unhandled ajv.addSchema(schema)
    🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-input-declarations/🟦️.ts:103 unhandled schema.$id
    🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-input-declarations/🟦️.ts:105 unhandled schema.$id
    🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🧪️tests/🧪️mutation-input-declarations/🟦️.ts:109 unhandled schema.$id
    🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️dependency-direction/🟦️.ts:22 unhandled schema = read("🧬️schema/🧱️dependency-direction/🔣️.json")
    🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📁️runtime/🧪️tests/🏘️execution/🟦️.ts:10 unhandled schema = JSON.parse(readFileSync(new URL("../../../../../../🧬️schema/🧱️rust-source-direction/🔣️.json", import.meta.url), "utf8"))
    🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📁️runtime/🧪️tests/🏘️execution/🟦️.ts:11 unhandled schema.$schema
    🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🦀️source/📁️runtime/🧪️tests/🏘️execution/🟦️.ts:11 unhandled schema.$defs
    🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️cargo-dependency-direction/🟦️.ts:12 unhandled schema = read("🧬️schema/🧱️cargo-dependency-direction/🔣️.json")
    🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️cargo-dependency-direction/🟦️.ts:88 unhandled schema.$defs
    🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🟦️.ts:32 unhandled schema = JSON.parse(asset("../../🧬️schema/📣️typescript-declaration-facts/🔣️.json"))
    🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🟦️.ts:36 unhandled schema.$defs
    🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🟦️.ts:101 unhandled malformedSchema = JSON.parse(asset("../../🧬️schema/📣️typescript-declaration-facts/💥️malformed/🔣️.json"))
    🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎭️source-roster-roles/🟦️.ts:11 unhandled vectorsSchemaPath = resolve(import.meta.dir, "../../🧬️schema/📋️mutation-inventory/🎭️source-roster-roles/🔣️.json")
