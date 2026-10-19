
/** 🪣️ The taxonomy authority functions the rule group validators call; the authority module injects them so no rule group imports the module that loads and validates the taxonomy. */
export type TaxonomyAuditPorts = Readonly<{
  semanticDirectoryKindId: (name: string, taxonomy: Taxonomy, context?: SemanticDirectoryMatchContext) => string | null;
  fileKindIdForSourcePath: (path: string, taxonomy: Taxonomy) => string | null;
  canonicalFilenameForKind: (kindId: string, taxonomy: Taxonomy) => string;
  canonicalFilenamesForKind: (kindId: string, taxonomy: Taxonomy) => string[];
  taxonomyCliAttemptPreparationsProblems: (facts: readonly Readonly<TaxonomyCliAttemptPreparationFacts>[], taxonomy: Taxonomy) => string[];
  taxonomyCliLeaseDirectoryProblems: (facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[]; writePreparations: readonly Readonly<TaxonomyCliWritePreparationFacts>[] }>, taxonomy: Taxonomy) => string[];
  artifactPathProjectionCatalogRoots: (artifactRoot: string, contractId: string, taxonomy: Taxonomy) => readonly Readonly<{ sourceRoot: string; destinationRoot: string }>[];
  semanticDescendantNodeRelativePath: (node: SemanticDescendantNode, taxonomy: Taxonomy) => string;
}>;

/** 🪣️ One taxonomy under audit with its shared problem list, its shared check helpers and the injected authority ports. */
export type TaxonomyAudit = TaxonomyAuditPorts & Readonly<{
  taxonomy: Taxonomy;
  problems: string[];
  pathMatcher: TaxonomyPathMatcher;
  canonicalEmoji: (value: unknown) => value is string;
  document: Record<string, unknown>;
  record: (value: unknown, key: string) => value is Record<string, unknown>;
  ids: (values: readonly string[] | undefined, registry: Readonly<Record<string, unknown>>, key: string) => void;
  pattern: (value: string, key: string) => void;
  fullPattern: (value: unknown, key: string) => void;
  pathPattern: (value: unknown, key: string) => void;
  workspacePath: (value: unknown, key: string) => value is string;
  extensionChain: (value: unknown, key: string) => void;
  exactKeys: (value: object, allowed: readonly string[], key: string) => void;
  kebabId: (value: string, key: string) => void;
}>;

