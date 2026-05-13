#!/usr/bin/env bun
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
assertSchema(!graphqlSchema.includes(disallowed), "semio GraphQL schema MUST NOT add compatibility-only field names in identifiers or generated descriptions");
for (const definition of [
  "type Graph",
  "type Kit",
  "type Query",
  "type Mutation",
  "type Subscription",
  "type Type",
  "type Design",
  "type Piece",
  "type Connection",
  "input KitReadPointInput",
]) {
  definitionBody(definition);
}
const graphBody = definitionBody("type Graph");
assertSchema(
  graphBody.includes("theKit(at: KitReadPointInput): Kit") && !graphBody.includes("kitReadScope"),
  "Graph MUST expose theKit(at:) materialized reads gated by KitReadPointInput",
);
const kitReadPointBody = definitionBody("input KitReadPointInput");
for (const field of ["theKit:", "checkpointId:", "checkpointChangeId:", "checkpointOperationId:", "alternativeId:", "draftAlternativeId:", "draftId:", "draftChangeId:", "draftTransactionId:", "draftOperationId:"]) {
  assertSchema(kitReadPointBody.includes(field), `KitReadPointInput MUST expose ${field}`);
}
const mutationBody = definitionBody("type Mutation");
assertSchema(mutationBody.includes("renameKit("), "Mutation MUST expose renameKit");
assertSchema(mutationBody.includes("addFixedPieceToDesign(") && mutationBody.includes("dragPieceInDesign("), "Mutation MUST expose flat draft/transaction-scoped kit mutators");
const subscriptionBody = definitionBody("type Subscription");
assertSchema(subscriptionBody.includes("commandSucceeded: Command!") && subscriptionBody.includes("operationSucceeded: OperationKind!"), "Subscription MUST expose Rust command/operation streams");

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
