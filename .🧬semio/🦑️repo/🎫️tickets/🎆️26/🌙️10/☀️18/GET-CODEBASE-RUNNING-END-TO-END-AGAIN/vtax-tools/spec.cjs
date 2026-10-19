// Spec for the validateTaxonomy decomposition. Emoji are written without U+FE0F; the generator appends it for folder names and docstrings.
const V = "️";
const ROOT = "/Users/ueli/Documents/semio/";
const DISCOVERY = ROOT + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/";
const AUTHORITY_DIR = "🔐️taxonomy-authority";
const FACTORY_STATEMENTS = [0, 1, 2, 3, 4, 11, 14, 15, 16, 17, 19, 20, 21, 22];
const PORTS = ["semanticDirectoryKindId", "fileKindIdForSourcePath", "canonicalFilenameForKind", "canonicalFilenamesForKind", "taxonomyCliAttemptPreparationsProblems", "taxonomyCliLeaseDirectoryProblems", "artifactPathProjectionCatalogRoots", "semanticDescendantNodeRelativePath"];
const AUDIT_MEMBERS = ["taxonomy", "problems", "pathMatcher", "canonicalEmoji", "document", "record", "ids", "pattern", "fullPattern", "pathPattern", "workspacePath", "extensionChain", "exactKeys", "kebabId"];
const context = { emoji: "🪣", slug: "taxonomy-audit-context", doc: "Carries the taxonomy under audit, the shared problem list, the shared check helpers and the injected taxonomy authority ports to every rule group validator." };
const scope = { emoji: "🪜", slug: "audit-schema-scope-vocabulary", doc: "Validates the schema scope-owner, export-resolution and dialect vocabulary families." };
const g = (n, emoji, slug, fn, ranges, doc) => ({ n, emoji, slug, fn, ranges, doc });
const groups = [
  g(1, "📆", "audit-schema-version-and-evidence", "auditSchemaVersionAndEvidence", [[5, 10]], "Audits the removed vocabulary keys, the exact schema version and the frozen coordinate, frozen markdown coordinate and historical document evidence populations."),
  g(2, "📎", "audit-leaf-and-reference-policies", "auditLeafAndReferencePolicies", [[12, 13], [18, 18]], "Audits the physical leaf rendering, the reference closure and the implementation leaf policy declarations."),
  g(3, "📔", "audit-file-kinds", "auditFileKinds", [[23, 25]], "Audits the file kind registry, its resolution rules and the single owner of every extension chain."),
  g(4, "📗", "audit-scoped-and-directory-kinds", "auditScopedAndDirectoryKinds", [[26, 27]], "Audits the scoped file kinds and the semantic directory kind registry."),
  g(5, "🚥", "audit-transaction-artifacts", "auditTransactionArtifacts", [[28, 49]], "Audits the permanent taxonomy CLI artifact directory kinds, the exact transaction scoped file kinds and the preparation and lease grammar they enable."),
  g(6, "🛤", "audit-member-and-descendant-contracts", "auditMemberAndDescendantContracts", [[50, 55]], "Audits the semantic directory member kinds, the projected member kinds, the profile renderers and the descendant contracts."),
  g(7, "🚏", "audit-projection-and-package-contracts", "auditProjectionAndPackageContracts", [[56, 59]], "Audits the path projection catalog and projection contracts, the policy state coordinate contracts and the package projection contracts."),
  g(8, "🔑", "audit-owned-file-projection", "auditOwnedFileProjection", [[60, 60]], "Audits the semantic owned file projection contracts."),
  g(9, "🗞", "audit-reference-consumers-and-catalog", "auditReferenceConsumersAndMutationCatalog", [[61, 69]], "Audits the path projection reference consumer contracts and the mutation catalog projection with its canonical descendant contracts."),
  g(10, "🔩", "audit-fixed-contracts", "auditFixedContracts", [[70, 90]], "Audits the fixed directory contract sets, the fixed filename and directory contracts, the exact Cargo, Nx and Bun scopes and the filename rejection contracts."),
  g(11, "🚛", "audit-package-boundary-contracts", "auditPackageBoundaryContracts", [[91, 95]], "Audits the configurable entry contracts, the package glue grammar, the package boundary rules and profiles and the package source dispositions."),
  g(12, "⚗", "audit-generator-contracts", "auditGeneratorContracts", [[96, 96]], "Audits every generator contract with its ownership, targets, inputs, outputs and preview declarations."),
  g(13, "🛑", "audit-path-policies-and-areas", "auditPathPoliciesAndAreas", [[97, 110]], "Audits the opaque path exclusions, the Unicode, variation selector, path emoji and collision policies and the area enforcement and area states."),
  g(14, "🚇", "audit-dependency-directions", "auditDependencyDirections", [[111, 113]], "Audits the dependency direction roles and rules, the Cargo dependency direction rules and the area layers."),
  g(15, "🌄", "audit-ecosystems-and-targets", "auditEcosystemsAndTargets", [[114, 115]], "Audits the ecosystem and target registries."),
  g(16, "🎲", "audit-test-vocabulary", "auditTestVocabulary", [[116, 132]], "Audits the test vocabulary: implementation kind mappings, directory names, obsolete stems, modules, patterns and in-package implementations."),
  g(17, "📙", "audit-manifest-and-root-contracts", "auditManifestAndRootContracts", [[133, 139]], "Audits the semantic manifest filename overrides, the subset directory overrides, the spec file kinds and the root contract id lists."),
  g(18, "📘", "audit-schema-formats-and-facets", "auditSchemaFormatsAndFacets", [[140, 143]], "Audits the schema formats, the schema facet kinds and the schema scope vocabulary."),
  g(19, "💉", "audit-mutation-vocabulary", "auditMutationVocabulary", [[144, 157]], "Audits the mutation directory pattern, domain owners, facet directories, forbidden region markers, file kinds and payload schema declarations."),
  g(20, "🛖", "audit-representation-and-child-dirs", "auditRepresentationAndChildDirectories", [[158, 163]], "Audits the semantic directory registrations of every declared directory value and the required child directory subsets."),
];
module.exports = { V, ROOT, DISCOVERY, AUTHORITY_DIR, FACTORY_STATEMENTS, PORTS, AUDIT_MEMBERS, context, scope, groups };
