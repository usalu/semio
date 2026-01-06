// #region Header

// js/vscode/queries.ts

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

import { graphql } from "./generated/gql";

// #endregion Imports

// #region Queries

export const RepoQuery = graphql(`
  query Repo {
    repo {
      id
      name
      path
      bundles {
        id
        name
        root
        sourceRoot
        projectType
        tags
        uri
      }
      tickets {
        id
        year
        month
        day
        slug
        path
        uri
        prompt
        summary
        status
        commit
      }
      policies {
        id
        name
        description
        scopes
      }
      contributors {
        id
        github
        name
        emails
      }
    }
  }
`);

export const BundlesQuery = graphql(`
  query Bundles {
    repo {
      bundles {
        id
        name
        root
        sourceRoot
        projectType
        tags
        uri
      }
    }
  }
`);

export const TicketsQuery = graphql(`
  query Tickets($year: Int, $month: Int, $day: Int, $status: TicketStatus) {
    repo {
      tickets(year: $year, month: $month, day: $day, status: $status) {
        id
        year
        month
        day
        slug
        path
        uri
        prompt
        summary
        status
        author {
          github
          name
        }
        model
        commit
        date {
          created
          finished
        }
        checkpoints {
          prompt
          model
          author {
            github
            name
          }
          commit
          date {
            created
          }
        }
        metrics {
          checkpoints
          files
          lines {
            added
            removed
          }
        }
      }
    }
  }
`);

export const PoliciesQuery = graphql(`
  query Policies {
    repo {
      policies {
        id
        name
        description
        scopes
        violationKinds {
          id
          priority
          autofixable
          reason
          solution
        }
      }
    }
  }
`);

export const ContributorsQuery = graphql(`
  query Contributors {
    repo {
      contributors {
        id
        github
        name
        emails
        links {
          name
          url
        }
        icons {
          avatar
          avatarRound
          github
        }
        metrics {
          commits
          tickets
          bundles
          folders
          files
          sections
          definitions
          lines
        }
      }
    }
  }
`);

export const AnalyzeQuery = graphql(`
  query Analyze($scope: String) {
    analyze(scope: $scope) {
      violations {
        id
        summary
        priority
        autofixable
        scope
        line
        column
        excerpt
        kind {
          id
          policy {
            id
            name
          }
          reason
          solution
        }
        autofix {
          description
        }
      }
      metrics {
        total
        byPriority {
          high
          medium
          low
        }
        autofixable
      }
    }
  }
`);

// #endregion Queries

// #region Mutations

export const FixMutation = graphql(`
  mutation Fix($scope: String) {
    fix(scope: $scope) {
      fixed
      remaining
      violations {
        id
        summary
        priority
        scope
      }
    }
  }
`);

// #endregion Mutations
