// #region Header
// SPDX-License-Identifier: AGPL-3.0-or-later
// #endregion Header

// #region Package
package cli

// #endregion Package

// #region Imports
import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/spf13/cobra"
	mcpadapter "github.com/usalu/semio/go/repo/internal/adapters/mcp"
	"github.com/usalu/semio/go/repo/internal/core"
	"github.com/usalu/semio/go/repo/internal/events"
)

// #endregion Imports

// #region Types
type Config struct {
	Format  string
	Verbose bool
	Repo    string
	Timeout time.Duration
}

func mcpCommand(factory EngineFactory, config *Config) *cobra.Command {
	return &cobra.Command{
		Use:   "mcp",
		Short: "Run MCP server",
		RunE: func(cmd *cobra.Command, args []string) error {
			engine, err := factory(*config)
			if err != nil {
				return err
			}
			ctx := context.Background()
			if config.Timeout > 0 {
				ctxWithTimeout, cancel := context.WithTimeout(ctx, config.Timeout)
				defer cancel()
				ctx = ctxWithTimeout
			}
			return mcpadapter.Serve(ctx, engine)
		},
	}
}

type EngineFactory func(Config) (*core.Engine, error)

// #endregion Types

// #region Commands
func NewRoot(factory EngineFactory) *cobra.Command {
	root, _ := NewRootWithConfig(factory)
	return root
}

func NewRootWithConfig(factory EngineFactory) (*cobra.Command, *Config) {
	config := Config{Format: "compact"}
	root := &cobra.Command{
		Use:   "repo",
		Short: "Monorepo CLI for Semio",
	}
	root.PersistentFlags().StringVar(&config.Format, "format", "compact", "Output format: compact|jsonl|json")
	root.PersistentFlags().BoolVar(&config.Verbose, "verbose", false, "Verbose output")
	root.PersistentFlags().StringVar(&config.Repo, "repo", "", "Repo root path")
	root.PersistentFlags().DurationVar(&config.Timeout, "timeout", 0, "Timeout for command execution")
	root.AddCommand(mcpCommand(factory, &config))
	root.AddCommand(graphqlCommand(factory, &config))
	root.AddCommand(analyzeCommand(factory, &config))
	root.AddCommand(fixCommand(factory, &config))
	root.AddCommand(policyCommand(factory, &config))
	root.AddCommand(ticketCommand(factory, &config))
	root.AddCommand(contributorCommand(factory, &config))
	root.AddCommand(bundleCommand(factory, &config))
	root.AddCommand(folderCommand(factory, &config))
	root.AddCommand(fileCommand(factory, &config))
	root.AddCommand(sectionCommand(factory, &config))
	root.AddCommand(definitionCommand(factory, &config))
	return root, &config
}

func Execute(factory EngineFactory) error {
	return NewRoot(factory).Execute()
}

// #endregion Commands

// #region GraphQL
func graphqlCommand(factory EngineFactory, config *Config) *cobra.Command {
	var query string
	var variablesJSON string
	cmd := &cobra.Command{
		Use:   "graphql [query]",
		Short: "Execute a GraphQL query",
		Args:  cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			resolvedQuery := query
			if resolvedQuery == "" && len(args) > 0 {
				resolvedQuery = args[0]
			}
			if resolvedQuery == "" {
				return fmt.Errorf("missing query")
			}
			var variables map[string]interface{}
			if variablesJSON != "" {
				if err := json.Unmarshal([]byte(variablesJSON), &variables); err != nil {
					return fmt.Errorf("invalid variables JSON: %w", err)
				}
			}
			var payload struct {
				Query     string                 `json:"query"`
				Variables map[string]interface{} `json:"variables"`
			}
			if err := json.Unmarshal([]byte(resolvedQuery), &payload); err == nil && payload.Query != "" {
				resolvedQuery = payload.Query
				if variables == nil {
					variables = map[string]interface{}{}
				}
				for key, value := range payload.Variables {
					if _, exists := variables[key]; !exists {
						variables[key] = value
					}
				}
			}
			return runGraphQL(cmd, factory, config, resolvedQuery, variables)
		},
	}
	cmd.Flags().StringVar(&query, "query", "", "GraphQL query")
	cmd.Flags().StringVarP(&variablesJSON, "vars", "v", "", "GraphQL variables JSON")
	return cmd
}

func analyzeCommand(factory EngineFactory, config *Config) *cobra.Command {
	var scope string
	cmd := &cobra.Command{
		Use:   "analyze",
		Short: "Analyze codebase for policy violations",
		RunE: func(cmd *cobra.Command, args []string) error {
			variables := map[string]interface{}{}
			if scope != "" {
				variables["scope"] = scope
			}
			query := `query Analyze($scope: String) {
				analyze(scope: $scope) {
					violations {
						id
						summary
						scope
						line
						column
						excerpt
						kind { id priority autofixable reason solution }
					}
					metrics { total autofixable byPriority { high medium low } }
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	cmd.Flags().StringVar(&scope, "scope", "", "Scope to analyze")
	return cmd
}

func fixCommand(factory EngineFactory, config *Config) *cobra.Command {
	var scope string
	cmd := &cobra.Command{
		Use:   "fix",
		Short: "Apply autofixes for violations",
		RunE: func(cmd *cobra.Command, args []string) error {
			variables := map[string]interface{}{}
			if scope != "" {
				variables["scope"] = scope
			}
			query := `mutation Fix($scope: String) {
				fix(scope: $scope) {
					fixed
					remaining
					violations { id summary scope excerpt line }
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	cmd.Flags().StringVar(&scope, "scope", "", "Scope to fix")
	return cmd
}

func policyCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "policy", Short: "Policy management commands"}
	listCmd := &cobra.Command{
		Use:   "list",
		Short: "List all registered policies",
		RunE: func(cmd *cobra.Command, args []string) error {
			query := `query Policies {
				repo {
					policies {
						id
						name
						description
						scopes
						violationKinds { id priority autofixable reason solution }
					}
				}
			}`
			return runGraphQL(cmd, factory, config, query, nil)
		},
	}
	checkCmd := &cobra.Command{
		Use:   "check",
		Short: "Check a policy against a scope",
		RunE: func(cmd *cobra.Command, args []string) error {
			policyID, err := cmd.Flags().GetString("id")
			if err != nil {
				return err
			}
			if policyID == "" {
				return fmt.Errorf("missing policy id")
			}
			scope, err := cmd.Flags().GetString("scope")
			if err != nil {
				return err
			}
			variables := map[string]interface{}{"id": policyID}
			if scope != "" {
				variables["scope"] = scope
			}
			query := `query PolicyCheck($id: String!, $scope: String) {
				policy(id: $id) {
					id
					name
					description
					scopes
					violationKinds { id priority autofixable reason solution }
				}
				violations(scope: $scope) {
					id
					summary
					scope
					excerpt
					kind { id priority autofixable reason solution }
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	checkCmd.Flags().String("id", "", "Policy id")
	checkCmd.Flags().String("scope", "", "Scope to analyze")
	root.AddCommand(listCmd)
	root.AddCommand(checkCmd)
	return root
}

func ticketCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "ticket", Short: "Ticket management commands"}
	openCmd := &cobra.Command{
		Use:   "open",
		Short: "Open a new ticket",
		RunE: func(cmd *cobra.Command, args []string) error {
			title, _ := cmd.Flags().GetString("title")
			prompt, _ := cmd.Flags().GetString("prompt")
			llm, _ := cmd.Flags().GetString("llm")
			ui, _ := cmd.Flags().GetString("ui")
			noIssue, _ := cmd.Flags().GetBool("no-issue")
			if title == "" {
				return fmt.Errorf("missing title")
			}
			if prompt == "" {
				prompt = title
			}
			if llm == "" {
				return fmt.Errorf("missing llm")
			}
			if ui == "" {
				return fmt.Errorf("missing ui")
			}
			input := map[string]interface{}{
				"title":   title,
				"prompt":  prompt,
				"llm":     llm,
				"ui":      strings.ToUpper(strings.ReplaceAll(ui, "-", "_")),
				"noIssue": noIssue,
			}
			variables := map[string]interface{}{"input": input}
			query := `mutation TicketOpen($input: TicketOpenInput!) {
				ticketOpen(input: $input) {
					id
					slug
					status
					path
					uri
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	openCmd.Flags().String("title", "", "Ticket title")
	openCmd.Flags().String("prompt", "", "Ticket prompt")
	openCmd.Flags().String("llm", "", "LLM")
	openCmd.Flags().String("ui", "", "UI")
	openCmd.Flags().Bool("no-issue", false, "Skip GitHub issue")
	listCmd := &cobra.Command{
		Use:   "list",
		Short: "List tickets",
		RunE: func(cmd *cobra.Command, args []string) error {
			year, _ := cmd.Flags().GetInt("year")
			month, _ := cmd.Flags().GetInt("month")
			day, _ := cmd.Flags().GetInt("day")
			variables := map[string]interface{}{}
			if year != 0 {
				variables["year"] = year
			}
			if month != 0 {
				variables["month"] = month
			}
			if day != 0 {
				variables["day"] = day
			}
			query := `query Tickets($year: Int, $month: Int, $day: Int) {
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
					}
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	listCmd.Flags().Int("year", 0, "Filter by year")
	listCmd.Flags().Int("month", 0, "Filter by month")
	listCmd.Flags().Int("day", 0, "Filter by day")
	closeCmd := &cobra.Command{
		Use:   "close",
		Short: "Close a ticket",
		RunE: func(cmd *cobra.Command, args []string) error {
			year, _ := cmd.Flags().GetInt("year")
			month, _ := cmd.Flags().GetInt("month")
			day, _ := cmd.Flags().GetInt("day")
			slug, _ := cmd.Flags().GetString("slug")
			summary, _ := cmd.Flags().GetString("summary")
			files, _ := cmd.Flags().GetStringSlice("files")
			title, _ := cmd.Flags().GetString("title")
			if year == 0 || month == 0 || day == 0 || slug == "" {
				return fmt.Errorf("missing ticket path")
			}
			if summary == "" {
				return fmt.Errorf("missing summary")
			}
			if len(files) == 0 {
				return fmt.Errorf("missing files")
			}
			input := map[string]interface{}{
				"year":    year,
				"month":   month,
				"day":     day,
				"slug":    slug,
				"summary": summary,
				"files":   files,
			}
			if title != "" {
				input["title"] = title
			}
			variables := map[string]interface{}{"input": input}
			query := `mutation TicketClose($input: TicketCloseInput!) {
				ticketClose(input: $input) {
					id
					slug
					status
					date { created finished }
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	closeCmd.Flags().Int("year", 0, "Ticket year")
	closeCmd.Flags().Int("month", 0, "Ticket month")
	closeCmd.Flags().Int("day", 0, "Ticket day")
	closeCmd.Flags().String("slug", "", "Ticket slug")
	closeCmd.Flags().String("summary", "", "Summary")
	closeCmd.Flags().StringSlice("files", nil, "Files")
	closeCmd.Flags().String("title", "", "Title")
	reopenCmd := &cobra.Command{
		Use:   "reopen",
		Short: "Reopen a ticket",
		RunE: func(cmd *cobra.Command, args []string) error {
			year, _ := cmd.Flags().GetInt("year")
			month, _ := cmd.Flags().GetInt("month")
			day, _ := cmd.Flags().GetInt("day")
			slug, _ := cmd.Flags().GetString("slug")
			prompt, _ := cmd.Flags().GetString("prompt")
			llm, _ := cmd.Flags().GetString("llm")
			title, _ := cmd.Flags().GetString("title")
			if year == 0 || month == 0 || day == 0 || slug == "" {
				return fmt.Errorf("missing ticket path")
			}
			if prompt == "" || llm == "" {
				return fmt.Errorf("missing prompt or llm")
			}
			input := map[string]interface{}{
				"year":   year,
				"month":  month,
				"day":    day,
				"slug":   slug,
				"prompt": prompt,
				"llm":    llm,
			}
			if title != "" {
				input["title"] = title
			}
			variables := map[string]interface{}{"input": input}
			query := `mutation TicketReopen($input: TicketReopenInput!) {
				ticketReopen(input: $input) {
					id
					slug
					status
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	reopenCmd.Flags().Int("year", 0, "Ticket year")
	reopenCmd.Flags().Int("month", 0, "Ticket month")
	reopenCmd.Flags().Int("day", 0, "Ticket day")
	reopenCmd.Flags().String("slug", "", "Ticket slug")
	reopenCmd.Flags().String("prompt", "", "Prompt")
	reopenCmd.Flags().String("llm", "", "LLM")
	reopenCmd.Flags().String("title", "", "Title")
	root.AddCommand(openCmd)
	root.AddCommand(listCmd)
	root.AddCommand(closeCmd)
	root.AddCommand(reopenCmd)
	return root
}

func contributorCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "contributor", Short: "Contributor management commands"}
	listCmd := &cobra.Command{
		Use:   "list",
		Short: "List contributors",
		RunE: func(cmd *cobra.Command, args []string) error {
			query := `query Contributors {
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
			}`
			return runGraphQL(cmd, factory, config, query, nil)
		},
	}
	addCmd := &cobra.Command{
		Use:   "add",
		Short: "Add a contributor",
		RunE: func(cmd *cobra.Command, args []string) error {
			github, _ := cmd.Flags().GetString("github")
			name, _ := cmd.Flags().GetString("name")
			emails, _ := cmd.Flags().GetStringSlice("email")
			if github == "" {
				return fmt.Errorf("missing github")
			}
			input := map[string]interface{}{"github": github}
			if name != "" {
				input["name"] = name
			}
			if len(emails) > 0 {
				input["emails"] = emails
			}
			variables := map[string]interface{}{"input": input}
			query := `mutation ContributorAdd($input: ContributorAddInput!) {
				contributorAdd(input: $input) {
					id
					github
					name
					emails
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	addCmd.Flags().String("github", "", "GitHub username")
	addCmd.Flags().String("name", "", "Contributor name")
	addCmd.Flags().StringSlice("email", nil, "Contributor emails")
	removeCmd := &cobra.Command{
		Use:   "remove",
		Short: "Remove a contributor",
		RunE: func(cmd *cobra.Command, args []string) error {
			github, _ := cmd.Flags().GetString("github")
			if github == "" {
				return fmt.Errorf("missing github")
			}
			variables := map[string]interface{}{"github": github}
			query := `mutation ContributorRemove($github: String!) {
				contributorRemove(github: $github)
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	removeCmd.Flags().String("github", "", "GitHub username")
	root.AddCommand(listCmd)
	root.AddCommand(addCmd)
	root.AddCommand(removeCmd)
	return root
}

func bundleCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "bundle", Short: "Bundle management commands"}
	listCmd := &cobra.Command{
		Use:   "list",
		Short: "List bundles",
		RunE: func(cmd *cobra.Command, args []string) error {
			query := `query Bundles {
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
			}`
			return runGraphQL(cmd, factory, config, query, nil)
		},
	}
	root.AddCommand(listCmd)
	return root
}

func folderCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "folder", Short: "Folder management commands"}
	createCmd := &cobra.Command{
		Use:   "create",
		Short: "Create a folder",
		RunE: func(cmd *cobra.Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" {
				return fmt.Errorf("missing path")
			}
			variables := map[string]interface{}{"path": path}
			query := `mutation FolderCreate($path: String!) {
				folderCreate(path: $path) { id path name uri }
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	createCmd.Flags().String("path", "", "Folder path")
	moveCmd := &cobra.Command{
		Use:   "move",
		Short: "Move a folder",
		RunE: func(cmd *cobra.Command, args []string) error {
			src, _ := cmd.Flags().GetString("source")
			dst, _ := cmd.Flags().GetString("target")
			if src == "" || dst == "" {
				return fmt.Errorf("missing source or target")
			}
			variables := map[string]interface{}{"src": src, "dst": dst}
			query := `mutation FolderMove($src: String!, $dst: String!) {
				folderMove(src: $src, dst: $dst) { id path name uri }
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	moveCmd.Flags().String("source", "", "Source path")
	moveCmd.Flags().String("target", "", "Target path")
	deleteCmd := &cobra.Command{
		Use:   "delete",
		Short: "Delete a folder",
		RunE: func(cmd *cobra.Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" {
				return fmt.Errorf("missing path")
			}
			variables := map[string]interface{}{"path": path}
			query := `mutation FolderDelete($path: String!) { folderDelete(path: $path) }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	deleteCmd.Flags().String("path", "", "Folder path")
	root.AddCommand(createCmd)
	root.AddCommand(moveCmd)
	root.AddCommand(deleteCmd)
	return root
}

func fileCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "file", Short: "File management commands"}
	createCmd := &cobra.Command{
		Use:   "create",
		Short: "Create a file",
		RunE: func(cmd *cobra.Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" {
				return fmt.Errorf("missing path")
			}
			variables := map[string]interface{}{"path": path}
			query := `mutation FileCreate($path: String!) { fileCreate(path: $path) { id path name uri } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	createCmd.Flags().String("path", "", "File path")
	moveCmd := &cobra.Command{
		Use:   "move",
		Short: "Move a file",
		RunE: func(cmd *cobra.Command, args []string) error {
			src, _ := cmd.Flags().GetString("source")
			dst, _ := cmd.Flags().GetString("target")
			if src == "" || dst == "" {
				return fmt.Errorf("missing source or target")
			}
			variables := map[string]interface{}{"src": src, "dst": dst}
			query := `mutation FileMove($src: String!, $dst: String!) { fileMove(src: $src, dst: $dst) { id path name uri } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	moveCmd.Flags().String("source", "", "Source path")
	moveCmd.Flags().String("target", "", "Target path")
	deleteCmd := &cobra.Command{
		Use:   "delete",
		Short: "Delete a file",
		RunE: func(cmd *cobra.Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" {
				return fmt.Errorf("missing path")
			}
			variables := map[string]interface{}{"path": path}
			query := `mutation FileDelete($path: String!) { fileDelete(path: $path) }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	deleteCmd.Flags().String("path", "", "File path")
	root.AddCommand(createCmd)
	root.AddCommand(moveCmd)
	root.AddCommand(deleteCmd)
	return root
}

func sectionCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "section", Short: "Section management commands"}
	createCmd := &cobra.Command{
		Use:   "create",
		Short: "Create a section",
		RunE: func(cmd *cobra.Command, args []string) error {
			file, _ := cmd.Flags().GetString("file")
			name, _ := cmd.Flags().GetString("name")
			parent, _ := cmd.Flags().GetString("parent")
			if file == "" || name == "" {
				return fmt.Errorf("missing file or name")
			}
			variables := map[string]interface{}{"file": file, "name": name}
			if parent != "" {
				variables["parent"] = parent
			}
			query := `mutation SectionCreate($file: String!, $name: String!, $parent: String) {
				sectionCreate(file: $file, name: $name, parent: $parent) { id name range { start end } }
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	createCmd.Flags().String("file", "", "File path")
	createCmd.Flags().String("name", "", "Section name")
	createCmd.Flags().String("parent", "", "Parent section")
	moveCmd := &cobra.Command{
		Use:   "move",
		Short: "Move a section",
		RunE: func(cmd *cobra.Command, args []string) error {
			file, _ := cmd.Flags().GetString("file")
			oldName, _ := cmd.Flags().GetString("old")
			newName, _ := cmd.Flags().GetString("new")
			if file == "" || oldName == "" || newName == "" {
				return fmt.Errorf("missing file or names")
			}
			variables := map[string]interface{}{"file": file, "oldName": oldName, "newName": newName}
			query := `mutation SectionMove($file: String!, $oldName: String!, $newName: String!) {
				sectionMove(file: $file, oldName: $oldName, newName: $newName) { id name range { start end } }
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	moveCmd.Flags().String("file", "", "File path")
	moveCmd.Flags().String("old", "", "Old section name")
	moveCmd.Flags().String("new", "", "New section name")
	deleteCmd := &cobra.Command{
		Use:   "delete",
		Short: "Delete a section",
		RunE: func(cmd *cobra.Command, args []string) error {
			file, _ := cmd.Flags().GetString("file")
			name, _ := cmd.Flags().GetString("name")
			if file == "" || name == "" {
				return fmt.Errorf("missing file or name")
			}
			variables := map[string]interface{}{"file": file, "name": name}
			query := `mutation SectionDelete($file: String!, $name: String!) { sectionDelete(file: $file, name: $name) }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	deleteCmd.Flags().String("file", "", "File path")
	deleteCmd.Flags().String("name", "", "Section name")
	listCmd := &cobra.Command{
		Use:   "list",
		Short: "List sections",
		RunE: func(cmd *cobra.Command, args []string) error {
			file, _ := cmd.Flags().GetString("file")
			if file == "" {
				return fmt.Errorf("missing file")
			}
			variables := map[string]interface{}{"path": file}
			query := `query SectionList($path: String!) {
				file(path: $path) {
					sections {
						id
						name
						range { start end }
						children {
							id
							name
							range { start end }
							children {
								id
								name
								range { start end }
								children {
									id
									name
									range { start end }
								}
							}
						}
					}
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	listCmd.Flags().String("file", "", "File path")
	integrateCmd := &cobra.Command{
		Use:   "integrate",
		Short: "Integrate source into target section",
		RunE: func(cmd *cobra.Command, args []string) error {
			source, _ := cmd.Flags().GetString("source")
			targetSection, _ := cmd.Flags().GetString("target-section")
			targetFile, _ := cmd.Flags().GetString("target-file")
			targetParent, _ := cmd.Flags().GetString("target-parent")
			if source == "" || targetSection == "" || targetFile == "" {
				return fmt.Errorf("missing source or target")
			}
			variables := map[string]interface{}{
				"source":        source,
				"targetSection": targetSection,
				"targetFile":    targetFile,
			}
			if targetParent != "" {
				variables["targetParent"] = targetParent
			}
			query := `mutation Integrate($source: String!, $targetSection: String!, $targetFile: String!, $targetParent: String) {
				integrate(source: $source, targetSection: $targetSection, targetFile: $targetFile, targetParent: $targetParent) { id path }
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	integrateCmd.Flags().String("source", "", "Source path")
	integrateCmd.Flags().String("target-section", "", "Target section name")
	integrateCmd.Flags().String("target-file", "", "Target file path")
	integrateCmd.Flags().String("target-parent", "", "Target parent section name")
	root.AddCommand(createCmd)
	root.AddCommand(moveCmd)
	root.AddCommand(deleteCmd)
	root.AddCommand(listCmd)
	root.AddCommand(integrateCmd)
	return root
}

func definitionCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "definition", Short: "Definition management commands"}
	listCmd := &cobra.Command{
		Use:   "list",
		Short: "List definitions",
		RunE: func(cmd *cobra.Command, args []string) error {
			path, _ := cmd.Flags().GetString("file")
			if path == "" {
				return fmt.Errorf("missing file")
			}
			variables := map[string]interface{}{"path": path}
			query := `query DefinitionList($path: String!) {
				file(path: $path) {
					definitions { id name kind range { start end } }
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	listCmd.Flags().String("file", "", "File path")
	root.AddCommand(listCmd)
	return root
}

// #endregion GraphQL

// #region Rendering
func renderStream(cmd *cobra.Command, config *Config, stream <-chan events.Event) error {
	switch config.Format {
	case "jsonl":
		exitCode, err := RenderJSONL(cmd.OutOrStdout(), stream)
		if err != nil {
			return err
		}
		if exitCode != 0 {
			return ExitError{Code: exitCode}
		}
		return nil
	case "json":
		exitCode, err := RenderJSON(cmd.OutOrStdout(), stream)
		if err != nil {
			return err
		}
		if exitCode != 0 {
			return ExitError{Code: exitCode}
		}
		return nil
	default:
		exitCode, err := RenderCompact(cmd.OutOrStdout(), cmd.ErrOrStderr(), stream, config.Verbose)
		if err != nil {
			return err
		}
		if exitCode != 0 {
			return ExitError{Code: exitCode}
		}
		return nil
	}
}

func runGraphQL(cmd *cobra.Command, factory EngineFactory, config *Config, query string, variables map[string]interface{}) error {
	argsPayload := core.GraphQLArgs{Query: query, Variables: variables}
	payloadBytes, err := json.Marshal(argsPayload)
	if err != nil {
		return err
	}
	engine, err := factory(*config)
	if err != nil {
		return err
	}
	ctx := context.Background()
	if config.Timeout > 0 {
		ctxWithTimeout, cancel := context.WithTimeout(ctx, config.Timeout)
		defer cancel()
		ctx = ctxWithTimeout
	}
	request := core.Request{Command: core.CmdGraphQL, Args: payloadBytes, RepoRoot: config.Repo, Verbose: config.Verbose}
	stream := engine.Run(ctx, request)
	return renderStream(cmd, config, stream)
}

// #endregion Rendering
