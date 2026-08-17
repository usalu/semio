//#region 🔧️PolicyRuleArtifactIo
/** 🎫 Normative owner table for stdio roster, DAG, and curated IO matrix (ticket 26/08/10/STDIO-ARTIFACTS-AND-IO). */
const POLICY_STDIO_OWNER_TABLE_REL =
  ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/STDIO-ARTIFACTS-AND-IO/🧪owner-table.json";
const POLICY_STDIO_PLUGIN_REL = "✏️s/🔌️plugins/🗄️stdio";
const POLICY_STDIO_ARTIFACTS_REL = `${POLICY_STDIO_PLUGIN_REL}/🗿️artifacts`;
const POLICY_STDIO_FACET_BUILDER = "🏗️builder";
const POLICY_STDIO_FACET_DECOMPOSER = "🪓️decomposer";
const POLICY_STDIO_FACET_TEXT = "📝️text";
const POLICY_STDIO_FACET_BINARY = "💾️binary";
const POLICY_STDIO_FACET_DESERIALIZERS = "🧩️deserializers";
const POLICY_STDIO_FACET_SERIALIZERS = "🧵️serializers";
const POLICY_STDIO_IO_IMPORT = "📥️import";
const POLICY_STDIO_IO_EXPORT = "📤️export";
const POLICY_STDIO_SCHEMA_CHILD_FALLBACK = ["📸️snapshot", "🔺️diff", "🧬️mutations"] as const;
const POLICY_STDIO_REPRESENTATION_FALLBACK = [POLICY_STDIO_FACET_TEXT, POLICY_STDIO_FACET_BINARY] as const;
const POLICY_STDIO_ARTIFACT_FACET_FALLBACK = ["🧬️schema", "⚙️engine", "🚪️io", POLICY_STDIO_FACET_BUILDER, POLICY_STDIO_FACET_DECOMPOSER] as const;
const POLICY_STDIO_LEGACY_ARTIFACT_FACETS = new Set(["🗣️dsl", "🔧️op", "📡️spr", "🔺️diff", "📸️snapshot"]);
const POLICY_STDIO_TEXT_SPEC_LEAVES = [
  "📖️component.grammar.semio",
  "🔤️component.ebnf",
  "🅰️component.g4",
  "🔗️component.graphql",
  "🔣️component.json",
  "🛰️component.proto",
  POLICY_RS_COMPONENT_LEAF,
  POLICY_TS_COMPONENT_LEAF,
] as const;
const POLICY_STDIO_BINARY_SPEC_LEAVES = [
  "📡️component.protocol.semio",
  "🔠️component.abnf",
  "🥋️component.ksy",
  "🌶️component.spicy",
  POLICY_RS_COMPONENT_LEAF,
  POLICY_TS_COMPONENT_LEAF,
] as const;
const POLICY_STDIO_CODEC_BANNED_MARKERS = ["SRAS", "IFCCARTOONMESH", "b\"minimal\"", "stub codec", "minimal stub codec"] as const;

type PolicyStdioOwnerTable = {
  stdio_roster: Record<string, { dir: string; depends: string[] }>;
  stdio_dag_edges: { from: string; to: string }[];
  owners: Array<{
    path: string;
    stdio_artifacts: string[];
    import: string[];
    export: string[];
  }>;
  counts?: { stdio_artifacts?: number };
};

function policyLoadStdioOwnerTable(repoRoot: string): PolicyStdioOwnerTable | null {
  const abs = join(repoRoot, POLICY_STDIO_OWNER_TABLE_REL);
  if (!existsSync(abs)) return null;
  return JSON.parse(readFileSync(abs, "utf8")) as PolicyStdioOwnerTable;
}

function policyStdioArtifactFacets(taxonomy: ReturnType<typeof loadTaxonomy>): string[] {
  const dirs = taxonomy.artifactComponentDirs;
  if (!dirs?.length) return [...POLICY_STDIO_ARTIFACT_FACET_FALLBACK];
  const out = new Set<string>(POLICY_STDIO_ARTIFACT_FACET_FALLBACK);
  for (const d of dirs) {
    if (!POLICY_STDIO_LEGACY_ARTIFACT_FACETS.has(d)) out.add(d);
  }
  return [...out];
}

function policyStdioSchemaChildDirs(taxonomy: ReturnType<typeof loadTaxonomy>): string[] {
  const from = taxonomy.schemaChildDirs as string[] | undefined;
  return from?.length ? [...from] : [...POLICY_STDIO_SCHEMA_CHILD_FALLBACK];
}

function policyStdioRepresentationDirs(taxonomy: ReturnType<typeof loadTaxonomy>): string[] {
  const from = taxonomy.representationDirs as string[] | undefined;
  return from?.length ? [...from] : [...POLICY_STDIO_REPRESENTATION_FALLBACK];
}

function policyStdioFormatDir(roster: PolicyStdioOwnerTable["stdio_roster"], formatId: string): string | undefined {
  return roster[formatId]?.dir;
}

function policyStdioArtifactsDirName(): string {
  return loadTaxonomy().artifactsDirName ?? "🗿️artifacts";
}

function policyStdioFacetRsTsBreaches(
  repoRoot: string,
  facetRel: string,
  scope: string,
  kind: string,
  traitName: string,
): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const rsRel = `${facetRel}/${POLICY_RS_COMPONENT_LEAF}`;
  const tsRel = `${facetRel}/${POLICY_TS_COMPONENT_LEAF}`;
  if (!existsSync(join(repoRoot, facetRel))) {
    breaches.push({
      id: `${kind}-missing-${facetRel}`,
      summary: `"${scope}" is missing ${facetRel.split("/").pop()}/`,
      kind,
      scope,
      priority: "high",
      reason: `Every artifact must expose ${facetRel.split("/").pop()} with Rust and TypeScript taxonomy leaves.`,
      solution: `Create ${facetRel}/ with ${POLICY_RS_COMPONENT_LEAF} and ${POLICY_TS_COMPONENT_LEAF}.`,
    });
    return breaches;
  }
  if (!existsSync(join(repoRoot, rsRel))) {
    breaches.push({
      id: `${kind}-rs-${facetRel}`,
      summary: `"${rsRel}" is missing`,
      kind,
      scope,
      priority: "high",
      reason: `Facet ${facetRel} must declare ${traitName} in ${POLICY_RS_COMPONENT_LEAF}.`,
      solution: `Add ${rsRel} implementing ${traitName}.`,
    });
  } else {
    const body = readFileSync(join(repoRoot, rsRel), "utf8");
    if (!body.includes(traitName)) {
      breaches.push({
        id: `${kind}-trait-${facetRel}`,
        summary: `"${rsRel}" does not mention ${traitName}`,
        kind,
        scope,
        priority: "high",
        reason: `${POLICY_RS_COMPONENT_LEAF} must implement the SDK ${traitName} trait.`,
        solution: `Implement ${traitName} in ${rsRel}.`,
      });
    }
  }
  if (!existsSync(join(repoRoot, tsRel))) {
    breaches.push({
      id: `${kind}-ts-${facetRel}`,
      summary: `"${tsRel}" is missing`,
      kind,
      scope,
      priority: "high",
      reason: `Facet ${facetRel} must re-export ${traitName} from the TypeScript barrel leaf.`,
      solution: `Add ${tsRel} exporting ${traitName}.`,
    });
  }
  return breaches;
}

/** ⚖️Twenty-nine stdio codec artifacts exist under 🗄️stdio with required completeness facets. */
export function policyStdioCatalogBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) {
    breaches.push({
      id: "stdio-catalog-owner-table-missing",
      summary: `owner table missing at ${POLICY_STDIO_OWNER_TABLE_REL}`,
      kind: "stdio-artifacts/catalog",
      scope: POLICY_STDIO_PLUGIN_REL,
      priority: "high",
      reason: "Stdio roster and DAG are normative in the ticket owner table until taxonomy absorbs them.",
      solution: `Restore ${POLICY_STDIO_OWNER_TABLE_REL}.`,
    });
    return breaches;
  }
  const expectedCount = table.counts?.stdio_artifacts ?? 29;
  const rosterIds = Object.keys(table.stdio_roster ?? {});
  if (rosterIds.length !== expectedCount) {
    breaches.push({
      id: "stdio-catalog-roster-count",
      summary: `stdio_roster has ${rosterIds.length} entries but normative count is ${expectedCount}`,
      kind: "stdio-artifacts/catalog",
      scope: POLICY_STDIO_PLUGIN_REL,
      priority: "high",
      reason: "The closed stdio catalog must list exactly 29 format artifacts.",
      solution: `Fix stdio_roster in ${POLICY_STDIO_OWNER_TABLE_REL}.`,
    });
  }
  const pluginRoot = join(repoRoot, POLICY_STDIO_PLUGIN_REL);
  if (!existsSync(pluginRoot)) {
    breaches.push({
      id: "stdio-catalog-plugin-missing",
      summary: `${POLICY_STDIO_PLUGIN_REL} plugin root is missing`,
      kind: "stdio-artifacts/catalog",
      scope: POLICY_STDIO_PLUGIN_REL,
      priority: "high",
      reason: "Stdio codecs live in the dedicated 🗄️stdio plugin (zero apps).",
      solution: `Scaffold ${POLICY_STDIO_PLUGIN_REL} per ticket W2.`,
    });
    return breaches;
  }
  const taxonomy = loadTaxonomy();
  const requiredFacets = policyStdioArtifactFacets(taxonomy);
  for (const formatId of rosterIds) {
    const entry = table.stdio_roster[formatId]!;
    const artRel = `${POLICY_STDIO_ARTIFACTS_REL}/${entry.dir}`;
    const scope = `🗄️stdio/${entry.dir}`;
    if (!existsSync(join(repoRoot, artRel))) {
      breaches.push({
        id: `stdio-catalog-artifact-${formatId}`,
        summary: `stdio artifact "${formatId}" missing at ${artRel}`,
        kind: "stdio-artifacts/catalog",
        scope,
        priority: "high",
        reason: "Every stdio roster id must materialize as an artifact directory under 🗄️stdio.",
        solution: `Create ${artRel}/ with builder, decomposer, schema, engine, and io facets.`,
      });
      continue;
    }
    for (const facet of requiredFacets) {
      const facetRel = `${artRel}/${facet}`;
      if (existsSync(join(repoRoot, facetRel))) continue;
      breaches.push({
        id: `stdio-catalog-facet-${formatId}-${facet}`,
        summary: `"${artRel}" is missing required facet ${facet}/`,
        kind: "stdio-artifacts/catalog",
        scope,
        priority: "high",
        reason: "Stdio codec artifacts carry the same completeness facets as domain artifacts.",
        solution: `Add ${facetRel}/ per normative spec §2.`,
      });
    }
  }
  return breaches;
}

/** ⚖️Every plugin artifact exposes 🏗️builder with rs+ts implementing ArtifactBuilder. */
export function policyArtifactBuilderBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    breaches.push(
      ...policyStdioFacetRsTsBreaches(
        repoRoot,
        `${artRel}/${POLICY_STDIO_FACET_BUILDER}`,
        artRel,
        "stdio-artifacts/builder",
        "ArtifactBuilder",
      ),
    );
  }
  return breaches;
}

/** ⚖️Every plugin artifact exposes 🪓️decomposer with rs+ts implementing ArtifactDecomposer. */
export function policyArtifactDecomposerBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    breaches.push(
      ...policyStdioFacetRsTsBreaches(
        repoRoot,
        `${artRel}/${POLICY_STDIO_FACET_DECOMPOSER}`,
        artRel,
        "stdio-artifacts/decomposer",
        "ArtifactDecomposer",
      ),
    );
  }
  return breaches;
}

function policySchemaRepresentationLeavesFor(repDir: string): readonly string[] {
  if (repDir === POLICY_STDIO_FACET_TEXT) return POLICY_STDIO_TEXT_SPEC_LEAVES;
  if (repDir === POLICY_STDIO_FACET_BINARY) return POLICY_STDIO_BINARY_SPEC_LEAVES;
  return [];
}

function policySchemaFormatLeafBreaches(
  repoRoot: string,
  facetAbs: string,
  artRel: string,
  taxonomy: ReturnType<typeof loadTaxonomy>,
): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const formats = taxonomy.schemaFormats ?? {};
  for (const [formatId, format] of Object.entries(formats)) {
    const leafRel = `${facetAbs}/${format.leafFilename}`;
    if (existsSync(join(repoRoot, leafRel))) continue;
    breaches.push({
      id: `stdio-schema-format-${leafRel}`,
      summary: `"${facetAbs}" is missing schemaFormats leaf ${format.leafFilename} (${formatId})`,
      kind: "stdio-artifacts/schema-representation",
      scope: artRel,
      priority: "high",
      reason: "Each schema node carries all five schemaFormats leaves from 🔣️taxonomy.json.",
      solution: `Add ${leafRel}.`,
    });
  }
  return breaches;
}

/** ⚖️Schema tree under 🧬️schema with representation text/binary spec leaves (ticket STDIO-ARTIFACTS-AND-IO). */
export function policySchemaRepresentationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const taxonomy = loadTaxonomy();
  const schemaChildDirs = policyStdioSchemaChildDirs(taxonomy);
  const representationDirs = policyStdioRepresentationDirs(taxonomy);
  const schemaFacet = "🧬️schema";
  for (const artRel of policyListPluginArtifactDirs(repoRoot)) {
    const schemaRoot = `${artRel}/${schemaFacet}`;
    if (!existsSync(join(repoRoot, schemaRoot))) {
      breaches.push({
        id: `stdio-schema-root-${artRel}`,
        summary: `"${artRel}" is missing ${schemaFacet}/`,
        kind: "stdio-artifacts/schema-representation",
        scope: artRel,
        priority: "high",
        reason: "Artifact-level schema facet is required; snapshot/diff/mutations nest beneath it.",
        solution: `Create ${schemaRoot}/ per normative spec §1.`,
      });
      continue;
    }
    breaches.push(...policySchemaFormatLeafBreaches(repoRoot, schemaRoot, artRel, taxonomy));
    for (const child of schemaChildDirs) {
      const childAbs = `${schemaRoot}/${child}`;
      if (!existsSync(join(repoRoot, childAbs))) {
        breaches.push({
          id: `stdio-schema-child-${childAbs}`,
          summary: `"${schemaRoot}" is missing child ${child}/`,
          kind: "stdio-artifacts/schema-representation",
          scope: artRel,
          priority: "high",
          reason: `taxonomy.schemaChildDirs requires ${child} under every 🧬️schema facet.`,
          solution: `Add ${childAbs}/ with representation dirs and schemaFormats leaves.`,
        });
        continue;
      }
      breaches.push(...policySchemaFormatLeafBreaches(repoRoot, childAbs, artRel, taxonomy));
      for (const rep of representationDirs) {
        const repAbs = `${childAbs}/${rep}`;
        if (!existsSync(join(repoRoot, repAbs))) {
          breaches.push({
            id: `stdio-schema-rep-${repAbs}`,
            summary: `"${childAbs}" is missing representation ${rep}/`,
            kind: "stdio-artifacts/schema-representation",
            scope: artRel,
            priority: "high",
            reason: `Each schema child carries ${POLICY_STDIO_FACET_TEXT} and ${POLICY_STDIO_FACET_BINARY} spec trees.`,
            solution: `Add ${repAbs}/ with all normative spec leaves.`,
          });
          continue;
        }
        for (const leaf of policySchemaRepresentationLeavesFor(rep)) {
          const leafRel = `${repAbs}/${leaf}`;
          if (existsSync(join(repoRoot, leafRel))) continue;
          breaches.push({
            id: `stdio-schema-leaf-${leafRel}`,
            summary: `"${repAbs}" is missing spec leaf ${leaf}`,
            kind: "stdio-artifacts/schema-representation",
            scope: artRel,
            priority: "high",
            reason: "Text and binary representation nodes own fixed handcrafted spec filenames.",
            solution: `Add ${leafRel}.`,
          });
        }
      }
    }
  }
  return breaches;
}

/** ⚖️Curated import/export deserializer and serializer matrix from 🧪owner-table.json. */
export function policyIoSerializerMatrixBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) return breaches;
  const roster = table.stdio_roster ?? {};
  const artifactsDir = policyStdioArtifactsDirName();
  for (const owner of table.owners ?? []) {
    const scope = owner.path;
    const ioRoot = `${scope}/🚪️io`;
    if (!existsSync(join(repoRoot, ioRoot))) {
      breaches.push({
        id: `stdio-io-matrix-io-${scope}`,
        summary: `"${scope}" is missing 🚪️io/`,
        kind: "stdio-artifacts/io-matrix",
        scope,
        priority: "high",
        reason: "Curated stdio pairs are wired through the io facet deserializer/serializer tree.",
        solution: `Create ${ioRoot}/ with import/deserializers and export/serializers.`,
      });
      continue;
    }
    const checkLeaves = (direction: string, childFacet: string, formatIds: string[], label: string) => {
      for (const formatId of formatIds) {
        const formatDir = policyStdioFormatDir(roster, formatId);
        if (!formatDir) {
          breaches.push({
            id: `stdio-io-matrix-unknown-${scope}-${formatId}`,
            summary: `unknown stdio format id "${formatId}" on ${scope}`,
            kind: "stdio-artifacts/io-matrix",
            scope,
            priority: "high",
            reason: "Matrix format ids must exist in stdio_roster.",
            solution: `Fix ${label} list for ${scope} in ${POLICY_STDIO_OWNER_TABLE_REL}.`,
          });
          continue;
        }
        const leafBase = `${ioRoot}/${direction}/${childFacet}/${artifactsDir}/${formatDir}`;
        for (const leaf of [POLICY_RS_COMPONENT_LEAF, POLICY_TS_COMPONENT_LEAF] as const) {
          const leafRel = `${leafBase}/${leaf}`;
          if (existsSync(join(repoRoot, leafRel))) continue;
          breaches.push({
            id: `stdio-io-matrix-${leafRel}`,
            summary: `missing ${label} ${leaf} for ${formatId} under ${scope}`,
            kind: "stdio-artifacts/io-matrix",
            scope,
            priority: "high",
            reason: "Each curated pair needs both Rust and TypeScript codec leaves under 🗿️artifacts/<stdio-dir>/.",
            solution: `Add ${leafRel}.`,
          });
        }
      }
    };
    checkLeaves(POLICY_STDIO_IO_IMPORT, POLICY_STDIO_FACET_DESERIALIZERS, owner.import ?? [], "import");
    checkLeaves(POLICY_STDIO_IO_EXPORT, POLICY_STDIO_FACET_SERIALIZERS, owner.export ?? [], "export");
  }
  return breaches;
}

/** ⚖️Stdio dependency DAG is acyclic and every format eventually depends on binary. */
export function policyIoTerminalityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) return breaches;
  const roster = table.stdio_roster ?? {};
  const nodes = new Set(Object.keys(roster));
  const adj = new Map<string, Set<string>>();
  for (const id of nodes) adj.set(id, new Set(roster[id]?.depends ?? []));
  for (const edge of table.stdio_dag_edges ?? []) {
    if (!nodes.has(edge.from) || !nodes.has(edge.to)) {
      breaches.push({
        id: `stdio-dag-unknown-${edge.from}-${edge.to}`,
        summary: `stdio_dag_edges references unknown node (${edge.from} → ${edge.to})`,
        kind: "stdio-artifacts/io-terminality",
        scope: POLICY_STDIO_PLUGIN_REL,
        priority: "high",
        reason: "DAG edges must only connect roster format ids.",
        solution: `Align stdio_dag_edges with stdio_roster in ${POLICY_STDIO_OWNER_TABLE_REL}.`,
      });
      continue;
    }
    adj.get(edge.from)?.add(edge.to);
    const rosterDeps = new Set(roster[edge.from]?.depends ?? []);
    if (!rosterDeps.has(edge.to)) {
      breaches.push({
        id: `stdio-dag-roster-edge-${edge.from}-${edge.to}`,
        summary: `stdio_dag_edges has ${edge.from}→${edge.to} but stdio_roster.depends omits "${edge.to}"`,
        kind: "stdio-artifacts/io-terminality",
        scope: POLICY_STDIO_PLUGIN_REL,
        priority: "high",
        reason: "stdio_dag_edges must mirror stdio_roster depends arrays.",
        solution: `Add "${edge.to}" to stdio_roster.${edge.from}.depends or remove the edge.`,
      });
    }
  }
  const visitedGlobal = new Set<string>();
  const findCycle = (start: string): string[] | null => {
    const stack: string[] = [];
    const onStack = new Set<string>();
    const dfs = (n: string): string[] | null => {
      if (onStack.has(n)) return [...stack.slice(stack.indexOf(n)), n];
      if (visitedGlobal.has(n)) return null;
      onStack.add(n);
      stack.push(n);
      for (const dep of adj.get(n) ?? []) {
        const cyc = dfs(dep);
        if (cyc) return cyc;
      }
      stack.pop();
      onStack.delete(n);
      visitedGlobal.add(n);
      return null;
    };
    return dfs(start);
  };
  for (const n of nodes) {
    if (visitedGlobal.has(n)) continue;
    const cyc = findCycle(n);
    if (!cyc) continue;
    breaches.push({
      id: `stdio-dag-cycle-${cyc.join("-")}`,
      summary: `stdio DAG cycle: ${cyc.join(" → ")}`,
      kind: "stdio-artifacts/io-terminality",
      scope: POLICY_STDIO_PLUGIN_REL,
      priority: "high",
      reason: "Stdio codec dependencies must form a DAG.",
      solution: "Remove cyclic depends entries from stdio_roster.",
    });
    break;
  }
  const memo = new Map<string, boolean>();
  const reachesBinary = (n: string, trail: Set<string>): boolean => {
    if (n === "binary") return true;
    if (memo.has(n)) return memo.get(n)!;
    if (trail.has(n)) return false;
    trail.add(n);
    const deps = adj.get(n);
    if (!deps || deps.size === 0) {
      memo.set(n, false);
      return false;
    }
    let ok = true;
    for (const d of deps) ok = ok && reachesBinary(d, trail);
    memo.set(n, ok);
    return ok;
  };
  for (const n of nodes) {
    if (n === "binary") continue;
    if (reachesBinary(n, new Set())) continue;
    breaches.push({
      id: `stdio-dag-term-${n}`,
      summary: `stdio format "${n}" does not terminate at binary via depends`,
      kind: "stdio-artifacts/io-terminality",
      scope: POLICY_STDIO_PLUGIN_REL,
      priority: "high",
      reason: "Every stdio artifact dependency chain must eventually reach the binary root.",
      solution: `Fix depends for "${n}" in stdio_roster until binary is reachable.`,
    });
  }
  return breaches;
}

/** ⚖️Banned stub codec markers (SRAS, IFCCARTOONMESH, minimal stubs) must not remain in Rust sources. */
export function policyCodecFidelityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const scanRoots = [
    "🧰️framework/🔨️modules/🔺️mesh",
    "🧰️framework/🛍️products/💻️os",
    "🧰️framework/🛍️products/💻️os/🖥️host",
    "✏️s/🔌️plugins",
  ];
  const rsFiles = policyWalkRelFiles(repoRoot, scanRoots, (_p, name) => name.endsWith(".rs"));
  for (const rel of rsFiles) {
    const body = readFileSync(join(repoRoot, rel), "utf8");
    for (const marker of POLICY_STDIO_CODEC_BANNED_MARKERS) {
      if (!body.includes(marker)) continue;
      breaches.push({
        id: `stdio-codec-ban-${rel}-${marker}`,
        summary: `"${rel}" contains banned stub marker ${JSON.stringify(marker)}`,
        kind: "stdio-artifacts/codec-fidelity",
        scope: rel,
        priority: "high",
        reason: "Framework and plugin codecs must be real round-trip implementations, not SRAS/IFCCARTOONMESH/minimal stubs.",
        solution: `Replace the stub in ${rel} with a stdio-owned codec or delete the dead path.`,
      });
    }
  }
  return breaches;
}

/** ⚖️Aggregates stdio-artifact policy scanners (catalog, builder, decomposer, schema, io matrix, DAG, codecs). */
export function policyStdioArtifactsBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyStdioCatalogBreaches(repoRoot),
    ...policyArtifactBuilderBreaches(repoRoot),
    ...policyArtifactDecomposerBreaches(repoRoot),
    ...policySchemaRepresentationBreaches(repoRoot),
    ...policyIoSerializerMatrixBreaches(repoRoot),
    ...policyIoTerminalityBreaches(repoRoot),
    ...policyCodecFidelityBreaches(repoRoot),
  ];
}

//#endregion 🔧️PolicyRuleArtifactIo
