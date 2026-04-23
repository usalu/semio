#!/usr/bin/env tsx
// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

// #region 📷Test Runner
import { execFileSync, execSync } from "child_process";
import { existsSync, readFileSync } from "fs";
import { join } from "path";

const env = { ...process.env, UV_PROJECT_ENVIRONMENT: join(__dirname, ".venv") };
const python = process.platform === "win32" ? join(__dirname, ".venv", "Scripts", "python.exe") : join(__dirname, ".venv", "bin", "python");
const pythonCommand = existsSync(python) ? python : "python";

// #region 🔗SchemaValidation

const assertSchema = (condition: boolean, message: string): void => {
  if (!condition) throw new Error(message);
};

const definitionBody = (definition: string): string => {
  const escapedDefinition = definition.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const match = graphqlSchema.match(new RegExp(`${escapedDefinition}\\s*\\{([\\s\\S]*?)\\n\\}`));
  assertSchema(Boolean(match), `${definition} MUST exist`);
  return match![1];
};

const graphqlSchema = readFileSync(join(__dirname, "..", "graphql", "schema.graphql"), "utf8");
const disallowed = "leg" + "acy";
assertSchema(!graphqlSchema.includes(disallowed), "semio GraphQL schema MUST NOT add backward-only compatibility field names in identifiers");
for (const definition of ["type KitStore", "type KitSession", "type KitChangeCandidate", "type KitConflict", "type Type", "type Design", "type Port", "type Piece", "type Connection"]) {
  assertSchema(definitionBody(definition).includes("hash: Hash!"), `${definition} MUST expose a computed hash`);
}
assertSchema(!/\bFamily\b/.test(graphqlSchema), "semio GraphQL schema MUST NOT expose Family because semio/rs has no Family DTO");
const kitBody = definitionBody("type Kit");
assertSchema(!/\n  (families|ports):/.test(kitBody), "Kit MUST NOT expose kit-level families or ports");
assertSchema(kitBody.includes("version: String") && kitBody.includes("uri: String") && kitBody.includes("props: [Prop!]!"), "Kit MUST expose Rust KitFullDto version, uri, and props fields");
const typeBody = definitionBody("type Type");
assertSchema(typeBody.includes("ports: [Port!]!"), "Type MUST own ports to match semio/rs TypeFullDto");
assertSchema(!typeBody.includes("parent:"), "Type MUST NOT expose removed parent links");
assertSchema(!definitionBody("type Design").includes("parent:"), "Design MUST NOT expose removed parent links");
const coordinateBody = definitionBody("type Coordinate");
assertSchema(coordinateBody.includes("x: Float!") && coordinateBody.includes("y: Float!") && coordinateBody.includes("z: Float!"), "Coordinate MUST be the Rust 3D coordinate shape");
assertSchema(graphqlSchema.includes("union KitInteractionRecord"), "semio GraphQL schema MUST expose dedicated kit interaction records");
assertSchema(graphqlSchema.includes("replacementTypeCandidates"), "semio GraphQL schema MUST expose computed replacement type candidates");

execFileSync(pythonCommand, ["-c", "from pathlib import Path; from ariadne import gql, make_executable_schema; s=Path('../graphql/schema.graphql').read_text(encoding='utf-8'); make_executable_schema(gql(s))"], {
  cwd: __dirname,
  env,
  stdio: "inherit",
});

const openapiSchema = JSON.parse(readFileSync(join(__dirname, "..", "openapi", "schema.json"), "utf8"));
assertSchema(Boolean(openapiSchema.paths?.["/api/graphql"]?.post), "semio OpenAPI schema MUST expose the GraphQL store endpoint");
assertSchema(Boolean(openapiSchema.components?.schemas?.GraphqlStoreRequest), "semio OpenAPI schema MUST define GraphqlStoreRequest");
assertSchema(Boolean(openapiSchema.components?.schemas?.GraphqlStoreResponse), "semio OpenAPI schema MUST define GraphqlStoreResponse");

// #endregion 🔗SchemaValidation

execFileSync(pythonCommand, ["-m", "pytest", "--cov", "--cov-config=pyproject.toml", "--cov-report", "html"], {
  cwd: __dirname,
  env,
  stdio: "inherit",
});

console.log("✅ Tests complete");

// #endregion 📷Test Runner
