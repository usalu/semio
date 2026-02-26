// #region 🔖Header

// 💻semio-repo/vscode/codegen/graphql.ts

// 2026 Ueli Saluz <ueli@semio-tech.de>

// #region 🔖License

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion 🔖License

// #region 🔖Requirements
// #endregion 🔖Requirements

// #endregion 🔖Header


import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in ands = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  DateTime: { input: any; output: any; }
};

export type AnalyzeMetrics = {
  __typename?: 'AnalyzeMetrics';
  autofixable: Scalars['Int']['output'];
  byPriority: PriorityCount;
  total: Scalars['Int']['output'];
};

export type AnalyzeResult = {
  __typename?: 'AnalyzeResult';
  metrics: AnalyzeMetrics;
  breachs: Array<Breach>;
};

export type Autofix = {
  __typename?: 'Autofix';
  description: Scalars['String']['output'];
  edits: Array<FileEdit>;
};

export type Bundle = Node & {
  __typename?: 'Bundle';
  contributors: Array<Contributor>;
  files: Array<File>;
  folders: Array<Folder>;
  id: Scalars['ID']['output'];
  metrics: BundleMetrics;
  name: Scalars['String']['output'];
  projectType?: Maybe<Scalars['String']['output']>;
  root: Scalars['String']['output'];
  sourceRoot?: Maybe<Scalars['String']['output']>;
  tags: Array<Scalars['String']['output']>;
  tickets: Array<Ticket>;
  uri: Scalars['String']['output'];
  breachs: Array<Breach>;
};

export type BundleMetrics = {
  __typename?: 'BundleMetrics';
  definitions: Scalars['Int']['output'];
  files: Scalars['Int']['output'];
  folders: Scalars['Int']['output'];
  lines: Scalars['Int']['output'];
  sections: Scalars['Int']['output'];
  breachs: Scalars['Int']['output'];
};

export type CheckpointDate = {
  __typename?: 'CheckpointDate';
  created: Scalars['DateTime']['output'];
};

export type CheckpointDefinitionContrib = {
  __typename?: 'CheckpointDefinitionContrib';
  definition: Scalars['String']['output'];
  file: Scalars['String']['output'];
  metrics: LineMetrics;
  range: Range;
  section: Scalars['String']['output'];
};

export type CheckpointMetrics = {
  __typename?: 'CheckpointMetrics';
  definitions: Scalars['Int']['output'];
  files: Scalars['Int']['output'];
  lines: LineMetrics;
  sections: Scalars['Int']['output'];
};

export type CheckpointSectionContrib = {
  __typename?: 'CheckpointSectionContrib';
  file: Scalars['String']['output'];
  metrics: LineMetrics;
  range: Range;
  section: Scalars['String']['output'];
};

export type Checkpoint = Node & {
  __typename?: 'Checkpoint';
  author?: Maybe<Contributor>;
  bundles: Array<Bundle>;
  date: Scalars['DateTime']['output'];
  files: Array<File>;
  id: Scalars['ID']['output'];
  sha: Scalars['String']['output'];
  title: Scalars['String']['output'];
};

export type ContributionBundle = {
  __typename?: 'ContributionBundle';
  bundle: Bundle;
  metrics: CountMetrics;
};

export type ContributionDefinition = {
  __typename?: 'ContributionDefinition';
  definition: Definition;
  metrics: LineMetrics;
};

export type ContributionFile = {
  __typename?: 'ContributionFile';
  file: File;
  metrics: LineMetrics;
};

export type ContributionFolder = {
  __typename?: 'ContributionFolder';
  folder: Folder;
  metrics: CountMetrics;
};

export type ContributionSection = {
  __typename?: 'ContributionSection';
  metrics: LineMetrics;
  section: Section;
};

export type Contributor = Node & {
  __typename?: 'Contributor';
  bundles: Array<Bundle>;
  checkpoints: Array<Checkpoint>;
  contributions: ContributorContributions;
  emails: Array<Scalars['String']['output']>;
  files: Array<File>;
  github: Scalars['String']['output'];
  icons?: Maybe<ContributorIcons>;
  id: Scalars['ID']['output'];
  links: Array<ContributorLink>;
  metrics: ContributorMetrics;
  name?: Maybe<Scalars['String']['output']>;
  tickets: Array<Ticket>;
};

export type ContributorAddInput = {
  emails?: InputMaybe<Array<Scalars['String']['input']>>;
  github: Scalars['String']['input'];
  name?: InputMaybe<Scalars['String']['input']>;
};

export type ContributorContributions = {
  __typename?: 'ContributorContributions';
  bundles: Array<ContributionBundle>;
  definitions: Array<ContributionDefinition>;
  files: Array<ContributionFile>;
  folders: Array<ContributionFolder>;
  sections: Array<ContributionSection>;
};

export type ContributorIcons = {
  __typename?: 'ContributorIcons';
  avatar?: Maybe<Scalars['String']['output']>;
  avatarRound?: Maybe<Scalars['String']['output']>;
  github?: Maybe<Scalars['String']['output']>;
};

export type ContributorLink = {
  __typename?: 'ContributorLink';
  name: Scalars['String']['output'];
  url: Scalars['String']['output'];
};

export type ContributorMetrics = {
  __typename?: 'ContributorMetrics';
  bundles: Scalars['Int']['output'];
  checkpoints: Scalars['Int']['output'];
  definitions: Scalars['Int']['output'];
  files: Scalars['Int']['output'];
  folders: Scalars['Int']['output'];
  lines: Scalars['Int']['output'];
  sections: Scalars['Int']['output'];
  tickets: Scalars['Int']['output'];
};

export type CountMetrics = {
  __typename?: 'CountMetrics';
  added: Scalars['Int']['output'];
  removed: Scalars['Int']['output'];
  updated: Scalars['Int']['output'];
};

export type Definition = Node & {
  __typename?: 'Definition';
  file: File;
  id: Scalars['ID']['output'];
  kind: DefinitionKind;
  metrics: DefinitionMetrics;
  name: Scalars['String']['output'];
  range: Range;
  section?: Maybe<Section>;
  breachs: Array<Breach>;
};

export enum DefinitionKind {
  Implementation = 'IMPLEMENTATION',
  Interface = 'INTERFACE',
  Constant = 'CONSTANT'
}

export type DefinitionMetrics = {
  __typename?: 'DefinitionMetrics';
  definitions: Scalars['Int']['output'];
  lines: Scalars['Int']['output'];
  breachs: Scalars['Int']['output'];
};

export type File = Node & {
  __typename?: 'File';
  bundle?: Maybe<Bundle>;
  content?: Maybe<Scalars['String']['output']>;
  contributors: Array<Contributor>;
  definitions: Array<Definition>;
  extension: Scalars['String']['output'];
  folder?: Maybe<Folder>;
  id: Scalars['ID']['output'];
  metrics: FileMetrics;
  name: Scalars['String']['output'];
  path: Scalars['String']['output'];
  sections: Array<Section>;
  uri: Scalars['String']['output'];
  breachs: Array<Breach>;
};

export type FileEdit = {
  __typename?: 'FileEdit';
  edits: Array<TextEdit>;
  path: Scalars['String']['output'];
};

export type FileMetrics = {
  __typename?: 'FileMetrics';
  definitions: Scalars['Int']['output'];
  lines: Scalars['Int']['output'];
  sections: Scalars['Int']['output'];
};

export type FixResult = {
  __typename?: 'FixResult';
  fixed: Scalars['Int']['output'];
  remaining: Scalars['Int']['output'];
  breachs: Array<Breach>;
};

export type Folder = Node & {
  __typename?: 'Folder';
  bundle?: Maybe<Bundle>;
  children: Array<Folder>;
  files: Array<File>;
  id: Scalars['ID']['output'];
  metrics: FolderMetrics;
  name: Scalars['String']['output'];
  parent?: Maybe<Folder>;
  path: Scalars['String']['output'];
  uri: Scalars['String']['output'];
  breachs: Array<Breach>;
};

export type FolderMetrics = {
  __typename?: 'FolderMetrics';
  files: Scalars['Int']['output'];
  lines: Scalars['Int']['output'];
  breachs: Scalars['Int']['output'];
};

export type LineMetrics = {
  __typename?: 'LineMetrics';
  added: Scalars['Int']['output'];
  removed: Scalars['Int']['output'];
};

export type Mutation = {
  __typename?: 'Mutation';
  contributorAdd?: Maybe<Contributor>;
  contributorRemove: Scalars['Boolean']['output'];
  fileCreate?: Maybe<File>;
  fileDelete: Scalars['Boolean']['output'];
  fileMove?: Maybe<File>;
  fix: FixResult;
  folderCreate?: Maybe<Folder>;
  folderDelete: Scalars['Boolean']['output'];
  folderMove?: Maybe<Folder>;
  sectionCreate?: Maybe<Section>;
  sectionDelete: Scalars['Boolean']['output'];
  sectionMove?: Maybe<Section>;
  ticketCheckpoint?: Maybe<Ticket>;
  ticketOpen?: Maybe<Ticket>;
  ticketClose?: Maybe<Ticket>;
  ticketReopen?: Maybe<Ticket>;
};


export type MutationContributorAddArgs = {
  input: ContributorAddInput;
};


export type MutationContributorRemoveArgs = {
  github: Scalars['String']['input'];
};


export type MutationFileCreateArgs = {
  path: Scalars['String']['input'];
};


export type MutationFileDeleteArgs = {
  path: Scalars['String']['input'];
};


export type MutationFileMoveArgs = {
  dst: Scalars['String']['input'];
  src: Scalars['String']['input'];
};


export type MutationFixArgs = {
  scope?: InputMaybe<Scalars['String']['input']>;
};


export type MutationFolderCreateArgs = {
  path: Scalars['String']['input'];
};


export type MutationFolderDeleteArgs = {
  path: Scalars['String']['input'];
};


export type MutationFolderMoveArgs = {
  dst: Scalars['String']['input'];
  src: Scalars['String']['input'];
};


export type MutationSectionCreateArgs = {
  file: Scalars['String']['input'];
  name: Scalars['String']['input'];
  parent?: InputMaybe<Scalars['String']['input']>;
};


export type MutationSectionDeleteArgs = {
  file: Scalars['String']['input'];
  name: Scalars['String']['input'];
};


export type MutationSectionMoveArgs = {
  file: Scalars['String']['input'];
  newName: Scalars['String']['input'];
  oldName: Scalars['String']['input'];
};


export type MutationTicketCheckpointArgs = {
  input: TicketCheckpointInput;
};


export type MutationTicketOpenArgs = {
  input: TicketOpenInput;
};


export type MutationTicketCloseArgs = {
  input: TicketCloseInput;
};


export type MutationTicketReopenArgs = {
  input: TicketReopenInput;
};

export type Node = {
  id: Scalars['ID']['output'];
};

export type Policy = Node & {
  __typename?: 'Policy';
  description?: Maybe<Scalars['String']['output']>;
  id: Scalars['ID']['output'];
  name: Scalars['String']['output'];
  scopes: Array<Scalars['String']['output']>;
  statutes: Array<Statute>;
};

export type Position = {
  __typename?: 'Position';
  column: Scalars['Int']['output'];
  line: Scalars['Int']['output'];
};

export type PriorityCount = {
  __typename?: 'PriorityCount';
  high: Scalars['Int']['output'];
  low: Scalars['Int']['output'];
  medium: Scalars['Int']['output'];
};

export type Query = {
  __typename?: 'Query';
  analyze: AnalyzeResult;
  bundle?: Maybe<Bundle>;
  bundles: Array<Bundle>;
  contributor?: Maybe<Contributor>;
  contributors: Array<Contributor>;
  definition?: Maybe<Definition>;
  file?: Maybe<File>;
  files: Array<File>;
  folder?: Maybe<Folder>;
  folders: Array<Folder>;
  node?: Maybe<Node>;
  policies: Array<Policy>;
  policy?: Maybe<Policy>;
  repo: Repo;
  section?: Maybe<Section>;
  ticket?: Maybe<Ticket>;
  tickets: Array<Ticket>;
  statute?: Maybe<Statute>;
  statutes: Array<Statute>;
  breachs: Array<Breach>;
};


export type QueryAnalyzeArgs = {
  scope?: InputMaybe<Scalars['String']['input']>;
};


export type QueryBundleArgs = {
  name: Scalars['String']['input'];
};


export type QueryContributorArgs = {
  id: Scalars['String']['input'];
};


export type QueryDefinitionArgs = {
  name: Scalars['String']['input'];
  path: Scalars['String']['input'];
};


export type QueryFileArgs = {
  path: Scalars['String']['input'];
};


export type QueryFolderArgs = {
  path: Scalars['String']['input'];
};


export type QueryNodeArgs = {
  id: Scalars['ID']['input'];
};


export type QueryPolicyArgs = {
  id: Scalars['String']['input'];
};


export type QuerySectionArgs = {
  path: Scalars['String']['input'];
  sectionPath: Array<Scalars['String']['input']>;
};


export type QueryTicketArgs = {
  day: Scalars['Int']['input'];
  month: Scalars['Int']['input'];
  slug: Scalars['String']['input'];
  year: Scalars['Int']['input'];
};


export type QueryTicketsArgs = {
  day?: InputMaybe<Scalars['Int']['input']>;
  month?: InputMaybe<Scalars['Int']['input']>;
  status?: InputMaybe<TicketStatus>;
  year?: InputMaybe<Scalars['Int']['input']>;
};


export type QueryStatuteArgs = {
  id: Scalars['String']['input'];
};


export type QueryBreachsArgs = {
  scope?: InputMaybe<Scalars['String']['input']>;
};

export type Range = {
  __typename?: 'Range';
  end: Position;
  start: Position;
};

export type Repo = Node & {
  __typename?: 'Repo';
  bundles: Array<Bundle>;
  contributors: Array<Contributor>;
  files: Array<File>;
  folders: Array<Folder>;
  id: Scalars['ID']['output'];
  metrics: RepoMetrics;
  name: Scalars['String']['output'];
  path: Scalars['String']['output'];
  policies: Array<Policy>;
  tickets: Array<Ticket>;
  statutes: Array<Statute>;
  breachs: Array<Breach>;
};


export type RepoTicketsArgs = {
  day?: InputMaybe<Scalars['Int']['input']>;
  month?: InputMaybe<Scalars['Int']['input']>;
  status?: InputMaybe<TicketStatus>;
  year?: InputMaybe<Scalars['Int']['input']>;
};


export type RepoBreachsArgs = {
  scope?: InputMaybe<Scalars['String']['input']>;
};

export type RepoMetrics = {
  __typename?: 'RepoMetrics';
  bundles: Scalars['Int']['output'];
  contributors: Scalars['Int']['output'];
  definitions: Scalars['Int']['output'];
  files: Scalars['Int']['output'];
  folders: Scalars['Int']['output'];
  lines: Scalars['Int']['output'];
  sections: Scalars['Int']['output'];
  tickets: Scalars['Int']['output'];
  breachs: Scalars['Int']['output'];
};

export type Section = Node & {
  __typename?: 'Section';
  children: Array<Section>;
  definitions: Array<Definition>;
  file: File;
  id: Scalars['ID']['output'];
  metrics: SectionMetrics;
  name: Scalars['String']['output'];
  parent?: Maybe<Section>;
  path: Scalars['String']['output'];
  range: Range;
  breachs: Array<Breach>;
};

export type SectionMetrics = {
  __typename?: 'SectionMetrics';
  definitions: Scalars['Int']['output'];
  lines: Scalars['Int']['output'];
  breachs: Scalars['Int']['output'];
};

export type TextEdit = {
  __typename?: 'TextEdit';
  end: Scalars['Int']['output'];
  newText: Scalars['String']['output'];
  start: Scalars['Int']['output'];
};

export type Ticket = Node & {
  __typename?: 'Ticket';
  author?: Maybe<Contributor>;
  bundles: Array<Bundle>;
  checkpoints: Array<TicketCheckpoint>;
  checkpoint?: Maybe<Scalars['String']['output']>;
  date: TicketDate;
  day: Scalars['Int']['output'];
  files: Array<File>;
  id: Scalars['ID']['output'];
  llm?: Maybe<Scalars['String']['output']>;
  metrics: TicketMetrics;
  model?: Maybe<Scalars['String']['output']>;
  month: Scalars['Int']['output'];
  path: Scalars['String']['output'];
  prompt: Scalars['String']['output'];
  slug: Scalars['String']['output'];
  status: TicketStatus;
  summary?: Maybe<Scalars['String']['output']>;
  title?: Maybe<Scalars['String']['output']>;
  uri: Scalars['String']['output'];
  year: Scalars['Int']['output'];
};

export type TicketBundleContrib = {
  __typename?: 'TicketBundleContrib';
  bundle: Bundle;
  files: Array<TicketFileContrib>;
};

export type TicketCheckpoint = {
  __typename?: 'TicketCheckpoint';
  author?: Maybe<Contributor>;
  checkpoint?: Maybe<Scalars['String']['output']>;
  date: CheckpointDate;
  definitions: Array<CheckpointDefinitionContrib>;
  files: Array<Scalars['String']['output']>;
  metrics: CheckpointMetrics;
  model?: Maybe<Scalars['String']['output']>;
  prompt: Scalars['String']['output'];
  sections: Array<CheckpointSectionContrib>;
};

export type TicketCheckpointInput = {
  day: Scalars['Int']['input'];
  files: Array<Scalars['String']['input']>;
  model?: InputMaybe<Scalars['String']['input']>;
  month: Scalars['Int']['input'];
  prompt: Scalars['String']['input'];
  slug: Scalars['String']['input'];
  year: Scalars['Int']['input'];
};

export type TicketOpenInput = {
  llm: Scalars['String']['input'];
  planPath?: InputMaybe<Scalars['String']['input']>;
  prompt: Scalars['String']['input'];
  title: Scalars['String']['input'];
};

export type TicketDate = {
  __typename?: 'TicketDate';
  created: Scalars['DateTime']['output'];
  finished?: Maybe<Scalars['DateTime']['output']>;
};

export type TicketFileContrib = {
  __typename?: 'TicketFileContrib';
  file: File;
  sections: Array<TicketSectionContrib>;
};

export type TicketCloseInput = {
  day: Scalars['Int']['input'];
  month: Scalars['Int']['input'];
  slug: Scalars['String']['input'];
  summary?: InputMaybe<Scalars['String']['input']>;
  year: Scalars['Int']['input'];
};

export type TicketMetrics = {
  __typename?: 'TicketMetrics';
  checkpoints: Scalars['Int']['output'];
  definitions: Scalars['Int']['output'];
  files: Scalars['Int']['output'];
  lines: LineMetrics;
  sections: Scalars['Int']['output'];
};

export type TicketReopenInput = {
  day: Scalars['Int']['input'];
  month: Scalars['Int']['input'];
  slug: Scalars['String']['input'];
  year: Scalars['Int']['input'];
};

export type TicketSectionContrib = {
  __typename?: 'TicketSectionContrib';
  definitions: Array<Scalars['String']['output']>;
  metrics: LineMetrics;
  section: Section;
};

export enum TicketStatus {
  Closed = 'CLOSED',
  Open = 'OPEN'
}

export type Breach = Node & {
  __typename?: 'Breach';
  autofix?: Maybe<Autofix>;
  autofixable: Scalars['Boolean']['output'];
  column?: Maybe<Scalars['Int']['output']>;
  excerpt?: Maybe<Scalars['String']['output']>;
  file?: Maybe<File>;
  folder?: Maybe<Folder>;
  id: Scalars['ID']['output'];
  kind: Statute;
  line?: Maybe<Scalars['Int']['output']>;
  priority: BreachPriority;
  scope: Scalars['String']['output'];
  summary: Scalars['String']['output'];
};

export type Statute = Node & {
  __typename?: 'Statute';
  autofixable: Scalars['Boolean']['output'];
  id: Scalars['ID']['output'];
  policy: Policy;
  priority: BreachPriority;
  reason: Scalars['String']['output'];
  solution: Scalars['String']['output'];
  breachs: Array<Breach>;
};


export type StatuteBreachsArgs = {
  scope?: InputMaybe<Scalars['String']['input']>;
};

export enum BreachPriority {
  High = 'HIGH',
  Low = 'LOW',
  Medium = 'MEDIUM'
}

export type RepoQueryVariables = Exact<{ [key: string]: never; }>;


export type RepoQuery = { __typename?: 'Query', repo: { __typename?: 'Repo', id: string, name: string, path: string, bundles: Array<{ __typename?: 'Bundle', id: string, name: string, root: string, sourceRoot?: string | null, projectType?: string | null, tags: Array<string>, uri: string }>, tickets: Array<{ __typename?: 'Ticket', id: string, year: number, month: number, day: number, slug: string, path: string, uri: string, prompt: string, summary?: string | null, status: TicketStatus, checkpoint?: string | null }>, policies: Array<{ __typename?: 'Policy', id: string, name: string, description?: string | null, scopes: Array<string> }>, contributors: Array<{ __typename?: 'Contributor', id: string, github: string, name?: string | null, emails: Array<string> }> } };

export type BundlesQueryVariables = Exact<{ [key: string]: never; }>;


export type BundlesQuery = { __typename?: 'Query', repo: { __typename?: 'Repo', bundles: Array<{ __typename?: 'Bundle', id: string, name: string, root: string, sourceRoot?: string | null, projectType?: string | null, tags: Array<string>, uri: string }> } };

export type TicketsQueryVariables = Exact<{
  year?: InputMaybe<Scalars['Int']['input']>;
  month?: InputMaybe<Scalars['Int']['input']>;
  day?: InputMaybe<Scalars['Int']['input']>;
  status?: InputMaybe<TicketStatus>;
}>;


export type TicketsQuery = { __typename?: 'Query', repo: { __typename?: 'Repo', tickets: Array<{ __typename?: 'Ticket', id: string, year: number, month: number, day: number, slug: string, path: string, uri: string, prompt: string, summary?: string | null, status: TicketStatus, model?: string | null, checkpoint?: string | null, author?: { __typename?: 'Contributor', github: string, name?: string | null } | null, date: { __typename?: 'TicketDate', created: any, finished?: any | null }, checkpoints: Array<{ __typename?: 'TicketCheckpoint', prompt: string, model?: string | null, checkpoint?: string | null, author?: { __typename?: 'Contributor', github: string, name?: string | null } | null, date: { __typename?: 'CheckpointDate', created: any } }>, metrics: { __typename?: 'TicketMetrics', checkpoints: number, files: number, lines: { __typename?: 'LineMetrics', added: number, removed: number } } }> } };

export type PoliciesQueryVariables = Exact<{ [key: string]: never; }>;


export type PoliciesQuery = { __typename?: 'Query', repo: { __typename?: 'Repo', policies: Array<{ __typename?: 'Policy', id: string, name: string, description?: string | null, scopes: Array<string>, statutes: Array<{ __typename?: 'Statute', id: string, priority: BreachPriority, autofixable: boolean, reason: string, solution: string }> }> } };

export type ContributorsQueryVariables = Exact<{ [key: string]: never; }>;


export type ContributorsQuery = { __typename?: 'Query', repo: { __typename?: 'Repo', contributors: Array<{ __typename?: 'Contributor', id: string, github: string, name?: string | null, emails: Array<string>, links: Array<{ __typename?: 'ContributorLink', name: string, url: string }>, icons?: { __typename?: 'ContributorIcons', avatar?: string | null, avatarRound?: string | null, github?: string | null } | null, metrics: { __typename?: 'ContributorMetrics', checkpoints: number, tickets: number, bundles: number, folders: number, files: number, sections: number, definitions: number, lines: number } }> } };

export type AnalyzeQueryVariables = Exact<{
  scope?: InputMaybe<Scalars['String']['input']>;
}>;


export type AnalyzeQuery = { __typename?: 'Query', analyze: { __typename?: 'AnalyzeResult', breachs: Array<{ __typename?: 'Breach', id: string, summary: string, priority: BreachPriority, autofixable: boolean, scope: string, line?: number | null, column?: number | null, excerpt?: string | null, kind: { __typename?: 'Statute', id: string, reason: string, solution: string, policy: { __typename?: 'Policy', id: string, name: string } }, autofix?: { __typename?: 'Autofix', description: string } | null }>, metrics: { __typename?: 'AnalyzeMetrics', total: number, autofixable: number, byPriority: { __typename?: 'PriorityCount', high: number, medium: number, low: number } } } };

export type FixMutationVariables = Exact<{
  scope?: InputMaybe<Scalars['String']['input']>;
}>;


export type FixMutation = { __typename?: 'Mutation', fix: { __typename?: 'FixResult', fixed: number, remaining: number, breachs: Array<{ __typename?: 'Breach', id: string, summary: string, priority: BreachPriority, scope: string }> } };

export type CodebaseQueryVariables = Exact<{ [key: string]: never; }>;


export type CodebaseQuery = { __typename?: 'Query', repo: { __typename?: 'Repo', id: string, name: string, path: string, bundles: Array<{ __typename?: 'Bundle', id: string, name: string, root: string, sourceRoot?: string | null, projectType?: string | null, tags: Array<string>, uri: string, metrics: { __typename?: 'BundleMetrics', folders: number, files: number, sections: number, definitions: number, lines: number, breachs: number } }>, folders: Array<{ __typename?: 'Folder', id: string, path: string, uri: string, metrics: { __typename?: 'FolderMetrics', files: number, lines: number, breachs: number } }>, files: Array<{ __typename?: 'File', id: string, path: string, uri: string, metrics: { __typename?: 'FileMetrics', sections: number, definitions: number, lines: number }, sections: Array<{ __typename?: 'Section', id: string, name: string, path: string, range: { __typename?: 'Range', start: { __typename?: 'Position', line: number }, end: { __typename?: 'Position', line: number } }, metrics: { __typename?: 'SectionMetrics', definitions: number, lines: number, breachs: number } }>, definitions: Array<{ __typename?: 'Definition', id: string, name: string, kind: DefinitionKind, range: { __typename?: 'Range', start: { __typename?: 'Position', line: number }, end: { __typename?: 'Position', line: number } }, metrics: { __typename?: 'DefinitionMetrics', definitions: number, lines: number, breachs: number } }> }>, contributors: Array<{ __typename?: 'Contributor', id: string, github: string, name?: string | null, emails: Array<string>, links: Array<{ __typename?: 'ContributorLink', name: string, url: string }>, metrics: { __typename?: 'ContributorMetrics', checkpoints: number, tickets: number, bundles: number, folders: number, files: number, sections: number, definitions: number, lines: number } }>, tickets: Array<{ __typename?: 'Ticket', id: string, year: number, month: number, day: number, slug: string, path: string, uri: string, prompt: string, summary?: string | null, status: TicketStatus, checkpoint?: string | null, author?: { __typename?: 'Contributor', github: string, name?: string | null } | null, checkpoints: Array<{ __typename?: 'TicketCheckpoint', checkpoint?: string | null }>, metrics: { __typename?: 'TicketMetrics', checkpoints: number, files: number, lines: { __typename?: 'LineMetrics', added: number, removed: number } } }>, policies: Array<{ __typename?: 'Policy', id: string, name: string, description?: string | null, scopes: Array<string>, statutes: Array<{ __typename?: 'Statute', id: string, priority: BreachPriority, autofixable: boolean, reason: string, solution: string }> }> } };


export const RepoDocument = { "kind": "Document", "definitions": [{ "kind": "OperationDefinition", "operation": "query", "name": { "kind": "Name", "value": "Repo" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "repo" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "name" } }, { "kind": "Field", "name": { "kind": "Name", "value": "path" } }, { "kind": "Field", "name": { "kind": "Name", "value": "bundles" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "name" } }, { "kind": "Field", "name": { "kind": "Name", "value": "root" } }, { "kind": "Field", "name": { "kind": "Name", "value": "sourceRoot" } }, { "kind": "Field", "name": { "kind": "Name", "value": "projectType" } }, { "kind": "Field", "name": { "kind": "Name", "value": "tags" } }, { "kind": "Field", "name": { "kind": "Name", "value": "uri" } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "tickets" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "year" } }, { "kind": "Field", "name": { "kind": "Name", "value": "month" } }, { "kind": "Field", "name": { "kind": "Name", "value": "day" } }, { "kind": "Field", "name": { "kind": "Name", "value": "slug" } }, { "kind": "Field", "name": { "kind": "Name", "value": "path" } }, { "kind": "Field", "name": { "kind": "Name", "value": "uri" } }, { "kind": "Field", "name": { "kind": "Name", "value": "prompt" } }, { "kind": "Field", "name": { "kind": "Name", "value": "summary" } }, { "kind": "Field", "name": { "kind": "Name", "value": "status" } }, { "kind": "Field", "name": { "kind": "Name", "value": "checkpoint" } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "policies" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "name" } }, { "kind": "Field", "name": { "kind": "Name", "value": "description" } }, { "kind": "Field", "name": { "kind": "Name", "value": "scopes" } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "contributors" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "github" } }, { "kind": "Field", "name": { "kind": "Name", "value": "name" } }, { "kind": "Field", "name": { "kind": "Name", "value": "emails" } }] } }] } }] } }] } as unknown as DocumentNode<RepoQuery, RepoQueryVariables>;
export const BundlesDocument = { "kind": "Document", "definitions": [{ "kind": "OperationDefinition", "operation": "query", "name": { "kind": "Name", "value": "Bundles" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "repo" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "bundles" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "name" } }, { "kind": "Field", "name": { "kind": "Name", "value": "root" } }, { "kind": "Field", "name": { "kind": "Name", "value": "sourceRoot" } }, { "kind": "Field", "name": { "kind": "Name", "value": "projectType" } }, { "kind": "Field", "name": { "kind": "Name", "value": "tags" } }, { "kind": "Field", "name": { "kind": "Name", "value": "uri" } }] } }] } }] } }] } as unknown as DocumentNode<BundlesQuery, BundlesQueryVariables>;
export const TicketsDocument = { "kind": "Document", "definitions": [{ "kind": "OperationDefinition", "operation": "query", "name": { "kind": "Name", "value": "Tickets" }, "variableDefinitions": [{ "kind": "VariableDefinition", "variable": { "kind": "Variable", "name": { "kind": "Name", "value": "year" } }, "type": { "kind": "NamedType", "name": { "kind": "Name", "value": "Int" } } }, { "kind": "VariableDefinition", "variable": { "kind": "Variable", "name": { "kind": "Name", "value": "month" } }, "type": { "kind": "NamedType", "name": { "kind": "Name", "value": "Int" } } }, { "kind": "VariableDefinition", "variable": { "kind": "Variable", "name": { "kind": "Name", "value": "day" } }, "type": { "kind": "NamedType", "name": { "kind": "Name", "value": "Int" } } }, { "kind": "VariableDefinition", "variable": { "kind": "Variable", "name": { "kind": "Name", "value": "status" } }, "type": { "kind": "NamedType", "name": { "kind": "Name", "value": "TicketStatus" } } }], "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "repo" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "tickets" }, "arguments": [{ "kind": "Argument", "name": { "kind": "Name", "value": "year" }, "value": { "kind": "Variable", "name": { "kind": "Name", "value": "year" } } }, { "kind": "Argument", "name": { "kind": "Name", "value": "month" }, "value": { "kind": "Variable", "name": { "kind": "Name", "value": "month" } } }, { "kind": "Argument", "name": { "kind": "Name", "value": "day" }, "value": { "kind": "Variable", "name": { "kind": "Name", "value": "day" } } }, { "kind": "Argument", "name": { "kind": "Name", "value": "status" }, "value": { "kind": "Variable", "name": { "kind": "Name", "value": "status" } } }], "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "year" } }, { "kind": "Field", "name": { "kind": "Name", "value": "month" } }, { "kind": "Field", "name": { "kind": "Name", "value": "day" } }, { "kind": "Field", "name": { "kind": "Name", "value": "slug" } }, { "kind": "Field", "name": { "kind": "Name", "value": "path" } }, { "kind": "Field", "name": { "kind": "Name", "value": "uri" } }, { "kind": "Field", "name": { "kind": "Name", "value": "prompt" } }, { "kind": "Field", "name": { "kind": "Name", "value": "summary" } }, { "kind": "Field", "name": { "kind": "Name", "value": "status" } }, { "kind": "Field", "name": { "kind": "Name", "value": "author" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "github" } }, { "kind": "Field", "name": { "kind": "Name", "value": "name" } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "model" } }, { "kind": "Field", "name": { "kind": "Name", "value": "checkpoint" } }, { "kind": "Field", "name": { "kind": "Name", "value": "date" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "created" } }, { "kind": "Field", "name": { "kind": "Name", "value": "finished" } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "checkpoints" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "prompt" } }, { "kind": "Field", "name": { "kind": "Name", "value": "model" } }, { "kind": "Field", "name": { "kind": "Name", "value": "author" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "github" } }, { "kind": "Field", "name": { "kind": "Name", "value": "name" } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "checkpoint" } }, { "kind": "Field", "name": { "kind": "Name", "value": "date" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "created" } }] } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "metrics" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "checkpoints" } }, { "kind": "Field", "name": { "kind": "Name", "value": "files" } }, { "kind": "Field", "name": { "kind": "Name", "value": "lines" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "added" } }, { "kind": "Field", "name": { "kind": "Name", "value": "removed" } }] } }] } }] } }] } }] } }] } as unknown as DocumentNode<TicketsQuery, TicketsQueryVariables>;
export const PoliciesDocument = { "kind": "Document", "definitions": [{ "kind": "OperationDefinition", "operation": "query", "name": { "kind": "Name", "value": "Policies" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "repo" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "policies" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "name" } }, { "kind": "Field", "name": { "kind": "Name", "value": "description" } }, { "kind": "Field", "name": { "kind": "Name", "value": "scopes" } }, { "kind": "Field", "name": { "kind": "Name", "value": "statutes" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "priority" } }, { "kind": "Field", "name": { "kind": "Name", "value": "autofixable" } }, { "kind": "Field", "name": { "kind": "Name", "value": "reason" } }, { "kind": "Field", "name": { "kind": "Name", "value": "solution" } }] } }] } }] } }] } }] } as unknown as DocumentNode<PoliciesQuery, PoliciesQueryVariables>;
export const ContributorsDocument = { "kind": "Document", "definitions": [{ "kind": "OperationDefinition", "operation": "query", "name": { "kind": "Name", "value": "Contributors" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "repo" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "contributors" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "github" } }, { "kind": "Field", "name": { "kind": "Name", "value": "name" } }, { "kind": "Field", "name": { "kind": "Name", "value": "emails" } }, { "kind": "Field", "name": { "kind": "Name", "value": "links" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "name" } }, { "kind": "Field", "name": { "kind": "Name", "value": "url" } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "icons" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "avatar" } }, { "kind": "Field", "name": { "kind": "Name", "value": "avatarRound" } }, { "kind": "Field", "name": { "kind": "Name", "value": "github" } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "metrics" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "checkpoints" } }, { "kind": "Field", "name": { "kind": "Name", "value": "tickets" } }, { "kind": "Field", "name": { "kind": "Name", "value": "bundles" } }, { "kind": "Field", "name": { "kind": "Name", "value": "folders" } }, { "kind": "Field", "name": { "kind": "Name", "value": "files" } }, { "kind": "Field", "name": { "kind": "Name", "value": "sections" } }, { "kind": "Field", "name": { "kind": "Name", "value": "definitions" } }, { "kind": "Field", "name": { "kind": "Name", "value": "lines" } }] } }] } }] } }] } }] } as unknown as DocumentNode<ContributorsQuery, ContributorsQueryVariables>;
export const AnalyzeDocument = { "kind": "Document", "definitions": [{ "kind": "OperationDefinition", "operation": "query", "name": { "kind": "Name", "value": "Analyze" }, "variableDefinitions": [{ "kind": "VariableDefinition", "variable": { "kind": "Variable", "name": { "kind": "Name", "value": "scope" } }, "type": { "kind": "NamedType", "name": { "kind": "Name", "value": "String" } } }], "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "analyze" }, "arguments": [{ "kind": "Argument", "name": { "kind": "Name", "value": "scope" }, "value": { "kind": "Variable", "name": { "kind": "Name", "value": "scope" } } }], "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "breachs" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "summary" } }, { "kind": "Field", "name": { "kind": "Name", "value": "priority" } }, { "kind": "Field", "name": { "kind": "Name", "value": "autofixable" } }, { "kind": "Field", "name": { "kind": "Name", "value": "scope" } }, { "kind": "Field", "name": { "kind": "Name", "value": "line" } }, { "kind": "Field", "name": { "kind": "Name", "value": "column" } }, { "kind": "Field", "name": { "kind": "Name", "value": "excerpt" } }, { "kind": "Field", "name": { "kind": "Name", "value": "kind" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "policy" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "name" } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "reason" } }, { "kind": "Field", "name": { "kind": "Name", "value": "solution" } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "autofix" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "description" } }] } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "metrics" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "total" } }, { "kind": "Field", "name": { "kind": "Name", "value": "byPriority" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "high" } }, { "kind": "Field", "name": { "kind": "Name", "value": "medium" } }, { "kind": "Field", "name": { "kind": "Name", "value": "low" } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "autofixable" } }] } }] } }] } }] } as unknown as DocumentNode<AnalyzeQuery, AnalyzeQueryVariables>;
export const FixDocument = { "kind": "Document", "definitions": [{ "kind": "OperationDefinition", "operation": "mutation", "name": { "kind": "Name", "value": "Fix" }, "variableDefinitions": [{ "kind": "VariableDefinition", "variable": { "kind": "Variable", "name": { "kind": "Name", "value": "scope" } }, "type": { "kind": "NamedType", "name": { "kind": "Name", "value": "String" } } }], "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "fix" }, "arguments": [{ "kind": "Argument", "name": { "kind": "Name", "value": "scope" }, "value": { "kind": "Variable", "name": { "kind": "Name", "value": "scope" } } }], "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "fixed" } }, { "kind": "Field", "name": { "kind": "Name", "value": "remaining" } }, { "kind": "Field", "name": { "kind": "Name", "value": "breachs" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "summary" } }, { "kind": "Field", "name": { "kind": "Name", "value": "priority" } }, { "kind": "Field", "name": { "kind": "Name", "value": "scope" } }] } }] } }] } }] } as unknown as DocumentNode<FixMutation, FixMutationVariables>;
export const CodebaseDocument = { "kind": "Document", "definitions": [{ "kind": "OperationDefinition", "operation": "query", "name": { "kind": "Name", "value": "Codebase" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "repo" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "name" } }, { "kind": "Field", "name": { "kind": "Name", "value": "path" } }, { "kind": "Field", "name": { "kind": "Name", "value": "bundles" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "name" } }, { "kind": "Field", "name": { "kind": "Name", "value": "root" } }, { "kind": "Field", "name": { "kind": "Name", "value": "sourceRoot" } }, { "kind": "Field", "name": { "kind": "Name", "value": "projectType" } }, { "kind": "Field", "name": { "kind": "Name", "value": "tags" } }, { "kind": "Field", "name": { "kind": "Name", "value": "uri" } }, { "kind": "Field", "name": { "kind": "Name", "value": "metrics" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "folders" } }, { "kind": "Field", "name": { "kind": "Name", "value": "files" } }, { "kind": "Field", "name": { "kind": "Name", "value": "sections" } }, { "kind": "Field", "name": { "kind": "Name", "value": "definitions" } }, { "kind": "Field", "name": { "kind": "Name", "value": "lines" } }, { "kind": "Field", "name": { "kind": "Name", "value": "breachs" } }] } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "folders" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "path" } }, { "kind": "Field", "name": { "kind": "Name", "value": "uri" } }, { "kind": "Field", "name": { "kind": "Name", "value": "metrics" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "files" } }, { "kind": "Field", "name": { "kind": "Name", "value": "lines" } }, { "kind": "Field", "name": { "kind": "Name", "value": "breachs" } }] } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "files" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "path" } }, { "kind": "Field", "name": { "kind": "Name", "value": "uri" } }, { "kind": "Field", "name": { "kind": "Name", "value": "metrics" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "sections" } }, { "kind": "Field", "name": { "kind": "Name", "value": "definitions" } }, { "kind": "Field", "name": { "kind": "Name", "value": "lines" } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "sections" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "name" } }, { "kind": "Field", "name": { "kind": "Name", "value": "path" } }, { "kind": "Field", "name": { "kind": "Name", "value": "range" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "start" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "line" } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "end" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "line" } }] } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "metrics" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "definitions" } }, { "kind": "Field", "name": { "kind": "Name", "value": "lines" } }, { "kind": "Field", "name": { "kind": "Name", "value": "breachs" } }] } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "definitions" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "name" } }, { "kind": "Field", "name": { "kind": "Name", "value": "kind" } }, { "kind": "Field", "name": { "kind": "Name", "value": "range" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "start" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "line" } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "end" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "line" } }] } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "metrics" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "definitions" } }, { "kind": "Field", "name": { "kind": "Name", "value": "lines" } }, { "kind": "Field", "name": { "kind": "Name", "value": "breachs" } }] } }] } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "contributors" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "github" } }, { "kind": "Field", "name": { "kind": "Name", "value": "name" } }, { "kind": "Field", "name": { "kind": "Name", "value": "emails" } }, { "kind": "Field", "name": { "kind": "Name", "value": "links" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "name" } }, { "kind": "Field", "name": { "kind": "Name", "value": "url" } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "metrics" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "checkpoints" } }, { "kind": "Field", "name": { "kind": "Name", "value": "tickets" } }, { "kind": "Field", "name": { "kind": "Name", "value": "bundles" } }, { "kind": "Field", "name": { "kind": "Name", "value": "folders" } }, { "kind": "Field", "name": { "kind": "Name", "value": "files" } }, { "kind": "Field", "name": { "kind": "Name", "value": "sections" } }, { "kind": "Field", "name": { "kind": "Name", "value": "definitions" } }, { "kind": "Field", "name": { "kind": "Name", "value": "lines" } }] } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "tickets" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "year" } }, { "kind": "Field", "name": { "kind": "Name", "value": "month" } }, { "kind": "Field", "name": { "kind": "Name", "value": "day" } }, { "kind": "Field", "name": { "kind": "Name", "value": "slug" } }, { "kind": "Field", "name": { "kind": "Name", "value": "path" } }, { "kind": "Field", "name": { "kind": "Name", "value": "uri" } }, { "kind": "Field", "name": { "kind": "Name", "value": "prompt" } }, { "kind": "Field", "name": { "kind": "Name", "value": "summary" } }, { "kind": "Field", "name": { "kind": "Name", "value": "status" } }, { "kind": "Field", "name": { "kind": "Name", "value": "checkpoint" } }, { "kind": "Field", "name": { "kind": "Name", "value": "author" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "github" } }, { "kind": "Field", "name": { "kind": "Name", "value": "name" } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "checkpoints" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "checkpoint" } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "metrics" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "checkpoints" } }, { "kind": "Field", "name": { "kind": "Name", "value": "files" } }, { "kind": "Field", "name": { "kind": "Name", "value": "lines" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "added" } }, { "kind": "Field", "name": { "kind": "Name", "value": "removed" } }] } }] } }] } }, { "kind": "Field", "name": { "kind": "Name", "value": "policies" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "name" } }, { "kind": "Field", "name": { "kind": "Name", "value": "description" } }, { "kind": "Field", "name": { "kind": "Name", "value": "scopes" } }, { "kind": "Field", "name": { "kind": "Name", "value": "statutes" }, "selectionSet": { "kind": "SelectionSet", "selections": [{ "kind": "Field", "name": { "kind": "Name", "value": "id" } }, { "kind": "Field", "name": { "kind": "Name", "value": "priority" } }, { "kind": "Field", "name": { "kind": "Name", "value": "autofixable" } }, { "kind": "Field", "name": { "kind": "Name", "value": "reason" } }, { "kind": "Field", "name": { "kind": "Name", "value": "solution" } }] } }] } }] } }] } }] } as unknown as DocumentNode<CodebaseQuery, CodebaseQueryVariables>;