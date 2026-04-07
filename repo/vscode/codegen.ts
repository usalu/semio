// #region 🔖Header
// [🧰repo🖱️vscode💻codegen](repo://p/i/repo/b/u/vscode/f/codegen.ts)

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Affero General Public License for more details. You should have received a copy of the GNU Affero General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Code generation script for VS Code extension GraphQL types.

// #endregion 🔖Header

// #region 🔖Configuration
// [🧰repo🖱️vscode💻codegen🔖configuration](repo://p/i/repo/b/u/vscode/f/codegen.ts/s/Configuration)
// GraphQL code generation configuration for the VS Code extension.
// Configuration MUST generate typed client code from the GraphQL schema.

import type { CodegenConfig } from "@graphql-codegen/cli";

/**
 * GraphQL codegen configuration targeting the schema and query documents.
// [🧰repo🖱️vscode💻codegen🔖configuration🪨config](repo://p/i/repo/b/u/vscode/f/codegen.ts/s/Configuration/d/i/config)
 * Config MUST reference the GraphQL schema and generate client preset output.
 **/
const config: CodegenConfig = {
  schema: "../graphql/schema.graphql",
  documents: ["queries.ts"],
  generates: {
    "./generated/": {
      preset: "client",
      plugins: [],
      presetConfig: {
        gqlTagName: "graphql",
      },
    },
  },
  ignoreNoDocuments: true,
};

export default config;
// #endregion 🔖Configuration
