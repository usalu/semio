# Root Script Region Ownership Map

Root parsed the current root 📜️script.ts with the installed TypeScript AST on 2026-09-12 and mapped top-level function declarations to source regions. This is an execution planning map, not semantic acceptance. Region boundaries are editorial: the SourceFileFacts region also contains cleanup helpers, demonstrating why extraction must follow lexical dependencies and actual responsibilities rather than blindly cutting regions. Anonymous implementation leaves may own coherent domain APIs; one filename per function is not required.

The WGPU/JCO and dependency clusters already have the concrete 📓️root-artifact-dependency-extraction-packet-2026-09-12.md. The table below retains the later normalization, source evidence, cleanup, authoring, graph export and policy domains. Earlier interactivity/job proofs already import domain-owned test registrations; their remaining actual behavior should move alongside the appropriate runtime/verification concern with consumer closure.

## Bounded Execution Groups

1. Taxonomy inventory shard contract, digest, validation and publication belong to normalization/inventory/output concerns. Keep canonical byte streaming, cancellation and staged shard publication independent from command argument/log formatting.
2. Mutation source admission/facts, inventory/plan/verify and structural reachability belong to their actual mutation/normalization domains. The owned JSON Schema subset validator is reusable schema behavior and must not remain hidden in a command file or be replaced by an external runtime dependency.
3. Workspace cleanup/ticket protection and scaffold authoring require distinct lifecycle/ownership domains. Preserve closed-ticket protection, no-follow checks, exact generated-output ownership, cancellation and rollback. This extraction does not authorize executing destructive cleanup or modifying ticket inputs.
4. Neo4j graph names/CLI partitioning and export implementation belong to graph export concerns; preserve read-only rendering/output protocol and native credentials. Do not execute remote publication merely to validate a move.
5. Policy source parsing/discovery and individual repository laws must split by actual contract family. Reuse existing Rust/source parsing infrastructure where semantics coincide. Generic evidence parsing belongs in repository discovery; artifact/mutation/schema/OS state/stdio laws remain in explicit neutral law concerns. Do not create another thirteen-thousand-line catch-all helper.
6. Source-as-data consumers must follow the moved function owner. Existing declaration-only router/class exports can remain only when they are actual routing contracts; public domain APIs must move and all direct/dynamic consumers must import their neutral owners.

## Current Source Region Map

Line coordinates are the inspected snapshot, not permanent authority. The line count includes all text between adjacent region openers. Function counts include top-level declarations only, excluding class methods, arrows and embedded source strings. Re-read live AST before every edit.

| Region | Start Line | Lines | Top-Level Functions | Representative APIs |
| --- | ---: | ---: | ---: | --- |
| 📊️TaxonomyInventoryShards | 15491 | 490 | 26 | `taxonomyCliRecord`, `taxonomyCliExactKeys`, `taxonomyCliSha256`, `taxonomyCliCanonicalJson`, `taxonomyCliCanonicalArrayDigest` … |
| 🧬️MutationTaxonomyWorkflow | 15981 | 196 | 11 | `mutationTaxonomySegmentAfter`, `mutationTaxonomyLocations`, `mutationTaxonomyCompare`, `mutationTaxonomyCapturedSchema`, `mutationTaxonomyStructuralDirectories` … |
| 🧬️SourceFileFacts | 16177 | 591 | 32 | `mutationTaxonomySourceFileFacts`, `mutationTaxonomyAssignmentLedger`, `mutationTaxonomySourceIndex`, `mutationTaxonomyLeafAlias`, `mutationTaxonomyFileAlias` … |
| 🛡️TicketProtection | 16768 | 481 | 23 | `cleanIntersectsProtected`, `cleanTicketManifestIsClosed`, `cleanIsTicketFolderBoundary`, `cleanTicketFolderForPath`, `cleanRemovalProtection` … |
| 🔖️MicroCommitScript | 17249 | 9 | 0 |  |
| 🔖️CommitScript | 17258 | 9 | 0 |  |
| 🔖️OsScript | 17267 | 60 | 2 | `pluginWasmArtifactExists`, `missingPluginWasmArtifacts` |
| 🔖️SemioScript | 17327 | 16 | 0 |  |
| 🔖️ExamplesScript | 17343 | 124 | 0 |  |
| 🔖️CleanMechanismNewScript | 17467 | 129 | 9 | `newScaffoldRustLeaf`, `newScaffoldTsLeaf`, `newScaffoldEmptyFacetMarkdown`, `newScaffoldWriteIfAbsent`, `newResolveChildDir` … |
| 🧬️MutationScaffolding | 17596 | 332 | 12 | `newMutationSemanticParts`, `newMutationRustLeaf`, `newMutationDescriptor`, `newMutationScaffoldLstat`, `newMutationScaffoldPath` … |
| 🔖️SchemaScript | 17928 | 283 | 0 |  |
| 🔖️Dispatch | 18211 | 43 | 0 |  |
| 🔖️generate-neo4j-gen | 18254 | 222 | 6 | `joinNeo4jGraphDatabaseName`, `defaultNeo4jGraphDatabaseName`, `parseExtraNeo4jGraphDatabaseNamesFromEnv`, `getAllNeo4jGraphExportSpecs`, `neo4jExportDatabaseNameSet` … |
| 🔖️Policy | 18476 | 10 | 0 |  |
| 🔧️PolicyFsScan | 18486 | 221 | 10 | `policyStripEmoji`, `policyCanonicalComponent`, `policyPluginOwnerDirs`, `policyScopeKey`, `policyAppIdFromCrateDir` … |
| 🔧️PolicyRegionParsing | 18707 | 100 | 7 | `policyParseRegionEvents`, `policyPairRegionSpans`, `policyMaskLiterals`, `policyParseModSpans`, `policyModAtLine` … |
| 🔧️PolicyFnParsing | 18807 | 28 | 3 | `policyExtractFnBody`, `policyLineOfIndex`, `policyPascalAppStructName` |
| 🔧️PolicyAllowlists | 18835 | 257 | 0 |  |
| 🔧️PolicyRuleRegionFormat | 19092 | 69 | 1 | `policyRegionFormatBreaches` |
| 🔧️PolicyRuleManifestRegion | 19161 | 28 | 1 | `policyManifestRegionBreaches` |
| 🔧️PolicyRuleStructNaming | 19189 | 35 | 1 | `policyStructNamingBreaches` |
| 🔧️PolicyRuleModLayout | 19224 | 29 | 1 | `policyModLayoutBreaches` |
| 🔧️PolicyRuleSdkMechanisms | 19253 | 141 | 5 | `policyResolveImportAlias`, `policySelectionIdsBreaches`, `policyArtifactAppLawDelegateBreaches`, `policyTreeItemBreaches`, `policyLabelsStructBreaches` |
| 🔧️PolicyRuleCargoArtifacts | 19394 | 26 | 1 | `policyCargoArtifactBreaches` |
| 🔧️PolicyRuleAppCoupling | 19420 | 63 | 1 | `policyAppCouplingBreaches` |
| 🔧️PolicyRuleNoJsonFixtures | 19483 | 46 | 2 | `policyDiscoverExampleJsonFiles`, `policyJsonFixtureBreaches` |
| 🔧️PolicyRuleOpsGrammar | 19529 | 78 | 2 | `policyDiscoverOpsFiles`, `policyOpsGrammarBreaches` |
| 🔧️PolicyRuleDslCompleteness | 19607 | 158 | 6 | `policyAllRustFiles`, `policyDocumentAppUsages`, `policyDslCompleteTypeNames`, `policyTypeAliasMap`, `policyResolveAlias` … |
| 🔧️PolicyRulePackCompleteness | 19765 | 35 | 1 | `policyPackCompletenessBreaches` |
| 🔧️PolicyRuleCommandEnvelopeCompleteness | 19800 | 32 | 1 | `policyCommandEnvelopeCompletenessBreaches` |
| 🔧️PolicyRuleDiffCompleteness | 19832 | 36 | 1 | `policyDiffCompletenessBreaches` |
| 🔧️PolicyRuleGrammarFileCompleteness | 19868 | 104 | 5 | `policyGrammarFileBreaches`, `policyProtocolFileBreaches`, `policyIsConstitutionalTsFacadePath`, `policyTsFacadeIsScaffoldStub`, `policyTsFacadeBreaches` |
| 🔧️PolicyRuleProtocolMigration | 19972 | 90 | 1 | `policyProtocolMigrationBreaches` |
| 🔧️PolicyRuleDbServerOnly | 20062 | 68 | 3 | `policyDiscoverCargoTomlFiles`, `policyDbAllowedDir`, `policyDbServerOnlyBreaches` |
| 🔧️PolicyRuleOsStateAuthority | 20130 | 217 | 5 | `policyOsStateAuthorityPathInScope`, `policyLineInTestMod`, `policyOsStateAuthorityBreaches`, `policyStructDeclaresFields`, `policyDocumentAppShapeBreaches` |
| 🔧️PolicyRuleNoPackFiles | 20347 | 50 | 2 | `policyDiscoverPackFiles`, `policyNoPackFilesBreaches` |
| 🔧️PolicyRuleNoRawSpawn | 20397 | 115 | 3 | `policyStripTsCommentsAndStrings`, `policyDiscoverScriptTsFiles`, `policyRawSpawnBreaches` |
| 🔧️PolicyRuleNoBudgetNull | 20512 | 33 | 1 | `policyBudgetNullBreaches` |
| 🔧️PolicyRuleMcpConfig | 20545 | 60 | 4 | `policyMcpRepoServerUsesBootstrap`, `policyMcpRepoServerFromJson`, `policyMcpRepoServerFromToml`, `policyMcpConfigBreaches` |
| 🔧️PolicyRuleTaxonomy | 20605 | 1218 | 28 | `policyReaddirSafe`, `policyReadFileSafe`, `policyRustProductionSource`, `policyRustTestSource`, `policyReadRustSourceEvidence` … |
| 🔧️PolicyRuleArtifactsOnlyPluginArchitecture | 21823 | 14 | 0 |  |
| 🔧️PolicyRulePluginClosedShape | 21837 | 147 | 1 | `policyPluginClosedShapeBreaches` |
| 🔧️PolicyRulePluginPurity | 21984 | 79 | 2 | `policyPluginPurityTestFnSpans`, `policyPluginPurityTsFiles` |
| 🔖️StateLaneExhaustiveness | 22063 | 236 | 2 | `policyStateLaneExhaustivenessBreaches`, `policyPluginPurityBreaches` |
| 🔧️PolicyRuleDeclarativeRegistration | 22299 | 130 | 3 | `policyRegistrationIsEngineSite`, `policyRegistrationBreach`, `policyDeclarativeRegistrationBreaches` |
| 🔧️PolicyRulePluginDependencyAllowlist | 22429 | 117 | 2 | `policyPluginOwnerFromApaPath`, `policyPluginDependencyAllowlistBreaches` |
| 🔧️PolicyRuleEffectCapabilityParity | 22546 | 119 | 1 | `policyEffectCapabilityParityBreaches` |
| 🔧️PolicyRuleApaRatchet | 22665 | 96 | 3 | `policyApaRatchetKey`, `policyApaRatchetApply`, `policyApaBreaches` |
| 🔧️PolicyRuleSubsetConformance | 22761 | 241 | 9 | `policyListTopLevelSubsetDirs`, `policySubsetFacetTotalityBreaches`, `policyArtifactEngineFacetForbiddenBreaches`, `policyArtifactEngineOwnerDirs`, `policyExampleNotAtArtifactLevelBreaches` … |
| 🔧️PolicyRuleArtifactViewersEditors | 23002 | 256 | 4 | `policySubsetSurfaceCompletenessBreaches`, `policyViewerPurityBreaches`, `policyContributedSurfaceTargetBreaches`, `policyOsConfigShapeBreaches` |
| 🔧️PolicyRuleHandcraftedSpecP3 | 23258 | 450 | 16 | `policyNormalizeSpecContent`, `policyHashSpecContent`, `policyWalkRelFiles`, `policyDiscoverGrammarAndProtocolSpecs`, `policySpecDistinctnessBreaches` … |
| 🔧️PolicyRuleMutationArtifactEngines | 23708 | 580 | 29 | `policyLeadingEmojiPrefix`, `policyListMutationDirs`, `policyStructuralMutationChildren`, `policyStructuralMutationDirs`, `policyMutationSemanticIdentity` … |
| 🧬️DirectMutationPolicies | 24288 | 41 | 0 |  |
| 🔣️JsonSchemaSubset | 24329 | 444 | 17 | `jsonSchemaSubsetObject`, `jsonSchemaSubsetValueEquals`, `jsonSchemaSubsetTypeMatches`, `jsonSchemaSubsetErrors`, `validateJsonSchemaSubset` … |
| 🔧️PolicyRuleMutationOutcomeMergePolicy | 24773 | 343 | 10 | `policyIsCompositeMutationDir`, `policyMutationOutcomeBreaches`, `policyMutationMessageCodeBreaches`, `policyNoCrdtVocabularyBreaches`, `policyNoValidateOverrideBreaches` … |
| 🔧️PolicyRuleInferenceFamily | 25116 | 346 | 11 | `policyListInferenceDirs`, `policyFindAllInferencesDirs`, `policyArtifactRootOfInferencesDir`, `policyInferenceFamilyRootCompletenessBreaches`, `policyInferenceSlugLeafPresenceBreaches` … |
| 🔧️PolicyRuleArtifactSchemas | 25462 | 829 | 23 | `policyDiscoverArtifactSchemaOwners`, `policyCanonicalState`, `policyCanonicalScalar`, `policyDeclaredSchemaExportName`, `policyFindSchemaDeclaration` … |
| 🔧️PolicyRuleAppSchemas | 26291 | 649 | 19 | `policyAppPresenceTypeName`, `policyDiscoverAppSchemaOwners`, `policyLoadAppSchemaFacetLeaves`, `policyAppSchemaFacetRole`, `policyAppSchemaFacetCompletenessBreaches` … |
| 🔧️PolicyRuleArtifactIo | 26940 | 352 | 13 | `policyLoadStdioOwnerTable`, `policyStdioArtifactFacets`, `policyStdioSchemaChildDirs`, `policyStdioRepresentationDirs`, `policyStdioFormatDir` … |
| 🏅️PolicyRuleStandardsSubsets | 27292 | 1253 | 26 | `policyListArtifactDialectDirs`, `policyStandardsCoverageBreaches`, `policyDerivedArtifactFacetBreaches`, `policyArtifactBuilderMigratedBreaches`, `policyArtifactAnalyzerBreaches` … |
| 🔧️PolicyRuleSniffReality | 28545 | 112 | 1 | `policySniffRealityBreaches` |
| 🔧️PolicyRuleSchemaOverhaulS2 | 28657 | 659 | 10 | `policyStdioVcsMachineryBanBreaches`, `policyListStdioStandardEntries`, `policyListStdioSchemaOwningEntries`, `policyDiffAlgebraBreaches`, `policyStdioStandardKey` … |
| 🔧️PolicyFacetMirrorDriftReverse | 29316 | 312 | 9 | `policyFacetRustTagFieldNames`, `policyFacetRustVariantFieldNames`, `policyFacetTsFieldNames`, `policyFacetGraphqlFieldNames`, `policyFacetJsonFieldNames` … |
| 🔧️PolicyRuleSchemaOverhaulPC | 29628 | 507 | 9 | `policyLooksLikeRealGrammarOrProtocolDialect`, `policyGrammarParseabilityBreaches`, `policyProtocolParseabilityBreaches`, `policyStdioArtifactKey`, `policyFixtureHonestyBreaches` … |
| 🔧️PolicyRuleDissolvedKernels | 30135 | 122 | 4 | `policyDissolvedRepEscapeBreaches`, `policyDissolvedEngineCacheScopeBreaches`, `policyDissolvedWholeDocumentReplaceBreaches`, `policyDissolvedKernelsBreaches` |
| 🔧️PolicyRuleComposition | 30257 | 170 | 6 | `policyIsCanonicalArtifactKind`, `policyMatchIsCommentedOut`, `policyCanonicalArtifactKindBreaches`, `policyCanonicalChildKindBreaches`, `policyDissolvedKindRedefinitionBreaches` … |
| 🔧️PolicyRuleCleanMechanism | 30427 | 12 | 0 |  |
| 🔖️ModulePathSlug | 30439 | 25 | 3 | `policyModulePathSlug`, `policyStandardModulePathSlug`, `policyEscapeRegExp` |
| 🔖️OwnerDiscovery | 30464 | 87 | 7 | `policySplitSubsetRel`, `policyAllSubsetSplits`, `policyListArtifactRels`, `policyListStandardRels`, `policyListPluginRels` … |
| 🔖️Policy1-OwnerMountsChildren | 30551 | 110 | 3 | `policyExtractPathMountTargets`, `policyExistingLegalChildren`, `policyOwnerMountsChildrenBreaches` |
| 🔖️Policy2-SubsetIsolation | 30661 | 77 | 1 | `policySubsetIsolationBreaches` |
| 🔖️Policy3-ModuleConsumerCount | 30738 | 51 | 1 | `policyModuleConsumerCountBreaches` |
| 🔖️Policy4-IoExclusivity | 30789 | 78 | 2 | `policyStripCfgTestModules`, `policyIoExclusivityBreaches` |
| 🔖️Policy5-IoDeclaration | 30867 | 45 | 1 | `policyIoDeclarationBreaches` |
| 🔖️Policy6-SubsetStandalone | 30912 | 45 | 1 | `policySubsetStandaloneBreaches` |
| 🔖️Policy7-DeclarationTree | 30957 | 113 | 3 | `policyDeclarationTreeBreaches`, `policyCleanArtifactStandardSubsetMechanismBreaches`, `policyPackageLanguagePurityBreaches` |
| 🔖️PolicyExport | 31070 | 296 | 0 |  |

## Validation

Start each bounded extraction with portable schema/source/consumer contracts and retain existing native behavior vectors. Close all root command imports and test dependency injection seams without compatibility facades or source-body SHA snapshots. Existing output/protocol hashes remain normative where their contract says so. Register exact semantic ancestry and run focused direct and Nx/native checks; retain genuine observed law failures without inventing unrelated feature behavior. Coordinate root file edits through small current-context mutations, no Git worktrees or modifying Git. Temporary outputs remain under ticket generated ownership.
