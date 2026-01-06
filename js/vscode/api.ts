// #region Header

// js/vscode/api.ts

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

import type { Client } from "@urql/core";
import { getUrqlClient, query, mutation } from "./graphql";
import {
  RepoQuery,
  BundlesQuery,
  TicketsQuery,
  PoliciesQuery,
  ContributorsQuery,
  AnalyzeQuery,
  FixMutation,
} from "./queries";
import type {
  RepoQuery as RepoQueryResult,
  BundlesQuery as BundlesQueryResult,
  TicketsQuery as TicketsQueryResult,
  PoliciesQuery as PoliciesQueryResult,
  ContributorsQuery as ContributorsQueryResult,
  AnalyzeQuery as AnalyzeQueryResult,
  FixMutation as FixMutationResult,
  TicketStatus,
} from "./generated/graphql";

// #endregion Imports

// #region Client Management

let cachedClient: Client | null = null;
let cachedWorkspaceRoot: string | null = null;
let cachedRepoCommand: string | null = null;

export function initializeClient(workspaceRoot: string, repoCommand: string): Client {
  if (!cachedClient || cachedWorkspaceRoot !== workspaceRoot || cachedRepoCommand !== repoCommand) {
    cachedClient = getUrqlClient(workspaceRoot, repoCommand);
    cachedWorkspaceRoot = workspaceRoot;
    cachedRepoCommand = repoCommand;
  }
  return cachedClient;
}

export function getClient(): Client | null {
  return cachedClient;
}

export function resetClient(): void {
  cachedClient = null;
  cachedWorkspaceRoot = null;
  cachedRepoCommand = null;
}

// #endregion Client Management

// #region Type Aliases

export type Repo = NonNullable<RepoQueryResult["repo"]>;
export type Bundle = Repo["bundles"][number];
export type Ticket = NonNullable<TicketsQueryResult["repo"]>["tickets"][number];
export type Policy = NonNullable<PoliciesQueryResult["repo"]>["policies"][number];
export type ViolationKind = Policy["violationKinds"][number];
export type Contributor = NonNullable<ContributorsQueryResult["repo"]>["contributors"][number];
export type Violation = NonNullable<AnalyzeQueryResult["analyze"]>["violations"][number];
export type AnalyzeResult = NonNullable<AnalyzeQueryResult["analyze"]>;
export type FixResult = NonNullable<FixMutationResult["fix"]>;

// #endregion Type Aliases

// #region API Functions

export async function fetchRepo(): Promise<Repo | null> {
  const client = getClient();
  if (!client) return null;
  const result = await query(client, RepoQuery);
  return result?.repo ?? null;
}

export async function fetchBundles(): Promise<Bundle[]> {
  const client = getClient();
  if (!client) return [];
  const result = await query(client, BundlesQuery);
  return result?.repo?.bundles ?? [];
}

export async function fetchTickets(filters?: {
  year?: number;
  month?: number;
  day?: number;
  status?: TicketStatus;
}): Promise<Ticket[]> {
  const client = getClient();
  if (!client) return [];
  const result = await query(client, TicketsQuery, filters);
  return result?.repo?.tickets ?? [];
}

export async function fetchPolicies(): Promise<Policy[]> {
  const client = getClient();
  if (!client) return [];
  const result = await query(client, PoliciesQuery);
  return result?.repo?.policies ?? [];
}

export async function fetchContributors(): Promise<Contributor[]> {
  const client = getClient();
  if (!client) return [];
  const result = await query(client, ContributorsQuery);
  return result?.repo?.contributors ?? [];
}

export async function analyze(scope?: string): Promise<AnalyzeResult | null> {
  const client = getClient();
  if (!client) return null;
  const result = await query(client, AnalyzeQuery, { scope });
  return result?.analyze ?? null;
}

export async function fix(scope?: string): Promise<FixResult | null> {
  const client = getClient();
  if (!client) return null;
  const result = await mutation(client, FixMutation, { scope });
  return result?.fix ?? null;
}

// #endregion API Functions
