// #region Header

// js/vscode/graphql.ts

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

// #region Imports

import { Client, cacheExchange, fetchExchange, type TypedDocumentNode } from "@urql/core";
import { exec } from "child_process";
import { promisify } from "util";

const execAsync = promisify(exec);

// #endregion Imports

// #region Client Setup

let urqlClient: Client | null = null;

export function getUrqlClient(workspaceRoot: string, repoCommand: string): Client {
  if (!urqlClient) {
    urqlClient = new Client({
      url: "local://graphql",
      exchanges: [cacheExchange, fetchExchange],
      fetch: async (input: RequestInfo | URL, init?: RequestInit) => {
        const request = typeof input === "string" ? JSON.parse(input) : input;
        const body = init?.body ? JSON.parse(init.body as string) : {};
        const query = body.query as string;
        const variables = body.variables || {};

        const variablesJson = Object.keys(variables).length > 0 ? JSON.stringify(variables) : "";
        const escapedQuery = query.replace(/"/g, '\\"').replace(/\n/g, " ");
        const escapedVariables = variablesJson ? variablesJson.replace(/"/g, '\\"') : "";
        const fullCommand = escapedVariables
          ? `"${repoCommand}" graphql "${escapedQuery}" -v "${escapedVariables}"`
          : `"${repoCommand}" graphql "${escapedQuery}"`;

        try {
          const { stdout, stderr } = await execAsync(fullCommand, { cwd: workspaceRoot, timeout: 60000, maxBuffer: 10 * 1024 * 1024 });
          if (stderr) {
            console.log("[urql] stderr:", stderr.substring(0, 500));
          }
          const data = JSON.parse(stdout);
          return new Response(JSON.stringify({ data }), {
            status: 200,
            headers: { "Content-Type": "application/json" },
          });
        } catch (error) {
          console.error("[urql] error:", error);
          return new Response(JSON.stringify({ errors: [{ message: String(error) }] }), {
            status: 500,
            headers: { "Content-Type": "application/json" },
          });
        }
      },
    });
  }
  return urqlClient;
}

export function resetUrqlClient(): void {
  urqlClient = null;
}

// #endregion Client Setup

// #region Helper Functions

export async function query<TData, TVariables extends Record<string, unknown> = Record<string, unknown>>(
  client: Client,
  document: TypedDocumentNode<TData, TVariables>,
  variables?: TVariables,
): Promise<TData | null> {
  const result = await client.query(document, variables || ({} as TVariables));
  if (result.error) {
    console.error("[urql] query error:", result.error);
    return null;
  }
  return result.data || null;
}

export async function mutation<TData, TVariables extends Record<string, unknown> = Record<string, unknown>>(
  client: Client,
  document: TypedDocumentNode<TData, TVariables>,
  variables: TVariables,
): Promise<TData | null> {
  const result = await client.mutation(document, variables);
  if (result.error) {
    console.error("[urql] mutation error:", result.error);
    return null;
  }
  return result.data || null;
}

// #endregion Helper Functions
