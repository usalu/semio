/* eslint-disable */
import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 * Learn more about it here: https://the-guild.dev/graphql/codegen/plugins/presets/preset-client#reducing-bundle-size
 */
type Documents = {
    "\n  query Repo {\n    repo {\n      id\n      name\n      path\n      bundles { id name root sourceRoot projectType tags uri }\n      tickets { id year month day slug path uri prompt summary status commit }\n      policies { id name description scopes }\n      contributors { id github name emails }\n    }\n  }\n": typeof types.RepoDocument,
    "\n  query Bundles {\n    repo {\n      bundles { id name root sourceRoot projectType tags uri }\n    }\n  }\n": typeof types.BundlesDocument,
    "\n  query Tickets($year: Int, $month: Int, $day: Int, $status: TicketStatus) {\n    repo {\n      tickets(year: $year, month: $month, day: $day, status: $status) {\n        id year month day slug path uri prompt summary status\n        author { github name }\n        model commit\n        date { created finished }\n        checkpoints { prompt model author { github name } commit date { created } }\n        metrics { checkpoints files lines { added removed } }\n      }\n    }\n  }\n": typeof types.TicketsDocument,
    "\n  query Policies {\n    repo {\n      policies { id name description scopes violationKinds { id priority autofixable reason solution } }\n    }\n  }\n": typeof types.PoliciesDocument,
    "\n  query Contributors {\n    repo {\n      contributors {\n        id github name emails\n        links { name url }\n        icons { avatar avatarRound github }\n        metrics { commits tickets bundles folders files sections definitions lines }\n      }\n    }\n  }\n": typeof types.ContributorsDocument,
    "\n  query Analyze($scope: String) {\n    analyze(scope: $scope) {\n      violations {\n        id summary priority autofixable scope line column excerpt\n        kind { id policy { id name } reason solution }\n        autofix { description }\n      }\n      metrics { total byPriority { high medium low } autofixable }\n    }\n  }\n": typeof types.AnalyzeDocument,
    "\n  mutation Fix($scope: String) {\n    fix(scope: $scope) {\n      fixed remaining\n      violations { id summary priority scope }\n    }\n  }\n": typeof types.FixDocument,
    "\n  query Codebase {\n    repo {\n      id name path\n      bundles {\n        id name root sourceRoot projectType tags uri\n        metrics { folders files sections definitions lines violations }\n      }\n      folders {\n        id path uri\n        metrics { files lines violations }\n      }\n      files {\n        id path uri\n        metrics { sections definitions lines }\n        sections {\n          id name path\n          range { start { line } end { line } }\n          metrics { definitions lines violations }\n        }\n        definitions {\n          id name kind\n          range { start { line } end { line } }\n          metrics { definitions lines violations }\n        }\n      }\n      contributors {\n        id github name emails\n        links { name url }\n        metrics { commits tickets bundles folders files sections definitions lines }\n      }\n      tickets {\n        id year month day slug path uri prompt summary status commit\n        author { github name }\n        checkpoints { commit }\n        metrics { checkpoints files lines { added removed } }\n      }\n      policies {\n        id name description scopes\n        violationKinds { id priority autofixable reason solution }\n      }\n    }\n  }\n": typeof types.CodebaseDocument,
};
const documents: Documents = {
    "\n  query Repo {\n    repo {\n      id\n      name\n      path\n      bundles { id name root sourceRoot projectType tags uri }\n      tickets { id year month day slug path uri prompt summary status commit }\n      policies { id name description scopes }\n      contributors { id github name emails }\n    }\n  }\n": types.RepoDocument,
    "\n  query Bundles {\n    repo {\n      bundles { id name root sourceRoot projectType tags uri }\n    }\n  }\n": types.BundlesDocument,
    "\n  query Tickets($year: Int, $month: Int, $day: Int, $status: TicketStatus) {\n    repo {\n      tickets(year: $year, month: $month, day: $day, status: $status) {\n        id year month day slug path uri prompt summary status\n        author { github name }\n        model commit\n        date { created finished }\n        checkpoints { prompt model author { github name } commit date { created } }\n        metrics { checkpoints files lines { added removed } }\n      }\n    }\n  }\n": types.TicketsDocument,
    "\n  query Policies {\n    repo {\n      policies { id name description scopes violationKinds { id priority autofixable reason solution } }\n    }\n  }\n": types.PoliciesDocument,
    "\n  query Contributors {\n    repo {\n      contributors {\n        id github name emails\n        links { name url }\n        icons { avatar avatarRound github }\n        metrics { commits tickets bundles folders files sections definitions lines }\n      }\n    }\n  }\n": types.ContributorsDocument,
    "\n  query Analyze($scope: String) {\n    analyze(scope: $scope) {\n      violations {\n        id summary priority autofixable scope line column excerpt\n        kind { id policy { id name } reason solution }\n        autofix { description }\n      }\n      metrics { total byPriority { high medium low } autofixable }\n    }\n  }\n": types.AnalyzeDocument,
    "\n  mutation Fix($scope: String) {\n    fix(scope: $scope) {\n      fixed remaining\n      violations { id summary priority scope }\n    }\n  }\n": types.FixDocument,
    "\n  query Codebase {\n    repo {\n      id name path\n      bundles {\n        id name root sourceRoot projectType tags uri\n        metrics { folders files sections definitions lines violations }\n      }\n      folders {\n        id path uri\n        metrics { files lines violations }\n      }\n      files {\n        id path uri\n        metrics { sections definitions lines }\n        sections {\n          id name path\n          range { start { line } end { line } }\n          metrics { definitions lines violations }\n        }\n        definitions {\n          id name kind\n          range { start { line } end { line } }\n          metrics { definitions lines violations }\n        }\n      }\n      contributors {\n        id github name emails\n        links { name url }\n        metrics { commits tickets bundles folders files sections definitions lines }\n      }\n      tickets {\n        id year month day slug path uri prompt summary status commit\n        author { github name }\n        checkpoints { commit }\n        metrics { checkpoints files lines { added removed } }\n      }\n      policies {\n        id name description scopes\n        violationKinds { id priority autofixable reason solution }\n      }\n    }\n  }\n": types.CodebaseDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function graphql(source: string): unknown;

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query Repo {\n    repo {\n      id\n      name\n      path\n      bundles { id name root sourceRoot projectType tags uri }\n      tickets { id year month day slug path uri prompt summary status commit }\n      policies { id name description scopes }\n      contributors { id github name emails }\n    }\n  }\n"): (typeof documents)["\n  query Repo {\n    repo {\n      id\n      name\n      path\n      bundles { id name root sourceRoot projectType tags uri }\n      tickets { id year month day slug path uri prompt summary status commit }\n      policies { id name description scopes }\n      contributors { id github name emails }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query Bundles {\n    repo {\n      bundles { id name root sourceRoot projectType tags uri }\n    }\n  }\n"): (typeof documents)["\n  query Bundles {\n    repo {\n      bundles { id name root sourceRoot projectType tags uri }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query Tickets($year: Int, $month: Int, $day: Int, $status: TicketStatus) {\n    repo {\n      tickets(year: $year, month: $month, day: $day, status: $status) {\n        id year month day slug path uri prompt summary status\n        author { github name }\n        model commit\n        date { created finished }\n        checkpoints { prompt model author { github name } commit date { created } }\n        metrics { checkpoints files lines { added removed } }\n      }\n    }\n  }\n"): (typeof documents)["\n  query Tickets($year: Int, $month: Int, $day: Int, $status: TicketStatus) {\n    repo {\n      tickets(year: $year, month: $month, day: $day, status: $status) {\n        id year month day slug path uri prompt summary status\n        author { github name }\n        model commit\n        date { created finished }\n        checkpoints { prompt model author { github name } commit date { created } }\n        metrics { checkpoints files lines { added removed } }\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query Policies {\n    repo {\n      policies { id name description scopes violationKinds { id priority autofixable reason solution } }\n    }\n  }\n"): (typeof documents)["\n  query Policies {\n    repo {\n      policies { id name description scopes violationKinds { id priority autofixable reason solution } }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query Contributors {\n    repo {\n      contributors {\n        id github name emails\n        links { name url }\n        icons { avatar avatarRound github }\n        metrics { commits tickets bundles folders files sections definitions lines }\n      }\n    }\n  }\n"): (typeof documents)["\n  query Contributors {\n    repo {\n      contributors {\n        id github name emails\n        links { name url }\n        icons { avatar avatarRound github }\n        metrics { commits tickets bundles folders files sections definitions lines }\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query Analyze($scope: String) {\n    analyze(scope: $scope) {\n      violations {\n        id summary priority autofixable scope line column excerpt\n        kind { id policy { id name } reason solution }\n        autofix { description }\n      }\n      metrics { total byPriority { high medium low } autofixable }\n    }\n  }\n"): (typeof documents)["\n  query Analyze($scope: String) {\n    analyze(scope: $scope) {\n      violations {\n        id summary priority autofixable scope line column excerpt\n        kind { id policy { id name } reason solution }\n        autofix { description }\n      }\n      metrics { total byPriority { high medium low } autofixable }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation Fix($scope: String) {\n    fix(scope: $scope) {\n      fixed remaining\n      violations { id summary priority scope }\n    }\n  }\n"): (typeof documents)["\n  mutation Fix($scope: String) {\n    fix(scope: $scope) {\n      fixed remaining\n      violations { id summary priority scope }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query Codebase {\n    repo {\n      id name path\n      bundles {\n        id name root sourceRoot projectType tags uri\n        metrics { folders files sections definitions lines violations }\n      }\n      folders {\n        id path uri\n        metrics { files lines violations }\n      }\n      files {\n        id path uri\n        metrics { sections definitions lines }\n        sections {\n          id name path\n          range { start { line } end { line } }\n          metrics { definitions lines violations }\n        }\n        definitions {\n          id name kind\n          range { start { line } end { line } }\n          metrics { definitions lines violations }\n        }\n      }\n      contributors {\n        id github name emails\n        links { name url }\n        metrics { commits tickets bundles folders files sections definitions lines }\n      }\n      tickets {\n        id year month day slug path uri prompt summary status commit\n        author { github name }\n        checkpoints { commit }\n        metrics { checkpoints files lines { added removed } }\n      }\n      policies {\n        id name description scopes\n        violationKinds { id priority autofixable reason solution }\n      }\n    }\n  }\n"): (typeof documents)["\n  query Codebase {\n    repo {\n      id name path\n      bundles {\n        id name root sourceRoot projectType tags uri\n        metrics { folders files sections definitions lines violations }\n      }\n      folders {\n        id path uri\n        metrics { files lines violations }\n      }\n      files {\n        id path uri\n        metrics { sections definitions lines }\n        sections {\n          id name path\n          range { start { line } end { line } }\n          metrics { definitions lines violations }\n        }\n        definitions {\n          id name kind\n          range { start { line } end { line } }\n          metrics { definitions lines violations }\n        }\n      }\n      contributors {\n        id github name emails\n        links { name url }\n        metrics { commits tickets bundles folders files sections definitions lines }\n      }\n      tickets {\n        id year month day slug path uri prompt summary status commit\n        author { github name }\n        checkpoints { commit }\n        metrics { checkpoints files lines { added removed } }\n      }\n      policies {\n        id name description scopes\n        violationKinds { id priority autofixable reason solution }\n      }\n    }\n  }\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;