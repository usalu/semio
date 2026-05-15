#!/usr/bin/env bun
/**
 * @deprecated Prefer `bun run generate:neo4j` or `bun ./generate.neo4j.script.ts`. This file is a thin alias so historic `export:neo4j` / `bun ./export.neo4j.script.ts` keep working.
 */
import { runGenerateNeo4jFromLiveDatabase } from "./generate.neo4j.script.ts";

runGenerateNeo4jFromLiveDatabase();
