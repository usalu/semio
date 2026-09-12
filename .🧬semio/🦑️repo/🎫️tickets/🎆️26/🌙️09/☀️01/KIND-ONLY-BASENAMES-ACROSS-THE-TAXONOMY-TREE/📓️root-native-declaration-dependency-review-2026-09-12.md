# Root Native Declaration Dependency Review

Root used the installed TypeScript parser and symbol checker on the current root script. This is a read-only extraction aid, not a typecheck or runtime acceptance. The program retains local lexical/type bindings and intentionally disables module/library resolution; external import implementations, dynamic/source-as-data consumers and string-based native calls require separate consumer review.

The current snapshot has 1206 top-level statements and 3034 resolved declaration edges, zero native syntax diagnostics and no multi-declaration strongly connected components. Direct self-references are excluded from this graph; the result does not assert absence of recursive calls or cross-module cycles. It shows that these local concerns can be separated along their actual dependencies without mechanically preserving editorial regions.

Source SHA-256 d7a8a6c27882d4c9b6d7ba213d5c3ed04bb9f1c066dd90d0fbe8fafceb0e13ae is report-only snapshot provenance, never a permanent source-body contract. Raw rows/edges remain in generated/coordinator/root-native-declaration-graph.json.

## Candidate Local Closures

| Concern | Non-import declarations | Declaration lines | Explicit seeds |
| --- | ---: | ---: | --- |
| Inventory shard protocol | 24 | 382 | buildTaxonomyInventoryArtifactShards, validateTaxonomyInventoryArtifactShards, publishTaxonomyInventoryArtifactShards |
| Mutation inventory and structural evidence | 101 | 1088 | inventoryMutationTaxonomy, planMutationTaxonomy, runMutationTaxonomyCli, inspectMutationRootReachability, policyMutationStructuralBreaches, validateJsonSchemaSubset |
| Workspace cleanup | 50 | 500 | runWorkspaceClean |
| Artifact and mutation scaffold | 36 | 360 | newScaffoldMutationTree, newScaffoldArtifactTree, newScaffoldStandardTree, newScaffoldSubsetTree |

These are overlapping transitive local closures, not proposed single output files. Separate common primitives by their actual domain and reuse their canonical owner. Preserve existing imported library interfaces. Mutation source capture/structural validation must not import back into its higher-level workflow after extraction.

## Shared Upstream Definitions

| Definition | Direct declaration consumers | Snapshot line |
| --- | ---: | ---: |
| policyReadFileSafe | 92 | 19338 |
| toolJobRustBlock | 61 | 868 |
| POLICY_RS_COMPONENT_LEAF_NAME | 34 | 17746 |
| policyStripEmoji | 32 | 17187 |
| policyReaddirSafe | 31 | 19317 |
| interactivityProductionSource | 27 | 11344 |
| policyAllRustFiles | 27 | 18306 |
| policyWalkRelFiles | 26 | 21984 |
| POLICY_RS_COMPONENT_LEAF | 17 | 21962 |
| policyLineOfIndex | 16 | 17521 |
| policyNormalizeRelPath | 15 | 17259 |
| PolicyCrateRef | 15 | 17306 |
| POLICY_TS_COMPONENT_LEAF | 14 | 17745 |
| MutationTaxonomyStructuralSourceView | 13 | 14790 |
| POLICY_SKIP_DIRS | 13 | 17184 |

In particular, policyReadFileSafe/policyReaddirSafe and source lexing/discovery have broad use. Inspect existing source-capture/discovery implementations before extracting these separately. Their current empty-on-error behavior must not silently replace stronger captured-source authority. The topology goal does not authorize relaxing law evidence or losing cancellation/read failures.

## Concrete Closure Members

### Inventory shard protocol

- TAXONOMY_INVENTORY_SHARD_MAX_BYTES (line 14189, 1 declaration lines)
- TaxonomyInventoryShardDescriptor (line 14191, 10 declaration lines)
- TaxonomyInventoryShardManifest (line 14202, 10 declaration lines)
- TaxonomyInventoryArtifactShard (line 14213, 4 declaration lines)
- TaxonomyInventoryArtifactShards (line 14218, 5 declaration lines)
- TaxonomyInventoryShardValidation (line 14224, 5 declaration lines)
- TaxonomyInventoryShardProgress (line 14230, 6 declaration lines)
- taxonomyCliRecord (line 14237, 4 declaration lines)
- taxonomyCliExactKeys (line 14242, 5 declaration lines)
- taxonomyCliSha256 (line 14248, 3 declaration lines)
- taxonomyCliCanonicalJson (line 14252, 10 declaration lines)
- taxonomyCliCanonicalArrayDigest (line 14263, 10 declaration lines)
- taxonomyInventoryCanonicalChunks (line 14275, 30 declaration lines)
- taxonomyInventoryIncrementalCanonicalDigest (line 14307, 5 declaration lines)
- taxonomyCliByteCompare (line 14313, 3 declaration lines)
- taxonomyCliEntryViolations (line 14317, 6 declaration lines)
- taxonomyCliStableViolations (line 14324, 15 declaration lines)
- taxonomyCliInventoryEntries (line 14340, 9 declaration lines)
- taxonomyCliInventoryMetadata (line 14350, 3 declaration lines)
- buildTaxonomyInventoryArtifactShards (line 14355, 77 declaration lines)
- validateTaxonomyInventoryArtifactShards (line 14434, 63 declaration lines)
- taxonomyCliShardPayloadPaths (line 14498, 13 declaration lines)
- taxonomyCliValidatePublishedShardRoot (line 14512, 27 declaration lines)
- publishTaxonomyInventoryArtifactShards (line 14541, 58 declaration lines)

### Mutation inventory and structural evidence

- TaxonomyCliFormat (line 14070, 1 declaration lines)
- TaxonomyCliOperation (line 14071, 1 declaration lines)
- TaxonomyCliKind (line 14072, 1 declaration lines)
- TAXONOMY_CLI_ARTIFACT_DIRECTORIES (line 14074, 6 declaration lines)
- TaxonomyCliOptions (line 14081, 13 declaration lines)
- taxonomyCliGuardedPath (line 14154, 24 declaration lines)
- taxonomyCliArtifactPath (line 14183, 4 declaration lines)
- taxonomyCliWriteJson (line 14601, 4 declaration lines)
- taxonomyCliProgress (line 14613, 4 declaration lines)
- taxonomyCliRequireCommittedApply (line 14670, 3 declaration lines)
- taxonomyCliPrintJson (line 14674, 3 declaration lines)
- MutationTaxonomyRecord (line 14679, 33 declaration lines)
- MutationTaxonomyConsumerKind (line 14713, 1 declaration lines)
- MutationTaxonomyConsumerEdge (line 14714, 1 declaration lines)
- MutationTaxonomyAssignmentEvidence (line 14715, 1 declaration lines)
- MutationTaxonomyAssignmentRow (line 14716, 1 declaration lines)
- MutationTaxonomyEvidence (line 14717, 1 declaration lines)
- MutationTaxonomySourceRecord (line 14718, 1 declaration lines)
- MutationTaxonomyInventory (line 14720, 10 declaration lines)
- MutationTaxonomyInventoryOptions (line 14731, 9 declaration lines)
- MutationTaxonomyPlanMove (line 14741, 1 declaration lines)
- MutationTaxonomyPlan (line 14742, 9 declaration lines)
- mutationTaxonomySegmentAfter (line 14752, 5 declaration lines)
- mutationTaxonomyLocations (line 14758, 3 declaration lines)
- MUTATION_TAXONOMY_ASSIGNMENT_LEDGER_SCHEMA (line 14762, 23 declaration lines)
- MutationTaxonomyCapturedSchema (line 14786, 1 declaration lines)
- MutationTaxonomyStructuralDirectory (line 14787, 1 declaration lines)
- MutationTaxonomySourceIndex (line 14788, 1 declaration lines)
- MutationTaxonomyStructuralSourceView (line 14790, 1 declaration lines)
- mutationTaxonomyCompare (line 14792, 3 declaration lines)
- mutationTaxonomyCapturedSchema (line 14796, 6 declaration lines)
- mutationTaxonomyStructuralDirectories (line 14803, 19 declaration lines)
- mutationTaxonomyStructuralView (line 14823, 3 declaration lines)
- mutationTaxonomyCancelled (line 14827, 5 declaration lines)
- mutationTaxonomyInputPath (line 14833, 13 declaration lines)
- mutationTaxonomyScope (line 14847, 5 declaration lines)
- mutationTaxonomySourceAdmission (line 14854, 10 declaration lines)
- mutationTaxonomySourceFiles (line 14866, 7 declaration lines)
- MutationTaxonomySourceFileFact (line 14875, 5 declaration lines)
- mutationTaxonomySourceFileFacts (line 14882, 6 declaration lines)
- mutationTaxonomyAssignmentLedger (line 14890, 18 declaration lines)
- mutationTaxonomySourceIndex (line 14909, 31 declaration lines)
- mutationTaxonomyLeafAlias (line 14941, 3 declaration lines)
- mutationTaxonomyConsumerKind (line 14949, 13 declaration lines)
- mutationTaxonomySourceSnapshot (line 14963, 3 declaration lines)
- mutationTaxonomyRustSpecs (line 14967, 3 declaration lines)
- mutationTaxonomyTsSpecs (line 14971, 3 declaration lines)
- mutationTaxonomyAssignment (line 14975, 7 declaration lines)
- MutationTaxonomyRustModuleContext (line 14983, 1 declaration lines)
- MutationTaxonomyRustModuleGraph (line 14984, 1 declaration lines)
- mutationTaxonomyRustModuleKey (line 14986, 3 declaration lines)
- mutationTaxonomyRustModuleGraph (line 14991, 3 declaration lines)
- mutationTaxonomyRustUsePath (line 14995, 4 declaration lines)
- mutationTaxonomyResolveRustGraphTargets (line 15000, 25 declaration lines)
- mutationTaxonomyResolveTargets (line 15026, 18 declaration lines)
- inventoryMutationTaxonomy (line 15046, 93 declaration lines)
- planMutationTaxonomy (line 15141, 28 declaration lines)
- verifyMutationTaxonomy (line 15170, 4 declaration lines)
- mutationTaxonomyCheckCancellation (line 15175, 3 declaration lines)
- runMutationTaxonomyCli (line 15179, 51 declaration lines)
- policyStripEmoji (line 17187, 3 declaration lines)
- POLICY_MUTATIONS_FACET (line 17742, 1 declaration lines)
- POLICY_TS_COMPONENT_LEAF (line 17745, 1 declaration lines)
- POLICY_RS_COMPONENT_LEAF_NAME (line 17746, 1 declaration lines)
- policyLeadingEmojiPrefix (line 22413, 6 declaration lines)
- PolicyStructuralMutationChildClassification (line 22431, 1 declaration lines)
- PolicyStructuralMutationChild (line 22432, 1 declaration lines)
- policyStructuralMutationChildren (line 22434, 38 declaration lines)
- policyStructuralMutationDirs (line 22473, 3 declaration lines)
- policyMutationSemanticIdentity (line 22478, 6 declaration lines)
- policyStructuralSource (line 22485, 3 declaration lines)
- policyStructuralNodeState (line 22489, 7 declaration lines)
- policyStructuralRelativeLocator (line 22497, 5 declaration lines)
- policyMutationPayloadSchemaProblems (line 22504, 7 declaration lines)
- policyStructuralMutationRoots (line 22512, 9 declaration lines)
- policyMutationDirectOwnerBreachesView (line 22522, 12 declaration lines)
- policyFindAllMutationsDirs (line 22804, 14 declaration lines)
- policyArtifactRootOfMutationsDir (line 22824, 7 declaration lines)
- policyMutationEnumVariantNames (line 22906, 3 declaration lines)
- policyKebabToPascal (line 22911, 7 declaration lines)
- MUTATION_STRUCTURAL_POLICY_KINDS (line 22986, 19 declaration lines)
- MutationStructuralKind (line 23005, 1 declaration lines)
- MutationLeafDescriptor (line 23007, 16 declaration lines)
- MUTATION_DESCRIPTOR_SCHEMA_REL (line 23024, 1 declaration lines)
- jsonSchemaSubsetObject (line 23027, 3 declaration lines)
- jsonSchemaSubsetValueEquals (line 23031, 3 declaration lines)
- jsonSchemaSubsetTypeMatches (line 23035, 10 declaration lines)
- jsonSchemaSubsetErrors (line 23046, 36 declaration lines)
- validateJsonSchemaSubset (line 23084, 3 declaration lines)
- policyMutationDescriptorView (line 23089, 10 declaration lines)
- policyMutationAggregateMembers (line 23106, 44 declaration lines)
- policyMutationStructuralBreach (line 23151, 3 declaration lines)
- policyMutationBinaryTag (line 23156, 13 declaration lines)
- policyMutationRootPurityBreaches (line 23170, 13 declaration lines)
- WrappedMutationTypeOrigin (line 23184, 5 declaration lines)
- MutationRootReachability (line 23190, 9 declaration lines)
- inspectMutationRootReachability (line 23201, 9 declaration lines)
- inspectMutationRootReachabilityView (line 23210, 42 declaration lines)
- policyMutationLeafHasRunnableTestView (line 23253, 20 declaration lines)
- policyMutationStructuralBreachesView (line 23275, 125 declaration lines)
- policyMutationStructuralBreaches (line 23401, 4 declaration lines)

### Workspace cleanup

- CLEAN_TICKET_FILE_MAX_BYTES (line 14052, 1 declaration lines)
- CLEAN_TICKET_DIR_MAX_BYTES (line 14053, 1 declaration lines)
- CLEAN_BUILD_ARTIFACT_MAX_BYTES (line 14054, 1 declaration lines)
- CLEAN_CANONICAL_REPO_DIR (line 14055, 1 declaration lines)
- CLEAN_CANONICAL_TICKETS_DIR (line 14056, 1 declaration lines)
- CLEAN_BUILD_DIR_NAMES (line 14057, 1 declaration lines)
- CLEAN_CACHE_DIR_NAME (line 14058, 1 declaration lines)
- CLEAN_TICKET_GENERATED_OUTPUT_DIRS (line 14059, 1 declaration lines)
- CLEAN_TICKET_GENERATED_PROBE_PREFIXES (line 14060, 1 declaration lines)
- CleanRemovalKind (line 14062, 1 declaration lines)
- CleanRemoval (line 14064, 5 declaration lines)
- cleanEndsWithAscii (line 15397, 3 declaration lines)
- cleanIsCanonicalRepoDir (line 15401, 3 declaration lines)
- cleanIsCanonicalTicketsDir (line 15405, 3 declaration lines)
- cleanIsMisplacedRepoDir (line 15409, 3 declaration lines)
- cleanIsMisplacedTicketsDir (line 15413, 3 declaration lines)
- cleanIsBuildArtifactDirName (line 15417, 5 declaration lines)
- cleanIsCargoTargetDirName (line 15424, 3 declaration lines)
- cleanIsCargoTargetDir (line 15429, 3 declaration lines)
- cleanIsSemioRootName (line 15433, 3 declaration lines)
- CLEAN_WINDOWS_RESERVED_DEVICE_NAMES (line 15437, 1 declaration lines)
- CLEAN_WINDOWS_FORBIDDEN_CHARS (line 15438, 1 declaration lines)
- cleanIsWindowsIllegalName (line 15441, 8 declaration lines)
- cleanProtectedPrefixes (line 15450, 9 declaration lines)
- cleanIsProtected (line 15460, 4 declaration lines)
- CleanProtectionNodeKind (line 15466, 1 declaration lines)
- CleanProtectionView (line 15469, 5 declaration lines)
- CLEAN_PROTECTION_VIEW (line 15475, 8 declaration lines)
- cleanIntersectsProtected (line 15484, 4 declaration lines)
- cleanTicketManifestIsClosed (line 15489, 10 declaration lines)
- cleanIsTicketFolderBoundary (line 15500, 4 declaration lines)
- cleanTicketFolderForPath (line 15505, 11 declaration lines)
- cleanRemovalProtection (line 15518, 43 declaration lines)
- cleanProjectRemovals (line 15563, 14 declaration lines)
- cleanPathBytes (line 15579, 12 declaration lines)
- cleanRemovePath (line 15592, 17 declaration lines)
- cleanWalkDirs (line 15610, 25 declaration lines)
- cleanCollectMisplaced (line 15636, 54 declaration lines)
- cleanDiscoverTicketRoots (line 15691, 14 declaration lines)
- cleanDiscoverTicketFolders (line 15707, 13 declaration lines)
- cleanTicketSizeRemovals (line 15735, 39 declaration lines)
- cleanTicketGeneratedOutputRemovals (line 15775, 12 declaration lines)
- cleanIsTicketGeneratedOutputDir (line 15788, 3 declaration lines)
- cleanTicketGeneratedOutputTicketRoot (line 15792, 5 declaration lines)
- cleanGitignoredMapForTicketRoots (line 15798, 24 declaration lines)
- cleanCollectWindowsIllegal (line 15823, 51 declaration lines)
- cleanBuildArtifactRemovals (line 15875, 17 declaration lines)
- cleanDedupePreferDeepest (line 15894, 9 declaration lines)
- cleanDedupePreferShallowest (line 15905, 9 declaration lines)
- runWorkspaceClean (line 15915, 29 declaration lines)

### Artifact and mutation scaffold

- taxonomyMappedFilename (line 652, 5 declaration lines)
- NEW_SCAFFOLD_MARKER (line 16179, 1 declaration lines)
- NEW_SCAFFOLD_TICKET_PATH (line 16180, 1 declaration lines)
- newScaffoldRustLeaf (line 16182, 3 declaration lines)
- newScaffoldTsLeaf (line 16186, 3 declaration lines)
- newScaffoldEmptyFacetMarkdown (line 16190, 3 declaration lines)
- newScaffoldWriteIfAbsent (line 16195, 19 declaration lines)
- newScaffoldIoTree (line 16231, 17 declaration lines)
- newScaffoldSubsetTree (line 16249, 18 declaration lines)
- newScaffoldStandardTree (line 16268, 13 declaration lines)
- newScaffoldArtifactTree (line 16282, 10 declaration lines)
- NewMutationScaffoldOptions (line 16294, 10 declaration lines)
- newMutationSemanticParts (line 16305, 8 declaration lines)
- newMutationRustLeaf (line 16314, 16 declaration lines)
- newMutationDescriptor (line 16331, 3 declaration lines)
- NewMutationScaffoldOwnedPath (line 16335, 1 declaration lines)
- newMutationScaffoldLstat (line 16337, 8 declaration lines)
- newMutationScaffoldPath (line 16346, 17 declaration lines)
- newMutationScaffoldOwnPath (line 16364, 5 declaration lines)
- newMutationScaffoldRemoveOwnedFile (line 16370, 10 declaration lines)
- newMutationScaffoldEnsureParents (line 16381, 19 declaration lines)
- newMutationScaffoldRemoveOwnedDirectory (line 16401, 10 declaration lines)
- newMutationCheckCancellation (line 16412, 3 declaration lines)
- newMutationUpdateAggregate (line 16416, 39 declaration lines)
- newScaffoldMutationTree (line 16457, 74 declaration lines)
- POLICY_SKIP_DIRS (line 17184, 1 declaration lines)
- policyStripEmoji (line 17187, 3 declaration lines)
- POLICY_MUTATION_PLAN_DIR (line 17741, 1 declaration lines)
- POLICY_MUTATIONS_FACET (line 17742, 1 declaration lines)
- POLICY_TS_COMPONENT_LEAF (line 17745, 1 declaration lines)
- POLICY_RS_COMPONENT_LEAF_NAME (line 17746, 1 declaration lines)
- policyReaddirSafe (line 19317, 9 declaration lines)
- policyLeadingEmojiPrefix (line 22413, 6 declaration lines)
- policyListMutationDirs (line 22421, 9 declaration lines)
- policyStructuralRelativeLocator (line 22497, 5 declaration lines)
- policyKebabToPascal (line 22911, 7 declaration lines)

## Execution

The root-taxonomy-workflow packet owns the first two groups. Cleanup/ticket protection and scaffold authorization are distinct subsequent lifecycle concerns. Re-read live source and imports before each move; close every native/test/source-as-data consumer, retain portable protocol fixtures plus third-party/native oracles, and keep root commands as dispatch. No live cleanup, mutation apply, scaffold write into active product trees or remote graph publication was performed by this probe.
