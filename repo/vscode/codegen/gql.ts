// #region 🧲Header

// 💻repo/vscode/codegen/gql.ts

// 2026 Ueli Saluz <ueli@semio-tech.de>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or \(at your option\) any later version\. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE\.  See the GNU Affero General Public License for more details\. You should have received a copy of the GNU Affero General Public License along with this program\.  If not, see <https://www\.gnu\.org/licenses/>\.

// #region 🎯Requirements
// #endregion 🎯Requirements

// #endregion 🧲Header


import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export function graphql(source: string): unknown;
export function graphql(source: "\n  query Repo {\n    repo {\n      id\n      name\n      path\n      bundles { id name root s  bundles { id name root sourceRoot projectType tags uri }\n      tickets { id year month day slug path uri prompt summary status checkpoint }\n      policies { id name description scopes }\n      contributors { id github name emails }\n    }\n  }\n"];
export function graphql(source: "\n  query Tickets($year: Int, $month: Int, $day: Int, $status: TicketStatus) {\n    repo {\n      tickets(year: $year, month: $month, day: $day, status: $status) {\n        id year month day slug path uri prompt summary status\n        author { github name }\n        model checkpoint\n        date { created finished }\n        checkpoints { prompt model author { github name } checkpoint date { created } }\n        metrics { checkpoints files lines { added removed } }\n      }\n    }\n  }\n"): tickets(year: $year, month: $month, day: $day, status: $status) { \n        id year month day slug path uri prompt summary status\n        author { github name } \n        model checkpoint\n        date { created finished } \n        checkpoints { prompt model author { github name } checkpoint date { created } } \n        metrics { checkpoints files lines { added removed } } \n }\n    }\n  }\n"];
/**
 *  function graphql(source: "\n  query Policies {\n    repo {\n      policies { id name description scopes statutes { id priority autofixable reason solution } }\n    }\n  }\n"): (typeof documents)["\n  query Policies {\n    repo {\n      policies { id name description scopes statutes { id priority autofixable reason solution } }\n    }\n  }\n"];
/**
 * 🕸️The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query Contributors {\n    repo {\n      contributors {\n        id github name emails\n        links { name url }\n        icons { avatar avatarRound github }\n        metrics { checkpoints tickets bundles folders files sections definitions lines }\n      }\n    }\n  }\n"): (typeof documents)["\n  query Contributors {\n    repo {\n      contributors {\n        id github name emails\n        links { name url }\n        icons { avatar avatarRound github }\n        metrics { checkpoints tickets bundles folders files sections definitions lines }\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a d\n    analyze(scope: $scope) {\n      breachs {\n        id summary priority autofixable scope line column excerpt\n        kind { id policy { id name } reason solution }\n        autofix { description }\n      }\n      metrics { total byPriority { high medium low } autofixable }\n    }\n  }\n"): (typeof documents)["\n  query Analyze($scope: String) {\n    analyze(scope: $scope) {\n      breachs {\n        id summary priority autofixable scope line column excerpt\n        kind{ high medium low } autofixable }\n    }\n  }\n"];
export function graphql(source: "\n  mutation Fix($scope: String) {\n    fix(scope: $scope) {\n      fixed remaining\n      breachs { id summary priority scope }\n    }\n  }\n"): (typeof documents)["\n  mutation Fix($scope: String) {\n    fix(scope: $scope) {\n      fixed remaining\n      breachs { id summary priority scope }\n    }\n  }\n"];
export function graphql(source: "\n  query Codebase {\n    repo {\n      id nam definitions lines breachs }\n      }\n      folders {\n        id path uri\n        metrics { files lines breachs }\n      }\n      files {\n        id path uri\n        metrics { sections definitions lines }\n        sections {\n          id name path\n          range { start { line } end { line } }\n          metrics { definitions lines breachs }\n        }\n        definitions {\n          id name kind\n          range { start { line } end { line } }\n          metrics { definitions lines breachs }\n        }\n      }\n      contributors {\n        id github name emails\n        links { name url }\n        metrics { checkpoints tickets bundles folders files sections definitions lines }\n      }\n      tickets {\n        id year month day slug path uri prompt summary status checkpoint\n        author { github name }\n        cheid name description scopes\n        statutes { id priority autofixable reason solution }\n      }\n    }\n  }\n"): (typeof documents)["\n  query Codebase {\n    repo {\n      id name path\n      bundles {\n        id name root sourceRoot projectType tags uri\n        metrics { folders files sections definitions lines breachs }\n      }\n      folders {\n        id path uri\n        metrics { files lines breachs }\n      }\n      files {\n        id path uri line } end { line } }\n          metrics { definitions lines breachs }\n        }\n        definitions {\n          id name kind\n          range { start { line } end { line } }\n          metrics { definitions lines breachs }\n        }\n      }\n      contributors {\n        id github name emails\n        links { name url }\n        metrics { checkpoints tickets bundles folders files sections definitions lines }\n      }\n      tickets {\n        id year month day slug path uri prompt summary status checkpoint\n        author { github name }\n        checkpoints { checkpoint }\n        metrics { checkpoints files lines { added removed } }\n      }\n      policies {\n        id name description scopes\n        statutes { id priority autofixable reason solution }\n      }\n    }\n  }\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;