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
	"encoding/json"
	"fmt"
	"os"
	"strconv"

	"github.com/spf13/cobra"
	"github.com/usalu/semio/go/repo"
)

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
	Long:  `repo - Monorepo CLI for Semio. All commands output JSON for programmatic use.`,
}

func init() {
	rootCmd.AddCommand(codebaseCmd)
	rootCmd.AddCommand(analyzeCmd)
	rootCmd.AddCommand(fixCmd)
	rootCmd.AddCommand(policyCmd)
	rootCmd.AddCommand(ticketCmd)
	rootCmd.AddCommand(contributorCmd)
	rootCmd.AddCommand(projectCmd)
	rootCmd.AddCommand(folderCmd)
	rootCmd.AddCommand(fileCmd)
	rootCmd.AddCommand(sectionCmd)
	rootCmd.AddCommand(definitionCmd)
	rootCmd.AddCommand(updateMetabolismCmd)
	rootCmd.AddCommand(graphqlCmd)
}

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
		result, err := repo.ExecuteGraphQL(query, variables)
		if err != nil {
			return err
		}
		fmt.Println(result)
		return nil
	},
}

func init() {
	graphqlCmd.Flags().StringP("variables", "v", "", "JSON object with query variables")
}

var codebaseCmd = &cobra.Command{
	Use:   "codebase",
	Short: "Get comprehensive codebase structure",
	Long:  `Returns a complete JSON structure with bundles, folders, files, sections, definitions, contributors, tickets, policies, violations, and tree.`,
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolCodebase()
		return repo.OutputResult(result)
	},
}

var analyzeCmd = &cobra.Command{
	Use:   "analyze [scope...]",
	Short: "Analyze codebase for violations",
	RunE: func(cmd *cobra.Command, args []string) error {
		scope := "@semio"
		if len(args) > 0 {
			scope = args[0]
		}
		result := repo.ToolAnalyze(scope, args)
		return repo.OutputResult(result)
	},
}

var fixCmd = &cobra.Command{
	Use:   "fix [scope]",
	Short: "Apply autofixes for violations",
	RunE: func(cmd *cobra.Command, args []string) error {
		scope := "@semio"
		if len(args) > 0 {
			scope = args[0]
		}
		result := repo.ToolFix(scope)
		return repo.OutputResult(result)
	},
}

var policyCmd = &cobra.Command{
	Use:   "policy",
	Short: "Policy management commands",
}

var policyListCmd = &cobra.Command{
	Use:   "list",
	Short: "List all registered policies",
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolPolicyList()
		return repo.OutputResult(result)
	},
}

var policyCheckCmd = &cobra.Command{
	Use:   "check <id> [scope]",
	Short: "Check a specific policy",
	Args:  cobra.RangeArgs(1, 2),
	RunE: func(cmd *cobra.Command, args []string) error {
		scope := "@semio"
		if len(args) > 1 {
			scope = args[1]
		}
		result := repo.ToolPolicyCheck(args[0], scope)
		return repo.OutputResult(result)
	},
}

var policyViolationCmd = &cobra.Command{
	Use:   "violation",
	Short: "Policy violation commands",
}

var policyViolationListCmd = &cobra.Command{
	Use:   "list <policyId>",
	Short: "List violation kinds for a policy",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolPolicyViolationList(args[0])
		return repo.OutputResult(result)
	},
}

func init() {
	policyCmd.AddCommand(policyListCmd)
	policyCmd.AddCommand(policyCheckCmd)
	policyCmd.AddCommand(policyViolationCmd)
	policyViolationCmd.AddCommand(policyViolationListCmd)
}

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
		result := repo.ToolTicketCreate(args[0], prompt, model, files)
		return repo.OutputResult(result)
	},
}

var ticketListCmd = &cobra.Command{
	Use:   "list [year] [month] [day]",
	Short: "List tickets",
	RunE: func(cmd *cobra.Command, args []string) error {
		var year, month, day *int
		if len(args) > 0 {
			y, _ := strconv.Atoi(args[0])
			year = &y
		}
		if len(args) > 1 {
			m, _ := strconv.Atoi(args[1])
			month = &m
		}
		if len(args) > 2 {
			d, _ := strconv.Atoi(args[2])
			day = &d
		}
		result := repo.ToolTicketList(year, month, day)
		return repo.OutputResult(result)
	},
}

var ticketReadCmd = &cobra.Command{
	Use:   "read <year> <month> <day> <slug>",
	Short: "Read a ticket",
	Args:  cobra.ExactArgs(4),
	RunE: func(cmd *cobra.Command, args []string) error {
		year, _ := strconv.Atoi(args[0])
		month, _ := strconv.Atoi(args[1])
		day, _ := strconv.Atoi(args[2])
		result := repo.ToolTicketRead(year, month, day, args[3])
		return repo.OutputResult(result)
	},
}

var ticketIterateCmd = &cobra.Command{
	Use:   "iterate",
	Short: "Ticket iteration commands",
}

var ticketIterateStartCmd = &cobra.Command{
	Use:   "start <year> <month> <day> <slug>",
	Short: "Start a ticket iteration (deprecated, use progress)",
	Args:  cobra.ExactArgs(4),
	RunE: func(cmd *cobra.Command, args []string) error {
		prompt, _ := cmd.Flags().GetString("prompt")
		model, _ := cmd.Flags().GetString("model")
		year, _ := strconv.Atoi(args[0])
		month, _ := strconv.Atoi(args[1])
		day, _ := strconv.Atoi(args[2])
		result := repo.ToolTicketProgress(year, month, day, args[3], prompt, model)
		return repo.OutputResult(result)
	},
}

var ticketProgressCmd = &cobra.Command{
	Use:   "progress <year> <month> <day> <slug>",
	Short: "Record progress on a ticket (creates iteration from git changes)",
	Args:  cobra.ExactArgs(4),
	RunE: func(cmd *cobra.Command, args []string) error {
		prompt, _ := cmd.Flags().GetString("prompt")
		model, _ := cmd.Flags().GetString("model")
		year, _ := strconv.Atoi(args[0])
		month, _ := strconv.Atoi(args[1])
		day, _ := strconv.Atoi(args[2])
		result := repo.ToolTicketProgress(year, month, day, args[3], prompt, model)
		return repo.OutputResult(result)
	},
}

var ticketFinishCmd = &cobra.Command{
	Use:   "finish <year> <month> <day> <slug>",
	Short: "Finish a ticket",
	Args:  cobra.ExactArgs(4),
	RunE: func(cmd *cobra.Command, args []string) error {
		year, _ := strconv.Atoi(args[0])
		month, _ := strconv.Atoi(args[1])
		day, _ := strconv.Atoi(args[2])
		result := repo.ToolTicketFinish(year, month, day, args[3])
		return repo.OutputResult(result)
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
		result := repo.ToolTicketReopen(year, month, day, args[3])
		return repo.OutputResult(result)
	},
}

func init() {
	ticketCreateCmd.Flags().String("prompt", "", "Ticket prompt")
	ticketCreateCmd.Flags().String("model", "", "Model used")
	ticketCreateCmd.Flags().StringSlice("file", nil, "Files to include (can be specified multiple times)")
	ticketIterateStartCmd.Flags().String("prompt", "", "Iteration prompt")
	ticketIterateStartCmd.Flags().String("model", "", "Model used")
	ticketProgressCmd.Flags().String("prompt", "", "Iteration prompt")
	ticketProgressCmd.Flags().String("model", "", "Model used")
	ticketIterateCmd.AddCommand(ticketIterateStartCmd)
	ticketCmd.AddCommand(ticketCreateCmd)
	ticketCmd.AddCommand(ticketListCmd)
	ticketCmd.AddCommand(ticketReadCmd)
	ticketCmd.AddCommand(ticketIterateCmd)
	ticketCmd.AddCommand(ticketProgressCmd)
	ticketCmd.AddCommand(ticketFinishCmd)
	ticketCmd.AddCommand(ticketReopenCmd)
}

var contributorCmd = &cobra.Command{
	Use:   "contributor",
	Short: "Contributor management commands",
}

var contributorAddCmd = &cobra.Command{
	Use:   "add <github>",
	Short: "Add a contributor",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolContributorAdd(args[0])
		return repo.OutputResult(result)
	},
}

var contributorListCmd = &cobra.Command{
	Use:   "list",
	Short: "List contributors",
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolContributorList()
		return repo.OutputResult(result)
	},
}

var contributorRemoveCmd = &cobra.Command{
	Use:   "remove <github>",
	Short: "Remove a contributor",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolContributorRemove(args[0])
		return repo.OutputResult(result)
	},
}

func init() {
	contributorCmd.AddCommand(contributorAddCmd)
	contributorCmd.AddCommand(contributorListCmd)
	contributorCmd.AddCommand(contributorRemoveCmd)
}

var projectCmd = &cobra.Command{
	Use:   "bundle",
	Short: "Bundle management commands",
}

var projectListCmd = &cobra.Command{
	Use:   "list",
	Short: "List Nx bundles",
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolProjectList()
		return repo.OutputResult(result)
	},
}

var projectTreeCmd = &cobra.Command{
	Use:   "tree",
	Short: "Show bundle dependency tree",
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolProjectTree()
		return repo.OutputResult(result)
	},
}

func init() {
	projectCmd.AddCommand(projectListCmd)
	projectCmd.AddCommand(projectTreeCmd)
}

var folderCmd = &cobra.Command{
	Use:   "folder",
	Short: "Folder management commands",
}

var folderCreateCmd = &cobra.Command{
	Use:   "create <path>",
	Short: "Create a folder",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolFolderCreate(args[0])
		return repo.OutputResult(result)
	},
}

var folderMoveCmd = &cobra.Command{
	Use:   "move <source> <target>",
	Short: "Move a folder",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolFolderMove(args[0], args[1])
		return repo.OutputResult(result)
	},
}

var folderDeleteCmd = &cobra.Command{
	Use:   "delete <path>",
	Short: "Delete a folder",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolFolderDelete(args[0])
		return repo.OutputResult(result)
	},
}

var folderListCmd = &cobra.Command{
	Use:   "list [path]",
	Short: "List folders",
	RunE: func(cmd *cobra.Command, args []string) error {
		path := "."
		if len(args) > 0 {
			path = args[0]
		}
		result := repo.ToolFolderList(path)
		return repo.OutputResult(result)
	},
}

var folderTreeCmd = &cobra.Command{
	Use:   "tree [path]",
	Short: "Show folder tree",
	RunE: func(cmd *cobra.Command, args []string) error {
		path := "."
		if len(args) > 0 {
			path = args[0]
		}
		result := repo.ToolFolderTree(path)
		return repo.OutputResult(result)
	},
}

func init() {
	folderCmd.AddCommand(folderCreateCmd)
	folderCmd.AddCommand(folderMoveCmd)
	folderCmd.AddCommand(folderDeleteCmd)
	folderCmd.AddCommand(folderListCmd)
	folderCmd.AddCommand(folderTreeCmd)
}

var fileCmd = &cobra.Command{
	Use:   "file",
	Short: "File management commands",
}

var fileCreateCmd = &cobra.Command{
	Use:   "create <path>",
	Short: "Create a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolFileCreate(args[0])
		return repo.OutputResult(result)
	},
}

var fileMoveCmd = &cobra.Command{
	Use:   "move <source> <target>",
	Short: "Move a file",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolFileMove(args[0], args[1])
		return repo.OutputResult(result)
	},
}

var fileDeleteCmd = &cobra.Command{
	Use:   "delete <path>",
	Short: "Delete a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolFileDelete(args[0])
		return repo.OutputResult(result)
	},
}

var fileListCmd = &cobra.Command{
	Use:   "list [scope]",
	Short: "List files in scope",
	RunE: func(cmd *cobra.Command, args []string) error {
		scope := "@semio"
		if len(args) > 0 {
			scope = args[0]
		}
		result := repo.ToolFileList(scope)
		return repo.OutputResult(result)
	},
}

var fileTreeCmd = &cobra.Command{
	Use:   "tree [path]",
	Short: "Show file tree",
	RunE: func(cmd *cobra.Command, args []string) error {
		path := "."
		if len(args) > 0 {
			path = args[0]
		}
		result := repo.ToolFileTree(path)
		return repo.OutputResult(result)
	},
}

func init() {
	fileCmd.AddCommand(fileCreateCmd)
	fileCmd.AddCommand(fileMoveCmd)
	fileCmd.AddCommand(fileDeleteCmd)
	fileCmd.AddCommand(fileListCmd)
	fileCmd.AddCommand(fileTreeCmd)
}

var sectionCmd = &cobra.Command{
	Use:   "section",
	Short: "Section management commands",
}

var sectionCreateCmd = &cobra.Command{
	Use:   "create <file> <section-path>",
	Short: "Create a section in a file",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolSectionCreate(args[0], args[1])
		return repo.OutputResult(result)
	},
}

var sectionMoveCmd = &cobra.Command{
	Use:   "move <file> <old-section> <new-section>",
	Short: "Move/rename a section",
	Args:  cobra.ExactArgs(3),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolSectionMove(args[0], args[1], args[2])
		return repo.OutputResult(result)
	},
}

var sectionDeleteCmd = &cobra.Command{
	Use:   "delete <file> <section-path>",
	Short: "Delete a section from a file",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolSectionDelete(args[0], args[1])
		return repo.OutputResult(result)
	},
}

var sectionListCmd = &cobra.Command{
	Use:   "list <file>",
	Short: "List sections in a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolSectionList(args[0])
		return repo.OutputResult(result)
	},
}

var sectionTreeCmd = &cobra.Command{
	Use:   "tree <file>",
	Short: "Show section tree",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolSectionTree(args[0])
		return repo.OutputResult(result)
	},
}

func init() {
	sectionCmd.AddCommand(sectionCreateCmd)
	sectionCmd.AddCommand(sectionMoveCmd)
	sectionCmd.AddCommand(sectionDeleteCmd)
	sectionCmd.AddCommand(sectionListCmd)
	sectionCmd.AddCommand(sectionTreeCmd)
}

var definitionCmd = &cobra.Command{
	Use:   "definition",
	Short: "Definition management commands",
}

var definitionListCmd = &cobra.Command{
	Use:   "list <file>",
	Short: "List definitions in a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolDefinitionList(args[0])
		return repo.OutputResult(result)
	},
}

var definitionTreeCmd = &cobra.Command{
	Use:   "tree <file>",
	Short: "Show definition tree",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolDefinitionTree(args[0])
		return repo.OutputResult(result)
	},
}

func init() {
	definitionCmd.AddCommand(definitionListCmd)
	definitionCmd.AddCommand(definitionTreeCmd)
}

var updateMetabolismCmd = &cobra.Command{
	Use:   "update-metabolism",
	Short: "Update metabolism assets (exports kit to zip and copies to public folders)",
	RunE: func(cmd *cobra.Command, args []string) error {
		result := repo.ToolUpdateMetabolism()
		return repo.OutputResult(result)
	},
}

// #endregion Commands