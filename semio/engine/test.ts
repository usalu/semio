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
assertSchema(!graphqlSchema.includes(disallowed), "semio GraphQL schema MUST NOT add compatibility-only field names in identifiers or generated descriptions");
for (const definition of [
  "type KitStore",
  "type Query",
  "type Mutation",
  "type Subscription",
  "type TypeStore",
  "type DesignStore",
  "type PieceStore",
  "type ConnectionStore",
]) {
  definitionBody(definition);
}
const queryBody = definitionBody("type Query");
assertSchema(
  queryBody.includes("kit(scope: KitReadScopeInput!): KitStore!") && !queryBody.includes("kitReadScope"),
  "Query MUST expose kit(scope:) root read",
);
const mutationBody = definitionBody("type Mutation");
assertSchema(mutationBody.includes("kitStore: KitStoreMutation!"), "Mutation MUST expose nested kit store mutations");
const kitStoreMutationBody = definitionBody("type KitStoreMutation");
assertSchema(kitStoreMutationBody.includes("batch(input: KitStoreInput!): KitStorePayload!"), "KitStoreMutation MUST expose batched scoped kit writes");
const subscriptionBody = definitionBody("type Subscription");
assertSchema(subscriptionBody.includes("eventStream: KitEvent!"), "Subscription MUST expose the Rust event stream");
const kitBody = definitionBody("type KitStore");
assertSchema(
  kitBody.includes("fullDto: KitFullSnapshot!") && kitBody.includes("typeByDtoId(id: String!): TypeStore") && kitBody.includes("designByDtoId(id: String!): DesignStore"),
  "KitStore MUST expose the current semio/rs live graph API",
);
const typeBody = definitionBody("type TypeStore");
assertSchema(
  typeBody.includes("connectors: [ConnectorStore!]!") && typeBody.includes("representations: [RepresentationStore!]!"),
  "TypeStore MUST expose the Rust catalog handles",
);
const designBody = definitionBody("type DesignStore");
assertSchema(
  designBody.includes("clusterableGroups(selection: [String!]!): [[String!]!]!") && designBody.includes("replaceableCatalog(selection: [String!]!): ReplaceableCatalogStore!"),
  "DesignStore MUST expose computed semio/rs graph operations",
);
assertSchema(graphqlSchema.includes("input KitReadScopeInput @oneOf"), "semio GraphQL schema MUST expose Rust read scopes as one-of input");

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
