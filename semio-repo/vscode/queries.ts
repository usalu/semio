// #region 🔖Header

// [💻semio-repo/vscode/queries.ts](semiorepo://file/semio-repo/vscode/queries.ts)

// 2026 Ueli Saluz <ueli@semio-tech.de>

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

// GraphQL query document constants for the VS Code extension.

// #endregion 🔖Header

//#region 🔖Queries
// Typed GraphQL document constants MUST use generated graphql tag functions.

import { graphql } from "./generated/gql";

// GraphQL document for querying the full repo structure with projects and bundles.
export const RepoStructureDocument = graphql(`
  query RepoStructure {
    repo {
      id
      name
      path
      projects {
        id
        name
        kind
        root
        bundles {
          id
          name
          kind
          root
          sourceRoot
          projectType
          tags
          uri
        }
      }
      bundles {
        id
        name
        kind
        root
        sourceRoot
        projectType
        tags
        uri
      }
    }
  }
`);

// GraphQL document for querying recent repo commits.
export const RepoCommitsDocument = graphql(`
  query RepoCommits {
    repo {
      commits(limit: 100) {
        id
        sha
        title
        date
      }
    }
  }
`);

// GraphQL document for querying folder contents by path.
export const FolderContentDocument = graphql(`
  query FolderContent($path: String!) {
    folder(path: $path) {
      children {
        path
        name
        uri
      }
      files {
        path
        name
        uri
      }
    }
  }
`);

// GraphQL document for querying all bundles in the repo.
export const BundlesDocument = graphql(`
  query Bundles {
    repo {
      bundles { id name root sourceRoot projectType tags uri }
    }
  }
`);

// GraphQL document for querying tickets with filtering by date and status.
export const TicketsDocument = graphql(`
  query Tickets($year: Int, $month: Int, $day: Int, $status: TicketStatus) {
    repo {
      tickets(year: $year, month: $month, day: $day, status: $status) {
        id year month day slug path uri prompt summary status
        author { github name }
        llm commit
        goal
        dates { started finished }
        interactions {
          prompt
          system { client version }
          author
          dates {
            started
            finished
          }
          commit
        }
      }
    }
  }
`);

// GraphQL document for querying policies and their violation kinds.
export const PoliciesDocument = graphql(`
  query Policies {
    repo {
      policies { id name description scopes violationKinds { id priority autofixable reason solution } }
    }
  }
`);

// GraphQL document for querying contributors with their contributions.
export const ContributorsDocument = graphql(`
  query Contributors {
    repo {
      contributors {
        id github name emails
        links { name url }
        icons { avatar avatarRound github }
        contributions {
          commits {
            id sha title
          }
          tickets {
            year
            months {
              month
              days {
                day
                tickets { id slug title status }
              }
            }
          }
          bundles {
            name
            folders {
              name
              files {
                name
                sections {
                  name
                  definitions {
                    name
                  }
                }
              }
            }
          }
        }
      }
    }
  }
`);

// GraphQL document for running codebase analysis with optional scope.
export const AnalyzeDocument = graphql(`
  query Analyze($scope: String) {
    analyze(scope: $scope) {
      violations {
        id summary priority autofixable scope line column excerpt
        kind { id policy { id name } reason solution }
      }
      metrics { total byPriority { high medium low } autofixable }
    }
  }
`);

// GraphQL mutation document for applying autofixes with optional scope.
export const FixDocument = graphql(`
  mutation Fix($scope: String) {
    fix(scope: $scope) {
      fixed remaining
      violations { id summary priority scope }
    }
  }
`);

// GraphQL document for querying file content with sections and definitions.
export const FileContentDocument = graphql(`
  query FileContent($path: String!) {
    file(path: $path) {
      path
      name
      uri
      sections {
        id
        name
        range { start end }
        parent { id }
        ... on Section {
          children {
            id
            name
            range { start end }
            ... on Section {
              children {
                id
                name
                range { start end }
                ... on Section {
                  children {
                    id
                    name
                    range { start end }
                  }
                }
              }
            }
          }
        }
      }
      definitions {
        id
        name
        kind
        range { start end }
        section { id name }
      }
    }
  }
`);

// GraphQL document for querying all goals in the repo.
export const GoalsDocument = graphql(`
  query Goals {
    repo {
      goals {
        id
        title
        description
        prompt
        status
        dueDate
        client
        llm
        milestone
      }
    }
  }
`);

//#endregion Queries
