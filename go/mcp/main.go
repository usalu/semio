// #region Header

// go/mcp/main.go

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
	"math"
	"os"
	"path/filepath"
	"strings"

	"github.com/mark3labs/mcp-go/mcp"
	"github.com/mark3labs/mcp-go/server"
	"github.com/usalu/semio/go/repo"
)

func init() {
	wd, _ := os.Getwd()
	rootDir := findRepoRoot(wd)
	repo.SetRootDir(rootDir)
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

func main() {
	s := server.NewMCPServer(
		"semio-repo",
		"1.0.0",
		server.WithToolCapabilities(true),
	)
	s.AddTool(
		mcp.NewTool("analyze",
			mcp.WithDescription("Analyze codebase for policy violations"),
			mcp.WithString("scope", mcp.Description("Scope to analyze (e.g., @semio, @semio/js, path/to/file.ts)"), mcp.DefaultString("@semio")),
		),
		analyze,
	)
	s.AddTool(
		mcp.NewTool("fix",
			mcp.WithDescription("Apply autofixes for policy violations"),
			mcp.WithString("scope", mcp.Description("Scope to fix"), mcp.DefaultString("@semio")),
		),
		fix,
	)
	s.AddTool(
		mcp.NewTool("policy_list",
			mcp.WithDescription("List all registered policies"),
		),
		policyList,
	)
	s.AddTool(
		mcp.NewTool("policy_check",
			mcp.WithDescription("Check a specific policy"),
			mcp.WithString("id", mcp.Required(), mcp.Description("Policy ID to check")),
			mcp.WithString("scope", mcp.Description("Scope to analyze"), mcp.DefaultString("@semio")),
		),
		policyCheck,
	)
	s.AddTool(
		mcp.NewTool("ticket_create",
			mcp.WithDescription("Create a new development ticket"),
			mcp.WithString("slug", mcp.Required(), mcp.Description("Ticket slug (will be uppercased and kebab-cased)")),
			mcp.WithString("prompt", mcp.Required(), mcp.Description("Ticket prompt/description")),
			mcp.WithString("model", mcp.Required(), mcp.Description("Large-Language-Model (LLM) used for this ticket")),
			mcp.WithArray("files", mcp.Description("Files to include (at least one required)")),
		),
		ticketCreate,
	)
	s.AddTool(
		mcp.NewTool("ticket_list",
			mcp.WithDescription("List development tickets"),
			mcp.WithNumber("year", mcp.Description("Filter by year")),
			mcp.WithNumber("month", mcp.Description("Filter by month")),
			mcp.WithNumber("day", mcp.Description("Filter by day")),
		),
		ticketList,
	)
	s.AddTool(
		mcp.NewTool("ticket_read",
			mcp.WithDescription("Read a specific ticket"),
			mcp.WithNumber("year", mcp.Required(), mcp.Description("Ticket year")),
			mcp.WithNumber("month", mcp.Required(), mcp.Description("Ticket month")),
			mcp.WithNumber("day", mcp.Required(), mcp.Description("Ticket day")),
			mcp.WithString("slug", mcp.Required(), mcp.Description("Ticket slug")),
		),
		ticketRead,
	)
	s.AddTool(
		mcp.NewTool("ticket_progress",
			mcp.WithDescription("Record progress on a ticket (creates iteration from git changes)"),
			mcp.WithNumber("year", mcp.Required(), mcp.Description("Ticket year")),
			mcp.WithNumber("month", mcp.Required(), mcp.Description("Ticket month")),
			mcp.WithNumber("day", mcp.Required(), mcp.Description("Ticket day")),
			mcp.WithString("slug", mcp.Required(), mcp.Description("Ticket slug")),
			mcp.WithString("prompt", mcp.Required(), mcp.Description("Iteration prompt")),
			mcp.WithString("model", mcp.Required(), mcp.Description("Large-Language-Model (LLM) used")),
		),
		ticketProgress,
	)
	s.AddTool(
		mcp.NewTool("ticket_finish",
			mcp.WithDescription("Finish a ticket"),
			mcp.WithNumber("year", mcp.Required(), mcp.Description("Ticket year")),
			mcp.WithNumber("month", mcp.Required(), mcp.Description("Ticket month")),
			mcp.WithNumber("day", mcp.Required(), mcp.Description("Ticket day")),
			mcp.WithString("slug", mcp.Required(), mcp.Description("Ticket slug")),
		),
		ticketFinish,
	)
	s.AddTool(
		mcp.NewTool("contributor_add",
			mcp.WithDescription("Add a contributor by GitHub username"),
			mcp.WithString("github", mcp.Required(), mcp.Description("GitHub username")),
		),
		contributorAdd,
	)
	s.AddTool(
		mcp.NewTool("contributor_list",
			mcp.WithDescription("List all contributors"),
		),
		contributorList,
	)
	s.AddTool(
		mcp.NewTool("contributor_remove",
			mcp.WithDescription("Remove a contributor"),
			mcp.WithString("github", mcp.Required(), mcp.Description("GitHub username")),
		),
		contributorRemove,
	)
	s.AddTool(
		mcp.NewTool("project_list",
			mcp.WithDescription("List Nx bundles in the monorepo"),
		),
		projectList,
	)
	s.AddTool(
		mcp.NewTool("project_tree",
			mcp.WithDescription("Show bundle dependency tree"),
		),
		projectTree,
	)
	s.AddTool(
		mcp.NewTool("folder_create",
			mcp.WithDescription("Create a folder"),
			mcp.WithString("path", mcp.Required(), mcp.Description("Folder path to create")),
		),
		folderCreate,
	)
	s.AddTool(
		mcp.NewTool("folder_move",
			mcp.WithDescription("Move a folder"),
			mcp.WithString("source", mcp.Required(), mcp.Description("Source folder path")),
			mcp.WithString("target", mcp.Required(), mcp.Description("Target folder path")),
		),
		folderMove,
	)
	s.AddTool(
		mcp.NewTool("folder_delete",
			mcp.WithDescription("Delete a folder"),
			mcp.WithString("path", mcp.Required(), mcp.Description("Folder path to delete")),
		),
		folderDelete,
	)
	s.AddTool(
		mcp.NewTool("folder_list",
			mcp.WithDescription("List folders in a path"),
			mcp.WithString("path", mcp.Description("Path to list folders from"), mcp.DefaultString(".")),
		),
		folderList,
	)
	s.AddTool(
		mcp.NewTool("folder_tree",
			mcp.WithDescription("Show folder tree structure"),
			mcp.WithString("path", mcp.Description("Path to show tree from"), mcp.DefaultString(".")),
		),
		folderTree,
	)
	s.AddTool(
		mcp.NewTool("file_create",
			mcp.WithDescription("Create a file with appropriate header"),
			mcp.WithString("path", mcp.Required(), mcp.Description("File path to create")),
		),
		fileCreate,
	)
	s.AddTool(
		mcp.NewTool("file_move",
			mcp.WithDescription("Move a file"),
			mcp.WithString("source", mcp.Required(), mcp.Description("Source file path")),
			mcp.WithString("target", mcp.Required(), mcp.Description("Target file path")),
		),
		fileMove,
	)
	s.AddTool(
		mcp.NewTool("file_delete",
			mcp.WithDescription("Delete a file"),
			mcp.WithString("path", mcp.Required(), mcp.Description("File path to delete")),
		),
		fileDelete,
	)
	s.AddTool(
		mcp.NewTool("file_list",
			mcp.WithDescription("List files in scope"),
			mcp.WithString("scope", mcp.Description("Scope to list files from"), mcp.DefaultString("@semio")),
		),
		fileList,
	)
	s.AddTool(
		mcp.NewTool("file_tree",
			mcp.WithDescription("Show file tree structure"),
			mcp.WithString("path", mcp.Description("Path to show tree from"), mcp.DefaultString(".")),
		),
		fileTree,
	)
	s.AddTool(
		mcp.NewTool("section_create",
			mcp.WithDescription("Create a section in a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
			mcp.WithString("section", mcp.Required(), mcp.Description("Section name")),
		),
		sectionCreate,
	)
	s.AddTool(
		mcp.NewTool("section_move",
			mcp.WithDescription("Rename a section in a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
			mcp.WithString("old_name", mcp.Required(), mcp.Description("Old section name")),
			mcp.WithString("new_name", mcp.Required(), mcp.Description("New section name")),
		),
		sectionMove,
	)
	s.AddTool(
		mcp.NewTool("section_delete",
			mcp.WithDescription("Delete a section from a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
			mcp.WithString("section", mcp.Required(), mcp.Description("Section name")),
		),
		sectionDelete,
	)
	s.AddTool(
		mcp.NewTool("section_list",
			mcp.WithDescription("List sections in a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
		),
		sectionList,
	)
	s.AddTool(
		mcp.NewTool("section_tree",
			mcp.WithDescription("Show section tree in a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
		),
		sectionTree,
	)
	s.AddTool(
		mcp.NewTool("definition_list",
			mcp.WithDescription("List definitions in a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
		),
		definitionList,
	)
	s.AddTool(
		mcp.NewTool("graphql",
			mcp.WithDescription("Execute a GraphQL query against the repo schema"),
			mcp.WithString("query", mcp.Required(), mcp.Description("GraphQL query string")),
			mcp.WithString("variables", mcp.Description("JSON object with query variables")),
		),
		graphqlQuery,
	)
	if err := server.ServeStdio(s); err != nil {
		fmt.Printf("Server error: %v\n", err)
	}
}

func textResult(text string) *mcp.CallToolResult {
	return mcp.NewToolResultText(text)
}

// #region Args
func getArgs(request mcp.CallToolRequest) map[string]interface{} {
	if args, ok := request.Params.Arguments.(map[string]interface{}); ok {
		return args
	}
	return make(map[string]interface{})
}

func getStringArg(args map[string]interface{}, key string) (string, bool, error) {
	value, ok := args[key]
	if !ok {
		return "", false, nil
	}
	str, ok := value.(string)
	if !ok || str == "" {
		return "", true, fmt.Errorf("invalid %s", key)
	}
	return str, true, nil
}

func requireStringArg(args map[string]interface{}, key string) (string, error) {
	value, ok, err := getStringArg(args, key)
	if err != nil {
		return "", err
	}
	if !ok {
		return "", fmt.Errorf("missing %s", key)
	}
	return value, nil
}

func getIntArg(args map[string]interface{}, key string) (int, bool, error) {
	value, ok := args[key]
	if !ok {
		return 0, false, nil
	}
	number, ok := value.(float64)
	if !ok || number != math.Trunc(number) {
		return 0, true, fmt.Errorf("invalid %s", key)
	}
	return int(number), true, nil
}

func requireIntArg(args map[string]interface{}, key string) (int, error) {
	value, ok, err := getIntArg(args, key)
	if err != nil {
		return 0, err
	}
	if !ok {
		return 0, fmt.Errorf("missing %s", key)
	}
	return value, nil
}

func getStringSliceArg(args map[string]interface{}, key string) ([]string, bool, error) {
	value, ok := args[key]
	if !ok {
		return nil, false, nil
	}
	list, ok := value.([]interface{})
	if !ok || len(list) == 0 {
		return nil, true, fmt.Errorf("invalid %s", key)
	}
	result := make([]string, 0, len(list))
	for _, item := range list {
		str, ok := item.(string)
		if !ok || str == "" {
			return nil, true, fmt.Errorf("invalid %s", key)
		}
		result = append(result, str)
	}
	return result, true, nil
}

// #endregion Args

// #region Paths
func requireFilePath(path string) error {
	info, err := os.Stat(path)
	if err != nil {
		return fmt.Errorf("invalid file path: %s", path)
	}
	if info.IsDir() {
		return fmt.Errorf("invalid file path: %s", path)
	}
	return nil
}

func requireFolderPath(path string) error {
	info, err := os.Stat(path)
	if err != nil {
		return fmt.Errorf("invalid folder path: %s", path)
	}
	if !info.IsDir() {
		return fmt.Errorf("invalid folder path: %s", path)
	}
	return nil
}

func requireFileTargetPath(path string) error {
	info, err := os.Stat(path)
	if err == nil {
		if info.IsDir() {
			return fmt.Errorf("invalid file path: %s", path)
		}
		return nil
	}
	if !os.IsNotExist(err) {
		return fmt.Errorf("invalid file path: %s", path)
	}
	return nil
}

func requireFolderTargetPath(path string) error {
	info, err := os.Stat(path)
	if err == nil {
		if !info.IsDir() {
			return fmt.Errorf("invalid folder path: %s", path)
		}
		return nil
	}
	if !os.IsNotExist(err) {
		return fmt.Errorf("invalid folder path: %s", path)
	}
	return nil
}

// #endregion Paths

// #region Handlers
func analyze(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	scope, ok, err := getStringArg(args, "scope")
	if err != nil {
		return nil, err
	}
	if !ok {
		scope = "@semio"
	}
	result := repo.ToolAnalyze(scope, nil)
	return textResult(repo.FormatResult(result)), nil
}

func fix(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	scope, ok, err := getStringArg(args, "scope")
	if err != nil {
		return nil, err
	}
	if !ok {
		scope = "@semio"
	}
	result := repo.ToolFix(scope)
	return textResult(repo.FormatResult(result)), nil
}

func policyList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	result := repo.ToolPolicyList()
	return textResult(repo.FormatResult(result)), nil
}

func policyCheck(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	id, err := requireStringArg(args, "id")
	if err != nil {
		return nil, err
	}
	scope, ok, err := getStringArg(args, "scope")
	if err != nil {
		return nil, err
	}
	if !ok {
		scope = "@semio"
	}
	result := repo.ToolPolicyCheck(id, scope)
	return textResult(repo.FormatResult(result)), nil
}

func ticketCreate(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	slug, err := requireStringArg(args, "slug")
	if err != nil {
		return nil, err
	}
	prompt, err := requireStringArg(args, "prompt")
	if err != nil {
		return nil, err
	}
	model, err := requireStringArg(args, "model")
	if err != nil {
		return nil, err
	}
	files, _, err := getStringSliceArg(args, "files")
	if err != nil {
		return nil, err
	}
	for _, file := range files {
		if err := requireFilePath(file); err != nil {
			return nil, err
		}
	}
	result := repo.ToolTicketCreate(slug, prompt, model, files)
	return textResult(repo.FormatResult(result)), nil
}

func ticketList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	year, yearOk, err := getIntArg(args, "year")
	if err != nil {
		return nil, err
	}
	month, monthOk, err := getIntArg(args, "month")
	if err != nil {
		return nil, err
	}
	day, dayOk, err := getIntArg(args, "day")
	if err != nil {
		return nil, err
	}
	if monthOk && !yearOk {
		return nil, fmt.Errorf("missing year")
	}
	if dayOk && !monthOk {
		return nil, fmt.Errorf("missing month")
	}
	var yp, mp, dp *int
	if yearOk {
		yp = &year
	}
	if monthOk {
		mp = &month
	}
	if dayOk {
		dp = &day
	}
	result := repo.ToolTicketList(yp, mp, dp)
	return textResult(repo.FormatResult(result)), nil
}

func ticketRead(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	year, err := requireIntArg(args, "year")
	if err != nil {
		return nil, err
	}
	month, err := requireIntArg(args, "month")
	if err != nil {
		return nil, err
	}
	day, err := requireIntArg(args, "day")
	if err != nil {
		return nil, err
	}
	slug, err := requireStringArg(args, "slug")
	if err != nil {
		return nil, err
	}
	result := repo.ToolTicketRead(year, month, day, slug)
	return textResult(repo.FormatResult(result)), nil
}

func ticketProgress(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	year, err := requireIntArg(args, "year")
	if err != nil {
		return nil, err
	}
	month, err := requireIntArg(args, "month")
	if err != nil {
		return nil, err
	}
	day, err := requireIntArg(args, "day")
	if err != nil {
		return nil, err
	}
	slug, err := requireStringArg(args, "slug")
	if err != nil {
		return nil, err
	}
	prompt, err := requireStringArg(args, "prompt")
	if err != nil {
		return nil, err
	}
	model, err := requireStringArg(args, "model")
	if err != nil {
		return nil, err
	}
	result := repo.ToolTicketProgress(year, month, day, slug, prompt, model)
	return textResult(repo.FormatResult(result)), nil
}

func ticketFinish(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	year, err := requireIntArg(args, "year")
	if err != nil {
		return nil, err
	}
	month, err := requireIntArg(args, "month")
	if err != nil {
		return nil, err
	}
	day, err := requireIntArg(args, "day")
	if err != nil {
		return nil, err
	}
	slug, err := requireStringArg(args, "slug")
	if err != nil {
		return nil, err
	}
	result := repo.ToolTicketFinish(year, month, day, slug)
	return textResult(repo.FormatResult(result)), nil
}

func contributorAdd(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	github, err := requireStringArg(args, "github")
	if err != nil {
		return nil, err
	}
	result := repo.ToolContributorAdd(github)
	return textResult(repo.FormatResult(result)), nil
}

func contributorList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	result := repo.ToolContributorList()
	return textResult(repo.FormatResult(result)), nil
}

func contributorRemove(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	github, err := requireStringArg(args, "github")
	if err != nil {
		return nil, err
	}
	result := repo.ToolContributorRemove(github)
	return textResult(repo.FormatResult(result)), nil
}

func projectList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	result := repo.ToolProjectList()
	return textResult(repo.FormatResult(result)), nil
}

func projectTree(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	result := repo.ToolProjectTree()
	return textResult(repo.FormatResult(result)), nil
}

func folderCreate(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	if err := requireFolderTargetPath(path); err != nil {
		return nil, err
	}
	result := repo.ToolFolderCreate(path)
	return textResult(repo.FormatResult(result)), nil
}

func folderMove(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	source, err := requireStringArg(args, "source")
	if err != nil {
		return nil, err
	}
	target, err := requireStringArg(args, "target")
	if err != nil {
		return nil, err
	}
	if err := requireFolderPath(source); err != nil {
		return nil, err
	}
	if err := requireFolderTargetPath(target); err != nil {
		return nil, err
	}
	result := repo.ToolFolderMove(source, target)
	return textResult(repo.FormatResult(result)), nil
}

func folderDelete(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	if err := requireFolderPath(path); err != nil {
		return nil, err
	}
	result := repo.ToolFolderDelete(path)
	return textResult(repo.FormatResult(result)), nil
}

func folderList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, ok, err := getStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	if !ok {
		path = "."
	}
	if err := requireFolderPath(path); err != nil {
		return nil, err
	}
	result := repo.ToolFolderList(path)
	return textResult(repo.FormatResult(result)), nil
}

func folderTree(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, ok, err := getStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	if !ok {
		path = "."
	}
	if err := requireFolderPath(path); err != nil {
		return nil, err
	}
	result := repo.ToolFolderTree(path)
	return textResult(repo.FormatResult(result)), nil
}

func fileCreate(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	if err := requireFileTargetPath(path); err != nil {
		return nil, err
	}
	result := repo.ToolFileCreate(path)
	return textResult(repo.FormatResult(result)), nil
}

func fileMove(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	source, err := requireStringArg(args, "source")
	if err != nil {
		return nil, err
	}
	target, err := requireStringArg(args, "target")
	if err != nil {
		return nil, err
	}
	if err := requireFilePath(source); err != nil {
		return nil, err
	}
	if err := requireFileTargetPath(target); err != nil {
		return nil, err
	}
	result := repo.ToolFileMove(source, target)
	return textResult(repo.FormatResult(result)), nil
}

func fileDelete(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, err := requireStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	if err := requireFilePath(path); err != nil {
		return nil, err
	}
	result := repo.ToolFileDelete(path)
	return textResult(repo.FormatResult(result)), nil
}

func fileList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	scope, ok, err := getStringArg(args, "scope")
	if err != nil {
		return nil, err
	}
	if !ok {
		scope = "@semio"
	}
	result := repo.ToolFileList(scope)
	return textResult(repo.FormatResult(result)), nil
}

func fileTree(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, ok, err := getStringArg(args, "path")
	if err != nil {
		return nil, err
	}
	if !ok {
		path = "."
	}
	if err := requireFolderPath(path); err != nil {
		return nil, err
	}
	result := repo.ToolFileTree(path)
	return textResult(repo.FormatResult(result)), nil
}

func sectionCreate(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	section, err := requireStringArg(args, "section")
	if err != nil {
		return nil, err
	}
	if err := requireFilePath(file); err != nil {
		return nil, err
	}
	result := repo.ToolSectionCreate(file, section)
	return textResult(repo.FormatResult(result)), nil
}

func sectionMove(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	oldName, err := requireStringArg(args, "old_name")
	if err != nil {
		return nil, err
	}
	newName, err := requireStringArg(args, "new_name")
	if err != nil {
		return nil, err
	}
	if err := requireFilePath(file); err != nil {
		return nil, err
	}
	result := repo.ToolSectionMove(file, oldName, newName)
	return textResult(repo.FormatResult(result)), nil
}

func sectionDelete(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	section, err := requireStringArg(args, "section")
	if err != nil {
		return nil, err
	}
	if err := requireFilePath(file); err != nil {
		return nil, err
	}
	result := repo.ToolSectionDelete(file, section)
	return textResult(repo.FormatResult(result)), nil
}

func sectionList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	if err := requireFilePath(file); err != nil {
		return nil, err
	}
	result := repo.ToolSectionList(file)
	return textResult(repo.FormatResult(result)), nil
}

func sectionTree(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	if err := requireFilePath(file); err != nil {
		return nil, err
	}
	result := repo.ToolSectionTree(file)
	return textResult(repo.FormatResult(result)), nil
}

func definitionList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, err := requireStringArg(args, "file")
	if err != nil {
		return nil, err
	}
	if err := requireFilePath(file); err != nil {
		return nil, err
	}
	result := repo.ToolDefinitionList(file)
	return textResult(repo.FormatResult(result)), nil
}

func graphqlQuery(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	query, err := requireStringArg(args, "query")
	if err != nil {
		return nil, err
	}
	variablesStr, _, err := getStringArg(args, "variables")
	if err != nil {
		return nil, err
	}
	var variables map[string]interface{}
	if variablesStr != "" {
		if err := json.Unmarshal([]byte(variablesStr), &variables); err != nil {
			return nil, fmt.Errorf("invalid variables JSON: %w", err)
		}
	}
	result, gqlErr := repo.ExecuteGraphQL(query, variables)
	if gqlErr != nil {
		return textResult(fmt.Sprintf(`{"error": %q}`, gqlErr.Error())), nil
	}
	return textResult(result), nil
}

// #endregion Handlers

var _ = strings.TrimSpace
