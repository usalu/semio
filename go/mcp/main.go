// mcp/main.go

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

package main

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strconv"
	"strings"

	"github.com/mark3labs/mcp-go/mcp"
	"github.com/mark3labs/mcp-go/server"
)

var repoPath string

func init() {
	wd, _ := os.Getwd()
	repoPath = findRepoBinary(wd)
}

func findRepoBinary(start string) string {
	dir := start
	for {
		binPath := filepath.Join(dir, "go", "repo", "repo")
		if _, err := os.Stat(binPath); err == nil {
			return binPath
		}
		binPath = filepath.Join(dir, "go", "repo", "repo.exe")
		if _, err := os.Stat(binPath); err == nil {
			return binPath
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			return "repo"
		}
		dir = parent
	}
}

func runRepo(args ...string) string {
	cmd := exec.Command(repoPath, args...)
	out, _ := cmd.Output()
	return string(out)
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
		analyzeHandler,
	)
	s.AddTool(
		mcp.NewTool("fix",
			mcp.WithDescription("Apply autofixes for policy violations"),
			mcp.WithString("scope", mcp.Description("Scope to fix"), mcp.DefaultString("@semio")),
		),
		fixHandler,
	)
	s.AddTool(
		mcp.NewTool("policy_list",
			mcp.WithDescription("List all registered policies"),
		),
		policyListHandler,
	)
	s.AddTool(
		mcp.NewTool("policy_run",
			mcp.WithDescription("Run a specific policy"),
			mcp.WithString("id", mcp.Required(), mcp.Description("Policy ID to run")),
			mcp.WithString("scope", mcp.Description("Scope to analyze"), mcp.DefaultString("@semio")),
		),
		policyRunHandler,
	)
	s.AddTool(
		mcp.NewTool("ticket_create",
			mcp.WithDescription("Create a new development ticket"),
			mcp.WithString("slug", mcp.Required(), mcp.Description("Ticket slug (will be uppercased and kebab-cased)")),
			mcp.WithString("prompt", mcp.Description("Ticket prompt/description")),
			mcp.WithString("model", mcp.Description("Model used for this ticket")),
		),
		ticketCreateHandler,
	)
	s.AddTool(
		mcp.NewTool("ticket_list",
			mcp.WithDescription("List development tickets"),
			mcp.WithNumber("year", mcp.Description("Filter by year")),
			mcp.WithNumber("month", mcp.Description("Filter by month")),
			mcp.WithNumber("day", mcp.Description("Filter by day")),
		),
		ticketListHandler,
	)
	s.AddTool(
		mcp.NewTool("ticket_read",
			mcp.WithDescription("Read a specific ticket"),
			mcp.WithNumber("year", mcp.Required(), mcp.Description("Ticket year")),
			mcp.WithNumber("month", mcp.Required(), mcp.Description("Ticket month")),
			mcp.WithNumber("day", mcp.Required(), mcp.Description("Ticket day")),
			mcp.WithString("slug", mcp.Required(), mcp.Description("Ticket slug")),
		),
		ticketReadHandler,
	)
	s.AddTool(
		mcp.NewTool("ticket_iterate_start",
			mcp.WithDescription("Start a ticket iteration"),
			mcp.WithNumber("year", mcp.Required(), mcp.Description("Ticket year")),
			mcp.WithNumber("month", mcp.Required(), mcp.Description("Ticket month")),
			mcp.WithNumber("day", mcp.Required(), mcp.Description("Ticket day")),
			mcp.WithString("slug", mcp.Required(), mcp.Description("Ticket slug")),
			mcp.WithString("prompt", mcp.Description("Iteration prompt")),
			mcp.WithString("model", mcp.Description("Model used")),
		),
		ticketIterateStartHandler,
	)
	s.AddTool(
		mcp.NewTool("ticket_iterate_end",
			mcp.WithDescription("End a ticket iteration"),
			mcp.WithNumber("year", mcp.Required(), mcp.Description("Ticket year")),
			mcp.WithNumber("month", mcp.Required(), mcp.Description("Ticket month")),
			mcp.WithNumber("day", mcp.Required(), mcp.Description("Ticket day")),
			mcp.WithString("slug", mcp.Required(), mcp.Description("Ticket slug")),
		),
		ticketIterateEndHandler,
	)
	s.AddTool(
		mcp.NewTool("ticket_finish",
			mcp.WithDescription("Finish a ticket"),
			mcp.WithNumber("year", mcp.Required(), mcp.Description("Ticket year")),
			mcp.WithNumber("month", mcp.Required(), mcp.Description("Ticket month")),
			mcp.WithNumber("day", mcp.Required(), mcp.Description("Ticket day")),
			mcp.WithString("slug", mcp.Required(), mcp.Description("Ticket slug")),
		),
		ticketFinishHandler,
	)
	s.AddTool(
		mcp.NewTool("project_list",
			mcp.WithDescription("List Nx projects in the monorepo"),
		),
		projectListHandler,
	)
	s.AddTool(
		mcp.NewTool("project_tree",
			mcp.WithDescription("Show project dependency tree"),
		),
		projectTreeHandler,
	)
	s.AddTool(
		mcp.NewTool("folder_create",
			mcp.WithDescription("Create a folder"),
			mcp.WithString("path", mcp.Required(), mcp.Description("Folder path to create")),
		),
		folderCreateHandler,
	)
	s.AddTool(
		mcp.NewTool("folder_move",
			mcp.WithDescription("Move a folder"),
			mcp.WithString("source", mcp.Required(), mcp.Description("Source folder path")),
			mcp.WithString("target", mcp.Required(), mcp.Description("Target folder path")),
		),
		folderMoveHandler,
	)
	s.AddTool(
		mcp.NewTool("folder_delete",
			mcp.WithDescription("Delete a folder"),
			mcp.WithString("path", mcp.Required(), mcp.Description("Folder path to delete")),
		),
		folderDeleteHandler,
	)
	s.AddTool(
		mcp.NewTool("folder_list",
			mcp.WithDescription("List folders in a path"),
			mcp.WithString("path", mcp.Description("Path to list folders from"), mcp.DefaultString(".")),
		),
		folderListHandler,
	)
	s.AddTool(
		mcp.NewTool("folder_tree",
			mcp.WithDescription("Show folder tree structure"),
			mcp.WithString("path", mcp.Description("Path to show tree from"), mcp.DefaultString(".")),
		),
		folderTreeHandler,
	)
	s.AddTool(
		mcp.NewTool("file_create",
			mcp.WithDescription("Create a file with appropriate header"),
			mcp.WithString("path", mcp.Required(), mcp.Description("File path to create")),
		),
		fileCreateHandler,
	)
	s.AddTool(
		mcp.NewTool("file_move",
			mcp.WithDescription("Move a file"),
			mcp.WithString("source", mcp.Required(), mcp.Description("Source file path")),
			mcp.WithString("target", mcp.Required(), mcp.Description("Target file path")),
		),
		fileMoveHandler,
	)
	s.AddTool(
		mcp.NewTool("file_delete",
			mcp.WithDescription("Delete a file"),
			mcp.WithString("path", mcp.Required(), mcp.Description("File path to delete")),
		),
		fileDeleteHandler,
	)
	s.AddTool(
		mcp.NewTool("file_list",
			mcp.WithDescription("List files in scope"),
			mcp.WithString("scope", mcp.Description("Scope to list files from"), mcp.DefaultString("@semio")),
		),
		fileListHandler,
	)
	s.AddTool(
		mcp.NewTool("file_tree",
			mcp.WithDescription("Show file tree structure"),
			mcp.WithString("path", mcp.Description("Path to show tree from"), mcp.DefaultString(".")),
		),
		fileTreeHandler,
	)
	s.AddTool(
		mcp.NewTool("section_create",
			mcp.WithDescription("Create a section in a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
			mcp.WithString("section", mcp.Required(), mcp.Description("Section name")),
		),
		sectionCreateHandler,
	)
	s.AddTool(
		mcp.NewTool("section_move",
			mcp.WithDescription("Rename a section in a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
			mcp.WithString("old_name", mcp.Required(), mcp.Description("Old section name")),
			mcp.WithString("new_name", mcp.Required(), mcp.Description("New section name")),
		),
		sectionMoveHandler,
	)
	s.AddTool(
		mcp.NewTool("section_delete",
			mcp.WithDescription("Delete a section from a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
			mcp.WithString("section", mcp.Required(), mcp.Description("Section name")),
		),
		sectionDeleteHandler,
	)
	s.AddTool(
		mcp.NewTool("section_list",
			mcp.WithDescription("List sections in a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
		),
		sectionListHandler,
	)
	s.AddTool(
		mcp.NewTool("section_tree",
			mcp.WithDescription("Show section tree in a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
		),
		sectionTreeHandler,
	)
	s.AddTool(
		mcp.NewTool("definition_list",
			mcp.WithDescription("List definitions in a file"),
			mcp.WithString("file", mcp.Required(), mcp.Description("File path")),
		),
		definitionListHandler,
	)
	s.AddTool(
		mcp.NewTool("tool_run",
			mcp.WithDescription("Run a tool or Nx target"),
			mcp.WithString("name", mcp.Required(), mcp.Description("Tool/target name")),
		),
		toolRunHandler,
	)
	if err := server.ServeStdio(s); err != nil {
		fmt.Printf("Server error: %v\n", err)
	}
}

func textResult(text string) *mcp.CallToolResult {
	return mcp.NewToolResultText(text)
}

func getArgs(request mcp.CallToolRequest) map[string]interface{} {
	if args, ok := request.Params.Arguments.(map[string]interface{}); ok {
		return args
	}
	return make(map[string]interface{})
}

func analyzeHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	scope, _ := args["scope"].(string)
	if scope == "" {
		scope = "@semio"
	}
	return textResult(runRepo("analyze", scope)), nil
}

func fixHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	scope, _ := args["scope"].(string)
	if scope == "" {
		scope = "@semio"
	}
	return textResult(runRepo("fix", scope)), nil
}

func policyListHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	return textResult(runRepo("policy", "list")), nil
}

func policyRunHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	id, _ := args["id"].(string)
	scope, _ := args["scope"].(string)
	if scope == "" {
		return textResult(runRepo("policy", "run", id)), nil
	}
	return textResult(runRepo("policy", "run", id, scope)), nil
}

func ticketCreateHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	slug, _ := args["slug"].(string)
	prompt, _ := args["prompt"].(string)
	model, _ := args["model"].(string)
	cmdArgs := []string{"ticket", "create", slug}
	if prompt != "" {
		cmdArgs = append(cmdArgs, "--prompt="+prompt)
	}
	if model != "" {
		cmdArgs = append(cmdArgs, "--model="+model)
	}
	return textResult(runRepo(cmdArgs...)), nil
}

func ticketListHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	cmdArgs := []string{"ticket", "list"}
	if y, ok := args["year"].(float64); ok {
		cmdArgs = append(cmdArgs, strconv.Itoa(int(y)))
		if m, ok := args["month"].(float64); ok {
			cmdArgs = append(cmdArgs, strconv.Itoa(int(m)))
			if d, ok := args["day"].(float64); ok {
				cmdArgs = append(cmdArgs, strconv.Itoa(int(d)))
			}
		}
	}
	return textResult(runRepo(cmdArgs...)), nil
}

func ticketReadHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	year := strconv.Itoa(int(args["year"].(float64)))
	month := strconv.Itoa(int(args["month"].(float64)))
	day := strconv.Itoa(int(args["day"].(float64)))
	slug, _ := args["slug"].(string)
	return textResult(runRepo("ticket", "read", year, month, day, slug)), nil
}

func ticketIterateStartHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	year := strconv.Itoa(int(args["year"].(float64)))
	month := strconv.Itoa(int(args["month"].(float64)))
	day := strconv.Itoa(int(args["day"].(float64)))
	slug, _ := args["slug"].(string)
	prompt, _ := args["prompt"].(string)
	model, _ := args["model"].(string)
	cmdArgs := []string{"ticket", "iterate", "start", year, month, day, slug}
	if prompt != "" {
		cmdArgs = append(cmdArgs, "--prompt="+prompt)
	}
	if model != "" {
		cmdArgs = append(cmdArgs, "--model="+model)
	}
	return textResult(runRepo(cmdArgs...)), nil
}

func ticketIterateEndHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	year := strconv.Itoa(int(args["year"].(float64)))
	month := strconv.Itoa(int(args["month"].(float64)))
	day := strconv.Itoa(int(args["day"].(float64)))
	slug, _ := args["slug"].(string)
	return textResult(runRepo("ticket", "iterate", "end", year, month, day, slug)), nil
}

func ticketFinishHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	year := strconv.Itoa(int(args["year"].(float64)))
	month := strconv.Itoa(int(args["month"].(float64)))
	day := strconv.Itoa(int(args["day"].(float64)))
	slug, _ := args["slug"].(string)
	return textResult(runRepo("ticket", "finish", year, month, day, slug)), nil
}

func projectListHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	return textResult(runRepo("project", "list")), nil
}

func projectTreeHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	return textResult(runRepo("project", "tree")), nil
}

func folderCreateHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, _ := args["path"].(string)
	return textResult(runRepo("folder", "create", path)), nil
}

func folderMoveHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	source, _ := args["source"].(string)
	target, _ := args["target"].(string)
	return textResult(runRepo("folder", "move", source, target)), nil
}

func folderDeleteHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, _ := args["path"].(string)
	return textResult(runRepo("folder", "delete", path)), nil
}

func folderListHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, _ := args["path"].(string)
	if path == "" {
		path = "."
	}
	return textResult(runRepo("folder", "list", path)), nil
}

func folderTreeHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, _ := args["path"].(string)
	if path == "" {
		path = "."
	}
	return textResult(runRepo("folder", "tree", path)), nil
}

func fileCreateHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, _ := args["path"].(string)
	return textResult(runRepo("file", "create", path)), nil
}

func fileMoveHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	source, _ := args["source"].(string)
	target, _ := args["target"].(string)
	return textResult(runRepo("file", "move", source, target)), nil
}

func fileDeleteHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, _ := args["path"].(string)
	return textResult(runRepo("file", "delete", path)), nil
}

func fileListHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	scope, _ := args["scope"].(string)
	if scope == "" {
		scope = "@semio"
	}
	return textResult(runRepo("file", "list", scope)), nil
}

func fileTreeHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	path, _ := args["path"].(string)
	if path == "" {
		path = "."
	}
	return textResult(runRepo("file", "tree", path)), nil
}

func sectionCreateHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, _ := args["file"].(string)
	section, _ := args["section"].(string)
	return textResult(runRepo("section", "create", file, section)), nil
}

func sectionMoveHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, _ := args["file"].(string)
	oldName, _ := args["old_name"].(string)
	newName, _ := args["new_name"].(string)
	return textResult(runRepo("section", "move", file, oldName, newName)), nil
}

func sectionDeleteHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, _ := args["file"].(string)
	section, _ := args["section"].(string)
	return textResult(runRepo("section", "delete", file, section)), nil
}

func sectionListHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, _ := args["file"].(string)
	return textResult(runRepo("section", "list", file)), nil
}

func sectionTreeHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, _ := args["file"].(string)
	return textResult(runRepo("section", "tree", file)), nil
}

func definitionListHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	file, _ := args["file"].(string)
	return textResult(runRepo("definition", "list", file)), nil
}

func toolRunHandler(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	name, _ := args["name"].(string)
	return textResult(runRepo("tool", name)), nil
}

var _ = strings.TrimSpace

