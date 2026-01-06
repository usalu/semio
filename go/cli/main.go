// #region Header

// go/cli/main.go

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

package main

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"path/filepath"
	"strconv"

	"github.com/spf13/cobra"
	"github.com/usalu/semio/go/repo"
)

// #region Init

var executor *repo.Executor

func init() {
	wd, _ := os.Getwd()
	rootDir := findRepoRoot(wd)
	repo.SetRootDir(rootDir)
	ctx := repo.NewRepoContext(rootDir)
	var err error
	executor, err = repo.NewExecutorWithContext(rootDir, ctx)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Failed to initialize GraphQL executor: %v\n", err)
		os.Exit(1)
	}
}

func findRepoRoot(start string) string {
	dir := start
	for {
		if _, err := os.Stat(filepath.Join(dir, "AGENTS.md")); err == nil {
			return dir
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			return start
		}
		dir = parent
	}
}

// #endregion Init

// #region GraphQL Helpers

func gql(query string, variables map[string]interface{}) (string, error) {
	return executor.ExecuteJSON(context.Background(), query, variables)
}

func printGQL(query string, variables map[string]interface{}) error {
	result, err := gql(query, variables)
	if err != nil {
		return err
	}
	fmt.Println(result)
	return nil
}

// #endregion GraphQL Helpers

// #region Commands

func main() {
	if err := Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func Execute() error {
	return rootCmd.Execute()
}

var rootCmd = &cobra.Command{
	Use:   "repo",
	Short: "Monorepo CLI for Semio",
	Long:  `repo - Monorepo CLI for Semio. All commands output JSON via GraphQL.`,
}

func init() {
	rootCmd.AddCommand(graphqlCmd)
	rootCmd.AddCommand(analyzeCmd)
	rootCmd.AddCommand(fixCmd)
	rootCmd.AddCommand(policyCmd)
	rootCmd.AddCommand(ticketCmd)
	rootCmd.AddCommand(contributorCmd)
	rootCmd.AddCommand(bundleCmd)
	rootCmd.AddCommand(folderCmd)
	rootCmd.AddCommand(fileCmd)
	rootCmd.AddCommand(sectionCmd)
}

// #region GraphQL Command

var graphqlCmd = &cobra.Command{
	Use:   "graphql <query>",
	Short: "Execute a GraphQL query",
	Long:  `Execute a GraphQL query against the repo schema and return JSON result.`,
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		query := args[0]
		variablesJSON, _ := cmd.Flags().GetString("variables")
		var variables map[string]interface{}
		if variablesJSON != "" {
			if err := json.Unmarshal([]byte(variablesJSON), &variables); err != nil {
				return fmt.Errorf("invalid variables JSON: %w", err)
			}
		}
		return printGQL(query, variables)
	},
}

func init() {
	graphqlCmd.Flags().StringP("variables", "v", "", "JSON object with query variables")
}

// #endregion GraphQL Command

// #region Serve Command

var serveCmd = &cobra.Command{
	Use:   "serve",
	Short: "Start GraphQL server with GraphiQL interface",
	Long:  `Start an HTTP server exposing the GraphQL API with introspection and GraphiQL interface.`,
	RunE: func(cmd *cobra.Command, args []string) error {
		port, _ := cmd.Flags().GetInt("port")
		addr := fmt.Sprintf(":%d", port)

		http.HandleFunc("/graphql", graphqlHandler)
		http.HandleFunc("/", graphiqlHandler)

		fmt.Printf("GraphQL server running at http://localhost%s/graphql\n", addr)
		fmt.Printf("GraphiQL interface at http://localhost%s/\n", addr)
		return http.ListenAndServe(addr, nil)
	},
}

func graphqlHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	w.Header().Set("Access-Control-Allow-Origin", "*")
	w.Header().Set("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
	w.Header().Set("Access-Control-Allow-Headers", "Content-Type")

	if r.Method == http.MethodOptions {
		w.WriteHeader(http.StatusOK)
		return
	}

	var request struct {
		Query         string                 `json:"query"`
		Variables     map[string]interface{} `json:"variables"`
		OperationName string                 `json:"operationName"`
	}

	if err := json.NewDecoder(r.Body).Decode(&request); err != nil {
		http.Error(w, `{"errors":[{"message":"Invalid JSON"}]}`, http.StatusBadRequest)
		return
	}

	result, err := executor.Execute(r.Context(), request.Query, request.Variables)
	response := map[string]interface{}{}
	if err != nil {
		response["errors"] = []map[string]string{{"message": err.Error()}}
	} else {
		response["data"] = result
	}

	json.NewEncoder(w).Encode(response)
}

func graphiqlHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/html")
	w.Write([]byte(graphiqlHTML))
}

const graphiqlHTML = `<!DOCTYPE html>
<html>
<head>
  <title>GraphiQL - Semio Repo</title>
  <style>
    body { height: 100%; margin: 0; width: 100%; overflow: hidden; }
    #graphiql { height: 100vh; }
  </style>
  <script src="https://cdn.jsdelivr.net/npm/react@18.2.0/umd/react.production.min.js" crossorigin></script>
  <script src="https://cdn.jsdelivr.net/npm/react-dom@18.2.0/umd/react-dom.production.min.js" crossorigin></script>
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/graphiql@3.0.10/graphiql.min.css" />
</head>
<body>
  <div id="graphiql">Loading...</div>
  <script src="https://cdn.jsdelivr.net/npm/graphiql@3.0.10/graphiql.min.js" crossorigin></script>
  <script>
    const root = ReactDOM.createRoot(document.getElementById('graphiql'));
    root.render(
      React.createElement(GraphiQL, {
        fetcher: async (graphQLParams) => {
          const response = await fetch('/graphql', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(graphQLParams),
          });
          return response.json();
        },
      }),
    );
  </script>
</body>
</html>`

func init() {
	serveCmd.Flags().IntP("port", "p", 8080, "Port to listen on")
	rootCmd.AddCommand(serveCmd)
}

// #endregion Serve Command

// #region Analyze Command

var analyzeCmd = &cobra.Command{
	Use:   "analyze [scope]",
	Short: "Analyze codebase for violations",
	RunE: func(cmd *cobra.Command, args []string) error {
		var scope *string
		if len(args) > 0 {
			scope = &args[0]
		}
		variables := map[string]interface{}{}
		if scope != nil {
			variables["scope"] = *scope
		}
		return printGQL(`
			query Analyze($scope: String) {
				analyze(scope: $scope) {
					violations {
						id
						scope
						line
						column
						excerpt
						kind { id priority autofixable reason solution }
						autofix { description edits { path edits { start end newText } } }
					}
					metrics { total autofixable byPriority { high medium low } }
				}
			}
		`, variables)
	},
}

// #endregion Analyze Command

// #region Fix Command

var fixCmd = &cobra.Command{
	Use:   "fix [scope]",
	Short: "Apply autofixes for violations",
	RunE: func(cmd *cobra.Command, args []string) error {
		var scope *string
		if len(args) > 0 {
			scope = &args[0]
		}
		variables := map[string]interface{}{}
		if scope != nil {
			variables["scope"] = *scope
		}
		return printGQL(`
			mutation Fix($scope: String) {
				fix(scope: $scope) {
					fixed
					remaining
					violations {
						id
						scope
						excerpt
						line
					}
				}
			}
		`, variables)
	},
}

// #endregion Fix Command

// #region Policy Commands

var policyCmd = &cobra.Command{
	Use:   "policy",
	Short: "Policy management commands",
}

var policyListCmd = &cobra.Command{
	Use:   "list",
	Short: "List all registered policies",
	RunE: func(cmd *cobra.Command, args []string) error {
		return printGQL(`
			query Policies {
				repo {
					policies {
						id
						name
						description
						scopes
						violationKinds { id priority autofixable reason solution }
					}
				}
			}
		`, nil)
	},
}

func init() {
	policyCmd.AddCommand(policyListCmd)
}

// #endregion Policy Commands

// #region Ticket Commands

var ticketCmd = &cobra.Command{
	Use:   "ticket",
	Short: "Ticket management commands",
}

var ticketCreateCmd = &cobra.Command{
	Use:   "create <slug>",
	Short: "Create a new ticket",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		prompt, _ := cmd.Flags().GetString("prompt")
		model, _ := cmd.Flags().GetString("model")
		files, _ := cmd.Flags().GetStringSlice("file")
		input := map[string]interface{}{
			"slug":   args[0],
			"prompt": prompt,
		}
		if model != "" {
			input["model"] = model
		}
		if len(files) > 0 {
			input["files"] = map[string]interface{}{
				"updated": files,
			}
		}
		variables := map[string]interface{}{
			"input": input,
		}
		return printGQL(`
			mutation TicketCreate($input: TicketCreateInput!) {
				ticketCreate(input: $input) {
					id
					slug
					status
					path
					uri
				}
			}
		`, variables)
	},
}

var ticketListCmd = &cobra.Command{
	Use:   "list [year] [month] [day]",
	Short: "List tickets",
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{}
		if len(args) > 0 {
			y, _ := strconv.Atoi(args[0])
			variables["year"] = y
		}
		if len(args) > 1 {
			m, _ := strconv.Atoi(args[1])
			variables["month"] = m
		}
		if len(args) > 2 {
			d, _ := strconv.Atoi(args[2])
			variables["day"] = d
		}
		return printGQL(`
			query Tickets($year: Int, $month: Int, $day: Int) {
				repo {
					tickets(year: $year, month: $month, day: $day) {
						id
						year
						month
						day
						slug
						status
						prompt
						summary
						path
						uri
						date { created finished }
						metrics { iterations bundles files lines { added removed } }
					}
				}
			}
		`, variables)
	},
}

var ticketProgressCmd = &cobra.Command{
	Use:   "progress <year> <month> <day> <slug>",
	Short: "Record progress on a ticket",
	Args:  cobra.ExactArgs(4),
	RunE: func(cmd *cobra.Command, args []string) error {
		prompt, _ := cmd.Flags().GetString("prompt")
		model, _ := cmd.Flags().GetString("model")
		files, _ := cmd.Flags().GetStringSlice("file")
		year, _ := strconv.Atoi(args[0])
		month, _ := strconv.Atoi(args[1])
		day, _ := strconv.Atoi(args[2])
		input := map[string]interface{}{
			"year":   year,
			"month":  month,
			"day":    day,
			"slug":   args[3],
			"prompt": prompt,
		}
		if model != "" {
			input["model"] = model
		}
		if len(files) > 0 {
			input["files"] = map[string]interface{}{
				"updated": files,
			}
		}
		variables := map[string]interface{}{
			"input": input,
		}
		return printGQL(`
			mutation TicketProgress($input: TicketProgressInput!) {
				ticketProgress(input: $input) {
					id
					slug
					status
					prompt
					model
					date { created finished }
				}
			}
		`, variables)
	},
}

var ticketFinishCmd = &cobra.Command{
	Use:   "finish <year> <month> <day> <slug>",
	Short: "Finish a ticket",
	Args:  cobra.ExactArgs(4),
	RunE: func(cmd *cobra.Command, args []string) error {
		summary, _ := cmd.Flags().GetString("summary")
		year, _ := strconv.Atoi(args[0])
		month, _ := strconv.Atoi(args[1])
		day, _ := strconv.Atoi(args[2])
		input := map[string]interface{}{
			"year":  year,
			"month": month,
			"day":   day,
			"slug":  args[3],
		}
		if summary != "" {
			input["summary"] = summary
		}
		variables := map[string]interface{}{
			"input": input,
		}
		return printGQL(`
			mutation TicketFinish($input: TicketFinishInput!) {
				ticketFinish(input: $input) {
					id
					slug
					status
					date { created finished }
				}
			}
		`, variables)
	},
}

var ticketReopenCmd = &cobra.Command{
	Use:   "reopen <year> <month> <day> <slug>",
	Short: "Reopen a ticket",
	Args:  cobra.ExactArgs(4),
	RunE: func(cmd *cobra.Command, args []string) error {
		year, _ := strconv.Atoi(args[0])
		month, _ := strconv.Atoi(args[1])
		day, _ := strconv.Atoi(args[2])
		variables := map[string]interface{}{
			"input": map[string]interface{}{
				"year": year,
				"month": month,
				"day": day,
				"slug": args[3],
			},
		}
		return printGQL(`
			mutation TicketReopen($input: TicketReopenInput!) {
				ticketReopen(input: $input) {
					id
					slug
					status
				}
			}
		`, variables)
	},
}

func init() {
	ticketCreateCmd.Flags().String("prompt", "", "Ticket prompt")
	ticketCreateCmd.Flags().String("model", "", "Model used")
	ticketCreateCmd.Flags().StringSlice("file", nil, "Files to include")
	ticketProgressCmd.Flags().String("prompt", "", "Iteration prompt")
	ticketProgressCmd.Flags().String("model", "", "Model used")
	ticketProgressCmd.Flags().StringSlice("file", nil, "Files to include")
	ticketFinishCmd.Flags().String("summary", "", "Ticket summary")
	ticketCmd.AddCommand(ticketCreateCmd)
	ticketCmd.AddCommand(ticketListCmd)
	ticketCmd.AddCommand(ticketProgressCmd)
	ticketCmd.AddCommand(ticketFinishCmd)
	ticketCmd.AddCommand(ticketReopenCmd)
}

// #endregion Ticket Commands

// #region Contributor Commands

var contributorCmd = &cobra.Command{
	Use:   "contributor",
	Short: "Contributor management commands",
}

var contributorListCmd = &cobra.Command{
	Use:   "list",
	Short: "List contributors",
	RunE: func(cmd *cobra.Command, args []string) error {
		return printGQL(`
			query Contributors {
				repo {
					contributors {
						id
						github
						name
						emails
						icons { avatar avatarRound github }
						links { name url }
						metrics { commits tickets bundles folders files sections definitions lines }
					}
				}
			}
		`, nil)
	},
}

var contributorAddCmd = &cobra.Command{
	Use:   "add <github>",
	Short: "Add a contributor",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		name, _ := cmd.Flags().GetString("name")
		emails, _ := cmd.Flags().GetStringSlice("email")
		input := map[string]interface{}{
			"github": args[0],
		}
		if name != "" {
			input["name"] = name
		}
		if len(emails) > 0 {
			input["emails"] = emails
		}
		variables := map[string]interface{}{
			"input": input,
		}
		return printGQL(`
			mutation ContributorAdd($input: ContributorAddInput!) {
				contributorAdd(input: $input) {
					id
					github
					name
					emails
				}
			}
		`, variables)
	},
}

var contributorRemoveCmd = &cobra.Command{
	Use:   "remove <github>",
	Short: "Remove a contributor",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"github": args[0],
		}
		return printGQL(`
			mutation ContributorRemove($github: String!) {
				contributorRemove(github: $github)
			}
		`, variables)
	},
}

func init() {
	contributorAddCmd.Flags().String("name", "", "Contributor name")
	contributorAddCmd.Flags().StringSlice("email", nil, "Contributor emails")
	contributorCmd.AddCommand(contributorListCmd)
	contributorCmd.AddCommand(contributorAddCmd)
	contributorCmd.AddCommand(contributorRemoveCmd)
}

// #endregion Contributor Commands

// #region Bundle Commands

var bundleCmd = &cobra.Command{
	Use:   "bundle",
	Short: "Bundle management commands",
}

var bundleListCmd = &cobra.Command{
	Use:   "list",
	Short: "List Nx bundles",
	RunE: func(cmd *cobra.Command, args []string) error {
		return printGQL(`
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
		`, nil)
	},
}

func init() {
	bundleCmd.AddCommand(bundleListCmd)
}

// #endregion Bundle Commands

// #region Folder Commands

var folderCmd = &cobra.Command{
	Use:   "folder",
	Short: "Folder management commands",
}

var folderCreateCmd = &cobra.Command{
	Use:   "create <path>",
	Short: "Create a folder",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"path": args[0],
		}
		return printGQL(`
			mutation FolderCreate($path: String!) {
				folderCreate(path: $path) {
					id
					path
					name
					uri
				}
			}
		`, variables)
	},
}

var folderMoveCmd = &cobra.Command{
	Use:   "move <source> <target>",
	Short: "Move a folder",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"src": args[0],
			"dst": args[1],
		}
		return printGQL(`
			mutation FolderMove($src: String!, $dst: String!) {
				folderMove(src: $src, dst: $dst) {
					id
					path
					name
					uri
				}
			}
		`, variables)
	},
}

var folderDeleteCmd = &cobra.Command{
	Use:   "delete <path>",
	Short: "Delete a folder",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"path": args[0],
		}
		return printGQL(`
			mutation FolderDelete($path: String!) {
				folderDelete(path: $path)
			}
		`, variables)
	},
}

func init() {
	folderCmd.AddCommand(folderCreateCmd)
	folderCmd.AddCommand(folderMoveCmd)
	folderCmd.AddCommand(folderDeleteCmd)
}

// #endregion Folder Commands

// #region File Commands

var fileCmd = &cobra.Command{
	Use:   "file",
	Short: "File management commands",
}

var fileCreateCmd = &cobra.Command{
	Use:   "create <path>",
	Short: "Create a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"path": args[0],
		}
		return printGQL(`
			mutation FileCreate($path: String!) {
				fileCreate(path: $path) {
					id
					path
					name
					uri
				}
			}
		`, variables)
	},
}

var fileMoveCmd = &cobra.Command{
	Use:   "move <source> <target>",
	Short: "Move a file",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"src": args[0],
			"dst": args[1],
		}
		return printGQL(`
			mutation FileMove($src: String!, $dst: String!) {
				fileMove(src: $src, dst: $dst) {
					id
					path
					name
					uri
				}
			}
		`, variables)
	},
}

var fileDeleteCmd = &cobra.Command{
	Use:   "delete <path>",
	Short: "Delete a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"path": args[0],
		}
		return printGQL(`
			mutation FileDelete($path: String!) {
				fileDelete(path: $path)
			}
		`, variables)
	},
}

func init() {
	fileCmd.AddCommand(fileCreateCmd)
	fileCmd.AddCommand(fileMoveCmd)
	fileCmd.AddCommand(fileDeleteCmd)
}

// #endregion File Commands

// #region Section Commands

var sectionCmd = &cobra.Command{
	Use:   "section",
	Short: "Section management commands",
}

var sectionCreateCmd = &cobra.Command{
	Use:   "create <file> <name>",
	Short: "Create a section in a file",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		parent, _ := cmd.Flags().GetString("parent")
		variables := map[string]interface{}{
			"file": args[0],
			"name": args[1],
		}
		if parent != "" {
			variables["parent"] = parent
		}
		return printGQL(`
			mutation SectionCreate($file: String!, $name: String!, $parent: String) {
				sectionCreate(file: $file, name: $name, parent: $parent) {
					id
					name
					range { start { line column } end { line column } }
				}
			}
		`, variables)
	},
}

var sectionMoveCmd = &cobra.Command{
	Use:   "move <file> <old-name> <new-name>",
	Short: "Move/rename a section",
	Args:  cobra.ExactArgs(3),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"file":    args[0],
			"oldName": args[1],
			"newName": args[2],
		}
		return printGQL(`
			mutation SectionMove($file: String!, $oldName: String!, $newName: String!) {
				sectionMove(file: $file, oldName: $oldName, newName: $newName) {
					id
					name
					range { start { line column } end { line column } }
				}
			}
		`, variables)
	},
}

var sectionDeleteCmd = &cobra.Command{
	Use:   "delete <file> <name>",
	Short: "Delete a section from a file",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"file": args[0],
			"name": args[1],
		}
		return printGQL(`
			mutation SectionDelete($file: String!, $name: String!) {
				sectionDelete(file: $file, name: $name)
			}
		`, variables)
	},
}

var sectionListCmd = &cobra.Command{
	Use:   "list <file>",
	Short: "List sections in a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"path": args[0],
		}
		return printGQL(`
			query SectionList($path: String!) {
				file(path: $path) {
					sections {
						id
						name
						range { start { line column } end { line column } }
						children { id name }
					}
				}
			}
		`, variables)
	},
}

func init() {
	sectionCreateCmd.Flags().String("parent", "", "Parent section name")
	sectionCmd.AddCommand(sectionListCmd)
	sectionCmd.AddCommand(sectionCreateCmd)
	sectionCmd.AddCommand(sectionMoveCmd)
	sectionCmd.AddCommand(sectionDeleteCmd)
}

// #endregion Section Commands

// #region Definition Commands

var definitionCmd = &cobra.Command{
	Use:   "definition",
	Short: "Definition management commands",
}

var definitionListCmd = &cobra.Command{
	Use:   "list <file>",
	Short: "List definitions in a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		variables := map[string]interface{}{
			"path": args[0],
		}
		return printGQL(`
			query DefinitionList($path: String!) {
				file(path: $path) {
					definitions {
						id
						name
						kind
						range { start { line column } end { line column } }
					}
				}
			}
		`, variables)
	},
}

func init() {
	definitionCmd.AddCommand(definitionListCmd)
	rootCmd.AddCommand(definitionCmd)
}

// #endregion Definition Commands

// #endregion Commands
