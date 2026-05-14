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
CREATE FULLTEXT INDEX semio_name_fulltext IF NOT EXISTS FOR (n:Class|Constraint|Field|Interface|Module|Scalar) ON EACH [n.name];
//#endregion 🔍IndexesForSearch

//#region 🔗MaterializeTransitiveIs
// After direct `IS` subtype edges exist, merge transitive closure so every subtype `IS` every ancestor (e.g. Class→strongEntity also `IS` entity; Interface→richStrongEntity also `IS` entity).
MATCH (a:Interface)-[:IS*1..25]->(b:Interface) MERGE (a)-[:IS]->(b);
MATCH (c:Class)-[:IS]->(i:Interface) MATCH (i)-[:IS*1..25]->(j:Interface) MERGE (c)-[:IS]->(j);
MATCH (f:Field)-[:IS]->(i:Interface) MATCH (i)-[:IS*1..25]->(j:Interface) MERGE (f)-[:IS]->(j);
MATCH (f:Field)-[:IS]->(c:Class) MATCH (c)-[:IS*1..25]->(j:Interface) MERGE (f)-[:IS]->(j);
//#endregion 🔗MaterializeTransitiveIs
