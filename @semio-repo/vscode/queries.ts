import { graphql } from "./generated/gql";

export const RepoDocument = graphql(`
  query Repo {
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
          root
          sourceRoot
          projectType
          tags
          uri
        }
      }
      commits(limit: 100) {
        id
        sha
        title
        date
      }
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
        goal
        author {
          name
          github
        }
      }
      goals {
        id
        title
        description
        status
      }
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
      contributors {
        id
        github
        name
        emails
        links {
          name
          url
        }
        contributions {
           commits {
             id 
             sha
             title
             date
           }
        }
      }
    }
  }
`);

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

export const BundlesDocument = graphql(`
  query Bundles {
    repo {
      bundles { id name root sourceRoot projectType tags uri }
    }
  }
`);

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
          llm
          client
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

export const PoliciesDocument = graphql(`
  query Policies {
    repo {
      policies { id name description scopes violationKinds { id priority autofixable reason solution } }
    }
  }
`);

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
            slug year month day title summary status
          }
          bundles {
            name
            lines { added removed }
            folders {
              name
              lines { added removed }
              files {
                name
                lines { added removed }
                sections {
                  name
                  lines { added removed }
                  definitions {
                    name
                    lines { added removed }
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

export const FixDocument = graphql(`
  mutation Fix($scope: String) {
    fix(scope: $scope) {
      fixed remaining
      violations { id summary priority scope }
    }
  }
`);

export const CodebaseDocument = graphql(`
  query Codebase {
    repo {
      id name path
      bundles {
        id name root sourceRoot projectType tags uri
      }
      folders {
        id path uri
      }
      files {
        id path uri
        sections {
          id name path
          range { start end }
        }
        definitions {
          id name kind
          range { start end }
        }
      }
      contributors {
        id github name emails
        links { name url }
      }
      tickets {
        id year month day slug path uri prompt summary status commit
        author { github name }
      }
      policies {
        id name description scopes
        violationKinds { id priority autofixable reason solution }
      }
    }
  }
`);

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
      }
      definitions {
        id
        name
        kind
        range { start end }
        section { id }
      }
    }
  }
`);

export const TodosDocument = graphql(`
  query Todos($filter: FilterInput) {
    todos(filter: $filter) {
      id
      name
      location {
        filePath
        line
        column
      }
    }
  }
`);

export const TodoCreateDocument = graphql(`
  mutation TodoCreate($input: TodoCreateInput!) {
    todoCreate(input: $input) {
      id
    }
  }
`);

export const TodoDeleteDocument = graphql(`
  mutation TodoDelete($id: ID!) {
    todoDelete(id: $id)
  }
`);

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
        ui
        llm
        milestone
      }
    }
  }
`);
