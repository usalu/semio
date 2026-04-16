#!/usr/bin/env tsx
// #region 🧲Header

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🧲Header

// #region 📷Test Runner
import { execFileSync, execSync } from "child_process";
import { readFileSync } from "fs";
import { join } from "path";

const env = { ...process.env, UV_PROJECT_ENVIRONMENT: join(__dirname, ".venv") };

// #region 🔗SchemaValidation

const assertSchema = (condition: boolean, message: string): void => {
  if (!condition) throw new Error(message);
};

const graphqlSchema = readFileSync(join(__dirname, "..", "graphql", "schema.graphql"), "utf8");
assertSchema(!graphqlSchema.includes("legacy"), "semio GraphQL schema MUST NOT expose legacy compatibility fields");
for (const definition of ["type KitStore", "type KitSession", "type KitChangeCandidate", "type KitConflict", "type Family", "type Type", "type Design", "type Piece", "type Connection"]) {
  const escapedDefinition = definition.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  assertSchema(new RegExp(`${escapedDefinition}\\s*\\{[\\s\\S]*?\\n  hash: Hash!`).test(graphqlSchema), `${definition} MUST expose a computed hash`);
}
assertSchema(graphqlSchema.includes("union KitInteractionRecord"), "semio GraphQL schema MUST expose dedicated kit interaction records");
assertSchema(graphqlSchema.includes("replacementTypeCandidates"), "semio GraphQL schema MUST expose computed replacement type candidates");

execFileSync(
  "uv",
  [
    "run",
    "python",
    "-c",
    "from pathlib import Path; from ariadne import gql, make_executable_schema; s=Path('../graphql/schema.graphql').read_text(encoding='utf-8'); make_executable_schema(gql(s))",
  ],
  { cwd: __dirname, env, stdio: "inherit" },
);

const openapiSchema = JSON.parse(readFileSync(join(__dirname, "..", "openapi", "schema.json"), "utf8"));
assertSchema(Boolean(openapiSchema.paths?.["/api/graphql"]?.post), "semio OpenAPI schema MUST expose the GraphQL store endpoint");
assertSchema(Boolean(openapiSchema.components?.schemas?.GraphqlStoreRequest), "semio OpenAPI schema MUST define GraphqlStoreRequest");
assertSchema(Boolean(openapiSchema.components?.schemas?.GraphqlStoreResponse), "semio OpenAPI schema MUST define GraphqlStoreResponse");

// #endregion 🔗SchemaValidation

execSync("uv run pytest --cov --cov-config=pyproject.toml --cov-report html", {
  cwd: __dirname,
  env,
  stdio: "inherit",
});

console.log("✅ Tests complete");

// #endregion 📷Test Runner
