# Expanded Admission Review

✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:196

const validate = await ownedExport(repoRoot, "os", "GisMapApprovalHistoryV1");

✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:249

const validate = await ownedExport(repoRoot, "os", "GisMapInferencePortV1");

✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:430

const validate = await ownedExport(repoRoot, "os", "GisMapPeerRebootstrapV1");

✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:197

if (!validate(fixture)) throw new Error(`invalid GIS Map approval history corpus: ${JSON.stringify(validate.errors)}`);

✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:198

if (validate({ ...fixture, cases: fixture.cases.slice(1) })) throw new Error("approval history schema admitted a missing lifecycle law");

✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:250

if (!validate(fixture)) throw new Error(`invalid GIS Map inference port corpus: ${JSON.stringify(validate.errors)}`);

✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:260

for (const [index, candidate] of hostileCorpora.entries()) if (validate(candidate)) throw new Error(`GIS Map inference port corpus accepted hostile mutation ${index}`);

✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:431

if (!validate(fixture)) throw new Error(`invalid GIS Map peer rebootstrap fixture: ${JSON.stringify(validate.errors)}`);

✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts:73

const validate = ajv.addSchema(schema).compile<CadPresenceRetirementFixture>({ $ref: `${schema.$id}#/$defs/CadPresenceRetirementLaws` });

✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts:74

if (!validate(fixture)) throw new Error(`CAD presence retirement schema: ${JSON.stringify(validate.errors)}`);

✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧪️tests/🔬️cad-presence-retirement/🟦️.ts:87

if (validate(hostile)) throw new Error("CAD presence schema accepted an enlarged production grant");

✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🟦️.ts:1554

const validate = semioSchemaAjvV1({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/GisMapPeerRebootstrapV1`)! as unknown as (value: unknown) => value is GisMapPeerRebootstrapFixtureV1;

✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🟦️.ts:2869

const valid = semioSchemaAjvV1({ strict: true }).compile({ $defs: schema.$defs, $ref: "#/$defs/GisMapInferencePortV1" });

✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🟦️.ts:3193

const validate = semioSchemaAjvV1({ strict: true }).compile({ $defs: schema.$defs, $ref: "#/$defs/GisMapInferencePortV1" });

✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🟦️.ts:4165

const validateScope = validators.getSchema(`${schema.$id}#/$defs/DocumentOpeningScopeResolutionV1`)!;

✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🟦️.ts:4166

const validateFirstOpen = validators.getSchema(`${schema.$id}#/$defs/DocumentFirstOpenV1`)!;

✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🟦️.ts:1555

expect(validate(parsed)).toBe(true);

✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🟦️.ts:1556

if (!validate(parsed)) throw new Error("GIS Map peer rebootstrap fixture is invalid");

✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🟦️.ts:2870

expect(valid(fixture), JSON.stringify(valid.errors)).toBe(true);

✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🟦️.ts:3194

expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);

✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🟦️.ts:4167

expect(corpus.cases.every((row: unknown) => validateScope(row)) && validateFirstOpen(corpus.firstOpen), JSON.stringify([...validateScope.errors ?? [], ...validateFirstOpen.errors ?? []])).toBe(true);

🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🟦️.ts:18

const validate = await compileGisScopeExport(repoRoot, GIS_SCHEMA_MODULE, "GisComponentColdMapPatch");

🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🟦️.ts:19

if (!validate(fixture)) throw new Error(`invalid GIS component cold-map corpus: ${JSON.stringify(validate.errors)}`);

🌎️hub/🧩️compositions/🌍️gis/🧪️tests/💡️inference-control/🟦️.ts:11

const validate = await compileGisScopeExport(repoRoot, GIS_SCHEMA_MODULE, "GisInferenceControl");

🌎️hub/🧩️compositions/🌍️gis/🧪️tests/💡️inference-control/🟦️.ts:12

if (!validate(fixture)) throw new Error(`invalid GIS controlled corpus: ${JSON.stringify(validate.errors)}`);

🌎️hub/🧩️compositions/🌍️gis/🧪️tests/💡️inference-discovery/🟦️.ts:24

const validateIdentity = gisContract(this.repoRoot, "GisArtifactIdentity");

🌎️hub/🧩️compositions/🌍️gis/🧪️tests/💡️inference-discovery/🟦️.ts:43

const validateControl = gisContract(this.repoRoot, "GisInferenceControl");

🌎️hub/🧩️compositions/🌍️gis/🧪️tests/💡️inference-discovery/🟦️.ts:25

if (!validateIdentity(identity)) throw new Error(`invalid GIS identity fixture: ${JSON.stringify(validateIdentity.errors)}`);

🌎️hub/🧩️compositions/🌍️gis/🧪️tests/💡️inference-discovery/🟦️.ts:38

if (validateIdentity(candidate)) throw new Error(`GIS identity oracle admitted ${kind}`);

🌎️hub/🧩️compositions/🌍️gis/🧪️tests/💡️inference-discovery/🟦️.ts:44

if (!validateControl(control)) throw new Error(`invalid GIS control fixture: ${JSON.stringify(validateControl.errors)}`);

🌎️hub/🧩️compositions/🌍️gis/🧪️tests/📇️native-codecs/🟦️.ts:16

const validate = await compileGisScopeExport(repoRoot, GIS_SCHEMA_MODULE, "GisNativeCodecs");

🌎️hub/🧩️compositions/🌍️gis/🧪️tests/📇️native-codecs/🟦️.ts:25

const validateDocumentIds = registryAjv.compile({ $ref: `${registryModule.$id}#/$defs/ArtifactDocumentIdV1` });

🌎️hub/🧩️compositions/🌍️gis/🧪️tests/📇️native-codecs/🟦️.ts:17

if (!validate(fixture)) throw new Error(`invalid GIS receipt corpus: ${JSON.stringify(validate.errors)}`);

🌎️hub/🧩️compositions/🌍️gis/🧪️tests/📇️native-codecs/🟦️.ts:26

if (!validateDocumentIds(documentIds)) throw new Error(`invalid artifact document-id corpus: ${JSON.stringify(validateDocumentIds.errors)}`);

🌎️hub/🧩️compositions/🌿️vcs/🧪️tests/📇️native-codecs/🟦️.ts:50

const validateDocumentIds = registryAjv.compile({ $ref: `${registryModule.$id}#/$defs/ArtifactDocumentIdV1` });

🌎️hub/🧩️compositions/🌿️vcs/🧪️tests/📇️native-codecs/🟦️.ts:51

if (!validateDocumentIds(documentIds)) throw new Error(`invalid artifact document-id corpus: ${JSON.stringify(validateDocumentIds.errors)}`);

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:12

const validate = retirementAjv.getSchema(`${schemaDocument.$id}#/$defs/FlowSessionRetirementV1`)!;

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:49

const validateClose = flowWasmContract("FlowRetainedSessionCloseV1");

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:62

const validateRuntime = flowWasmContract("FlowBrowserRuntimeLifetimeV1");

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:13

assert(validate(fixture), JSON.stringify(validate.errors));

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:44

for (const mutant of [{ ...fixture, extra: true }, { ...fixture, grants: [16384] }, { ...fixture, dag: { ...fixture.dag, minimumUtf8Bytes: 1600 } }, { ...fixture, scene: { retirementCapacity: 1025 } }, { ...fixture, expected: { ...fixture.expected, zeroGrant: "progress" } }]) assert(!validate(mutant));

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:50

assert(validateClose(sessionClose), "retained session close fixture must satisfy its owned contract");

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:52

for (const mutant of [
  { ...sessionClose, extra: true },
  { ...sessionClose, browser: { ...sessionClose.browser, terminalBeforeClose: true } },
  { ...sessionClose, close: { ...sessionClose.close, retainedBeforePoll: 0 } },
  { ...sessionClose, ordering: ["session-closed", "session-released", "domain-retired"] },
]) assert(!validateClose(mutant));

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:63

assert(validateRuntime(runtimeLifetime), "browser runtime lifetime fixture must satisfy its owned contract");

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧪️source-contract/🟦️.ts:65

for (const mutant of [{ ...runtimeLifetime, initialSessions: 1 }, { ...runtimeLifetime, extra: true }, { ...runtimeLifetime, afterCloseA: { ...runtimeLifetime.afterCloseA, globalCloseCalls: 1 } }, { ...runtimeLifetime, receipt: { ...runtimeLifetime.receipt, completion: "control-admitted" } }, { ...runtimeLifetime, openFailure: { ...runtimeLifetime.openFailure, uncertainTransport: { ...runtimeLifetime.openFailure.uncertainTransport, terminal: false } } }]) assert(!validateRuntime(mutant));

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/🔬️flow-selected-copy/🟦️.ts:15

const validate = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/FlowTypedCopyV1" });

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/🔬️flow-selected-copy/🟦️.ts:16

if (!validate(fixture)) throw new Error("Flow selected copy strict schema failed");

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/📑️copy/🧪️tests/🔬️flow-selected-copy/🟦️.ts:19

for (const value of hostiles) if (validate(value)) throw new Error("Flow selected copy accepted hostile schema");

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🧪️tests/🔬️flow-typed-retirement/🟦️.ts:13

const validate = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/FlowRetirementV1" });

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🧪️tests/🔬️flow-typed-retirement/🟦️.ts:14

if (!validate(fixture)) throw new Error("Flow retirement strict fixture schema failed");

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🧪️tests/🔬️flow-typed-retirement/🟦️.ts:17

for (const value of hostiles) if (validate(value)) throw new Error("Flow retirement schema accepted hostile payload");

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/🌐️browser-declaration/🟦️.ts:18

const validate = flowWasmContract("FlowBrowserTypesV1");

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/🌐️browser-declaration/🟦️.ts:19

assert.equal(validate(fixture), true);

🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/🌐️browser-declaration/🟦️.ts:53

assert.equal(validate(bad), false);

🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧬️generation/🧪️tests/🔬️procedural-generation-root/🟦️.ts:12

const validate = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/GenerationRootV1" });

🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧬️generation/🧪️tests/🔬️procedural-generation-root/🟦️.ts:13

if (!validate(fixture)) throw new Error("generation root fixture failed strict schema");

🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧬️generation/🧪️tests/🔬️procedural-generation-root/🟦️.ts:15

for (const value of hostiles) if (validate(value)) throw new Error("generation root schema accepted hostile input");

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚡️quick/🟦️.ts:64

const identity = ajv.getSchema(`${rendererSchema.$id}#/$defs/HostIdentityResolutionV1`)!;

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/⚡️quick/🟦️.ts:66

expect(identity(hostBootstrapFixture.identity), JSON.stringify(identity.errors)).toBe(true);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:3071

const validate = ajv.getSchema(`${rendererSchema.$id}#/$defs/TutorialInteractionCaptureV1`)!;

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:5206

const validate = peerExport(flowWasmSchema, "FlowBrowserRuntimeLifetimeV1");

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:12541

const validate = rendererExport("ShellLabelResolutionV1");

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:3072

expect(validate(tutorialInteractionFixture), JSON.stringify(validate.errors)).toBe(true);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:3073

expect(
      validate({
        ...tutorialInteractionFixture,
        observed: {
          ...tutorialInteractionFixture.observed,
          selection: { mesh: { ...tutorialInteractionFixture.observed.selection.mesh, ids: [7] } },
        },
      }),
    ).toBe(false);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:5207

expect(validate(flowBrowserRuntimeFixture), JSON.stringify(validate.errors)).toBe(true);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:12542

expect(validate(labelResolutionFixture), JSON.stringify(validate.errors)).toBe(true);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts:12560

expect(validate({ ...labelResolutionFixture, matrix: { native: { en: "Only English" }, reuse: matrix.reuse } })).toBe(false);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🚪️opening/🟦️.ts:22

const validate = rendererExport("OpenArtifactRelayV1");

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🚪️opening/🟦️.ts:46

const resolution = rendererExport("DocumentOpeningScopeResolutionV1");

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🚪️opening/🟦️.ts:68

const validate = rendererExport("SharedDocumentOpeningAccessV1");

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🚪️opening/🟦️.ts:23

expect(validate(artifactOpeningFixture)).toBe(true);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🚪️opening/🟦️.ts:47

expect(openingScopeFixture.cases.every((row) => resolution(row)) && rendererExport("DocumentFirstOpenV1")(openingScopeFixture.firstOpen)).toBe(true);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🚪️opening/🟦️.ts:70

expect(validate(row), `${row.id}: ${JSON.stringify(validate.errors)}`).toBe(true);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:163

export const parseAdmittedShellInstanceTransitionV1 = (value: unknown): AdmittedShellInstanceTransitionV1 => parseRendererExport("AdmittedShellInstanceTransitionV1", value);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:164

export const parseDocumentOpeningTransitionV1 = (value: unknown): DocumentOpeningTransitionV1 => parseRendererExport("DocumentOpeningTransitionV1", value);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:165

export const parseDocumentOpeningAdmissionV1 = (value: unknown): DocumentOpeningAdmissionV1 => parseRendererExport("DocumentOpeningAdmissionV1", value);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:166

export const parseDocumentOpeningBackgroundSequenceV1 = (value: unknown): DocumentOpeningBackgroundSequenceV1 => parseRendererExport("DocumentOpeningBackgroundSequenceV1", value);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:167

export const parseDocumentOpeningCloseFailureV1 = (value: unknown): DocumentOpeningCloseFailureV1 => parseRendererExport("DocumentOpeningCloseFailureV1", value);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:168

export const parseDocumentOpeningScopeResolutionV1 = (value: unknown): DocumentOpeningScopeResolutionV1 => parseRendererExport("DocumentOpeningScopeResolutionV1", value);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:169

export const parseDocumentFirstOpenV1 = (value: unknown): DocumentFirstOpenV1 => parseRendererExport("DocumentFirstOpenV1", value);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:170

export const parseTutorialRunTransitionV1 = (value: unknown): TutorialRunTransitionV1 => parseRendererExport("TutorialRunTransitionV1", value);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:171

export const parseTutorialDriveTransitionV1 = (value: unknown): TutorialDriveTransitionV1 => parseRendererExport("TutorialDriveTransitionV1", value);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:172

export const parsePausedTutorialSeekTransitionV1 = (value: unknown): PausedTutorialSeekTransitionV1 => parseRendererExport("PausedTutorialSeekTransitionV1", value);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:173

export const parseSerialTutorialDriveTransitionV1 = (value: unknown): SerialTutorialDriveTransitionV1 => parseRendererExport("SerialTutorialDriveTransitionV1", value);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🟦️.ts:174

export const parseHostIdentityResolutionV1 = (value: unknown): HostIdentityResolutionV1 => parseRendererExport("HostIdentityResolutionV1", value);

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts:974

const validate = await registrySchemaValidator("NativeCatalogSelectionV1");

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts:975

if (!validate(fixture)) throw new Error(`native catalog selection fixture denied: ${JSON.stringify(validate.errors)}`);

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts:730

const validate = await registrySchemaValidator("RustTaxonomyMountsV1");

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts:796

const validate = await registrySchemaValidator("PluginRootOwnershipV1");

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts:731

if (!validate(fixture)) throw new Error(JSON.stringify(validate.errors));

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🗿️taxonomy-validation/🟦️.ts:797

if (!validate(fixture)) throw new Error(JSON.stringify(validate.errors));

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts:309

const validate = moduleExportValidator(new Ajv({ strict: true, allErrors: true }), join(owner, "🧬️schema", "🔣️.json"), "NativeUiPatchMarshallingV1");

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts:310

assert(validate(fixture), JSON.stringify(validate.errors));

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/♻️relay-lifecycle/🟦️.ts:19

const validate = ajv.getSchema(`${schema.$id}#/$defs/RelayLifecycleV1`)!;

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/♻️relay-lifecycle/🟦️.ts:20

if (!validate(fixture)) throw new Error(`relay lifecycle fixture schema violation: ${JSON.stringify(validate.errors)}`);

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-factory-proof-join/🟦️.ts:13

const validate = ajv.compile({ ...schema, $ref: "#/$defs/ToolFactoryProofV1" });

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-factory-proof-join/🟦️.ts:14

if (!validate(fixture)) throw new Error(`factory runtime fixture schema: ${JSON.stringify(validate.errors)}`);

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts:13

const validate = ajv.compile({ $ref: schema.$id + "#/$defs/ToolLatestWinsV1" });

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts:30

const validateIntegration = ajv.compile({ $ref: schema.$id + "#/$defs/ToolLatestWinsIntegrationV1" });

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts:91

const validateRaw = ajv.compile({ $ref: schema.$id + "#/$defs/RawAllocationCloseV1" });

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts:104

const validateChildClose = ajv.compile({ $ref: schema.$id + "#/$defs/ChildPrepublicationCloseV1" });

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts:147

const validateDispatch = ajv.compile({ $ref: schema.$id + "#/$defs/MountedDispatchBindingV1" });

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts:14

if (!validate(fixture)) throw new Error(`latest-wins schema: ${JSON.stringify(validate.errors)}`);

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts:28

for (const hostile of hostiles) if (validate(hostile)) throw new Error("latest-wins schema accepted a forged scope or enlarged grant");

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts:31

if (!validateIntegration(integration)) throw new Error(`latest-wins integration schema: ${JSON.stringify(validateIntegration.errors)}`);

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts:47

for (const hostile of integrationHostiles) if (validateIntegration(hostile)) throw new Error("latest-wins integration schema accepted stale authority, collision, starvation, or a missing accepted target");

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts:92

if (!validateRaw(rawFixture)) throw new Error(`retained raw allocation schema: ${JSON.stringify(validateRaw.errors)}`);

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts:105

if (!validateChildClose(childCloseFixture)) throw new Error(`retained child close fixture: ${JSON.stringify(validateChildClose.errors)}`);

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-latest-wins/🟦️.ts:148

if (!validateDispatch(dispatchFixture)) throw new Error(`mounted dispatch fixture: ${JSON.stringify(validateDispatch.errors)}`);

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️reactor-contract-oracles/🟦️.ts:73

const validateReceipt = ajv.getSchema<IssuedPatchFixture>(`${schema.$id}#/$defs/PendingPatchReceiptV1`)!;

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️reactor-contract-oracles/🟦️.ts:74

assert(validateReceipt(fixture), JSON.stringify(validateReceipt.errors));

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-checkpoint/🟦️.ts:16

const validate = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/ArtifactCommandCheckpointV1" });

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-checkpoint/🟦️.ts:17

if (!validate(fixture)) throw new Error(`[verify interactivity tool-jobs] native checkpoint fixture/schema mismatch: ${JSON.stringify(validate.errors)}`);

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-checkpoint/🟦️.ts:46

if (validate(value)) throw new Error(`[verify interactivity tool-jobs] checkpoint schema admitted ${name}`);

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-owner-factory-resolution/🟦️.ts:12

const validate = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/OwnerFactoryResolutionV1" });

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-owner-factory-resolution/🟦️.ts:13

if (!validate(fixture)) throw new Error(`owner factory fixture schema: ${JSON.stringify(validate.errors)}`);

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-owner-factory-resolution/🟦️.ts:17

if (validate(hostile)) throw new Error("owner factory strict schema accepted an adversarial fixture");

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-scalar-config-cohort/🟦️.ts:15

const validate = new Ajv({ strict: true, allErrors: true }).compile({ ...schema, $ref: "#/$defs/ScalarConfigCohortV1" });

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-scalar-config-cohort/🟦️.ts:16

if (!validate(fixture)) throw new Error(`scalar Config fixture schema: ${JSON.stringify(validate.errors)}`);

🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🔬️tool-job-scalar-config-cohort/🟦️.ts:65

if (validate(hostile)) throw new Error("scalar Config strict schema accepted a hostile fixture");

🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🛂️mutation-source-authority/🟦️.ts:14

const validate = ajv.getSchema(`${document.$id}#/$defs/MutationSourceAuthorityV1`)!;

🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🛂️mutation-source-authority/🟦️.ts:15

assert(validate(fixture), JSON.stringify(validate.errors));

🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🛂️mutation-source-authority/🟦️.ts:31

for (const item of hostile) assert(!validate(item));

🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts:15

const validate = contract("ValueRetirementV1");

🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts:35

const validateCache = contract("CacheRetirementV1");

🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts:60

const validateEvaluation = contract("EvaluationOwnersV1");

🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts:16

assert(validate(fixture), JSON.stringify(validate.errors));

🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts:29

assert(!validate(mutant));

🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts:36

assert(validateCache(cacheFixture), JSON.stringify(validateCache.errors));

🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts:55

for (const mutant of [{ ...cacheFixture, extra: true }, { ...cacheFixture, expected: { ...cacheFixture.expected, sharedReleasedBytes: 1 } }, { ...cacheFixture, operations: [{ op: "erase" }, ...cacheFixture.operations.slice(1)] }]) assert(!validateCache(mutant));

🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts:61

assert(validateEvaluation(evaluation), JSON.stringify(validateEvaluation.errors));

🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🧵️retirement/🧪️tests/🧪️source-contract/🟦️.ts:65

for (const mutant of [{ ...evaluation, extra: true }, { ...evaluation, expectedBytes: 0 }]) assert(!validateEvaluation(mutant));

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️document-opening/🟦️.ts:191

const transition = ajv.getSchema(`${rendererSchema.$id}#/$defs/DocumentOpeningTransitionV1`)!;

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️document-opening/🟦️.ts:192

const admission = ajv.getSchema(`${rendererSchema.$id}#/$defs/DocumentOpeningAdmissionV1`)!;

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️document-opening/🟦️.ts:193

const closeFailure = ajv.getSchema(`${rendererSchema.$id}#/$defs/DocumentOpeningCloseFailureV1`)!;

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️document-opening/🟦️.ts:194

const background = ajv.getSchema(`${rendererSchema.$id}#/$defs/DocumentOpeningBackgroundSequenceV1`)!;

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️document-opening/🟦️.ts:195

expect(documentOpeningFixture.cases.every((row) => transition(row))).toBe(true);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️document-opening/🟦️.ts:196

expect(documentOpeningFixture.admissions.every((row) => admission(row))).toBe(true);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️document-opening/🟦️.ts:197

expect(documentOpeningFixture.closeFailures.every((row) => closeFailure(row))).toBe(true);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️document-opening/🟦️.ts:198

expect(Object.values(documentOpeningFixture.backgroundSessions).every((row) => background(row))).toBe(true);

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts:289

const validate = semioSchemaAjvV1({ strict: true }).addSchema(rendererSchema).getSchema(`${rendererSchema.$id}#/$defs/PluginRuntimeLifecycleSchedulerV1`)!;

🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️package-integration/🟦️.ts:290

expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);

🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/📤️macro-exports/🟦️.ts:10

const validate = ajv.getSchema(`${document.$id}#/$defs/MacroExportsV1`)!;

🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/📤️macro-exports/🟦️.ts:11

assert(validate(fixture), JSON.stringify(validate.errors));

🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/📤️macro-exports/🟦️.ts:22

for (const mutant of [{ ...fixture, extra: true }, { ...fixture, facadeExports: ["DslRecord", "DslRecord"] }, { ...fixture, traitOnly: ["invalid-name"] }]) assert(!validate(mutant));

## Unhandled

🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🌉️component-cold-map-patch/🟦️.ts:25 remaining validate(candidate)
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts:978 remaining validate(candidate)
