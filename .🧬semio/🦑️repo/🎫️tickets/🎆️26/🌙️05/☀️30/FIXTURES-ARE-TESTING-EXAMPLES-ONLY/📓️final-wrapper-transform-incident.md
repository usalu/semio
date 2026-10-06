# Plugin and Hub Wrapper Refactor

## Modified Consumers

- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🟦️.ts`
- `🌎️hub/🔐️auth/🔌️client/🧪️tests/🧬️source/🟦️.ts`
- `🌎️hub/🧩️compositions/🌿️vcs/🧪️tests/🪪️native-openable-identity/🟦️.ts`
- `✏️s/🧑‍💻dev/📐️cad/🧪️tests/🌐️geometry-ownership/🟦️.ts`
- `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🧑‍💻dev/🌊️flow/🧪️tests/🏷️ownership/🟦️.ts`
- `✏️s/🧑‍💻dev/🧹️fixture-sweep/🧪️tests/🔬️ownership/🟦️.ts`
- `✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/📚️office-schema-contract/🟦️.ts`
- `🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust/📜️script.ts`
- `🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🔬️ownership/🟦️.ts`
- `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/🏷️ownership/🟦️.ts`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts`
- `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️packages/🦀️rust/📜️script.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📎️references/🧪️tests/🟦️.ts`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🧪️frame-selection/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🌳️ast/🧪️tests/📦️wire-types/🟦️.ts`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🧬️semantic-wire/🟦️.ts`

## Follow-Up Expressions

    🌎️hub/📦️packages/🦀️rust/📜️script.ts:5649 unhandled new Ajv({ strict: true }).compile<BrowserActorChildWorkerFixture>(schema)
    🌎️hub/🧩️compositions/🌿️vcs/🧪️tests/🪪️native-openable-identity/🟦️.ts:17 remaining ajv.addSchema(module);
    🌎️hub/🧩️compositions/🌿️vcs/🧪️tests/🪪️native-openable-identity/🟦️.ts:17 unhandled ajv.addSchema(module)
    🌎️hub/🧩️compositions/🌿️vcs/🧪️tests/🪪️native-openable-identity/🟦️.ts:19 unhandled validate(fixture.authority)
    🌎️hub/🧩️compositions/🌿️vcs/🧪️tests/🪪️native-openable-identity/🟦️.ts:19 unhandled validate.errors
    🌎️hub/🧩️compositions/🌿️vcs/🧪️tests/🪪️native-openable-identity/🟦️.ts:27 unhandled validate(candidate.authority)
    🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:1321 unhandled validate(law)
    🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:1321 unhandled validate.errors
    🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:1322 unhandled validate(invalid)
    ✏️s/🧑‍💻dev/🌊️flow/🧪️tests/🏷️ownership/🟦️.ts:11 unhandled new Ajv({ strict: true, allErrors: true }).addKeyword("x-semio-formats").addSchema(schema)
    ✏️s/🧑‍💻dev/🧹️fixture-sweep/🧪️tests/🔬️ownership/🟦️.ts:112 unhandled new Ajv({ strict: true, allErrors: true }).addKeyword("x-semio-formats").addSchema(schema)
    ✏️s/🧑‍💻dev/🧹️fixture-sweep/🧪️tests/🔬️ownership/🟦️.ts:113 unhandled schema.$id
    ✏️s/🧑‍💻dev/🗄️stdio/🧪️tests/📚️office-schema-contract/🟦️.ts:113 unhandled validators.entries
    🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust/📜️script.ts:402 remaining ajv.addSchema(module);
    🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust/📜️script.ts:402 unhandled ajv.addSchema(module)
    🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust/📜️script.ts:404 unhandled validate(fixture)
    🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust/📜️script.ts:404 unhandled validate.errors
    🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust/📜️script.ts:409 unhandled validate(missing)
    🌎️hub/🧩️compositions/📕️norm/📦️packages/🦀️rust/📜️script.ts:412 unhandled validate(wrongLayout)
    🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:28 remaining ajv.addSchema(module);
    🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:28 unhandled ajv.addSchema(module)
    🌎️hub/🧩️compositions/🪐️space/📦️packages/🦀️rust/📜️script.ts:29 unhandled module.$id
    ✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🔬️ownership/🟦️.ts:25 unhandled new Ajv({ strict: true, allErrors: true }).addKeyword("x-semio-formats").addSchema(schema)
    ✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🔬️ownership/🟦️.ts:26 unhandled schema.$id
    ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/🏷️ownership/🟦️.ts:11 unhandled new Ajv({ strict:true, allErrors:true }).addKeyword("x-semio-formats").addSchema(schema)
    ✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧪️tests/🏷️ownership/🟦️.ts:13 unhandled schema.$id
    ✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts:122 remaining ajv.addSchema(module);
    ✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts:124 remaining reject(validateOracle(fixture), `Ajv oracle rejected canonical fixture: ${ajv.errorsText(validateOracle.errors)}`);
    ✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts:135 remaining reject(!validateOracle(hostile), "Ajv oracle accepted hostile fixture");
    ✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts:122 unhandled ajv.addSchema(module)
    ✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts:124 unhandled validateOracle(fixture)
    ✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts:124 unhandled validateOracle.errors
    ✏️s/🔌️plugins/💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts:135 unhandled validateOracle(hostile)
    ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts:73 unhandled ajv.addSchema(schema)
    ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts:73 unhandled schema.$id
    ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts:92 unhandled ...storeSchema
    ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts:220 unhandled ...commitSchema
    ✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts:238 unhandled ...peerSchema
