// SPDX-License-Identifier: AGPL-3.0-only
// 🛂 Hand-maintained Neo4j replay bundle — edit in Neo4j Browser/Bloom or Cypher; refresh with `bun ./export.neo4j.script.ts`.
// technology: semio | database: semio | exported: 2026-05-14T21:05:28.383Z
//

//#region 🔍IndexesForSearch
// Neo4j Browser / Bloom “full text search” requires a FULLTEXT index (RANGE indexes on `name` are not enough).
CREATE RANGE INDEX index_class_name IF NOT EXISTS FOR (n:Class) ON (n.name);
CREATE RANGE INDEX index_constraint_name IF NOT EXISTS FOR (n:Constraint) ON (n.name);
CREATE RANGE INDEX index_field_name IF NOT EXISTS FOR (n:Field) ON (n.name);
CREATE RANGE INDEX index_interface_name IF NOT EXISTS FOR (n:Interface) ON (n.name);
CREATE RANGE INDEX index_module_name IF NOT EXISTS FOR (n:Module) ON (n.name);
CREATE RANGE INDEX index_scalar_name IF NOT EXISTS FOR (n:Scalar) ON (n.name);
CREATE RANGE INDEX index_enum_name IF NOT EXISTS FOR (n:Enum) ON (n.name);
// Neo4j 5: if `semio_name_fulltext` already exists with an older label set, drop it in Browser before replaying an updated definition that adds `Enum`.
CREATE FULLTEXT INDEX semio_name_fulltext IF NOT EXISTS FOR (n:Class|Constraint|Field|Interface|Module|Scalar|Enum) ON EACH [n.name];
//#endregion 🔍IndexesForSearch

//#region 🔗MaterializeTransitiveIs
// After direct `IS` subtype edges exist, merge transitive closure so every subtype `IS` every ancestor (e.g. Class→strongEntity also `IS` entity; Interface→richStrongEntity also `IS` entity).
MATCH (a:Interface)-[:IS*1..25]->(b:Interface) MERGE (a)-[:IS]->(b);
MATCH (c:Class)-[:IS]->(i:Interface) MATCH (i)-[:IS*1..25]->(j:Interface) MERGE (c)-[:IS]->(j);
MATCH (f:Field)-[:IS]->(i:Interface) MATCH (i)-[:IS*1..25]->(j:Interface) MERGE (f)-[:IS]->(j);
MATCH (f:Field)-[:IS]->(c:Class) MATCH (c)-[:IS*1..25]->(j:Interface) MERGE (f)-[:IS]->(j);
//#endregion 🔗MaterializeTransitiveIs

//#region 📦ModuleHasPrimitives
// Own every Interface/Scalar under the `graphql` module tree (sibling-unique `Module.name`); remove duplicate containment from schema-level `general` / `domain` modules.
MATCH (root:Module {name: 'graphql'})-[:HAS]->(entity:Module {name: 'Entity'})
UNWIND ['entity', 'weakEntity', 'data', 'strongEntity', 'richStrongEntity', 'artifact', 'document', 'event'] AS nm
MATCH (i:Interface {name: nm})
MERGE (entity)-[:HAS]->(i);
MATCH (root:Module {name: 'graphql'})-[:HAS]->(scalarMod:Module {name: 'Scalar'})
UNWIND ['string', 'number', 'boolean', 'timestamp', 'color', 'uri'] AS nm
MATCH (s:Scalar {name: nm})
MERGE (scalarMod)-[:HAS]->(s);
MATCH (root:Module {name: 'graphql'})-[:HAS*1..15]->(gen:Module {name: 'General'})-[:HAS]->(m:Module {name: 'Command'})
MATCH (iface:Interface {name: 'command'})
MERGE (m)-[:HAS]->(iface);
MATCH (root:Module {name: 'graphql'})-[:HAS*1..15]->(gen:Module {name: 'General'})-[:HAS]->(m:Module {name: 'Workspace'})
MATCH (i:Interface {name: 'workspace'})
MERGE (m)-[:HAS]->(i);
MATCH (root:Module {name: 'graphql'})-[:HAS*1..15]->(gen:Module {name: 'General'})-[:HAS]->(m:Module {name: 'Diff'})
MATCH (i:Interface {name: 'diff'})
MERGE (m)-[:HAS]->(i);
MATCH (root:Module {name: 'graphql'})-[:HAS*1..15]->(gen:Module {name: 'General'})-[:HAS]->(m:Module {name: 'Modification'})
MATCH (i:Interface {name: 'modification'})
MERGE (m)-[:HAS]->(i);
MATCH (root:Module {name: 'graphql'})-[:HAS*1..15]->(gen:Module {name: 'General'})-[:HAS]->(m:Module {name: 'Operation'})
MATCH (i:Interface {name: 'operation'})
MERGE (m)-[:HAS]->(i);
MATCH (root:Module {name: 'graphql'})-[:HAS*1..15]->(gen:Module {name: 'General'})-[:HAS]->(m:Module {name: 'Backbone'})
MATCH (i:Interface {name: 'backbone'})
MERGE (m)-[:HAS]->(i);
MATCH (root:Module {name: 'graphql'})-[:HAS*1..15]->(gen:Module {name: 'General'})-[:HAS]->(m:Module {name: 'Provider'})
MATCH (i:Interface {name: 'provider'})
MERGE (m)-[:HAS]->(i);
MATCH (old:Module)-[r:HAS]->(x)
WHERE old.name IN ['general', 'domain'] AND (x:Interface OR x:Scalar)
DELETE r;
MATCH (root:Module {name: 'graphql'})
MERGE (root)-[:HAS]->(fk:Module {name: 'FieldKind'})
WITH fk
UNWIND ['EMBEDDED', 'REFERENCE', 'COMPUTED', 'CACHED'] AS nm
MERGE (e:Enum {name: nm})
MERGE (fk)-[:HAS]->(e);
//#endregion 📦ModuleHasPrimitives
