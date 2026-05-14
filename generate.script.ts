#!/usr/bin/env bun
/** 🗄️ Seeds the semio Neo4j schema graph from the schema sketch and writes a replayable dump to .repo/🛂/semio.cypher. */
import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

//#region 🧭Constants
const REPO_ROOT = import.meta.dir;
const SCHEMA_SOURCE = "semio/dev/schema/neo4j/schema.graphql";
const OUTPUT_FILE = ".repo/🛂/semio.cypher";
const NEO4J_VERSION = "5.26.26";
const SCHEMA_TAG = "semio-neo4j-schema";
//#endregion 🧭Constants

//#region 🧩Kinds
type SemioFieldKind = "EMBEDDED" | "REFERENCE" | "COMPUTED" | "CACHED";
type SemioNodeKind = "Module" | "Scalar" | "Interface" | "Class" | "Field" | "Constraint";
type SemioEntityKind = "Interface" | "Class";

type FieldDef = {
  name: string;
  kind: SemioFieldKind;
  targetName: string;
  isList: boolean;
};

type EntityDef = {
  name: string;
  kind: SemioEntityKind;
  moduleName: string;
  implements: string[];
  constraints: string[];
  fields: FieldDef[];
};

type ModuleDef = {
  name: string;
  scalars: string[];
  interfaces: EntityDef[];
  classes: EntityDef[];
  modules: string[];
};

type SchemaModel = {
  modules: ModuleDef[];
};

type GraphNode = {
  id: string;
  label: SemioNodeKind;
  properties: Record<string, string | boolean>;
};

type GraphRelationship = {
  from: string;
  to: string;
  kind: string;
};
//#endregion 🧩Kinds

//#region 🪄Parsing
function extractSketchLines(rawSchema: string): string[] {
  const lines = rawSchema.split(/\r?\n/);
  const sketchLines: string[] = [];
  let started = false;

  for (const line of lines) {
    if (line.startsWith("#")) {
      started = true;
      sketchLines.push(line.replace(/^#\s?/, ""));
      continue;
    }

    if (!started) {
      continue;
    }

    if (!line.trim()) {
      sketchLines.push("");
      continue;
    }

    if (started) {
      break;
    }
  }

  return sketchLines;
}

function parseAliasList(rawValue: string): string[] {
  const trimmed = rawValue.trim();
  if (!trimmed.startsWith("[") || !trimmed.endsWith("]")) {
    return [];
  }

  return trimmed
    .slice(1, -1)
    .split(",")
    .map((entry) => normalizeAlias(entry))
    .filter(Boolean);
}

function normalizeAlias(rawValue: string): string {
  return rawValue.trim().replace(/^\*/, "").replace(/^&/, "");
}

function parseScalarItem(rawLine: string): string {
  const trimmed = rawLine.trim();
  return normalizeAlias(trimmed.replace(/^-\s*/, ""));
}

function parseEntityName(rawLine: string): string {
  const key = rawLine.split(":", 1)[0] ?? "";
  return key.trim();
}

function parseConstraintDescription(rawLine: string): string {
  const trimmed = rawLine.trim().replace(/^-\s*/, "");
  const withoutAnchor = trimmed.replace(/^&[^\s]+\s+/, "");
  const quotedMatch = withoutAnchor.match(/^"(.+)"$/);
  return quotedMatch ? quotedMatch[1] : withoutAnchor;
}

function mapFieldKind(rawKind: string): SemioFieldKind {
  const normalized = rawKind.trim().toLowerCase();
  if (normalized === "reference") {
    return "REFERENCE";
  }
  if (normalized === "computed") {
    return "COMPUTED";
  }
  if (normalized === "cached") {
    return "CACHED";
  }

  return "EMBEDDED";
}

function parseField(rawLine: string, kind: SemioFieldKind): FieldDef {
  const [rawName, rawTarget] = rawLine.split(":", 2);
  if (!rawName || !rawTarget) {
    throw new Error(`Invalid field declaration: ${rawLine}`);
  }

  const targetValue = rawTarget.trim();
  const isList = targetValue.startsWith("[") && targetValue.endsWith("]");
  const targetName = isList ? parseAliasList(targetValue)[0] ?? "" : normalizeAlias(targetValue);

  return {
    name: rawName.trim(),
    kind,
    targetName,
    isList,
  };
}

function parseSchemaSketch(rawSchema: string): SchemaModel {
  const lines = extractSketchLines(rawSchema);
  const rootModule: ModuleDef = {
    name: "schema",
    scalars: [],
    interfaces: [],
    classes: [],
    modules: ["general", "domain"],
  };
  const modules = new Map<string, ModuleDef>([[rootModule.name, rootModule]]);
  let currentModule: ModuleDef | null = null;
  let currentSection: "scalars" | "interfaces" | "classes" | null = null;
  let currentEntity: EntityDef | null = null;
  let currentFieldKind: SemioFieldKind | null = null;
  let inConstraints = false;
  let inFields = false;

  for (const rawLine of lines) {
    if (!rawLine.trim()) {
      continue;
    }

    const indent = rawLine.length - rawLine.trimStart().length;
    const trimmed = rawLine.trim();

    if (indent === 0 && trimmed === "schema:") {
      currentModule = null;
      currentSection = null;
      currentEntity = null;
      currentFieldKind = null;
      inConstraints = false;
      inFields = false;
      continue;
    }

    if (indent === 2 && /^(general|domain):/.test(trimmed)) {
      const moduleName = parseEntityName(trimmed);
      const moduleDef: ModuleDef = {
        name: moduleName,
        scalars: [],
        interfaces: [],
        classes: [],
        modules: [],
      };
      modules.set(moduleName, moduleDef);
      currentModule = moduleDef;
      currentSection = null;
      currentEntity = null;
      currentFieldKind = null;
      inConstraints = false;
      inFields = false;
      continue;
    }

    if (!currentModule) {
      continue;
    }

    if (indent === 4 && /^(scalars|interfaces|classes):/.test(trimmed)) {
      currentSection = parseEntityName(trimmed) as "scalars" | "interfaces" | "classes";
      currentEntity = null;
      currentFieldKind = null;
      inConstraints = false;
      inFields = false;
      continue;
    }

    if (currentSection === "scalars" && indent === 6 && trimmed.startsWith("- ")) {
      currentModule.scalars.push(parseScalarItem(trimmed));
      continue;
    }

    if ((currentSection === "interfaces" || currentSection === "classes") && indent === 6 && !trimmed.startsWith("- ")) {
      currentEntity = {
        name: parseEntityName(trimmed),
        kind: currentSection === "interfaces" ? "Interface" : "Class",
        moduleName: currentModule.name,
        implements: [],
        constraints: [],
        fields: [],
      };
      if (currentSection === "interfaces") {
        currentModule.interfaces.push(currentEntity);
      } else {
        currentModule.classes.push(currentEntity);
      }
      currentFieldKind = null;
      inConstraints = false;
      inFields = false;
      continue;
    }

    if (!currentEntity) {
      continue;
    }

    if (indent === 8 && trimmed.startsWith("implements:")) {
      currentEntity.implements = parseAliasList(trimmed.split(":", 2)[1] ?? "");
      continue;
    }

    if (indent === 8 && /^(contraints|constraints):$/.test(trimmed)) {
      inConstraints = true;
      inFields = false;
      currentFieldKind = null;
      continue;
    }

    if (indent === 8 && trimmed === "fields:") {
      inFields = true;
      inConstraints = false;
      currentFieldKind = null;
      continue;
    }

    if (inConstraints && indent === 10 && trimmed.startsWith("- ")) {
      currentEntity.constraints.push(parseConstraintDescription(trimmed));
      continue;
    }

    if (inFields && indent === 10 && trimmed.endsWith(":")) {
      currentFieldKind = mapFieldKind(trimmed.slice(0, -1));
      continue;
    }

    if (inFields && currentFieldKind && indent === 12) {
      currentEntity.fields.push(parseField(trimmed, currentFieldKind));
    }
  }

  return {
    modules: [rootModule, ...[...modules.values()].filter((moduleDef) => moduleDef.name !== rootModule.name)],
  };
}
//#endregion 🪄Parsing

//#region 🏗️GraphBuild
function emojiForLabel(label: SemioNodeKind, name: string): string {
  if (label === "Module") {
    if (name === "schema") {
      return "🗺️";
    }
    if (name === "general") {
      return "🧰";
    }
    return "🏛️";
  }
  if (label === "Scalar") {
    return "🔣";
  }
  if (label === "Interface") {
    return "🧩";
  }
  if (label === "Class") {
    return "🧱";
  }
  if (label === "Field") {
    return "🏷️";
  }
  return "📏";
}

function nodeId(label: SemioNodeKind, name: string): string {
  return `${label.toLowerCase()}:${name}`;
}

function entityNodeId(entity: EntityDef): string {
  return nodeId(entity.kind, entity.name);
}

function fieldNodeId(entity: EntityDef, field: FieldDef): string {
  return `field:${entity.kind.toLowerCase()}:${entity.name}:${field.kind.toLowerCase()}:${field.name}`;
}

function constraintNodeId(entity: EntityDef, index: number): string {
  return `constraint:${entity.kind.toLowerCase()}:${entity.name}:${index + 1}`;
}

function buildGraph(model: SchemaModel): { nodes: GraphNode[]; relationships: GraphRelationship[] } {
  const nodes = new Map<string, GraphNode>();
  const relationships = new Map<string, GraphRelationship>();
  const knownTargets = new Map<string, string>();

  const addNode = (label: SemioNodeKind, name: string, properties: Record<string, string | boolean> = {}): string => {
    const id = nodeId(label, name);
    nodes.set(id, {
      id,
      label,
      properties: {
        id,
        name,
        emoji: emojiForLabel(label, name),
        schemaSource: SCHEMA_SOURCE,
        schemaTag: SCHEMA_TAG,
        ...properties,
      },
    });
    knownTargets.set(name, id);
    return id;
  };

  const addRelationship = (from: string, to: string, kind: string): void => {
    relationships.set(`${from}|${kind}|${to}`, { from, to, kind });
  };

  for (const moduleDef of model.modules) {
    addNode("Module", moduleDef.name);
  }

  for (const moduleDef of model.modules.filter((entry) => entry.name !== "schema")) {
    addRelationship(nodeId("Module", "schema"), nodeId("Module", moduleDef.name), "HAS");
  }

  for (const moduleDef of model.modules.filter((entry) => entry.name !== "schema")) {
    for (const scalarName of moduleDef.scalars) {
      addNode("Scalar", scalarName, { scalarKind: scalarName });
      addRelationship(nodeId("Module", moduleDef.name), nodeId("Scalar", scalarName), "HAS");
    }

    for (const entity of [...moduleDef.interfaces, ...moduleDef.classes]) {
      const entityId = addNode(entity.kind, entity.name, {
        moduleName: entity.moduleName,
        entityKind: entity.kind,
      });
      addRelationship(nodeId("Module", moduleDef.name), entityId, "HAS");

      for (const implementedName of entity.implements) {
        const targetId = knownTargets.get(implementedName) ?? nodeId("Interface", implementedName);
        addRelationship(entityId, targetId, "IMPLEMENTS");
      }

      entity.fields.forEach((field, index) => {
        const fieldId = fieldNodeId(entity, field);
        nodes.set(fieldId, {
          id: fieldId,
          label: "Field",
          properties: {
            id: fieldId,
            name: field.name,
            emoji: emojiForLabel("Field", field.name),
            kind: field.kind,
            targetName: field.targetName,
            isList: field.isList,
            ownerName: entity.name,
            ownerKind: entity.kind,
            fieldOrder: String(index + 1),
            schemaSource: SCHEMA_SOURCE,
            schemaTag: SCHEMA_TAG,
          },
        });
        addRelationship(entityId, fieldId, "HAS");

        const targetId = knownTargets.get(field.targetName);
        if (targetId) {
          addRelationship(fieldId, targetId, "TARGETS");
        }
      });

      entity.constraints.forEach((description, index) => {
        const constraintId = constraintNodeId(entity, index);
        nodes.set(constraintId, {
          id: constraintId,
          label: "Constraint",
          properties: {
            id: constraintId,
            name: `${entity.name} constraint ${index + 1}`,
            emoji: emojiForLabel("Constraint", entity.name),
            description,
            ownerName: entity.name,
            ownerKind: entity.kind,
            schemaSource: SCHEMA_SOURCE,
            schemaTag: SCHEMA_TAG,
          },
        });
        addRelationship(entityId, constraintId, "HAS");
      });
    }
  }

  return {
    nodes: [...nodes.values()].sort((left, right) => left.id.localeCompare(right.id)),
    relationships: [...relationships.values()].sort((left, right) => {
      const leftKey = `${left.kind}|${left.from}|${left.to}`;
      const rightKey = `${right.kind}|${right.from}|${right.to}`;
      return leftKey.localeCompare(rightKey);
    }),
  };
}
//#endregion 🏗️GraphBuild

//#region 📝CypherRender
function cypherLiteral(value: string | boolean): string {
  if (typeof value === "boolean") {
    return value ? "true" : "false";
  }

  return `'${value.replace(/\\/g, "\\\\").replace(/'/g, "\\'")}'`;
}

function renderPropertyMap(properties: Record<string, string | boolean>): string {
  return `{ ${Object.entries(properties)
    .map(([key, value]) => `${key}: ${cypherLiteral(value)}`)
    .join(", ")} }`;
}

function renderCypher(nodes: GraphNode[], relationships: GraphRelationship[]): string {
  const lines = [
    "// SPDX-License-Identifier: AGPL-3.0-only",
    "// Neo4j Cypher persistence for semio.",
    "// Keep this file replayable with cypher-shell or APOC.",
    `// Generated by bun ./generate.script.ts from ${SCHEMA_SOURCE}.`,
    "",
    `MATCH (n { schemaTag: ${cypherLiteral(SCHEMA_TAG)} }) DETACH DELETE n;`,
    "",
  ];

  for (const node of nodes) {
    lines.push(`MERGE (n:${node.label} { id: ${cypherLiteral(node.id)} })`);
    lines.push(`SET n = ${renderPropertyMap(node.properties)};`);
    lines.push("");
  }

  for (const relationship of relationships) {
    lines.push(
      `MATCH (from { id: ${cypherLiteral(relationship.from)} }), (to { id: ${cypherLiteral(relationship.to)} }) MERGE (from)-[:${relationship.kind}]->(to);`,
    );
  }

  lines.push("");
  return `${lines.join("\n")}\n`;
}
//#endregion 📝CypherRender

//#region 🚀Runtime
function resolveCypherShell(): string | null {
  const runtimeName = process.platform === "win32" ? "cypher-shell.bat" : "cypher-shell";
  const cachedShell = join(REPO_ROOT, ".repo", "cache", "neo4j", `neo4j-community-${NEO4J_VERSION}`, "bin", runtimeName);
  const candidates = [process.env.NEO4J_CYPHER_SHELL, cachedShell, runtimeName].filter((value): value is string => Boolean(value));

  for (const candidate of candidates) {
    if (candidate.includes("/") || candidate.includes("\\")) {
      if (existsSync(candidate)) {
        return candidate;
      }
      continue;
    }

    const probe = spawnSync(candidate, ["--version"], { stdio: "ignore" });
    if (probe.status === 0) {
      return candidate;
    }
  }

  return null;
}

function buildCypherEnv(): NodeJS.ProcessEnv {
  const env = {
    ...process.env,
  };

  if (process.platform === "win32") {
    const javaHome = "C:\\Program Files\\Microsoft\\jdk-21.0.11.10-hotspot";
    const javaExecutable = join(javaHome, "bin", "java.exe");
    if (existsSync(javaExecutable)) {
      env.JAVA_HOME = javaHome;
      env.Path = `${join(javaHome, "bin")};${env.Path || ""}`;
    }
  }

  return env;
}

function applyCypherFile(outputPath: string): void {
  const cypherShell = resolveCypherShell();
  if (!cypherShell) {
    console.warn(`[generate] cypher-shell not found. Wrote ${OUTPUT_FILE} but did not apply it to Neo4j.`);
    return;
  }

  const cypherInput = readFileSync(outputPath, "utf8");

  const result = spawnSync(
    cypherShell,
    [
      "-a",
      process.env.NEO4J_URI || "bolt://localhost:7687",
      "-u",
      process.env.NEO4J_USERNAME || "neo4j",
      "-p",
      process.env.NEO4J_PASSWORD || "password",
      "-d",
      process.env.NEO4J_DATABASE || "neo4j",
      "--format",
      "plain",
    ],
    {
      cwd: REPO_ROOT,
      input: cypherInput,
      stdio: "inherit",
      env: buildCypherEnv(),
    },
  );

  if (result.status !== 0) {
    throw new Error(`cypher-shell failed with exit code ${result.status ?? 1}`);
  }
}

function main(): void {
  const schemaPath = join(REPO_ROOT, SCHEMA_SOURCE);
  const outputPath = join(REPO_ROOT, OUTPUT_FILE);
  const rawSchema = readFileSync(schemaPath, "utf8");
  const model = parseSchemaSketch(rawSchema);
  const graph = buildGraph(model);
  const cypher = renderCypher(graph.nodes, graph.relationships);

  mkdirSync(join(REPO_ROOT, ".repo", "🛂"), { recursive: true });
  writeFileSync(outputPath, cypher, "utf8");
  applyCypherFile(outputPath);

  console.log(
    `[generate] wrote ${OUTPUT_FILE} with ${graph.nodes.length} nodes and ${graph.relationships.length} relationships from ${SCHEMA_SOURCE}.`,
  );
}

main();
//#endregion 🚀Runtime