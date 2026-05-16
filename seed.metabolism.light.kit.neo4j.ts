/**
 * 🧬Loads `wip.initialKit` from `semio/assets/fixtures/metabolism.kit.light.semio.json` into the Neo4j `metabolism` graph using only `HAS`, `IS`, and `REFERENCES` (no VCS/history fields).
 * @see https://neo4j.com/docs/cypher-manual/current/clauses/match/
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { Neo4jCypherExport } from "./generate.neo4j.gen.ts";

//#region 🔖Escape
/** 🔒Escapes a string for single-quoted Cypher literals. */
function cypherQuote(value: string): string {
  return `'${value.replace(/\\/g, "\\\\").replace(/'/g, "\\'")}'`;
}
//#endregion 🔖Escape

//#region 🔖Types
type KitConnector = { id: string; name?: string; port?: { id: string } };
type KitType = { id: string; name?: string; isAbstract?: boolean; connectors?: { items: KitConnector[] } };
type KitPiece = { id: string; name?: string; type: { id: string } };
type KitConnection = {
  id: string;
  parent: { piece: { id: string }; connector: { id: string } };
  child: { piece: { id: string }; connector: { id: string } };
};
type KitDesign = { id: string; name?: string; pieces: { items: KitPiece[] }; connections: { items: KitConnection[] } };
type KitFamily = { id: string; name?: string; ports?: { items: { id: string; name?: string }[] } };
type InitialKit = {
  types: { items: KitType[] };
  families: { items: KitFamily[] };
  designs: { items: KitDesign[] };
};
//#endregion 🔖Types

//#region 🔖CypherChunks
function unwindTypes(types: KitType[]): string {
  const maps = types.map((t) => {
    const name = t.name != null ? cypherQuote(t.name) : "null";
    const abs = t.isAbstract === true ? "true" : "false";
    return `{id: ${cypherQuote(t.id)}, name: ${name}, isAbstract: ${abs}}`;
  });
  return [
    `UNWIND [${maps.join(", ")}] AS row`,
    `MERGE (n:Type {id: row.id})`,
    `SET n.name = coalesce(row.name, n.name), n.isAbstract = row.isAbstract;`,
  ].join("\n");
}

function unwindPorts(families: KitFamily[]): string {
  const rows: { id: string; name: string | null }[] = [];
  for (const fam of families) {
    for (const p of fam.ports?.items ?? []) {
      rows.push({ id: p.id, name: p.name ?? null });
    }
  }
  if (rows.length === 0) return "";
  const maps = rows.map((r) => `{id: ${cypherQuote(r.id)}, name: ${r.name != null ? cypherQuote(r.name) : "null"}}`);
  return [`UNWIND [${maps.join(", ")}] AS row`, `MERGE (n:Port {id: row.id})`, `SET n.name = coalesce(row.name, n.name);`].join("\n");
}

function unwindConnectors(types: KitType[]): { all: string; withPort: string } {
  const rowsAll: { typeId: string; connectorId: string; name: string | null }[] = [];
  const rowsPort: { connectorId: string; portId: string }[] = [];
  for (const t of types) {
    for (const c of t.connectors?.items ?? []) {
      rowsAll.push({ typeId: t.id, connectorId: c.id, name: c.name ?? null });
      if (c.port?.id) rowsPort.push({ connectorId: c.id, portId: c.port.id });
    }
  }
  const allMaps = rowsAll.map(
    (r) =>
      `{typeId: ${cypherQuote(r.typeId)}, connectorId: ${cypherQuote(r.connectorId)}, name: ${r.name != null ? cypherQuote(r.name) : "null"}}`,
  );
  const allCypher =
    rowsAll.length === 0
      ? ""
      : [
          `UNWIND [${allMaps.join(", ")}] AS row`,
          `MATCH (ty:Type {id: row.typeId})`,
          `MERGE (co:Connector {id: row.connectorId})`,
          `SET co.name = row.name`,
          `MERGE (ty)-[:HAS]->(co);`,
        ].join("\n");
  const portMaps = rowsPort.map(
    (r) => `{connectorId: ${cypherQuote(r.connectorId)}, portId: ${cypherQuote(r.portId)}}`,
  );
  const portCypher =
    rowsPort.length === 0
      ? ""
      : [
          `UNWIND [${portMaps.join(", ")}] AS row`,
          `MATCH (co:Connector {id: row.connectorId})`,
          `MATCH (po:Port {id: row.portId})`,
          `MERGE (co)-[:IS]->(po);`,
        ].join("\n");
  return { all: allCypher, withPort: portCypher };
}

function unwindDesigns(designs: KitDesign[]): string {
  const maps = designs.map((d) => {
    const name = d.name != null ? cypherQuote(d.name) : "null";
    return `{id: ${cypherQuote(d.id)}, name: ${name}}`;
  });
  return [`UNWIND [${maps.join(", ")}] AS row`, `MERGE (d:Design {id: row.id})`, `SET d.name = coalesce(row.name, d.name);`].join("\n");
}

function unwindPieces(designs: KitDesign[]): string {
  const rows: { designId: string; pieceId: string; pieceName: string | null; typeId: string; blueprintId: string }[] = [];
  for (const d of designs) {
    for (const p of d.pieces.items) {
      rows.push({
        designId: d.id,
        pieceId: p.id,
        pieceName: p.name ?? null,
        typeId: p.type.id,
        blueprintId: `${p.id}/blueprint`,
      });
    }
  }
  const maps = rows.map(
    (r) =>
      `{designId: ${cypherQuote(r.designId)}, pieceId: ${cypherQuote(r.pieceId)}, pieceName: ${r.pieceName != null ? cypherQuote(r.pieceName) : "null"}, typeId: ${cypherQuote(r.typeId)}, blueprintId: ${cypherQuote(r.blueprintId)}}`,
  );
  return [
    `UNWIND [${maps.join(", ")}] AS row`,
    `MATCH (d:Design {id: row.designId})`,
    `MERGE (pi:Piece {id: row.pieceId})`,
    `SET pi.name = coalesce(row.pieceName, pi.name)`,
    `MERGE (d)-[:HAS]->(pi)`,
    `MERGE (b:Blueprint {id: row.blueprintId})`,
    `MERGE (pi)-[:HAS]->(b)`,
    `WITH b, row`,
    `MATCH (ty:Type {id: row.typeId})`,
    `MERGE (b)-[:IS]->(ty);`,
  ].join("\n");
}

function unwindConnections(designs: KitDesign[]): string {
  const rows: {
    designId: string;
    connectionId: string;
    parentConnectorId: string;
    childConnectorId: string;
    sideParentId: string;
    sideChildId: string;
  }[] = [];
  for (const d of designs) {
    for (const c of d.connections.items) {
      rows.push({
        designId: d.id,
        connectionId: c.id,
        parentConnectorId: c.parent.connector.id,
        childConnectorId: c.child.connector.id,
        sideParentId: `${c.id}/side/parent`,
        sideChildId: `${c.id}/side/child`,
      });
    }
  }
  const maps = rows.map(
    (r) =>
      `{designId: ${cypherQuote(r.designId)}, connectionId: ${cypherQuote(r.connectionId)}, parentConnectorId: ${cypherQuote(r.parentConnectorId)}, childConnectorId: ${cypherQuote(r.childConnectorId)}, sideParentId: ${cypherQuote(r.sideParentId)}, sideChildId: ${cypherQuote(r.sideChildId)}}`,
  );
  return [
    `UNWIND [${maps.join(", ")}] AS row`,
    `MATCH (d:Design {id: row.designId})`,
    `MERGE (cn:Connection {id: row.connectionId})`,
    `MERGE (d)-[:HAS]->(cn)`,
    `MERGE (sp:Side {id: row.sideParentId})`,
    `MERGE (sc:Side {id: row.sideChildId})`,
    `MERGE (cn)-[:HAS {parent: true}]->(sp)`,
    `MERGE (cn)-[:HAS {parent: false}]->(sc)`,
    `WITH sp, sc, row`,
    `MATCH (cop:Connector {id: row.parentConnectorId})`,
    `MATCH (coc:Connector {id: row.childConnectorId})`,
    `MERGE (sp)-[:REFERENCES]->(cop)`,
    `MERGE (sc)-[:REFERENCES]->(coc);`,
  ].join("\n");
}
//#endregion 🔖CypherChunks

//#region 🔖Entry
/** 🧬Clears the target database and loads the light metabolism kit graph (HAS/IS/REFERENCES only). */
export function seedMetabolismLightKitNeo4j(repoRoot: string): void {
  const jsonPath = join(repoRoot, "semio", "assets", "fixtures", "metabolism.kit.light.semio.json");
  const raw = JSON.parse(readFileSync(jsonPath, "utf8")) as { wip?: { initialKit?: InitialKit } };
  const kit = raw.wip?.initialKit;
  if (!kit?.types?.items || !kit.designs?.items) {
    throw new Error("[seed.metabolism.light.kit.neo4j] missing wip.initialKit in fixture JSON.");
  }
  const database = process.env.NEO4J_DATABASE || "metabolism";
  const exporter = new Neo4jCypherExport(repoRoot);

  const constraints = [
    "CREATE CONSTRAINT design_id IF NOT EXISTS FOR (n:Design) REQUIRE n.id IS UNIQUE;",
    "CREATE CONSTRAINT piece_id IF NOT EXISTS FOR (n:Piece) REQUIRE n.id IS UNIQUE;",
    "CREATE CONSTRAINT blueprint_id IF NOT EXISTS FOR (n:Blueprint) REQUIRE n.id IS UNIQUE;",
    "CREATE CONSTRAINT type_id IF NOT EXISTS FOR (n:Type) REQUIRE n.id IS UNIQUE;",
    "CREATE CONSTRAINT connector_id IF NOT EXISTS FOR (n:Connector) REQUIRE n.id IS UNIQUE;",
    "CREATE CONSTRAINT port_id IF NOT EXISTS FOR (n:Port) REQUIRE n.id IS UNIQUE;",
    "CREATE CONSTRAINT connection_id IF NOT EXISTS FOR (n:Connection) REQUIRE n.id IS UNIQUE;",
    "CREATE CONSTRAINT side_id IF NOT EXISTS FOR (n:Side) REQUIRE n.id IS UNIQUE;",
  ].join("\n");

  const { all: connAll, withPort: connPort } = unwindConnectors(kit.types.items);
  const steps: string[] = [
    "MATCH (n) DETACH DELETE n;",
    constraints,
    unwindTypes(kit.types.items),
    unwindPorts(kit.families?.items ?? []),
    connAll,
    connPort,
    unwindDesigns(kit.designs.items),
    unwindPieces(kit.designs.items),
    unwindConnections(kit.designs.items),
  ];

  for (let i = 0; i < steps.length; i++) {
    const cypher = steps[i]!.trim();
    if (!cypher) continue;
    const { ok, stdout, stderr } = exporter.runCypher(database, cypher);
    if (!ok) {
      console.error(`[seed.metabolism.light.kit.neo4j] step ${i + 1} failed:\n${stderr || stdout}`);
      process.exit(1);
    }
  }
  console.log(`[seed.metabolism.light.kit.neo4j] loaded into database ${JSON.stringify(database)}.`);
}
//#endregion 🔖Entry
