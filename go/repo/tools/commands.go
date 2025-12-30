// repo/tools/commands.go

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

package tools

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"

	"github.com/spf13/cobra"
)

var (
	jsonOutput bool
	scopeFlag  string
	dryRun     bool
)

func Execute() error {
	return rootCmd.Execute()
}

var rootCmd = &cobra.Command{
	Use:   "repo",
	Short: "Monorepo CLI for Semio",
	Long:  `repo - Monorepo CLI for Semio. Exposes tools for analyzing, fixing, and managing the codebase.`,
}

func init() {
	rootCmd.PersistentFlags().BoolVar(&jsonOutput, "json", false, "Output as JSON")
	rootCmd.PersistentFlags().StringVar(&scopeFlag, "scope", "", "Limit operation to scope")
	rootCmd.PersistentFlags().BoolVar(&dryRun, "dry-run", false, "Preview without making changes")
	rootCmd.AddCommand(analyzeCmd)
	rootCmd.AddCommand(fixCmd)
	rootCmd.AddCommand(policyCmd)
	rootCmd.AddCommand(ticketCmd)
	rootCmd.AddCommand(projectCmd)
	rootCmd.AddCommand(folderCmd)
	rootCmd.AddCommand(fileCmd)
	rootCmd.AddCommand(sectionCmd)
	rootCmd.AddCommand(definitionCmd)
	rootCmd.AddCommand(toolCmd)
}

var analyzeCmd = &cobra.Command{
	Use:   "analyze [scope...]",
	Short: "Analyze codebase for violations",
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolAnalyze(scopeFlag, args)
		return outputResult(result)
	},
}

var fixCmd = &cobra.Command{
	Use:   "fix [scope]",
	Short: "Apply autofixes for violations",
	RunE: func(cmd *cobra.Command, args []string) error {
		scope := scopeFlag
		if len(args) > 0 {
			scope = args[0]
		}
		result := ToolFix(scope, dryRun)
		return outputResult(result)
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
		result := ToolPolicyList()
		return outputResult(result)
	},
}

var policyRunCmd = &cobra.Command{
	Use:   "run <id>",
	Short: "Run a specific policy",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolPolicyRun(args[0], scopeFlag)
		return outputResult(result)
	},
}

func init() {
	policyCmd.AddCommand(policyListCmd)
	policyCmd.AddCommand(policyRunCmd)
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
		result := ToolTicketCreate(args[0], prompt, model)
		return outputResult(result)
	},
}

var ticketListCmd = &cobra.Command{
	Use:   "list",
	Short: "List tickets",
	RunE: func(cmd *cobra.Command, args []string) error {
		yearStr, _ := cmd.Flags().GetString("year")
		monthStr, _ := cmd.Flags().GetString("month")
		dayStr, _ := cmd.Flags().GetString("day")
		var year, month, day *int
		if yearStr != "" {
			y, _ := strconv.Atoi(yearStr)
			year = &y
		}
		if monthStr != "" {
			m, _ := strconv.Atoi(monthStr)
			month = &m
		}
		if dayStr != "" {
			d, _ := strconv.Atoi(dayStr)
			day = &d
		}
		result := ToolTicketList(year, month, day)
		return outputResult(result)
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
		result := ToolTicketRead(year, month, day, args[3])
		return outputResult(result)
	},
}

var ticketIterateCmd = &cobra.Command{
	Use:   "iterate",
	Short: "Ticket iteration commands",
}

var ticketIterateStartCmd = &cobra.Command{
	Use:   "start <year> <month> <day> <slug>",
	Short: "Start a ticket iteration",
	Args:  cobra.ExactArgs(4),
	RunE: func(cmd *cobra.Command, args []string) error {
		prompt, _ := cmd.Flags().GetString("prompt")
		model, _ := cmd.Flags().GetString("model")
		year, _ := strconv.Atoi(args[0])
		month, _ := strconv.Atoi(args[1])
		day, _ := strconv.Atoi(args[2])
		result := ToolTicketIterateStart(year, month, day, args[3], prompt, model)
		return outputResult(result)
	},
}

var ticketIterateEndCmd = &cobra.Command{
	Use:   "end <year> <month> <day> <slug>",
	Short: "End a ticket iteration",
	Args:  cobra.ExactArgs(4),
	RunE: func(cmd *cobra.Command, args []string) error {
		year, _ := strconv.Atoi(args[0])
		month, _ := strconv.Atoi(args[1])
		day, _ := strconv.Atoi(args[2])
		result := ToolTicketIterateEnd(year, month, day, args[3])
		return outputResult(result)
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
		result := ToolTicketFinish(year, month, day, args[3])
		return outputResult(result)
	},
}

func init() {
	ticketCreateCmd.Flags().String("prompt", "", "Ticket prompt")
	ticketCreateCmd.Flags().String("model", "", "Model used")
	ticketListCmd.Flags().String("year", "", "Filter by year")
	ticketListCmd.Flags().String("month", "", "Filter by month")
	ticketListCmd.Flags().String("day", "", "Filter by day")
	ticketIterateStartCmd.Flags().String("prompt", "", "Iteration prompt")
	ticketIterateStartCmd.Flags().String("model", "", "Model used")
	ticketIterateCmd.AddCommand(ticketIterateStartCmd)
	ticketIterateCmd.AddCommand(ticketIterateEndCmd)
	ticketCmd.AddCommand(ticketCreateCmd)
	ticketCmd.AddCommand(ticketListCmd)
	ticketCmd.AddCommand(ticketReadCmd)
	ticketCmd.AddCommand(ticketIterateCmd)
	ticketCmd.AddCommand(ticketFinishCmd)
}

var projectCmd = &cobra.Command{
	Use:   "project",
	Short: "Project management commands",
}

var projectListCmd = &cobra.Command{
	Use:   "list",
	Short: "List Nx projects",
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolProjectList()
		return outputResult(result)
	},
}

var projectTreeCmd = &cobra.Command{
	Use:   "tree",
	Short: "Show project dependency tree",
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolProjectTree()
		return outputResult(result)
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
		result := ToolFolderCreate(args[0])
		return outputResult(result)
	},
}

var folderMoveCmd = &cobra.Command{
	Use:   "move <source> <target>",
	Short: "Move a folder",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolFolderMove(args[0], args[1])
		return outputResult(result)
	},
}

var folderDeleteCmd = &cobra.Command{
	Use:   "delete <path>",
	Short: "Delete a folder",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolFolderDelete(args[0])
		return outputResult(result)
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
		if scopeFlag != "" {
			path = scopeFlag
		}
		result := ToolFolderList(path)
		return outputResult(result)
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
		if scopeFlag != "" {
			path = scopeFlag
		}
		result := ToolFolderTree(path)
		return outputResult(result)
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
		result := ToolFileCreate(args[0])
		return outputResult(result)
	},
}

var fileMoveCmd = &cobra.Command{
	Use:   "move <source> <target>",
	Short: "Move a file",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolFileMove(args[0], args[1])
		return outputResult(result)
	},
}

var fileDeleteCmd = &cobra.Command{
	Use:   "delete <path>",
	Short: "Delete a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolFileDelete(args[0])
		return outputResult(result)
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
		if scopeFlag != "" {
			scope = scopeFlag
		}
		result := ToolFileList(scope)
		return outputResult(result)
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
		if scopeFlag != "" {
			path = scopeFlag
		}
		result := ToolFileTree(path)
		return outputResult(result)
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
		result := ToolSectionCreate(args[0], args[1])
		return outputResult(result)
	},
}

var sectionMoveCmd = &cobra.Command{
	Use:   "move <file> <old-section> <new-section>",
	Short: "Move/rename a section",
	Args:  cobra.ExactArgs(3),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolSectionMove(args[0], args[1], args[2])
		return outputResult(result)
	},
}

var sectionDeleteCmd = &cobra.Command{
	Use:   "delete <file> <section-path>",
	Short: "Delete a section from a file",
	Args:  cobra.ExactArgs(2),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolSectionDelete(args[0], args[1])
		return outputResult(result)
	},
}

var sectionListCmd = &cobra.Command{
	Use:   "list <file>",
	Short: "List sections in a file",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolSectionList(args[0])
		return outputResult(result)
	},
}

var sectionTreeCmd = &cobra.Command{
	Use:   "tree <file>",
	Short: "Show section tree",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolSectionTree(args[0])
		return outputResult(result)
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
		result := ToolDefinitionList(args[0])
		return outputResult(result)
	},
}

var definitionTreeCmd = &cobra.Command{
	Use:   "tree <file>",
	Short: "Show definition tree",
	Args:  cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolDefinitionTree(args[0])
		return outputResult(result)
	},
}

func init() {
	definitionCmd.AddCommand(definitionListCmd)
	definitionCmd.AddCommand(definitionTreeCmd)
}

var toolCmd = &cobra.Command{
	Use:   "tool <name> [args...]",
	Short: "Run a tool (e.g., i18n, update-metabolism)",
	Args:  cobra.MinimumNArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		result := ToolRunTool(args[0], args[1:])
		return outputResult(result)
	},
}

func outputResult(result ToolResult) error {
	if jsonOutput {
		enc := json.NewEncoder(os.Stdout)
		enc.SetIndent("", "  ")
		return enc.Encode(result)
	}
	result.Output.Print()
	if result.Output.ExitCode != 0 {
		os.Exit(result.Output.ExitCode)
	}
	return nil
}

func ToolAnalyze(scope string, scopes []string) ToolResult {
	output := NewOutput()
	var scopeRaws []string
	if scope != "" {
		scopeRaws = []string{scope}
	} else if len(scopes) > 0 {
		scopeRaws = scopes
	} else {
		scopeRaws = []string{"@semio"}
	}
	var allViolations []Violation
	projects := GetNxProjects()
	for _, scopeRaw := range scopeRaws {
		s := ParseScope(scopeRaw)
		violations, err := RunPolicies(s, projects, nil)
		if err != nil {
			output.Error(fmt.Sprintf("Error running policies: %v", err))
			return ToolResult{Output: *output, Error: err.Error()}
		}
		allViolations = append(allViolations, violations...)
	}
	report := AnalyzeReport{
		Timestamp: ISOTimestamp(),
		Status:    "success",
		Scope:     strings.Join(scopeRaws, " "),
		Summary: Summary{
			Total:      len(allViolations),
			ByPriority: make(map[string]int),
			ByKind:     make(map[string]int),
		},
		Violations: allViolations,
	}
	if len(allViolations) > 0 {
		report.Status = "error"
	}
	for _, v := range allViolations {
		report.Summary.ByPriority[string(v.Priority)]++
		report.Summary.ByKind[v.Kind]++
	}
	reportsDir := filepath.Join(rootDir, "reports")
	if err := EnsureDir(reportsDir); err == nil {
		WriteJSONFile(filepath.Join(reportsDir, "policies.json"), report)
	}
	output.Success(fmt.Sprintf("\n📊 Analysis complete: %d violations found", len(allViolations)))
	output.Info(fmt.Sprintf("   Report: %s", filepath.Join(reportsDir, "policies.json")))
	if report.Status == "error" {
		output.ExitCode = 1
	}
	return ToolResult{Output: *output, Data: report}
}

func ToolFix(scopeRaw string, dryRun bool) ToolResult {
	output := NewOutput()
	if scopeRaw == "" {
		scopeRaw = "@semio"
	}
	scope := ParseScope(scopeRaw)
	projects := GetNxProjects()
	violations, err := RunPolicies(scope, projects, nil)
	if err != nil {
		output.Error(fmt.Sprintf("Error running policies: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	var fixable []Violation
	for _, v := range violations {
		if v.Autofixable && v.Autofix != nil {
			fixable = append(fixable, v)
		}
	}
	if dryRun {
		output.Info(fmt.Sprintf("\n🔧 Dry run: %d fixable violations found", len(fixable)))
		for _, v := range fixable {
			output.Plain(fmt.Sprintf("   - %s: %s", v.Kind, v.Summary))
		}
	} else {
		fixed := 0
		for _, v := range fixable {
			if v.Autofix != nil {
				for filePath, edits := range v.Autofix.Edits {
					absPath := filepath.Join(rootDir, filePath)
					content, err := ReadTextFile(absPath)
					if err != nil {
						continue
					}
					for i := len(edits) - 1; i >= 0; i-- {
						edit := edits[i]
						content = content[:edit.Start] + edit.NewText + content[edit.End:]
					}
					WriteTextFile(absPath, content)
				}
				fixed++
			}
		}
		output.Success(fmt.Sprintf("\n✅ Fixed %d violations", fixed))
	}
	return ToolResult{Output: *output}
}

func ToolPolicyList() ToolResult {
	output := NewOutput()
	policies := GetRegisteredPolicies()
	output.Info("\n📜 Registered policies:\n")
	for _, p := range policies {
		output.Plain(fmt.Sprintf("   %s", p.ID))
		output.Plain(fmt.Sprintf("      %s: %s", p.Name, p.Description))
		output.Plain(fmt.Sprintf("      Priority: %s", p.Priority))
		output.Plain("")
	}
	return ToolResult{Output: *output, Data: policies}
}

func ToolPolicyRun(policyID, scopeRaw string) ToolResult {
	output := NewOutput()
	if scopeRaw == "" {
		scopeRaw = "@semio"
	}
	scope := ParseScope(scopeRaw)
	projects := GetNxProjects()
	violations, err := RunPolicies(scope, projects, []string{policyID})
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Info(fmt.Sprintf("\n📊 Policy \"%s\" found %d violations", policyID, len(violations)))
	for i, v := range violations {
		if i >= 10 {
			output.Plain(fmt.Sprintf("   ... and %d more", len(violations)-10))
			break
		}
		output.Plain(fmt.Sprintf("   - %s", v.Summary))
	}
	return ToolResult{Output: *output, Data: violations}
}

func ToolTicketCreate(slug, prompt, model string) ToolResult {
	output := NewOutput()
	if prompt == "" {
		prompt = slug
	}
	ticket, err := CreateTicket(slug, prompt, model)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🎫 Created ticket: %s", ticket.Slug))
	output.Info(fmt.Sprintf("   Path: %s", ticket.FilePath))
	return ToolResult{Output: *output, Data: ticket}
}

func ToolTicketList(year, month, day *int) ToolResult {
	output := NewOutput()
	tickets, err := ListTickets(year, month, day)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Info(fmt.Sprintf("\n🎫 Found %d tickets:\n", len(tickets)))
	for _, t := range tickets {
		status := "🟢"
		if t.Frontmatter.Status == TicketClosed {
			status = "✅"
		}
		output.Plain(fmt.Sprintf("   %s %d/%s/%s/%s", status, t.Year, PadNumber(t.Month, 2), PadNumber(t.Day, 2), t.Slug))
		if t.Frontmatter.Summary != "" {
			output.Plain(fmt.Sprintf("      %s", t.Frontmatter.Summary))
		}
	}
	return ToolResult{Output: *output, Data: tickets}
}

func ToolTicketRead(year, month, day int, slug string) ToolResult {
	output := NewOutput()
	ticket, err := ReadTicket(year, month, day, slug)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Info(fmt.Sprintf("\n🎫 Ticket: %s", ticket.Slug))
	output.Plain(fmt.Sprintf("   Status: %s", ticket.Frontmatter.Status))
	output.Plain(fmt.Sprintf("   Created: %s", ticket.Frontmatter.Date.Created))
	output.Plain(fmt.Sprintf("   Prompt: %s", ticket.Frontmatter.Prompt))
	if ticket.Frontmatter.Model != "" {
		output.Plain(fmt.Sprintf("   Model: %s", ticket.Frontmatter.Model))
	}
	output.Plain(fmt.Sprintf("\n%s", ticket.Content))
	return ToolResult{Output: *output, Data: ticket}
}

func ToolTicketIterateStart(year, month, day int, slug, prompt, model string) ToolResult {
	output := NewOutput()
	ticket, err := ReadTicket(year, month, day, slug)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := StartIteration(ticket, prompt, model); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🔄 Started iteration on ticket: %s", ticket.Slug))
	return ToolResult{Output: *output, Data: ticket}
}

func ToolTicketIterateEnd(year, month, day int, slug string) ToolResult {
	output := NewOutput()
	ticket, err := ReadTicket(year, month, day, slug)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := EndIteration(ticket); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n✅ Ended iteration on ticket: %s", ticket.Slug))
	return ToolResult{Output: *output, Data: ticket}
}

func ToolTicketFinish(year, month, day int, slug string) ToolResult {
	output := NewOutput()
	ticket, err := ReadTicket(year, month, day, slug)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := FinishTicket(ticket); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n✅ Ticket finished: %s", ticket.Slug))
	return ToolResult{Output: *output, Data: ticket}
}

func ToolProjectList() ToolResult {
	output := NewOutput()
	projects := GetNxProjects()
	output.Info(fmt.Sprintf("\n📦 Found %d projects:\n", len(projects)))
	for _, p := range projects {
		output.Plain(fmt.Sprintf("   %s", p.Name))
		output.Plain(fmt.Sprintf("      Root: %s", p.Root))
		if len(p.Tags) > 0 {
			output.Plain(fmt.Sprintf("      Tags: %s", strings.Join(p.Tags, ", ")))
		}
	}
	return ToolResult{Output: *output, Data: projects}
}

func ToolProjectTree() ToolResult {
	output := NewOutput()
	projects := GetNxProjects()
	output.Info("\n📦 Project tree:\n")
	for _, p := range projects {
		output.Plain(fmt.Sprintf("   └── %s (%s)", p.Name, p.Root))
	}
	return ToolResult{Output: *output, Data: projects}
}

func ToolFolderCreate(path string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, path)
	if FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: Folder already exists: %s", path))
		return ToolResult{Output: *output, Error: "folder already exists"}
	}
	if err := EnsureDir(absPath); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n📁 Created folder: %s", path))
	return ToolResult{Output: *output}
}

func ToolFolderMove(source, target string) ToolResult {
	output := NewOutput()
	absSource := filepath.Join(rootDir, source)
	absTarget := filepath.Join(rootDir, target)
	if !FileExists(absSource) {
		output.Error(fmt.Sprintf("Error: Source folder not found: %s", source))
		return ToolResult{Output: *output, Error: "source not found"}
	}
	if FileExists(absTarget) {
		output.Error(fmt.Sprintf("Error: Target folder already exists: %s", target))
		return ToolResult{Output: *output, Error: "target exists"}
	}
	if err := EnsureDir(filepath.Dir(absTarget)); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := os.Rename(absSource, absTarget); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n📁 Moved folder: %s → %s", source, target))
	return ToolResult{Output: *output}
}

func ToolFolderDelete(path string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, path)
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: Folder not found: %s", path))
		return ToolResult{Output: *output, Error: "folder not found"}
	}
	if err := os.RemoveAll(absPath); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🗑️ Deleted folder: %s", path))
	return ToolResult{Output: *output}
}

func ToolFolderList(path string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, strings.TrimSuffix(path, "/"))
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: Folder not found: %s", path))
		return ToolResult{Output: *output, Error: "folder not found"}
	}
	folders, err := ListDirEntries(absPath, true)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	var relPaths []string
	for _, f := range folders {
		relPaths = append(relPaths, NormalizePath(filepath.Join(path, f)))
	}
	ignored := GetGitIgnoredSet(relPaths)
	var filtered []string
	for _, f := range folders {
		relPath := NormalizePath(filepath.Join(path, f))
		if !ignored[relPath] && !ignored[relPath+"/"] {
			filtered = append(filtered, f)
		}
	}
	output.Info(fmt.Sprintf("\n📁 Found %d folders in %s:\n", len(filtered), path))
	for _, f := range filtered {
		output.Plain(fmt.Sprintf("   %s/", f))
	}
	return ToolResult{Output: *output, Data: filtered}
}

func ToolFolderTree(path string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, strings.TrimSuffix(path, "/"))
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: Folder not found: %s", path))
		return ToolResult{Output: *output, Error: "folder not found"}
	}
	output.Info(fmt.Sprintf("\n📁 Folder tree: %s\n", path))
	printTree(output, absPath, "")
	return ToolResult{Output: *output}
}

func printTree(output *CommandOutput, dir, prefix string) {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return
	}
	var items []os.DirEntry
	for _, e := range entries {
		if !strings.HasPrefix(e.Name(), ".") {
			items = append(items, e)
		}
	}
	var relPaths []string
	for _, e := range items {
		relPaths = append(relPaths, GetRelativePath(filepath.Join(dir, e.Name())))
	}
	ignored := GetGitIgnoredSet(relPaths)
	var filtered []os.DirEntry
	for _, e := range items {
		relPath := GetRelativePath(filepath.Join(dir, e.Name()))
		if !ignored[relPath] && !ignored[relPath+"/"] {
			filtered = append(filtered, e)
		}
	}
	for i, e := range filtered {
		isLast := i == len(filtered)-1
		connector := "├── "
		if isLast {
			connector = "└── "
		}
		suffix := ""
		if e.IsDir() {
			suffix = "/"
		}
		output.Plain(fmt.Sprintf("%s%s%s%s", prefix, connector, e.Name(), suffix))
		if e.IsDir() {
			newPrefix := prefix + "│   "
			if isLast {
				newPrefix = prefix + "    "
			}
			printTree(output, filepath.Join(dir, e.Name()), newPrefix)
		}
	}
}

func ToolFileCreate(path string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, path)
	if FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: File already exists: %s", path))
		return ToolResult{Output: *output, Error: "file already exists"}
	}
	lang := GetLanguageFromPath(path)
	content := generateFileHeader(path, lang)
	if err := WriteTextFile(absPath, content); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n📄 Created file: %s", path))
	return ToolResult{Output: *output}
}

func generateFileHeader(path, lang string) string {
	gitAuthor := GetGitAuthor()
	year := fmt.Sprintf("%d", PadNumber(2025, 4))
	license := `This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.`
	switch lang {
	case "typescript":
		return fmt.Sprintf(`// #region Header

// %s

// %s %s

%s

// #endregion Header
`, path, year, gitAuthor, formatLicenseLines(license, "//"))
	case "python":
		return fmt.Sprintf(`# region Header

# %s

# %s %s

%s

# endregion Header
`, path, year, gitAuthor, formatLicenseLines(license, "#"))
	case "csharp":
		return fmt.Sprintf(`#region Header

// %s

// %s %s

%s

#endregion Header
`, path, year, gitAuthor, formatLicenseLines(license, "//"))
	default:
		return ""
	}
}

func formatLicenseLines(license, prefix string) string {
	lines := strings.Split(license, "\n")
	var formatted []string
	for _, line := range lines {
		if line == "" {
			formatted = append(formatted, prefix)
		} else {
			formatted = append(formatted, prefix+" "+line)
		}
	}
	return strings.Join(formatted, "\n")
}

func ToolFileMove(source, target string) ToolResult {
	output := NewOutput()
	absSource := filepath.Join(rootDir, source)
	absTarget := filepath.Join(rootDir, target)
	if !FileExists(absSource) {
		output.Error(fmt.Sprintf("Error: Source file not found: %s", source))
		return ToolResult{Output: *output, Error: "source not found"}
	}
	if FileExists(absTarget) {
		output.Error(fmt.Sprintf("Error: Target file already exists: %s", target))
		return ToolResult{Output: *output, Error: "target exists"}
	}
	if err := EnsureDir(filepath.Dir(absTarget)); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := os.Rename(absSource, absTarget); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n📄 Moved file: %s → %s", source, target))
	return ToolResult{Output: *output}
}

func ToolFileDelete(path string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, path)
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: File not found: %s", path))
		return ToolResult{Output: *output, Error: "file not found"}
	}
	if err := os.Remove(absPath); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🗑️ Deleted file: %s", path))
	return ToolResult{Output: *output}
}

func ToolFileList(scopeRaw string) ToolResult {
	output := NewOutput()
	scope := ParseScope(scopeRaw)
	projects := GetNxProjects()
	files, err := ScopeToFiles(scope, projects)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Info(fmt.Sprintf("\n📄 Found %d files in scope \"%s\":\n", len(files), scopeRaw))
	for i, f := range files {
		if i >= 50 {
			output.Plain(fmt.Sprintf("   ... and %d more", len(files)-50))
			break
		}
		output.Plain(fmt.Sprintf("   %s", f))
	}
	return ToolResult{Output: *output, Data: files}
}

func ToolFileTree(path string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, strings.TrimSuffix(path, "/"))
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: Path not found: %s", path))
		return ToolResult{Output: *output, Error: "path not found"}
	}
	output.Info(fmt.Sprintf("\n📄 File tree: %s\n", path))
	printTree(output, absPath, "")
	return ToolResult{Output: *output}
}

func ToolSectionCreate(filePath, sectionPath string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, filePath)
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: File not found: %s", filePath))
		return ToolResult{Output: *output, Error: "file not found"}
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	parts := strings.Split(sectionPath, "#")
	sectionName := parts[len(parts)-1]
	lang := GetLanguageFromPath(filePath)
	var newSection string
	switch lang {
	case "markdown":
		newSection = fmt.Sprintf("\n## %s\n\n", sectionName)
	case "typescript":
		newSection = fmt.Sprintf("\n// #region %s\n\n// #endregion %s\n", sectionName, sectionName)
	case "python":
		newSection = fmt.Sprintf("\n# region %s\n\n# endregion %s\n", sectionName, sectionName)
	case "csharp":
		newSection = fmt.Sprintf("\n#region %s\n\n#endregion %s\n", sectionName, sectionName)
	default:
		output.Error("Error: Unsupported file type")
		return ToolResult{Output: *output, Error: "unsupported file type"}
	}
	if err := WriteTextFile(absPath, content+newSection); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🏷️ Created section \"%s\" in %s", sectionName, filePath))
	return ToolResult{Output: *output}
}

func ToolSectionMove(filePath, oldPath, newPath string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, filePath)
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: File not found: %s", filePath))
		return ToolResult{Output: *output, Error: "file not found"}
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	oldParts := strings.Split(oldPath, "#")
	oldName := oldParts[len(oldParts)-1]
	newParts := strings.Split(newPath, "#")
	newName := newParts[len(newParts)-1]
	lang := GetLanguageFromPath(filePath)
	switch lang {
	case "markdown":
		content = strings.ReplaceAll(content, "# "+oldName, "# "+newName)
		content = strings.ReplaceAll(content, "## "+oldName, "## "+newName)
	case "typescript":
		content = strings.ReplaceAll(content, "// #region "+oldName, "// #region "+newName)
		content = strings.ReplaceAll(content, "// #endregion "+oldName, "// #endregion "+newName)
	case "python":
		content = strings.ReplaceAll(content, "# region "+oldName, "# region "+newName)
		content = strings.ReplaceAll(content, "# endregion "+oldName, "# endregion "+newName)
	case "csharp":
		content = strings.ReplaceAll(content, "#region "+oldName, "#region "+newName)
		content = strings.ReplaceAll(content, "#endregion "+oldName, "#endregion "+newName)
	}
	if err := WriteTextFile(absPath, content); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🏷️ Renamed section \"%s\" to \"%s\" in %s", oldName, newName, filePath))
	return ToolResult{Output: *output}
}

func ToolSectionDelete(filePath, sectionPath string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, filePath)
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: File not found: %s", filePath))
		return ToolResult{Output: *output, Error: "file not found"}
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	sections := ParseSections(content, filePath)
	parts := strings.Split(sectionPath, "#")
	sectionName := parts[len(parts)-1]
	section := FindSection(sections, sectionName)
	if section == nil {
		output.Error(fmt.Sprintf("Error: Section not found: %s", sectionName))
		return ToolResult{Output: *output, Error: "section not found"}
	}
	lines := strings.Split(content, "\n")
	var newLines []string
	for i, line := range lines {
		lineNum := i + 1
		if lineNum < section.StartLine || lineNum > section.EndLine {
			newLines = append(newLines, line)
		}
	}
	if err := WriteTextFile(absPath, strings.Join(newLines, "\n")); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🗑️ Deleted section \"%s\" from %s", sectionName, filePath))
	return ToolResult{Output: *output}
}

func ToolSectionList(filePath string) ToolResult {
	output := NewOutput()
	scope := ParseScope(filePath)
	if scope.Kind != ScopeFile && scope.Kind != ScopeSection {
		output.Error("Error: Scope must be a file or section")
		return ToolResult{Output: *output, Error: "invalid scope"}
	}
	absPath := filepath.Join(rootDir, scope.FilePath)
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: File not found: %s", scope.FilePath))
		return ToolResult{Output: *output, Error: "file not found"}
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	sections := ParseSections(content, scope.FilePath)
	output.Info(fmt.Sprintf("\n🏷️ Sections in %s:\n", scope.FilePath))
	var printSection func(s SectionInfo, indent string)
	printSection = func(s SectionInfo, indent string) {
		output.Plain(fmt.Sprintf("%s%s (lines %d-%d)", indent, s.Name, s.StartLine, s.EndLine))
		for _, child := range s.Children {
			printSection(child, indent+"  ")
		}
	}
	for _, s := range sections {
		printSection(s, "   ")
	}
	if len(sections) == 0 {
		output.Plain("   (no sections found)")
	}
	return ToolResult{Output: *output, Data: sections}
}

func ToolSectionTree(filePath string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, strings.Split(filePath, "#")[0])
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: File not found: %s", filePath))
		return ToolResult{Output: *output, Error: "file not found"}
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	sections := ParseSections(content, filePath)
	output.Info(fmt.Sprintf("\n🏷️ Sections in %s:\n", filePath))
	var printSection func(s SectionInfo, prefix string)
	printSection = func(s SectionInfo, prefix string) {
		output.Plain(fmt.Sprintf("%s└── %s (lines %d-%d)", prefix, s.Name, s.StartLine, s.EndLine))
		for _, child := range s.Children {
			printSection(child, prefix+"    ")
		}
	}
	for _, s := range sections {
		printSection(s, "")
	}
	if len(sections) == 0 {
		output.Plain("   (no sections found)")
	}
	return ToolResult{Output: *output, Data: sections}
}

func ToolDefinitionList(filePath string) ToolResult {
	output := NewOutput()
	absPath := filepath.Join(rootDir, filePath)
	if !FileExists(absPath) {
		output.Error(fmt.Sprintf("Error: File not found: %s", filePath))
		return ToolResult{Output: *output, Error: "file not found"}
	}
	output.Info(fmt.Sprintf("\n📋 Definitions in %s:\n", filePath))
	output.Plain("   (definition parsing not implemented in Go - use TypeScript API)")
	return ToolResult{Output: *output, Data: []DefinitionInfo{}}
}

func ToolDefinitionTree(filePath string) ToolResult {
	return ToolDefinitionList(filePath)
}

func ToolRunTool(name string, args []string) ToolResult {
	output := NewOutput()
	switch name {
	case "update-metabolism":
		output.Info("\n🔄 Running update-metabolism via npx tsx...")
		stdout, stderr, exitCode := ExecCommand("npx", []string{"tsx", "scripts/update-metabolism.tsx"}, "")
		if exitCode != 0 {
			output.Error(fmt.Sprintf("Error: %s%s", stdout, stderr))
			return ToolResult{Output: *output, Error: "tool failed"}
		}
		output.Success(stdout)
	default:
		output.Info(fmt.Sprintf("\n🔧 Running Nx target: %s", name))
		var projects []string
		if scopeFlag != "" {
			projects = []string{scopeFlag}
		}
		success, out := RunNxTarget(name, projects, args)
		if !success {
			output.Error(out)
			return ToolResult{Output: *output, Error: "nx target failed"}
		}
		output.Plain(out)
	}
	return ToolResult{Output: *output}
}

