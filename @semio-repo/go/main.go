// #region Header

// SPDX-License-Identifier: AGPL-3.0-or-later

// go/repo/repo.go

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
	"bufio"
	"context"
	"database/sql"
	"encoding/csv"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"io/fs"
	"io/ioutil"
	"math"
	"math/rand"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"sync"
	"text/template"
	"time"

	"github.com/Masterminds/sprig/v3"
	"github.com/bmatcuk/doublestar/v4"
	"github.com/graphql-go/graphql"
	"github.com/graphql-go/graphql/language/ast"
	"github.com/graphql-go/graphql/language/parser"
	"github.com/mark3labs/mcp-go/mcp"
	"github.com/mark3labs/mcp-go/server"
	"github.com/spf13/cobra"
	"gopkg.in/yaml.v3"
	_ "modernc.org/sqlite"
)

// #region Engine Events

type Kind string

const (
	KindStart    Kind = "start"
	KindLog      Kind = "log"
	KindProgress Kind = "progress"
	KindResult   Kind = "result"
	KindArtifact Kind = "artifact"
	KindError    Kind = "error"
	KindDone     Kind = "done"
)

type Event struct {
	Kind     Kind            `json:"kind"`
	Command  string          `json:"command,omitempty"`
	ID       string          `json:"id,omitempty"`
	Message  string          `json:"message,omitempty"`
	Level    string          `json:"level,omitempty"`
	Progress *Progress       `json:"progress,omitempty"`
	Data     json.RawMessage `json:"data,omitempty"`
	Artifact *Artifact       `json:"artifact,omitempty"`
	Error    *ErrPayload     `json:"error,omitempty"`
	Done     *DonePayload    `json:"done,omitempty"`
}

type Progress struct {
	Current int    `json:"current,omitempty"`
	Total   int    `json:"total,omitempty"`
	Percent int    `json:"percent,omitempty"`
	Step    string `json:"step,omitempty"`
}

type Artifact struct {
	Type string `json:"type"`
	URI  string `json:"uri"`
	Note string `json:"note,omitempty"`
}

type ErrPayload struct {
	Code    string `json:"code"`
	Message string `json:"message"`
	Detail  string `json:"detail,omitempty"`
	Fatal   bool   `json:"fatal,omitempty"`
}

type DonePayload struct {
	ExitCode int    `json:"exit_code"`
	Status   string `json:"status"`
}

// #endregion Engine Events

// #region Engine Errors

type ErrorCode string

const (
	ErrInternal ErrorCode = "E_INTERNAL"
	ErrParse    ErrorCode = "E_PARSE"
	ErrCanceled ErrorCode = "E_CANCELED"
	ErrNetwork  ErrorCode = "E_NETWORK"
	ErrAuth     ErrorCode = "E_AUTH"
)

// #endregion Engine Errors

// #region Engine Requests

type Command string

const (
	CmdGraphQL Command = "graphql"
	CmdAnalyze Command = "analyze"
	CmdFix     Command = "fix"
	CmdPolicy  Command = "policy"
	CmdTicket  Command = "ticket"
	CmdBundle  Command = "bundle"
	CmdFolder  Command = "folder"
	CmdFile    Command = "file"
	CmdSection Command = "section"
	CmdDef     Command = "definition"
)

type Request struct {
	Command  Command
	Args     json.RawMessage
	RepoRoot string
	Verbose  bool
}

type GraphQLArgs struct {
	Query     string         `json:"query"`
	Variables map[string]any `json:"variables,omitempty"`
}

// #endregion Engine Requests

// #region Engine

type GraphQLExecutor interface {
	Execute(ctx context.Context, query string, variables map[string]interface{}) (interface{}, error)
}

type Engine struct {
	GraphQL GraphQLExecutor
}

func NewEngine(graphql GraphQLExecutor) *Engine {
	return &Engine{GraphQL: graphql}
}

func (e *Engine) Run(ctx context.Context, req Request) <-chan Event {
	out := make(chan Event)
	go func() {
		defer func() {
			if recovered := recover(); recovered != nil {
				e.emitError(out, req, ErrPayload{Code: string(ErrInternal), Message: "internal error", Detail: fmt.Sprintf("%v", recovered), Fatal: true})
				e.emitDone(out, exitCodeError, "error")
			}
			close(out)
		}()

		e.emitStart(out, req)

		if ctx.Err() != nil {
			e.emitError(out, req, ErrPayload{Code: string(ErrCanceled), Message: ctx.Err().Error(), Fatal: true})
			e.emitDone(out, exitCodeCanceled, "canceled")
			return
		}

		switch req.Command {
		case CmdGraphQL, CmdAnalyze, CmdFix, CmdPolicy, CmdTicket, CmdBundle, CmdFolder, CmdFile, CmdSection, CmdDef:
			e.runGraphQL(ctx, req, out)
		default:
			e.emitError(out, req, ErrPayload{Code: string(ErrInternal), Message: "unsupported command", Fatal: true})
			e.emitDone(out, exitCodeError, "error")
		}
	}()
	return out
}

func (e *Engine) runGraphQL(ctx context.Context, req Request, out chan<- Event) {
	var args GraphQLArgs
	if err := json.Unmarshal(req.Args, &args); err != nil {
		e.emitError(out, req, ErrPayload{Code: string(ErrParse), Message: "invalid arguments", Detail: err.Error(), Fatal: true})
		e.emitDone(out, exitCodeUsage, "error")
		return
	}
	if e.GraphQL == nil {
		e.emitError(out, req, ErrPayload{Code: string(ErrInternal), Message: "graphql executor missing", Fatal: true})
		e.emitDone(out, exitCodeError, "error")
		return
	}
	result, err := e.GraphQL.Execute(ctx, args.Query, args.Variables)
	if err != nil {
		e.emitError(out, req, ErrPayload{Code: string(ErrInternal), Message: err.Error(), Fatal: true})
		e.emitDone(out, exitCodeError, "error")
		return
	}
	payload, err := json.Marshal(map[string]interface{}{"data": result})
	if err != nil {
		e.emitError(out, req, ErrPayload{Code: string(ErrInternal), Message: "failed to encode result", Detail: err.Error(), Fatal: true})
		e.emitDone(out, exitCodeError, "error")
		return
	}
	out <- Event{Kind: KindResult, Command: string(req.Command), Data: payload}
	e.emitDone(out, exitCodeOK, "ok")
}

func (e *Engine) emitStart(out chan<- Event, req Request) {
	out <- Event{Kind: KindStart, Command: string(req.Command)}
}

func (e *Engine) emitError(out chan<- Event, req Request, payload ErrPayload) {
	out <- Event{Kind: KindError, Command: string(req.Command), Error: &payload}
}

func (e *Engine) emitDone(out chan<- Event, code int, status string) {
	out <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: code, Status: status}}
}

const (
	exitCodeOK       = 0
	exitCodeError    = 1
	exitCodeUsage    = 2
	exitCodeCanceled = 130
)

// #endregion Engine

// #region Cli Adapter

type Config struct {
	JSON     bool
	Markdown bool
	Verbose  bool
	Repo     string
	Timeout  time.Duration
}

type EngineFactory func(Config) (*Engine, error)

type ExitError struct {
	Code int
}

func (e ExitError) Error() string {
	return fmt.Sprintf("exit status %d", e.Code)
}

func NewRoot(factory EngineFactory) *cobra.Command {
	root, _ := NewRootWithConfig(factory)
	return root
}

func NewRootWithConfig(factory EngineFactory) (*cobra.Command, *Config) {
	config := Config{}
	root := &cobra.Command{
		Use:   "repo",
		Short: "Monorepo CLI for Semio",
	}
	root.PersistentFlags().BoolVar(&config.JSON, "json", false, "Output NDJSON")
	root.PersistentFlags().BoolVar(&config.Markdown, "md", false, "Output Markdown")
	root.PersistentFlags().BoolVar(&config.Verbose, "verbose", false, "Verbose output")
	root.PersistentFlags().StringVar(&config.Repo, "repo", "", "Repo root path")
	root.PersistentFlags().DurationVar(&config.Timeout, "timeout", 0, "Timeout for command execution")
	root.AddCommand(mcpCommand(factory, &config))
	root.AddCommand(graphqlCommand(factory, &config))
	root.AddCommand(analyzeCommand(factory, &config))
	root.AddCommand(fixCommand(factory, &config))
	root.AddCommand(policyCommand(factory, &config))
	root.AddCommand(ticketCommand(factory, &config))
	root.AddCommand(todoCommand(factory, &config))
	root.AddCommand(goalCommand(factory, &config))
	root.AddCommand(projectCommand(factory, &config))
	root.AddCommand(contributorCommand(factory, &config))
	root.AddCommand(bundleCommand(factory, &config))
	root.AddCommand(folderCommand(factory, &config))
	root.AddCommand(fileCommand(factory, &config))
	root.AddCommand(sectionCommand(factory, &config))
	root.AddCommand(definitionCommand(factory, &config))
	root.AddCommand(treeCommand(factory, &config))
	root.AddCommand(exportCommand(factory, &config))
	root.AddCommand(benchmarkCmd)
	root.AddCommand(preflightCmd)
	root.AddCommand(updateCmd)
	return root, &config
}

func Execute(factory EngineFactory) error {
	return NewRoot(factory).Execute()
}

func defaultEngineFactory(config Config) (*Engine, error) {
	repoRoot := config.Repo
	if repoRoot == "" {
		cwd, err := os.Getwd()
		if err != nil {
			return nil, err
		}
		repoRoot = findRepoRoot(cwd)
	}
	SetRootDir(repoRoot)
	exec, err := NewExecutorWithContext(repoRoot, NewRepoContext(repoRoot))
	if err != nil {
		return nil, err
	}
	return NewEngine(exec), nil
}

func main() {
	if err := Execute(defaultEngineFactory); err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(1)
	}
}

func mcpCommand(factory EngineFactory, config *Config) *cobra.Command {
	var dryRun bool
	cmd := &cobra.Command{
		Use:   "mcp",
		Short: "Run MCP server",
		RunE: func(cmd *cobra.Command, args []string) error {
			if dryRun {
				return nil
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
			return serveMcp(ctx, engine)
		},
	}
	cmd.Flags().BoolVar(&dryRun, "dry-run", false, "Initialize and exit without starting server")
	return cmd
}

func serveMcp(ctx context.Context, engine *Engine) error {
	_ = ctx
	_ = engine
	return runMcpServer(nil, nil)
}

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
		Args:  cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			if scope == "" && len(args) > 0 {
				scope = args[0]
			}
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
		Args:  cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			if scope == "" && len(args) > 0 {
				scope = args[0]
			}
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

func treeCommand(factory EngineFactory, config *Config) *cobra.Command {
	var maxDepth int
	cmd := &cobra.Command{
		Use:   "tree [scope]",
		Short: "Show codebase tree",
		Args:  cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			scope := ""
			if len(args) > 0 {
				scope = args[0]
			}
			ctx := cmd.Context()

			folders := []Folder{}
			files := []File{}

			// Metadata collection
			type nodeMeta struct {
				status   string
				summary  string
				date     string
				author   string
				isGoal   bool
				isTicket bool
			}
			metaMap := make(map[string]nodeMeta)
			var metaMutex sync.Mutex

			folderCh := make(chan Folder)
			fileCh := make(chan File)
			errCh := make(chan error, 4) // Increased buffer for tickets/goals
			var wg sync.WaitGroup

			// Metadata streams
			wg.Add(1)
			go func() {
				defer wg.Done()
				// Only load metadata if scope covers repo root or .semio-repo
				// Simple heuristic: if scope is empty, ".", or contains ".semio-repo"
				
				// Using context for cancellation
				ticketCh := make(chan Ticket)
				go func() {
					StreamTickets(ctx, nil, nil, nil, ticketCh)
				}()

				for t := range ticketCh {
					metaMutex.Lock()
					author := ""
					if len(t.Iterations) > 0 {
						author = t.Iterations[0].Author.Name
					}
					date := t.Started.Format("2006-01-02")
					metaMap[filepath.Clean(t.FolderPath)] = nodeMeta{
						status:   string(t.Status),
						summary:  t.Summary,
						date:     date,
						author:   author,
						isTicket: true,
					}
					metaMutex.Unlock()
				}
			}()

			wg.Add(1)
			go func() {
				defer wg.Done()
				goalCh := make(chan *Goal)
				go func() {
					StreamGoals(ctx, goalCh)
				}()

				for g := range goalCh {
					metaMutex.Lock()
					date := g.Dates.Due
					if date == "" && g.Dates.Closed != nil {
						date = g.Dates.Closed.Format("2006-01-02")
					}
					metaMap[filepath.Clean(filepath.Join(GetRepoGoalsDir(), Slugify(g.Title)))] = nodeMeta{
						status:  g.Status,
						summary: g.Summary,
						date:    date,
						isGoal:  true,
					}
					metaMutex.Unlock()
				}
			}()

			wg.Add(1)
			go func() {
				defer wg.Done()
				if err := StreamFolders(ctx, scope, folderCh); err != nil {
					errCh <- err
				}
			}()

			wg.Add(1)
			go func() {
				defer wg.Done()
				if err := StreamFiles(ctx, scope, fileCh); err != nil {
					errCh <- err
				}
			}()

			var wgCollect sync.WaitGroup
			wgCollect.Add(2)

			go func() {
				defer wgCollect.Done()
				for f := range folderCh {
					folders = append(folders, f)
				}
			}()
			go func() {
				defer wgCollect.Done()
				for f := range fileCh {
					files = append(files, f)
				}
			}()

			wg.Wait()
			wgCollect.Wait()
			close(errCh)

			for err := range errCh {
				if err != nil {
					return err
				}
			}

			type node struct {
				name     string
				path     string
				children []*node
				isFile   bool
			}

			nodeMap := make(map[string]*node)

			// Resolve root path to absolute to match StreamFiles/StreamFolders output
			rootPath := scope
			rootDir := GetRootDir()
			if bundleName, found := strings.CutPrefix(scope, "@semio/"); found {
				bundles := LoadBundles()
				for _, b := range bundles {
					if b.Name == bundleName || normalizeBundleLabel(b.Name) == bundleName {
						rootPath = filepath.Join(rootDir, b.Root)
						break
					}
				}
			} else if scope != "" && scope != "@semio" {
				if filepath.IsAbs(scope) {
					rootPath = scope
				} else {
					rootPath = filepath.Join(rootDir, scope)
				}
			} else {
				rootPath = rootDir
			}
			rootPath = filepath.Clean(rootPath)

			root := &node{name: scope, path: rootPath, children: []*node{}} // Keep name as scope for display?
			// If scope is empty, display root dir name or "."?
			if scope == "" {
				root.name = filepath.Base(rootPath)
			}

			nodeMap[rootPath] = root

			for _, f := range folders {
				cleanPath := filepath.Clean(f.Path)
				if _, ok := nodeMap[cleanPath]; !ok {
					nodeMap[cleanPath] = &node{name: filepath.Base(cleanPath), path: cleanPath, children: []*node{}}
				}
			}
			for _, f := range files {
				cleanPath := filepath.Clean(f.Path)
				nodeMap[cleanPath] = &node{name: filepath.Base(cleanPath), path: cleanPath, children: []*node{}, isFile: true}
			}

			for path, n := range nodeMap {
				if n == root {
					continue
				}
				if path == rootPath || path == "." {
					continue
				}

				parentPath := filepath.Dir(path)
				if p, ok := nodeMap[parentPath]; ok {
					p.children = append(p.children, n)
				}
			}

			var sortChildren func(*node)
			sortChildren = func(n *node) {
				sort.Slice(n.children, func(i, j int) bool {
					return n.children[i].name < n.children[j].name
				})
				for _, c := range n.children {
					sortChildren(c)
				}
			}
			sortChildren(root)

			var walk func([]*node, string, int)
			walk = func(nodes []*node, prefix string, depth int) {
				if maxDepth > 0 && depth >= maxDepth {
					return
				}
				for i, node := range nodes {
					isLast := i == len(nodes)-1
					connector := "├── "
					if isLast {
						connector = "└── "
					}
					
					name := node.name
					metaMutex.Lock()
					meta, hasMeta := metaMap[node.path]
					metaMutex.Unlock()
					
					if hasMeta {
						isOpen := meta.status == "open"
						color := ColorGreen
						if isOpen {
							color = ColorBlue
						}
						
						name = colorize(name, color, true)
						
						icon := "✅"
						if isOpen {
							icon = "🔵"
						}
						
						metaStr := fmt.Sprintf(" %s %s", icon, meta.date)
						if meta.author != "" {
							metaStr += " " + meta.author
						}
						if meta.summary != "" {
							metaStr += " # " + meta.summary
						}
						name += colorize(metaStr, ColorDim, true)
					}

					fmt.Fprintf(cmd.OutOrStdout(), "%s%s%s\n", prefix, connector, name)

					newPrefix := prefix + "│   "
					if isLast {
						newPrefix = prefix + "    "
					}
					walk(node.children, newPrefix, depth+1)
				}
			}
			fmt.Fprintln(cmd.OutOrStdout(), root.path)
			walk(root.children, "", 0)

			return nil
		},
	}
	cmd.Flags().IntVarP(&maxDepth, "depth", "d", 0, "Max depth")
	return cmd
}

func exportCommand(factory EngineFactory, config *Config) *cobra.Command {
	return &cobra.Command{
		Use:   "export [output]",
		Short: "Export repo data to SQLite database",
		Long:  `Export all repo data (bundles, folders, files, sections, contributors, tickets, policies, violations) to a SQLite database file.`,
		Args:  cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			outputPath := ""
			if len(args) > 0 {
				outputPath = args[0]
			}
			repoRoot := config.Repo
			if repoRoot == "" {
				repoRoot = findRepoRoot(".")
			}
			ctx := NewRepoContext(repoRoot)
			result, err := ExportToSQLite(outputPath, ctx)
			if err != nil {
				return err
			}
			jsonBytes, err := json.MarshalIndent(result, "", "  ")
			if err != nil {
				return err
			}
			cmd.Println(string(jsonBytes))
			return nil
		},
	}
}

func policyCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "policy", Short: "Policy management commands"}
	listCmd := &cobra.Command{
		Use:   "list",
		Short: "List all registered policies",
		RunE: func(cmd *cobra.Command, args []string) error {
			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "policy list"}

				policyChan := make(chan PolicyDef)
				go func() {
					StreamPolicies(context.Background(), policyChan, opts)
				}()

				for p := range policyChan {
					data, err := json.Marshal(map[string]interface{}{"policy": p})
					if err != nil {
						continue
					}
					stream <- Event{Kind: KindResult, Command: "policy list", Data: data}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(listCmd)
	checkCmd := &cobra.Command{
		Use:   "check",
		Short: "Check a policy against a scope",
		Args:  cobra.MaximumNArgs(2),
		RunE: func(cmd *cobra.Command, args []string) error {
			policyID, err := cmd.Flags().GetString("id")
			if err != nil {
				return err
			}
			if policyID == "" && len(args) > 0 {
				policyID = args[0]
			}
			if policyID == "" {
				return fmt.Errorf("missing policy id")
			}
			scope, err := cmd.Flags().GetString("scope")
			if err != nil {
				return err
			}
			if scope == "" && len(args) > 1 {
				scope = args[1]
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

// extractLLMFromArgs extracts LLM from flags or positional args
func extractLLMFromArgs(cmd *cobra.Command, args []string) (string, []string) {
	// First check named --llm flag
	if llm, _ := cmd.Flags().GetString("llm"); llm != "" {
		return llm, args
	}
	// Then check boolean flags for each allowed LLM
	for _, allowed := range AllowedLLMs {
		flagName := allowed
		if val, _ := cmd.Flags().GetBool(flagName); val {
			return allowed, args
		}
	}
	// Then check positional args
	remaining := []string{}
	foundLLM := ""
	for _, arg := range args {
		if foundLLM != "" {
			remaining = append(remaining, arg)
			continue
		}
		normalized := NormalizeLLMSlug(arg)
		matched := false
		bestMatch := ""
		for _, allowed := range AllowedLLMs {
			if strings.Contains(normalized, NormalizeLLMSlug(allowed)) {
				if len(allowed) > len(bestMatch) {
					bestMatch = allowed
					matched = true
				}
			}
		}
		if matched {
			foundLLM = bestMatch
		}
		if !matched {
			remaining = append(remaining, arg)
		}
	}
	return foundLLM, remaining
}

// extractUIFromArgs extracts UI from flags or positional args
func extractUIFromArgs(cmd *cobra.Command, args []string) (string, []string) {
	// First check named --ui flag
	if ui, _ := cmd.Flags().GetString("client"); ui != "" {
		return ui, args
	}
	// Then check boolean flags for each allowed UI
	for _, allowed := range AllowedUIs {
		flagName := allowed
		if val, _ := cmd.Flags().GetBool(flagName); val {
			return allowed, args
		}
	}
	// Then check positional args
	remaining := []string{}
	foundUI := ""
	for _, arg := range args {
		if foundUI != "" {
			remaining = append(remaining, arg)
			continue
		}
		normalized := NormalizeUISlug(arg)
		matched := false
		bestMatch := ""
		for _, allowed := range AllowedUIs {
			if strings.Contains(normalized, NormalizeUISlug(allowed)) {
				if len(allowed) > len(bestMatch) {
					bestMatch = allowed
					matched = true
				}
			}
		}
		if matched {
			foundUI = bestMatch
		}
		if !matched {
			remaining = append(remaining, arg)
		}
	}
	return foundUI, remaining
}

// addLLMFlags adds boolean flags for each allowed LLM
func addLLMFlags(cmd *cobra.Command) {
	for _, llm := range AllowedLLMs {
		cmd.Flags().Bool(llm, false, fmt.Sprintf("Use %s as LLM", llm))
	}
}

// addUIFlags adds boolean flags for each allowed UI
func addUIFlags(cmd *cobra.Command) {
	for _, ui := range AllowedUIs {
		cmd.Flags().Bool(ui, false, fmt.Sprintf("Use %s as UI", ui))
	}
}

func draftCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "draft", Short: "Draft management commands"}
	createCmd := &cobra.Command{
		Use:   "create [slug]",
		Short: "Create a new draft",
		RunE: func(cmd *cobra.Command, args []string) error {
			if len(args) < 1 {
				return fmt.Errorf("missing slug")
			}
			slug := args[0]
			files, _ := cmd.Flags().GetStringSlice("files")

			draft, err := CreateDraft(slug, files)
			if err != nil {
				return err
			}
			data, _ := json.Marshal(map[string]interface{}{"draft": draft})
			// Print event directly since draft commands are local fs operations
			fmt.Printf("{\"kind\":\"result\",\"command\":\"draft create\",\"data\":%s}\n", string(data))
			fmt.Printf("{\"kind\":\"done\",\"done\":{\"status\":\"ok\",\"exitCode\":0}}\n")
			return nil
		},
	}
	createCmd.Flags().StringSlice("files", nil, "Files to include in the draft")

	deleteCmd := &cobra.Command{
		Use:   "delete [slug]",
		Short: "Delete a draft",
		RunE: func(cmd *cobra.Command, args []string) error {
			if len(args) < 1 {
				return fmt.Errorf("missing slug")
			}
			slug := args[0]
			if err := DeleteDraft(slug); err != nil {
				return err
			}
			fmt.Printf("{\"kind\":\"done\",\"done\":{\"status\":\"ok\",\"exitCode\":0}}\n")
			return nil
		},
	}

	listCmd := &cobra.Command{
		Use:   "list",
		Short: "List drafts",
		RunE: func(cmd *cobra.Command, args []string) error {
			drafts, err := ListDrafts()
			if err != nil {
				return err
			}
			for _, d := range drafts {
				data, _ := json.Marshal(map[string]interface{}{"draft": d})
				fmt.Printf("{\"kind\":\"result\",\"command\":\"draft list\",\"data\":%s}\n", string(data))
			}
			fmt.Printf("{\"kind\":\"done\",\"done\":{\"status\":\"ok\",\"exitCode\":0}}\n")
			return nil
		},
	}

	root.AddCommand(createCmd)
	root.AddCommand(deleteCmd)
	root.AddCommand(listCmd)
	return root
}

func todoCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "todo", Short: "Todo management commands"}
	createCmd := &cobra.Command{
		Use:   "create [parent-id] [name] [description]",
		Short: "Create a todo",
		Args:  cobra.MaximumNArgs(3),
		RunE: func(cmd *cobra.Command, args []string) error {
			parentID, _ := cmd.Flags().GetString("parent")
			name, _ := cmd.Flags().GetString("name")
			description, _ := cmd.Flags().GetString("description")
			if len(args) > 0 {
				parentID = args[0]
			}
			if len(args) > 1 {
				name = args[1]
			}
			if len(args) > 2 {
				description = args[2]
			}
			if parentID == "" || name == "" {
				return fmt.Errorf("missing parent-id or name")
			}
			variables := map[string]interface{}{
				"input": map[string]interface{}{
					"parentId":    parentID,
					"name":        name,
					"description": description,
				},
			}
			query := `mutation TodoCreate($input: TodoCreateInput!) { todoCreate(input: $input) { id name description } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	createCmd.Flags().String("parent", "", "Parent ID")
	createCmd.Flags().String("name", "", "Todo name")
	createCmd.Flags().String("description", "", "Todo description")

	changeCmd := &cobra.Command{
		Use:   "change [id] --name <new-name> --description <new-description>",
		Short: "Change a todo",
		Args:  cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			id, _ := cmd.Flags().GetString("id")
			if id == "" && len(args) > 0 {
				id = args[0]
			}
			name, _ := cmd.Flags().GetString("name")
			description, _ := cmd.Flags().GetString("description")
			if id == "" {
				return fmt.Errorf("missing id")
			}
			input := map[string]interface{}{
				"id": id,
			}
			if cmd.Flags().Changed("name") {
				input["name"] = name
			}
			if cmd.Flags().Changed("description") {
				input["description"] = description
			}
			variables := map[string]interface{}{"input": input}
			query := `mutation TodoChange($input: TodoUpdateInput!) { todoChange(input: $input) { id name description } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	changeCmd.Flags().String("id", "", "Todo ID")
	changeCmd.Flags().String("name", "", "New name")
	changeCmd.Flags().String("description", "", "New description")

	deleteCmd := &cobra.Command{
		Use:   "delete [id]",
		Short: "Delete a todo",
		Args:  cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			id, _ := cmd.Flags().GetString("id")
			if id == "" && len(args) > 0 {
				id = args[0]
			}
			if id == "" {
				return fmt.Errorf("missing id")
			}
			variables := map[string]interface{}{"id": id}
			query := `mutation TodoDelete($id: ID!) { todoDelete(id: $id) }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	deleteCmd.Flags().String("id", "", "Todo ID")

	listCmd := &cobra.Command{
		Use:   "list",
		Short: "List todos",
		RunE: func(cmd *cobra.Command, args []string) error {
			query := `query Todos { todos { id name description parent { id } location { file { path } line } } }`
			return runGraphQL(cmd, factory, config, query, nil)
		},
	}

	treeCmd := &cobra.Command{
		Use:   "tree",
		Short: "Show todo tree",
		RunE: func(cmd *cobra.Command, args []string) error {
			// Tree command typically requires specific handling to print tree structure, but for now we can rely on GraphQL output or implement custom renderer.
			// Given "repo todo tree", it might be like "repo folder tree".
			// For now, let's just query todos and maybe structure them?
			// The instruction says "The semio repo go binary should provide commands to ... tree ... todos".
			// "repo folder tree" uses `runGraphQL` but maybe specific query?
			// Let's use list for now and maybe the user wants hierarchy. But todos don't have hierarchy among themselves except parent is a resource.
			// So tree probably means resource tree with todos attached.
			query := `query Todos { todos { id name description parent { id } } }`
			// TODO: Implement actual tree rendering if needed.
			return runGraphQL(cmd, factory, config, query, nil)
		},
	}

	searchCmd := &cobra.Command{
		Use:   "search [search-string]",
		Short: "Search todos",
		Args:  cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			search := ""
			if len(args) > 0 {
				search = args[0]
			}
			variables := map[string]interface{}{"filter": map[string]interface{}{"filter": search}}
			query := `query Todos($filter: FilterInput) { todos(filter: $filter) { id name description } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}

	root.AddCommand(createCmd)
	root.AddCommand(changeCmd)
	root.AddCommand(deleteCmd)
	root.AddCommand(listCmd)
	root.AddCommand(treeCmd)
	root.AddCommand(searchCmd)
	return root
}

func ticketCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "ticket", Short: "Ticket management commands"}
	openCmd := &cobra.Command{
		Use:   "open [title] [prompt] [ui] [llm]",
		Short: "Open a new ticket",
		RunE: func(cmd *cobra.Command, args []string) error {
			title, _ := cmd.Flags().GetString("title")
			prompt, _ := cmd.Flags().GetString("prompt")
			noIssue, _ := cmd.Flags().GetBool("no-issue")
			draft, _ := cmd.Flags().GetString("draft")
			goal, _ := cmd.Flags().GetString("goal")
			parent, _ := cmd.Flags().GetString("parent")
			noGithub, _ := cmd.Flags().GetBool("no-github")
			issue, _ := cmd.Flags().GetString("issue")

			// Process positional args for title and prompt first
			remainingArgs := args
			if title == "" && len(remainingArgs) > 0 {
				title = remainingArgs[0]
				remainingArgs = remainingArgs[1:]
			}
			if prompt == "" && len(remainingArgs) > 0 {
				prompt = remainingArgs[0]
				remainingArgs = remainingArgs[1:]
			}

			// Extract UI and LLM from flags or remaining positional args
			ui, remainingArgs := extractUIFromArgs(cmd, remainingArgs)
			llm, _ := extractLLMFromArgs(cmd, remainingArgs)

			if title == "" {
				return fmt.Errorf("missing title")
			}
			if prompt == "" {
				prompt = title
			}
			if ui == "" {
				return fmt.Errorf("missing ui. Use --ui <value>, --<ui-name> flag, or positional arg. Allowed: %s", strings.Join(AllowedUIs, ", "))
			}
			input := map[string]interface{}{
				"title":    title,
				"prompt":   prompt,
				"client":   strings.ToUpper(strings.ReplaceAll(ui, "-", "_")),
				"noIssue":  noIssue,
				"noGithub": noGithub,
			}
			if llm != "" {
				input["llm"] = llm
			}
			if draft != "" {
				input["draft"] = draft
			}
			if goal != "" {
				input["goal"] = goal
			}
			if parent != "" {
				input["parent"] = parent
			}
			if issue != "" {
				input["issue"] = issue
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
	openCmd.Flags().String("client", "", "UI")
	openCmd.Flags().Bool("no-issue", false, "Skip GitHub issue")
	openCmd.Flags().String("draft", "", "Draft ID")
	openCmd.Flags().String("goal", "", "Goal ID")
	openCmd.Flags().Bool("no-github", false, "Skip GitHub operations")
	openCmd.Flags().String("parent", "", "Parent ticket slug")
	openCmd.Flags().String("issue", "", "Link to existing GitHub issue URL instead of creating new one")
	addLLMFlags(openCmd)
	addUIFlags(openCmd)
	listCmd := &cobra.Command{
		Use:   "list",
		Short: "List tickets",
		RunE: func(cmd *cobra.Command, args []string) error {
			yearVal, _ := cmd.Flags().GetInt("year")
			monthVal, _ := cmd.Flags().GetInt("month")
			dayVal, _ := cmd.Flags().GetInt("day")

			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "ticket list"}

				var year, month, day *int
				if yearVal != 0 {
					year = &yearVal
				}
				if monthVal != 0 {
					month = &monthVal
				}
				if dayVal != 0 {
					day = &dayVal
				}

				opts := getStreamOptions(cmd)
				ticketChan := make(chan Ticket)
				go func() {
					StreamTickets(context.Background(), year, month, day, ticketChan, opts)
				}()

				for t := range ticketChan {
					// Flatten for viewer expectation (Case 6b)
					flat := map[string]interface{}{
						"slug":   t.Slug,
						"year":   t.Year,
						"month":  t.Month,
						"day":    t.Day,
						"title":  t.Title,
						"status": t.Status,
					}

					created := fmt.Sprintf("%04d-%02d-%02dT00:00:00Z", t.Year, t.Month, t.Day)
					dates := map[string]interface{}{
						"created": created,
					}
					// Closed date if present
					if t.Finished != nil {
						dates["finished"] = t.Finished.Format(time.RFC3339)
					}
					flat["date"] = dates

					data, err := json.Marshal(map[string]interface{}{"ticket": flat})
					if err != nil {
						continue
					}
					stream <- Event{Kind: KindResult, Command: "ticket list", Data: data}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()

			return renderStream(cmd, config, stream)
		},
	}
	listCmd.Flags().Int("year", 0, "Filter by year")
	listCmd.Flags().Int("month", 0, "Filter by month")
	listCmd.Flags().Int("day", 0, "Filter by day")
	bindStreamFlags(listCmd)
	closeCmd := &cobra.Command{
		Use:   "close [path] [summary] [files...]",
		Short: "Close a ticket",
		RunE: func(cmd *cobra.Command, args []string) error {
			year, _ := cmd.Flags().GetInt("year")
			month, _ := cmd.Flags().GetInt("month")
			day, _ := cmd.Flags().GetInt("day")
			slug, _ := cmd.Flags().GetString("slug")
			summary, _ := cmd.Flags().GetString("summary")
			noGithub, _ := cmd.Flags().GetBool("no-github")
			files, _ := cmd.Flags().GetStringSlice("files")
			title, _ := cmd.Flags().GetString("title")
			closeAll, _ := cmd.Flags().GetBool("all")

			if !closeAll {
				if len(args) > 0 && (year == 0 || month == 0 || day == 0 || slug == "") {
					parts := strings.Split(args[0], "/")
					if len(parts) >= 4 {
						if y, err := strconv.Atoi(parts[0]); err == nil {
							year = y
						}
						if m, err := strconv.Atoi(parts[1]); err == nil {
							month = m
						}
						if d, err := strconv.Atoi(parts[2]); err == nil {
							day = d
						}
						slug = strings.Join(parts[3:], "/")
					}
				}

				if summary == "" && len(args) > 1 {
					summary = args[1]
				}
				if len(files) == 0 && len(args) > 2 {
					files = args[2:]
				}

				if year == 0 || month == 0 || day == 0 || slug == "" {
					return fmt.Errorf("missing ticket path (use YYYY/MM/DD/SLUG or flags)")
				}
				if summary == "" {
					return fmt.Errorf("missing summary")
				}
				if len(files) == 0 {
					return fmt.Errorf("missing files")
				}
			}

			input := map[string]interface{}{
				"noGithub": noGithub,
				"all":      closeAll,
			}
			if !closeAll {
				input["year"] = year
				input["month"] = month
				input["day"] = day
				input["slug"] = slug
				input["summary"] = summary
				input["files"] = files
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
					dates { started finished }
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	closeCmd.Flags().Bool("all", false, "Close all open tickets")
	closeCmd.Flags().Int("year", 0, "Ticket year")
	closeCmd.Flags().Int("month", 0, "Ticket month")
	closeCmd.Flags().Int("day", 0, "Ticket day")
	closeCmd.Flags().String("slug", "", "Ticket slug")
	closeCmd.Flags().Bool("no-github", false, "Skip GitHub operations")
	closeCmd.Flags().String("summary", "", "Summary")
	closeCmd.Flags().StringSlice("files", nil, "Files")
	closeCmd.Flags().String("title", "", "Title")
	reopenCmd := &cobra.Command{
		Use:   "reopen [path] [prompt] [ui] [llm]",
		Short: "Reopen a ticket",
		RunE: func(cmd *cobra.Command, args []string) error {
			year, _ := cmd.Flags().GetInt("year")
			month, _ := cmd.Flags().GetInt("month")
			day, _ := cmd.Flags().GetInt("day")
			noGithub, _ := cmd.Flags().GetBool("no-github")
			slug, _ := cmd.Flags().GetString("slug")
			prompt, _ := cmd.Flags().GetString("prompt")
			title, _ := cmd.Flags().GetString("title")
			draft, _ := cmd.Flags().GetString("draft")
			goal, _ := cmd.Flags().GetString("goal")
			parent, _ := cmd.Flags().GetString("parent")

			remainingArgs := args
			if len(remainingArgs) > 0 && (year == 0 || month == 0 || day == 0 || slug == "") {
				parts := strings.Split(remainingArgs[0], "/")
				if len(parts) >= 4 {
					if y, err := strconv.Atoi(parts[0]); err == nil {
						year = y
					}
					if m, err := strconv.Atoi(parts[1]); err == nil {
						month = m
					}
					if d, err := strconv.Atoi(parts[2]); err == nil {
						day = d
					}
					slug = strings.Join(parts[3:], "/")
					remainingArgs = remainingArgs[1:]
				}
			}

			if prompt == "" && len(remainingArgs) > 0 {
				prompt = remainingArgs[0]
				remainingArgs = remainingArgs[1:]
			}

			// Extract UI and LLM from flags or remaining positional args
			ui, remainingArgs := extractUIFromArgs(cmd, remainingArgs)
			llm, _ := extractLLMFromArgs(cmd, remainingArgs)

			if year == 0 || month == 0 || day == 0 || slug == "" {
				return fmt.Errorf("missing ticket path")
			}
			if prompt == "" {
				return fmt.Errorf("missing prompt")
			}
			if ui == "" {
				return fmt.Errorf("missing ui. Use --ui <value>, --<ui-name> flag, or positional arg. Allowed: %s", strings.Join(AllowedUIs, ", "))
			}
			input := map[string]interface{}{
				"year":     year,
				"month":    month,
				"noGithub": noGithub,
				"day":      day,
				"slug":     slug,
				"prompt":   prompt,
				"client":   strings.ToUpper(strings.ReplaceAll(ui, "-", "_")),
			}
			if llm != "" {
				input["llm"] = llm
			}
			if title != "" {
				input["title"] = title
			}
			if draft != "" {
				input["draft"] = draft
			}
			if goal != "" {
				input["goal"] = goal
			}
			if parent != "" {
				input["parent"] = parent
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
	reopenCmd.Flags().Bool("no-github", false, "Skip GitHub operations")
	reopenCmd.Flags().Int("year", 0, "Ticket year")
	reopenCmd.Flags().Int("month", 0, "Ticket month")
	reopenCmd.Flags().Int("day", 0, "Ticket day")
	reopenCmd.Flags().String("slug", "", "Ticket slug")
	reopenCmd.Flags().String("prompt", "", "Prompt")
	reopenCmd.Flags().String("llm", "", "LLM")
	reopenCmd.Flags().String("client", "", "UI")
	reopenCmd.Flags().String("title", "", "Title")
	reopenCmd.Flags().String("draft", "", "Draft ID")
	reopenCmd.Flags().String("goal", "", "Goal ID")
	reopenCmd.Flags().String("parent", "", "Parent ticket slug")
	addLLMFlags(reopenCmd)
	addUIFlags(reopenCmd)

	treeCmd := &cobra.Command{
		Use:   "tree",
		Short: "Show ticket tree",
		RunE: func(cmd *cobra.Command, args []string) error {
			yearVal, _ := cmd.Flags().GetInt("year")
			monthVal, _ := cmd.Flags().GetInt("month")
			dayVal, _ := cmd.Flags().GetInt("day")

			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "ticket tree"}

				var year, month, day *int
				if yearVal != 0 {
					year = &yearVal
				}
				if monthVal != 0 {
					month = &monthVal
				}
				if dayVal != 0 {
					day = &dayVal
				}

				opts := getStreamOptions(cmd)
				ticketChan := make(chan Ticket)
				go func() {
					StreamTickets(context.Background(), year, month, day, ticketChan, opts)
				}()

				var tickets []Ticket
				for t := range ticketChan {
					tickets = append(tickets, t)
				}

				type node struct {
					name     string
					children map[string]*node
					isTicket bool
					ticket   *Ticket
				}
				root := &node{children: make(map[string]*node)}

				for _, t := range tickets {
					path := fmt.Sprintf("%d/%02d/%02d/%s", t.Year, t.Month, t.Day, t.Slug)
					parts := strings.Split(path, "/")
					curr := root
					for i, part := range parts {
						if _, ok := curr.children[part]; !ok {
							curr.children[part] = &node{name: part, children: make(map[string]*node)}
						}
						curr = curr.children[part]
						if i == len(parts)-1 {
							curr.isTicket = true
							tCopy := t
							curr.ticket = &tCopy
						}
					}
				}

				var printNode func(*node, string)
				printNode = func(n *node, prefix string) {
					keys := make([]string, 0, len(n.children))
					for k := range n.children {
						keys = append(keys, k)
					}
					sort.Strings(keys)

					for i, k := range keys {
						child := n.children[k]
						isLast := i == len(keys)-1
						connector := "├── "
						if isLast {
							connector = "└── "
						}
						childPrefix := prefix + "│   "
						if isLast {
							childPrefix = prefix + "    "
						}

						display := child.name
						if child.isTicket && child.ticket != nil {
							// Optional: add title?
							// display += " (" + child.ticket.Title + ")"
						}
						if !child.isTicket {
							display += "/"
						}

						stream <- Event{Kind: KindLog, Level: "info", Command: "ticket tree", Message: prefix + connector + display}
						printNode(child, childPrefix)
					}
				}

				printNode(root, "")
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	treeCmd.Flags().Int("year", 0, "Filter by year")
	treeCmd.Flags().Int("month", 0, "Filter by month")
	treeCmd.Flags().Int("day", 0, "Filter by day")
	bindStreamFlags(treeCmd)

	changeCmd := &cobra.Command{
		Use:   "change <path>",
		Short: "Change a ticket",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			path := args[0]
			parts := strings.Split(path, "/")
			if len(parts) < 4 {
				return fmt.Errorf("invalid ticket path format: expected YYYY/MM/DD/SLUG")
			}
			y, err := strconv.Atoi(parts[0])
			if err != nil {
				return fmt.Errorf("invalid year")
			}
			m, err := strconv.Atoi(parts[1])
			if err != nil {
				return fmt.Errorf("invalid month")
			}
			d, err := strconv.Atoi(parts[2])
			if err != nil {
				return fmt.Errorf("invalid day")
			}
			slug := strings.Join(parts[3:], "/")

			title, _ := cmd.Flags().GetString("title")
			prompt, _ := cmd.Flags().GetString("prompt")
			goal, _ := cmd.Flags().GetString("goal")
			parent, _ := cmd.Flags().GetString("parent")
			noGithub, _ := cmd.Flags().GetBool("no-github")

			ui, _ := extractUIFromArgs(cmd, []string{})
			llm, _ := extractLLMFromArgs(cmd, []string{})

			input := map[string]interface{}{
				"year":     y,
				"month":    m,
				"day":      d,
				"slug":     slug,
				"noGithub": noGithub,
			}
			if cmd.Flags().Changed("title") {
				input["title"] = title
			}
			if cmd.Flags().Changed("prompt") {
				input["prompt"] = prompt
			}
			if cmd.Flags().Changed("goal") {
				input["goal"] = goal
			}
			if cmd.Flags().Changed("parent") {
				input["parent"] = parent
			}
			if ui != "" {
				input["client"] = strings.ToUpper(strings.ReplaceAll(ui, "-", "_"))
			}
			if llm != "" {
				input["llm"] = llm
			}

			variables := map[string]interface{}{"input": input}
			query := `mutation TicketChange($input: TicketChangeInput!) {
				ticketChange(input: $input) {
					id
					slug
					status
					parent
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	changeCmd.Flags().String("parent", "", "New parent ticket slug")
	changeCmd.Flags().String("title", "", "New title")
	changeCmd.Flags().String("prompt", "", "New prompt")
	changeCmd.Flags().String("goal", "", "New goal ID")
	changeCmd.Flags().Bool("no-github", false, "Skip GitHub sync")
	addLLMFlags(changeCmd)
	addUIFlags(changeCmd)

	root.AddCommand(changeCmd)
	root.AddCommand(openCmd)
	root.AddCommand(listCmd)
	root.AddCommand(closeCmd)
	root.AddCommand(reopenCmd)
	root.AddCommand(treeCmd)

	return root
}

func goalCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "goal", Short: "Goal management commands"}
	listCmd := &cobra.Command{
		Use:   "list",
		Short: "List goals",
		RunE: func(cmd *cobra.Command, args []string) error {
			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "goal list"}

				goalChan := make(chan *Goal)
				go func() {
					StreamGoals(context.Background(), goalChan, opts)
				}()

				for g := range goalChan {
					data, err := json.Marshal(map[string]interface{}{"goal": g, "id": g.ID})
					if err != nil {
						continue
					}
					stream <- Event{Kind: KindResult, Command: "goal list", Data: data}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(listCmd)
	changeCmd := &cobra.Command{
		Use:   "change <SLUG>",
		Short: "Change a goal",
		Args:  cobra.ExactArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			id := args[0]
			title, _ := cmd.Flags().GetString("title")
			description, _ := cmd.Flags().GetString("description")
			dueDate, _ := cmd.Flags().GetString("due-date")
			parent, _ := cmd.Flags().GetString("parent")
			noGithub, _ := cmd.Flags().GetBool("no-github")

			input := map[string]interface{}{
				"id":       id,
				"noGithub": noGithub,
			}
			if cmd.Flags().Changed("title") {
				input["title"] = title
			}
			if cmd.Flags().Changed("description") {
				input["description"] = description
			}
			if cmd.Flags().Changed("due-date") {
				input["dueDate"] = dueDate
			}
			if cmd.Flags().Changed("parent") {
				input["parent"] = parent
			}

			variables := map[string]interface{}{"id": id, "input": input}
			query := `mutation GoalChange($id: ID!, $input: GoalChangeInput!) {
				goalChange(id: $id, input: $input) {
					id
					title
					description
					dueDate
					parent
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	changeCmd.Flags().String("title", "", "New title")
	changeCmd.Flags().String("description", "", "New description")
	changeCmd.Flags().String("due-date", "", "New due date (YYYY-MM-DD)")
	changeCmd.Flags().String("parent", "", "New parent goal ID")
	changeCmd.Flags().Bool("no-github", false, "Skip GitHub sync")

	openCmd := &cobra.Command{
		Use:   "open [title] [description] [prompt] [ui] [llm]",
		Short: "Open a new goal",
		RunE: func(cmd *cobra.Command, args []string) error {
			title, _ := cmd.Flags().GetString("title")
			description, _ := cmd.Flags().GetString("description")
			prompt, _ := cmd.Flags().GetString("prompt")
			dueDate, _ := cmd.Flags().GetString("due-date")
			noGithub, _ := cmd.Flags().GetBool("no-github")

			// Process positional args
			remainingArgs := args
			if title == "" && len(remainingArgs) > 0 {
				title = remainingArgs[0]
				remainingArgs = remainingArgs[1:]
			}
			if description == "" && len(remainingArgs) > 0 {
				description = remainingArgs[0]
				remainingArgs = remainingArgs[1:]
			}
			if prompt == "" && len(remainingArgs) > 0 {
				prompt = remainingArgs[0]
				remainingArgs = remainingArgs[1:]
			}

			// Extract UI and LLM from flags or remaining positional args
			ui, remainingArgs := extractUIFromArgs(cmd, remainingArgs)
			llm, _ := extractLLMFromArgs(cmd, remainingArgs)

			if title == "" {
				return fmt.Errorf("missing title")
			}
			if description == "" {
				return fmt.Errorf("missing description")
			}
			if prompt == "" {
				return fmt.Errorf("missing prompt")
			}
			if dueDate == "" {
				return fmt.Errorf("missing due-date")
			}
			if ui == "" {
				return fmt.Errorf("missing ui. Use --ui <value>, --<ui-name> flag, or positional arg. Allowed: %s", strings.Join(AllowedUIs, ", "))
			}
			if llm == "" {
				return fmt.Errorf("missing llm. Use --llm <value>, --<llm-name> flag, or positional arg. Allowed: %s", strings.Join(AllowedLLMs, ", "))
			}

			input := map[string]interface{}{
				"title":       title,
				"description": description,
				"prompt":      prompt,
				"dueDate":     dueDate,
				"llm":         llm,
				"client":      ui,
				"noGithub":    noGithub,
			}
			parent, _ := cmd.Flags().GetString("parent")
			if parent != "" {
				input["parent"] = parent
			}
			milestone, _ := cmd.Flags().GetString("milestone")
			if milestone != "" {
				input["milestone"] = milestone
			}
			variables := map[string]interface{}{"input": input}
			query := `mutation GoalCreate($input: GoalCreateInput!) {
				goalCreate(input: $input) {
					id
					title
					status
					prompt
					dueDate
					ui
					llm
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	openCmd.Flags().String("title", "", "Goal title")
	openCmd.Flags().String("description", "", "Goal description")
	openCmd.Flags().String("prompt", "", "Goal prompt")
	openCmd.Flags().String("due-date", "", "Goal due date (e.g., 2026-02-15)")
	openCmd.Flags().String("llm", "", "LLM")
	openCmd.Flags().String("client", "", "UI")
	openCmd.Flags().Bool("no-github", false, "Skip GitHub synchronization")
	openCmd.Flags().String("parent", "", "Parent goal ID")
	openCmd.Flags().String("milestone", "", "Link to existing GitHub milestone URL instead of creating new one")
	addLLMFlags(openCmd)
	addUIFlags(openCmd)

	closeCmd := &cobra.Command{
		Use:   "close [id] [summary]",
		Short: "Close a goal",
		RunE: func(cmd *cobra.Command, args []string) error {
			id := ""
			summary := ""
			if len(args) > 0 {
				id = args[0]
			}
			if len(args) > 1 {
				summary = args[1]
			}
			if id == "" {
				return fmt.Errorf("missing goal id")
			}
			if summary == "" {
				return fmt.Errorf("missing summary")
			}
			noGithub, _ := cmd.Flags().GetBool("no-github")
			input := map[string]interface{}{
				"id":       id,
				"summary":  summary,
				"noGithub": noGithub,
			}
			variables := map[string]interface{}{"input": input}
			query := `mutation GoalClose($input: GoalCloseInput!) {
				goalClose(input: $input) {
					id
					status
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	closeCmd.Flags().Bool("no-github", false, "Skip GitHub synchronization")

	reopenCmd := &cobra.Command{
		Use:   "reopen [id] [prompt] [ui] [llm]",
		Short: "Reopen a goal",
		RunE: func(cmd *cobra.Command, args []string) error {
			id := ""
			prompt, _ := cmd.Flags().GetString("prompt")
			title, _ := cmd.Flags().GetString("title")
			description, _ := cmd.Flags().GetString("description")
			dueDate, _ := cmd.Flags().GetString("due-date")
			parent, _ := cmd.Flags().GetString("parent")
			noGithub, _ := cmd.Flags().GetBool("no-github")

			if len(args) > 0 {
				id = args[0]
			}
			remainingArgs := args
			if len(remainingArgs) > 0 {
				remainingArgs = remainingArgs[1:]
			}

			if prompt == "" && len(remainingArgs) > 0 {
				prompt = remainingArgs[0]
				remainingArgs = remainingArgs[1:]
			}

			ui, remainingArgs := extractUIFromArgs(cmd, remainingArgs)
			llm, _ := extractLLMFromArgs(cmd, remainingArgs)

			if id == "" {
				return fmt.Errorf("missing goal id")
			}
			if prompt == "" {
				return fmt.Errorf("missing prompt")
			}
			if ui == "" {
				return fmt.Errorf("missing ui. Use --ui <value>, --<ui-name> flag, or positional arg. Allowed: %s", strings.Join(AllowedUIs, ", "))
			}
			if llm == "" {
				return fmt.Errorf("missing llm. Use --llm <value>, --<llm-name> flag, or positional arg. Allowed: %s", strings.Join(AllowedLLMs, ", "))
			}

			input := map[string]interface{}{
				"id":       id,
				"prompt":   prompt,
				"client":   ui,
				"llm":      llm,
				"noGithub": noGithub,
			}
			if title != "" {
				input["title"] = title
			}
			if description != "" {
				input["description"] = description
			}
			if dueDate != "" {
				input["dueDate"] = dueDate
			}
			if parent != "" {
				input["parent"] = parent
			}

			variables := map[string]interface{}{"input": input}
			query := `mutation GoalReopen($input: GoalReopenInput!) {
				goalReopen(input: $input) {
					id
					status
				}
			}`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	reopenCmd.Flags().Bool("no-github", false, "Skip GitHub synchronization")
	reopenCmd.Flags().String("prompt", "", "Prompt")
	reopenCmd.Flags().String("title", "", "New title")
	reopenCmd.Flags().String("description", "", "New description")
	reopenCmd.Flags().String("due-date", "", "New due date")
	reopenCmd.Flags().String("parent", "", "New parent goal")
	addLLMFlags(reopenCmd)
	addUIFlags(reopenCmd)

	treeCmd := &cobra.Command{
		Use:   "tree",
		Short: "Show goal and ticket tree",
		RunE: func(cmd *cobra.Command, args []string) error {
			query := `query GoalTree {
				repo {
					goals {
						id
						title
						status
						dueDate
						createdAt
						parent
					}
					tickets {
						id
						slug
						status
						goal
						parent
					}
				}
			}`
			return runGraphQL(cmd, factory, config, query, nil)
		},
	}

	root.AddCommand(listCmd)
	root.AddCommand(changeCmd)
	root.AddCommand(openCmd)
	root.AddCommand(closeCmd)
	root.AddCommand(reopenCmd)
	root.AddCommand(treeCmd)
	return root
}

func contributorCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "contributor", Short: "Contributor management commands"}
	listCmd := &cobra.Command{
		Use:   "list",
		Short: "List contributors",
		RunE: func(cmd *cobra.Command, args []string) error {
			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "contributor list"}

				contributorChan := make(chan Contributor)
				go func() {
					StreamContributors(context.Background(), contributorChan, opts)
				}()

				for c := range contributorChan {
					data, err := json.Marshal(map[string]interface{}{"contributor": c})
					if err != nil {
						continue
					}
					stream <- Event{Kind: KindResult, Command: "contributor list", Data: data}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(listCmd)
	addCmd := &cobra.Command{
		Use:   "add",
		Short: "Add a contributor",
		Args:  cobra.MaximumNArgs(3),
		RunE: func(cmd *cobra.Command, args []string) error {
			github, _ := cmd.Flags().GetString("github")
			name, _ := cmd.Flags().GetString("name")
			emails, _ := cmd.Flags().GetStringSlice("email")
			if github == "" && len(args) > 0 {
				github = args[0]
			}
			if name == "" && len(args) > 1 {
				name = args[1]
			}
			if len(emails) == 0 && len(args) > 2 {
				emails = args[2:]
			}
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
				contributorAdd(input: $input) { id github name emails }
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
		Args:  cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			github, _ := cmd.Flags().GetString("github")
			if github == "" && len(args) > 0 {
				github = args[0]
			}
			if github == "" {
				return fmt.Errorf("missing github")
			}
			variables := map[string]interface{}{"github": github}
			query := `mutation ContributorRemove($github: String!) { contributorRemove(github: $github) }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	removeCmd.Flags().String("github", "", "GitHub username")
	root.AddCommand(listCmd)
	root.AddCommand(addCmd)
	root.AddCommand(removeCmd)
	return root
}

func projectCommand(factory EngineFactory, config *Config) *cobra.Command {
	cmd := &cobra.Command{
		Use:   "project",
		Short: "Manage projects",
	}

	listCmd := &cobra.Command{
		Use:   "list",
		Short: "List projects",
		RunE: func(cmd *cobra.Command, args []string) error {
			return runProjectList(factory, *config, cmd, args)
		},
	}
	bindStreamFlags(listCmd)
	cmd.AddCommand(listCmd)

	treeCmd := &cobra.Command{
		Use:   "tree",
		Short: "Show project tree Structure",
		RunE: func(cmd *cobra.Command, args []string) error {
			return runProjectTree(factory, *config, cmd, args)
		},
	}
	bindStreamFlags(treeCmd)
	cmd.AddCommand(treeCmd)

	return cmd
}

func bundleCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "bundle", Short: "Bundle management commands"}
	listCmd := &cobra.Command{
		Use:   "list",
		Short: "List bundles",
		RunE: func(cmd *cobra.Command, args []string) error {
			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "bundle list"}

				bundleChan := make(chan Bundle)
				go func() {
					StreamBundles(context.Background(), bundleChan, opts)
				}()

				for b := range bundleChan {
					// Bundle struct tags match Case 6c expectations when wrapped in "bundle"
					data, err := json.Marshal(map[string]interface{}{"bundle": b})
					if err != nil {
						continue
					}
					stream <- Event{Kind: KindResult, Command: "bundle list", Data: data}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()

			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(listCmd)

	treeCmd := &cobra.Command{
		Use:   "tree",
		Short: "Show bundle tree",
		RunE: func(cmd *cobra.Command, args []string) error {
			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "bundle tree"}

				bundleChan := make(chan Bundle)
				go func() {
					StreamBundles(context.Background(), bundleChan, opts)
				}()

				var bundles []Bundle
				for b := range bundleChan {
					bundles = append(bundles, b)
				}

				sort.Slice(bundles, func(i, j int) bool {
					return bundles[i].Name < bundles[j].Name
				})

				if config.Markdown {
					var sb strings.Builder
					for _, b := range bundles {
						// Template: - [<id>](#<id>): <title> - <description>
						// For bundle: id=Name, title=Name, description=""
						sb.WriteString(fmt.Sprintf("- [%s](#%s): %s\n", b.Name, b.Name, b.Name))
					}
					data, _ := json.Marshal(map[string]string{"markdown": sb.String()})
					stream <- Event{Kind: KindResult, Command: "bundle tree", Data: data}
				} else {
					stream <- Event{Kind: KindLog, Level: "info", Command: "bundle tree", Message: "."}
					for i, b := range bundles {
						connector := "├── "
						if i == len(bundles)-1 {
							connector = "└── "
						}
						stream <- Event{Kind: KindLog, Level: "info", Command: "bundle tree", Message: connector + b.Name}
					}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(treeCmd)

	root.AddCommand(listCmd)
	root.AddCommand(treeCmd)
	return root
}

func folderCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "folder", Short: "Folder management commands"}
	createCmd := &cobra.Command{
		Use:   "create",
		Short: "Create a folder",
		Args:  cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" && len(args) > 0 {
				path = args[0]
			}
			if path == "" {
				return fmt.Errorf("missing path")
			}
			variables := map[string]interface{}{"path": path}
			query := `mutation FolderCreate($path: String!) { folderCreate(path: $path) { id path name uri } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	createCmd.Flags().String("path", "", "Folder path")
	moveCmd := &cobra.Command{
		Use:   "move",
		Short: "Move a folder",
		Args:  cobra.MaximumNArgs(2),
		RunE: func(cmd *cobra.Command, args []string) error {
			src, _ := cmd.Flags().GetString("source")
			dst, _ := cmd.Flags().GetString("target")
			if (src == "" || dst == "") && len(args) > 0 {
				if src == "" && len(args) > 0 {
					src = args[0]
				}
				if dst == "" && len(args) > 1 {
					dst = args[1]
				}
			}
			if src == "" || dst == "" {
				return fmt.Errorf("missing source or target")
			}
			variables := map[string]interface{}{"src": src, "dst": dst}
			query := `mutation FolderMove($src: String!, $dst: String!) { folderMove(src: $src, dst: $dst) { id path name uri } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	moveCmd.Flags().String("source", "", "Source path")
	moveCmd.Flags().String("target", "", "Target path")
	deleteCmd := &cobra.Command{
		Use:   "delete",
		Short: "Delete a folder",
		Args:  cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" && len(args) > 0 {
				path = args[0]
			}
			if path == "" {
				return fmt.Errorf("missing path")
			}
			variables := map[string]interface{}{"path": path}
			query := `mutation FolderDelete($path: String!) { folderDelete(path: $path) }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	deleteCmd.Flags().String("path", "", "Folder path")

	listCmd := &cobra.Command{
		Use:   "list [scope]",
		Short: "List folders",
		RunE: func(cmd *cobra.Command, args []string) error {
			scope := ""
			if len(args) > 0 {
				scope = args[0]
			}
			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "folder list"}

				folderChan := make(chan Folder)
				go func() {
					StreamFolders(context.Background(), scope, folderChan, opts)
				}()

				for f := range folderChan {
					data, err := json.Marshal(map[string]interface{}{"folder": f})
					if err != nil {
						continue
					}
					stream <- Event{Kind: KindResult, Command: "folder list", Data: data}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(listCmd)

	treeCmd := &cobra.Command{
		Use:   "tree [scope]",
		Short: "Show folder tree",
		RunE: func(cmd *cobra.Command, args []string) error {
			scope := ""
			if len(args) > 0 {
				scope = args[0]
			}
			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "folder tree"}

				folderChan := make(chan Folder)
				go func() {
					StreamFolders(context.Background(), scope, folderChan, opts)
				}()

				var folders []Folder
				for f := range folderChan {
					folders = append(folders, f)
				}

				type node struct {
					name     string
					children map[string]*node
					isFolder bool
				}
				root := &node{children: make(map[string]*node)}

				for _, f := range folders {
					parts := strings.Split(f.Path, "/")
					curr := root
					for i, part := range parts {
						if _, ok := curr.children[part]; !ok {
							curr.children[part] = &node{name: part, children: make(map[string]*node)}
						}
						curr = curr.children[part]
						if i == len(parts)-1 {
							curr.isFolder = true
						}
					}
				}

				if config.Markdown {
					var sb strings.Builder
					var printMarkdown func(*node, int, string)
					printMarkdown = func(n *node, depth int, pathPrefix string) {
						keys := make([]string, 0, len(n.children))
						for k := range n.children {
							keys = append(keys, k)
						}
						sort.Strings(keys)

						for _, k := range keys {
							child := n.children[k]
							currentPath := k
							if pathPrefix != "" {
								currentPath = pathPrefix + "/" + k
							}

							indent := strings.Repeat("  ", depth)
							// - [<id>](#<id>): <title> - <description>
							sb.WriteString(fmt.Sprintf("%s- [%s](#%s): %s\n", indent, currentPath, currentPath, child.name))
							printMarkdown(child, depth+1, currentPath)
						}
					}
					printMarkdown(root, 0, "")
					data, _ := json.Marshal(map[string]string{"markdown": sb.String()})
					stream <- Event{Kind: KindResult, Command: "folder tree", Data: data}
				} else {
					var printNode func(*node, string)
					printNode = func(n *node, prefix string) {
						keys := make([]string, 0, len(n.children))
						for k := range n.children {
							keys = append(keys, k)
						}
						sort.Strings(keys)

						for i, k := range keys {
							child := n.children[k]
							isLast := i == len(keys)-1
							connector := "├── "
							if isLast {
								connector = "└── "
							}
							childPrefix := prefix + "│   "
							if isLast {
								childPrefix = prefix + "    "
							}

							display := child.name + "/"
							stream <- Event{Kind: KindLog, Level: "info", Command: "folder tree", Message: prefix + connector + display}
							printNode(child, childPrefix)
						}
					}

					printNode(root, "")
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(treeCmd)

	root.AddCommand(createCmd)
	root.AddCommand(moveCmd)
	root.AddCommand(deleteCmd)
	root.AddCommand(listCmd)
	root.AddCommand(treeCmd)
	return root
}

func bindStreamFlags(cmd *cobra.Command) {
	cmd.Flags().Bool("show-ignored", false, "Show ignored folders and files")
	cmd.Flags().Bool("show-generated", false, "Show generated folders and files")
	cmd.Flags().Bool("no-code", false, "Exclude code files")
	cmd.Flags().Bool("no-script", false, "Exclude script files")
	cmd.Flags().Bool("no-config", false, "Exclude config files")
	cmd.Flags().Bool("no-test", false, "Exclude test files")
	cmd.Flags().Bool("no-docs", false, "Exclude docs files")
	cmd.Flags().Bool("no-resource", false, "Exclude resource files")
	cmd.Flags().Bool("no-license", false, "Exclude license files")

	cmd.Flags().Bool("only-code", false, "Only show code files")
	cmd.Flags().Bool("only-script", false, "Only show script files")
	cmd.Flags().Bool("only-config", false, "Only show config files")
	cmd.Flags().Bool("only-test", false, "Only show test files")
	cmd.Flags().Bool("only-docs", false, "Only show docs files")
	cmd.Flags().Bool("only-resource", false, "Only show resource files")
	cmd.Flags().Bool("only-license", false, "Only show license files")

	cmd.Flags().Bool("no-library", false, "Exclude library bundles")
	cmd.Flags().Bool("no-schema", false, "Exclude schema bundles")
	cmd.Flags().Bool("no-binary", false, "Exclude binary bundles")
	cmd.Flags().Bool("no-ui", false, "Exclude UI bundles")
	cmd.Flags().Bool("no-site", false, "Exclude site bundles")
	cmd.Flags().Bool("no-assets", false, "Exclude asset bundles")

	cmd.Flags().Bool("only-library", false, "Only show library bundles")
	cmd.Flags().Bool("only-schema", false, "Only show schema bundles")
	cmd.Flags().Bool("only-binary", false, "Only show binary bundles")
	cmd.Flags().Bool("only-ui", false, "Only show UI bundles")
	cmd.Flags().Bool("only-site", false, "Only show site bundles")
	cmd.Flags().Bool("only-assets", false, "Only show asset bundles")

	cmd.Flags().Bool("no-organization", false, "Exclude organization folders")
	cmd.Flags().Bool("no-required", false, "Exclude required folders")
	cmd.Flags().Bool("only-organization", false, "Only show organization folders")
	cmd.Flags().Bool("only-required", false, "Only show required folders")

	cmd.Flags().Bool("no-implementation", false, "Exclude implementation definitions")
	cmd.Flags().Bool("no-interface", false, "Exclude interface definitions")
	cmd.Flags().Bool("no-constant", false, "Exclude constant definitions")
	cmd.Flags().Bool("only-implementation", false, "Only show implementation definitions")
	cmd.Flags().Bool("only-interface", false, "Only show interface definitions")
	cmd.Flags().Bool("only-constant", false, "Only show constant definitions")

	cmd.Flags().IntSlice("no-year", nil, "Exclude years")
	cmd.Flags().IntSlice("only-year", nil, "Only show years")
	cmd.Flags().IntSlice("no-month", nil, "Exclude months")
	cmd.Flags().IntSlice("only-month", nil, "Only show months")
	cmd.Flags().IntSlice("no-day", nil, "Exclude days")
	cmd.Flags().IntSlice("only-day", nil, "Only show days")

	cmd.Flags().StringSlice("no-contributor", nil, "Exclude contributors")
	cmd.Flags().StringSlice("only-contributor", nil, "Only show contributors")
	cmd.Flags().StringSlice("no-policy", nil, "Exclude policies")
	cmd.Flags().StringSlice("only-policy", nil, "Only show policies")
	cmd.Flags().StringSlice("no-violation", nil, "Exclude violation kinds")
	cmd.Flags().StringSlice("only-violation", nil, "Only show violation kinds")

	cmd.Flags().String("filter", "", "Filter string")
	cmd.Flags().Bool("regex", false, "Use regex for filter")
	cmd.Flags().Bool("match-case", false, "Match case for filter")
	cmd.Flags().Bool("match-whole-word", false, "Match whole word for filter")
}

func getStreamOptions(cmd *cobra.Command) StreamOptions {
	showIgnored, _ := cmd.Flags().GetBool("show-ignored")
	showGenerated, _ := cmd.Flags().GetBool("show-generated")

	var excludeKinds []string
	if v, _ := cmd.Flags().GetBool("no-code"); v {
		excludeKinds = append(excludeKinds, FileKindCode)
	}
	if v, _ := cmd.Flags().GetBool("no-script"); v {
		excludeKinds = append(excludeKinds, FileKindScript)
	}
	if v, _ := cmd.Flags().GetBool("no-config"); v {
		excludeKinds = append(excludeKinds, FileKindConfig)
	}
	if v, _ := cmd.Flags().GetBool("no-test"); v {
		excludeKinds = append(excludeKinds, FileKindTest)
	}
	if v, _ := cmd.Flags().GetBool("no-docs"); v {
		excludeKinds = append(excludeKinds, FileKindDocs)
	}
	if v, _ := cmd.Flags().GetBool("no-resource"); v {
		excludeKinds = append(excludeKinds, FileKindResource)
	}
	if v, _ := cmd.Flags().GetBool("no-license"); v {
		excludeKinds = append(excludeKinds, FileKindLicense)
	}

	var includeKinds []string
	if v, _ := cmd.Flags().GetBool("only-code"); v {
		includeKinds = append(includeKinds, FileKindCode)
	}
	if v, _ := cmd.Flags().GetBool("only-script"); v {
		includeKinds = append(includeKinds, FileKindScript)
	}
	if v, _ := cmd.Flags().GetBool("only-config"); v {
		includeKinds = append(includeKinds, FileKindConfig)
	}
	if v, _ := cmd.Flags().GetBool("only-test"); v {
		includeKinds = append(includeKinds, FileKindTest)
	}
	if v, _ := cmd.Flags().GetBool("only-docs"); v {
		includeKinds = append(includeKinds, FileKindDocs)
	}
	if v, _ := cmd.Flags().GetBool("only-resource"); v {
		includeKinds = append(includeKinds, FileKindResource)
	}
	if v, _ := cmd.Flags().GetBool("only-license"); v {
		includeKinds = append(includeKinds, FileKindLicense)
	}

	var excludeBundleKinds []BundleKind
	if v, _ := cmd.Flags().GetBool("no-library"); v {
		excludeBundleKinds = append(excludeBundleKinds, BundleKindLibrary)
	}
	if v, _ := cmd.Flags().GetBool("no-schema"); v {
		excludeBundleKinds = append(excludeBundleKinds, BundleKindSchema)
	}
	if v, _ := cmd.Flags().GetBool("no-binary"); v {
		excludeBundleKinds = append(excludeBundleKinds, BundleKindBinary)
	}
	if v, _ := cmd.Flags().GetBool("no-ui"); v {
		excludeBundleKinds = append(excludeBundleKinds, BundleKindUI)
	}
	if v, _ := cmd.Flags().GetBool("no-site"); v {
		excludeBundleKinds = append(excludeBundleKinds, BundleKindSite)
	}
	if v, _ := cmd.Flags().GetBool("no-assets"); v {
		excludeBundleKinds = append(excludeBundleKinds, BundleKindAssets)
	}

	var includeBundleKinds []BundleKind
	if v, _ := cmd.Flags().GetBool("only-library"); v {
		includeBundleKinds = append(includeBundleKinds, BundleKindLibrary)
	}
	if v, _ := cmd.Flags().GetBool("only-schema"); v {
		includeBundleKinds = append(includeBundleKinds, BundleKindSchema)
	}
	if v, _ := cmd.Flags().GetBool("only-binary"); v {
		includeBundleKinds = append(includeBundleKinds, BundleKindBinary)
	}
	if v, _ := cmd.Flags().GetBool("only-ui"); v {
		includeBundleKinds = append(includeBundleKinds, BundleKindUI)
	}
	if v, _ := cmd.Flags().GetBool("only-site"); v {
		includeBundleKinds = append(includeBundleKinds, BundleKindSite)
	}
	if v, _ := cmd.Flags().GetBool("only-assets"); v {
		includeBundleKinds = append(includeBundleKinds, BundleKindAssets)
	}

	var excludeFolderKinds []FolderKind
	if v, _ := cmd.Flags().GetBool("no-organization"); v {
		excludeFolderKinds = append(excludeFolderKinds, FolderKindOrganization)
	}
	if v, _ := cmd.Flags().GetBool("no-required"); v {
		excludeFolderKinds = append(excludeFolderKinds, FolderKindRequired)
	}

	var includeFolderKinds []FolderKind
	if v, _ := cmd.Flags().GetBool("only-organization"); v {
		includeFolderKinds = append(includeFolderKinds, FolderKindOrganization)
	}
	if v, _ := cmd.Flags().GetBool("only-required"); v {
		includeFolderKinds = append(includeFolderKinds, FolderKindRequired)
	}

	var excludeDefinitionKinds []DefinitionKind
	if v, _ := cmd.Flags().GetBool("no-implementation"); v {
		excludeDefinitionKinds = append(excludeDefinitionKinds, DefinitionKindImplementation)
	}
	if v, _ := cmd.Flags().GetBool("no-interface"); v {
		excludeDefinitionKinds = append(excludeDefinitionKinds, DefinitionKindInterface)
	}
	if v, _ := cmd.Flags().GetBool("no-constant"); v {
		excludeDefinitionKinds = append(excludeDefinitionKinds, DefinitionKindConstant)
	}

	var includeDefinitionKinds []DefinitionKind
	if v, _ := cmd.Flags().GetBool("only-implementation"); v {
		includeDefinitionKinds = append(includeDefinitionKinds, DefinitionKindImplementation)
	}
	if v, _ := cmd.Flags().GetBool("only-interface"); v {
		includeDefinitionKinds = append(includeDefinitionKinds, DefinitionKindInterface)
	}
	if v, _ := cmd.Flags().GetBool("only-constant"); v {
		includeDefinitionKinds = append(includeDefinitionKinds, DefinitionKindConstant)
	}

	excludeYears, _ := cmd.Flags().GetIntSlice("no-year")
	includeYears, _ := cmd.Flags().GetIntSlice("only-year")
	excludeMonths, _ := cmd.Flags().GetIntSlice("no-month")
	includeMonths, _ := cmd.Flags().GetIntSlice("only-month")
	excludeDays, _ := cmd.Flags().GetIntSlice("no-day")
	includeDays, _ := cmd.Flags().GetIntSlice("only-day")

	excludeContributors, _ := cmd.Flags().GetStringSlice("no-contributor")
	includeContributors, _ := cmd.Flags().GetStringSlice("only-contributor")
	excludePolicies, _ := cmd.Flags().GetStringSlice("no-policy")
	includePolicies, _ := cmd.Flags().GetStringSlice("only-policy")
	excludeViolations, _ := cmd.Flags().GetStringSlice("no-violation")
	includeViolations, _ := cmd.Flags().GetStringSlice("only-violation")

	filter, _ := cmd.Flags().GetString("filter")
	regex, _ := cmd.Flags().GetBool("regex")
	matchCase, _ := cmd.Flags().GetBool("match-case")
	matchWholeWord, _ := cmd.Flags().GetBool("match-whole-word")

	return StreamOptions{
		ShowIgnored:            showIgnored,
		ShowGenerated:          showGenerated,
		ExcludeKinds:           excludeKinds,
		IncludeKinds:           includeKinds,
		ExcludeBundleKinds:     excludeBundleKinds,
		IncludeBundleKinds:     includeBundleKinds,
		ExcludeFolderKinds:     excludeFolderKinds,
		IncludeFolderKinds:     includeFolderKinds,
		ExcludeDefinitionKinds: excludeDefinitionKinds,
		IncludeDefinitionKinds: includeDefinitionKinds,
		ExcludeYears:           excludeYears,
		IncludeYears:           includeYears,
		ExcludeMonths:          excludeMonths,
		IncludeMonths:          includeMonths,
		ExcludeDays:            excludeDays,
		IncludeDays:            includeDays,
		ExcludeContributors:    excludeContributors,
		IncludeContributors:    includeContributors,
		ExcludePolicies:        excludePolicies,
		IncludePolicies:        includePolicies,
		ExcludeViolations:      excludeViolations,
		IncludeViolations:      includeViolations,
		Filter:                 filter,
		Regex:                  regex,
		MatchCase:              matchCase,
		MatchWholeWord:         matchWholeWord,
	}
}

func fileCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "file", Short: "File management commands"}
	createCmd := &cobra.Command{
		Use:   "create",
		Short: "Create a file",
		Args:  cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" && len(args) > 0 {
				path = args[0]
			}
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
		Args:  cobra.MaximumNArgs(2),
		RunE: func(cmd *cobra.Command, args []string) error {
			src, _ := cmd.Flags().GetString("source")
			dst, _ := cmd.Flags().GetString("target")
			if (src == "" || dst == "") && len(args) > 0 {
				if src == "" && len(args) > 0 {
					src = args[0]
				}
				if dst == "" && len(args) > 1 {
					dst = args[1]
				}
			}
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
		Args:  cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			path, _ := cmd.Flags().GetString("path")
			if path == "" && len(args) > 0 {
				path = args[0]
			}
			if path == "" {
				return fmt.Errorf("missing path")
			}
			variables := map[string]interface{}{"path": path}
			query := `mutation FileDelete($path: String!) { fileDelete(path: $path) }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	deleteCmd.Flags().String("path", "", "File path")

	listCmd := &cobra.Command{
		Use:   "list [scope]",
		Short: "List files",
		RunE: func(cmd *cobra.Command, args []string) error {
			scope := ""
			if len(args) > 0 {
				scope = args[0]
			}
			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "file list"}

				fileChan := make(chan File)
				go func() {
					StreamFiles(context.Background(), scope, fileChan, opts)
				}()

				for f := range fileChan {
					data, err := json.Marshal(map[string]interface{}{"file": f})
					if err != nil {
						continue
					}
					stream <- Event{Kind: KindResult, Command: "file list", Data: data}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(listCmd)

	treeCmd := &cobra.Command{
		Use:   "tree [scope]",
		Short: "Show file tree",
		RunE: func(cmd *cobra.Command, args []string) error {
			scope := ""
			if len(args) > 0 {
				scope = args[0]
			}
			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "file tree"}

				fileChan := make(chan File)
				go func() {
					StreamFiles(context.Background(), scope, fileChan, opts)
				}()

				var files []File
				for f := range fileChan {
					files = append(files, f)
				}

				type node struct {
					name     string
					children map[string]*node
					isFile   bool
				}
				root := &node{children: make(map[string]*node)}

				for _, f := range files {
					parts := strings.Split(f.Path, "/")
					curr := root
					for i, part := range parts {
						if _, ok := curr.children[part]; !ok {
							curr.children[part] = &node{name: part, children: make(map[string]*node)}
						}
						curr = curr.children[part]
						if i == len(parts)-1 {
							curr.isFile = true
						}
					}
				}

				if config.Markdown {
					var sb strings.Builder
					var printMarkdown func(*node, int, string)
					printMarkdown = func(n *node, depth int, pathPrefix string) {
						keys := make([]string, 0, len(n.children))
						for k := range n.children {
							keys = append(keys, k)
						}
						sort.Strings(keys)

						for _, k := range keys {
							child := n.children[k]
							currentPath := k
							if pathPrefix != "" {
								currentPath = pathPrefix + "/" + k
							}

							indent := strings.Repeat("  ", depth)
							sb.WriteString(fmt.Sprintf("%s- [%s](#%s): %s\n", indent, currentPath, currentPath, child.name))
							printMarkdown(child, depth+1, currentPath)
						}
					}
					printMarkdown(root, 0, "")
					data, _ := json.Marshal(map[string]string{"markdown": sb.String()})
					stream <- Event{Kind: KindResult, Command: "file tree", Data: data}
				} else {
					var printNode func(*node, string)
					printNode = func(n *node, prefix string) {
						keys := make([]string, 0, len(n.children))
						for k := range n.children {
							keys = append(keys, k)
						}
						sort.Strings(keys)

						for i, k := range keys {
							child := n.children[k]
							isLast := i == len(keys)-1
							connector := "├── "
							if isLast {
								connector = "└── "
							}
							childPrefix := prefix + "│   "
							if isLast {
								childPrefix = prefix + "    "
							}

							display := child.name
							if !child.isFile {
								display += "/"
							}
							stream <- Event{Kind: KindLog, Level: "info", Command: "file tree", Message: prefix + connector + display}
							printNode(child, childPrefix)
						}
					}
					printNode(root, "")
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()
			return renderStream(cmd, config, stream)
		},
	}
	bindStreamFlags(treeCmd)

	root.AddCommand(createCmd)
	root.AddCommand(moveCmd)
	root.AddCommand(deleteCmd)
	root.AddCommand(listCmd)
	root.AddCommand(treeCmd)
	return root
}

func sectionCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "section", Short: "Section management commands"}
	createCmd := &cobra.Command{
		Use:   "create",
		Short: "Create a section",
		Args:  cobra.MaximumNArgs(3),
		RunE: func(cmd *cobra.Command, args []string) error {
			file, _ := cmd.Flags().GetString("file")
			name, _ := cmd.Flags().GetString("name")
			parent, _ := cmd.Flags().GetString("parent")
			if file == "" && len(args) > 0 {
				file = args[0]
			}
			if name == "" && len(args) > 1 {
				name = args[1]
			}
			if parent == "" && len(args) > 2 {
				parent = args[2]
			}
			if file == "" || name == "" {
				return fmt.Errorf("missing file or name")
			}
			variables := map[string]interface{}{"file": file, "name": name}
			if parent != "" {
				variables["parent"] = parent
			}
			query := `mutation SectionCreate($file: String!, $name: String!, $parent: String) { sectionCreate(file: $file, name: $name, parent: $parent) { id name range { start { line column } end { line column } } } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	createCmd.Flags().String("file", "", "File path")
	createCmd.Flags().String("name", "", "Section name")
	createCmd.Flags().String("parent", "", "Parent section")
	moveCmd := &cobra.Command{
		Use:   "move",
		Short: "Move a section",
		Args:  cobra.MaximumNArgs(3),
		RunE: func(cmd *cobra.Command, args []string) error {
			file, _ := cmd.Flags().GetString("file")
			oldName, _ := cmd.Flags().GetString("old")
			newName, _ := cmd.Flags().GetString("new")
			if file == "" && len(args) > 0 {
				file = args[0]
			}
			if oldName == "" && len(args) > 1 {
				oldName = args[1]
			}
			if newName == "" && len(args) > 2 {
				newName = args[2]
			}
			if file == "" || oldName == "" || newName == "" {
				return fmt.Errorf("missing file or names")
			}
			variables := map[string]interface{}{"file": file, "oldName": oldName, "newName": newName}
			query := `mutation SectionMove($file: String!, $oldName: String!, $newName: String!) { sectionMove(file: $file, oldName: $oldName, newName: $newName) { id name range { start { line column } end { line column } } } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	moveCmd.Flags().String("file", "", "File path")
	moveCmd.Flags().String("old", "", "Old section name")
	moveCmd.Flags().String("new", "", "New section name")
	deleteCmd := &cobra.Command{
		Use:   "delete",
		Short: "Delete a section",
		Args:  cobra.MaximumNArgs(2),
		RunE: func(cmd *cobra.Command, args []string) error {
			file, _ := cmd.Flags().GetString("file")
			name, _ := cmd.Flags().GetString("name")
			if file == "" && len(args) > 0 {
				file = args[0]
			}
			if name == "" && len(args) > 1 {
				name = args[1]
			}
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
		Use:   "list [file]",
		Short: "List sections",
		Args:  cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			file, _ := cmd.Flags().GetString("file")
			if file == "" && len(args) > 0 {
				file = args[0]
			}
			if file == "" {
				return fmt.Errorf("missing file")
			}

			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "section list"}

				sectionChan := make(chan Section)
				go func() {
					StreamSections(context.Background(), file, sectionChan, opts)
				}()

				for s := range sectionChan {
					data, err := json.Marshal(map[string]interface{}{"section": s})
					if err != nil {
						continue
					}
					stream <- Event{Kind: KindResult, Command: "section list", Data: data}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()

			return renderStream(cmd, config, stream)
		},
	}
	listCmd.Flags().String("file", "", "File path")
	bindStreamFlags(listCmd)

	treeCmd := &cobra.Command{
		Use:   "tree [file]",
		Short: "Show section tree",
		Args:  cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			file, _ := cmd.Flags().GetString("file")
			if file == "" && len(args) > 0 {
				file = args[0]
			}
			if file == "" {
				return fmt.Errorf("missing file")
			}

			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "section tree"}

				sectionChan := make(chan Section)
				go func() {
					StreamSections(context.Background(), file, sectionChan, opts)
				}()

				// Sections from StreamSections might be flat or tree.
				// If they are top-level with Children populated, we just iterate.
				var sections []Section
				for s := range sectionChan {
					sections = append(sections, s)
				}

				if config.Markdown {
					var sb strings.Builder
					var printMarkdown func(Section, int)
					printMarkdown = func(s Section, depth int) {
						indent := strings.Repeat("  ", depth)
						uri := fmt.Sprintf("semiorepo://section/%s/%s", s.FilePath, Slugify(s.Path))
						// - [<id>](#<id>): <title> - <description>
						id := s.Name // Using name as ID for display
						sb.WriteString(fmt.Sprintf("%s- [%s](%s): %s\n", indent, id, uri, s.Name))
						for _, child := range s.Children {
							printMarkdown(child, depth+1)
						}
					}
					for _, s := range sections {
						printMarkdown(s, 0)
					}
					data, _ := json.Marshal(map[string]string{"markdown": sb.String()})
					stream <- Event{Kind: KindResult, Command: "section tree", Data: data}
				} else {
					var printNode func(Section, string)
					printNode = func(s Section, prefix string) {
						// Logic similar to folder/file but for section structure
						children := s.Children
						for i, child := range children {
							isLast := i == len(children)-1
							connector := "├── "
							if isLast {
								connector = "└── "
							}
							childPrefix := prefix + "│   "
							if isLast {
								childPrefix = prefix + "    "
							}
							stream <- Event{Kind: KindLog, Level: "info", Command: "section tree", Message: prefix + connector + child.Name}
							printNode(child, childPrefix)
						}
					}

					for _, s := range sections {
						stream <- Event{Kind: KindLog, Level: "info", Command: "section tree", Message: s.Name}
						printNode(s, "")
					}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()

			return renderStream(cmd, config, stream)
		},
	}
	treeCmd.Flags().String("file", "", "File path")
	bindStreamFlags(treeCmd)
	integrateCmd := &cobra.Command{
		Use:   "integrate",
		Short: "Integrate source code into a target file section",
		Args:  cobra.MaximumNArgs(4),
		RunE: func(cmd *cobra.Command, args []string) error {
			source, _ := cmd.Flags().GetString("source")
			targetSection, _ := cmd.Flags().GetString("target-section")
			targetFile, _ := cmd.Flags().GetString("target-file")
			targetParent, _ := cmd.Flags().GetString("target-parent")
			if source == "" && len(args) > 0 {
				source = args[0]
			}
			if targetSection == "" && len(args) > 1 {
				targetSection = args[1]
			}
			if targetFile == "" && len(args) > 2 {
				targetFile = args[2]
			}
			if targetParent == "" && len(args) > 3 {
				targetParent = args[3]
			}
			if source == "" || targetSection == "" || targetFile == "" {
				return fmt.Errorf("missing source, target section, or target file")
			}
			variables := map[string]interface{}{
				"source":        source,
				"targetSection": targetSection,
				"targetFile":    targetFile,
			}
			if targetParent != "" {
				variables["targetParent"] = targetParent
			}
			query := `mutation Integrate($source: String!, $targetSection: String!, $targetFile: String!, $targetParent: String) { integrate(source: $source, targetSection: $targetSection, targetFile: $targetFile, targetParent: $targetParent) { id } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	integrateCmd.Flags().String("source", "", "Source file path")
	integrateCmd.Flags().String("target-section", "", "Target section name")
	integrateCmd.Flags().String("target-file", "", "Target file path")
	integrateCmd.Flags().String("target-parent", "", "Target parent section name")

	extractCmd := &cobra.Command{
		Use:   "extract",
		Short: "Extract a section from a source file into a target file",
		Args:  cobra.MaximumNArgs(3),
		RunE: func(cmd *cobra.Command, args []string) error {
			sourceFile, _ := cmd.Flags().GetString("source-file")
			sourceSection, _ := cmd.Flags().GetString("source-section")
			targetFile, _ := cmd.Flags().GetString("target-file")

			if sourceFile == "" && len(args) > 0 {
				sourceFile = args[0]
			}
			if sourceSection == "" && len(args) > 1 {
				sourceSection = args[1]
			}
			if targetFile == "" && len(args) > 2 {
				targetFile = args[2]
			}

			if sourceFile == "" || sourceSection == "" || targetFile == "" {
				return fmt.Errorf("missing source file, source section, or target file")
			}

			variables := map[string]interface{}{
				"sourceFile":    sourceFile,
				"sourceSection": sourceSection,
				"targetFile":    targetFile,
			}
			query := `mutation Extract($sourceFile: String!, $sourceSection: String!, $targetFile: String!) { extract(sourceFile: $sourceFile, sourceSection: $sourceSection, targetFile: $targetFile) { id } }`
			return runGraphQL(cmd, factory, config, query, variables)
		},
	}
	extractCmd.Flags().String("source-file", "", "Source file path")
	extractCmd.Flags().String("source-section", "", "Source section name")
	extractCmd.Flags().String("target-file", "", "Target file path")

	root.AddCommand(createCmd)
	root.AddCommand(moveCmd)
	root.AddCommand(deleteCmd)
	root.AddCommand(listCmd)
	root.AddCommand(treeCmd)
	root.AddCommand(integrateCmd)
	root.AddCommand(extractCmd)
	return root
}

func definitionCommand(factory EngineFactory, config *Config) *cobra.Command {
	root := &cobra.Command{Use: "definition", Short: "Definition management commands"}
	listCmd := &cobra.Command{
		Use:     "list",
		Aliases: []string{"tree"},
		Short:   "List definitions",
		Args:    cobra.MaximumNArgs(1),
		RunE: func(cmd *cobra.Command, args []string) error {
			path, _ := cmd.Flags().GetString("file")
			if path == "" && len(args) > 0 {
				path = args[0]
			}
			if path == "" {
				return fmt.Errorf("missing file")
			}

			opts := getStreamOptions(cmd)
			stream := make(chan Event)
			go func() {
				defer close(stream)
				stream <- Event{Kind: KindStart, Command: "definition list"}

				defChan := make(chan Definition)
				go func() {
					StreamDefinitions(context.Background(), path, defChan, opts)
				}()

				for d := range defChan {
					data, err := json.Marshal(map[string]interface{}{"definition": d})
					if err != nil {
						continue
					}
					stream <- Event{Kind: KindResult, Command: "definition list", Data: data}
				}
				stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
			}()

			return renderStream(cmd, config, stream)
		},
	}
	listCmd.Flags().String("file", "", "File path")
	bindStreamFlags(listCmd)
	root.AddCommand(listCmd)
	return root
}

// #region CLI Renderers

type StreamRenderer interface {
	Render(ctx context.Context, out, errOut io.Writer, stream <-chan Event) (int, error)
}

type NDJSONRenderer struct{}

func (r NDJSONRenderer) Render(ctx context.Context, out, errOut io.Writer, stream <-chan Event) (int, error) {
	encoder := json.NewEncoder(out)
	encoder.SetEscapeHTML(false)
	// In strict NDJSON mode, everything goes to stdout as JSON events
	// but we respect the out writer passed in (which is usually stdout)

	exitCode := 0
	for event := range stream {
		if event.Kind == KindDone && event.Done != nil {
			exitCode = event.Done.ExitCode
		}
		if err := encoder.Encode(event); err != nil {
			return exitCode, err
		}
		// Flush if possible to ensure streaming
		if f, ok := out.(interface{ Flush() error }); ok {
			f.Flush()
		}
	}
	return exitCode, nil
}

// #region ANSI

const (
	ColorReset  = "\033[0m"
	ColorRed    = "\033[31m"
	ColorGreen  = "\033[32m"
	ColorYellow = "\033[33m"
	ColorBlue   = "\033[34m"
	ColorDim    = "\033[2m"
	ColorBold   = "\033[1m"
)

func colorize(s string, color string, enabled bool) string {
	if !enabled {
		return s
	}
	return color + s + ColorReset
}

// #endregion ANSI

type HumanRenderer struct {
	Verbose bool
}

func (r HumanRenderer) Render(ctx context.Context, out, errOut io.Writer, stream <-chan Event) (int, error) {
	exitCode := 0
	isTTY := false
	if f, ok := out.(*os.File); ok {
		stat, _ := f.Stat()
		if (stat.Mode() & os.ModeCharDevice) != 0 {
			isTTY = true
		}
	}
	// Also check env var NO_COLOR
	if os.Getenv("NO_COLOR") != "" {
		isTTY = false
	}

	startTime := time.Now()

	for event := range stream {
		if event.Kind == KindDone && event.Done != nil {
			exitCode = event.Done.ExitCode
			duration := time.Since(startTime).Round(time.Millisecond)

			statusIcon := "✓"
			color := ColorGreen
			if exitCode != 0 {
				statusIcon = "✗"
				color = ColorRed
			}

			summary := fmt.Sprintf("%s done  %s  %s", statusIcon, event.Command, duration)
			if exitCode != 0 {
				summary = fmt.Sprintf("%s failed %s  %s (exit: %d)", statusIcon, event.Command, duration, exitCode)
			}

			if isTTY {
				fmt.Fprint(out, "\r\033[K") // Clear line
				fmt.Fprintln(out, colorize(summary, color, true))
			} else {
				fmt.Fprintln(out, summary)
			}
			continue
		}
		if event.Kind == KindError && event.Error != nil {
			if isTTY {
				fmt.Fprint(out, "\r\033[K")
			}
			fmt.Fprintf(errOut, "%s error: %s\n", colorize("✗", ColorRed, isTTY), event.Error.Message)
			if r.Verbose && event.Error.Detail != "" {
				fmt.Fprintf(errOut, "%s\n", event.Error.Detail)
			}
			continue
		}
		if event.Kind == KindLog && event.Message != "" {
			if isTTY {
				fmt.Fprint(out, "\r\033[K")
			}
			prefix := colorize("•", ColorDim, isTTY)
			fmt.Fprintf(errOut, "%s %s\n", prefix, event.Message)
			continue
		}
		if event.Kind == KindResult && len(event.Data) > 0 {
			if isTTY {
				fmt.Fprint(out, "\r\033[K")
			}
			fmt.Fprint(out, formatResult(event.Command, event.Data, isTTY))
		}
		if event.Kind == KindProgress && event.Progress != nil {
			if isTTY {
				fmt.Fprintf(out, "\r%s %d%% (%d/%d) %s", colorize("↻", ColorBlue, true), event.Progress.Percent, event.Progress.Current, event.Progress.Total, event.Progress.Step)
			} else {
				// Avoid spamming non-TTY logs
				if event.Progress.Percent%10 == 0 && event.Progress.Percent > 0 {
					fmt.Fprintf(out, "progress: %d%% %s\n", event.Progress.Percent, event.Progress.Step)
				}
			}
		}
	}
	return exitCode, nil
}

func formatResult(command string, data json.RawMessage, isTTY bool) string {
	var raw map[string]interface{}
	if err := json.Unmarshal(data, &raw); err != nil {
		return fmt.Sprintf("→ %s\n", string(data))
	}

	// Helper to handle GraphQL response wrappers
	// The engine wraps the graphql result in "data".
	var payload map[string]interface{} = raw
	if len(raw) == 1 {
		if d, ok := raw["data"].(map[string]interface{}); ok {
			payload = d
		}
	}

	var sb strings.Builder
	prefix := colorize("→", ColorBlue, isTTY)

	// Case 1: Analyze result
	if analyze, ok := payload["analyze"].(map[string]interface{}); ok {
		// metrics
		if metrics, ok := analyze["metrics"].(map[string]interface{}); ok {
			total := metrics["total"]
			autofixable := metrics["autofixable"]

			summaryText := fmt.Sprintf("found %v violations", total)
			if autofixable != nil {
				summaryText += fmt.Sprintf(" (%v autofixable)", autofixable)
			}
			sb.WriteString(fmt.Sprintf("%s %s\n", prefix, summaryText))
		}

		// violations
		if violations, ok := analyze["violations"].([]interface{}); ok {
			for _, v := range violations {
				if vio, ok := v.(map[string]interface{}); ok {
					kind := ""
					if k, ok := vio["kind"].(map[string]interface{}); ok {
						kind = fmt.Sprintf("%v", k["id"])
					} else if kStr, ok := vio["kind"].(string); ok {
						kind = kStr
					}

					scope := fmt.Sprintf("%v", vio["scope"])
					line := fmt.Sprintf("%v", vio["line"])
					summary := fmt.Sprintf("%v", vio["summary"])

					loc := fmt.Sprintf("%s:%s", scope, line)

					// Indented violation line
					lineStr := fmt.Sprintf("  %s %s %s %s\n",
						colorize("violation", ColorRed, isTTY),
						kind,
						colorize(loc, ColorDim, isTTY),
						summary)
					sb.WriteString(lineStr)
				}
			}
		}
		if sb.Len() > 0 {
			return sb.String()
		}
	}

	// Case 2: Fix result
	if fix, ok := payload["fix"].(map[string]interface{}); ok {
		fixed := fix["fixed"]
		remaining := fix["remaining"]
		sb.WriteString(fmt.Sprintf("%s fixed %v violations (%v remaining)\n", prefix, fixed, remaining))
		return sb.String()
	}

	// Case 3: Goal Tree
	if repo, ok := payload["repo"].(map[string]interface{}); ok {
		goals, goalsOk := repo["goals"].([]interface{})
		tickets, ticketsOk := repo["tickets"].([]interface{})
		if goalsOk && ticketsOk {
			return formatGoalTree(goals, tickets, isTTY)
		}
	}

	// Case 4: Ticket operations
	for k, v := range payload {
		if strings.HasPrefix(k, "ticket") || k == "ticket" {
			if tMap, ok := v.(map[string]interface{}); ok {
				if slug, ok := tMap["slug"].(string); ok {
					status, _ := tMap["status"].(string)
					statusColor := ColorGreen
					if status == "open" {
						statusColor = ColorBlue
					}

					// Check for rich details (Title, Date) for list view
					if title, ok := tMap["title"].(string); ok {
						statusIcon := "◯"
						if status == "finished" || status == "closed" {
							statusIcon = "✓"
							statusColor = ColorGreen
						} else {
							statusColor = ColorBlue
						}

						dateDisplay := ""
						if dates, ok := tMap["date"].(map[string]interface{}); ok {
							if created, ok := dates["created"].(string); ok {
								dateDisplay = created
								if tParsed, err := time.Parse(time.RFC3339, created); err == nil {
									dateDisplay = tParsed.Format("2006-01-02")
								}
							}
						}

						sb.WriteString(fmt.Sprintf("  %s %s %s %s\n",
							colorize(statusIcon, statusColor, isTTY),
							colorize(title, ColorBold, isTTY),
							colorize("("+slug+")", ColorDim, isTTY),
							colorize("("+dateDisplay+")", ColorDim, isTTY)))
						return sb.String()
					}

					sb.WriteString(fmt.Sprintf("%s ticket %s is %s\n", prefix, colorize(slug, ColorBold, isTTY), colorize(status, statusColor, isTTY)))
					return sb.String()
				}
			}
		}
	}

	// Case 4: Direct ticket object
	if slug, ok := payload["slug"].(string); ok {
		status, _ := payload["status"].(string)
		path, _ := payload["path"].(string)
		sb.WriteString(fmt.Sprintf("%s ticket %s (%s) %s\n", prefix, colorize(slug, ColorBold, isTTY), status, path))
		return sb.String()
	}

	// Case 5: Generic ID/Path
	if id, ok := payload["id"].(string); ok {
		if path, ok := payload["path"].(string); ok {
			return fmt.Sprintf("%s %s %s\n", prefix, id, path)
		}
		return fmt.Sprintf("%s item %s\n", prefix, id)
	}

	// Case 6: Repo result
	if repo, ok := payload["repo"].(map[string]interface{}); ok {
		// Tickets
		if tickets, ok := repo["tickets"].([]interface{}); ok {
			sb.WriteString(fmt.Sprintf("%s found %d tickets\n", prefix, len(tickets)))
			for _, t := range tickets {
				if ticket, ok := t.(map[string]interface{}); ok {
					slug, _ := ticket["slug"].(string)
					status, _ := ticket["status"].(string)
					title, _ := ticket["title"].(string)
					dates, _ := ticket["date"].(map[string]interface{})
					createdStr, _ := dates["created"].(string)

					// Simple date parsing to YYYY-MM-DD
					dateDisplay := createdStr
					if tParsed, err := time.Parse(time.RFC3339, createdStr); err == nil {
						dateDisplay = tParsed.Format("2006-01-02")
					}

					statusIcon := "◯"
					statusColor := ColorBlue
					if status == "finished" || status == "closed" {
						statusIcon = "✓"
						statusColor = ColorGreen
					}

					// Compact line: Icon Title (SLUG) (Date)
					line := fmt.Sprintf("  %s %s %s %s",
						colorize(statusIcon, statusColor, isTTY),
						colorize(title, ColorBold, isTTY),
						colorize("("+slug+")", ColorDim, isTTY),
						colorize("("+dateDisplay+")", ColorDim, isTTY))

					sb.WriteString(line + "\n")
				}
			}
			return sb.String()
		}

		// Bundles
		if bundles, ok := repo["bundles"].([]interface{}); ok {
			sb.WriteString(fmt.Sprintf("%s found %d bundles\n", prefix, len(bundles)))
			for _, b := range bundles {
				if bundle, ok := b.(map[string]interface{}); ok {
					name, _ := bundle["name"].(string)
					root, _ := bundle["root"].(string)
					ptype, _ := bundle["projectType"].(string)
					if ptype == "" {
						ptype, _ = bundle["type"].(string)
					}

					// Compact line: 📦 NAME (root: ROOT, type: PROJECTTYPE)
					line := fmt.Sprintf("  %s %s (root: %s, type: %s)",
						colorize("📦", ColorBlue, isTTY),
						colorize(name, ColorBold, isTTY),
						root,
						ptype)
					sb.WriteString(line + "\n")
				}
			}
			return sb.String()
		}
	}

	// Case 6b: Single Ticket Stream Item
	if ticket, ok := payload["ticket"].(map[string]interface{}); ok {
		slug, _ := ticket["slug"].(string)
		status, _ := ticket["status"].(string)
		title, _ := ticket["title"].(string)
		dates, _ := ticket["date"].(map[string]interface{})
		createdStr, _ := dates["created"].(string)

		// Simple date parsing to YYYY-MM-DD
		dateDisplay := createdStr
		if tParsed, err := time.Parse(time.RFC3339, createdStr); err == nil {
			dateDisplay = tParsed.Format("2006-01-02")
		}

		statusIcon := "◯"
		statusColor := ColorBlue
		if status == "finished" || status == "closed" {
			statusIcon = "✓"
			statusColor = ColorGreen
		}

		// Compact line: Icon Title (SLUG) (Date)
		return fmt.Sprintf("  %s %s %s %s\n",
			colorize(statusIcon, statusColor, isTTY),
			colorize(title, ColorBold, isTTY),
			colorize("("+slug+")", ColorDim, isTTY),
			colorize("("+dateDisplay+")", ColorDim, isTTY))
	}

	// Case 6c: Single Bundle Stream Item
	if bundle, ok := payload["bundle"].(map[string]interface{}); ok {
		name, _ := bundle["name"].(string)
		root, _ := bundle["root"].(string)
		ptype, _ := bundle["projectType"].(string)
		if ptype == "" {
			ptype, _ = bundle["type"].(string)
		}

		typeStr := ""
		if ptype != "" {
			typeStr = fmt.Sprintf(", type: %s", ptype)
		}

		// Compact line: 📦 NAME (root: ROOT, type: PROJECTTYPE)
		return fmt.Sprintf("  %s %s (root: %s%s)\n",
			colorize("📦", ColorBlue, isTTY),
			colorize(name, ColorBold, isTTY),
			root,
			typeStr)
	}

	// Case 7: File result
	if file, ok := payload["file"].(map[string]interface{}); ok {
		// Sections
		if sections, ok := file["sections"].([]interface{}); ok {
			sb.WriteString(fmt.Sprintf("%s found %d top-level sections\n", prefix, len(sections)))

			var printSections func(items []interface{}, indent string)
			printSections = func(items []interface{}, indent string) {
				for _, item := range items {
					if sec, ok := item.(map[string]interface{}); ok {
						name, _ := sec["name"].(string)
						rng, _ := sec["range"].(map[string]interface{})
						start := 0
						end := 0
						if rng != nil {
							if s, ok := rng["start"].(map[string]interface{}); ok {
								start = int(s["line"].(float64))
							} else if s, ok := rng["start"].(float64); ok {
								start = int(s)
							}
							if e, ok := rng["end"].(map[string]interface{}); ok {
								end = int(e["line"].(float64))
							} else if e, ok := rng["end"].(float64); ok {
								end = int(e)
							}
						}

						line := fmt.Sprintf("%s%s %s",
							indent,
							colorize("#"+name, ColorBold, isTTY),
							colorize(fmt.Sprintf("(%d-%d)", start, end), ColorDim, isTTY))
						sb.WriteString(line + "\n")

						if children, ok := sec["children"].([]interface{}); ok && len(children) > 0 {
							printSections(children, indent+"  ")
						}
					}
				}
			}
			printSections(sections, "  ")
			return sb.String()
		}

		// Definitions
		if definitions, ok := file["definitions"].([]interface{}); ok {
			sb.WriteString(fmt.Sprintf("%s found %d definitions\n", prefix, len(definitions)))
			for _, d := range definitions {
				if def, ok := d.(map[string]interface{}); ok {
					name, _ := def["name"].(string)
					kind, _ := def["kind"].(string)
					rng, _ := def["range"].(map[string]interface{})
					start := 0
					end := 0
					if rng != nil {
						start = int(rng["start"].(float64))
						end = int(rng["end"].(float64))
					}

					// Compact line: ƒ NAME (kind: KIND, L:START-END)
					line := fmt.Sprintf("  %s %s (kind: %s, L:%d-%d)",
						colorize("ƒ", ColorGreen, isTTY),
						colorize(name, ColorBold, isTTY),
						kind,
						start, end)
					sb.WriteString(line + "\n")
				}
			}
			return sb.String()
		}
	}

	// Case 8: Single Section Result
	if sec, ok := payload["section"].(map[string]interface{}); ok {
		name, _ := sec["name"].(string)
		startLine, _ := sec["startLine"].(float64)
		endLine, _ := sec["endLine"].(float64)

		childrenCount := 0
		if children, ok := sec["children"].([]interface{}); ok {
			childrenCount = len(children)
		}
		childrenStr := ""
		if childrenCount > 0 {
			childrenStr = fmt.Sprintf(" (+%d children)", childrenCount)
		}

		// # SECTION_PATH (:START-END) (+X children)
		sectionPath, _ := sec["path"].(string)
		displayName := name
		if sectionPath != "" {
			displayName = "#" + sectionPath
		} else {
			displayName = "#" + name
		}
		return fmt.Sprintf("  %s %s%s\n",
			colorize(displayName, ColorBold, isTTY),
			colorize(fmt.Sprintf("(:%d-%d)", int(startLine), int(endLine)), ColorDim, isTTY),
			colorize(childrenStr, ColorDim, isTTY))
	}

	// Case 9: Single Definition Result
	if def, ok := payload["definition"].(map[string]interface{}); ok {
		name, _ := def["name"].(string)
		kind, _ := def["kind"].(string)
		filePath, _ := def["filePath"].(string)
		startLine, _ := def["startLine"].(float64)
		endLine, _ := def["endLine"].(float64)

		// ƒ NAME (kind: KIND, FILE:START-END)
		return fmt.Sprintf("  %s %s %s\n",
			colorize("ƒ", ColorGreen, isTTY),
			colorize(name, ColorBold, isTTY),
			colorize(fmt.Sprintf("(kind: %s, %s:%d-%d)", kind, filePath, int(startLine), int(endLine)), ColorDim, isTTY))
	}

	// Case 11: Single Folder Result
	if folder, ok := payload["folder"].(map[string]interface{}); ok {
		path, _ := folder["path"].(string)
		kind, _ := folder["kind"].(string)

		// 📂 PATH (KIND)
		return fmt.Sprintf("  %s %s %s\n",
			colorize("📂", ColorBlue, isTTY),
			colorize(path, ColorBold, isTTY),
			colorize(fmt.Sprintf("(%s)", kind), ColorDim, isTTY))
	}

	// Case 12: Single File Result
	if file, ok := payload["file"].(map[string]interface{}); ok {
		path, _ := file["id"].(string)

		// 📄 PATH
		return fmt.Sprintf("  %s %s\n",
			colorize("📄", ColorDim, isTTY),
			path)
	}

	// Case 13: Single Goal Result
	if goal, ok := payload["goal"].(map[string]interface{}); ok {
		id, _ := goal["id"].(string)
		title, _ := goal["title"].(string)
		status, _ := goal["status"].(string)
		dates, _ := goal["dates"].(map[string]interface{})
		dueStr, _ := dates["due"].(string)

		// Icon logic
		statusIcon := "◯"
		statusColor := ColorBlue
		if status == "closed" || status == "finished" {
			statusIcon = "✓"
			statusColor = ColorGreen
		}

		dateStr := ""
		if dueStr != "" {
			dateStr = fmt.Sprintf(" (due: %s)", dueStr)
		}

		// 🎯 TITLE (ID) (due: DATE)
		return fmt.Sprintf("  %s %s %s%s\n",
			colorize(statusIcon, statusColor, isTTY),
			colorize(title, ColorBold, isTTY),
			colorize("("+id+")", ColorDim, isTTY),
			colorize(dateStr, ColorDim, isTTY))
	}

	// Case 14: Single Contributor Result
	if contrib, ok := payload["contributor"].(map[string]interface{}); ok {
		github, _ := contrib["github"].(string)
		name, _ := contrib["name"].(string)
		email, _ := contrib["email"].(string)
		// contributions might be a map
		contributions, _ := contrib["contributions"].(map[string]interface{})

		stats := []string{}
		if c, ok := contributions["commits"].(float64); ok && c > 0 {
			stats = append(stats, fmt.Sprintf("%d commits", int(c)))
		}
		if t, ok := contributions["tickets"].(float64); ok && t > 0 {
			stats = append(stats, fmt.Sprintf("%d tickets", int(t)))
		}

		statsStr := ""
		if len(stats) > 0 {
			statsStr = fmt.Sprintf(" (%s)", strings.Join(stats, ", "))
		}

		display := name
		if display == "" {
			display = github
		}
		if display == "" {
			display = email
		}

		// 👤 NAME (@GITHUB) stats
		return fmt.Sprintf("  %s %s %s%s\n",
			colorize("👤", ColorBlue, isTTY),
			colorize(display, ColorBold, isTTY),
			colorize("(@"+github+")", ColorDim, isTTY),
			colorize(statsStr, ColorDim, isTTY))
	}

	// Case 15: Single Policy Result
	if policy, ok := payload["policy"].(map[string]interface{}); ok {
		id, _ := policy["id"].(string)
		desc, _ := policy["description"].(string)
		kinds, _ := policy["kinds"].([]interface{})

		kindsCount := len(kinds)
		kindsStr := ""
		if kindsCount > 0 {
			kindsStr = fmt.Sprintf(" (%d kinds)", kindsCount)
		}

		// 🛡️ ID DESCRIPTION (X kinds)
		return fmt.Sprintf("  %s %s %s%s\n",
			colorize("🛡️", ColorBlue, isTTY),
			colorize(id, ColorBold, isTTY),
			colorize(desc, ColorDim, isTTY),
			colorize(kindsStr, ColorDim, isTTY))
	}

	// Default: JSON dump
	jsonBytes, _ := json.Marshal(payload) // Remove whitespace
	jsonStr := string(jsonBytes)
	if len(jsonStr) > 120 {
		jsonStr = jsonStr[:117] + "..."
	}
	return fmt.Sprintf("%s %s\n", prefix, jsonStr)
}

func formatGoalTree(goalsRaw []interface{}, ticketsRaw []interface{}, isTTY bool) string {
	var sb strings.Builder

	// Define Types
	type TicketNode struct {
		ID, Slug, Status string
		GoalID, ParentID string
		Children         []*TicketNode
	}

	type GoalNode struct {
		ID, Title, Status  string
		DueDate, CreatedAt string
		Children           []*GoalNode
		Tickets            []*TicketNode
	}

	// Parse Goals
	goalMap := make(map[string]*GoalNode)

	for _, g := range goalsRaw {
		if gm, ok := g.(map[string]interface{}); ok {
			id, _ := gm["id"].(string)
			title, _ := gm["title"].(string)
			status, _ := gm["status"].(string)
			dueDate, _ := gm["dueDate"].(string)
			createdAt, _ := gm["createdAt"].(string)
			node := &GoalNode{ID: id, Title: title, Status: status, DueDate: dueDate, CreatedAt: createdAt}
			goalMap[id] = node
		}
	}

	// Build goal hierarchy from slash-separated IDs
	// e.g. "R26-02/RUNNING-SKETCHPAD" is a child of "R26-02"
	var rootGoals []*GoalNode
	for _, g := range goalMap {
		parentID := ""
		if idx := strings.LastIndex(g.ID, "/"); idx >= 0 {
			parentID = g.ID[:idx]
		}
		if parentID != "" {
			if parent, ok := goalMap[parentID]; ok {
				parent.Children = append(parent.Children, g)
			} else {
				rootGoals = append(rootGoals, g)
			}
		} else {
			rootGoals = append(rootGoals, g)
		}
	}

	// Sort goals by due date, then by creation date
	var sortGoals func(goals []*GoalNode)
	sortGoals = func(goals []*GoalNode) {
		sort.SliceStable(goals, func(i, j int) bool {
			a, b := goals[i], goals[j]
			if a.DueDate != b.DueDate {
				if a.DueDate == "" {
					return false
				}
				if b.DueDate == "" {
					return true
				}
				return a.DueDate < b.DueDate
			}
			if a.CreatedAt != b.CreatedAt {
				if a.CreatedAt == "" {
					return false
				}
				if b.CreatedAt == "" {
					return true
				}
				return a.CreatedAt < b.CreatedAt
			}
			return a.ID < b.ID
		})
		for _, g := range goals {
			if len(g.Children) > 0 {
				sortGoals(g.Children)
			}
		}
	}
	sortGoals(rootGoals)

	// Parse Tickets
	var allTickets []*TicketNode

	for _, t := range ticketsRaw {
		if tm, ok := t.(map[string]interface{}); ok {
			id, _ := tm["id"].(string)
			slug, _ := tm["slug"].(string)
			status, _ := tm["status"].(string)
			goalID, _ := tm["goal"].(string)
			parentID, _ := tm["parent"].(string)
			node := &TicketNode{ID: id, Slug: slug, Status: status, GoalID: goalID, ParentID: parentID}
			allTickets = append(allTickets, node)
		}
	}

	// Assign tickets to goals
	var noGoalTickets []*TicketNode
	for _, t := range allTickets {
		if t.GoalID != "" {
			if g, ok := goalMap[t.GoalID]; ok {
				g.Tickets = append(g.Tickets, t)
			} else {
				noGoalTickets = append(noGoalTickets, t)
			}
		} else {
			noGoalTickets = append(noGoalTickets, t)
		}
	}

	// Helper to nest tickets by parent
	nestTickets := func(nodes []*TicketNode) []*TicketNode {
		var roots []*TicketNode
		lookup := make(map[string]*TicketNode)
		for _, n := range nodes {
			lookup[n.ID] = n
		}
		for _, n := range nodes {
			if n.ParentID != "" && lookup[n.ParentID] != nil {
				lookup[n.ParentID].Children = append(lookup[n.ParentID].Children, n)
			} else {
				roots = append(roots, n)
			}
		}
		return roots
	}

	for _, g := range goalMap {
		g.Tickets = nestTickets(g.Tickets)
	}
	noGoalTickets = nestTickets(noGoalTickets)

	// Recursive print for tickets
	var printTickets func(nodes []*TicketNode, prefix string)
	printTickets = func(nodes []*TicketNode, prefix string) {
		for i, t := range nodes {
			isLast := i == len(nodes)-1
			marker := "├──"
			newPrefix := prefix + "│   "
			if isLast {
				marker = "└──"
				newPrefix = prefix + "    "
			}
			statusColor := ColorBlue
			statusIcon := "◯"
			statusLower := strings.ToLower(t.Status)
			if statusLower == "finished" || statusLower == "closed" {
				statusColor = ColorGreen
				statusIcon = "✓"
			}
			sb.WriteString(fmt.Sprintf("%s%s %s %s\n", prefix, marker, colorize(statusIcon, statusColor, isTTY), colorize(t.Slug, ColorBold, isTTY)))
			printTickets(t.Children, newPrefix)
		}
	}

	// Recursive print for goals (renders children goals and tickets within a prefix)
	var printGoalContents func(g *GoalNode, prefix string)
	printGoalContents = func(g *GoalNode, prefix string) {
		total := len(g.Children) + len(g.Tickets)
		idx := 0
		for _, child := range g.Children {
			idx++
			isLast := idx == total
			marker := "├──"
			newPrefix := prefix + "│   "
			if isLast {
				marker = "└──"
				newPrefix = prefix + "    "
			}
			childColor := ColorBlue
			if strings.ToLower(child.Status) == "closed" {
				childColor = ColorGreen
			}
			sb.WriteString(fmt.Sprintf("%s%s %s %s\n", prefix, marker, colorize("■", childColor, isTTY), colorize(child.Title, ColorBold, isTTY)))
			printGoalContents(child, newPrefix)
		}
		// Print tickets after child goals
		for i, t := range g.Tickets {
			idx++
			isLast := idx == total
			// If no children goals, use ticket index for isLast
			if len(g.Children) == 0 {
				isLast = i == len(g.Tickets)-1
			}
			marker := "├──"
			newPrefix := prefix + "│   "
			if isLast {
				marker = "└──"
				newPrefix = prefix + "    "
			}
			statusColor := ColorBlue
			statusIcon := "◯"
			statusLower := strings.ToLower(t.Status)
			if statusLower == "finished" || statusLower == "closed" {
				statusColor = ColorGreen
				statusIcon = "✓"
			}
			sb.WriteString(fmt.Sprintf("%s%s %s %s\n", prefix, marker, colorize(statusIcon, statusColor, isTTY), colorize(t.Slug, ColorBold, isTTY)))
			printTickets(t.Children, newPrefix)
		}
	}

	// Render root goals
	for _, g := range rootGoals {
		statusColor := ColorBlue
		if strings.ToLower(g.Status) == "closed" {
			statusColor = ColorGreen
		}
		sb.WriteString(fmt.Sprintf("%s %s\n", colorize("■", statusColor, isTTY), colorize(g.Title, ColorBold, isTTY)))
		printGoalContents(g, "")
	}

	if len(noGoalTickets) > 0 {
		sb.WriteString(fmt.Sprintf("%s %s\n", colorize("?", ColorDim, isTTY), colorize("No Goal", ColorDim, isTTY)))
		printTickets(noGoalTickets, "")
	}

	return sb.String()
}

type MarkdownRenderer struct{}

func (r MarkdownRenderer) Render(ctx context.Context, out, errOut io.Writer, stream <-chan Event) (int, error) {
	exitCode := 0
	for event := range stream {
		if event.Kind == KindDone && event.Done != nil {
			exitCode = event.Done.ExitCode
			continue
		}
		if event.Kind == KindResult && len(event.Data) > 0 {
			fmt.Fprint(out, formatMarkdownResult(event.Command, event.Data))
		}
		if event.Kind == KindError && event.Error != nil {
			fmt.Fprintf(errOut, "**Error: %s**\n", event.Error.Message)
			if event.Error.Detail != "" {
				fmt.Fprintf(errOut, "> %s\n", event.Error.Detail)
			}
		}
	}
	return exitCode, nil
}

func formatMarkdownResult(command string, data json.RawMessage) string {
	var raw map[string]interface{}
	if err := json.Unmarshal(data, &raw); err != nil {
		return fmt.Sprintf("```json\n%s\n```\n", string(data))
	}

	// Helper to handle GraphQL response wrappers
	var payload map[string]interface{} = raw
	if len(raw) == 1 {
		if d, ok := raw["data"].(map[string]interface{}); ok {
			payload = d
		}
	}

	var sb strings.Builder

	// Case 0: Direct Markdown
	if markdown, ok := payload["markdown"].(string); ok {
		return markdown
	}

	// Case 1: Analyze result
	if analyze, ok := payload["analyze"].(map[string]interface{}); ok {
		sb.WriteString("## Analysis Result\n\n")
		// metrics
		if metrics, ok := analyze["metrics"].(map[string]interface{}); ok {
			total := metrics["total"]
			autofixable := metrics["autofixable"]
			sb.WriteString(fmt.Sprintf("- **Total Violations**: %v\n", total))
			if autofixable != nil {
				sb.WriteString(fmt.Sprintf("- **Autofixable**: %v\n", autofixable))
			}
			sb.WriteString("\n")
		}

		// violations
		if violations, ok := analyze["violations"].([]interface{}); ok {
			for _, v := range violations {
				if vio, ok := v.(map[string]interface{}); ok {
					kind := ""
					if k, ok := vio["kind"].(map[string]interface{}); ok {
						kind = fmt.Sprintf("%v", k["id"])
					} else if kStr, ok := vio["kind"].(string); ok {
						kind = kStr
					}

					scope := fmt.Sprintf("%v", vio["scope"])
					line := fmt.Sprintf("%v", vio["line"])
					summary := fmt.Sprintf("%v", vio["summary"])
					// Try to extract policy id if available from kind id (e.g. policy:violation)
					// parts := strings.Split(kind, ":")

					// violationKind: "semiorepo://policies/{policy-id}/{path*}"
					// path is kind without policy prefix? logic is fuzzy, assume kind is the path
					// Actually prompt says: violationKind: "semiorepo://policies/{policy-id}/{path*}"

					// We don't have full context to reconstruct exact URIs for everything perfectly without assumptions.
					// Assuming kind is the full ID.

					// Construct a file URI?
					// file: "semiorepo://{bundle-id}/{path*}"
					// Scope is usually a file path. We need bundle-id.

					// For now, generate a simple markdown representation.
					sb.WriteString(fmt.Sprintf("- **[%s](semiorepo://VIOLATION/%s)** in `%s:%s`: %s\n", kind, Slugify(kind), scope, line, summary))
				}
			}
		}
		return sb.String()
	}

	// Case 6: Repo result (Tickets, Bundles)
	if repo, ok := payload["repo"].(map[string]interface{}); ok {
		hasContent := false
		// Tickets
		if tickets, ok := repo["tickets"].([]interface{}); ok {
			hasContent = true
			sb.WriteString(fmt.Sprintf("## Tickets (%d)\n\n", len(tickets)))
			for _, t := range tickets {
				if ticket, ok := t.(map[string]interface{}); ok {
					slug, _ := ticket["slug"].(string)
					status, _ := ticket["status"].(string)
					title, _ := ticket["title"].(string)
					dates, _ := ticket["date"].(map[string]interface{})
					createdStr, _ := dates["created"].(string)

					dateParsed, _ := time.Parse(time.RFC3339, createdStr)
					ticketUri := fmt.Sprintf("semiorepo://TICKET/%d/%02d/%02d/%s", dateParsed.Year(), dateParsed.Month(), dateParsed.Day(), Slugify(slug))

					statusIcon := "◯"
					if status == "finished" || status == "closed" {
						statusIcon = "✓"
					}

					sb.WriteString(fmt.Sprintf("- %s [%s](%s) (%s) - %s\n", statusIcon, title, ticketUri, slug, status))
				}
			}
			sb.WriteString("\n")
		}

		// Bundles
		if bundles, ok := repo["bundles"].([]interface{}); ok {
			hasContent = true
			sb.WriteString(fmt.Sprintf("## Bundles (%d)\n\n", len(bundles)))
			for _, b := range bundles {
				if bundle, ok := b.(map[string]interface{}); ok {
					name, _ := bundle["name"].(string)
					bundleUri := fmt.Sprintf("semiorepo://BUNDLE/%s", Slugify(name))
					sb.WriteString(fmt.Sprintf("- [📦 %s](%s)\n", name, bundleUri))
				}
			}
			sb.WriteString("\n")
		}

		if hasContent {
			return sb.String()
		}
	}

	// Case 6b: Single Ticket Stream Item
	if ticket, ok := payload["ticket"].(map[string]interface{}); ok {
		slug, _ := ticket["slug"].(string)
		status, _ := ticket["status"].(string)
		title, _ := ticket["title"].(string)
		dates, _ := ticket["date"].(map[string]interface{})
		createdStr, _ := dates["created"].(string)

		dateParsed, _ := time.Parse(time.RFC3339, createdStr)
		ticketUri := fmt.Sprintf("semiorepo://TICKET/%d/%02d/%02d/%s", dateParsed.Year(), dateParsed.Month(), dateParsed.Day(), Slugify(slug))

		statusIcon := "◯"
		if status == "finished" || status == "closed" {
			statusIcon = "✓"
		}

		return fmt.Sprintf("- %s [%s](%s) (%s) - %s\n", statusIcon, title, ticketUri, slug, status)
	}

	// Case 6c: Single Bundle Stream Item
	if bundle, ok := payload["bundle"].(map[string]interface{}); ok {
		name, _ := bundle["name"].(string)
		bundleUri := fmt.Sprintf("semiorepo://BUNDLE/%s", Slugify(name))
		return fmt.Sprintf("- [📦 %s](%s)\n", name, bundleUri)
	}

	// Case 11: Single Folder Result
	if folder, ok := payload["folder"].(map[string]interface{}); ok {
		path, _ := folder["path"].(string)
		kind, _ := folder["kind"].(string)
		uri := fmt.Sprintf("semiorepo://FOLDER/%s", Slugify(path))
		return fmt.Sprintf("[📂 %s](%s) (%s)\n", path, uri, kind)
	}

	// Case 12: Single File Result
	if file, ok := payload["file"].(map[string]interface{}); ok {
		path, _ := file["id"].(string)
		uri := fmt.Sprintf("semiorepo://FILE/%s", Slugify(path))
		sb.WriteString(fmt.Sprintf("[📄 %s](%s)\n", path, uri))

		// Sections
		if sections, ok := file["sections"].([]interface{}); ok {
			sb.WriteString(fmt.Sprintf("\n### Sections (%d)\n", len(sections)))
			var printSections func(items []interface{}, indent string)
			printSections = func(items []interface{}, indent string) {
				for _, item := range items {
					if sec, ok := item.(map[string]interface{}); ok {
						name, _ := sec["name"].(string)
						sb.WriteString(fmt.Sprintf("%s- %s\n", indent, name))

						if children, ok := sec["children"].([]interface{}); ok && len(children) > 0 {
							printSections(children, indent+"  ")
						}
					}
				}
			}
			printSections(sections, "")
		}

		// Definitions
		if definitions, ok := file["definitions"].([]interface{}); ok {
			sb.WriteString(fmt.Sprintf("\n### Definitions (%d)\n", len(definitions)))
			for _, d := range definitions {
				if def, ok := d.(map[string]interface{}); ok {
					name, _ := def["name"].(string)
					kind, _ := def["kind"].(string)
					sb.WriteString(fmt.Sprintf("- %s (%s)\n", name, kind))
				}
			}
		}
		return sb.String()
	}

	// Case 13: Single Goal Result
	if goal, ok := payload["goal"].(map[string]interface{}); ok {
		id, _ := goal["id"].(string)
		title, _ := goal["title"].(string)
		status, _ := goal["status"].(string)
		uri := fmt.Sprintf("semiorepo://GOAL/%s", Slugify(id))

		statusIcon := "◯"
		if status == "closed" || status == "finished" {
			statusIcon = "✓"
		}

		return fmt.Sprintf("%s [%s](%s) (%s)\n", statusIcon, title, uri, id)
	}

	// Case 14: Single Contributor Result
	if contrib, ok := payload["contributor"].(map[string]interface{}); ok {
		github, _ := contrib["github"].(string)
		uri := fmt.Sprintf("semiorepo://CONTRIBUTOR/%s", Slugify(github))
		name, _ := contrib["name"].(string)
		return fmt.Sprintf("[👤 %s](%s)\n", name, uri)
	}

	// Default fallback
	return fmt.Sprintf("Result: ```json\n%s\n```\n", string(data))
}

func renderStream(cmd *cobra.Command, config *Config, stream <-chan Event) error {
	var renderer StreamRenderer
	if config.JSON {
		renderer = NDJSONRenderer{}
	} else if config.Markdown {
		renderer = MarkdownRenderer{}
	} else {
		renderer = HumanRenderer{Verbose: config.Verbose}
	}

	exitCode, err := renderer.Render(cmd.Context(), cmd.OutOrStdout(), cmd.ErrOrStderr(), stream)
	if err != nil {
		return err
	}
	if exitCode != 0 {
		return ExitError{Code: exitCode}
	}
	return nil
}

func runGraphQL(cmd *cobra.Command, factory EngineFactory, config *Config, query string, variables map[string]interface{}) error {
	argsPayload := GraphQLArgs{Query: query, Variables: variables}
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
	request := Request{Command: CmdGraphQL, Args: payloadBytes, RepoRoot: config.Repo, Verbose: config.Verbose}
	stream := engine.Run(ctx, request)
	return renderStream(cmd, config, stream)
}

// #endregion CLI Renderers

// #endregion Cli Adapter

// #region GraphQL Types

type Node interface {
	IsNode()
	GetID() string
}

type DefinitionKind string

const (
	DefinitionKindImplementation DefinitionKind = "implementation"
	DefinitionKindInterface      DefinitionKind = "interface"
	DefinitionKindConstant       DefinitionKind = "constant"
)

func (e DefinitionKind) IsValid() bool {
	switch e {
	case DefinitionKindImplementation, DefinitionKindInterface, DefinitionKindConstant:
		return true
	}
	return false
}

func (e DefinitionKind) String() string {
	return string(e)
}

func DeriveDefinitionKind(rawKind string) DefinitionKind {
	switch rawKind {
	case "interface", "type", "trait":
		return DefinitionKindInterface
	case "constant", "enum":
		return DefinitionKindConstant
	default:
		return DefinitionKindImplementation
	}
}

type TicketStatus string

const (
	TicketStatusOpen   TicketStatus = "open"
	TicketStatusClosed TicketStatus = "closed"
)

func (e TicketStatus) IsValid() bool {
	switch e {
	case TicketStatusOpen, TicketStatusClosed:
		return true
	}
	return false
}

func (e TicketStatus) String() string {
	return string(e)
}

type ViolationPriority string

const (
	ViolationPriorityHigh   ViolationPriority = "high"
	ViolationPriorityMedium ViolationPriority = "medium"
	ViolationPriorityLow    ViolationPriority = "low"
)

func (e ViolationPriority) IsValid() bool {
	switch e {
	case ViolationPriorityHigh, ViolationPriorityMedium, ViolationPriorityLow:
		return true
	}
	return false
}

func (e ViolationPriority) String() string {
	return string(e)
}

var AllowedLLMs = []string{
	"opus-4-5",
	"opus-4",
	"sonnet-4-5",
	"sonnet-4",
	"haiku-4-5",
	"gemini-3-pro",
	"gemini-3-flash",
	"gpt-5-2",
	"gpt-5-2-codex",
	"gpt-5-mini",
	"swe-1-5",
}

var AllowedUIs = []string{
	"vscode",
	"copilot-chat",
	"windsurf",
	"windsurf-chat",
	"claude-code",
	"codex",
	"cursor",
	"cursor-chat",
	"antigravity",
	"antigravity-chat",
	"droid",
}

func NormalizeLLMSlug(llm string) string {
	return strings.ToLower(Slugify(llm))
}

func NormalizeUISlug(ui string) string {
	return strings.ToLower(Slugify(ui))
}

func ResolveAllowedLLM(llm string) (string, error) {
	llmSlug := NormalizeLLMSlug(llm)
	bestMatch := ""
	for _, allowed := range AllowedLLMs {
		if strings.Contains(llmSlug, NormalizeLLMSlug(allowed)) {
			if len(allowed) > len(bestMatch) {
				bestMatch = allowed
			}
		}
	}
	if bestMatch == "" {
		return "", fmt.Errorf("llm '%s' is not allowed. Please use one of: %s", llmSlug, strings.Join(AllowedLLMs, ", "))
	}
	return bestMatch, nil
}

func ResolveAllowedUI(ui string) (string, error) {
	uiSlug := NormalizeUISlug(ui)
	bestMatch := ""
	for _, allowed := range AllowedUIs {
		if strings.Contains(uiSlug, NormalizeUISlug(allowed)) {
			if len(allowed) > len(bestMatch) {
				bestMatch = allowed
			}
		}
	}
	if bestMatch == "" {
		return "", fmt.Errorf("ui '%s' is not allowed. Please use one of: %s", uiSlug, strings.Join(AllowedUIs, ", "))
	}
	return bestMatch, nil
}

type Position struct {
	Line   int `json:"line"`
	Column int `json:"column"`
}

type Range struct {
	Start Position `json:"start"`
	End   Position `json:"end"`
}

type LineMetrics struct {
	Added   int `yaml:"added" json:"added"`
	Removed int `yaml:"removed" json:"removed"`
}

type DiffLines struct {
	Added   []int
	Removed []int
}

type CountMetrics struct {
	Added   int `json:"added"`
	Updated int `json:"updated"`
	Removed int `json:"removed"`
}

type ContributorIcons struct {
	Avatar      *string `json:"avatar,omitempty"`
	AvatarRound *string `json:"avatarRound,omitempty"`
	Github      *string `json:"github,omitempty"`
}

type ContributorLink struct {
	Name string `json:"name"`
	URL  string `json:"url"`
}

type TicketDate struct {
	Created  time.Time  `json:"created"`
	Finished *time.Time `json:"finished,omitempty"`
}

type TicketSectionMetrics struct {
	Range       *Range       `json:"range,omitempty"`
	Definitions []string     `json:"definitions,omitempty"`
	Lines       *LineMetrics `json:"lines,omitempty"`
}

type TicketFileMetricsEntry struct {
	Path     string                          `json:"path"`
	Lines    *LineMetrics                    `json:"lines,omitempty"`
	Sections map[string]TicketSectionMetrics `json:"sections,omitempty"`
}

type AnalyzeMetrics struct {
	Total       int            `json:"total"`
	ByPriority  *PriorityCount `json:"byPriority"`
	Autofixable int            `json:"autofixable"`
}

type PriorityCount struct {
	High   int `json:"high"`
	Medium int `json:"medium"`
	Low    int `json:"low"`
}

type Repo struct {
	ID       string    `json:"id"`
	Name     string    `json:"name"`
	Path     string    `json:"path"`
	Projects []Project `json:"projects"`
	Bundles  []Bundle  `json:"bundles"`
}

func (r *Repo) IsNode()        {}
func (r *Repo) GetID() string  { return "repo:semio" }
func (r *Repo) GetURI() string { return "semiorepo://REPO/SEMIO" }

type ProjectKind string

const (
	ProjectKindUser           ProjectKind = "👤"
	ProjectKindInfrastructure ProjectKind = "🧰"
	ProjectKindResearch       ProjectKind = "🔬"
)

func (e ProjectKind) String() string {
	return string(e)
}

type BundleKind string

const (
	BundleKindLibrary BundleKind = "📚"
	BundleKindSchema  BundleKind = "🛂"
	BundleKindBinary  BundleKind = "⌨️"
	BundleKindUI      BundleKind = "🖱️"
	BundleKindSite    BundleKind = "🌐"
	BundleKindAssets  BundleKind = "🏪"
)

func (e BundleKind) IsValid() bool {
	switch e {
	case BundleKindLibrary, BundleKindSchema, BundleKindBinary, BundleKindUI, BundleKindSite, BundleKindAssets:
		return true
	}
	return false
}

func (e BundleKind) String() string {
	return string(e)
}

func DeriveProjectKind(name string) ProjectKind {
	switch name {
	case "semio":
		return ProjectKindUser
	case "semio-repo":
		return ProjectKindInfrastructure
	case "coda":
		return ProjectKindResearch
	}
	if strings.HasPrefix(name, "@") {
		return DeriveProjectKind(strings.TrimPrefix(name, "@"))
	}
	return ProjectKindUser
}

func DeriveBundleKind(name string, root string) BundleKind {
	normalized := normalizeBundleLabel(name)
	switch normalized {
	case "@semio/sqlite", "@semio/graphql", "@semio/json-schema", "@semio/openapi", "@semio/rdf":
		return BundleKindSchema
	case "@semio-repo/go", "@semio-repo/server":
		return BundleKindBinary
	case "@semio-repo/vscode", "@semio/grasshopper", "@semio/desktop", "@semio/sketchpad":
		return BundleKindUI
	case "@semio/play", "@semio/docs":
		return BundleKindSite
	case "@semio/icons", "@semio/assets", "@semio/logo", "@semio/images":
		return BundleKindAssets
	case "@semio/js", "@semio/go", "@semio/py", "@semio/rs", "@semio/net", "@semio/engine", "@semio/semio", "@semio/rb":
		return BundleKindLibrary
	}
	if strings.Contains(root, "sql/") || strings.Contains(root, "graphql/") || strings.Contains(root, "jsonschema/") || strings.Contains(root, "openapi/") || strings.Contains(root, "rdf/") {
		return BundleKindSchema
	}
	if strings.Contains(root, "assets/") || strings.Contains(root, "icons/") || strings.Contains(root, "images/") || strings.Contains(root, "logo/") {
		return BundleKindAssets
	}
	return BundleKindLibrary
}

type Project struct {
	Name    string      `json:"name"`
	Root    string      `json:"root"`
	Kind    ProjectKind `json:"kind"`
	Bundles []Bundle    `json:"bundles"`
}

func (p *Project) IsNode()        {}
func (p *Project) GetID() string  { return "project:" + p.Name }
func (p *Project) GetURI() string { return "semiorepo://PROJECT/" + strings.ToUpper(Slugify(p.Name)) }

type Bundle struct {
	Name        string     `json:"name"`
	Root        string     `json:"root"`
	SourceRoot  string     `json:"sourceRoot,omitempty"`
	ProjectName string     `json:"projectName"`
	Tags        []string   `json:"tags,omitempty"`
	Kind        BundleKind `json:"kind"`
	Packages    []Package  `json:"packages,omitempty"`
}

type Package struct {
	Name    string `json:"name"`
	Version string `json:"version"`
	Path    string `json:"path"`
	Kind    string `json:"kind"` // npm, go, cargo, pip, nuget
}

func (b *Bundle) IsNode() {}
func (b *Bundle) GetID() string {
	return "bundle:" + b.Name
}

func (b *Bundle) GetURI() string { return "semiorepo://BUNDLE/" + strings.ToUpper(Slugify(b.Name)) }

func normalizeBundleLabel(name string) string {
	if name == "" {
		return ""
	}
	if strings.HasPrefix(name, "@") {
		return name
	}
	if name == "vscode" {
		return "@semio-repo/vscode"
	}
	if name == "repo" {
		return "@semio-repo/go"
	}
	return "@semio/" + name
}

func normalizeBundleID(name string) string {
	return normalizeBundleLabel(name)
}

func bundlePathPrefix(name string) string {
	if name == "" {
		return ""
	}
	if name == "@semio-repo" {
		return ""
	}
	if strings.HasPrefix(name, "@semio/") {
		return strings.TrimPrefix(name, "@semio/") + "/"
	}
	return name + "/"
}

type FolderKind string

const (
	FolderKindOrganization FolderKind = "organization"
	FolderKindRequired     FolderKind = "required"
)

func (e FolderKind) IsValid() bool {
	switch e {
	case FolderKindOrganization, FolderKindRequired:
		return true
	}
	return false
}

func (e FolderKind) String() string {
	return string(e)
}

func DeriveFolderKind(path string) FolderKind {
	base := filepath.Base(path)
	parent := filepath.Dir(path)
	if parent == "." || parent == "" {
		orgFolders := []string{"js", "go", "py", "rs", "net", "rb", "sql", "graphql", "jsonschema", "openapi", "rdf", "assets", "examples", "scripts", "reports", "yak", "antlr", "peg", "liveblocks", "meta", "temp", "engineering", "dotnet"}
		for _, org := range orgFolders {
			if base == org {
				return FolderKindOrganization
			}
		}
	}
	requiredIndicators := []string{"package.json", "pyproject.toml", "go.mod", "Cargo.toml", "*.csproj", "*.sln"}
	for _, indicator := range requiredIndicators {
		pattern := filepath.Join(GetRootDir(), path, indicator)
		matches, _ := filepath.Glob(pattern)
		if len(matches) > 0 {
			return FolderKindRequired
		}
	}
	return FolderKindOrganization
}

func IsGeneratedFolder(path string) bool {
	parts := strings.Split(filepath.ToSlash(path), "/")
	for _, part := range parts {
		if part == "generated" || part == "dist" || part == "build" || part == "node_modules" || part == "__pycache__" || part == ".next" || part == "coverage" {
			return true
		}
	}
	generatedFolders := []string{
		"js/vscode/generated",
		"js/semio/generated",
	}
	normalized := filepath.ToSlash(path)
	for _, gen := range generatedFolders {
		if normalized == gen || strings.HasPrefix(normalized, gen+"/") {
			return true
		}
	}
	return false
}

type Folder struct {
	ID        string     `json:"id"`
	Path      string     `json:"path"`
	URI       string     `json:"uri"`
	Name      string     `json:"name"`
	ParentID  *string    `json:"parentId,omitempty"`
	BundleID  *string    `json:"bundleId,omitempty"`
	Kind      FolderKind `json:"kind"`
	Ignored   bool       `json:"ignored"`
	Generated bool       `json:"generated"`
}

func (f *Folder) IsNode()       {}
func (f *Folder) GetID() string { return "folder:" + f.Path }
func (f *Folder) GetURI() string { return "semiorepo://FOLDER/" + strings.ToUpper(Slugify(f.Path)) }

type File struct {
	ID        string  `json:"id"`
	Path      string  `json:"path"`
	URI       string  `json:"uri"`
	Name      string  `json:"name"`
	Extension string  `json:"extension"`
	FolderID  *string `json:"folderId,omitempty"`
	BundleID  *string `json:"bundleId,omitempty"`
	Kind      string  `json:"kind"`
	Ignored   bool    `json:"ignored"`
	Generated bool    `json:"generated"`
}

const (
	FileKindCode     = "code"
	FileKindScript   = "script"
	FileKindConfig   = "config"
	FileKindTest     = "test"
	FileKindDocs     = "docs"
	FileKindResource = "resource"
	FileKindLicense  = "license"
)

func DeriveFileKind(name string) string {
	ext := strings.ToLower(filepath.Ext(name))
	nameLower := strings.ToLower(name)

	if strings.Contains(nameLower, "license") {
		return FileKindLicense
	}

	if nameLower == "package-lock.json" || nameLower == "yarn.lock" || nameLower == "pnpm-lock.yaml" || nameLower == "go.sum" || nameLower == "uv.lock" {
		return FileKindConfig // Or Generated? Kind is usually about content type.
	}

	if strings.HasSuffix(nameLower, ".test.ts") || strings.HasSuffix(nameLower, "_test.go") || strings.HasSuffix(nameLower, ".spec.ts") || strings.HasPrefix(nameLower, "test_") {
		return FileKindTest
	}

	switch ext {
	case ".sh", ".bat", ".ps1":
		return FileKindScript
	case ".json", ".yaml", ".yml", ".toml", ".xml", ".ini", ".conf", ".config", ".gitignore", ".dockerignore", ".env":
		return FileKindConfig
	case ".md", ".txt", ".rst", ".adoc":
		return FileKindDocs
	case ".png", ".jpg", ".jpeg", ".gif", ".svg", ".ico", ".webp", ".ttf", ".woff", ".woff2", ".eot":
		return FileKindResource
	case ".ts", ".tsx", ".js", ".jsx", ".py", ".cs", ".go", ".c", ".cpp", ".h", ".hpp", ".rs", ".java", ".kt", ".swift":
		return FileKindCode
	case ".css", ".scss", ".less", ".sass", ".html", ".htm":
		return FileKindCode
	case ".sql", ".graphql", ".gql":
		return FileKindCode // Schema code
	}

	if strings.HasPrefix(nameLower, "dockerfile") || nameLower == "makefile" || nameLower == "justfile" || nameLower == "rakefile" {
		return FileKindConfig
	}

	return FileKindResource
}

func (f *File) IsNode() {}

func IsGenerated(path string) bool {
	base := strings.ToLower(filepath.Base(path))
	if base == "package-lock.json" || base == "yarn.lock" || base == "pnpm-lock.yaml" || base == "go.sum" || base == "uv.lock" {
		return true
	}
	if base == "ticket.json" || base == "goal.json" {
		return true
	}
	if strings.HasSuffix(base, ".generated.go") || strings.HasSuffix(base, ".pb.go") {
		return true
	}
	// "assets/semio/validation.json" is generated?
	if strings.Contains(path, "generated") { // risky
		// return true
	}
	return false
}

func IsSemanticallyIgnored(path string) bool {
	base := filepath.Base(path)
	if strings.HasPrefix(base, ".") && base != ".gitignore" && base != ".env" {
		// Dotfiles generally ignored, except specific configs
		return true
	}
	// "Currently some folders and files are just ignored (e.g. ... LICENSE.md files, json files, etc)."
	// User wants these NOT ignored/excluded, but maybe marked Ignored=true?
	// No, "Files kinds: ... license". Vscode has "codebase shortcuts" toggles.
	// I think LICENSE.md should NOT be ignored.
	// JSON files? Config.
	// So what IS ignored?
	// Maybe things like "dist", "build", "coverage", "__pycache__" if they show up (even if not gitignored).
	if base == "dist" || base == "build" || base == "coverage" || base == "__pycache__" || base == "node_modules" {
		return true
	}
	return false
}

func (f *File) GetID() string { return "file:" + f.Path }
func (f *File) GetURI() string { return "semiorepo://FILE/" + strings.ToUpper(Slugify(f.Path)) }

type Section struct {
	ID          string       `json:"id,omitempty"`
	Name        string       `json:"name"`
	Path        string       `json:"path,omitempty"`
	FilePath    string       `json:"filePath,omitempty"`
	StartLine   int          `json:"startLine"`
	EndLine     int          `json:"endLine"`
	StartIndex  int          `json:"startIndex"`
	EndIndex    int          `json:"endIndex"`
	Children    []Section    `json:"children,omitempty"`
	Definitions []Definition `json:"definitions,omitempty"`
}

func (s *Section) IsNode() {}
func (s *Section) GetID() string {
	if s.ID != "" {
		if strings.HasPrefix(s.ID, "section:") {
			return s.ID
		}
		return "section:" + s.ID
	}
	if s.FilePath != "" && s.Path != "" {
		return "section:" + s.FilePath + "#" + s.Path
	}
	return "section:" + s.Name
}

func (s *Section) GetURI() string {
	raw := ""
	if s.FilePath != "" && s.Path != "" {
		raw = s.FilePath + "#" + s.Path
	} else {
		raw = s.Name
	}
	return "semiorepo://SECTION/" + strings.ToUpper(Slugify(raw))
}

type Definition struct {
	ID          string         `json:"id,omitempty"`
	Name        string         `json:"name"`
	Kind        DefinitionKind `json:"kind"`
	FilePath    string         `json:"filePath,omitempty"`
	SectionPath string         `json:"sectionPath,omitempty"`
	StartLine   int            `json:"startLine"`
	EndLine     int            `json:"endLine"`
	StartIndex  int            `json:"startIndex"`
	EndIndex    int            `json:"endIndex"`
}

func (d *Definition) IsNode() {}
func (d *Definition) GetID() string {
	if d.ID != "" {
		if strings.HasPrefix(d.ID, "definition:") {
			return d.ID
		}
		return "definition:" + d.ID
	}
	if d.FilePath != "" {
		if d.SectionPath != "" {
			return "definition:" + d.FilePath + "#" + d.SectionPath + "§" + d.Name
		}
		return "definition:" + d.FilePath + "§" + d.Name
	}
	return "definition:" + d.Name
}

func (d *Definition) GetURI() string {
	raw := ""
	if d.FilePath != "" {
		if d.SectionPath != "" {
			raw = d.FilePath + "#" + d.SectionPath + "§" + d.Name
		} else {
			raw = d.FilePath + "§" + d.Name
		}
	} else {
		raw = d.Name
	}
	return "semiorepo://DEFINITION/" + strings.ToUpper(Slugify(raw))
}

type Contributor struct {
	Github        string                          `yaml:"github" json:"github"`
	Name          string                          `yaml:"name,omitempty" json:"name,omitempty"`
	Emails        []string                        `yaml:"emails,omitempty" json:"emails,omitempty"`
	Links         map[string]string               `yaml:"links,omitempty" json:"links,omitempty"`
	Contributions ContributorContributionsStorage `yaml:"contributions,omitempty" json:"contributions,omitempty"`
}

type ContributorContributionsTree struct {
	Commits []*Commit
	Tickets []*Ticket
	Bundles []*ContributorBundle
}

type ContributorBundle struct {
	Name    string
	Lines   LineMetrics
	Folders []*ContributorFolder
}

type ContributorFolder struct {
	Name  string
	Lines LineMetrics
	Files []*ContributorFile
}

type ContributorFile struct {
	Name     string
	Lines    LineMetrics
	Sections []*ContributorSection
}

type ContributorSection struct {
	Name        string
	Lines       LineMetrics
	Definitions []*ContributorDefinition
}

type ContributorDefinition struct {
	Name  string
	Lines LineMetrics
}

func (c *Contributor) IsNode()       {}
func (c *Contributor) GetID() string {
	raw := c.Name
	if raw == "" {
		raw = c.Github
	}
	return "contributor:" + raw
}

func (c *Contributor) GetURI() string { return "semiorepo://CONTRIBUTOR/" + strings.ToUpper(Slugify(c.Github)) }

type Commit struct {
	ID       string    `json:"id"`
	SHA      string    `json:"sha"`
	Title    string    `json:"title"`
	AuthorID *string   `json:"authorId,omitempty"`
	Date     time.Time `json:"date"`
}

func (c *Commit) IsNode() {}

// #region Drafts

type Draft struct {
	ID string `json:"id"`
}

func GetDraftsPath() string {
	return filepath.Join(GetRootDir(), ".semio-repo", "drafts")
}

func ListDrafts() ([]*Draft, error) {
	draftsDir := GetDraftsPath()
	if !IsDir(draftsDir) {
		return []*Draft{}, nil
	}
	entries, err := os.ReadDir(draftsDir)
	if err != nil {
		return nil, err
	}
	var drafts []*Draft
	for _, entry := range entries {
		if entry.IsDir() {
			drafts = append(drafts, &Draft{ID: entry.Name()})
		}
	}
	return drafts, nil
}

func CreateDraft(id string, files []string) (*Draft, error) {
	id = Slugify(id)
	if id == "" {
		return nil, fmt.Errorf("invalid draft id")
	}
	draftPath := filepath.Join(GetDraftsPath(), id)
	if IsDir(draftPath) {
		return nil, fmt.Errorf("draft already exists: %s", id)
	}
	if err := EnsureDir(draftPath); err != nil {
		return nil, err
	}
	for _, f := range files {
		src := f
		dst := filepath.Join(draftPath, filepath.Base(f))
		// We copy the files into the draft
		if err := CopyFile(src, dst); err != nil {
			// Clean up if copy fails
			os.RemoveAll(draftPath)
			return nil, fmt.Errorf("failed to copy file %s: %w", src, err)
		}
	}
	return &Draft{ID: id}, nil
}

func DeleteDraft(id string) error {
	draftPath := filepath.Join(GetDraftsPath(), id)
	if !IsDir(draftPath) {
		return nil // idempotent
	}
	return os.RemoveAll(draftPath)
}

// #endregion Drafts

func (c *Commit) GetID() string { return "commit:" + c.SHA }
func (c *Commit) GetURI() string { return "semiorepo://COMMIT/" + strings.ToUpper(Slugify(c.SHA)) }

type Ticket struct {
	Year          int               `json:"-" yaml:"-"`
	Month         int               `json:"-" yaml:"-"`
	Day           int               `json:"-" yaml:"-"`
	Slug          string            `json:"-" yaml:"-"`
	Title         string            `json:"title" yaml:"title"`
	Status        TicketStatus      `json:"status" yaml:"status"`
	Prompt        string            `json:"prompt" yaml:"prompt"`
	Summary       string            `json:"summary,omitempty" yaml:"summary,omitempty"`
	GitHub        *TicketGithubData `json:"github,omitempty" yaml:"github,omitempty"`
	Goal          string            `json:"goal,omitempty" yaml:"goal,omitempty"`
	Parent        string            `json:"parent,omitempty" yaml:"parent,omitempty"`
	Started       time.Time         `json:"started" yaml:"started"`
	Finished      *time.Time        `json:"finished,omitempty" yaml:"finished,omitempty"`
	Iterations    []TicketIteration `json:"iterations" yaml:"iterations"`
	FolderPath    string            `json:"-" yaml:"-"`
	JsonPath      string            `json:"-" yaml:"-"`
	TicketPath    string            `json:"-" yaml:"-"`
	ImportantPath string            `json:"-" yaml:"-"`
}

func (t *Ticket) IsNode() {}
func (t *Ticket) GetID() string {
	return fmt.Sprintf("ticket:%d/%02d/%02d/%s", t.Year, t.Month, t.Day, t.Slug)
}

func (t *Ticket) GetURI() string {
	return fmt.Sprintf("semiorepo://TICKET/%d/%02d/%02d/%s", t.Year, t.Month, t.Day, strings.ToUpper(Slugify(t.Slug)))
}

func (t *Ticket) GetTitle() string {
	return t.Title
}

func (t *Ticket) GetPrompt() string {
	if len(t.Iterations) > 0 {
		return t.Iterations[0].Prompt
	}
	return ""
}

func (t *Ticket) GetLatestPrompt() string {
	if len(t.Iterations) > 0 {
		return t.Iterations[len(t.Iterations)-1].Prompt
	}
	return ""
}

func (t *Ticket) GetLLM() string {
	if len(t.Iterations) > 0 {
		return t.Iterations[len(t.Iterations)-1].LLM
	}
	return ""
}

func (t *Ticket) GetUI() string {
	if len(t.Iterations) > 0 {
		return t.Iterations[len(t.Iterations)-1].UI
	}
	return ""
}

func (t *Ticket) GetStatus() TicketStatus {
	return t.Status
}

func (t *Ticket) GetAuthor() string {
	if len(t.Iterations) > 0 {
		return t.Iterations[0].Author.String()
	}
	return ""
}

func (t *Ticket) GetCommit() string {
	if len(t.Iterations) > 0 {
		return t.Iterations[0].Commit
	}
	return ""
}

func (t *Ticket) GetSummary() string {
	return t.Summary
}

func (t *Ticket) GetDateStarted() time.Time {
	return t.Started
}

func (t *Ticket) GetDateFinished() *time.Time {
	return t.Finished
}

func (t *Ticket) GetFiles() *TicketDiffs {
	result := newTicketDiffs()
	for _, iteration := range t.Iterations {
		if iteration.Diff == nil {
			continue
		}
		result.Bundles.Added = append(result.Bundles.Added, iteration.Diff.Bundles.Added...)
		result.Bundles.Modified = append(result.Bundles.Modified, iteration.Diff.Bundles.Modified...)
		result.Bundles.Deleted = append(result.Bundles.Deleted, iteration.Diff.Bundles.Deleted...)
		result.Bundles.Renamed = append(result.Bundles.Renamed, iteration.Diff.Bundles.Renamed...)
		result.Folders.Added = append(result.Folders.Added, iteration.Diff.Folders.Added...)
		result.Folders.Modified = append(result.Folders.Modified, iteration.Diff.Folders.Modified...)
		result.Folders.Deleted = append(result.Folders.Deleted, iteration.Diff.Folders.Deleted...)
		result.Folders.Renamed = append(result.Folders.Renamed, iteration.Diff.Folders.Renamed...)
		result.Files.Added = append(result.Files.Added, iteration.Diff.Files.Added...)
		result.Files.Modified = append(result.Files.Modified, iteration.Diff.Files.Modified...)
		result.Files.Deleted = append(result.Files.Deleted, iteration.Diff.Files.Deleted...)
		result.Files.Renamed = append(result.Files.Renamed, iteration.Diff.Files.Renamed...)
		result.Sections.Added = append(result.Sections.Added, iteration.Diff.Sections.Added...)
		result.Sections.Modified = append(result.Sections.Modified, iteration.Diff.Sections.Modified...)
		result.Sections.Deleted = append(result.Sections.Deleted, iteration.Diff.Sections.Deleted...)
		result.Sections.Renamed = append(result.Sections.Renamed, iteration.Diff.Sections.Renamed...)
		result.Definitions.Added = append(result.Definitions.Added, iteration.Diff.Definitions.Added...)
		result.Definitions.Modified = append(result.Definitions.Modified, iteration.Diff.Definitions.Modified...)
		result.Definitions.Deleted = append(result.Definitions.Deleted, iteration.Diff.Definitions.Deleted...)
		result.Definitions.Renamed = append(result.Definitions.Renamed, iteration.Diff.Definitions.Renamed...)
	}
	return result
}

type TicketBundleContrib struct {
	BundleID string              `json:"bundleId"`
	Files    []TicketFileContrib `json:"files"`
}

type TicketFileContrib struct {
	FileID   string                 `json:"fileId"`
	Sections []TicketSectionContrib `json:"sections"`
}

type TicketSectionContrib struct {
	SectionID   string       `json:"sectionId"`
	Definitions []string     `json:"definitions"`
	Metrics     *LineMetrics `json:"metrics"`
}

type Policy struct {
	ID             string               `json:"id"`
	Name           string               `json:"name"`
	Description    *string              `json:"description,omitempty"`
	Scopes         []string             `json:"scopes"`
	ViolationKinds []*ViolationKindMeta `json:"violationKinds"`
}

func (p *Policy) IsNode()       {}
func (p *Policy) GetID() string { return "policy:" + p.ID }
func (p *Policy) GetURI() string { return "semiorepo://POLICY/" + strings.ToUpper(Slugify(p.ID)) }

type ViolationKindMeta struct {
	Kind        ViolationKind     `json:"kind"`
	PolicyID    string            `json:"policyId"`
	Priority    ViolationPriority `json:"priority"`
	Reason      string            `json:"reason"`
	Solution    string            `json:"solution"`
	Autofixable bool              `json:"autofixable"`
}

func (v *ViolationKindMeta) IsNode() {}
func (v *ViolationKindMeta) GetID() string {
	return "violation_kind:" + string(v.Kind)
}

func (v *ViolationKindMeta) GetURI() string { return "semiorepo://VIOLATION_KIND/" + strings.ToUpper(Slugify(string(v.Kind))) }

type AnalyzeResult struct {
	Violations []*Violation    `json:"violations"`
	Metrics    *AnalyzeMetrics `json:"metrics"`
}

type FixResult struct {
	Fixed      int          `json:"fixed"`
	Remaining  int          `json:"remaining"`
	Violations []*Violation `json:"violations"`
}

type ContributorContributions struct {
	Bundles     []ContributionBundle     `json:"bundles"`
	Folders     []ContributionFolder     `json:"folders"`
	Files       []ContributionFile       `json:"files"`
	Sections    []ContributionSection    `json:"sections"`
	Definitions []ContributionDefinition `json:"definitions"`
}

type ContributionBundle struct {
	BundleID string        `json:"bundleId"`
	Metrics  *CountMetrics `json:"metrics"`
}

type ContributionFolder struct {
	FolderID string        `json:"folderId"`
	Metrics  *CountMetrics `json:"metrics"`
}

type ContributionFile struct {
	FileID  string       `json:"fileId"`
	Metrics *LineMetrics `json:"metrics"`
}

type ContributionSection struct {
	SectionID string       `json:"sectionId"`
	Metrics   *LineMetrics `json:"metrics"`
}

type ContributionDefinition struct {
	DefinitionID string       `json:"definitionId"`
	Metrics      *LineMetrics `json:"metrics"`
}

type SemanticChangeType string

const (
	SemanticChangeAdded    SemanticChangeType = "added"
	SemanticChangeDeleted  SemanticChangeType = "deleted"
	SemanticChangeModified SemanticChangeType = "modified"
	SemanticChangeRenamed  SemanticChangeType = "renamed"
)

type SemanticChange struct {
	Kind     string
	Status   SemanticChangeType
	Path     string
	FromPath string
	ToPath   string
	Lines    LineMetrics
}

func newTicketDiffSet() TicketDiffSet {
	return TicketDiffSet{
		Deleted:  []TicketFile{},
		Renamed:  []TicketFileRenamed{},
		Modified: []TicketFile{},
		Added:    []TicketFile{},
	}
}

func newTicketDiffs() *TicketDiffs {
	return &TicketDiffs{
		Bundles:     newTicketDiffSet(),
		Folders:     newTicketDiffSet(),
		Files:       newTicketDiffSet(),
		Sections:    newTicketDiffSet(),
		Definitions: newTicketDiffSet(),
	}
}

func addTicketDiffEntry(set *TicketDiffSet, change SemanticChange) {
	lines := &LineMetrics{Added: change.Lines.Added, Removed: change.Lines.Removed}
	switch change.Status {
	case SemanticChangeAdded:
		set.Added = append(set.Added, TicketFile{Path: change.Path, Lines: lines})
	case SemanticChangeDeleted:
		set.Deleted = append(set.Deleted, TicketFile{Path: change.Path, Lines: lines})
	case SemanticChangeRenamed:
		set.Renamed = append(set.Renamed, TicketFileRenamed{From: change.FromPath, To: change.ToPath, Lines: lines})
	case SemanticChangeModified:
		set.Modified = append(set.Modified, TicketFile{Path: change.Path, Lines: lines})
	}
}

func mergeLineMetrics(target *LineMetrics, add LineMetrics) {
	if target == nil {
		return
	}
	target.Added += add.Added
	target.Removed += add.Removed
}

func computeLineMetricsForDiff(diff *DiffLines, baseCommit, filePath string) LineMetrics {
	if diff == nil {
		return LineMetrics{}
	}
	if len(diff.Added) > 0 && len(diff.Removed) == 0 {
		return LineMetrics{Added: CountLinesInFile(filepath.Join(GetRootDir(), filePath)), Removed: 0}
	}
	if len(diff.Removed) > 0 && len(diff.Added) == 0 {
		return LineMetrics{Added: 0, Removed: CountLinesAtCommit(baseCommit, filePath)}
	}
	return LineMetrics{Added: len(diff.Added), Removed: len(diff.Removed)}
}

func buildCodebasePathSet(codebase *Codebase) map[string]struct{} {
	result := make(map[string]struct{})
	if codebase == nil {
		return result
	}
	for _, bundle := range codebase.Bundles {
		result[bundle.ID] = struct{}{}
	}
	for _, folder := range codebase.Folders {
		result[folder.Path] = struct{}{}
	}
	for _, file := range codebase.Files {
		result[file.Path] = struct{}{}
	}
	for _, section := range codebase.Sections {
		result[section.Path] = struct{}{}
	}
	for _, def := range codebase.Definitions {
		result[def.Path] = struct{}{}
	}
	return result
}

func buildFolderLineTotals(files []string, baseCommit string, bundles []Bundle) (map[string]int, map[string]int) {
	currentTotals := make(map[string]int)
	baseTotals := make(map[string]int)
	ctx := &CodebaseContext{Bundles: bundles}
	for _, file := range files {
		folderPath := NormalizePath(filepath.Dir(file))
		if folderPath == "." {
			continue
		}
		id := ctx.GetFolderID(folderPath)
		currentTotals[id] += CountLinesInFile(filepath.Join(GetRootDir(), file))
		baseTotals[id] += CountLinesAtCommit(baseCommit, file)
	}
	return currentTotals, baseTotals
}

func buildBundleLineTotals(files []string, baseCommit string, bundles []Bundle) (map[string]int, map[string]int) {
	currentTotals := make(map[string]int)
	baseTotals := make(map[string]int)
	ctx := &CodebaseContext{Bundles: bundles}
	for _, file := range files {
		if file == "README.md" || file == "AGENTS.md" {
			continue
		}
		bundleName := ctx.GetBundleForFile(file)
		currentTotals[bundleName] += CountLinesInFile(filepath.Join(GetRootDir(), file))
		baseTotals[bundleName] += CountLinesAtCommit(baseCommit, file)
	}
	return currentTotals, baseTotals
}

func extractFilePrefix(path string) string {
	if path == "" {
		return ""
	}
	if idx := strings.Index(path, "#"); idx != -1 {
		return path[:idx]
	}
	if idx := strings.Index(path, "§"); idx != -1 {
		return path[:idx]
	}
	return path
}

func reconcileRenamePairs(diffSet *TicketDiffSet, matchKey func(path string) string) {
	if diffSet == nil {
		return
	}
	usedAdded := make(map[int]struct{})
	var renamed []TicketFileRenamed
	var remainingDeleted []TicketFile
	for _, del := range diffSet.Deleted {
		matchIndex := -1
		key := matchKey(del.Path)
		for i, add := range diffSet.Added {
			if _, ok := usedAdded[i]; ok {
				continue
			}
			if key != "" && matchKey(add.Path) != key {
				continue
			}
			removedLines := 0
			if del.Lines != nil {
				removedLines = del.Lines.Removed
			}
			addedLines := 0
			if add.Lines != nil {
				addedLines = add.Lines.Added
			}
			if removedLines > 0 && addedLines > 0 && removedLines != addedLines {
				continue
			}
			matchIndex = i
			break
		}
		if matchIndex == -1 {
			remainingDeleted = append(remainingDeleted, del)
			continue
		}
		add := diffSet.Added[matchIndex]
		usedAdded[matchIndex] = struct{}{}
		lines := &LineMetrics{}
		if add.Lines != nil {
			lines.Added = add.Lines.Added
		}
		if del.Lines != nil {
			lines.Removed = del.Lines.Removed
		}
		renamed = append(renamed, TicketFileRenamed{From: del.Path, To: add.Path, Lines: lines})
	}
	var remainingAdded []TicketFile
	for i, add := range diffSet.Added {
		if _, ok := usedAdded[i]; ok {
			continue
		}
		remainingAdded = append(remainingAdded, add)
	}
	diffSet.Added = remainingAdded
	diffSet.Deleted = remainingDeleted
	diffSet.Renamed = append(diffSet.Renamed, renamed...)
}

func buildSectionDiffs(baseCodebase, currentCodebase *Codebase, baseCommit string, diffLines map[string]*DiffLines, bundles []Bundle) TicketDiffSet {
	result := newTicketDiffSet()
	ctx := &CodebaseContext{Bundles: bundles}
	currentSectionMap := make(map[string]CodebaseSection)
	baseSectionMap := make(map[string]CodebaseSection)
	// ... (rest of the map building)
	for _, section := range currentCodebase.Sections {
		currentSectionMap[section.Path] = section
	}
	for _, section := range baseCodebase.Sections {
		baseSectionMap[section.Path] = section
	}

	for path := range currentSectionMap {
		if _, ok := baseSectionMap[path]; !ok {
			lines := 0
			if currentSectionMap[path].Metrics != nil {
				lines = currentSectionMap[path].Metrics.Lines
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "section", Status: SemanticChangeAdded, Path: path, Lines: LineMetrics{Added: lines}})
		}
	}
	for path := range baseSectionMap {
		if _, ok := currentSectionMap[path]; !ok {
			lines := 0
			if baseSectionMap[path].Metrics != nil {
				lines = baseSectionMap[path].Metrics.Lines
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "section", Status: SemanticChangeDeleted, Path: path, Lines: LineMetrics{Removed: lines}})
		}
	}

	for filePath, diff := range diffLines {
		if diff == nil {
			continue
		}
		fileID := ctx.GetFileID(filePath)
		content, err := ReadTextFile(filepath.Join(GetRootDir(), filePath))
		if err != nil {
			continue
		}
		baseContent, err := ReadTextFileAtCommit(baseCommit, filePath)
		if err != nil {
			continue
		}
		lang := GetLanguage(filePath)
		if lang == nil || !lang.SupportsSections() {
			continue
		}
		currentSections := lang.ParseSections(content)
		baseSections := lang.ParseSections(baseContent)
		addedMap := computeSectionLineMap(currentSections, diff.Added, "")
		removedMap := computeSectionLineMap(baseSections, diff.Removed, "")
		for sectionPath, addedLines := range addedMap {
			removedLines := removedMap[sectionPath]
			if len(addedLines) == 0 && len(removedLines) == 0 {
				continue
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "section", Status: SemanticChangeModified, Path: fileID + "#" + sectionPath, Lines: LineMetrics{Added: len(addedLines), Removed: len(removedLines)}})
		}
		for sectionPath, removedLines := range removedMap {
			if _, ok := addedMap[sectionPath]; ok {
				continue
			}
			if len(removedLines) == 0 {
				continue
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "section", Status: SemanticChangeModified, Path: fileID + "#" + sectionPath, Lines: LineMetrics{Removed: len(removedLines)}})
		}
	}
	reconcileRenamePairs(&result, func(path string) string {
		return extractFilePrefix(path)
	})

	return result
}

func buildDefinitionDiffs(baseCodebase, currentCodebase *Codebase, baseCommit string, diffLines map[string]*DiffLines, bundles []Bundle) TicketDiffSet {
	result := newTicketDiffSet()
	ctx := &CodebaseContext{Bundles: bundles}
	currentDefMap := make(map[string]CodebaseDefinition)
	baseDefMap := make(map[string]CodebaseDefinition)
	// ... (rest of the map building)
	for _, def := range currentCodebase.Definitions {
		currentDefMap[def.Path] = def
	}
	for _, def := range baseCodebase.Definitions {
		baseDefMap[def.Path] = def
	}

	for path := range currentDefMap {
		if _, ok := baseDefMap[path]; !ok {
			lines := 0
			if currentDefMap[path].Metrics != nil {
				lines = currentDefMap[path].Metrics.Lines
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "definition", Status: SemanticChangeAdded, Path: path, Lines: LineMetrics{Added: lines}})
		}
	}
	for path := range baseDefMap {
		if _, ok := currentDefMap[path]; !ok {
			lines := 0
			if baseDefMap[path].Metrics != nil {
				lines = baseDefMap[path].Metrics.Lines
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "definition", Status: SemanticChangeDeleted, Path: path, Lines: LineMetrics{Removed: lines}})
		}
	}

	for filePath, diff := range diffLines {
		if diff == nil {
			continue
		}
		fileID := ctx.GetFileID(filePath)
		content, err := ReadTextFile(filepath.Join(GetRootDir(), filePath))
		if err != nil {
			continue
		}
		baseContent, err := ReadTextFileAtCommit(baseCommit, filePath)
		if err != nil {
			continue
		}
		lang := GetLanguage(filePath)
		if lang == nil || !lang.SupportsDefinitions() {
			continue
		}
		currentLines := strings.Split(content, "\n")
		baseLines := strings.Split(baseContent, "\n")
		currentDefs := lang.ParseDefinitions(content, currentLines)
		baseDefs := lang.ParseDefinitions(baseContent, baseLines)
		currentSections := lang.ParseSections(content)
		baseSections := lang.ParseSections(baseContent)
		for _, def := range currentDefs {
			addedLines := computeLinesInRange(diff.Added, def.Start, def.End)
			removedLines := computeLinesInRange(diff.Removed, def.Start, def.End)
			if len(addedLines) == 0 && len(removedLines) == 0 {
				continue
			}
			sectionPath := findSectionForDefinition(currentSections, def.Start, def.End, "")
			defPath := fileID + "§" + def.Name
			if sectionPath != "" {
				defPath = fileID + "#" + sectionPath + "§" + def.Name
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "definition", Status: SemanticChangeModified, Path: defPath, Lines: LineMetrics{Added: len(addedLines), Removed: len(removedLines)}})
		}
		for _, def := range baseDefs {
			removedLines := computeLinesInRange(diff.Removed, def.Start, def.End)
			if len(removedLines) == 0 {
				continue
			}
			sectionPath := findSectionForDefinition(baseSections, def.Start, def.End, "")
			defPath := fileID + "§" + def.Name
			if sectionPath != "" {
				defPath = fileID + "#" + sectionPath + "§" + def.Name
			}
			addTicketDiffEntry(&result, SemanticChange{Kind: "definition", Status: SemanticChangeModified, Path: defPath, Lines: LineMetrics{Removed: len(removedLines)}})
		}
	}
	reconcileRenamePairs(&result, func(path string) string {
		return extractFilePrefix(path)
	})

	return result
}

func BuildSemanticDiffs(baseCodebase, currentCodebase *Codebase, baseCommit string, diffLines map[string]*DiffLines, diffStatuses []GitDiffStatus, bundles []Bundle) *TicketDiffs {
	result := newTicketDiffs()
	ctx := &CodebaseContext{Bundles: bundles}

	currentFilesSet := make(map[string]struct{})
	baseFilesSet := make(map[string]struct{})
	for filePath := range diffLines {
		currentFilesSet[filePath] = struct{}{}
		baseFilesSet[filePath] = struct{}{}
	}
	for _, status := range diffStatuses {
		if status.From != "" {
			baseFilesSet[status.From] = struct{}{}
		}
		if status.To != "" {
			currentFilesSet[status.To] = struct{}{}
		}
	}
	var currentFiles []string
	for f := range currentFilesSet {
		currentFiles = append(currentFiles, f)
	}
	var baseFiles []string
	for f := range baseFilesSet {
		baseFiles = append(baseFiles, f)
	}

	currentFolderLines, _ := buildFolderLineTotals(currentFiles, baseCommit, bundles)
	_, baseFolderLines := buildFolderLineTotals(baseFiles, baseCommit, bundles)
	currentBundleLines, _ := buildBundleLineTotals(currentFiles, baseCommit, bundles)
	_, baseBundleLines := buildBundleLineTotals(baseFiles, baseCommit, bundles)

	currentBundleMap := make(map[string]CodebaseBundle)
	baseBundleMap := make(map[string]CodebaseBundle)
	if currentCodebase != nil {
		for _, bundle := range currentCodebase.Bundles {
			currentBundleMap[bundle.ID] = bundle
		}
	}
	if baseCodebase != nil {
		for _, bundle := range baseCodebase.Bundles {
			baseBundleMap[bundle.ID] = bundle
		}
	}
	for id, bundle := range currentBundleMap {
		if _, ok := baseBundleMap[id]; !ok {
			lines := LineMetrics{Added: currentBundleLines[id], Removed: 0}
			addTicketDiffEntry(&result.Bundles, SemanticChange{Kind: "bundle", Status: SemanticChangeAdded, Path: normalizeBundleLabel(id), Lines: lines})
		} else if bundle.Metrics != nil {
			baseBundle := baseBundleMap[id]
			if baseBundle.Metrics != nil {
				added := currentBundleLines[id] - baseBundleLines[id]
				removed := 0
				if added < 0 {
					removed = -added
					added = 0
				}
				if added > 0 || removed > 0 {
					addTicketDiffEntry(&result.Bundles, SemanticChange{Kind: "bundle", Status: SemanticChangeModified, Path: normalizeBundleLabel(id), Lines: LineMetrics{Added: added, Removed: removed}})
				}
			}
		}
	}
	for id := range baseBundleMap {
		if _, ok := currentBundleMap[id]; !ok {
			lines := LineMetrics{Added: 0, Removed: baseBundleLines[id]}
			addTicketDiffEntry(&result.Bundles, SemanticChange{Kind: "bundle", Status: SemanticChangeDeleted, Path: normalizeBundleLabel(id), Lines: lines})
		}
	}

	currentFolderMap := make(map[string]CodebaseFolder)
	baseFolderMap := make(map[string]CodebaseFolder)
	if currentCodebase != nil {
		for _, folder := range currentCodebase.Folders {
			currentFolderMap[folder.Path] = folder
		}
	}
	if baseCodebase != nil {
		for _, folder := range baseCodebase.Folders {
			baseFolderMap[folder.Path] = folder
		}
	}
	for path := range currentFolderMap {
		if _, ok := baseFolderMap[path]; !ok {
			addTicketDiffEntry(&result.Folders, SemanticChange{Kind: "folder", Status: SemanticChangeAdded, Path: path, Lines: LineMetrics{Added: currentFolderLines[path]}})
		}
	}
	for path := range baseFolderMap {
		if _, ok := currentFolderMap[path]; !ok {
			addTicketDiffEntry(&result.Folders, SemanticChange{Kind: "folder", Status: SemanticChangeDeleted, Path: path, Lines: LineMetrics{Removed: baseFolderLines[path]}})
		}
	}

	for _, status := range diffStatuses {
		if status.Status == "renamed" {
			fromFolder := NormalizePath(filepath.Dir(status.From))
			toFolder := NormalizePath(filepath.Dir(status.To))
			fromFolderID := ctx.GetFolderID(fromFolder)
			toFolderID := ctx.GetFolderID(toFolder)
			if fromFolderID != toFolderID && fromFolder != "." && toFolder != "." {
				addTicketDiffEntry(&result.Folders, SemanticChange{Kind: "folder", Status: SemanticChangeRenamed, FromPath: fromFolderID, ToPath: toFolderID, Lines: LineMetrics{Added: currentFolderLines[toFolderID], Removed: baseFolderLines[fromFolderID]}})
			}
			fromFileID := ctx.GetFileID(status.From)
			toFileID := ctx.GetFileID(status.To)
			addTicketDiffEntry(&result.Files, SemanticChange{Kind: "file", Status: SemanticChangeRenamed, FromPath: fromFileID, ToPath: toFileID, Lines: LineMetrics{Added: CountLinesInFile(filepath.Join(GetRootDir(), status.To)), Removed: CountLinesAtCommit(baseCommit, status.From)}})
		}
	}

	for filePath, diff := range diffLines {
		fileID := ctx.GetFileID(filePath)
		metrics := computeLineMetricsForDiff(diff, baseCommit, filePath)
		status := SemanticChangeModified
		if len(diff.Added) > 0 && len(diff.Removed) == 0 {
			status = SemanticChangeAdded
		} else if len(diff.Removed) > 0 && len(diff.Added) == 0 {
			status = SemanticChangeDeleted
		}
		addTicketDiffEntry(&result.Files, SemanticChange{Kind: "file", Status: status, Path: fileID, Lines: metrics})
	}

	reconcileRenamePairs(&result.Bundles, func(path string) string {
		return path
	})

	result.Sections = buildSectionDiffs(baseCodebase, currentCodebase, baseCommit, diffLines, bundles)
	result.Definitions = buildDefinitionDiffs(baseCodebase, currentCodebase, baseCommit, diffLines, bundles)

	return result
}

// #region GraphQL Input Types

type FileListInput struct {
	Updated []string `json:"updated,omitempty"`
	Created []string `json:"created,omitempty"`
	Removed []string `json:"removed,omitempty"`
}

type TicketOpenInput struct {
	Title    string `json:"title"`
	Prompt   string `json:"prompt"`
	LLM      string `json:"llm,omitempty"`
	UI       string `json:"client"`
	NoIssue  bool   `json:"noIssue,omitempty"`
	Draft    string `json:"draft,omitempty"`
	Goal     string `json:"goal,omitempty"`
	Parent   string `json:"parent,omitempty"`
	NoGithub bool   `json:"noGithub,omitempty"`
	Issue    string `json:"issue,omitempty"` // Link to existing GitHub issue URL instead of creating new one
}

type DraftCreateInput struct {
	Slug  string   `json:"slug"`
	Files []string `json:"files,omitempty"`
}

type TicketProgressInput struct {
	Year    int    `json:"year"`
	Month   int    `json:"month"`
	Day     int    `json:"day"`
	Slug    string `json:"slug"`
	Summary string `json:"summary,omitempty"`
}

type GoalCreateInput struct {
	Title       string `json:"title"`
	Description string `json:"description"`
	Prompt      string `json:"prompt"`
	DueDate     string `json:"dueDate"`
	LLM         string `json:"llm"`
	UI          string `json:"client"`
	NoGithub    bool   `json:"noGithub,omitempty"`
	Parent      string `json:"parent,omitempty"`
	Milestone   string `json:"milestone,omitempty"` // Link to existing GitHub milestone URL instead of creating new one
}

type GoalChangeInput struct {
	ID          string  `json:"id"`
	Title       *string `json:"title,omitempty"`
	Description *string `json:"description,omitempty"`
	DueDate     *string `json:"dueDate,omitempty"`
	Parent      *string `json:"parent,omitempty"`
	NoGithub    bool    `json:"noGithub,omitempty"`
}

type GoalCloseInput struct {
	ID       string `json:"id"`
	Summary  string `json:"summary"`
	NoGithub bool   `json:"noGithub,omitempty"`
}

type GoalReopenInput struct {
	ID          string  `json:"id"`
	Prompt      string  `json:"prompt"`
	UI          string  `json:"client"`
	LLM         string  `json:"llm"`
	Title       *string `json:"title,omitempty"`
	Description *string `json:"description,omitempty"`
	DueDate     *string `json:"dueDate,omitempty"`
	Parent      *string `json:"parent,omitempty"`
	NoGithub    bool    `json:"noGithub,omitempty"`
}

type GoalDeleteInput struct {
	ID       string `json:"id"`
	NoGithub bool   `json:"noGithub,omitempty"`
}

type TicketDeleteInput struct {
	Year     int    `json:"year"`
	Month    int    `json:"month"`
	Day      int    `json:"day"`
	Slug     string `json:"slug"`
	NoGithub bool   `json:"noGithub,omitempty"`
}

type TicketCloseInput struct {
	Year     int      `json:"year"`
	Month    int      `json:"month"`
	Day      int      `json:"day"`
	Slug     string   `json:"slug"`
	Summary  string   `json:"summary"`
	Files    []string `json:"files"`
	Title    *string  `json:"title,omitempty"`
	NoGithub bool     `json:"noGithub,omitempty"`
	All      bool     `json:"all,omitempty"`
}

type TicketReopenInput struct {
	Year     int     `json:"year"`
	Month    int     `json:"month"`
	Day      int     `json:"day"`
	Slug     string  `json:"slug"`
	Prompt   string  `json:"prompt"`
	LLM      string  `json:"llm,omitempty"`
	UI       string  `json:"client"`
	Title    *string `json:"title,omitempty"`
	Draft    string  `json:"draft,omitempty"`
	Goal     string  `json:"goal,omitempty"`
	Parent   string  `json:"parent,omitempty"`
	NoGithub bool    `json:"noGithub,omitempty"`
}

type TicketChangeInput struct {
	Year     int     `json:"year"`
	Month    int     `json:"month"`
	Day      int     `json:"day"`
	Slug     string  `json:"slug"`
	Title    *string `json:"title,omitempty"`
	Prompt   *string `json:"prompt,omitempty"`
	LLM      *string `json:"llm,omitempty"`
	UI       *string `json:"ui,omitempty"`
	Goal     *string `json:"goal,omitempty"`
	Parent   *string `json:"parent,omitempty"`
	NoGithub bool    `json:"noGithub,omitempty"`
}

type ContributorAddInput struct {
	Github string   `json:"github"`
	Name   *string  `json:"name,omitempty"`
	Emails []string `json:"emails,omitempty"`
}

type FilterInput struct {
	Filter         *string  `json:"filter,omitempty"`
	Regex          *bool    `json:"regex,omitempty"`
	MatchCase      *bool    `json:"matchCase,omitempty"`
	MatchWholeWord *bool    `json:"matchWholeWord,omitempty"`
	ShowIgnored    *bool    `json:"showIgnored,omitempty"`
	ShowGenerated  *bool    `json:"showGenerated,omitempty"`
	ExcludeKinds   []string `json:"excludeKinds,omitempty"`
	IncludeKinds   []string `json:"includeKinds,omitempty"`
}

func (f *FilterInput) ToStreamOptions() StreamOptions {
	if f == nil {
		return StreamOptions{}
	}
	opts := StreamOptions{}
	if f.Filter != nil {
		opts.Filter = *f.Filter
	}
	if f.Regex != nil {
		opts.Regex = *f.Regex
	}
	if f.MatchCase != nil {
		opts.MatchCase = *f.MatchCase
	}
	if f.MatchWholeWord != nil {
		opts.MatchWholeWord = *f.MatchWholeWord
	}
	if f.ShowIgnored != nil {
		opts.ShowIgnored = *f.ShowIgnored
	}
	if f.ShowGenerated != nil {
		opts.ShowGenerated = *f.ShowGenerated
	}
	opts.ExcludeKinds = f.ExcludeKinds
	opts.IncludeKinds = f.IncludeKinds
	return opts
}

// #endregion GraphQL Input Types

// #endregion GraphQL Types

// #region Types

type ScopeKind string

const (
	ScopeRepo       ScopeKind = "repo"
	ScopeProject    ScopeKind = "bundle"
	ScopeFolder     ScopeKind = "folder"
	ScopeFile       ScopeKind = "file"
	ScopeSection    ScopeKind = "section"
	ScopeDefinition ScopeKind = "definition"
)

type Scope struct {
	Raw            string    `json:"raw"`
	Kind           ScopeKind `json:"kind"`
	ProjectName    string    `json:"projectName,omitempty"`
	FilePath       string    `json:"filePath,omitempty"`
	SectionPath    []string  `json:"sectionPath,omitempty"`
	DefinitionName string    `json:"definitionName,omitempty"`
}

type TodoCreateInput struct {
	ParentID    string `json:"parentId"`
	Name        string `json:"name"`
	Description string `json:"description"`
}

type TodoChangeInput struct {
	ID          string  `json:"id"`
	Name        *string `json:"name"`
	Description *string `json:"description"`
}

type Todo struct {
	ID          string    `json:"id"`
	Name        string    `json:"name"`
	Description string    `json:"description,omitempty"`
	ParentID    string    `json:"parentId"`
	Location    *Location `json:"location,omitempty"`
}

func (t *Todo) IsNode()       {}
func (t *Todo) GetID() string { return "todo:" + t.ID }
func (t *Todo) GetURI() string { return "semiorepo://TODO/" + strings.ToUpper(Slugify(t.ID)) }

type Location struct {
	FilePath string `json:"filePath"`
	Line     int    `json:"line"`
	Column   int    `json:"column"`
}

type Violation struct {
	ID      string        `json:"id"`
	Summary string        `json:"summary"`
	Kind    ViolationKind `json:"kind"`
	Scope   string        `json:"scope"`
	Line    int           `json:"line,omitempty"`
	Column  int           `json:"column,omitempty"`
	Excerpt string        `json:"excerpt,omitempty"`
}

func (v *Violation) IsNode()       {}
func (v *Violation) GetID() string { return "violation:" + v.ID }
func (v *Violation) GetURI() string { return "semiorepo://VIOLATION/" + strings.ToUpper(Slugify(v.ID)) }

func (v *Violation) Priority() ViolationPriority {
	return v.Kind.Info().Priority
}

func (v *Violation) Autofixable() bool {
	return v.Kind.Info().Autofixable
}

type TicketFileMetrics struct {
	Sections map[string]TicketSectionMetrics `yaml:"sections" json:"sections"`
}

type TicketBundleMetrics struct {
	Files map[string]TicketFileMetrics `yaml:"files" json:"files"`
}

type TicketBundles map[string]TicketBundleMetrics

// #region Languages

type LanguagePlugin interface {
	Name() string
	Extensions() []string
	MatchesExtension(ext string) bool
	SupportsSections() bool
	SupportsDefinitions() bool
	SupportsComments() bool
	SupportsHeaders() bool
	UsesIndentScoping() bool
	CommentPrefix() string
	ParseSections(content string) []Section
	ParseDefinitions(content string, lines []string) []DefinitionRange
	FormatSectionStart(name string) string
	FormatSectionEnd(name string) string
	FormatSectionBoth(name string) string
	FormatHeader(filePath, year, author, license string) string
	PolicySectionStartMatch(line string) (matched bool, name string)
	PolicySectionEndMatch(line string) (matched bool, name string)
	ExtraOrphanDefinitions(lines []string) []DefinitionRange
	ScanComments(ctx *PolicyContext, file, content string, lines []string) []Violation
	ExtractImports(content string) ([]string, string)
	FormatImports(imports []string) string
	ExtractPackage(content string) (string, string)
}

type DefinitionRange struct {
	Name    string
	Kind    string
	Start   int
	End     int
	Excerpt string
}

type BaseLanguage struct {
	name               string
	extensions         []string
	sectionStart       *regexp.Regexp
	sectionEnd         *regexp.Regexp
	definitionRegexp   *regexp.Regexp
	commentPrefix      string
	sectionStartFmt    string
	sectionEndFmt      string
	sectionBothFmt     string
	headerFmt          string
	usesIndentScoping  bool
	policySectionStart *regexp.Regexp
	policySectionEnd   *regexp.Regexp
}

func (l *BaseLanguage) Name() string              { return l.name }
func (l *BaseLanguage) Extensions() []string      { return l.extensions }
func (l *BaseLanguage) CommentPrefix() string     { return l.commentPrefix }
func (l *BaseLanguage) UsesIndentScoping() bool   { return l.usesIndentScoping }
func (l *BaseLanguage) SupportsSections() bool    { return l.sectionStart != nil }
func (l *BaseLanguage) SupportsDefinitions() bool { return l.definitionRegexp != nil }
func (l *BaseLanguage) SupportsComments() bool    { return l.commentPrefix != "" }
func (l *BaseLanguage) SupportsHeaders() bool     { return l.headerFmt != "" }

func (l *BaseLanguage) MatchesExtension(ext string) bool {
	ext = strings.ToLower(ext)
	for _, langExt := range l.extensions {
		if ext == langExt {
			return true
		}
	}
	return false
}

func (l *BaseLanguage) FormatSectionStart(name string) string {
	if l.sectionStartFmt == "" {
		return ""
	}
	return fmt.Sprintf(l.sectionStartFmt, name)
}

func (l *BaseLanguage) FormatSectionEnd(name string) string {
	if l.sectionEndFmt == "" {
		return ""
	}
	return fmt.Sprintf(l.sectionEndFmt, name)
}

func (l *BaseLanguage) FormatSectionBoth(name string) string {
	if l.sectionBothFmt == "" {
		return ""
	}
	if l.sectionEndFmt == "" {
		return fmt.Sprintf(l.sectionBothFmt, name)
	}
	return fmt.Sprintf(l.sectionBothFmt, name, name)
}

func (l *BaseLanguage) FormatHeader(filePath, year, author, license string) string {
	if l.headerFmt == "" {
		return ""
	}
	return fmt.Sprintf(l.headerFmt, filePath, year, author, license)
}

func (l *BaseLanguage) PolicySectionStartMatch(line string) (bool, string) {
	if l.policySectionStart == nil {
		return false, ""
	}
	match := l.policySectionStart.FindStringSubmatch(line)
	if match == nil {
		return false, ""
	}
	name := ""
	if len(match) > 1 {
		name = strings.TrimSpace(match[1])
	}
	return true, name
}

func (l *BaseLanguage) PolicySectionEndMatch(line string) (bool, string) {
	if l.policySectionEnd == nil {
		return false, ""
	}
	match := l.policySectionEnd.FindStringSubmatch(line)
	if match == nil {
		return false, ""
	}
	name := ""
	if len(match) > 1 {
		name = strings.TrimSpace(match[1])
	}
	return true, name
}

func (l *BaseLanguage) ParseSections(content string) []Section {
	if l.sectionStart == nil {
		return nil
	}
	lines := strings.Split(content, "\n")

	type sectionPtr struct {
		s        *Section
		children []*sectionPtr
	}

	var stack []*sectionPtr
	var roots []*sectionPtr
	charIndex := 0
	for i, line := range lines {
		lineStart := charIndex
		lineNum := i + 1
		if match := l.sectionStart.FindStringSubmatch(line); match != nil {
			name := strings.TrimSpace(match[1])
			s := &Section{
				Name:       name,
				StartLine:  lineNum,
				EndLine:    len(lines),
				StartIndex: lineStart,
				EndIndex:   len(content),
				Children:   nil,
			}
			sp := &sectionPtr{s: s}
			if len(stack) > 0 {
				parent := stack[len(stack)-1]
				parent.children = append(parent.children, sp)
			} else {
				roots = append(roots, sp)
			}
			stack = append(stack, sp)
		} else if l.sectionEnd != nil && l.sectionEnd.MatchString(line) {
			if len(stack) > 0 {
				sp := stack[len(stack)-1]
				sp.s.EndLine = lineNum
				sp.s.EndIndex = charIndex + len(line)
				stack = stack[:len(stack)-1]
			}
		}
		charIndex += len(line) + 1
	}

	var convert func(*sectionPtr) Section
	convert = func(sp *sectionPtr) Section {
		s := *sp.s
		if len(sp.children) > 0 {
			s.Children = make([]Section, len(sp.children))
			for i, child := range sp.children {
				s.Children[i] = convert(child)
			}
		}
		return s
	}

	result := make([]Section, len(roots))
	for i, root := range roots {
		result[i] = convert(root)
	}
	return result
}

func (l *BaseLanguage) ParseDefinitions(content string, lines []string) []DefinitionRange {
	if l.definitionRegexp == nil {
		return nil
	}
	type defStart struct {
		name string
		kind string
		line int
	}
	var defStarts []defStart
	for i, line := range lines {
		matches := l.definitionRegexp.FindAllStringSubmatch(line, -1)
		for _, match := range matches {
			if len(match) > 1 && match[1] != "" {
				kind := extractDefinitionKeyword(match[0], match[1])
				defStarts = append(defStarts, defStart{name: match[1], kind: kind, line: i + 1})
			}
		}
	}
	var defRanges []DefinitionRange
	for i := 0; i < len(defStarts); i++ {
		start := defStarts[i].line
		end := start
		if l.usesIndentScoping {
			startIndent := len(lines[start-1]) - len(strings.TrimLeft(lines[start-1], " \t"))
			for lineIndex := start; lineIndex < len(lines); lineIndex++ {
				line := strings.TrimSuffix(lines[lineIndex], "\r")
				if strings.TrimSpace(line) == "" {
					continue
				}
				currentIndent := len(line) - len(strings.TrimLeft(line, " \t"))
				if currentIndent <= startIndent {
					end = lineIndex
					break
				}
				end = lineIndex + 1
			}
		} else {
			braceDepth := 0
			sawOpen := false
			for lineIndex := start - 1; lineIndex < len(lines); lineIndex++ {
				line := lines[lineIndex]
				for _, ch := range line {
					if ch == '{' {
						braceDepth++
						sawOpen = true
					} else if ch == '}' {
						if braceDepth > 0 {
							braceDepth--
						}
						if sawOpen && braceDepth == 0 {
							end = lineIndex + 1
							lineIndex = len(lines)
							break
						}
					}
				}
				if sawOpen && braceDepth == 0 && end > start {
					break
				}
			}
			if !sawOpen {
				if i+1 < len(defStarts) {
					end = defStarts[i+1].line - 1
				}
			}
		}
		if end < start {
			end = start
		}
		defRanges = append(defRanges, DefinitionRange{
			Name:    defStarts[i].name,
			Kind:    defStarts[i].kind,
			Start:   start,
			End:     end,
			Excerpt: defStarts[i].name,
		})
	}
	return defRanges
}

func extractDefinitionKeyword(fullMatch, name string) string {
	keywords := []string{
		"async def", "async function",
		"function", "class", "interface", "type", "enum", "const", "let", "var",
		"func", "struct", "trait", "impl", "mod",
		"def", "module",
		"public", "private", "protected", "internal", "abstract", "sealed", "virtual", "override",
		"delegate", "record", "union", "scalar", "query", "mutation", "subscription", "fragment",
		"CREATE TABLE", "CREATE VIEW", "CREATE PROCEDURE", "CREATE FUNCTION", "CREATE TRIGGER",
		"CREATE INDEX", "CREATE TYPE", "CREATE SCHEMA", "CREATE DATABASE", "CREATE SEQUENCE",
		"input", "extend type", "extend interface", "extend enum", "extend union", "extend input",
	}
	lower := strings.ToLower(fullMatch)
	for _, kw := range keywords {
		if strings.Contains(lower, strings.ToLower(kw)) {
			return kw
		}
	}
	return "definition"
}

func (l *BaseLanguage) ExtraOrphanDefinitions(lines []string) []DefinitionRange {
	return nil
}

func (l *BaseLanguage) ScanComments(ctx *PolicyContext, file, content string, lines []string) []Violation {
	return nil
}

func (l *BaseLanguage) ExtractImports(content string) ([]string, string) {
	return []string{}, content
}

func (l *BaseLanguage) FormatImports(imports []string) string {
	return ""
}

func (l *BaseLanguage) ExtractPackage(content string) (string, string) {
	return "", content
}

// #region TypeScript

type TypeScriptLanguage struct {
	BaseLanguage
}

func NewTypeScriptLanguage() *TypeScriptLanguage {
	return &TypeScriptLanguage{
		BaseLanguage: BaseLanguage{
			name:               "typescript",
			extensions:         []string{".ts", ".tsx", ".js", ".jsx"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*//\s*#region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`^(?:export\s+)?(?:const|let|var|function|class|interface|type|enum)\s+([A-Za-z_][A-Za-z0-9_]*)`),
			commentPrefix:      "//",
			sectionStartFmt:    "// #region %s",
			sectionEndFmt:      "// #endregion %s",
			sectionBothFmt:     "\n// #region %s\n\n// #endregion %s\n",
			headerFmt:          "// #region Header\n\n// %s\n\n// %s %s\n\n%s\n\n// #endregion Header\n",
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*//\s*#region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

func (l *TypeScriptLanguage) ScanComments(ctx *PolicyContext, file, content string, lines []string) []Violation {
	var violations []Violation
	// Find header section to exclude comments within it
	sections := l.ParseSections(content)
	var headerSection *Section
	for i := range sections {
		if strings.ToLower(sections[i].Name) == "header" {
			headerSection = &sections[i]
			break
		}
	}
	charIndex := 0
	scanState := CommentScanState{}
	inlineCommentActive := false
	for i, line := range lines {
		lineNum := i + 1
		// Skip comments inside header section
		if headerSection != nil && lineNum >= headerSection.StartLine && lineNum <= headerSection.EndLine {
			charIndex += len(line) + 1
			continue
		}
		lineStart := charIndex
		j := 0
		foundInline := false
		for j < len(line) {
			if scanState.InBlockComment {
				if j+1 < len(line) && line[j] == '*' && line[j+1] == '/' {
					if scanState.BlockCommentIsJsDoc {
						violations = append(violations, ctx.CreateViolation(
							fmt.Sprintf("JSDoc comment in %s:%d", file, scanState.BlockCommentStartLine),
							ViolationCodeCommentJSDoc,
							file, scanState.BlockCommentStartLine, ""))
					} else {
						violations = append(violations, ctx.CreateViolation(
							fmt.Sprintf("Block comment in %s:%d", file, scanState.BlockCommentStartLine),
							ViolationCodeCommentBlock,
							file, scanState.BlockCommentStartLine, ""))
					}
					scanState.InBlockComment = false
					j += 2
					continue
				}
				j++
				continue
			}
			if scanState.Escaped {
				scanState.Escaped = false
				j++
				continue
			}
			if line[j] == '\\' && (scanState.InSingleQuote || scanState.InDoubleQuote || scanState.InTemplateRaw()) {
				scanState.Escaped = true
				j++
				continue
			}
			if scanState.InSingleQuote {
				if line[j] == '\'' {
					scanState.InSingleQuote = false
				}
				j++
				continue
			}
			if scanState.InDoubleQuote {
				if line[j] == '"' {
					scanState.InDoubleQuote = false
				}
				j++
				continue
			}
			if scanState.InTemplateRaw() {
				if line[j] == '`' {
					scanState.Templates = scanState.Templates[:len(scanState.Templates)-1]
					j++
					continue
				}
				if j+1 < len(line) && line[j] == '$' && line[j+1] == '{' {
					scanState.Templates[len(scanState.Templates)-1].ExprDepth = 1
					j += 2
					continue
				}
				j++
				continue
			}
			if len(scanState.Templates) > 0 && scanState.Templates[len(scanState.Templates)-1].ExprDepth > 0 {
				if line[j] == '{' {
					scanState.Templates[len(scanState.Templates)-1].ExprDepth++
					j++
					continue
				}
				if line[j] == '}' {
					scanState.Templates[len(scanState.Templates)-1].ExprDepth--
					j++
					continue
				}
			}
			if line[j] == '\'' {
				scanState.InSingleQuote = true
				j++
				continue
			}
			if line[j] == '"' {
				scanState.InDoubleQuote = true
				j++
				continue
			}
			if line[j] == '`' {
				scanState.Templates = append(scanState.Templates, CommentTemplateState{ExprDepth: 0})
				j++
				continue
			}
			if j+1 < len(line) && line[j] == '/' && line[j+1] == '*' {
				isJsDoc := j+2 < len(line) && line[j+2] == '*'
				scanState.InBlockComment = true
				scanState.BlockCommentStartLine = lineNum
				scanState.BlockCommentStartIndex = lineStart + j
				scanState.BlockCommentIsJsDoc = isJsDoc
				j += 2
				continue
			}
			if j+1 < len(line) && line[j] == '/' && line[j+1] == '/' {
				// Skip URL schemes like http://, https://, ftp://, etc.
				if j > 0 && line[j-1] == ':' {
					j += 2
					continue
				}
				// Skip escaped slashes in regex like /pattern\//
				if j > 0 && line[j-1] == '\\' {
					j += 2
					continue
				}
				trimmed := strings.TrimSpace(line)
				if strings.HasPrefix(trimmed, "// #region") || strings.HasPrefix(trimmed, "// #endregion") {
					break
				}
				if strings.HasPrefix(trimmed, "// eslint-") || strings.HasPrefix(trimmed, "// @ts-") || strings.HasPrefix(trimmed, "// noinspection") || strings.HasPrefix(trimmed, "// TODO") || strings.HasPrefix(trimmed, "// semio-ignore-") {
					break
				}
				debugMarker := strings.Contains(line, "[DEBUG]")
				if !debugMarker {
					foundInline = true
					if !inlineCommentActive {
						violations = append(violations, ctx.CreateViolation(
							fmt.Sprintf("Inline comment in %s:%d", file, lineNum),
							ViolationCodeCommentInline,
							file, lineNum, strings.TrimSpace(line[j:])))
						inlineCommentActive = true
					}
				}
				break
			}
			j++
		}
		if !foundInline {
			if strings.TrimSpace(line) != "" {
				inlineCommentActive = false
			}
		}
		charIndex += len(line) + 1
	}
	return violations
}

// #endregion TypeScript

func (l *TypeScriptLanguage) ExtractImports(content string) ([]string, string) {
	lines := strings.Split(content, "\n")
	var imports []string
	var bodyLines []string
	for i := 0; i < len(lines); i++ {
		line := lines[i]
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "import ") {
			currentImport := line
			if !strings.Contains(line, ";") {
				for j := i + 1; j < len(lines); j++ {
					currentImport += "\n" + lines[j]
					if strings.Contains(lines[j], ";") {
						i = j
						break
					}
				}
			}
			imports = append(imports, currentImport)
		} else {
			bodyLines = append(bodyLines, line)
		}
	}
	return imports, strings.Join(bodyLines, "\n")
}

func (l *TypeScriptLanguage) FormatImports(imports []string) string {
	if len(imports) == 0 {
		return ""
	}
	seen := make(map[string]bool)
	var uniqueImports []string
	for _, imp := range imports {
		if !seen[imp] {
			seen[imp] = true
			uniqueImports = append(uniqueImports, imp)
		}
	}
	return strings.Join(uniqueImports, "\n")
}

// #region Go

type GoLanguage struct {
	BaseLanguage
}

func NewGoLanguage() *GoLanguage {
	return &GoLanguage{
		BaseLanguage: BaseLanguage{
			name:               "go",
			extensions:         []string{".go"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*//\s*#region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`^(?:func|type|var|const)\s+(?:\([^)]+\)\s+)?([A-Za-z_][A-Za-z0-9_]*)`),
			commentPrefix:      "//",
			sectionStartFmt:    "// #region %s",
			sectionEndFmt:      "// #endregion %s",
			sectionBothFmt:     "\n// #region %s\n\n// #endregion %s\n",
			headerFmt:          "// #region Header\n\n// %s\n\n// %s %s\n\n%s\n\n// #endregion Header\n",
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*//\s*#region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

func (l *GoLanguage) ExtraOrphanDefinitions(lines []string) []DefinitionRange {
	var defs []DefinitionRange
	for i, line := range lines {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "package ") {
			name := fmt.Sprintf("package-%d", i+1)
			defs = append(defs, DefinitionRange{Name: name, Start: i + 1, End: i + 1, Excerpt: trimmed})
			break
		}
	}
	for i := 0; i < len(lines); i++ {
		trimmed := strings.TrimSpace(lines[i])
		if strings.HasPrefix(trimmed, "import ") {
			start := i + 1
			end := start
			if strings.HasPrefix(trimmed, "import (") {
				for j := i + 1; j < len(lines); j++ {
					if strings.TrimSpace(lines[j]) == ")" {
						end = j + 1
						i = j
						break
					}
				}
			}
			name := fmt.Sprintf("import-%d", start)
			defs = append(defs, DefinitionRange{Name: name, Start: start, End: end, Excerpt: strings.TrimSpace(lines[start-1])})
		}
	}
	return defs
}

func (l *GoLanguage) ExtractImports(content string) ([]string, string) {
	lines := strings.Split(content, "\n")
	var imports []string
	var bodyLines []string
	inImportBlock := false
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)

		if inImportBlock {
			if trimmed == ")" {
				inImportBlock = false
			} else if trimmed != "" {
				// Inside import block, keep it raw or trim?
				// existing logic was just trimmed, but we might want to keep quotes logic?
				// But FormatImports adds quotes if we provide strings.
				// Wait, ExtractImports implementation I wrote earlier just appended lines.
				// Go imports inside () don't have "import" prefix.
				// e.g. "fmt"
				imports = append(imports, strings.Trim(trimmed, ",")) // handle comma if present (Go doesn't use commas usually in imports)
			}
			continue
		}

		if strings.HasPrefix(trimmed, "import (") {
			inImportBlock = true
			continue
		}

		if strings.HasPrefix(trimmed, "import ") {
			// Single line import: import "fmt" or import alias "pkg"
			imports = append(imports, strings.TrimPrefix(trimmed, "import "))
			continue
		}

		bodyLines = append(bodyLines, line)
	}
	return imports, strings.Join(bodyLines, "\n")
}

func (l *GoLanguage) FormatImports(imports []string) string {
	if len(imports) == 0 {
		return ""
	}
	importBlock := "import (\n"
	seen := make(map[string]bool)
	var uniqueImports []string
	for _, imp := range imports {
		if !seen[imp] {
			seen[imp] = true
			uniqueImports = append(uniqueImports, imp)
		}
	}
	sort.Strings(uniqueImports)
	for _, imp := range uniqueImports {
		importBlock += "\t" + imp + "\n"
	}
	importBlock += ")"
	return importBlock
}

func (l *GoLanguage) ExtractPackage(content string) (string, string) {
	lines := strings.Split(content, "\n")
	pkg := ""
	var bodyLines []string
	foundPkg := false
	for _, line := range lines {
		if !foundPkg && strings.HasPrefix(strings.TrimSpace(line), "package ") {
			pkg = line
			foundPkg = true
			continue
		}
		bodyLines = append(bodyLines, line)
	}
	return pkg, strings.Join(bodyLines, "\n")
}

type PythonLanguage struct {
	BaseLanguage
}

func NewPythonLanguage() *PythonLanguage {
	return &PythonLanguage{
		BaseLanguage: BaseLanguage{
			name:               "python",
			extensions:         []string{".py"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#\s*region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`(?:^|\s)(?:def|class|async\s+def)\s+([A-Za-z_][A-Za-z0-9_]*)`),
			commentPrefix:      "#",
			sectionStartFmt:    "# region %s",
			sectionEndFmt:      "# endregion %s",
			sectionBothFmt:     "\n# region %s\n\n# endregion %s\n",
			headerFmt:          "# region Header\n\n# %s\n\n# %s %s\n\n%s\n\n# endregion Header\n",
			usesIndentScoping:  true,
			policySectionStart: regexp.MustCompile(`(?i)^\s*#\s*region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

func (l *PythonLanguage) ExtractImports(content string) ([]string, string) {
	lines := strings.Split(content, "\n")
	var imports []string
	var bodyLines []string
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "import ") || strings.HasPrefix(trimmed, "from ") {
			imports = append(imports, line)
		} else {
			bodyLines = append(bodyLines, line)
		}
	}
	return imports, strings.Join(bodyLines, "\n")
}

func (l *PythonLanguage) FormatImports(imports []string) string {
	if len(imports) == 0 {
		return ""
	}
	seen := make(map[string]bool)
	var uniqueImports []string
	for _, imp := range imports {
		if !seen[imp] {
			seen[imp] = true
			uniqueImports = append(uniqueImports, imp)
		}
	}
	sort.Strings(uniqueImports)
	return strings.Join(uniqueImports, "\n")
}

// #endregion Go

// #region C#

type CSharpLanguage struct {
	BaseLanguage
}

func NewCSharpLanguage() *CSharpLanguage {
	return &CSharpLanguage{
		BaseLanguage: BaseLanguage{
			name:               "csharp",
			extensions:         []string{".cs"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`(?:public|private|protected|internal|static|partial|abstract|sealed|virtual|override|async)*\s*(?:class|struct|interface|enum|delegate|record|void|string|int|bool|[A-Z][A-Za-z0-9_<>]*)\s+([A-Z][A-Za-z0-9_]*)\s*[<({]`),
			commentPrefix:      "//",
			sectionStartFmt:    "#region %s",
			sectionEndFmt:      "#endregion %s",
			sectionBothFmt:     "\n#region %s\n\n#endregion %s\n",
			headerFmt:          "#region Header\n\n// %s\n\n// %s %s\n\n%s\n\n#endregion Header\n",
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*#region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

func (l *CSharpLanguage) ExtractImports(content string) ([]string, string) {
	lines := strings.Split(content, "\n")
	var imports []string
	var bodyLines []string
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "using ") && strings.HasSuffix(trimmed, ";") {
			imports = append(imports, line)
		} else {
			bodyLines = append(bodyLines, line)
		}
	}
	return imports, strings.Join(bodyLines, "\n")
}

func (l *CSharpLanguage) FormatImports(imports []string) string {
	if len(imports) == 0 {
		return ""
	}
	seen := make(map[string]bool)
	var uniqueImports []string
	for _, imp := range imports {
		if !seen[imp] {
			seen[imp] = true
			uniqueImports = append(uniqueImports, imp)
		}
	}
	sort.Strings(uniqueImports)
	return strings.Join(uniqueImports, "\n")
}

// #endregion C#

// #region JSON

type JSONLanguage struct {
	BaseLanguage
}

func NewJSONLanguage() *JSONLanguage {
	return &JSONLanguage{
		BaseLanguage: BaseLanguage{
			name:              "json",
			extensions:        []string{".json"},
			commentPrefix:     "",
			usesIndentScoping: false,
		},
	}
}

func (l *JSONLanguage) SupportsSections() bool    { return true }
func (l *JSONLanguage) SupportsDefinitions() bool { return false }
func (l *JSONLanguage) SupportsComments() bool    { return false }
func (l *JSONLanguage) SupportsHeaders() bool     { return false }

func (l *JSONLanguage) ParseSections(content string) []Section {
	sections, _, _ := ParseJSONSectionsDetailed(content)
	return sections
}

// #endregion JSON

// #region Markdown

type MarkdownLanguage struct {
	BaseLanguage
}

func NewMarkdownLanguage() *MarkdownLanguage {
	return &MarkdownLanguage{
		BaseLanguage: BaseLanguage{
			name:              "markdown",
			extensions:        []string{".md", ".mdx"},
			sectionStart:      regexp.MustCompile(`^(#{1,6})\s+(.+?)\s*$`),
			commentPrefix:     "",
			sectionStartFmt:   "## %s",
			sectionEndFmt:     "",
			sectionBothFmt:    "\n## %s\n\n",
			headerFmt:         "",
			usesIndentScoping: false,
		},
	}
}

func (l *MarkdownLanguage) SupportsSections() bool    { return true }
func (l *MarkdownLanguage) SupportsDefinitions() bool { return false }
func (l *MarkdownLanguage) SupportsComments() bool    { return false }
func (l *MarkdownLanguage) SupportsHeaders() bool     { return false }

func (l *MarkdownLanguage) ParseSections(content string) []Section {
	return ParseMarkdownSectionsInternal(content)
}

// #endregion Markdown

// #region Rust

type RustLanguage struct {
	BaseLanguage
}

func NewRustLanguage() *RustLanguage {
	return &RustLanguage{
		BaseLanguage: BaseLanguage{
			name:               "rust",
			extensions:         []string{".rs"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*//\s*#region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`(?:^|\s)(?:pub\s+)?(?:fn|struct|enum|trait|impl|type|const|static|mod)\s+([A-Za-z_][A-Za-z0-9_]*)`),
			commentPrefix:      "//",
			sectionStartFmt:    "// #region %s",
			sectionEndFmt:      "// #endregion %s",
			sectionBothFmt:     "\n// #region %s\n\n// #endregion %s\n",
			headerFmt:          "// #region Header\n\n// %s\n\n// %s %s\n\n%s\n\n// #endregion Header\n",
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*//\s*#region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

func (l *RustLanguage) ExtraOrphanDefinitions(lines []string) []DefinitionRange {
	var defs []DefinitionRange
	modRegexp := regexp.MustCompile(`^\s*(?:pub\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;`)
	for i, line := range lines {
		if match := modRegexp.FindStringSubmatch(line); match != nil {
			name := fmt.Sprintf("mod-%s-%d", match[1], i+1)
			defs = append(defs, DefinitionRange{Name: name, Start: i + 1, End: i + 1, Excerpt: strings.TrimSpace(line)})
		}
	}
	return defs
}

// #endregion Rust

// #region Ruby

type RubyLanguage struct {
	BaseLanguage
}

func NewRubyLanguage() *RubyLanguage {
	return &RubyLanguage{
		BaseLanguage: BaseLanguage{
			name:               "ruby",
			extensions:         []string{".rb", ".rake", ".gemspec"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#\s*region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`^(?:def|class|module)\s+([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)`),
			commentPrefix:      "#",
			sectionStartFmt:    "# region %s",
			sectionEndFmt:      "# endregion %s",
			sectionBothFmt:     "\n# region %s\n\n# endregion %s\n",
			headerFmt:          "# region Header\n\n# %s\n\n# %s %s\n\n%s\n\n# endregion Header\n",
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*#\s*region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

func (l *RubyLanguage) ParseDefinitions(content string, lines []string) []DefinitionRange {
	if l.definitionRegexp == nil {
		return nil
	}
	type defStart struct {
		name  string
		line  int
		depth int
	}
	var defStack []defStart
	var defRanges []DefinitionRange
	endRegexp := regexp.MustCompile(`^\s*end\s*$`)
	blockStartRegexp := regexp.MustCompile(`(?:^|\s)(?:if|unless|case|while|until|for|begin|do)\b`)
	depth := 0
	for i, line := range lines {
		lineNum := i + 1
		trimmed := strings.TrimSpace(line)
		if matches := l.definitionRegexp.FindAllStringSubmatch(line, -1); matches != nil {
			for _, match := range matches {
				if len(match) > 1 && match[1] != "" {
					defStack = append(defStack, defStart{name: match[1], line: lineNum, depth: depth})
					depth++
				}
			}
		} else if blockStartRegexp.MatchString(line) && !strings.Contains(line, " do ") {
			depth++
		}
		if endRegexp.MatchString(trimmed) {
			if depth > 0 {
				depth--
			}
			for len(defStack) > 0 && defStack[len(defStack)-1].depth == depth {
				def := defStack[len(defStack)-1]
				defStack = defStack[:len(defStack)-1]
				defRanges = append(defRanges, DefinitionRange{
					Name:    def.name,
					Start:   def.line,
					End:     lineNum,
					Excerpt: def.name,
				})
			}
		}
	}
	for _, def := range defStack {
		defRanges = append(defRanges, DefinitionRange{
			Name:    def.name,
			Start:   def.line,
			End:     len(lines),
			Excerpt: def.name,
		})
	}
	sort.Slice(defRanges, func(i, j int) bool {
		return defRanges[i].Start < defRanges[j].Start
	})
	return defRanges
}

func (l *RubyLanguage) ExtraOrphanDefinitions(lines []string) []DefinitionRange {
	var defs []DefinitionRange
	moduleRegexp := regexp.MustCompile(`^\s*module\s+([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*)`)
	for i, line := range lines {
		if match := moduleRegexp.FindStringSubmatch(line); match != nil {
			name := fmt.Sprintf("module-%s-%d", match[1], i+1)
			defs = append(defs, DefinitionRange{Name: name, Start: i + 1, End: i + 1, Excerpt: strings.TrimSpace(line)})
		}
	}
	return defs
}

// #endregion Ruby

// #region Shell

type ShellLanguage struct {
	BaseLanguage
}

func NewShellLanguage() *ShellLanguage {
	return &ShellLanguage{
		BaseLanguage: BaseLanguage{
			name:               "shell",
			extensions:         []string{".sh"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#\s*region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`^\s*(?:function\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*(?:\(\))?\s*\{`),
			commentPrefix:      "#",
			sectionStartFmt:    "# region %s",
			sectionEndFmt:      "# endregion %s",
			sectionBothFmt:     "\n# region %s\n\n# endregion %s\n",
			headerFmt:          "# region Header\n\n# %s\n\n# %s %s\n\n%s\n\n# endregion Header\n",
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*#\s*region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

// #endregion Shell

// #region TOML

type TomlLanguage struct {
	BaseLanguage
}

func NewTomlLanguage() *TomlLanguage {
	return &TomlLanguage{
		BaseLanguage: BaseLanguage{
			name:               "toml",
			extensions:         []string{".toml"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#\s*region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(.+?))?\s*$`),
			commentPrefix:      "#",
			sectionStartFmt:    "# region %s",
			sectionEndFmt:      "# endregion %s",
			sectionBothFmt:     "\n# region %s\n\n# endregion %s\n",
			headerFmt:          "# region Header\n\n# %s\n\n# %s %s\n\n%s\n\n# endregion Header\n",
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*#\s*region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

func (l *TomlLanguage) SupportsSections() bool    { return true }
func (l *TomlLanguage) SupportsDefinitions() bool { return false }
func (l *TomlLanguage) SupportsComments() bool    { return true }
func (l *TomlLanguage) SupportsHeaders() bool     { return false }

// #endregion TOML

// #region YAML

type YamlLanguage struct {
	BaseLanguage
}

func NewYamlLanguage() *YamlLanguage {
	return &YamlLanguage{
		BaseLanguage: BaseLanguage{
			name:               "yaml",
			extensions:         []string{".yaml", ".yml"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#\s*region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(.+?))?\s*$`),
			commentPrefix:      "#",
			sectionStartFmt:    "# region %s",
			sectionEndFmt:      "# endregion %s",
			sectionBothFmt:     "\n# region %s\n\n# endregion %s\n",
			headerFmt:          "# region Header\n\n# %s\n\n# %s %s\n\n%s\n\n# endregion Header\n",
			usesIndentScoping:  true,
			policySectionStart: regexp.MustCompile(`(?i)^\s*#\s*region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

func (l *YamlLanguage) SupportsSections() bool    { return true }
func (l *YamlLanguage) SupportsDefinitions() bool { return false }
func (l *YamlLanguage) SupportsComments() bool    { return true }
func (l *YamlLanguage) SupportsHeaders() bool     { return false }

// #endregion YAML

// #region SQL

type SqlLanguage struct {
	BaseLanguage
}

func NewSqlLanguage() *SqlLanguage {
	return &SqlLanguage{
		BaseLanguage: BaseLanguage{
			name:               "sql",
			extensions:         []string{".sql"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*--\s*#region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*--\s*#endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`(?i)^(?:CREATE\s+(?:OR\s+REPLACE\s+)?(?:TABLE|VIEW|PROCEDURE|FUNCTION|TRIGGER|INDEX|TYPE|SCHEMA|DATABASE|SEQUENCE|MATERIALIZED\s+VIEW))\s+(?:IF\s+NOT\s+EXISTS\s+)?([A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*)`),
			commentPrefix:      "--",
			sectionStartFmt:    "-- #region %s",
			sectionEndFmt:      "-- #endregion %s",
			sectionBothFmt:     "\n-- #region %s\n\n-- #endregion %s\n",
			headerFmt:          "-- #region Header\n\n-- %s\n\n-- %s %s\n\n%s\n\n-- #endregion Header\n",
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*--\s*#region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*--\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

// #endregion SQL

// #region GraphQL

type GraphqlLanguage struct {
	BaseLanguage
}

func NewGraphqlLanguage() *GraphqlLanguage {
	return &GraphqlLanguage{
		BaseLanguage: BaseLanguage{
			name:               "graphql",
			extensions:         []string{".graphql", ".gql"},
			sectionStart:       regexp.MustCompile(`(?i)^\s*#\s*#region\s+(.+?)\s*$`),
			sectionEnd:         regexp.MustCompile(`(?i)^\s*#\s*#endregion(?:\s+(.+?))?\s*$`),
			definitionRegexp:   regexp.MustCompile(`^(?:type|interface|enum|input|union|scalar|query|mutation|subscription|fragment|extend\s+type|extend\s+interface|extend\s+enum|extend\s+union|extend\s+input)\s+([A-Za-z_][A-Za-z0-9_]*)`),
			commentPrefix:      "#",
			sectionStartFmt:    "# #region %s",
			sectionEndFmt:      "# #endregion %s",
			sectionBothFmt:     "\n# #region %s\n\n# #endregion %s\n",
			headerFmt:          "# #region Header\n\n# %s\n\n# %s %s\n\n%s\n\n# #endregion Header\n",
			usesIndentScoping:  false,
			policySectionStart: regexp.MustCompile(`(?i)^\s*#\s*#region(?:\s+(\S.*?))?\s*$`),
			policySectionEnd:   regexp.MustCompile(`(?i)^\s*#\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
}

// #endregion GraphQL

var languageRegistry = []LanguagePlugin{
	NewTypeScriptLanguage(),
	NewGoLanguage(),
	NewPythonLanguage(),
	NewCSharpLanguage(),
	NewMarkdownLanguage(),
	NewRustLanguage(),
	NewRubyLanguage(),
	NewShellLanguage(),
	NewTomlLanguage(),
	NewYamlLanguage(),
	NewSqlLanguage(),
	NewGraphqlLanguage(),
}

func GetLanguage(filePath string) LanguagePlugin {
	ext := strings.ToLower(filepath.Ext(filePath))
	for _, lang := range languageRegistry {
		if lang.MatchesExtension(ext) {
			return lang
		}
	}
	return nil
}

func GetLanguageByName(name string) LanguagePlugin {
	for _, lang := range languageRegistry {
		if lang.Name() == name {
			return lang
		}
	}
	return nil
}

// #endregion Languages

type GitAuthor struct {
	Name   string `json:"name,omitempty" yaml:"name,omitempty"`
	Email  string `json:"email,omitempty" yaml:"email,omitempty"`
	GitHub string `json:"github,omitempty" yaml:"github,omitempty"`
}

func (a GitAuthor) String() string {
	if a.Email != "" {
		return fmt.Sprintf("%s <%s>", a.Name, a.Email)
	}
	return a.Name
}

func parseGitAuthor(s string) GitAuthor {
	res := GitAuthor{}
	if strings.Contains(s, " <") {
		parts := strings.Split(s, " <")
		res.Name = strings.TrimSpace(parts[0])
		res.Email = strings.TrimSuffix(parts[1], ">")
	} else {
		res.Name = s
	}
	return res
}

type TicketIteration struct {
	Prompt   string       `json:"prompt" yaml:"prompt"`
	LLM      string       `json:"llm" yaml:"llm"`
	UI       string       `json:"ui,omitempty" yaml:"ui,omitempty"`
	Author   GitAuthor    `json:"author" yaml:"author"`
	Started  time.Time    `json:"started" yaml:"started"`
	Finished *time.Time   `json:"finished,omitempty" yaml:"finished,omitempty"`
	Commit   string       `json:"commit" yaml:"commit"`
	Files    []TicketFile `json:"files,omitempty" yaml:"files,omitempty"`
	Diff     *TicketDiffs `json:"diff,omitempty" yaml:"diff,omitempty"`
}

type TicketSection struct {
	Name        string       `json:"name"`
	Range       *Range       `json:"range,omitempty"`
	Definitions []string     `json:"definitions,omitempty"`
	Lines       *LineMetrics `json:"lines,omitempty"`
}

type TicketFile struct {
	Path  string       `json:"path"`
	Lines *LineMetrics `json:"lines,omitempty"`
}

type TicketGithubData struct {
	Issue string `json:"issue,omitempty"`
}

type TicketFileRenamed struct {
	From  string       `json:"from"`
	To    string       `json:"to"`
	Lines *LineMetrics `json:"lines,omitempty"`
}

type TicketDiffSet struct {
	Deleted  []TicketFile        `json:"deleted"`
	Renamed  []TicketFileRenamed `json:"renamed"`
	Modified []TicketFile        `json:"modified"`
	Added    []TicketFile        `json:"added"`
}

type TicketDiffs struct {
	Bundles     TicketDiffSet `json:"bundles"`
	Folders     TicketDiffSet `json:"folders"`
	Files       TicketDiffSet `json:"files"`
	Sections    TicketDiffSet `json:"sections"`
	Definitions TicketDiffSet `json:"definitions"`
}

type TicketData struct {
	Title      string            `json:"title"`
	Iterations []TicketIteration `json:"iterations"`
	Status     TicketStatus      `json:"status"`
	Dates      TicketDates       `json:"dates"`
	Summary    string            `json:"summary,omitempty"`
	GitHub     *TicketGithubData `json:"github,omitempty"`
	Goal       string            `json:"goal,omitempty"`
	Parent     string            `json:"parent,omitempty"`
}

type Goal struct {
	Title       string            `json:"title"`
	Description string            `json:"description"`
	Prompt      string            `json:"prompt"`
	Status      string            `json:"status"`
	Summary     string            `json:"summary,omitempty"`
	DueDate     string            `json:"dueDate,omitempty"`
	Dates       GoalDates         `json:"dates"`
	UI          string            `json:"client"`
	LLM         string            `json:"llm"`
	Parent      string            `json:"parent,omitempty"`
	GitHub      *GoalGithubData   `json:"github,omitempty"`
	Iterations  []TicketIteration `json:"iterations"`
	// Derived fields
	ID   string `json:"-"`
	Path string `json:"-"`
}

func (g *Goal) IsNode()       {}
func (g *Goal) GetID() string { return "goal:" + Slugify(g.Title) }
func (g *Goal) GetURI() string { return "semiorepo://GOAL/" + strings.ToUpper(Slugify(g.Title)) }

type GoalDates struct {
	Due    string     `json:"due,omitempty"`
	Closed *time.Time `json:"closed,omitempty"`
}

type GoalGithubData struct {
	Milestone string `json:"milestone,omitempty"`
}

type TicketDates struct {
	Closed *time.Time `json:"closed,omitempty"`
}

type ViolationKind string

const (
	ViolationCodeHeaderMissingRegion        ViolationKind = "code:header:missing-region"
	ViolationCodeHeaderMissingFilename      ViolationKind = "code:header:missing-filename"
	ViolationCodeHeaderMissingContributors  ViolationKind = "code:header:missing-contributors"
	ViolationCodeHeaderMissingLicense       ViolationKind = "code:header:missing-license"
	ViolationCodeHeaderWrongLicense         ViolationKind = "code:header:wrong-license"
	ViolationCodeSectionEmpty               ViolationKind = "code:section:empty"
	ViolationCodeSectionOrphanDefinition    ViolationKind = "code:section:orphan-definition"
	ViolationCodeSectionMissingStartName    ViolationKind = "code:section:missing-start-name"
	ViolationCodeSectionMissingEndName      ViolationKind = "code:section:missing-end-name"
	ViolationCodeSectionNameMismatch        ViolationKind = "code:section:name-mismatch"
	ViolationCodeCommentInline              ViolationKind = "code:comment:inline"
	ViolationCodeCommentBlock               ViolationKind = "code:comment:block"
	ViolationCodeCommentJSDoc               ViolationKind = "code:comment:jsdoc"
	ViolationDevDocsMissingFile             ViolationKind = "dev-docs:missing-file"
	ViolationDevDocsMissingFolder           ViolationKind = "dev-docs:missing-folder"
	ViolationDevDocsWrongFilePath           ViolationKind = "dev-docs:wrong-file-path"
	ViolationDevDocsWrongFolderPath         ViolationKind = "dev-docs:wrong-folder-path"
	ViolationDevDocsWrongFileName           ViolationKind = "dev-docs:wrong-file-name"
	ViolationDevDocsWrongFolderName         ViolationKind = "dev-docs:wrong-folder-name"
	ViolationDevDocsWrongFileOrder          ViolationKind = "dev-docs:wrong-file-order"
	ViolationDevDocsWrongFolderOrder        ViolationKind = "dev-docs:wrong-folder-order"
	ViolationDevDocsMissingComponent        ViolationKind = "dev-docs:missing-component"
	ViolationDevDocsWrongComponentName      ViolationKind = "dev-docs:wrong-component-name"
	ViolationDevDocsWrongComponentOrder     ViolationKind = "dev-docs:wrong-component-order"
	ViolationSketchpadImportThirdParty      ViolationKind = "sketchpad:import:third-party-outside-elements"
	ViolationSketchpadStateMultipleMachines ViolationKind = "sketchpad:state:multiple-machines"
	ViolationSketchpadStateCreateActor      ViolationKind = "sketchpad:state:create-actor-usage"
	ViolationSketchpadStateYjsAppState      ViolationKind = "sketchpad:state:yjs-app-state"
	ViolationSketchpadStateForbiddenStore   ViolationKind = "sketchpad:state:forbidden-store"
	ViolationSketchpadHooksNonTriadic       ViolationKind = "sketchpad:hooks:non-triadic"
	ViolationRepoMissingCommand             ViolationKind = "repo:missing-command"
	ViolationRepoMissingTicketTracking      ViolationKind = "repo:missing-ticket-tracking"
)

var violationKindInfoTable = map[ViolationKind]ViolationKindMeta{
	ViolationRepoMissingCommand: {
		Kind:        ViolationRepoMissingCommand,
		Priority:    ViolationPriorityHigh,
		Reason:      "Command is missing from parity implementation (CLI, MCP, VS Code)",
		Solution:    "Implement the command in the missing platform",
		Autofixable: false,
	},
	ViolationRepoMissingTicketTracking: {
		Kind:        ViolationRepoMissingTicketTracking,
		Priority:    ViolationPriorityHigh,
		Reason:      "Ticket tracking code is missing or incomplete",
		Solution:    "Implement strict ticket tracking (open/close/log)",
		Autofixable: false,
	},
	ViolationCodeHeaderMissingRegion: {
		Kind:        ViolationCodeHeaderMissingRegion,
		Priority:    ViolationPriorityLow,
		Reason:      "Header region with license, filename, and contributors is required",
		Solution:    "Add header region with SPDX license, filename, and contributors",
		Autofixable: false,
	},
	ViolationCodeHeaderMissingFilename: {
		Kind:        ViolationCodeHeaderMissingFilename,
		Priority:    ViolationPriorityLow,
		Reason:      "Filename must be documented in header",
		Solution:    "Add filename comment in header region",
		Autofixable: false,
	},
	ViolationCodeHeaderMissingContributors: {
		Kind:        ViolationCodeHeaderMissingContributors,
		Priority:    ViolationPriorityLow,
		Reason:      "Contributors must be documented in header",
		Solution:    "Add contributor line in header region",
		Autofixable: false,
	},
	ViolationCodeHeaderMissingLicense: {
		Kind:        ViolationCodeHeaderMissingLicense,
		Priority:    ViolationPriorityLow,
		Reason:      "SPDX license identifier is required",
		Solution:    "Add SPDX license header comment",
		Autofixable: false,
	},
	ViolationCodeHeaderWrongLicense: {
		Kind:        ViolationCodeHeaderWrongLicense,
		Priority:    ViolationPriorityLow,
		Reason:      "License must be AGPL-3.0-or-later",
		Solution:    "Update license to AGPL-3.0-or-later",
		Autofixable: false,
	},
	ViolationCodeSectionEmpty: {
		Kind:        ViolationCodeSectionEmpty,
		Priority:    ViolationPriorityLow,
		Reason:      "Empty sections should be removed",
		Solution:    "Remove empty section or add content",
		Autofixable: true,
	},
	ViolationCodeSectionOrphanDefinition: {
		Kind:        ViolationCodeSectionOrphanDefinition,
		Priority:    ViolationPriorityLow,
		Reason:      "All code must be inside named sections",
		Solution:    "Move code into an existing section or add a new section",
		Autofixable: false,
	},
	ViolationCodeSectionMissingStartName: {
		Kind:        ViolationCodeSectionMissingStartName,
		Priority:    ViolationPriorityLow,
		Reason:      "Section start marker must have a name",
		Solution:    "Add name to section start marker",
		Autofixable: false,
	},
	ViolationCodeSectionMissingEndName: {
		Kind:        ViolationCodeSectionMissingEndName,
		Priority:    ViolationPriorityLow,
		Reason:      "Section end marker should have matching name",
		Solution:    "Add matching name to section end marker",
		Autofixable: true,
	},
	ViolationCodeSectionNameMismatch: {
		Kind:        ViolationCodeSectionNameMismatch,
		Priority:    ViolationPriorityLow,
		Reason:      "Section start and end names must match",
		Solution:    "Fix section end name to match start name",
		Autofixable: true,
	},
	ViolationCodeCommentInline: {
		Kind:        ViolationCodeCommentInline,
		Priority:    ViolationPriorityLow,
		Reason:      "Inline comments are forbidden",
		Solution:    "Remove inline comment",
		Autofixable: true,
	},
	ViolationCodeCommentBlock: {
		Kind:        ViolationCodeCommentBlock,
		Priority:    ViolationPriorityLow,
		Reason:      "Block comments are forbidden",
		Solution:    "Remove block comment",
		Autofixable: true,
	},
	ViolationCodeCommentJSDoc: {
		Kind:        ViolationCodeCommentJSDoc,
		Priority:    ViolationPriorityLow,
		Reason:      "JSDoc comments are forbidden",
		Solution:    "Remove JSDoc comment",
		Autofixable: true,
	},
	ViolationDevDocsMissingFile: {
		Kind:        ViolationDevDocsMissingFile,
		Priority:    ViolationPriorityLow,
		Reason:      "File exists but has no section in AGENTS.md Codebase",
		Solution:    "Add ## 📄 PATH section in AGENTS.md",
		Autofixable: true,
	},
	ViolationDevDocsMissingFolder: {
		Kind:        ViolationDevDocsMissingFolder,
		Priority:    ViolationPriorityLow,
		Reason:      "Folder exists but has no section in AGENTS.md Codebase",
		Solution:    "Add ## 📁 PATH section in AGENTS.md",
		Autofixable: true,
	},
	ViolationDevDocsWrongFilePath: {
		Kind:        ViolationDevDocsWrongFilePath,
		Priority:    ViolationPriorityLow,
		Reason:      "File section path does not match actual file path",
		Solution:    "Update file section path to match actual path",
		Autofixable: true,
	},
	ViolationDevDocsWrongFolderPath: {
		Kind:        ViolationDevDocsWrongFolderPath,
		Priority:    ViolationPriorityLow,
		Reason:      "Folder section path does not match actual folder path",
		Solution:    "Update folder section path to match actual path",
		Autofixable: true,
	},
	ViolationDevDocsWrongFileName: {
		Kind:        ViolationDevDocsWrongFileName,
		Priority:    ViolationPriorityLow,
		Reason:      "File section name format is incorrect (should be ## 📄 PATH)",
		Solution:    "Rename section to ## 📄 PATH",
		Autofixable: true,
	},
	ViolationDevDocsWrongFolderName: {
		Kind:        ViolationDevDocsWrongFolderName,
		Priority:    ViolationPriorityLow,
		Reason:      "Folder section name format is incorrect (should be ## 📁 PATH/)",
		Solution:    "Rename section to ## 📁 PATH/",
		Autofixable: true,
	},
	ViolationDevDocsWrongFileOrder: {
		Kind:        ViolationDevDocsWrongFileOrder,
		Priority:    ViolationPriorityLow,
		Reason:      "File sections are not in alphabetical order",
		Solution:    "Reorder file sections alphabetically",
		Autofixable: true,
	},
	ViolationDevDocsWrongFolderOrder: {
		Kind:        ViolationDevDocsWrongFolderOrder,
		Priority:    ViolationPriorityLow,
		Reason:      "Folder sections are not in alphabetical order",
		Solution:    "Reorder folder sections alphabetically",
		Autofixable: true,
	},
	ViolationDevDocsMissingComponent: {
		Kind:        ViolationDevDocsMissingComponent,
		Priority:    ViolationPriorityLow,
		Reason:      "Package.json workspace has no corresponding component in README.md",
		Solution:    "Add component section in README.md Components",
		Autofixable: true,
	},
	ViolationDevDocsWrongComponentName: {
		Kind:        ViolationDevDocsWrongComponentName,
		Priority:    ViolationPriorityLow,
		Reason:      "Component section name does not match workspace name",
		Solution:    "Rename component section to match workspace",
		Autofixable: true,
	},
	ViolationDevDocsWrongComponentOrder: {
		Kind:        ViolationDevDocsWrongComponentOrder,
		Priority:    ViolationPriorityLow,
		Reason:      "Component sections are not in package.json workspaces order",
		Solution:    "Reorder components to match package.json workspaces",
		Autofixable: true,
	},
	ViolationSketchpadImportThirdParty: {
		Kind:        ViolationSketchpadImportThirdParty,
		Priority:    ViolationPriorityHigh,
		Reason:      "Third party imports must only be in elements.tsx",
		Solution:    "Move third party import to elements.tsx and re-export from there",
		Autofixable: false,
	},
	ViolationSketchpadStateMultipleMachines: {
		Kind:        ViolationSketchpadStateMultipleMachines,
		Priority:    ViolationPriorityHigh,
		Reason:      "Only one state machine is allowed (createMachine can only be used once)",
		Solution:    "Consolidate state management into a single state machine",
		Autofixable: false,
	},
	ViolationSketchpadStateCreateActor: {
		Kind:        ViolationSketchpadStateCreateActor,
		Priority:    ViolationPriorityHigh,
		Reason:      "createActor is forbidden in sketchpad",
		Solution:    "Remove createActor usage and use the single state machine instead",
		Autofixable: false,
	},
	ViolationSketchpadStateYjsAppState: {
		Kind:        ViolationSketchpadStateYjsAppState,
		Priority:    ViolationPriorityHigh,
		Reason:      "Yjs should only be used for kit data synchronization, not app state",
		Solution:    "Move app state to the state machine and use Yjs only for kit data sync",
		Autofixable: false,
	},
	ViolationSketchpadStateForbiddenStore: {
		Kind:        ViolationSketchpadStateForbiddenStore,
		Priority:    ViolationPriorityHigh,
		Reason:      "Stores outside of State Management sections are forbidden",
		Solution:    "Move store to a State Management section or remove it",
		Autofixable: false,
	},
	ViolationSketchpadHooksNonTriadic: {
		Kind:        ViolationSketchpadHooksNonTriadic,
		Priority:    ViolationPriorityHigh,
		Reason:      "UI elements must use triadic hooks pattern [state, setState, canSetState]=useSELECTOR()",
		Solution:    "Refactor to use triadic hook pattern with useSELECTOR",
		Autofixable: false,
	},
}

func (k ViolationKind) Info() ViolationKindMeta {
	if info, ok := violationKindInfoTable[k]; ok {
		return info
	}
	return ViolationKindMeta{
		Kind:        k,
		Priority:    ViolationPriorityLow,
		Reason:      "Unknown violation",
		Solution:    "Fix the violation",
		Autofixable: false,
	}
}

type PolicyDef struct {
	ID          string            `json:"id"`
	Name        string            `json:"name"`
	Description string            `json:"description"`
	Scopes      []string          `json:"scopes"`
	Priority    ViolationPriority `json:"priority"`
	Kinds       []ViolationKind   `json:"kinds"`
	Run         PolicyFunc        `json:"-"`
}

type AnalyzeReport struct {
	Timestamp  string      `json:"timestamp"`
	Status     string      `json:"status"`
	Scope      string      `json:"scope"`
	Summary    Summary     `json:"summary"`
	Violations []Violation `json:"violations"`
}

type Summary struct {
	Total      int            `json:"total"`
	ByPriority map[string]int `json:"byPriority"`
	ByKind     map[string]int `json:"byKind"`
}

type FileCache struct {
	FilePath   string      `json:"filePath"`
	Hash       string      `json:"hash"`
	Timestamp  string      `json:"timestamp"`
	Violations []Violation `json:"violations"`
}

type OutputType string

const (
	OutputInfo    OutputType = "info"
	OutputSuccess OutputType = "success"
	OutputError   OutputType = "error"
	OutputWarn    OutputType = "warn"
	OutputPlain   OutputType = "plain"
)

type OutputLine struct {
	Type OutputType `json:"type"`
	Text string     `json:"text"`
}

type CommandOutput struct {
	Lines    []OutputLine `json:"lines"`
	ExitCode int          `json:"exitCode"`
}

type ToolResult struct {
	Output CommandOutput `json:"output"`
	Data   interface{}   `json:"data,omitempty"`
	Error  string        `json:"error,omitempty"`
}

type ContributorTicket struct {
	Year     int          `json:"year"`
	Month    int          `json:"month"`
	Day      int          `json:"day"`
	Slug     string       `json:"slug"`
	Status   TicketStatus `json:"status"`
	FilePath string       `json:"filePath,omitempty"`
}

type ContributorCommit struct {
	Title string `json:"title"`
	Sha   string `json:"sha"`
}

type ContributorContributionsStorage struct {
	Bundles     []string            `json:"bundles,omitempty"`
	Folders     []string            `json:"folders,omitempty"`
	Files       []string            `json:"files,omitempty"`
	Regions     []string            `json:"regions,omitempty"`
	Definitions []string            `json:"definitions,omitempty"`
	Tickets     []ContributorTicket `json:"tickets,omitempty"`
	Commits     []ContributorCommit `json:"commits,omitempty"`
	Lines       *LineMetrics        `json:"lines,omitempty"`
}

// #region Codebase Types

type BundleMetricsInternal struct {
	Folders     int `json:"folders"`
	Files       int `json:"files"`
	Sections    int `json:"sections"`
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
	Violations  int `json:"violations"`
}

type FolderMetricsInternal struct {
	Files      int `json:"files"`
	Lines      int `json:"lines"`
	Violations int `json:"violations"`
}

type FileMetricsInternal struct {
	Sections    int `json:"sections"`
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
}

type SectionMetricsInternal struct {
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
	Violations  int `json:"violations"`
}

type DefinitionMetricsInternal struct {
	Definitions int `json:"definitions"`
	Lines       int `json:"lines"`
	Violations  int `json:"violations"`
}

type RangePosition struct {
	Line   int `json:"line"`
	Column int `json:"column"`
}

type FileRange struct {
	Start RangePosition `json:"start"`
	End   RangePosition `json:"end"`
}

type ViolationFile struct {
	ID    string     `json:"id"`
	Path  string     `json:"path"`
	URI   string     `json:"uri"`
	Range *FileRange `json:"range,omitempty"`
}

type ViolationFolder struct {
	ID   string `json:"id"`
	Path string `json:"path"`
	URI  string `json:"uri"`
}

type CodebaseViolation struct {
	ID          string            `json:"id"`
	Folders     []ViolationFolder `json:"folders,omitempty"`
	Files       []ViolationFile   `json:"files,omitempty"`
	Kind        ViolationKind     `json:"kind"`
	Priority    ViolationPriority `json:"priority"`
	Autofixable bool              `json:"autofixable"`
	Reason      string            `json:"reason"`
	Solution    string            `json:"solution"`
}

type CodebaseBundle struct {
	ID           string                 `json:"id"`
	Folder       string                 `json:"folder"`
	URI          string                 `json:"uri"`
	Contributors []string               `json:"contributors,omitempty"`
	Tickets      []string               `json:"tickets,omitempty"`
	Metrics      *BundleMetricsInternal `json:"metrics,omitempty"`
}

type CodebaseFolder struct {
	ID       string                 `json:"id"`
	Path     string                 `json:"path"`
	URI      string                 `json:"uri"`
	Name     string                 `json:"name"`
	ParentID *string                `json:"parentId,omitempty"`
	Metrics  *FolderMetricsInternal `json:"metrics,omitempty"`
}

type FileViolationRef struct {
	Kind        ViolationKind     `json:"kind"`
	Priority    ViolationPriority `json:"priority"`
	Autofixable bool              `json:"autofixable"`
	Solution    string            `json:"solution"`
}

type CodebaseFile struct {
	ID         string               `json:"id"`
	Path       string               `json:"path"`
	URI        string               `json:"uri"`
	Metrics    *FileMetricsInternal `json:"metrics,omitempty"`
	Violations []FileViolationRef   `json:"violations,omitempty"`
}

type CodebaseSection struct {
	ID      string                  `json:"id"`
	Path    string                  `json:"path"`
	URI     string                  `json:"uri"`
	Metrics *SectionMetricsInternal `json:"metrics,omitempty"`
}

type CodebaseDefinition struct {
	ID      string                     `json:"id"`
	Path    string                     `json:"path"`
	URI     string                     `json:"uri"`
	Metrics *DefinitionMetricsInternal `json:"metrics,omitempty"`
}

type ContributorBundleContrib struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

type ContributorFolderContrib struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

type ContributorFileContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

type ContributorSectionContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

type ContributorDefinitionContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

type ContributorContributionsInternal struct {
	Bundles     []ContributorBundleContrib     `json:"bundles,omitempty"`
	Folders     []ContributorFolderContrib     `json:"folders,omitempty"`
	Files       []ContributorFileContrib       `json:"files,omitempty"`
	Sections    []ContributorSectionContrib    `json:"sections,omitempty"`
	Definitions []ContributorDefinitionContrib `json:"definitions,omitempty"`
}

type ContributorMetricsInternal struct {
	Commits     int `json:"commits"`
	Tickets     int `json:"tickets"`
	Bundles     int `json:"bundles"`
	Folders     int `json:"folders"`
	Files       int `json:"files"`
	Lines       int `json:"lines"`
	Sections    int `json:"sections"`
	Definitions int `json:"definitions"`
}

type CodebaseContributor struct {
	ID            string                            `json:"id"`
	URI           string                            `json:"uri"`
	Path          string                            `json:"path"`
	Name          string                            `json:"name,omitempty"`
	Icons         *ContributorIcons                 `json:"icons,omitempty"`
	Emails        []string                          `json:"emails,omitempty"`
	Links         map[string]string                 `json:"links,omitempty"`
	Contributions *ContributorContributionsInternal `json:"contributions,omitempty"`
	Metrics       *ContributorMetricsInternal       `json:"metrics,omitempty"`
}

type TicketDateInfo struct {
	Created  string `json:"created,omitempty"`
	Finished string `json:"finished,omitempty"`
}

type TicketBundleContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

type TicketFolderContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

type TicketFileContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

type TicketSectionContribInfo struct {
	ID      string        `json:"id"`
	Metrics *CountMetrics `json:"metrics,omitempty"`
}

type TicketDefinitionContrib struct {
	ID      string       `json:"id"`
	Metrics *LineMetrics `json:"metrics,omitempty"`
}

type CodebaseTicket struct {
	ID          string                     `json:"id"`
	Path        string                     `json:"path"`
	URI         string                     `json:"uri"`
	Date        *TicketDateInfo            `json:"date,omitempty"`
	Commit      string                     `json:"commit,omitempty"`
	Year        string                     `json:"year"`
	Month       string                     `json:"month"`
	Day         string                     `json:"day"`
	Slug        string                     `json:"slug"`
	Prompt      string                     `json:"prompt,omitempty"`
	LLM         string                     `json:"llm,omitempty"`
	Author      string                     `json:"author,omitempty"`
	Status      TicketStatus               `json:"status"`
	Bundles     []TicketBundleContribInfo  `json:"bundles,omitempty"`
	Folders     []TicketFolderContribInfo  `json:"folders,omitempty"`
	Files       []TicketFileContribInfo    `json:"files,omitempty"`
	Sections    []TicketSectionContribInfo `json:"sections,omitempty"`
	Definitions []TicketDefinitionContrib  `json:"definitions,omitempty"`
}

type PolicyViolationRef struct {
	Kind        ViolationKind     `json:"kind"`
	Priority    ViolationPriority `json:"priority"`
	Autofixable bool              `json:"autofixable"`
	Solution    string            `json:"solution"`
}

type CodebasePolicy struct {
	ID         string               `json:"id"`
	Name       string               `json:"name"`
	Scopes     []string             `json:"scopes,omitempty"`
	Violations []PolicyViolationRef `json:"violations,omitempty"`
}

type TreeNodeKind string

const (
	TreeNodeRepo       TreeNodeKind = "repo"
	TreeNodeBundle     TreeNodeKind = "bundle"
	TreeNodeFolder     TreeNodeKind = "folder"
	TreeNodeFile       TreeNodeKind = "file"
	TreeNodeSection    TreeNodeKind = "section"
	TreeNodeDefinition TreeNodeKind = "definition"
)

type TreeNode struct {
	Kind     TreeNodeKind         `json:"kind"`
	Children map[string]*TreeNode `json:"children,omitempty"`
}

type Codebase struct {
	Bundles      []CodebaseBundle      `json:"bundles"`
	Folders      []CodebaseFolder      `json:"folders"`
	Files        []CodebaseFile        `json:"files"`
	Sections     []CodebaseSection     `json:"sections"`
	Definitions  []CodebaseDefinition  `json:"definitions"`
	Contributors []CodebaseContributor `json:"contributors"`
	Tickets      []CodebaseTicket      `json:"tickets"`
	Policies     []CodebasePolicy      `json:"policies"`
	Violations   []CodebaseViolation   `json:"violations"`
	Tree         map[string]*TreeNode  `json:"tree"`
}

// #endregion Codebase Types

// #endregion Types

// #region Utils

var (
	rootDir  string
	executor *Executor
)

func init() {
	wd, err := os.Getwd()
	if err != nil {
		rootDir = "."
	} else {
		rootDir = findRepoRoot(wd)
	}
	SetRootDir(rootDir)
	var e error
	executor, e = NewExecutorWithContext(rootDir, NewRepoContext(rootDir))
	if e != nil {
		fmt.Fprintf(os.Stderr, "Warning: Failed to initialize GraphQL executor: %v\n", e)
	}
}

func GetRootDir() string {
	return rootDir
}

func SetRootDir(dir string) {
	rootDir = dir
	InvalidateProjectCache()
}

func GetRepoMetaDir() string {
	return filepath.Join(rootDir, ".semio-repo")
}

func GetRepoMetaPath(path string) string {
	return filepath.Join(GetRepoMetaDir(), path)
}

func findRepoRoot(startDir string) string {
	dir, err := filepath.Abs(startDir)
	if err != nil {
		return startDir
	}
	// First, walk up looking for .git (prioritize git root over go.mod)
	searchDir := dir
	for {
		if _, err := os.Stat(filepath.Join(searchDir, ".git")); err == nil {
			return searchDir
		}
		parent := filepath.Dir(searchDir)
		if parent == searchDir {
			break
		}
		searchDir = parent
	}
	// If no .git found, fall back to looking for go.mod
	searchDir = dir
	for {
		if _, err := os.Stat(filepath.Join(searchDir, "go.mod")); err == nil {
			return searchDir
		}
		parent := filepath.Dir(searchDir)
		if parent == searchDir {
			return startDir
		}
		searchDir = parent
	}
}

type GitignorePattern struct {
	Pattern string
	Negate  bool
}

var (
	cachedGitignorePatterns []GitignorePattern
	gitignoreLoaded         bool
	gitignoreMutex          sync.Mutex
)

func getGitignorePatterns() []GitignorePattern {
	gitignoreMutex.Lock()
	defer gitignoreMutex.Unlock()
	if gitignoreLoaded {
		return cachedGitignorePatterns
	}
	gitignorePath := filepath.Join(rootDir, ".gitignore")
	content, err := os.ReadFile(gitignorePath)
	if err != nil {
		gitignoreLoaded = true
		return nil
	}
	for _, line := range strings.Split(string(content), "\n") {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}
		if strings.HasPrefix(line, "#") {
			continue
		}
		negate := false
		if strings.HasPrefix(line, "!") {
			negate = true
			line = strings.TrimSpace(strings.TrimPrefix(line, "!"))
		}
		if line == "" {
			continue
		}
		rooted := strings.HasPrefix(line, "/")
		if rooted {
			line = strings.TrimPrefix(line, "/")
		}
		trimmed := strings.TrimSuffix(line, "/")
		if trimmed == "" {
			continue
		}
		patterns := []string{trimmed}
		if !strings.HasSuffix(trimmed, "/**") {
			patterns = append(patterns, trimmed+"/**")
		}
		for _, pattern := range patterns {
			if !strings.Contains(pattern, "/") && !rooted {
				pattern = "**/" + pattern
			}
			cachedGitignorePatterns = append(cachedGitignorePatterns, GitignorePattern{Pattern: pattern, Negate: negate})
		}
	}
	gitignoreLoaded = true
	return cachedGitignorePatterns
}

func isGitIgnored(filePath string) bool {
	if filepath.Base(filePath) == "LICENSE.md" {
		return true
	}

	info, err := os.Stat(filePath)
	if err == nil && !info.IsDir() {
		if GetLanguage(filePath) == nil {
			return true
		}
	}

	relPath := normalizeRepoPath(filePath)
	if relPath == "" {
		return false
	}
	ignored := false
	for _, pattern := range getGitignorePatterns() {
		if pattern.Pattern == "" {
			continue
		}
		if matched, _ := doublestar.Match(pattern.Pattern, relPath); matched {
			if pattern.Negate {
				ignored = false
			} else {
				ignored = true
			}
		}
	}
	return ignored
}

func policyAppliesToScope(policyID string, scope Scope) bool {
	switch policyID {
	case "code":
		return scope.Kind == ScopeFile && isSourceFile(scope.FilePath)
	case "dev-docs":
		return scope.Kind == ScopeRepo || scope.Kind == ScopeFolder || scope.Kind == ScopeFile
	default:
		return true
	}
}

func isSourceFile(filePath string) bool {
	ext := filepath.Ext(filePath)
	return ext == ".ts" || ext == ".tsx" || ext == ".js" || ext == ".jsx" ||
		ext == ".py" || ext == ".go" || ext == ".cs"
}

func NormalizePath(p string) string {
	return strings.ReplaceAll(p, "\\", "/")
}

func EnsureDir(dirPath string) error {
	return os.MkdirAll(dirPath, 0755)
}

func GetRelativePath(filePath string) string {
	rel, err := filepath.Rel(rootDir, filePath)
	if err != nil {
		return filePath
	}
	return NormalizePath(rel)
}

func ReadTextFile(filePath string) (string, error) {
	data, err := os.ReadFile(filePath)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

func WriteTextFile(filePath string, content string) error {
	if err := EnsureDir(filepath.Dir(filePath)); err != nil {
		return err
	}
	return os.WriteFile(filePath, []byte(content), 0644)
}

func WriteJSONFile(filePath string, data interface{}) error {
	jsonBytes, err := json.MarshalIndent(data, "", "  ")
	if err != nil {
		return err
	}
	return WriteTextFile(filePath, string(jsonBytes)+"\n")
}

func ReadJSONFile(filePath string, v interface{}) error {
	data, err := ReadTextFile(filePath)
	if err != nil {
		return err
	}
	return json.Unmarshal([]byte(data), v)
}

func FileExists(path string) bool {
	_, err := os.Stat(path)
	return err == nil
}

func IsDir(path string) bool {
	info, err := os.Stat(path)
	if err != nil {
		return false
	}
	return info.IsDir()
}

func LoadGitignore(cwd string) ([]string, error) {
	gitignorePath := filepath.Join(cwd, ".gitignore")
	if !FileExists(gitignorePath) {
		return nil, nil
	}
	content, err := ReadTextFile(gitignorePath)
	if err != nil {
		return nil, err
	}
	var patterns []string
	for _, line := range strings.Split(content, "\n") {
		line = strings.TrimSpace(line)
		if line != "" && !strings.HasPrefix(line, "#") {
			patterns = append(patterns, line)
		}
	}
	return patterns, nil
}

func SimpleGlob(pattern string, cwd string, ignorePatterns []string, respectGitignore bool) ([]string, error) {
	if cwd == "" {
		cwd = rootDir
	}
	var gitignorePatterns []string
	if respectGitignore {
		var err error
		gitignorePatterns, err = LoadGitignore(cwd)
		if err != nil {
			return nil, err
		}
	}
	allIgnore := append(ignorePatterns, gitignorePatterns...)
	var files []string
	absPattern := filepath.Join(cwd, pattern)
	matches, err := doublestar.FilepathGlob(absPattern)
	if err != nil {
		return nil, err
	}
	for _, match := range matches {
		rel, err := filepath.Rel(cwd, match)
		if err != nil {
			continue
		}
		relNorm := NormalizePath(rel)
		ignored := false
		for _, ig := range allIgnore {
			if matched, _ := doublestar.Match(ig, relNorm); matched {
				ignored = true
				break
			}
		}
		if !ignored {
			files = append(files, relNorm)
		}
	}
	return files, nil
}

func globByExtension(root string, patternBase string, exts []string, ignorePatterns []string, respectGitignore bool) ([]string, error) {
	base := strings.TrimSuffix(patternBase, "/**/*")
	if patternBase == "**/*" {
		base = ""
	}
	absBase := filepath.Join(root, base)
	info, err := os.Stat(absBase)
	if err != nil || !info.IsDir() {
		return nil, nil
	}
	gitignorePatterns := []string{}
	if respectGitignore {
		gitignorePatterns, err = LoadGitignore(root)
		if err != nil {
			return nil, err
		}
	}
	allIgnore := append(ignorePatterns, gitignorePatterns...)
	allowed := make(map[string]struct{}, len(exts))
	for _, ext := range exts {
		allowed[strings.ToLower(ext)] = struct{}{}
	}
	results := make([]string, 0)
	err = filepath.WalkDir(absBase, func(path string, d os.DirEntry, walkErr error) error {
		if walkErr != nil {
			return walkErr
		}
		name := d.Name()
		if d.IsDir() && strings.HasPrefix(name, ".") {
			return filepath.SkipDir
		}
		rel, relErr := filepath.Rel(root, path)
		if relErr != nil {
			return nil
		}
		rel = strings.TrimPrefix(NormalizePath(rel), "./")
		if rel == "" || rel == "." {
			return nil
		}
		for _, ig := range allIgnore {
			if matched, _ := doublestar.Match(ig, rel); matched {
				if d.IsDir() {
					return filepath.SkipDir
				}
				return nil
			}
		}
		if d.IsDir() {
			return nil
		}
		ext := strings.TrimPrefix(strings.ToLower(filepath.Ext(rel)), ".")
		if _, ok := allowed[ext]; !ok {
			return nil
		}
		results = append(results, rel)
		return nil
	})
	if err != nil {
		return nil, err
	}
	return results, nil
}

func ISOTimestamp() string {
	return time.Now().UTC().Format(time.RFC3339)
}

func FormatDate(t time.Time) (year, month, day int) {
	return t.Year(), int(t.Month()), t.Day()
}

func PadNumber(n, width int) string {
	return fmt.Sprintf("%0*d", width, n)
}

func Slugify(text string) string {
	re := regexp.MustCompile(`[^A-Z0-9]+`)
	slug := re.ReplaceAllString(strings.ToUpper(text), "-")
	return strings.Trim(slug, "-")
}

func ExecCommand(command string, args []string, cwd string) (stdout, stderr string, exitCode int) {
	if cwd == "" {
		cwd = rootDir
	}
	cmd := exec.Command(command, args...)
	cmd.Dir = cwd
	var stdoutBuf, stderrBuf strings.Builder
	cmd.Stdout = &stdoutBuf
	cmd.Stderr = &stderrBuf
	err := cmd.Run()
	exitCode = 0
	if err != nil {
		if exitErr, ok := err.(*exec.ExitError); ok {
			exitCode = exitErr.ExitCode()
		} else {
			exitCode = 1
		}
	}
	return stdoutBuf.String(), stderrBuf.String(), exitCode
}

func GetGitAuthor() string {
	name, _, _ := ExecCommand("git", []string{"config", "--get", "user.name"}, "")
	email, _, _ := ExecCommand("git", []string{"config", "--get", "user.email"}, "")
	name = strings.TrimSpace(name)
	email = strings.TrimSpace(email)
	if email != "" {
		return fmt.Sprintf("%s <%s>", name, email)
	}
	return name
}

func GetGitAuthorGithub() string {
	name, _, _ := ExecCommand("git", []string{"config", "--get", "user.name"}, "")
	name = strings.TrimSpace(name)
	email, _, _ := ExecCommand("git", []string{"config", "--get", "user.email"}, "")
	email = strings.TrimSpace(email)

	fallback := name
	if email != "" {
		fallback = fmt.Sprintf("%s <%s>", name, email)
	}

	// Try to find a matching contributor by email
	if email != "" {
		contributors, err := ListContributors()
		if err == nil {
			for _, c := range contributors {
				for _, e := range c.Emails {
					if strings.EqualFold(e, email) {
						return c.Github
					}
				}
			}
		}
	}

	return fallback
}

func GetGitCommit() string {
	commit, _, _ := ExecCommand("git", []string{"rev-parse", "HEAD"}, "")
	return strings.TrimSpace(commit)
}

func GetGitIgnoredSet(paths []string) map[string]bool {
	if len(paths) == 0 {
		return make(map[string]bool)
	}
	args := append([]string{"check-ignore"}, paths...)
	stdout, _, _ := ExecCommand("git", args, "")
	ignored := make(map[string]bool)
	for _, line := range strings.Split(stdout, "\n") {
		line = strings.TrimSpace(line)
		if line != "" {
			ignored[NormalizePath(line)] = true
		}
	}
	return ignored
}

func NewOutput() *CommandOutput {
	return &CommandOutput{Lines: []OutputLine{}, ExitCode: 0}
}

func (o *CommandOutput) Info(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputInfo, Text: text})
}

func (o *CommandOutput) Success(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputSuccess, Text: text})
}

func (o *CommandOutput) Error(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputError, Text: text})
	o.ExitCode = 1
}

func (o *CommandOutput) Warn(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputWarn, Text: text})
}

func (o *CommandOutput) Plain(text string) {
	o.Lines = append(o.Lines, OutputLine{Type: OutputPlain, Text: text})
}

func (o *CommandOutput) Print() {
	for _, line := range o.Lines {
		fmt.Println(line.Text)
	}
}

func (o *CommandOutput) Json(data interface{}) {
	bytes, err := json.MarshalIndent(data, "", "  ")
	if err == nil {
		o.Lines = append(o.Lines, OutputLine{Type: OutputPlain, Text: string(bytes)})
	}
}

func ListDirEntries(dir string, dirsOnly bool) ([]string, error) {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil, err
	}
	var names []string
	for _, e := range entries {
		if strings.HasPrefix(e.Name(), ".") {
			continue
		}
		if dirsOnly && !e.IsDir() {
			continue
		}
		if !dirsOnly && e.IsDir() {
			continue
		}
		names = append(names, e.Name())
	}
	return names, nil
}

func WalkDir(dir string, fn func(path string, isDir bool) error) error {
	return filepath.Walk(dir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if strings.HasPrefix(info.Name(), ".") {
			if info.IsDir() {
				return filepath.SkipDir
			}
			return nil
		}
		return fn(path, info.IsDir())
	})
}

func ParseScope(raw string) Scope {
	if raw == "" || raw == "@semio" {
		return Scope{Raw: "@semio", Kind: ScopeRepo}
	}
	if strings.Contains(raw, "§") {
		parts := strings.SplitN(raw, "§", 2)
		return Scope{Raw: raw, Kind: ScopeDefinition, FilePath: parts[0], DefinitionName: parts[1]}
	}
	if strings.Contains(raw, "#") {
		parts := strings.Split(raw, "#")
		return Scope{Raw: raw, Kind: ScopeSection, FilePath: parts[0], SectionPath: parts[1:]}
	}
	ext := strings.ToLower(filepath.Ext(raw))
	codeExtensions := map[string]bool{".ts": true, ".tsx": true, ".js": true, ".jsx": true, ".py": true, ".cs": true, ".go": true, ".json": true, ".md": true, ".yaml": true, ".yml": true, ".sql": true, ".graphql": true}
	if codeExtensions[ext] {
		return Scope{Raw: raw, Kind: ScopeFile, FilePath: raw}
	}
	if strings.HasPrefix(raw, "@semio/") {
		return Scope{Raw: raw, Kind: ScopeProject, ProjectName: raw}
	}
	if strings.HasSuffix(raw, "/") {
		return Scope{Raw: raw, Kind: ScopeFolder, FilePath: raw}
	}
	return Scope{Raw: raw, Kind: ScopeFolder, FilePath: raw}
}

func ReadLines(filePath string) ([]string, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return nil, err
	}
	defer file.Close()
	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	return lines, scanner.Err()
}

// #endregion Utils

// #region Sections

func ParseCodeSections(content string, languageName string) []Section {
	lang := GetLanguageByName(languageName)
	if lang == nil || !lang.SupportsSections() {
		return nil
	}
	return lang.ParseSections(content)
}

func ParseMarkdownSectionsInternal(content string) []Section {
	lines := strings.Split(content, "\n")
	var sections []Section
	type stackItem struct {
		level   int
		section *Section
	}
	var stack []stackItem
	headerRe := regexp.MustCompile(`^(#{1,6})\s+(.+?)\s*$`)
	frontmatterLines := 0
	if strings.HasPrefix(content, "---") {
		endIndex := strings.Index(content[3:], "---")
		if endIndex != -1 {
			frontmatterContent := content[:endIndex+6]
			frontmatterLines = strings.Count(frontmatterContent, "\n")
		}
	}
	charIndex := 0
	for i, line := range lines {
		lineStart := charIndex
		if match := headerRe.FindStringSubmatch(line); match != nil {
			level := len(match[1])
			name := strings.TrimSpace(match[2])
			for len(stack) > 0 && stack[len(stack)-1].level >= level {
				popped := stack[len(stack)-1]
				popped.section.EndLine = frontmatterLines + i
				popped.section.EndIndex = lineStart - 1
				stack = stack[:len(stack)-1]
			}
			section := &Section{
				Name:       name,
				StartLine:  frontmatterLines + i + 1,
				EndLine:    -1,
				StartIndex: lineStart,
				EndIndex:   -1,
				Children:   []Section{},
			}
			if len(stack) > 0 {
				parent := stack[len(stack)-1]
				parent.section.Children = append(parent.section.Children, *section)
				section = &parent.section.Children[len(parent.section.Children)-1]
			} else {
				sections = append(sections, *section)
				section = &sections[len(sections)-1]
			}
			stack = append(stack, stackItem{level: level, section: section})
		}
		charIndex += len(line) + 1
	}
	for len(stack) > 0 {
		popped := stack[len(stack)-1]
		popped.section.EndLine = frontmatterLines + len(lines)
		popped.section.EndIndex = len(content)
		stack = stack[:len(stack)-1]
	}
	return sections
}

type JsonSectionLocation struct {
	Path       string
	KeyStart   int
	KeyEnd     int
	ValueStart int
	ValueEnd   int
	Section    *Section
}

type jsonContext struct {
	kind      byte
	section   *Section
	path      string
	expectKey bool
	location  *JsonSectionLocation
}

func ParseJSONSectionsDetailed(content string) ([]Section, map[string]*JsonSectionLocation, error) {
	var sections []Section
	locations := make(map[string]*JsonSectionLocation)
	var stack []jsonContext
	line := 1
	inString := false
	escape := false
	stringStart := 0
	stringBuf := strings.Builder{}
	pendingKey := ""
	pendingKeyStart := 0
	pendingKeyEnd := 0
	pendingKeyLine := 0
	var awaitingValue *JsonSectionLocation
	for i := 0; i < len(content); i++ {
		ch := content[i]
		if ch == '\n' {
			line++
		}
		if inString {
			if escape {
				escape = false
				stringBuf.WriteByte(ch)
				continue
			}
			if ch == '\\' {
				escape = true
				stringBuf.WriteByte(ch)
				continue
			}
			if ch == '"' {
				inString = false
				value := stringBuf.String()
				stringBuf.Reset()
				if len(stack) > 0 && stack[len(stack)-1].kind == '{' && stack[len(stack)-1].expectKey && awaitingValue == nil {
					pendingKey = value
					pendingKeyStart = stringStart
					pendingKeyEnd = i
					pendingKeyLine = line
					stack[len(stack)-1].expectKey = false
				} else if awaitingValue != nil && awaitingValue.ValueStart == stringStart {
					awaitingValue.ValueEnd = i
					awaitingValue.Section.EndLine = line
					awaitingValue.Section.EndIndex = i + 1
					awaitingValue = nil
				}
				continue
			}
			stringBuf.WriteByte(ch)
			continue
		}
		if ch == '"' {
			if awaitingValue != nil {
				awaitingValue.ValueStart = i
			}
			inString = true
			stringStart = i
			continue
		}
		if len(stack) > 0 && stack[len(stack)-1].kind == '{' && pendingKey != "" && ch == ':' {
			parent := stack[len(stack)-1].section
			path := pendingKey
			if stack[len(stack)-1].path != "" {
				path = stack[len(stack)-1].path + "/" + pendingKey
			}
			section := Section{
				Name:       pendingKey,
				StartLine:  pendingKeyLine,
				EndLine:    -1,
				StartIndex: pendingKeyStart,
				EndIndex:   -1,
				Children:   []Section{},
			}
			var sectionRef *Section
			if parent != nil {
				parent.Children = append(parent.Children, section)
				sectionRef = &parent.Children[len(parent.Children)-1]
			} else {
				sections = append(sections, section)
				sectionRef = &sections[len(sections)-1]
			}
			location := &JsonSectionLocation{
				Path:     path,
				KeyStart: pendingKeyStart,
				KeyEnd:   pendingKeyEnd,
				Section:  sectionRef,
			}
			locations[path] = location
			awaitingValue = location
			pendingKey = ""
			continue
		}
		if awaitingValue != nil {
			if ch == '{' || ch == '[' {
				awaitingValue.ValueStart = i
				stack = append(stack, jsonContext{
					kind:      ch,
					section:   awaitingValue.Section,
					path:      awaitingValue.Path,
					expectKey: ch == '{',
					location:  awaitingValue,
				})
				awaitingValue = nil
				continue
			}
			if ch == '-' || (ch >= '0' && ch <= '9') || ch == 't' || ch == 'f' || ch == 'n' {
				awaitingValue.ValueStart = i
				end := i
				for end < len(content) {
					c := content[end]
					if c == '\n' {
						line++
					}
					if c == ',' || c == '}' || c == ']' || c == ' ' || c == '\t' || c == '\r' || c == '\n' {
						break
					}
					end++
				}
				awaitingValue.ValueEnd = end - 1
				awaitingValue.Section.EndLine = line
				awaitingValue.Section.EndIndex = end
				awaitingValue = nil
				i = end - 1
				continue
			}
		}
		if ch == '{' || ch == '[' {
			if awaitingValue == nil {
				stack = append(stack, jsonContext{
					kind:      ch,
					section:   nil,
					path:      "",
					expectKey: ch == '{',
					location:  nil,
				})
			}
			continue
		}
		if ch == '}' || ch == ']' {
			if len(stack) > 0 {
				top := stack[len(stack)-1]
				stack = stack[:len(stack)-1]
				if top.location != nil {
					top.location.ValueEnd = i
					top.location.Section.EndLine = line
					top.location.Section.EndIndex = i + 1
				}
				if len(stack) > 0 && stack[len(stack)-1].kind == '{' {
					stack[len(stack)-1].expectKey = true
				}
			}
			continue
		}
		if ch == ',' {
			if len(stack) > 0 && stack[len(stack)-1].kind == '{' {
				stack[len(stack)-1].expectKey = true
			}
			continue
		}
	}
	for _, location := range locations {
		if location.Section.EndIndex == -1 {
			location.Section.EndLine = line
			location.Section.EndIndex = len(content)
		}
	}
	return sections, locations, nil
}

func ParseJSONSections(content string) []Section {
	sections, _, _ := ParseJSONSectionsDetailed(content)
	return sections
}

func ParseSections(content string, filePath string) []Section {
	language := GetLanguage(filePath)
	if language == nil {
		return nil
	}
	return language.ParseSections(content)
}

func ParseDefinitions(content string, filePath string) []Definition {
	language := GetLanguage(filePath)
	if language == nil {
		return nil
	}
	if !language.SupportsDefinitions() {
		return nil
	}
	lines := strings.Split(content, "\n")
	ranges := language.ParseDefinitions(content, lines)
	definitions := make([]Definition, len(ranges))
	for i, r := range ranges {
		definitions[i] = Definition{
			Name:      r.Name,
			Kind:      DefinitionKindImplementation,
			StartLine: r.Start,
			EndLine:   r.End,
			FilePath:  filePath,
		}
	}
	return definitions
}

func HydrateSectionsWithDefinitions(sections []Section, definitions []Definition) []Section {
	if len(sections) == 0 {
		return sections
	}
	newSections := make([]Section, len(sections))
	for i := range sections {
		newSections[i] = sections[i]
		start := newSections[i].StartLine
		end := newSections[i].EndLine

		var subset []Definition
		for _, def := range definitions {
			if def.StartLine >= start && def.EndLine <= end {
				subset = append(subset, def)
			}
		}

		newSections[i].Children = HydrateSectionsWithDefinitions(newSections[i].Children, subset)

		var myDefs []Definition
		for _, def := range subset {
			inChild := false
			for _, child := range newSections[i].Children {
				if def.StartLine >= child.StartLine && def.EndLine <= child.EndLine {
					inChild = true
					break
				}
			}
			if !inChild {
				myDefs = append(myDefs, def)
			}
		}
		newSections[i].Definitions = myDefs
	}
	return newSections
}

func NormalizeSectionPath(sectionPath string) []string {
	cleaned := strings.ReplaceAll(sectionPath, "#", "/")
	raw := strings.FieldsFunc(cleaned, func(r rune) bool { return r == '/' })
	var parts []string
	for _, part := range raw {
		if part != "" {
			parts = append(parts, part)
		}
	}
	return parts
}

func jsonLineStart(content string, index int) int {
	if index <= 0 {
		return 0
	}
	pos := strings.LastIndex(content[:index], "\n")
	if pos == -1 {
		return 0
	}
	return pos + 1
}

func jsonLineIndent(content string, index int) string {
	start := jsonLineStart(content, index)
	end := start
	for end < len(content) && (content[end] == ' ' || content[end] == '\t') {
		end++
	}
	return content[start:end]
}

func jsonIsWhitespace(ch byte) bool {
	return ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}

func jsonFindMatching(content string, start int, open byte, close byte) (int, bool) {
	if start < 0 || start >= len(content) || content[start] != open {
		return -1, false
	}
	inString := false
	escape := false
	depth := 0
	for i := start; i < len(content); i++ {
		ch := content[i]
		if inString {
			if escape {
				escape = false
				continue
			}
			if ch == '\\' {
				escape = true
				continue
			}
			if ch == '"' {
				inString = false
			}
			continue
		}
		if ch == '"' {
			inString = true
			continue
		}
		if ch == open {
			depth++
			continue
		}
		if ch == close {
			depth--
			if depth == 0 {
				return i, true
			}
		}
	}
	return -1, false
}

func jsonFindRootObjectRange(content string) (int, int, bool) {
	for i := 0; i < len(content); i++ {
		if jsonIsWhitespace(content[i]) {
			continue
		}
		if content[i] != '{' {
			return -1, -1, false
		}
		end, ok := jsonFindMatching(content, i, '{', '}')
		return i, end, ok
	}
	return -1, -1, false
}

func jsonFindObjectRange(content string, locations map[string]*JsonSectionLocation, path string) (int, int, bool) {
	if path == "" {
		return jsonFindRootObjectRange(content)
	}
	location, ok := locations[path]
	if !ok || location.ValueStart < 0 || location.ValueStart >= len(content) {
		return -1, -1, false
	}
	if content[location.ValueStart] != '{' {
		return -1, -1, false
	}
	return location.ValueStart, location.ValueEnd, true
}

func jsonObjectHasEntries(content string, start, end int) bool {
	for i := start + 1; i < end; i++ {
		if !jsonIsWhitespace(content[i]) {
			return true
		}
	}
	return false
}

func jsonFindFirstKeyIndent(content string, start, end int) string {
	depth := 0
	inString := false
	escape := false
	expectKey := true
	for i := start + 1; i < end; i++ {
		ch := content[i]
		if inString {
			if escape {
				escape = false
				continue
			}
			if ch == '\\' {
				escape = true
				continue
			}
			if ch == '"' {
				inString = false
			}
			continue
		}
		if ch == '"' {
			if depth == 0 && expectKey {
				return jsonLineIndent(content, i)
			}
			inString = true
			continue
		}
		if ch == '{' || ch == '[' {
			depth++
			expectKey = ch == '{'
			continue
		}
		if ch == '}' || ch == ']' {
			if depth > 0 {
				depth--
			}
			if depth == 0 {
				expectKey = true
			}
			continue
		}
		if ch == ',' && depth == 0 {
			expectKey = true
		}
	}
	return ""
}

func jsonInsertEntry(content string, objectStart, objectEnd int, entry string) (string, bool) {
	if objectStart < 0 || objectEnd <= objectStart {
		return content, false
	}
	hasEntries := jsonObjectHasEntries(content, objectStart, objectEnd)
	parentIndent := jsonLineIndent(content, objectEnd)
	childIndent := jsonFindFirstKeyIndent(content, objectStart, objectEnd)
	if childIndent == "" {
		childIndent = parentIndent + "  "
	}
	insert := ""
	if hasEntries {
		insert = ",\n" + childIndent + entry + "\n" + parentIndent
	} else {
		insert = "\n" + childIndent + entry + "\n" + parentIndent
	}
	return content[:objectEnd] + insert + content[objectEnd:], true
}

func jsonReplaceKey(content string, keyStart, keyEnd int, newName string) string {
	quoted := strconv.Quote(newName)
	return content[:keyStart] + quoted + content[keyEnd+1:]
}

func jsonExtractEntry(content string, keyStart int, valueEnd int) (string, int, int) {
	start := jsonLineStart(content, keyStart)
	end := valueEnd + 1
	for end < len(content) && jsonIsWhitespace(content[end]) {
		end++
	}
	if end < len(content) && content[end] == ',' {
		end++
	} else {
		left := start
		for left > 0 && jsonIsWhitespace(content[left-1]) {
			left--
		}
		if left > 0 && content[left-1] == ',' {
			start = left - 1
		}
	}
	entry := strings.TrimSpace(content[start:end])
	entry = strings.TrimSuffix(entry, ",")
	return entry, start, end
}

func jsonRenameEntryKey(entry string, newName string) string {
	inString := false
	escape := false
	stringStart := -1
	for i := 0; i < len(entry); i++ {
		ch := entry[i]
		if inString {
			if escape {
				escape = false
				continue
			}
			if ch == '\\' {
				escape = true
				continue
			}
			if ch == '"' && stringStart >= 0 {
				quoted := strconv.Quote(newName)
				return entry[:stringStart] + quoted + entry[i+1:]
			}
			continue
		}
		if ch == '"' {
			inString = true
			stringStart = i
		}
	}
	return entry
}

func jsonReindentEntry(entry string, indent string) string {
	lines := strings.Split(entry, "\n")
	minIndent := -1
	for _, line := range lines {
		if strings.TrimSpace(line) == "" {
			continue
		}
		leading := 0
		for leading < len(line) && (line[leading] == ' ' || line[leading] == '\t') {
			leading++
		}
		if minIndent == -1 || leading < minIndent {
			minIndent = leading
		}
	}
	if minIndent < 0 {
		return indent + strings.TrimSpace(entry)
	}
	for i, line := range lines {
		if strings.TrimSpace(line) == "" {
			lines[i] = ""
			continue
		}
		if minIndent > 0 && len(line) >= minIndent {
			line = line[minIndent:]
		}
		lines[i] = indent + line
	}
	return strings.Join(lines, "\n")
}

func FindSection(sections []Section, name string) *Section {
	for i := range sections {
		if sections[i].Name == name {
			return &sections[i]
		}
		if found := FindSection(sections[i].Children, name); found != nil {
			return found
		}
	}
	return nil
}

// #endregion Sections

// #region Policies

type PolicyFunc func(ctx *PolicyContext) []Violation

var policies = []PolicyDef{
	{
		ID:          "code",
		Name:        "Code",
		Description: "Validates source file headers, sections, and comments",
		Scopes:      []string{"**/*.{ts,tsx,py,cs,go}"},
		Priority:    ViolationPriorityLow,
		Kinds: []ViolationKind{
			ViolationCodeHeaderMissingRegion,
			ViolationCodeHeaderMissingFilename,
			ViolationCodeHeaderMissingContributors,
			ViolationCodeHeaderMissingLicense,
			ViolationCodeHeaderWrongLicense,
			ViolationCodeSectionEmpty,
			ViolationCodeSectionOrphanDefinition,
			ViolationCodeSectionMissingStartName,
			ViolationCodeSectionMissingEndName,
			ViolationCodeSectionNameMismatch,
			ViolationCodeCommentInline,
			ViolationCodeCommentBlock,
			ViolationCodeCommentJSDoc,
		},
		Run: codePolicy,
	},
	{
		ID:          "dev-docs",
		Name:        "DevDocs",
		Description: "Validates README.md and AGENTS.md documentation structure",
		Scopes:      []string{"README.md", "AGENTS.md"},
		Priority:    ViolationPriorityLow,
		Kinds: []ViolationKind{
			ViolationDevDocsMissingFile,
			ViolationDevDocsMissingFolder,
			ViolationDevDocsWrongFilePath,
			ViolationDevDocsWrongFolderPath,
			ViolationDevDocsWrongFileName,
			ViolationDevDocsWrongFolderName,
			ViolationDevDocsWrongFileOrder,
			ViolationDevDocsWrongFolderOrder,
			ViolationDevDocsMissingComponent,
			ViolationDevDocsWrongComponentName,
			ViolationDevDocsWrongComponentOrder,
		},
		Run: devDocsPolicy,
	},
	{
		ID:          "sketchpad",
		Name:        "Sketchpad",
		Description: "Validates sketchpad imports, state management, and hook patterns",
		Scopes:      []string{"js/sketchpad/**/*.{ts,tsx}"},
		Priority:    ViolationPriorityHigh,
		Kinds: []ViolationKind{
			ViolationSketchpadImportThirdParty,
			ViolationSketchpadStateMultipleMachines,
			ViolationSketchpadStateCreateActor,
			ViolationSketchpadStateYjsAppState,
			ViolationSketchpadStateForbiddenStore,
			ViolationSketchpadHooksNonTriadic,
		},
		Run: sketchpadPolicy,
	},
	{
		ID:          "repo",
		Name:        "Repo",
		Description: "Validates strict repo command implementation parity and ticket tracking",
		Scopes:      []string{"go/repo/main.go", "js/vscode/package.json", "graphql/repo/schema.graphql"},
		Priority:    ViolationPriorityHigh,
		Kinds: []ViolationKind{
			ViolationRepoMissingCommand,
			ViolationRepoMissingTicketTracking,
		},
		Run: repoPolicy,
	},
}

func FindPolicy(id string) (PolicyDef, bool) {
	for _, p := range policies {
		if p.ID == id {
			return p, true
		}
	}
	return PolicyDef{}, false
}

func GetPolicies() []PolicyDef {
	return policies
}

func StreamPolicies(ctx context.Context, out chan<- PolicyDef, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	for _, p := range policies {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			if !matchesFilter(p.ID, options) && !matchesFilter(p.Name, options) {
				continue
			}
			if len(options.IncludePolicies) > 0 {
				found := false
				for _, id := range options.IncludePolicies {
					if p.ID == id || strings.HasPrefix(p.ID, id+":") {
						found = true
						break
					}
				}
				if !found {
					continue
				}
			}
			if len(options.ExcludePolicies) > 0 {
				excluded := false
				for _, id := range options.ExcludePolicies {
					if p.ID == id || strings.HasPrefix(p.ID, id+":") {
						excluded = true
						break
					}
				}
				if excluded {
					continue
				}
			}
			out <- p
		}
	}
	return nil
}

type PolicyContext struct {
	Scope         Scope
	RootDir       string
	Bundles       []Bundle
	fileCache     map[string]string
	sectionCache  map[string][]Section
	ignoreCache   map[string]map[int][]string // file -> line -> ignore patterns
	filesOverride []string
}

func NewPolicyContext(scope Scope, bundles []Bundle) *PolicyContext {
	return &PolicyContext{
		Scope:        scope,
		RootDir:      rootDir,
		Bundles:      bundles,
		fileCache:    make(map[string]string),
		sectionCache: make(map[string][]Section),
		ignoreCache:  make(map[string]map[int][]string),
	}
}

func NewPolicyContextWithFiles(scope Scope, bundles []Bundle, files []string) *PolicyContext {
	ctx := NewPolicyContext(scope, bundles)
	ctx.filesOverride = files
	return ctx
}

func (ctx *PolicyContext) Files() ([]string, error) {
	if ctx.filesOverride != nil {
		return ctx.filesOverride, nil
	}
	return ScopeToFiles(ctx.Scope, ctx.Bundles)
}

func (ctx *PolicyContext) ReadText(filePath string) string {
	absPath := filepath.Join(rootDir, filePath)
	if content, ok := ctx.fileCache[absPath]; ok {
		return content
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		ctx.fileCache[absPath] = ""
		return ""
	}
	ctx.fileCache[absPath] = content
	return content
}

func (ctx *PolicyContext) Sections(filePath string) []Section {
	if sections, ok := ctx.sectionCache[filePath]; ok {
		return sections
	}
	content := ctx.ReadText(filePath)
	sections := ParseSections(content, filePath)
	ctx.sectionCache[filePath] = sections
	return sections
}

// ParseIgnoreDirectives parses // semio-ignore-* comments from file content
// Returns a map of line number -> list of ignore patterns
func ParseIgnoreDirectives(content string) map[int][]string {
	result := make(map[int][]string)
	lines := strings.Split(content, "\n")
	ignorePrefix := "// semio-ignore-"
	for i, line := range lines {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, ignorePrefix) {
			pattern := strings.TrimPrefix(trimmed, ignorePrefix)
			// Handle multiple patterns on same line separated by comma
			patterns := strings.Split(pattern, ",")
			for _, p := range patterns {
				p = strings.TrimSpace(p)
				if p != "" {
					result[i+1] = append(result[i+1], p)
				}
			}
		}
	}
	return result
}

// IgnoreDirectives returns parsed ignore directives for a file (cached)
func (ctx *PolicyContext) IgnoreDirectives(filePath string) map[int][]string {
	if ignores, ok := ctx.ignoreCache[filePath]; ok {
		return ignores
	}
	content := ctx.ReadText(filePath)
	ignores := ParseIgnoreDirectives(content)
	ctx.ignoreCache[filePath] = ignores
	return ignores
}

// IsIgnored checks if a violation should be ignored based on semio-ignore directives
// The ignore directive on line N affects violations on lines N+1 through the end of
// the next definition (approximated as next 100 lines or until next ignore directive)
func (ctx *PolicyContext) IsIgnored(filePath string, violationLine int, kind ViolationKind) bool {
	ignores := ctx.IgnoreDirectives(filePath)
	kindStr := string(kind)
	// Check if any ignore directive applies to this line
	for ignoreLine, patterns := range ignores {
		// Ignore directive applies to lines after it (up to ~100 lines or next directive)
		if violationLine > ignoreLine && violationLine <= ignoreLine+100 {
			for _, pattern := range patterns {
				// Pattern matches if the kind starts with the pattern
				if strings.HasPrefix(kindStr, pattern) {
					return true
				}
			}
		}
	}
	return false
}

func (ctx *PolicyContext) CreateViolation(summary string, kind ViolationKind, scope string, line int, excerpt string) Violation {
	return Violation{
		ID:      buildViolationID(scope, line, 0),
		Summary: summary,
		Kind:    kind,
		Scope:   scope,
		Line:    line,
		Excerpt: excerpt,
	}
}

// extractFileFromScope extracts the file path from a scope string
// Scope formats: "file.ts", "file.ts#Section", "file.ts::definition"
func extractFileFromScope(scope string) string {
	// Remove section suffix (after #)
	if idx := strings.Index(scope, "#"); idx != -1 {
		scope = scope[:idx]
	}
	// Remove definition suffix (after ::)
	if idx := strings.Index(scope, "::"); idx != -1 {
		scope = scope[:idx]
	}
	return scope
}

// FilterIgnored removes violations that are ignored via semio-ignore directives
func (ctx *PolicyContext) FilterIgnored(violations []Violation) []Violation {
	var result []Violation
	for _, v := range violations {
		file := extractFileFromScope(v.Scope)
		if !ctx.IsIgnored(file, v.Line, v.Kind) {
			result = append(result, v)
		}
	}
	return result
}

func randomString(n int) string {
	const letters = "abcdefghijklmnopqrstuvwxyz0123456789"
	b := make([]byte, n)
	for i := range b {
		b[i] = letters[rand.Intn(len(letters))]
	}
	return string(b)
}

func CheckPolicies(scope Scope, bundles []Bundle, policyIDs []string) ([]Violation, error) {
	ctx := NewPolicyContext(scope, bundles)
	return CheckPoliciesWithContext(ctx, policyIDs)
}

func CheckPoliciesWithContext(ctx *PolicyContext, policyIDs []string) ([]Violation, error) {
	var violations []Violation
	var policiesToRun []PolicyDef
	if len(policyIDs) > 0 {
		for _, p := range policies {
			for _, id := range policyIDs {
				if p.ID == id {
					policiesToRun = append(policiesToRun, p)
					break
				}
			}
		}
	} else {
		for _, p := range policies {
			if matchesScope(p.Scopes, ctx.Scope) {
				policiesToRun = append(policiesToRun, p)
			}
		}
	}
	for _, policy := range policiesToRun {
		policyViolations := policy.Run(ctx)
		violations = append(violations, policyViolations...)
	}
	return violations, nil
}

func matchesScope(policyScopes []string, targetScope Scope) bool {
	for _, pattern := range policyScopes {
		if pattern == "*" || pattern == "**/*" {
			return true
		}
		if strings.HasPrefix(pattern, "@semio") {
			if targetScope.Kind == ScopeRepo || (targetScope.Kind == ScopeProject && strings.HasPrefix(targetScope.ProjectName, pattern)) {
				return true
			}
		}
		if targetScope.Kind == ScopeRepo && strings.HasPrefix(pattern, "**/*.") {
			return true
		}
		if targetScope.FilePath != "" {
			normalizedTarget := NormalizePath(targetScope.FilePath)
			normalizedPattern := NormalizePath(pattern)
			if matched, _ := doublestar.Match(normalizedPattern, normalizedTarget); matched {
				return true
			}
		}
	}
	return false
}

func headerPolicy(ctx *PolicyContext) []Violation {
	var violations []Violation
	files, err := ctx.Files()
	if err != nil {
		return violations
	}
	agplMarkers := []string{"GNU Affero General Public License", "AGPL", "https://www.gnu.org/licenses/"}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		language := GetLanguage(file)
		if language == nil || !language.SupportsHeaders() {
			continue
		}
		sections := ctx.Sections(file)
		var headerSection *Section
		for i := range sections {
			if strings.ToLower(sections[i].Name) == "header" {
				headerSection = &sections[i]
				break
			}
		}
		if headerSection == nil {
			headerContent := generateFileHeader(file, language)
			if headerContent != "" {
				violations = append(violations, ctx.CreateViolation(
					fmt.Sprintf("Missing header section in %s", file),
					ViolationCodeHeaderMissingRegion,
					file, 0, ""))
			} else {
				violations = append(violations, ctx.CreateViolation(
					fmt.Sprintf("Missing header section in %s", file),
					ViolationCodeHeaderMissingRegion,
					file, 0, ""))
			}
			continue
		}
		headerContent := content[headerSection.StartIndex:headerSection.EndIndex]
		headerLines := strings.Split(headerContent, "\n")
		filename := filepath.Base(file)
		hasFilename := false
		for _, line := range headerLines {
			if strings.Contains(line, filename) {
				hasFilename = true
				break
			}
		}
		if !hasFilename {
			violations = append(violations, ctx.CreateViolation(
				fmt.Sprintf("Missing filename in header of %s", file),
				ViolationCodeHeaderMissingFilename,
				fmt.Sprintf("%s#Header", file), headerSection.StartLine, ""))
		}
		contributorPattern := regexp.MustCompile(`\d{4}\s+[\w\s]+<[\w.@-]+>`)
		hasContributors := false
		for _, line := range headerLines {
			if contributorPattern.MatchString(line) {
				hasContributors = true
				break
			}
		}
		if !hasContributors {
			violations = append(violations, ctx.CreateViolation(
				fmt.Sprintf("Missing contributors in header of %s", file),
				ViolationCodeHeaderMissingContributors,
				fmt.Sprintf("%s#Header", file), headerSection.StartLine, ""))
		}
		hasLicense := false
		for _, marker := range agplMarkers {
			if strings.Contains(headerContent, marker) {
				hasLicense = true
				break
			}
		}
		if !hasLicense {
			violations = append(violations, ctx.CreateViolation(
				fmt.Sprintf("Missing license in header of %s", file),
				ViolationCodeHeaderMissingLicense,
				fmt.Sprintf("%s#Header", file), headerSection.StartLine, ""))
		} else {
			wrongLicenses := []string{"MIT", "Apache", "BSD"}
			hasWrongLicense := false
			for _, wrong := range wrongLicenses {
				if strings.Contains(headerContent, wrong) {
					hasWrongLicense = true
					break
				}
			}
			if strings.Contains(headerContent, "GPL") && !strings.Contains(headerContent, "AGPL") {
				hasWrongLicense = true
			}
			if hasWrongLicense {
				violations = append(violations, ctx.CreateViolation(
					fmt.Sprintf("Wrong license in header of %s", file),
					ViolationCodeHeaderWrongLicense,
					fmt.Sprintf("%s#Header", file), headerSection.StartLine, ""))
			}
		}
	}
	return ctx.FilterIgnored(violations)
}

func sectionPolicy(ctx *PolicyContext) []Violation {
	var violations []Violation
	files, err := ctx.Files()
	if err != nil {
		return violations
	}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		language := GetLanguage(file)
		if language == nil || !language.SupportsSections() {
			continue
		}
		lines := strings.Split(content, "\n")
		type stackItem struct {
			name string
			line int
		}
		var stack []stackItem
		for i, line := range lines {
			lineNum := i + 1
			line = strings.TrimSuffix(line, "\r")
			if matched, name := language.PolicySectionStartMatch(line); matched {
				if name == "" {
					violations = append(violations, ctx.CreateViolation(
						fmt.Sprintf("Missing section name at %s:%d", file, lineNum),
						ViolationCodeSectionMissingStartName,
						file, lineNum, strings.TrimSpace(line)))
				}
				stack = append(stack, stackItem{name: name, line: lineNum})
				continue
			}
			if matched, endName := language.PolicySectionEndMatch(line); matched {
				if len(stack) > 0 {
					open := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					if open.name != "" {
						if endName == "" {
							violations = append(violations, ctx.CreateViolation(
								fmt.Sprintf("Missing end section name at %s:%d", file, lineNum),
								ViolationCodeSectionMissingEndName,
								file, lineNum, strings.TrimSpace(line)))
						} else if endName != open.name {
							violations = append(violations, ctx.CreateViolation(
								fmt.Sprintf("Section name mismatch at %s:%d", file, lineNum),
								ViolationCodeSectionNameMismatch,
								file, lineNum, fmt.Sprintf("Start: \"%s\" at line %d, End: \"%s\"", open.name, open.line, endName)))
						}
					}
				}
			}
		}
		sections := ctx.Sections(file)
		var checkSection func(s Section)
		checkSection = func(s Section) {
			sectionContent := content[s.StartIndex:s.EndIndex]
			sectionLines := strings.Split(sectionContent, "\n")
			nonEmpty := 0
			for _, line := range sectionLines[1 : len(sectionLines)-1] {
				trimmed := strings.TrimSpace(line)
				if trimmed != "" && !strings.HasPrefix(trimmed, "//") && !strings.HasPrefix(trimmed, "#") {
					nonEmpty++
				}
			}
			if nonEmpty == 0 && len(s.Children) == 0 && s.Name != "Header" {
				violations = append(violations, ctx.CreateViolation(
					fmt.Sprintf("Empty section \"%s\" in %s", s.Name, file),
					ViolationCodeSectionEmpty,
					fmt.Sprintf("%s#%s", file, s.Name), s.StartLine, ""))
			}
			for _, child := range s.Children {
				checkSection(child)
			}
		}
		for _, s := range sections {
			checkSection(s)
		}
		covered := make([]bool, len(lines))
		var markCovered func(s Section)
		markCovered = func(s Section) {
			start := s.StartLine
			if start < 1 {
				start = 1
			}
			end := s.EndLine
			if end < start {
				end = start
			}
			if end > len(lines) {
				end = len(lines)
			}
			for lineIndex := start; lineIndex <= end; lineIndex++ {
				covered[lineIndex-1] = true
			}
			for _, child := range s.Children {
				markCovered(child)
			}
		}
		for _, s := range sections {
			markCovered(s)
		}
		type lineRange struct {
			start int
			end   int
		}
		type defRange struct {
			name  string
			start int
			end   int
		}
		type orphanRangeInfo struct {
			start          int
			end            int
			firstLine      string
			isCommentBlock bool
		}
		orphanLines := make([]bool, len(lines))
		for i, line := range lines {
			if covered[i] {
				continue
			}
			line = strings.TrimSuffix(line, "\r")
			if strings.TrimSpace(line) == "" {
				continue
			}
			if startMatched, _ := language.PolicySectionStartMatch(line); startMatched {
				continue
			}
			if endMatched, _ := language.PolicySectionEndMatch(line); endMatched {
				continue
			}
			orphanLines[i] = true
		}
		var orphanRanges []lineRange
		inOrphan := false
		startLine := 0
		for i := 0; i < len(orphanLines); i++ {
			if orphanLines[i] {
				if !inOrphan {
					inOrphan = true
					startLine = i + 1
				}
			} else if inOrphan {
				orphanRanges = append(orphanRanges, lineRange{start: startLine, end: i})
				inOrphan = false
			}
		}
		if inOrphan {
			orphanRanges = append(orphanRanges, lineRange{start: startLine, end: len(lines)})
		}
		commentPrefix := language.CommentPrefix()
		var defRanges []defRange
		defExcerpts := make(map[string]string)
		if language.SupportsDefinitions() {
			parsedDefs := language.ParseDefinitions(content, lines)
			for _, def := range parsedDefs {
				defRanges = append(defRanges, defRange{name: def.Name, start: def.Start, end: def.End})
				defExcerpts[def.Name] = def.Excerpt
			}
		}
		extraDefs := language.ExtraOrphanDefinitions(lines)
		for _, def := range extraDefs {
			defRanges = append(defRanges, defRange{name: def.Name, start: def.Start, end: def.End})
			defExcerpts[def.Name] = def.Excerpt
		}
		var orphanInfos []orphanRangeInfo
		for _, orphanRange := range orphanRanges {
			firstLine := ""
			isCommentBlock := true
			for lineIndex := orphanRange.start; lineIndex <= orphanRange.end; lineIndex++ {
				line := strings.TrimSuffix(lines[lineIndex-1], "\r")
				if strings.TrimSpace(line) == "" {
					continue
				}
				if firstLine == "" {
					firstLine = strings.TrimSpace(line)
				}
				if !strings.HasPrefix(strings.TrimSpace(line), commentPrefix) {
					isCommentBlock = false
				}
			}
			orphanInfos = append(orphanInfos, orphanRangeInfo{
				start:          orphanRange.start,
				end:            orphanRange.end,
				firstLine:      firstLine,
				isCommentBlock: isCommentBlock,
			})
			if isCommentBlock {
				name := fmt.Sprintf("comment-block-%d", orphanRange.start)
				defRanges = append(defRanges, defRange{name: name, start: orphanRange.start, end: orphanRange.end})
				defExcerpts[name] = firstLine
			}
		}
		reportedDefs := make(map[string]bool)
		for _, orphanRange := range orphanInfos {
			matched := false
			for _, defRange := range defRanges {
				if orphanRange.start <= defRange.end && orphanRange.end >= defRange.start {
					if !reportedDefs[defRange.name] {
						reportedDefs[defRange.name] = true
						excerpt := defRange.name
						if value, ok := defExcerpts[defRange.name]; ok && value != "" {
							excerpt = value
						}
						violations = append(violations, ctx.CreateViolation(
							fmt.Sprintf("Orphan definition outside sections at %s:%d", file, defRange.start),
							ViolationCodeSectionOrphanDefinition,
							fmt.Sprintf("%s::%s", file, defRange.name),
							defRange.start,
							excerpt))
					}
					matched = true
				}
			}
			if matched {
				continue
			}
			name := fmt.Sprintf("orphan-block-%d", orphanRange.start)
			violations = append(violations, ctx.CreateViolation(
				fmt.Sprintf("Orphan definition outside sections at %s:%d", file, orphanRange.start),
				ViolationCodeSectionOrphanDefinition,
				fmt.Sprintf("%s::%s", file, name),
				orphanRange.start,
				orphanRange.firstLine))
		}
	}
	return ctx.FilterIgnored(violations)
}

type CommentTemplateState struct {
	ExprDepth int
}

type CommentScanState struct {
	InBlockComment         bool
	BlockCommentStartLine  int
	BlockCommentStartIndex int
	BlockCommentIsJsDoc    bool
	InSingleQuote          bool
	InDoubleQuote          bool
	Templates              []CommentTemplateState
	Escaped                bool
}

func (state *CommentScanState) InTemplateRaw() bool {
	if len(state.Templates) == 0 {
		return false
	}
	return state.Templates[len(state.Templates)-1].ExprDepth == 0
}

func commentPolicy(ctx *PolicyContext) []Violation {
	var violations []Violation
	files, err := ctx.Files()
	if err != nil {
		return violations
	}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		language := GetLanguage(file)
		if language == nil || !language.SupportsComments() {
			continue
		}
		lines := strings.Split(content, "\n")
		langViolations := language.ScanComments(ctx, file, content, lines)
		violations = append(violations, langViolations...)
	}
	return ctx.FilterIgnored(violations)
}

func truncate(s string, maxLen int) string {
	if len(s) <= maxLen {
		return s
	}
	return s[:maxLen]
}

func codePolicy(ctx *PolicyContext) []Violation {
	var violations []Violation
	violations = append(violations, headerPolicy(ctx)...)
	violations = append(violations, sectionPolicy(ctx)...)
	violations = append(violations, commentPolicy(ctx)...)
	return violations
}

func devDocsPolicy(ctx *PolicyContext) []Violation {
	var violations []Violation
	agentsContent := ctx.ReadText("AGENTS.md")
	if agentsContent == "" {
		return violations
	}
	codebaseStart := strings.Index(agentsContent, "\n# Codebase")
	if codebaseStart == -1 {
		return violations
	}
	codebaseContent := agentsContent[codebaseStart:]
	nextH1 := strings.Index(codebaseContent[1:], "\n# ")
	if nextH1 != -1 {
		codebaseContent = codebaseContent[:nextH1+1]
	}
	fileSectionRegex := regexp.MustCompile(`(?m)^## 📄\s*(.+?)\s*$`)
	folderSectionRegex := regexp.MustCompile(`(?m)^## 📁\s*(.+?)\s*$`)
	fileMatches := fileSectionRegex.FindAllStringSubmatchIndex(codebaseContent, -1)
	folderMatches := folderSectionRegex.FindAllStringSubmatchIndex(codebaseContent, -1)
	var fileSections []struct {
		path string
		line int
		pos  int
	}
	var folderSections []struct {
		path string
		line int
		pos  int
	}
	for _, match := range fileMatches {
		path := codebaseContent[match[2]:match[3]]
		lineNum := strings.Count(agentsContent[:codebaseStart+match[0]], "\n") + 1
		fileSections = append(fileSections, struct {
			path string
			line int
			pos  int
		}{path: path, line: lineNum, pos: match[0]})
	}
	for _, match := range folderMatches {
		path := codebaseContent[match[2]:match[3]]
		lineNum := strings.Count(agentsContent[:codebaseStart+match[0]], "\n") + 1
		folderSections = append(folderSections, struct {
			path string
			line int
			pos  int
		}{path: path, line: lineNum, pos: match[0]})
	}
	for i := 0; i < len(fileSections)-1; i++ {
		if fileSections[i].path > fileSections[i+1].path {
			violations = append(violations, ctx.CreateViolation(
				fmt.Sprintf("File section '%s' should come after '%s' (alphabetical order)", fileSections[i].path, fileSections[i+1].path),
				ViolationDevDocsWrongFileOrder,
				"AGENTS.md", fileSections[i+1].line, ""))
		}
	}
	for i := 0; i < len(folderSections)-1; i++ {
		if folderSections[i].path > folderSections[i+1].path {
			violations = append(violations, ctx.CreateViolation(
				fmt.Sprintf("Folder section '%s' should come after '%s' (alphabetical order)", folderSections[i].path, folderSections[i+1].path),
				ViolationDevDocsWrongFolderOrder,
				"AGENTS.md", folderSections[i+1].line, ""))
		}
	}
	return ctx.FilterIgnored(violations)
}

func sketchpadPolicy(ctx *PolicyContext) []Violation {
	var violations []Violation
	files, err := ctx.Files()
	if err != nil {
		return violations
	}
	elementsFile := ""
	for _, file := range files {
		if strings.HasSuffix(file, "elements.tsx") {
			elementsFile = file
			break
		}
	}
	thirdPartyPackages := []string{
		"react", "xstate", "yjs", "@radix-ui", "@dnd-kit", "zustand", "immer",
		"framer-motion", "lucide-react", "clsx", "tailwind", "three", "@react-three",
	}
	createMachineCount := 0
	for _, file := range files {
		if !strings.HasSuffix(file, ".ts") && !strings.HasSuffix(file, ".tsx") {
			continue
		}
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		lines := strings.Split(content, "\n")
		isElementsFile := file == elementsFile
		sections := ctx.Sections(file)
		isStateManagementSection := func(lineNum int) bool {
			for _, section := range sections {
				if strings.Contains(strings.ToLower(section.Name), "state management") ||
					strings.Contains(strings.ToLower(section.Name), "state-management") {
					if lineNum >= section.StartLine && lineNum <= section.EndLine {
						return true
					}
				}
			}
			return false
		}
		for lineNum, line := range lines {
			lineNumber := lineNum + 1
			if !isElementsFile && strings.Contains(line, "import ") {
				for _, pkg := range thirdPartyPackages {
					importPattern := fmt.Sprintf(`from\s+['"]%s`, regexp.QuoteMeta(pkg))
					if matched, _ := regexp.MatchString(importPattern, line); matched {
						violations = append(violations, ctx.CreateViolation(
							fmt.Sprintf("Third party import '%s' must only be in elements.tsx", pkg),
							ViolationSketchpadImportThirdParty,
							file, lineNumber, strings.TrimSpace(line)))
						break
					}
				}
			}
			if strings.Contains(line, "createMachine(") || strings.Contains(line, "createMachine<") {
				createMachineCount++
				if createMachineCount > 1 {
					violations = append(violations, ctx.CreateViolation(
						"createMachine can only be used once in sketchpad",
						ViolationSketchpadStateMultipleMachines,
						file, lineNumber, strings.TrimSpace(line)))
				}
			}
			if strings.Contains(line, "createActor(") || strings.Contains(line, "createActor<") {
				violations = append(violations, ctx.CreateViolation(
					"createActor is forbidden in sketchpad",
					ViolationSketchpadStateCreateActor,
					file, lineNumber, strings.TrimSpace(line)))
			}
			yjsAppStatePatterns := []string{"Y.Doc(", "new Doc(", "Y.Map(", "Y.Array(", "Y.Text("}
			for _, pattern := range yjsAppStatePatterns {
				if strings.Contains(line, pattern) && !isStateManagementSection(lineNumber) {
					if !strings.Contains(strings.ToLower(file), "kit") &&
						!strings.Contains(strings.ToLower(file), "sync") {
						violations = append(violations, ctx.CreateViolation(
							"Yjs should only be used for kit data synchronization, not app state",
							ViolationSketchpadStateYjsAppState,
							file, lineNumber, strings.TrimSpace(line)))
					}
				}
			}
			storePatterns := []string{"create(", "createStore(", "useStore("}
			for _, pattern := range storePatterns {
				if strings.Contains(line, pattern) && !isStateManagementSection(lineNumber) {
					if strings.Contains(line, "zustand") || strings.Contains(line, "store") {
						violations = append(violations, ctx.CreateViolation(
							"Stores outside of State Management sections are forbidden",
							ViolationSketchpadStateForbiddenStore,
							file, lineNumber, strings.TrimSpace(line)))
					}
				}
			}
		}
	}
	return ctx.FilterIgnored(violations)
}

func repoPolicy(ctx *PolicyContext) []Violation {
	var violations []Violation

	canonicalCommands := []string{
		"ticket_open", "ticket_list", "ticket_read", "ticket_close", "ticket_reopen", "ticket_progress",
		"goal_list", "goal_open", "goal_close", "goal_reopen",
		"export",
		"contributor_add", "contributor_remove", "contributor_list",
		"project_list", "project_tree",
		"folder_create", "folder_move", "folder_delete", "folder_list", "folder_tree",
		"file_create", "file_move", "file_delete", "file_list", "file_tree",
		"section_create", "section_move", "section_delete", "section_list", "section_tree", "integrate",
		"definition_list",
		"analyze", "fix", "policy_list", "policy_check",
		"graphql",
	}

	mainGoPath := "go/repo/main.go"
	mainContent := ctx.ReadText(mainGoPath)
	if mainContent != "" {
		for _, cmd := range canonicalCommands {
			// Check MCP registration
			mcpPattern := fmt.Sprintf("mcp.NewTool(\"%s\"", cmd)
			if !strings.Contains(mainContent, mcpPattern) {
				violations = append(violations, ctx.CreateViolation(
					fmt.Sprintf("Missing MCP registration for %s in go/repo/main.go", cmd),
					ViolationRepoMissingCommand,
					mainGoPath, 1, cmd))
			}
		}

		trackingTokens := []string{
			"ToolTicketOpen",
			"ToolTicketClose",
			"ToolTicketProgress",
		}

		for _, token := range trackingTokens {
			if !strings.Contains(mainContent, token) {
				violations = append(violations, ctx.CreateViolation(
					fmt.Sprintf("Missing ticket tracking function %s in go/repo/main.go", token),
					ViolationRepoMissingTicketTracking,
					mainGoPath, 1, token))
			}
		}
	} else {
		violations = append(violations, ctx.CreateViolation(
			"Could not read go/repo/main.go for parity check",
			ViolationRepoMissingCommand,
			mainGoPath, 1, ""))
	}

	return ctx.FilterIgnored(violations)
}

// #endregion Policies

// #region Codebase

type CodebaseContext struct {
	RootDir    string
	RootURI    string
	Bundles    []Bundle
	Files      []string
	Violations []Violation
	Tickets    []Ticket
	Policies   []PolicyDef
}

func NewCodebaseContext() *CodebaseContext {
	rootURI := "file://" + NormalizePath(rootDir)
	return &CodebaseContext{
		RootDir: rootDir,
		RootURI: rootURI,
	}
}

func (ctx *CodebaseContext) LoadBundles() {
	ctx.Bundles = GetProjects()
}

func (ctx *CodebaseContext) LoadFiles() error {
	files, err := ScopeToFiles(Scope{Kind: ScopeRepo}, ctx.Bundles)
	if err != nil {
		return err
	}
	ctx.Files = files
	return nil
}

func (ctx *CodebaseContext) LoadViolations() error {
	for _, file := range ctx.Files {
		violations, err := AnalyzeFile(file, ctx.Bundles)
		if err != nil {
			continue
		}
		ctx.Violations = append(ctx.Violations, violations...)
	}
	return nil
}

func (ctx *CodebaseContext) LoadTickets() error {
	tickets, err := ListTickets(nil, nil, nil)
	if err != nil {
		return err
	}
	ctx.Tickets = tickets
	return nil
}

func (ctx *CodebaseContext) LoadPolicies() {
	ctx.Policies = GetPolicies()
}

func (ctx *CodebaseContext) GetBundleForFile(filePath string) string {
	name, _, ok := ctx.GetBundleInfo(filePath)
	if !ok {
		return "@semio-repo/repo"
	}
	return name
}

func (ctx *CodebaseContext) GetBundleInfo(path string) (name, root string, ok bool) {
	normalizedPath := NormalizePath(path)
	var matchedBundle string
	var matchedRoot string
	var matchedLen int
	for _, bundle := range ctx.Bundles {
		root := NormalizePath(bundle.Root)
		if strings.HasPrefix(normalizedPath, root+"/") || normalizedPath == root {
			if len(root) > matchedLen {
				matchedBundle = bundle.Name
				matchedRoot = root
				matchedLen = len(root)
			}
		}
	}
	if matchedLen > 0 || matchedBundle != "" {
		return normalizeBundleLabel(matchedBundle), matchedRoot, true
	}
	return "", "", false
}

func (ctx *CodebaseContext) GetFileID(file string) string {
	return "📄" + NormalizePath(file)
}

func (ctx *CodebaseContext) GetFolderID(folder string) string {
	return "📂" + NormalizePath(folder)
}

func (ctx *CodebaseContext) FileURI(path string) string {
	return "semiorepo://file/" + NormalizePath(path)
}

func (ctx *CodebaseContext) FolderURI(path string) string {
	return "semiorepo://folder/" + NormalizePath(path)
}

func BuildCodebaseBundles(ctx *CodebaseContext) []CodebaseBundle {
	var result []CodebaseBundle
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)
	sectionCounts := make(map[string]int)
	definitionCounts := make(map[string]int)
	folderSets := make(map[string]map[string]struct{})
	contributorSets := make(map[string]map[string]struct{})
	ticketSets := make(map[string]map[string]struct{})
	violationCounts := make(map[string]int)

	for _, bundle := range ctx.Bundles {
		name := normalizeBundleLabel(bundle.Name)
		folderSets[name] = make(map[string]struct{})
		contributorSets[name] = make(map[string]struct{})
		ticketSets[name] = make(map[string]struct{})
	}
	// Always ensure fallback bundle exists
	if _, ok := folderSets["@semio-repo/repo"]; !ok {
		folderSets["@semio-repo/repo"] = make(map[string]struct{})
		contributorSets["@semio-repo/repo"] = make(map[string]struct{})
		ticketSets["@semio-repo/repo"] = make(map[string]struct{})
	}

	for _, file := range ctx.Files {
		if file == "README.md" || file == "AGENTS.md" {
			continue
		}
		bundleName := ctx.GetBundleForFile(file)
		if bundleName == "" {
			continue
		}
		fileCounts[bundleName]++
		folder := NormalizePath(filepath.Dir(file))
		if folder != "." {
			folderSets[bundleName][folder] = struct{}{}
		}
		absPath := filepath.Join(rootDir, file)
		if content, err := ReadTextFile(absPath); err == nil {
			lineCounts[bundleName] += strings.Count(content, "\n") + 1
			sections := ParseSections(content, file)
			sectionCounts[bundleName] += countSections(sections)
			lang := GetLanguage(file)
			if lang != nil && lang.SupportsDefinitions() {
				lines := strings.Split(content, "\n")
				defs := lang.ParseDefinitions(content, lines)
				definitionCounts[bundleName] += len(defs)
			}
			headerSection := FindSection(sections, "Header")
			if headerSection != nil {
				headerContent := content[headerSection.StartIndex:headerSection.EndIndex]
				for _, line := range strings.Split(headerContent, "\n") {
					if name, email, ok := ParseContributorIdentity(line); ok {
						_ = name
						contributorSets[bundleName][email] = struct{}{}
					}
				}
			}
		}
	}

	for _, v := range ctx.Violations {
		bundleName := ctx.GetBundleForFile(v.Scope)
		if bundleName != "" {
			violationCounts[bundleName]++
		}
	}

	for _, ticket := range ctx.Tickets {
		ticketID := ticket.GetID()
		fileDiffs := ticket.GetFiles().Files
		if fileDiffs.Added != nil || fileDiffs.Modified != nil || fileDiffs.Deleted != nil || fileDiffs.Renamed != nil {
			for _, entry := range fileDiffs.Modified {
				bundleName := ctx.GetBundleForFile(entry.Path)
				if bundleName != "" {
					if _, ok := ticketSets[bundleName]; ok {
						ticketSets[bundleName][ticketID] = struct{}{}
					}
				}
			}
			for _, entry := range fileDiffs.Added {
				bundleName := ctx.GetBundleForFile(entry.Path)
				if bundleName != "" {
					if _, ok := ticketSets[bundleName]; ok {
						ticketSets[bundleName][ticketID] = struct{}{}
					}
				}
			}
			for _, entry := range fileDiffs.Deleted {
				bundleName := ctx.GetBundleForFile(entry.Path)
				if bundleName != "" {
					if _, ok := ticketSets[bundleName]; ok {
						ticketSets[bundleName][ticketID] = struct{}{}
					}
				}
			}
			for _, entry := range fileDiffs.Renamed {
				bundleName := ctx.GetBundleForFile(entry.To)
				if bundleName != "" {
					if _, ok := ticketSets[bundleName]; ok {
						ticketSets[bundleName][ticketID] = struct{}{}
					}
				}
			}
		}
	}

	var bundleNames []string
	for name := range folderSets {
		bundleNames = append(bundleNames, name)
	}
	sort.Strings(bundleNames)

	for _, name := range bundleNames {
		var contributors []string
		for c := range contributorSets[name] {
			contributors = append(contributors, c)
		}
		sort.Strings(contributors)

		var tickets []string
		for t := range ticketSets[name] {
			tickets = append(tickets, t)
		}
		sort.Strings(tickets)

		bundleRoot := ""
		for _, b := range ctx.Bundles {
			if normalizeBundleLabel(b.Name) == name {
				bundleRoot = b.Root
				break
			}
		}

		result = append(result, CodebaseBundle{
			ID:           name,
			Folder:       bundleRoot,
			URI:          ctx.FileURI(bundleRoot),
			Contributors: contributors,
			Tickets:      tickets,
			Metrics: &BundleMetricsInternal{
				Folders:     len(folderSets[name]),
				Files:       fileCounts[name],
				Sections:    sectionCounts[name],
				Definitions: definitionCounts[name],
				Lines:       lineCounts[name],
				Violations:  violationCounts[name],
			},
		})
	}
	return result
}

func countSections(sections []Section) int {
	count := len(sections)
	for _, s := range sections {
		count += countSections(s.Children)
	}
	return count
}

func BuildCodebaseFolders(ctx *CodebaseContext) []CodebaseFolder {
	folderSet := make(map[string]struct{})
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)
	violationCounts := make(map[string]int)

	for _, file := range ctx.Files {
		folder := NormalizePath(filepath.Dir(file))
		if folder == "." {
			continue
		}
		folderSet[folder] = struct{}{}
		fileCounts[folder]++
		absPath := filepath.Join(rootDir, file)
		if content, err := ReadTextFile(absPath); err == nil {
			lineCounts[folder] += strings.Count(content, "\n") + 1
		}
	}

	for _, v := range ctx.Violations {
		filePath := extractFilePath(v.Scope)
		if filePath != "" {
			folder := NormalizePath(filepath.Dir(filePath))
			if folder != "." {
				violationCounts[folder]++
			}
		}
	}

	var result []CodebaseFolder
	for folder := range folderSet {
		id := ctx.GetFolderID(folder)
		result = append(result, CodebaseFolder{
			ID:   id,
			Path: folder,
			URI:  ctx.FileURI(folder),
			Metrics: &FolderMetricsInternal{
				Files:      fileCounts[folder],
				Lines:      lineCounts[folder],
				Violations: violationCounts[folder],
			},
		})
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

func extractFilePath(scope string) string {
	scope = strings.Split(scope, "#")[0]
	scope = strings.Split(scope, "§")[0]
	return scope
}

func BuildCodebaseFiles(ctx *CodebaseContext) []CodebaseFile {
	var result []CodebaseFile
	violationsByFile := make(map[string][]Violation)

	for _, v := range ctx.Violations {
		filePath := extractFilePath(v.Scope)
		if filePath != "" {
			violationsByFile[filePath] = append(violationsByFile[filePath], v)
		}
	}

	for _, file := range ctx.Files {
		id := ctx.GetFileID(file)

		var metrics *FileMetricsInternal
		absPath := filepath.Join(rootDir, file)
		if content, err := ReadTextFile(absPath); err == nil {
			sections := ParseSections(content, file)
			sectionCount := countSections(sections)
			lines := strings.Split(content, "\n")
			lang := GetLanguage(file)
			defCount := 0
			if lang != nil && lang.SupportsDefinitions() {
				defs := lang.ParseDefinitions(content, lines)
				defCount = len(defs)
			}
			metrics = &FileMetricsInternal{
				Sections:    sectionCount,
				Definitions: defCount,
				Lines:       len(lines),
			}
		}

		var violations []FileViolationRef
		for _, v := range violationsByFile[file] {
			info := v.Kind.Info()
			violations = append(violations, FileViolationRef{
				Kind:        v.Kind,
				Priority:    info.Priority,
				Autofixable: info.Autofixable,
				Solution:    info.Solution,
			})
		}

		result = append(result, CodebaseFile{
			ID:         id,
			Path:       file,
			URI:        ctx.FileURI(file),
			Metrics:    metrics,
			Violations: violations,
		})
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

func BuildCodebaseSections(ctx *CodebaseContext) []CodebaseSection {
	var result []CodebaseSection

	for _, file := range ctx.Files {
		absPath := filepath.Join(rootDir, file)
		content, err := ReadTextFile(absPath)
		if err != nil {
			continue
		}
		sections := ParseSections(content, file)

		fileID := ctx.GetFileID(file)

		addSections(ctx, &result, file, fileID, content, sections, "")
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

func addSections(ctx *CodebaseContext, result *[]CodebaseSection, file, fileID, content string, sections []Section, parentPath string) {
	for _, section := range sections {
		sectionPath := section.Name
		if parentPath != "" {
			sectionPath = parentPath + "#" + section.Name
		}
		id := fileID + "#" + sectionPath
		sectionContent := ""
		if section.StartIndex < len(content) && section.EndIndex <= len(content) {
			sectionContent = content[section.StartIndex:section.EndIndex]
		}
		defCount := 0
		lang := GetLanguage(file)
		if lang != nil && lang.SupportsDefinitions() {
			lines := strings.Split(sectionContent, "\n")
			defs := lang.ParseDefinitions(sectionContent, lines)
			defCount = len(defs)
		}
		*result = append(*result, CodebaseSection{
			ID:   id,
			Path: file + "#" + sectionPath,
			URI:  ctx.FileURI(file) + "#" + sectionPath,
			Metrics: &SectionMetricsInternal{
				Definitions: defCount,
				Lines:       section.EndLine - section.StartLine + 1,
				Violations:  0,
			},
		})
		addSections(ctx, result, file, fileID, content, section.Children, sectionPath)
	}
}

func BuildCodebaseDefinitions(ctx *CodebaseContext) []CodebaseDefinition {
	var result []CodebaseDefinition

	for _, file := range ctx.Files {
		absPath := filepath.Join(rootDir, file)
		content, err := ReadTextFile(absPath)
		if err != nil {
			continue
		}
		lang := GetLanguage(file)
		if lang == nil || !lang.SupportsDefinitions() {
			continue
		}
		lines := strings.Split(content, "\n")
		defs := lang.ParseDefinitions(content, lines)
		sections := ParseSections(content, file)
		fileID := ctx.GetFileID(file)

		for _, def := range defs {
			sectionPath := findSectionForDefinition(sections, def.Start, def.End, "")
			id := ""
			if sectionPath != "" {
				id = fileID + "#" + sectionPath + "§" + def.Name
			} else {
				id = fileID + "§" + def.Name
			}
			result = append(result, CodebaseDefinition{
				ID:   id,
				Path: id, // Use ID as path too for codebase metrics
				URI:  ctx.FileURI(file) + "§" + def.Name,
				Metrics: &DefinitionMetricsInternal{
					Definitions: 0,
					Lines:       def.End - def.Start + 1,
					Violations:  0,
				},
			})
		}
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

func BuildCodebaseContributors(ctx *CodebaseContext) []CodebaseContributor {
	contributors, err := ListContributors()
	if err != nil {
		return nil
	}

	var result []CodebaseContributor
	for _, c := range contributors {
		avatarPath := GetContributorAvatarPath(c.Github)
		avatarRoundPath := GetContributorAvatarRoundPath(c.Github)

		var icons *ContributorIcons
		if FileExists(avatarPath) || FileExists(avatarRoundPath) {
			icons = &ContributorIcons{}
			if FileExists(avatarPath) {
				avatar := ctx.FileURI(GetRelativePath(avatarPath))
				icons.Avatar = &avatar
			}
			if FileExists(avatarRoundPath) {
				avatarRound := ctx.FileURI(GetRelativePath(avatarRoundPath))
				icons.AvatarRound = &avatarRound
			}
			if githubLink, ok := c.Links["github"]; ok {
				github := githubLink + ".png"
				icons.Github = &github
			}
		}

		var contributions *ContributorContributionsInternal
		if len(c.Contributions.Bundles) > 0 || len(c.Contributions.Files) > 0 {
			contributions = &ContributorContributionsInternal{}
			for _, b := range c.Contributions.Bundles {
				contributions.Bundles = append(contributions.Bundles, ContributorBundleContrib{ID: b})
			}
			for _, f := range c.Contributions.Folders {
				contributions.Folders = append(contributions.Folders, ContributorFolderContrib{ID: f})
			}
			for _, f := range c.Contributions.Files {
				contributions.Files = append(contributions.Files, ContributorFileContrib{ID: f})
			}
			for _, r := range c.Contributions.Regions {
				contributions.Sections = append(contributions.Sections, ContributorSectionContrib{ID: r})
			}
			for _, d := range c.Contributions.Definitions {
				contributions.Definitions = append(contributions.Definitions, ContributorDefinitionContrib{ID: d})
			}
		}

		linesTotal := 0
		if c.Contributions.Lines != nil {
			linesTotal = c.Contributions.Lines.Added + c.Contributions.Lines.Removed
		}

		result = append(result, CodebaseContributor{
			ID:            c.Github,
			URI:           ctx.FileURI(".semio-repo/contributors/" + c.Github),
			Path:          ".semio-repo/contributors/" + c.Github + "/contributor.json",
			Name:          c.Name,
			Icons:         icons,
			Emails:        c.Emails,
			Links:         c.Links,
			Contributions: contributions,
			Metrics: &ContributorMetricsInternal{
				Commits:     len(c.Contributions.Commits),
				Tickets:     len(c.Contributions.Tickets),
				Bundles:     len(c.Contributions.Bundles),
				Folders:     len(c.Contributions.Folders),
				Files:       len(c.Contributions.Files),
				Lines:       linesTotal,
				Sections:    len(c.Contributions.Regions),
				Definitions: len(c.Contributions.Definitions),
			},
		})
	}
	return result
}

func BuildCodebaseTickets(ctx *CodebaseContext) []CodebaseTicket {
	var result []CodebaseTicket

	for _, ticket := range ctx.Tickets {
		ticketID := ticket.GetID()
		ticketPath := ticket.TicketPath
		if ticketPath == "" {
			ticketPath = fmt.Sprintf(".semio-repo/tickets/%04d/%02d/%02d/%s/ticket.md", ticket.Year, ticket.Month, ticket.Day, ticket.Slug)
		}

		bundleFiles := make(map[string]int)
		var paths []string
		files := ticket.GetFiles().Files
		if files.Added != nil {
			for _, f := range files.Added {
				paths = append(paths, f.Path)
			}
		}
		if files.Modified != nil {
			for _, f := range files.Modified {
				paths = append(paths, f.Path)
			}
		}
		if files.Deleted != nil {
			for _, f := range files.Deleted {
				paths = append(paths, f.Path)
			}
		}
		if files.Renamed != nil {
			for _, f := range files.Renamed {
				paths = append(paths, f.To)
			}
		}
		for _, path := range paths {
			bundleName := ctx.GetBundleForFile(path)
			if bundleName != "" {
				bundleFiles[bundleName]++
			}
		}
		var bundleContribs []TicketBundleContribInfo
		for bundleName, fileCount := range bundleFiles {
			bundleContribs = append(bundleContribs, TicketBundleContribInfo{
				ID: bundleName,
				Metrics: &CountMetrics{
					Added: fileCount,
				},
			})
		}

		llm := ticket.GetLLM()

		var finishedStr string
		if f := ticket.GetDateFinished(); f != nil {
			finishedStr = f.Format(time.RFC3339)
		}

		result = append(result, CodebaseTicket{
			ID:   ticketID,
			Path: ticketPath,
			URI:  ctx.FileURI(ticketPath),
			Date: &TicketDateInfo{
				Created:  ticket.GetDateStarted().Format(time.RFC3339),
				Finished: finishedStr,
			},
			Commit:  ticket.GetCommit(),
			Year:    fmt.Sprintf("%04d", ticket.Year),
			Month:   fmt.Sprintf("%02d", ticket.Month),
			Day:     fmt.Sprintf("%02d", ticket.Day),
			Slug:    ticket.Slug,
			Prompt:  ticket.GetPrompt(),
			LLM:     llm,
			Author:  ticket.GetAuthor(),
			Status:  ticket.GetStatus(),
			Bundles: bundleContribs,
		})
	}
	return result
}

func BuildCodebasePolicies(ctx *CodebaseContext) []CodebasePolicy {
	var result []CodebasePolicy
	violationsByPolicy := make(map[string][]Violation)

	for _, v := range ctx.Violations {
		parts := strings.Split(string(v.Kind), ":")
		if len(parts) > 0 {
			policyID := parts[0]
			violationsByPolicy[policyID] = append(violationsByPolicy[policyID], v)
		}
	}

	for _, policy := range ctx.Policies {
		var violations []PolicyViolationRef
		for _, v := range violationsByPolicy[policy.ID] {
			info := v.Kind.Info()
			violations = append(violations, PolicyViolationRef{
				Kind:        v.Kind,
				Priority:    info.Priority,
				Autofixable: info.Autofixable,
				Solution:    info.Solution,
			})
		}
		result = append(result, CodebasePolicy{
			ID:         policy.ID,
			Name:       policy.Name,
			Scopes:     policy.Scopes,
			Violations: violations,
		})
	}
	return result
}

func BuildCodebaseViolations(ctx *CodebaseContext) []CodebaseViolation {
	var result []CodebaseViolation

	for i, v := range ctx.Violations {
		filePath := extractFilePath(v.Scope)
		bundleName := ctx.GetBundleForFile(filePath)
		info := v.Kind.Info()

		violationID := fmt.Sprintf("%s#|%s|%s#%d", v.Kind, bundleName, filePath, i)

		var folders []ViolationFolder
		if filePath != "" {
			folder := NormalizePath(filepath.Dir(filePath))
			if folder != "." {
				folderID := folder
				if bundleName != "" {
					folderID = bundleName + "/" + folder
				}
				folders = append(folders, ViolationFolder{
					ID:   folderID,
					Path: folder,
					URI:  ctx.FolderURI(folder),
				})
			}
		}

		var files []ViolationFile
		if filePath != "" {
			fileID := filePath
			if bundleName != "" {
				fileID = bundleName + "/" + filepath.Base(filePath)
			}
			files = append(files, ViolationFile{
				ID:   fileID,
				Path: filePath,
				URI:  ctx.FileURI(filePath),
				Range: &FileRange{
					Start: RangePosition{Line: v.Line, Column: v.Column},
					End:   RangePosition{Line: v.Line, Column: v.Column},
				},
			})
		}

		result = append(result, CodebaseViolation{
			ID:          violationID,
			Folders:     folders,
			Files:       files,
			Kind:        v.Kind,
			Priority:    info.Priority,
			Autofixable: info.Autofixable,
			Reason:      info.Reason,
			Solution:    info.Solution,
		})
	}
	return result
}

func BuildCodebaseTree(ctx *CodebaseContext, bundles []CodebaseBundle, files []CodebaseFile, sections []CodebaseSection, definitions []CodebaseDefinition) map[string]*TreeNode {
	tree := make(map[string]*TreeNode)
	tree["@semio"] = &TreeNode{Kind: TreeNodeRepo, Children: make(map[string]*TreeNode)}
	root := tree["@semio"]

	for _, bundle := range bundles {
		root.Children[bundle.ID] = &TreeNode{Kind: TreeNodeBundle, Children: make(map[string]*TreeNode)}
	}

	folderNodes := make(map[string]*TreeNode)
	for _, file := range files {
		bundleName := ctx.GetBundleForFile(file.Path)
		var parent *TreeNode
		if bundleName != "" {
			parent = root.Children[bundleName]
		} else {
			parent = root
		}
		folder := NormalizePath(filepath.Dir(file.Path))
		if folder != "." {
			parts := strings.Split(folder, "/")
			for i, part := range parts {
				folderPath := strings.Join(parts[:i+1], "/")
				if _, ok := folderNodes[folderPath]; !ok {
					folderNode := &TreeNode{Kind: TreeNodeFolder, Children: make(map[string]*TreeNode)}
					if i == 0 {
						parent.Children[part] = folderNode
					} else {
						parentPath := strings.Join(parts[:i], "/")
						folderNodes[parentPath].Children[part] = folderNode
					}
					folderNodes[folderPath] = folderNode
				}
			}
			fileNode := &TreeNode{Kind: TreeNodeFile, Children: make(map[string]*TreeNode)}
			folderNodes[folder].Children[file.ID] = fileNode
		} else {
			fileNode := &TreeNode{Kind: TreeNodeFile, Children: make(map[string]*TreeNode)}
			parent.Children[file.ID] = fileNode
		}
	}

	return tree
}

func BuildCodebase(ctx *CodebaseContext) *Codebase {
	bundles := BuildCodebaseBundles(ctx)
	folders := BuildCodebaseFolders(ctx)
	files := BuildCodebaseFiles(ctx)
	sections := BuildCodebaseSections(ctx)
	definitions := BuildCodebaseDefinitions(ctx)
	contributors := BuildCodebaseContributors(ctx)
	tickets := BuildCodebaseTickets(ctx)
	policies := BuildCodebasePolicies(ctx)
	violations := BuildCodebaseViolations(ctx)
	tree := BuildCodebaseTree(ctx, bundles, files, sections, definitions)

	return &Codebase{
		Bundles:      bundles,
		Folders:      folders,
		Files:        files,
		Sections:     sections,
		Definitions:  definitions,
		Contributors: contributors,
		Tickets:      tickets,
		Policies:     policies,
		Violations:   violations,
		Tree:         tree,
	}
}

func BuildCodebaseSnapshot(files []string, bundles []Bundle, commit string) (*Codebase, error) {
	ctx := &CodebaseContext{RootDir: rootDir, RootURI: "file://" + NormalizePath(rootDir)}
	ctx.Bundles = bundles
	ctx.Files = files
	ctx.Policies = GetPolicies()
	codebase := &Codebase{}
	codebase.Bundles = BuildCodebaseBundlesForFiles(ctx, commit)
	codebase.Folders = BuildCodebaseFoldersForFiles(ctx, commit)
	codebase.Files = BuildCodebaseFilesForFiles(ctx, commit)
	codebase.Sections = BuildCodebaseSectionsForFiles(ctx, commit)
	codebase.Definitions = BuildCodebaseDefinitionsForFiles(ctx, commit)
	codebase.Tree = BuildCodebaseTree(ctx, codebase.Bundles, codebase.Files, codebase.Sections, codebase.Definitions)
	return codebase, nil
}

func BuildCodebaseBundlesForFiles(ctx *CodebaseContext, commit string) []CodebaseBundle {
	var result []CodebaseBundle
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)
	sectionCounts := make(map[string]int)
	definitionCounts := make(map[string]int)
	folderSets := make(map[string]map[string]struct{})

	for _, bundle := range ctx.Bundles {
		name := normalizeBundleLabel(bundle.Name)
		folderSets[name] = make(map[string]struct{})
	}
	// Always ensure fallback bundle exists
	if _, ok := folderSets["@semio-repo/repo"]; !ok {
		folderSets["@semio-repo/repo"] = make(map[string]struct{})
	}

	for _, file := range ctx.Files {
		bundleName := ctx.GetBundleForFile(file)
		if bundleName == "" {
			continue
		}
		fileCounts[bundleName]++
		folder := NormalizePath(filepath.Dir(file))
		if folder != "." {
			folderSets[bundleName][folder] = struct{}{}
		}
		content, err := ReadTextFileAtCommit(commit, file)
		if err != nil {
			continue
		}
		lineCounts[bundleName] += CountLines(content)
		sections := ParseSections(content, file)
		sectionCounts[bundleName] += countSections(sections)
		lang := GetLanguage(file)
		if lang != nil && lang.SupportsDefinitions() {
			lines := strings.Split(content, "\n")
			defs := lang.ParseDefinitions(content, lines)
			definitionCounts[bundleName] += len(defs)
		}
	}

	var bundleNames []string
	for name := range folderSets {
		bundleNames = append(bundleNames, name)
	}
	sort.Strings(bundleNames)

	for _, name := range bundleNames {
		bundleRoot := ""
		for _, b := range ctx.Bundles {
			if normalizeBundleLabel(b.Name) == name {
				bundleRoot = b.Root
				break
			}
		}

		result = append(result, CodebaseBundle{
			ID:     name,
			Folder: bundleRoot,
			URI:    ctx.FileURI(bundleRoot),
			Metrics: &BundleMetricsInternal{
				Folders:     len(folderSets[name]),
				Files:       fileCounts[name],
				Sections:    sectionCounts[name],
				Definitions: definitionCounts[name],
				Lines:       lineCounts[name],
			},
		})
	}
	return result
}

func BuildCodebaseFoldersForFiles(ctx *CodebaseContext, commit string) []CodebaseFolder {
	folderSet := make(map[string]struct{})
	fileCounts := make(map[string]int)
	lineCounts := make(map[string]int)

	for _, file := range ctx.Files {
		folder := NormalizePath(filepath.Dir(file))
		if folder == "." {
			continue
		}
		folderSet[folder] = struct{}{}
		fileCounts[folder]++
		content, err := ReadTextFileAtCommit(commit, file)
		if err == nil {
			lineCounts[folder] += CountLines(content)
		}
	}

	var result []CodebaseFolder
	for folder := range folderSet {
		id := ctx.GetFolderID(folder)
		parent := filepath.Dir(folder)
		var parentID *string
		if parent != "." && parent != "" {
			parentValue := parent
			parentID = &parentValue
		}
		result = append(result, CodebaseFolder{
			ID:       id,
			Path:     folder,
			URI:      ctx.FolderURI(folder),
			Name:     filepath.Base(folder),
			ParentID: parentID,
			Metrics: &FolderMetricsInternal{
				Files: fileCounts[folder],
				Lines: lineCounts[folder],
			},
		})
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

func BuildCodebaseFilesForFiles(ctx *CodebaseContext, commit string) []CodebaseFile {
	var result []CodebaseFile
	for _, file := range ctx.Files {
		id := ctx.GetFileID(file)
		var metrics *FileMetricsInternal
		content, err := ReadTextFileAtCommit(commit, file)
		if err == nil {
			sections := ParseSections(content, file)
			sectionCount := countSections(sections)
			lines := strings.Split(content, "\n")
			lang := GetLanguage(file)
			defCount := 0
			if lang != nil && lang.SupportsDefinitions() {
				defs := lang.ParseDefinitions(content, lines)
				defCount = len(defs)
			}
			metrics = &FileMetricsInternal{
				Sections:    sectionCount,
				Definitions: defCount,
				Lines:       len(lines),
			}
		}
		result = append(result, CodebaseFile{
			ID:      id,
			Path:    file,
			URI:     ctx.FileURI(file),
			Metrics: metrics,
		})
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

func BuildCodebaseSectionsForFiles(ctx *CodebaseContext, commit string) []CodebaseSection {
	var result []CodebaseSection
	for _, file := range ctx.Files {
		content, err := ReadTextFileAtCommit(commit, file)
		if err != nil {
			continue
		}
		sections := ParseSections(content, file)
		fileID := ctx.GetFileID(file)
		addSectionsForContent(ctx, &result, file, fileID, content, sections, "")
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

func addSectionsForContent(ctx *CodebaseContext, result *[]CodebaseSection, file, fileID, content string, sections []Section, parentPath string) {
	for _, section := range sections {
		sectionPath := section.Name
		if parentPath != "" {
			sectionPath = parentPath + "#" + section.Name
		}
		id := fileID + "#" + sectionPath
		sectionContent := ""
		if section.StartIndex < len(content) && section.EndIndex <= len(content) {
			sectionContent = content[section.StartIndex:section.EndIndex]
		}
		defCount := 0
		lang := GetLanguage(file)
		if lang != nil && lang.SupportsDefinitions() {
			lines := strings.Split(sectionContent, "\n")
			defs := lang.ParseDefinitions(sectionContent, lines)
			defCount = len(defs)
		}
		*result = append(*result, CodebaseSection{
			ID:   id,
			Path: file + "#" + sectionPath,
			URI:  ctx.FileURI(file) + "#" + sectionPath,
			Metrics: &SectionMetricsInternal{
				Definitions: defCount,
				Lines:       section.EndLine - section.StartLine + 1,
			},
		})
		addSectionsForContent(ctx, result, file, fileID, content, section.Children, sectionPath)
	}
}

func BuildCodebaseDefinitionsForFiles(ctx *CodebaseContext, commit string) []CodebaseDefinition {
	var result []CodebaseDefinition
	for _, file := range ctx.Files {
		content, err := ReadTextFileAtCommit(commit, file)
		if err != nil {
			continue
		}
		lang := GetLanguage(file)
		if lang == nil || !lang.SupportsDefinitions() {
			continue
		}
		lines := strings.Split(content, "\n")
		defs := lang.ParseDefinitions(content, lines)
		sections := ParseSections(content, file)
		fileID := ctx.GetFileID(file)
		for _, def := range defs {
			sectionPath := findSectionForDefinition(sections, def.Start, def.End, "")
			id := ""
			if sectionPath != "" {
				id = fileID + "#" + sectionPath + "§" + def.Name
			} else {
				id = fileID + "§" + def.Name
			}
			result = append(result, CodebaseDefinition{
				ID:   id,
				Path: id,
				URI:  ctx.FileURI(file) + "§" + def.Name,
				Metrics: &DefinitionMetricsInternal{
					Lines: def.End - def.Start + 1,
				},
			})
		}
	}
	sort.Slice(result, func(i, j int) bool { return result[i].Path < result[j].Path })
	return result
}

func ToolCodebase() ToolResult {
	output := NewOutput()
	ctx := NewCodebaseContext()

	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		output.Error(fmt.Sprintf("Error loading files: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := ctx.LoadViolations(); err != nil {
		output.Error(fmt.Sprintf("Error loading violations: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := ctx.LoadTickets(); err != nil {
		output.Error(fmt.Sprintf("Error loading tickets: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	ctx.LoadPolicies()

	codebase := BuildCodebase(ctx)

	output.Success(fmt.Sprintf("Codebase loaded: %d bundles, %d files, %d violations",
		len(codebase.Bundles), len(codebase.Files), len(codebase.Violations)))

	return ToolResult{Output: *output, Data: codebase}
}

// #endregion Codebase

// #region Tickets

func GetTicketsDir() string {
	return filepath.Join(GetRepoMetaDir(), "tickets")
}

func GetTicketPath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketsDir(), strconv.Itoa(year), PadNumber(month, 2), PadNumber(day, 2), slug)
}

func GetTicketFilePath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketPath(year, month, day, slug), "ticket.md")
}

func GetImportantFilePath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketPath(year, month, day, slug), "important.md")
}

func GetTicketJsonPath(year, month, day int, slug string) string {
	return filepath.Join(GetTicketPath(year, month, day, slug), "ticket.json")
}

func normalizeTicketKeyword(value string) string {
	return strings.ToUpper(strings.TrimSpace(value))
}

func hasTicketKeyword(text, keyword string) bool {
	return strings.Contains(strings.ToUpper(text), keyword)
}

func FindTicketBySlug(slug string) (*Ticket, error) {
	tickets, err := ListTickets(nil, nil, nil)
	if err != nil {
		return nil, err
	}
	for i := len(tickets) - 1; i >= 0; i-- {
		t := tickets[i]
		if t.Slug == slug || filepath.Base(t.Slug) == slug {
			return &t, nil
		}
	}
	return nil, fmt.Errorf("ticket not found: %s", slug)
}

func LatestTicket() (*Ticket, error) {
	tickets, err := ListTickets(nil, nil, nil)
	if err != nil {
		return nil, err
	}
	if len(tickets) == 0 {
		return nil, fmt.Errorf("no tickets found")
	}
	sort.Slice(tickets, func(i, j int) bool {
		if tickets[i].Year != tickets[j].Year {
			return tickets[i].Year > tickets[j].Year
		}
		if tickets[i].Month != tickets[j].Month {
			return tickets[i].Month > tickets[j].Month
		}
		if tickets[i].Day != tickets[j].Day {
			return tickets[i].Day > tickets[j].Day
		}
		return tickets[i].GetDateStarted().After(tickets[j].GetDateStarted())
	})
	return &tickets[0], nil
}

func shouldContinueTicket(prompt string) bool {
	return hasTicketKeyword(prompt, "CONTINUE")
}

func shouldSkipTicket(prompt string) bool {
	return hasTicketKeyword(prompt, "NOTICKET")
}

func OpenTicket(title, prompt, llm, ui, draft string, noIssue bool, goal string, parent string, noGithub bool, issue string) (*Ticket, error) {
	if prompt == "" {
		prompt = title
	}
	if shouldSkipTicket(prompt) {
		return nil, nil
	}
	if shouldContinueTicket(prompt) {
		latest, err := LatestTicket()
		if err != nil {
			return nil, err
		}
		if latest.GetStatus() == TicketStatusClosed {
			return latest, ReopenTicket(latest, prompt, llm, ui, draft, goal, parent, noGithub)
		}
		return latest, nil
	}
	return CreateTicket(title, prompt, llm, ui, draft, noIssue, goal, parent, noGithub, issue)
}

func OpenGoal(title, description, prompt, dueDate, ui, llm string, noGithub bool) (*Goal, error) {
	ctx := NewRepoContext(rootDir)
	input := GoalCreateInput{
		Title:       title,
		Description: description,
		Prompt:      prompt,
		DueDate:     dueDate,
		UI:          ui,
		LLM:         llm,
		NoGithub:    noGithub,
	}
	return ctx.GoalCreate(input)
}

func UpdateTicketTitle(ticket *Ticket, title string) error {
	if ticket == nil {
		return fmt.Errorf("ticket is nil")
	}
	if false {
		return fmt.Errorf("ticket data is nil")
	}
	title = strings.TrimSpace(title)
	if title == "" {
		return fmt.Errorf("ticket title is required")
	}
	slug := Slugify(title)
	if title == slug {
		return fmt.Errorf("ticket title must be titleized (e.g. \"Some Title on Something\") and NOT an all-caps slug")
	}
	if title == strings.ToLower(slug) {
		return fmt.Errorf("ticket title must be titleized (e.g. \"Some Title on Something\") and NOT a slug")
	}

	// Preserve parent path if nested
	parentDir := filepath.Dir(ticket.Slug)
	if parentDir != "." {
		slug = filepath.ToSlash(filepath.Join(parentDir, slug))
	}

	newFolderPath := GetTicketPath(ticket.Year, ticket.Month, ticket.Day, slug)
	if slug != ticket.Slug {
		if FileExists(newFolderPath) {
			return fmt.Errorf("ticket folder already exists: %s", newFolderPath)
		}
		if err := EnsureDir(filepath.Dir(newFolderPath)); err != nil {
			return err
		}
		if err := os.Rename(ticket.FolderPath, newFolderPath); err != nil {
			return err
		}
	}
	ticket.Title = title
	ticket.Slug = slug
	ticket.FolderPath = newFolderPath
	ticket.JsonPath = GetTicketJsonPath(ticket.Year, ticket.Month, ticket.Day, slug)
	ticket.TicketPath = GetTicketFilePath(ticket.Year, ticket.Month, ticket.Day, slug)
	ticket.ImportantPath = GetImportantFilePath(ticket.Year, ticket.Month, ticket.Day, slug)
	return nil
}

func CreateTicket(title, prompt, llm, ui, draft string, noIssue bool, goal string, parent string, noGithub bool, issue string) (*Ticket, error) {
	title = strings.TrimSpace(title)
	slug := Slugify(title)
	if title == slug {
		return nil, fmt.Errorf("ticket title must be titleized (e.g. \"Some Title on Something\") and NOT an all-caps slug")
	}
	if title == strings.ToLower(slug) {
		return nil, fmt.Errorf("ticket title must be titleized (e.g. \"Some Title on Something\") and NOT a slug")
	}

	now := time.Now()
	year, month, day := FormatDate(now)

	if parent != "" {
		pTicket, err := FindTicketBySlug(parent)
		if err != nil {
			return nil, fmt.Errorf("failed to find parent ticket '%s': %w", parent, err)
		}
		year = pTicket.Year
		month = pTicket.Month
		day = pTicket.Day
		slug = filepath.ToSlash(filepath.Join(pTicket.Slug, slug))
	}

	var llmSlug string
	var err error
	if llm != "" {
		llmSlug, err = ResolveAllowedLLM(llm)
		if err != nil {
			return nil, err
		}
	}
	uiSlug, err := ResolveAllowedUI(ui)
	if err != nil {
		return nil, err
	}

	ticketDir := GetTicketPath(year, month, day, slug)
	if err := EnsureDir(ticketDir); err != nil {
		return nil, err
	}
	jsonPath := GetTicketJsonPath(year, month, day, slug)

	ticketFilePath := GetTicketFilePath(year, month, day, slug)
	importantFilePath := GetImportantFilePath(year, month, day, slug)
	gitAuthor := GetGitAuthorGithub()
	gitCommit := GetGitCommit()

	if draft != "" {
		draftPath := filepath.Join(GetDraftsPath(), draft)
		if IsDir(draftPath) {
			entries, err := os.ReadDir(draftPath)
			if err == nil {
				for _, entry := range entries {
					src := filepath.Join(draftPath, entry.Name())
					dst := filepath.Join(ticketDir, entry.Name())
					if err := MoveFile(src, dst); err != nil {
						fmt.Printf("Warning: Failed to move draft file %s: %v\n", entry.Name(), err)
						continue
					}
				}
			}
			os.RemoveAll(draftPath)
		}
	}

	if err := WriteTextFile(ticketFilePath, buildTicketMarkdown(goal, parent)); err != nil {
		return nil, fmt.Errorf("failed to write ticket file: %w", err)
	}

	if err := WriteTextFile(importantFilePath, ""); err != nil {
		return nil, fmt.Errorf("failed to write important file: %w", err)
	}

	ticket := &Ticket{
		Year:          year,
		Month:         month,
		Day:           day,
		Slug:          slug,
		Title:         title,
		Status:        TicketStatusOpen,
		Prompt:        prompt,
		Goal:          goal,
		Parent:        parent,
		Started:       now,
		FolderPath:    ticketDir,
		JsonPath:      jsonPath,
		TicketPath:    ticketFilePath,
		ImportantPath: importantFilePath,
		Iterations: []TicketIteration{{
			Prompt:  prompt,
			LLM:     llmSlug,
			UI:      uiSlug,
			Author:  parseGitAuthor(gitAuthor),
			Started: now,
			Commit:  gitCommit,
		}},
	}

	skipIssue := noIssue || noGithub || strings.Contains(prompt, "NOISSUE")
	if !skipIssue {
		if issue != "" {
			// Use existing issue URL
			ticket.GitHub = &TicketGithubData{Issue: issue}
			ghAddIssueToProject(issue)
		} else {
			// Create new issue
			issueBody := formatPromptHeading(prompt)

			var milestone *int
			if goal != "" {
				goals, err := ListGoals()
				if err == nil {
					for _, g := range goals {
						if g.ID == goal {
							if g.GitHub != nil && g.GitHub.Milestone != "" {
								if n, err := parseMilestoneNumber(g.GitHub.Milestone); err == nil {
									milestone = &n
								}
							}
							break
						}
					}
				}
			}

			issueURL, err := ghCreateIssue(title, issueBody, milestone)
			if err == nil && issueURL != "" {
				ticket.GitHub = &TicketGithubData{Issue: issueURL}
			} else if err != nil {
				fmt.Printf("Warning: Failed to create GitHub issue: %v\n", err)
			}
		}
	}

	if err := SaveTicket(ticket); err != nil {
		return nil, err
	}
	return ticket, nil
}

func ghCreateIssue(title, body string, milestone *int) (string, error) {
	args := []string{"issue", "create", "--title", title, "--body", body, "--label", "ticket"}
	if milestone != nil {
		args = append(args, "--milestone", fmt.Sprintf("%d", *milestone))
	}
	stdout, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return "", fmt.Errorf("gh issue create failed: %s", strings.TrimSpace(stderr))
	}
	issueURL := strings.TrimSpace(stdout)
	if issueURL != "" {
		ghAddIssueToProject(issueURL)
		ghAssignIssueToCurrentUser(issueURL)
	}
	return issueURL, nil
}

func buildProjectLinkArgs(issueURL string) []string {
	return []string{"project", "item-add", "2", "--owner", "usalu", "--url", issueURL}
}

func ghAddIssueToProject(issueURL string) {
	if issueURL == "" {
		return
	}
	ExecCommand("gh", buildProjectLinkArgs(issueURL), "")
}

func ghGetCurrentUser() string {
	stdout, _, exitCode := ExecCommand("gh", []string{"api", "user", "--jq", ".login"}, "")
	if exitCode != 0 {
		return ""
	}
	return strings.TrimSpace(stdout)
}

func ghAssignIssueToCurrentUser(issueURL string) {
	if issueURL == "" {
		return
	}
	user := ghGetCurrentUser()
	if user == "" {
		return
	}
	ExecCommand("gh", []string{"issue", "edit", issueURL, "--add-assignee", user}, "")
}

func CountLines(content string) int {
	if content == "" {
		return 0
	}
	return strings.Count(content, "\n") + 1
}

func CountLinesInFile(path string) int {
	content, err := ReadTextFile(path)
	if err != nil {
		return 0
	}
	return CountLines(content)
}

func CountLinesAtCommit(commit, filePath string) int {
	stdout, _, exitCode := ExecCommand("git", []string{"show", fmt.Sprintf("%s:%s", commit, filePath)}, "")
	if exitCode != 0 {
		return 0
	}
	return CountLines(stdout)
}

func ReadTextFileAtCommit(commit, filePath string) (string, error) {
	if commit == "" {
		return ReadTextFile(filepath.Join(rootDir, filePath))
	}
	stdout, stderr, exitCode := ExecCommand("git", []string{"show", fmt.Sprintf("%s:%s", commit, filePath)}, "")
	if exitCode != 0 {
		return "", fmt.Errorf("git show failed: %s", strings.TrimSpace(stderr))
	}
	return stdout, nil
}

func ListFilesAtCommit(commit string) ([]string, error) {
	if commit == "" {
		files, err := ScopeToFiles(Scope{Kind: ScopeRepo}, GetProjects())
		if err != nil {
			return nil, err
		}
		return files, nil
	}
	stdout, stderr, exitCode := ExecCommand("git", []string{"ls-tree", "-r", "--name-only", commit}, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("git ls-tree failed: %s", strings.TrimSpace(stderr))
	}
	var files []string
	for _, line := range strings.Split(strings.TrimSpace(stdout), "\n") {
		if line == "" {
			continue
		}
		files = append(files, strings.TrimSpace(line))
	}
	files = filterConsideredFiles(files)
	files = filterGitIgnored(files)
	return files, nil
}

func formatPromptHeading(body string) string {
	if body == "" {
		return "# 🤖 Prompt"
	}
	return "# 🤖 Prompt\n\n" + body
}

func formatSummaryHeading(body string) string {
	if body == "" {
		return "# 🔍 Summary"
	}
	return "# 🔍 Summary\n\n" + body
}

func buildTicketMarkdown(goal, parent string) string {
	var builder strings.Builder
	if goal != "" || parent != "" {
		builder.WriteString("---\n")
		if goal != "" {
			builder.WriteString(fmt.Sprintf("goal: %s\n", goal))
		}
		if parent != "" {
			builder.WriteString(fmt.Sprintf("parent: %s\n", parent))
		}
		builder.WriteString("---\n\n")
	}
	builder.WriteString("# Ticket\n\n## Summary\n\n")
	builder.WriteString("## Changes\n\n")
	builder.WriteString("## Log\n\n")
	builder.WriteString("## Todos\n\n")
	builder.WriteString("## Plan\n")
	return builder.String()
}

func updateTicketSummaryFile(ticketPath, summary string) error {
	if ticketPath == "" {
		return nil
	}
	content, err := ReadTextFile(ticketPath)
	if err != nil {
		return err
	}
	marker := "## Summary"
	if !strings.Contains(content, marker) {
		content = strings.TrimRight(content, "\n") + "\n\n" + marker + "\n\n" + summary + "\n"
		return WriteTextFile(ticketPath, content)
	}
	return WriteTextFile(ticketPath, replaceSectionContent(content, marker, summary))
}

// replaceSectionContent replaces the content of a markdown section (## heading)
// with new content, preserving all subsequent sections.
func replaceSectionContent(content, sectionHeading, newContent string) string {
	idx := strings.Index(content, sectionHeading)
	if idx == -1 {
		return content
	}
	before := content[:idx]
	after := content[idx+len(sectionHeading):]

	// Find the next ## heading after this section
	nextHeading := strings.Index(after, "\n## ")
	if nextHeading == -1 {
		// No next section, replace everything after the heading
		return strings.TrimRight(before, "\n") + "\n\n" + sectionHeading + "\n\n" + newContent + "\n"
	}
	// Keep everything from the next heading onward
	rest := after[nextHeading:]
	return strings.TrimRight(before, "\n") + "\n\n" + sectionHeading + "\n\n" + newContent + rest
}

func FilterTicketWorkspaceFiles(ticket *Ticket, files []string) []string {
	if ticket == nil {
		return files
	}
	if len(files) == 0 {
		return files
	}
	if ticket.FolderPath == "" {
		return files
	}
	relative := NormalizePath(ticket.FolderPath)
	if filepath.IsAbs(ticket.FolderPath) {
		relative = GetRelativePath(ticket.FolderPath)
	}
	relative = strings.TrimPrefix(relative, "./")
	if relative == "" {
		return files
	}
	filtered := make([]string, 0, len(files))
	for _, filePath := range files {
		normalized := NormalizePath(filePath)
		if filepath.IsAbs(filePath) {
			normalized = GetRelativePath(filePath)
		}
		normalized = strings.TrimPrefix(normalized, "./")
		if normalized == relative || strings.HasPrefix(normalized, relative+"/") {
			continue
		}
		filtered = append(filtered, filePath)
	}
	return filtered
}

func ghAddComment(issueURL, comment string) error {
	args := []string{"issue", "comment", issueURL, "--body", comment}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue comment failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

func ghAddLabels(issueURL string, labels []string) error {
	if len(labels) == 0 {
		return nil
	}
	args := []string{"issue", "edit", issueURL}
	for _, label := range labels {
		args = append(args, "--add-label", label)
	}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue edit failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

func ghCloseIssue(issueURL string) error {
	args := []string{"issue", "close", issueURL}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue close failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

func ghReopenIssue(issueURL string) error {
	args := []string{"issue", "reopen", issueURL}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue reopen failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

func ghUpdateIssueTitle(issueURL, title string) error {
	args := []string{"issue", "edit", issueURL, "--title", title}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue edit failed: %s", strings.TrimSpace(stderr))
	}
	return nil
}

// ghListOpenIssuesWithLabel returns all open issue URLs with the specified label
func ghListOpenIssuesWithLabel(label string) ([]string, error) {
	args := []string{"issue", "list", "--label", label, "--state", "open", "--json", "url", "--limit", "1000"}
	stdout, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("gh issue list failed: %s", strings.TrimSpace(stderr))
	}
	var issues []struct {
		URL string `json:"url"`
	}
	if err := json.Unmarshal([]byte(stdout), &issues); err != nil {
		return nil, fmt.Errorf("failed to parse gh issue list output: %w", err)
	}
	urls := make([]string, len(issues))
	for i, issue := range issues {
		urls[i] = issue.URL
	}
	return urls, nil
}

func SaveTicket(ticket *Ticket) error {
	if false {
		return fmt.Errorf("ticket data is nil")
	}
	jsonBytes, err := json.MarshalIndent(ticket, "", "  ")
	if err != nil {
		return err
	}
	return WriteTextFile(ticket.JsonPath, string(jsonBytes))
}

func ReadTicket(year, month, day int, slug string) (*Ticket, error) {
	folderPath := GetTicketPath(year, month, day, slug)
	jsonPath := GetTicketJsonPath(year, month, day, slug)
	ticketPath := GetTicketFilePath(year, month, day, slug)
	if !FileExists(jsonPath) {
		return nil, fmt.Errorf("ticket not found: %s", jsonPath)
	}
	raw, err := ReadTextFile(jsonPath)
	if err != nil {
		return nil, err
	}
	var ticket Ticket
	if err := json.Unmarshal([]byte(raw), &ticket); err != nil {
		return nil, err
	}

	ticket.Year = year
	ticket.Month = month
	ticket.Day = day
	ticket.Slug = slug
	ticket.FolderPath = folderPath
	ticket.JsonPath = jsonPath
	ticket.TicketPath = ticketPath
	ticket.ImportantPath = GetImportantFilePath(year, month, day, slug)

	return &ticket, nil
}

func ListTickets(year, month, day *int) ([]Ticket, error) {
	ticketsDir := GetTicketsDir()
	if !FileExists(ticketsDir) {
		return nil, nil
	}
	var tickets []Ticket
	var years []string
	if year != nil {
		years = []string{strconv.Itoa(*year)}
	} else {
		entries, err := os.ReadDir(ticketsDir)
		if err != nil {
			return nil, err
		}
		for _, e := range entries {
			if e.IsDir() {
				years = append(years, e.Name())
			}
		}
	}
	for _, y := range years {
		yearPath := filepath.Join(ticketsDir, y)
		if !FileExists(yearPath) {
			continue
		}
		var months []string
		if month != nil {
			months = []string{PadNumber(*month, 2)}
		} else {
			entries, err := os.ReadDir(yearPath)
			if err != nil {
				continue
			}
			for _, e := range entries {
				if e.IsDir() {
					months = append(months, e.Name())
				}
			}
		}
		for _, m := range months {
			monthPath := filepath.Join(yearPath, m)
			if !FileExists(monthPath) {
				continue
			}
			var days []string
			if day != nil {
				days = []string{PadNumber(*day, 2)}
			} else {
				entries, err := os.ReadDir(monthPath)
				if err != nil {
					continue
				}
				for _, e := range entries {
					if e.IsDir() {
						days = append(days, e.Name())
					}
				}
			}
			for _, d := range days {
				dayPath := filepath.Join(monthPath, d)
				if !FileExists(dayPath) {
					continue
				}
				filepath.WalkDir(dayPath, func(path string, dEntry fs.DirEntry, err error) error {
					if err != nil {
						return nil
					}
					if !dEntry.IsDir() && dEntry.Name() == "ticket.json" {
						dir := filepath.Dir(path)
						rel, err := filepath.Rel(dayPath, dir)
						if err != nil {
							return nil
						}
						slug := filepath.ToSlash(rel)
						yearInt, _ := strconv.Atoi(y)
						monthInt, _ := strconv.Atoi(m)
						dayInt, _ := strconv.Atoi(d)
						ticket, err := ReadTicket(yearInt, monthInt, dayInt, slug)
						if err == nil {
							tickets = append(tickets, *ticket)
						}
					}
					return nil
				})
			}
		}
	}
	return tickets, nil
}

func StreamTickets(ctx context.Context, year, month, day *int, out chan<- Ticket, opts ...StreamOptions) error {
	defer close(out)

	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	ticketsDir := GetTicketsDir()
	if !FileExists(ticketsDir) {
		return nil
	}
	var years []string
	if year != nil {
		years = []string{strconv.Itoa(*year)}
	} else {
		entries, err := os.ReadDir(ticketsDir)
		if err != nil {
			return err
		}
		for _, e := range entries {
			if e.IsDir() {
				years = append(years, e.Name())
			}
		}
	}
	// Sort years descending? ListTickets doesn't explicit sort but os.ReadDir returns sorted by name.
	// We proceed.

	for _, y := range years {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
		}

		yearPath := filepath.Join(ticketsDir, y)
		if !FileExists(yearPath) {
			continue
		}
		var months []string
		if month != nil {
			months = []string{PadNumber(*month, 2)}
		} else {
			entries, err := os.ReadDir(yearPath)
			if err != nil {
				continue
			}
			for _, e := range entries {
				if e.IsDir() {
					months = append(months, e.Name())
				}
			}
		}
		for _, m := range months {
			monthPath := filepath.Join(yearPath, m)
			if !FileExists(monthPath) {
				continue
			}
			var days []string
			if day != nil {
				days = []string{PadNumber(*day, 2)}
			} else {
				entries, err := os.ReadDir(monthPath)
				if err != nil {
					continue
				}
				for _, e := range entries {
					if e.IsDir() {
						days = append(days, e.Name())
					}
				}
			}
			for _, d := range days {
				dayPath := filepath.Join(monthPath, d)
				if !FileExists(dayPath) {
					continue
				}
				filepath.WalkDir(dayPath, func(path string, dEntry fs.DirEntry, err error) error {
					if err != nil {
						return nil
					}
					select {
					case <-ctx.Done():
						return ctx.Err()
					default:
					}

					if !dEntry.IsDir() && dEntry.Name() == "ticket.json" {
						dir := filepath.Dir(path)
						rel, err := filepath.Rel(dayPath, dir)
						if err != nil {
							return nil
						}
						slug := filepath.ToSlash(rel)
						yearInt, _ := strconv.Atoi(y)
						monthInt, _ := strconv.Atoi(m)
						dayInt, _ := strconv.Atoi(d)
						ticket, err := ReadTicket(yearInt, monthInt, dayInt, slug)
						if err == nil {
							// Filter ID/Slug/Title
							if !matchesFilter(ticket.GetID(), options) && !matchesFilter(ticket.Slug, options) && !matchesFilter(ticket.Title, options) {
								return nil
							}

							// Filter Kinds
							if !ticketMatchesKinds(ticket, options) {
								return nil
							}

							out <- *ticket
						}
					}
					return nil
				})
			}
		}
	}
	return nil
}

func ticketMatchesKinds(t *Ticket, opts StreamOptions) bool {
	if len(opts.IncludeKinds) == 0 && len(opts.ExcludeKinds) == 0 {
		return true
	}

	diffs := t.GetFiles()
	allFiles := map[string]bool{}
	for _, f := range diffs.Files.Added {
		allFiles[f.Path] = true
	}
	for _, f := range diffs.Files.Modified {
		allFiles[f.Path] = true
	}
	for _, f := range diffs.Files.Deleted {
		allFiles[f.Path] = true
	}
	for _, f := range diffs.Files.Renamed {
		allFiles[f.To] = true
	}

	if len(allFiles) == 0 {
		// If filtering by kind is active, valid matches must have files of that kind.
		// If only-code is set, and no files, return false.
		if len(opts.IncludeKinds) > 0 {
			return false
		}
		// If no-code is set, and no files, return true (ignoring kinds logic for empty set usually passes exclusions)
		return true
	}

	hasIncluded := false
	hasExcluded := false

	// If inclusions are specified, we start with false. If none, we start with true (everything matches unless excluded)
	if len(opts.IncludeKinds) == 0 {
		hasIncluded = true
	}

	for f := range allFiles {
		kind := DeriveFileKind(filepath.Base(f))

		if len(opts.IncludeKinds) > 0 {
			for _, k := range opts.IncludeKinds {
				if k == kind {
					hasIncluded = true
				}
			}
		}

		for _, k := range opts.ExcludeKinds {
			if k == kind {
				hasExcluded = true
			}
		}
	}

	if len(opts.IncludeKinds) > 0 && !hasIncluded {
		return false
	}

	if len(opts.ExcludeKinds) > 0 && hasExcluded {
		return false
	}

	return true
}

var (
	projectCache       []Project
	projectCacheLoaded bool
	projectCacheMutex  sync.Mutex
)

func InvalidateProjectCache() {
	projectCacheMutex.Lock()
	defer projectCacheMutex.Unlock()
	projectCacheLoaded = false
	projectCache = nil
}

func LoadProjects() []Project {
	projectCacheMutex.Lock()
	defer projectCacheMutex.Unlock()

	if projectCacheLoaded {
		return projectCache
	}
	projectCache = loadProjectsInternal()
	projectCacheLoaded = true
	return projectCache
}

func loadProjectsInternal() []Project {
	var projects []Project
	projectsDir := rootDir
	entries, err := os.ReadDir(projectsDir)
	if err != nil {
		return nil
	}
	// Add root project for semio-repo/go if needed?
	// User said "@semio-repo/go" -> Project semio-repo.
	// We handle folders starting with @.

	for _, d := range entries {
		if !d.IsDir() {
			continue
		}
		name := d.Name()
		if !strings.HasPrefix(name, "@") {
			continue
		}

		rawName := strings.TrimPrefix(name, "@")
		project := Project{
			Name:    rawName,
			Root:    name,
			Kind:    DeriveProjectKind(rawName),
			Bundles: []Bundle{},
		}

		// Scan for bundles in subdirectories
		projectPath := filepath.Join(projectsDir, name)
		subEntries, _ := os.ReadDir(projectPath)
		for _, sub := range subEntries {
			if !sub.IsDir() {
				continue
			}
			if strings.HasPrefix(sub.Name(), ".") {
				continue
			}
			bunName := sub.Name()
			fullBundleName := name + "/" + bunName

			// Derive bundle kind
			bundlePath := filepath.Join(name, bunName)
			kind := DeriveBundleKind(fullBundleName, bundlePath)

			bundle := Bundle{
				Name:        fullBundleName,
				Root:        bundlePath,
				ProjectName: rawName,
				Kind:        kind,
			}

			// Load Packages map
			bundle.Packages = loadPackages(filepath.Join(rootDir, bundlePath))

			// Try to load additional metadata from project.json/package.json
			configPath := filepath.Join(projectPath, bunName, "project.json")
			if !FileExists(configPath) {
				configPath = filepath.Join(projectPath, bunName, "package.json")
			}
			if FileExists(configPath) {
				content, err := ReadTextFile(configPath)
				if err == nil {
					var meta struct {
						SourceRoot string   `json:"sourceRoot"`
						Tags       []string `json:"tags"`
					}
					if json.Unmarshal([]byte(content), &meta) == nil {
						bundle.SourceRoot = meta.SourceRoot
						bundle.Tags = meta.Tags
					}
				}
			}
			project.Bundles = append(project.Bundles, bundle)
		}
		projects = append(projects, project)
	}

	return projects
}

func LoadCommits(limit *int) []Commit {
	args := []string{"log", "--pretty=format:%H|%aN|%ad|%s", "--date=iso-strict"}
	if limit != nil {
		args = append(args, fmt.Sprintf("-n%d", *limit))
	}

	cmd := exec.Command("git", args...)
	cmd.Dir = rootDir
	out, err := cmd.Output()
	if err != nil {
		return []Commit{}
	}
	lines := strings.Split(string(out), "\n")
	var commits []Commit
	for _, line := range lines {
		parts := strings.Split(line, "|")
		if len(parts) >= 4 {
			sha := parts[0]
			// author := parts[1]
			dateStr := parts[2]
			title := strings.Join(parts[3:], "|")
			date, _ := time.Parse(time.RFC3339, dateStr)

			commits = append(commits, Commit{
				ID:    sha,
				SHA:   sha,
				Title: title,
				Date:  date,
			})
		}
	}
	return commits
}

func LoadBundles() []Bundle {
	var bundles []Bundle
	projects := LoadProjects()
	for _, p := range projects {
		bundles = append(bundles, p.Bundles...)
	}
	return bundles
}

func GetProjects() []Bundle {
	return LoadBundles()
}

func StreamBundles(ctx context.Context, out chan<- Bundle, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	bundles := LoadBundles()
	for _, b := range bundles {
		if !matchesFilter(b.Name, options) {
			continue
		}

		if !shouldIncludeBundleKind(b.Kind, options) {
			continue
		}

		if !bundleMatchesKinds(b, options) {
			continue
		}

		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			out <- b
		}
	}
	return nil
}

func bundleMatchesKinds(b Bundle, opts StreamOptions) bool {
	if len(opts.IncludeKinds) == 0 && len(opts.ExcludeKinds) == 0 {
		return true
	}

	bundleRoot := filepath.Join(rootDir, b.Root)
	hasIncluded := false
	hasExcluded := false

	if len(opts.IncludeKinds) == 0 {
		hasIncluded = true
	}

	filepath.Walk(bundleRoot, func(path string, info os.FileInfo, err error) error {
		if err != nil || info.IsDir() {
			return nil
		}
		if isRepoExcludedPath(path) || isGitIgnored(path) {
			return nil
		}

		kind := DeriveFileKind(filepath.Base(path))

		if len(opts.IncludeKinds) > 0 {
			for _, k := range opts.IncludeKinds {
				if k == kind {
					hasIncluded = true
				}
			}
		}

		for _, k := range opts.ExcludeKinds {
			if k == kind {
				hasExcluded = true
			}
		}

		if hasIncluded && (len(opts.ExcludeKinds) == 0 || hasExcluded) {
			return filepath.SkipAll
		}
		return nil
	})

	if len(opts.IncludeKinds) > 0 && !hasIncluded {
		return false
	}

	if len(opts.ExcludeKinds) > 0 && hasExcluded {
		return false
	}

	return true
}

func loadPackages(bundleRoot string) []Package {
	var packages []Package
	filepath.WalkDir(bundleRoot, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return nil
		}
		// Skip known vendor/output directories and hidden directories
		if d.IsDir() {
			name := d.Name()
			if name == "node_modules" || name == ".git" || name == "bin" || name == "obj" || name == "target" || name == "dist" || name == "build" || name == "__pycache__" || strings.HasPrefix(name, ".") {
				return fs.SkipDir
			}
			return nil
		}

		rel, _ := filepath.Rel(bundleRoot, path)

		name := ""
		version := ""
		kind := ""

		filename := d.Name()
		switch filename {
		case "package.json":
			kind = "npm"
			if content, err := ioutil.ReadFile(path); err == nil {
				var meta struct {
					Name    string `json:"name"`
					Version string `json:"version"`
				}
				json.Unmarshal(content, &meta)
				name = meta.Name
				version = meta.Version
			}
		case "go.mod":
			kind = "go"
			if content, err := ioutil.ReadFile(path); err == nil {
				lines := strings.Split(string(content), "\n")
				if len(lines) > 0 && strings.HasPrefix(lines[0], "module ") {
					name = strings.TrimSpace(strings.TrimPrefix(lines[0], "module "))
				}
			}
		case "Cargo.toml":
			kind = "cargo"
			if content, err := ioutil.ReadFile(path); err == nil {
				lines := strings.Split(string(content), "\n")
				for _, line := range lines {
					if strings.HasPrefix(line, "name =") {
						name = strings.Trim(strings.TrimSpace(strings.TrimPrefix(line, "name =")), "\"")
						break
					}
				}
			}
		case "pyproject.toml":
			kind = "pip"
			if content, err := ioutil.ReadFile(path); err == nil {
				lines := strings.Split(string(content), "\n")
				for _, line := range lines {
					if strings.HasPrefix(line, "name =") {
						name = strings.Trim(strings.TrimSpace(strings.TrimPrefix(line, "name =")), "\"")
						break
					}
				}
			}
		}

		if strings.HasSuffix(filename, ".csproj") {
			kind = "nuget"
			name = strings.TrimSuffix(filename, ".csproj")
		}

		if kind != "" && name != "" {
			packages = append(packages, Package{
				Name:    name,
				Version: version,
				Path:    rel,
				Kind:    kind,
			})
		}
		return nil
	})
	return packages
}

func StreamProjects(ctx context.Context, out chan<- Project, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	projects := LoadProjects()
	for _, p := range projects {
		if !matchesFilter(p.Name, options) {
			continue
		}
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			out <- p
		}
	}
	return nil
}

func runProjectList(factory EngineFactory, config Config, cmd *cobra.Command, args []string) error {
	opts := getStreamOptions(cmd)
	stream := make(chan Event)
	go func() {
		defer close(stream)
		stream <- Event{Kind: KindStart, Command: "project list"}

		projChan := make(chan Project)
		go func() {
			StreamProjects(context.Background(), projChan, opts)
		}()

		for p := range projChan {
			data, err := json.Marshal(map[string]interface{}{"project": p})
			if err != nil {
				continue
			}
			stream <- Event{Kind: KindResult, Command: "project list", Data: data}
		}
		stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
	}()

	return renderStream(cmd, &config, stream)
}

func runProjectTree(factory EngineFactory, config Config, cmd *cobra.Command, args []string) error {
	opts := getStreamOptions(cmd)
	stream := make(chan Event)
	go func() {
		defer close(stream)
		stream <- Event{Kind: KindStart, Command: "project tree"}

		projChan := make(chan Project)
		go func() {
			StreamProjects(context.Background(), projChan, opts)
		}()

		var projects []Project
		for p := range projChan {
			projects = append(projects, p)
		}
		sort.Slice(projects, func(i, j int) bool { return projects[i].Name < projects[j].Name })

		for _, p := range projects {
			data, err := json.Marshal(map[string]interface{}{"project": p})
			if err != nil {
				continue
			}
			stream <- Event{Kind: KindResult, Command: "project tree", Data: data}
		}
		stream <- Event{Kind: KindDone, Done: &DonePayload{ExitCode: 0, Status: "ok"}}
	}()

	return renderStream(cmd, &config, stream)
}

type StreamOptions struct {
	ShowIgnored    bool
	ShowGenerated  bool
	ExcludeKinds   []string
	IncludeKinds   []string
	Filter         string
	Regex          bool
	MatchCase      bool
	MatchWholeWord bool

	ExcludeBundleKinds     []BundleKind
	IncludeBundleKinds     []BundleKind
	ExcludeFolderKinds     []FolderKind
	IncludeFolderKinds     []FolderKind
	ExcludeDefinitionKinds []DefinitionKind
	IncludeDefinitionKinds []DefinitionKind

	ExcludeYears        []int
	IncludeYears        []int
	ExcludeMonths       []int
	IncludeMonths       []int
	ExcludeDays         []int
	IncludeDays         []int
	ExcludeContributors []string
	IncludeContributors []string
	ExcludePolicies     []string
	IncludePolicies     []string
	ExcludeViolations   []string
	IncludeViolations   []string
}

func matchesFilter(name string, opts StreamOptions) bool {
	if opts.Filter == "" {
		return true
	}

	target := name
	pattern := opts.Filter

	if !opts.MatchCase && !opts.Regex {
		target = strings.ToLower(target)
		pattern = strings.ToLower(pattern)
	}

	if opts.Regex {
		re, err := regexp.Compile(opts.Filter)
		if err != nil {
			return false
		}
		return re.MatchString(name)
	}

	if opts.MatchWholeWord {
		words := strings.FieldsFunc(target, func(r rune) bool {
			return !((r >= 'a' && r <= 'z') || (r >= 'A' && r <= 'Z') || (r >= '0' && r <= '9') || r == '_')
		})
		for _, w := range words {
			if w == pattern {
				return true
			}
		}
		return false
	}

	return strings.Contains(target, pattern)
}

func shouldIncludeKind(kind string, opts StreamOptions) bool {
	if len(opts.IncludeKinds) > 0 {
		found := false
		for _, k := range opts.IncludeKinds {
			if k == kind {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}

	for _, k := range opts.ExcludeKinds {
		if k == kind {
			return false
		}
	}
	return true
}

func shouldIncludeBundleKind(kind BundleKind, opts StreamOptions) bool {
	if len(opts.IncludeBundleKinds) > 0 {
		found := false
		for _, k := range opts.IncludeBundleKinds {
			if k == kind {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}

	for _, k := range opts.ExcludeBundleKinds {
		if k == kind {
			return false
		}
	}
	return true
}

func shouldIncludeFolderKind(kind FolderKind, opts StreamOptions) bool {
	if len(opts.IncludeFolderKinds) > 0 {
		found := false
		for _, k := range opts.IncludeFolderKinds {
			if k == kind {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}

	for _, k := range opts.ExcludeFolderKinds {
		if k == kind {
			return false
		}
	}
	return true
}

func shouldIncludeDefinitionKind(kind DefinitionKind, opts StreamOptions) bool {
	if len(opts.IncludeDefinitionKinds) > 0 {
		found := false
		for _, k := range opts.IncludeDefinitionKinds {
			if k == kind {
				found = true
				break
			}
		}
		if !found {
			return false
		}
	}

	for _, k := range opts.ExcludeDefinitionKinds {
		if k == kind {
			return false
		}
	}
	return true
}

func StreamFolders(ctx context.Context, scope string, out chan<- Folder, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	root := rootDir
	if bundleName, found := strings.CutPrefix(scope, "@semio/"); found {
		bundles := GetProjects()
		for _, b := range bundles {
			if b.Name == bundleName || normalizeBundleLabel(b.Name) == bundleName {
				root = filepath.Join(rootDir, b.Root)
				break
			}
		}
	} else if scope != "" && scope != "@semio" {
		if filepath.IsAbs(scope) {
			root = scope
		} else {
			root = filepath.Join(rootDir, scope)
		}
	}

	var folders []string
	err := filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if !info.IsDir() {
			return nil
		}
		rel, _ := filepath.Rel(root, path)
		if rel == "." {
			return nil
		}
		if isRepoExcludedPath(path) {
			return filepath.SkipDir
		}
		if isGitIgnored(path) && !options.ShowIgnored {
			return filepath.SkipDir
		}
		folders = append(folders, path)
		return nil
	})
	if err != nil {
		return err
	}

	for _, folderPath := range folders {
		ignored := isGitIgnored(folderPath)
		relPath, _ := filepath.Rel(rootDir, folderPath)
		generated := IsGenerated(folderPath) || IsGeneratedFolder(relPath)
		folderKind := DeriveFolderKind(relPath)

		if ignored && !options.ShowIgnored {
			continue
		}
		if generated && !options.ShowGenerated {
			continue
		}

		if !shouldIncludeFolderKind(folderKind, options) {
			continue
		}

		if !matchesFilter(filepath.Base(folderPath), options) {
			continue
		}

		var bundleID *string
		if b := GetBundleByPath(relPath); b != nil {
			id := b.GetID()
			bundleID = &id
		}

		parentPath := filepath.Dir(relPath)
		var parentID *string
		if parentPath != "." {
			id := buildFolderID(parentPath, bundleID)
			parentID = &id
		}

		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			out <- Folder{
				ID:        buildFolderID(relPath, bundleID),
				Path:      relPath,
				URI:       "semiorepo://FOLDER/" + Slugify(buildFolderID(relPath, bundleID)),
				Name:      filepath.Base(relPath),
				ParentID:  parentID,
				BundleID:  bundleID,
				Kind:      folderKind,
				Ignored:   ignored,
				Generated: generated,
			}
		}
	}
	return nil
}

func StreamFiles(ctx context.Context, scope string, out chan<- File, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	root := rootDir
	if bundleName, found := strings.CutPrefix(scope, "@semio/"); found {
		bundles := GetProjects()
		matched := false
		for _, b := range bundles {
			if b.Name == bundleName || normalizeBundleLabel(b.Name) == bundleName {
				root = filepath.Join(rootDir, b.Root)
				matched = true
				break
			}
		}
		// If no exact bundle match, try matching bundle prefix + sub-path
		// e.g. "@semio/js/semio.ts" -> bundle "@semio/js" + sub-path "semio.ts"
		if !matched {
			parts := strings.SplitN(bundleName, "/", 2)
			if len(parts) == 2 {
				prefix := parts[0]
				subPath := parts[1]
				for _, b := range bundles {
					bName := b.Name
					if strings.HasPrefix(bName, "@semio/") {
						bName = strings.TrimPrefix(bName, "@semio/")
					}
					if bName == prefix || normalizeBundleLabel(bName) == "@semio/"+prefix {
						root = filepath.Join(rootDir, b.Root, subPath)
						matched = true
						break
					}
				}
			}
		}
	} else if scope != "" && scope != "@semio" {
		if filepath.IsAbs(scope) {
			root = scope
		} else {
			root = filepath.Join(rootDir, scope)
		}
	}

	// If root is a file (not a directory), return just that file
	if info, err := os.Stat(root); err == nil && !info.IsDir() {
		relPath, _ := filepath.Rel(rootDir, root)
		var bundleID *string
		if b := GetBundleByPath(relPath); b != nil {
			id := b.GetID()
			bundleID = &id
		}
		folderPath := filepath.Dir(relPath)
		var folderID *string
		if folderPath != "." {
			id := buildFolderID(folderPath, bundleID)
			folderID = &id
		}
		out <- File{
			ID:        buildFileID(relPath, bundleID),
			Path:      relPath,
			URI:       "semiorepo://FILE/" + Slugify(buildFileID(relPath, bundleID)),
			Name:      filepath.Base(relPath),
			Extension: filepath.Ext(relPath),
			FolderID:  folderID,
			BundleID:  bundleID,
			Kind:      DeriveFileKind(filepath.Base(relPath)),
			Ignored:   isGitIgnored(root),
			Generated: IsGenerated(root),
		}
		return nil
	}

	var files []string
	err := filepath.Walk(root, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return nil
		}
		if info.IsDir() {
			rel, _ := filepath.Rel(root, path)
			if rel != "." {
				if isRepoExcludedPath(path) {
					return filepath.SkipDir
				}
				if isGitIgnored(path) && !options.ShowIgnored {
					return filepath.SkipDir
				}
			}
			return nil
		}
		if isRepoExcludedPath(path) {
			return nil
		}
		if isGitIgnored(path) && !options.ShowIgnored {
			return nil
		}
		files = append(files, path)
		return nil
	})
	if err != nil {
		return err
	}

	for _, filePath := range files {
		ignored := isGitIgnored(filePath)
		generated := IsGenerated(filePath)
		kind := DeriveFileKind(filepath.Base(filePath))

		if ignored && !options.ShowIgnored {
			continue
		}
		if generated && !options.ShowGenerated {
			continue
		}

		if !shouldIncludeKind(kind, options) {
			continue
		}

		name := filepath.Base(filePath)
		if !matchesFilter(name, options) {
			continue
		}

		relPath, _ := filepath.Rel(rootDir, filePath)
		var bundleID *string
		if b := GetBundleByPath(relPath); b != nil {
			id := b.GetID()
			bundleID = &id
		}

		folderPath := filepath.Dir(relPath)
		var folderID *string
		if folderPath != "." {
			id := buildFolderID(folderPath, bundleID)
			folderID = &id
		}

		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			out <- File{
				ID:        buildFileID(relPath, bundleID),
				Path:      relPath,
				URI:       "semiorepo://FILE/" + Slugify(buildFileID(relPath, bundleID)),
				Name:      filepath.Base(relPath),
				Extension: filepath.Ext(relPath),
				FolderID:  folderID,
				BundleID:  bundleID,
				Kind:      kind,
				Ignored:   ignored,
				Generated: generated,
			}
		}
	}
	return nil
}

func flattenSections(sections []Section) []Section {
	return flattenSectionsWithPrefix(sections, "")
}

func flattenSectionsWithPrefix(sections []Section, prefix string) []Section {
	var result []Section
	for _, s := range sections {
		children := s.Children
		s.Children = nil
		if prefix != "" {
			s.Path = prefix + "#" + s.Name
		} else {
			s.Path = s.Name
		}
		result = append(result, s)
		result = append(result, flattenSectionsWithPrefix(children, s.Path)...)
	}
	return result
}

func StreamSections(ctx context.Context, scope string, out chan<- Section, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	filesOpts := options
	filesOpts.Filter = ""
	filesOpts.Regex = false

	// Iterate files, parse sections
	fileChan := make(chan File)
	go func() {
		StreamFiles(ctx, scope, fileChan, filesOpts)
	}()

	for f := range fileChan {
		fullPath := f.Path
		if !filepath.IsAbs(fullPath) {
			fullPath = filepath.Join(rootDir, fullPath)
		}
		content, err := ReadTextFile(fullPath)
		if err != nil {
			continue
		}
		sections := ParseSections(content, f.Path)
		flatSections := flattenSections(sections)
		for _, s := range flatSections {
			if !matchesFilter(s.Name, options) {
				continue
			}

			select {
			case <-ctx.Done():
				return ctx.Err()
			default:
				out <- s
			}
		}
	}
	return nil
}

func StreamDefinitions(ctx context.Context, scope string, out chan<- Definition, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	filesOpts := options
	filesOpts.Filter = ""
	filesOpts.Regex = false

	// Iterate files, parse definitions
	fileChan := make(chan File)
	go func() {
		StreamFiles(ctx, scope, fileChan, filesOpts)
	}()

	for f := range fileChan {
		fullPath := f.Path
		if !filepath.IsAbs(fullPath) {
			fullPath = filepath.Join(rootDir, fullPath)
		}
		content, err := ReadTextFile(fullPath)
		if err != nil {
			continue
		}
		lang := GetLanguage(f.Path)
		if lang == nil {
			continue
		}
		lines := strings.Split(content, "\n")
		defs := lang.ParseDefinitions(content, lines)
		for _, d := range defs {
			rawKind := d.Kind
			if rawKind == "" {
				rawKind = "definition"
			}
			kind := DeriveDefinitionKind(rawKind)

			if !shouldIncludeDefinitionKind(kind, options) {
				continue
			}

			def := Definition{
				Name:      d.Name,
				Kind:      kind,
				FilePath:  f.Path,
				StartLine: d.Start,
				EndLine:   d.End,
			}

			if !matchesFilter(def.Name, options) {
				continue
			}

			select {
			case <-ctx.Done():
				return ctx.Err()
			default:
				out <- def
			}
		}
	}
	return nil
}

// GetProjects legacy removed

func ResolveBundleForPath(filePath string, bundles []Bundle) string {
	var bestMatch string
	var maxLen int
	for _, b := range bundles {
		if b.Name == "" {
			continue
		}
		if strings.HasPrefix(filePath, b.Root+"/") || filePath == b.Root {
			if len(b.Root) > maxLen {
				maxLen = len(b.Root)
				bestMatch = normalizeBundleLabel(b.Name)
			}
		}
	}
	return bestMatch
}

func formatLineMetrics(metrics *LineMetrics) string {
	if metrics == nil {
		return ""
	}
	var parts []string
	if metrics.Removed > 0 {
		parts = append(parts, fmt.Sprintf("-%d", metrics.Removed))
	}
	if metrics.Added > 0 {
		parts = append(parts, fmt.Sprintf("+%d", metrics.Added))
	}
	if len(parts) == 0 {
		return ""
	}
	return " " + strings.Join(parts, " ")
}

func formatPathWithBundle(path string, bundles []Bundle) string {
	bundleName := ResolveBundleForPath(path, bundles)
	if bundleName == "" {
		return path
	}
	bundleLabel := normalizeBundleLabel(bundleName)
	root := ""
	for _, bundle := range bundles {
		if normalizeBundleLabel(bundle.Name) == bundleLabel || bundle.Name == bundleName {
			root = bundle.Root
			break
		}
	}
	if root != "" {
		relative := strings.TrimPrefix(path, root+"/")
		if relative != path {
			if relative == "" {
				return bundleLabel
			}
			return bundleLabel + "/" + relative
		}
	}
	return bundleLabel + "/" + path
}

func formatSemanticPath(path string, bundles []Bundle) string {
	filePath := extractFilePrefix(path)
	remainder := path[len(filePath):]
	return formatPathWithBundle(filePath, bundles) + remainder
}

func formatDeletedPath(path string, bundles []Bundle) string {
	filePath := extractFilePrefix(path)
	remainder := path[len(filePath):]
	base := formatPathWithBundle(filePath, bundles)
	if remainder == "" {
		return "<del>" + base + "</del>"
	}
	return base + "<del>" + remainder + "</del>"
}

func commonPrefixLength(a, b string) int {
	limit := len(a)
	if len(b) < limit {
		limit = len(b)
	}
	idx := 0
	for idx < limit {
		if a[idx] != b[idx] {
			break
		}
		idx++
	}
	return idx
}

func commonSuffixLength(a, b string, prefix int) int {
	max := len(a) - prefix
	if len(b)-prefix < max {
		max = len(b) - prefix
	}
	idx := 0
	for idx < max {
		if a[len(a)-1-idx] != b[len(b)-1-idx] {
			break
		}
		idx++
	}
	return idx
}

func formatRenameDelta(from, to string) string {
	if from == to {
		return from
	}
	prefix := commonPrefixLength(from, to)
	suffix := commonSuffixLength(from, to, prefix)
	fromMiddle := from[prefix : len(from)-suffix]
	toMiddle := to[prefix : len(to)-suffix]
	return from[:prefix] + "<del>" + fromMiddle + "</del>" + toMiddle + from[len(from)-suffix:]
}

func formatRenamePath(from, to string, bundles []Bundle) string {
	fromFormatted := formatSemanticPath(from, bundles)
	toFormatted := formatSemanticPath(to, bundles)
	return formatRenameDelta(fromFormatted, toFormatted)
}

func appendDiffLines(lines *[]string, diffSet TicketDiffSet, iconAdded, iconChanged, iconRemoved, iconRenamed string, bundles []Bundle, formatter func(string) string, renameFormatter func(string, string) string) {
	if len(diffSet.Added) > 0 {
		sort.Slice(diffSet.Added, func(i, j int) bool { return diffSet.Added[i].Path < diffSet.Added[j].Path })
		for _, entry := range diffSet.Added {
			path := formatter(entry.Path)
			*lines = append(*lines, fmt.Sprintf("%s%s%s", iconAdded, path, formatLineMetrics(entry.Lines)))
		}
	}
	if len(diffSet.Modified) > 0 {
		sort.Slice(diffSet.Modified, func(i, j int) bool { return diffSet.Modified[i].Path < diffSet.Modified[j].Path })
		for _, entry := range diffSet.Modified {
			path := formatter(entry.Path)
			*lines = append(*lines, fmt.Sprintf("%s%s%s", iconChanged, path, formatLineMetrics(entry.Lines)))
		}
	}
	if len(diffSet.Renamed) > 0 {
		sort.Slice(diffSet.Renamed, func(i, j int) bool { return diffSet.Renamed[i].To < diffSet.Renamed[j].To })
		for _, entry := range diffSet.Renamed {
			path := renameFormatter(entry.From, entry.To)
			*lines = append(*lines, fmt.Sprintf("%s%s%s", iconRenamed, path, formatLineMetrics(entry.Lines)))
		}
	}
	if len(diffSet.Deleted) > 0 {
		sort.Slice(diffSet.Deleted, func(i, j int) bool { return diffSet.Deleted[i].Path < diffSet.Deleted[j].Path })
		for _, entry := range diffSet.Deleted {
			path := formatDeletedPath(entry.Path, bundles)
			*lines = append(*lines, fmt.Sprintf("%s%s%s", iconRemoved, path, formatLineMetrics(entry.Lines)))
		}
	}
}

func generateMetricsComment(diffs *TicketDiffs, bundles []Bundle) string {
	if diffs == nil {
		return ""
	}
	var lines []string
	appendDiffLines(&lines, diffs.Bundles, "📦", "📦", "📦", "📦", bundles, func(path string) string {
		return path
	}, func(from, to string) string {
		return formatRenameDelta(from, to)
	})
	appendDiffLines(&lines, diffs.Folders, "📂", "📁", "📁", "📁", bundles, func(path string) string {
		return formatPathWithBundle(path, bundles)
	}, func(from, to string) string {
		return formatRenamePath(from, to, bundles)
	})
	appendDiffLines(&lines, diffs.Files, "📄", "📝", "📄", "📄", bundles, func(path string) string {
		return formatSemanticPath(path, bundles)
	}, func(from, to string) string {
		return formatRenamePath(from, to, bundles)
	})
	appendDiffLines(&lines, diffs.Sections, "📑", "🔖", "🔖", "🔖", bundles, func(path string) string {
		return formatSemanticPath(path, bundles)
	}, func(from, to string) string {
		return formatRenamePath(from, to, bundles)
	})
	appendDiffLines(&lines, diffs.Definitions, "🏷️", "🏷️", "🏷️", "🏷️", bundles, func(path string) string {
		return formatSemanticPath(path, bundles)
	}, func(from, to string) string {
		return formatRenamePath(from, to, bundles)
	})
	if len(lines) == 0 {
		return ""
	}
	return strings.Join(lines, "\n")
}

func ProgressTicket(ticket *Ticket, summary string) (string, error) {
	if summary == "" {
		return "No summary provided", nil
	}

	entry := fmt.Sprintf("\n- %s: %s", time.Now().Format("2006-01-02 15:04"), summary)

	content, err := ReadTextFile(ticket.TicketPath)
	if err != nil {
		return "", err
	}

	marker := "## Log"
	var newContent string

	if strings.Contains(content, marker) {
		newContent = strings.Replace(content, marker, marker+entry, 1)
	} else {
		sumMarker := "## Summary"
		if strings.Contains(content, sumMarker) {
			newContent = strings.Replace(content, sumMarker, marker+entry+"\n\n"+sumMarker, 1)
		} else {
			newContent = content + "\n\n" + marker + entry
		}
	}

	if err := WriteTextFile(ticket.TicketPath, newContent); err != nil {
		return "", err
	}

	return fmt.Sprintf("Logged progress to %s", ticket.Slug), nil
}

func FinishTicket(ticket *Ticket, summary string, files []string, noGithub bool, bulk bool) error {
	if !bulk {
		if summary == "" {
			return fmt.Errorf("summary is required to finish a ticket")
		}
		if len(files) == 0 {
			return fmt.Errorf("at least one file is required to finish a ticket")
		}
	} else {
		if summary == "" {
			summary = "Bulk close"
		}
	}

	// Validate important.md is empty
	if !bulk && FileExists(ticket.ImportantPath) {
		importantContent, err := ReadTextFile(ticket.ImportantPath)
		if err == nil && strings.TrimSpace(importantContent) != "" {
			return fmt.Errorf("cannot finish ticket: %s is not empty. Please complete all compulsory actions", filepath.Base(ticket.ImportantPath))
		}
		// Delete important.md file after validation
		if err := os.Remove(ticket.ImportantPath); err != nil {
			fmt.Printf("Warning: Failed to delete %s: %v\n", filepath.Base(ticket.ImportantPath), err)
		}
	}

	var tickFilesResult *TicketDiffs
	var err error
	if !bulk || len(files) > 0 {
		tickFilesResult, err = ComputeTicketFiles(ticket, files)
		if err != nil {
			return err
		}
	}

	if ticket.GitHub != nil && ticket.GitHub.Issue != "" && !noGithub {
		issueURL := ticket.GitHub.Issue

		// 1. Add comment with summary and metrics

		if !bulk {
			// 2. Add labels
			bundles := GetProjects()
			labels := make(map[string]struct{})
			if tickFilesResult != nil {
				addLabel := func(path string) {
					if path == "" {
						return
					}
					labels[path] = struct{}{}
				}
				bundleDiffs := tickFilesResult.Bundles
				for _, entry := range bundleDiffs.Added {
					addLabel(entry.Path)
				}
				for _, entry := range bundleDiffs.Modified {
					addLabel(entry.Path)
				}
				for _, entry := range bundleDiffs.Deleted {
					addLabel(entry.Path)
				}
				for _, entry := range bundleDiffs.Renamed {
					addLabel(entry.From)
					addLabel(entry.To)
				}
			}
			var labelList []string
			for l := range labels {
				labelList = append(labelList, l)
			}
			if len(labelList) > 0 {
				if err := ghAddLabels(issueURL, labelList); err != nil {
					fmt.Printf("Warning: Failed to add labels to GitHub issue: %v\n", err)
				}
			}

			// 3. Add metrics comment and close
			comment := formatSummaryHeading(summary)
			metricsComment := generateMetricsComment(tickFilesResult, bundles)
			if metricsComment != "" {
				comment += "\n\n# ✍️ Changes\n\n" + metricsComment
			}

			if err := ghAddComment(issueURL, comment); err != nil {
				fmt.Printf("Warning: Failed to add summary and metrics comment to GitHub issue: %v\n", err)
			}
		}

		// Close issue
		if err := ghCloseIssue(issueURL); err != nil {
			fmt.Printf("Warning: Failed to close GitHub issue: %v\n", err)
		}
	}

	ticket.Summary = summary
	if err := updateTicketSummaryFile(ticket.TicketPath, summary); err != nil {
		return err
	}
	ticket.Status = TicketStatusClosed
	now := time.Now()
	ticket.Finished = &now
	if len(ticket.Iterations) > 0 {
		lastIndex := len(ticket.Iterations) - 1
		ticket.Iterations[lastIndex].Diff = tickFilesResult
		ticket.Iterations[lastIndex].Finished = &now
	}
	ticket.Status = TicketStatusClosed
	return SaveTicket(ticket)
}

func ReopenTicket(ticket *Ticket, prompt, llm, ui, draft string, goal string, parent string, noGithub bool) error {
	if ticket.Status == TicketStatusOpen {
		return fmt.Errorf("ticket is already open")
	}
	if goal != "" {
		ticket.Goal = goal
	}
	if parent != "" {
		ticket.Parent = parent
	}
	gitAuthor := GetGitAuthorGithub()
	gitCommit := GetGitCommit()
	var llmSlug string
	var uiSlug string
	var err error
	if llm != "" {
		llmSlug, err = ResolveAllowedLLM(llm)
		if err != nil {
			return err
		}
	}
	if ui != "" {
		uiSlug, err = ResolveAllowedUI(ui)
		if err != nil {
			return err
		}
	} else {
		return fmt.Errorf("ui is required")
	}

	iteration := TicketIteration{
		Prompt:  prompt,
		LLM:     llmSlug,
		UI:      uiSlug,
		Author:  parseGitAuthor(gitAuthor),
		Started: time.Now(),
		Commit:  gitCommit,
	}

	// Handle draft for this iteration
	if draft != "" {
		draftPath := filepath.Join(GetDraftsPath(), draft)
		if IsDir(draftPath) {
			entries, err := os.ReadDir(draftPath)
			if err == nil {
				for _, entry := range entries {
					src := filepath.Join(draftPath, entry.Name())
					dst := filepath.Join(ticket.FolderPath, entry.Name())
					// Handle collision
					if FileExists(dst) {
						ext := filepath.Ext(entry.Name())
						name := strings.TrimSuffix(entry.Name(), ext)
						for i := 2; ; i++ {
							newDst := filepath.Join(ticket.FolderPath, fmt.Sprintf("%s_%d%s", name, i, ext))
							if !FileExists(newDst) {
								dst = newDst
								break
							}
						}
					}
					if err := MoveFile(src, dst); err != nil {
						fmt.Printf("Warning: Failed to move draft file %s: %v\n", entry.Name(), err)
						continue
					}
				}
			}
			os.RemoveAll(draftPath)
		}
	}

	ticket.Iterations = append(ticket.Iterations, iteration)
	ticket.Status = TicketStatusOpen
	ticket.Finished = nil

	if ticket.GitHub != nil && ticket.GitHub.Issue != "" && !noGithub {
		issueURL := ticket.GitHub.Issue
		if err := ghReopenIssue(issueURL); err != nil {
			fmt.Printf("Warning: Failed to reopen GitHub issue: %v\n", err)
		}
		ghAddIssueToProject(issueURL)
		comment := formatPromptHeading(prompt)
		if err := ghAddComment(issueURL, comment); err != nil {
			fmt.Printf("Warning: Failed to add prompt comment to GitHub issue: %v\n", err)
		}
	}

	return SaveTicket(ticket)
}

func ToolTicketOpen(title, prompt, llm, ui, draft string, noIssue bool, goal string, parent string, noGithub bool, issue string) ToolResult {
	output := NewOutput()
	resolvedPrompt := prompt
	if resolvedPrompt == "" {
		resolvedPrompt = title
	}
	ticket, err := OpenTicket(title, prompt, llm, ui, draft, noIssue, goal, parent, noGithub, issue)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if ticket == nil {
		output.Info("\n🚫 Ticket creation skipped (NOTICKET)")
		return ToolResult{Output: *output}
	}
	if shouldContinueTicket(resolvedPrompt) {
		output.Success(fmt.Sprintf("\n↩️ Continued ticket: %s", ticket.Slug))
		return ToolResult{Output: *output, Data: ticket}
	}
	output.Success(fmt.Sprintf("\n🎫 Created ticket: %s", ticket.Slug))
	output.Info(fmt.Sprintf("   Folder: %s", ticket.FolderPath))
	if ticket.JsonPath != "" {
		output.Info(fmt.Sprintf("   JSON:   %s", ticket.JsonPath))
	}
	if draft != "" {
		output.Info(fmt.Sprintf("   Draft:  %s", draft))
	}
	if ticket.TicketPath != "" {
		output.Info(fmt.Sprintf("   Ticket: %s", ticket.TicketPath))
	}
	if ticket.GitHub != nil && ticket.GitHub.Issue != "" {
		output.Info(fmt.Sprintf("   Issue:  %s", ticket.GitHub.Issue))
	}
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
		if t.GetStatus() == TicketStatusClosed {
			status = "✅"
		}
		output.Plain(fmt.Sprintf("   %s %d/%s/%s/%s", status, t.Year, PadNumber(t.Month, 2), PadNumber(t.Day, 2), t.Slug))
		output.Plain(fmt.Sprintf("      %s", t.GetTitle()))
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
	output.Plain(fmt.Sprintf("   Status: %s", ticket.GetStatus()))
	output.Plain(fmt.Sprintf("   Created: %s", ticket.GetDateStarted()))
	output.Plain(fmt.Sprintf("   Prompt: %s", ticket.GetPrompt()))
	if ticket.GetLLM() != "" {
		output.Plain(fmt.Sprintf("   LLM: %s", ticket.GetLLM()))
	}
	ticketContent, _ := ReadTextFile(ticket.TicketPath)
	if ticketContent != "" {
		output.Plain(fmt.Sprintf("\n%s", ticketContent))
	}
	return ToolResult{Output: *output, Data: ticket}
}

func ToolTicketClose(year, month, day int, slug, summary string, files []string, title string, noGithub bool) ToolResult {
	output := NewOutput()
	ticket, err := ReadTicket(year, month, day, slug)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if title != "" {
		if err := UpdateTicketTitle(ticket, title); err != nil {
			output.Error(fmt.Sprintf("Error: %v", err))
			return ToolResult{Output: *output, Error: err.Error()}
		}
		if ticket.GitHub != nil && ticket.GitHub.Issue != "" && !noGithub {
			if err := ghUpdateIssueTitle(ticket.GitHub.Issue, title); err != nil {
				fmt.Printf("Warning: Failed to update GitHub issue title: %v\n", err)
			}
		}
	}
	if err := FinishTicket(ticket, summary, files, noGithub, false); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	filesResult := ticket.GetFiles()
	filesCount := 0
	sectionsCount := 0
	defsCount := 0
	if filesResult != nil {
		filesCount = len(filesResult.Files.Modified) + len(filesResult.Files.Added) + len(filesResult.Files.Renamed) + len(filesResult.Files.Deleted)
		sectionsCount = len(filesResult.Sections.Modified) + len(filesResult.Sections.Added) + len(filesResult.Sections.Renamed) + len(filesResult.Sections.Deleted)
		defsCount = len(filesResult.Definitions.Modified) + len(filesResult.Definitions.Added) + len(filesResult.Definitions.Renamed) + len(filesResult.Definitions.Deleted)
	}
	output.Success(fmt.Sprintf("\n✅ Ticket finished: %s", ticket.Slug))
	output.Info(fmt.Sprintf("   Summary: %s", summary))
	output.Info(fmt.Sprintf("   Files: %d", filesCount))
	output.Info(fmt.Sprintf("   Sections affected: %d", sectionsCount))
	output.Info(fmt.Sprintf("   Definitions affected: %d", defsCount))
	return ToolResult{Output: *output, Data: ticket}
}

func ToolTicketReopen(year, month, day int, slug, prompt, llm, ui, draft string, title string, goal string, parent string, noGithub bool) ToolResult {
	output := NewOutput()
	ticket, err := ReadTicket(year, month, day, slug)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if title != "" {
		if err := UpdateTicketTitle(ticket, title); err != nil {
			output.Error(fmt.Sprintf("Error: %v", err))
			return ToolResult{Output: *output, Error: err.Error()}
		}
		if ticket.GitHub != nil && ticket.GitHub.Issue != "" && !noGithub {
			if err := ghUpdateIssueTitle(ticket.GitHub.Issue, title); err != nil {
				fmt.Printf("Warning: Failed to update GitHub issue title: %v\n", err)
			}
		}
	}
	if err := ReopenTicket(ticket, prompt, llm, ui, draft, goal, parent, noGithub); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🔓 Ticket reopened: %s", ticket.Slug))
	return ToolResult{Output: *output, Data: ticket}
}

func ToolDraftCreate(slug string, files []string) ToolResult {
	output := NewOutput()
	draft, err := CreateDraft(slug, files)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n📝 Created draft: %s", draft.ID))
	return ToolResult{Output: *output, Data: draft}
}

func ToolDraftList() ToolResult {
	output := NewOutput()
	drafts, err := ListDrafts()
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Info(fmt.Sprintf("\n📝 Found %d drafts:\n", len(drafts)))
	for _, d := range drafts {
		output.Plain(fmt.Sprintf("   %s", d.ID))
	}
	return ToolResult{Output: *output, Data: drafts}
}

func ToolDraftDelete(slug string) ToolResult {
	output := NewOutput()
	if err := DeleteDraft(slug); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🗑️ Deleted draft: %s", slug))
	return ToolResult{Output: *output}
}

func ToolGoalCreate(title, description, prompt, dueDate, llm, ui string, noGithub bool, parent, milestone string) ToolResult {
	output := NewOutput()
	ctx := NewRepoContext(rootDir)
	goal, err := ctx.GoalCreate(GoalCreateInput{
		Title:       title,
		Description: description,
		Prompt:      prompt,
		DueDate:     dueDate,
		LLM:         llm,
		UI:          ui,
		NoGithub:    noGithub,
		Parent:      parent,
		Milestone:   milestone,
	})
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🎯 Created goal: %s", goal.ID))
	output.Info(fmt.Sprintf("   Title: %s", goal.Title))
	output.Info(fmt.Sprintf("   Due: %s", goal.Dates.Due))
	if goal.GitHub != nil && goal.GitHub.Milestone != "" {
		output.Info(fmt.Sprintf("   Milestone: %s", goal.GitHub.Milestone))
	}
	return ToolResult{Output: *output, Data: goal}
}

func ToolGoalList() ToolResult {
	output := NewOutput()
	goals, err := ListGoals()
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Info(fmt.Sprintf("\n🎯 Found %d goals:\n", len(goals)))
	for _, g := range goals {
		status := "🟢"
		if g.Status == "closed" {
			status = "✅"
		}
		output.Plain(fmt.Sprintf("   %s %s", status, g.ID))
		output.Plain(fmt.Sprintf("      %s", g.Title))
		if g.Dates.Due != "" {
			output.Plain(fmt.Sprintf("      Due: %s", g.Dates.Due))
		}
	}
	return ToolResult{Output: *output, Data: goals}
}

func ToolGoalClose(id, summary string, noGithub bool) ToolResult {
	output := NewOutput()
	ctx := NewRepoContext(rootDir)
	res, err := ctx.GoalClose(GoalCloseInput{
		ID:       id,
		Summary:  summary,
		NoGithub: noGithub,
	})
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n✅ Closed goal: %s", res.ID))
	return ToolResult{Output: *output}
}

func ToolGoalReopen(id, prompt, llm, ui, title, description, dueDate string, noGithub bool) ToolResult {
	output := NewOutput()
	ctx := NewRepoContext(rootDir)
	var titlePtr, descriptionPtr, dueDatePtr *string
	if title != "" {
		titlePtr = &title
	}
	if description != "" {
		descriptionPtr = &description
	}
	if dueDate != "" {
		dueDatePtr = &dueDate
	}
	res, err := ctx.GoalReopen(GoalReopenInput{
		ID:          id,
		Prompt:      prompt,
		LLM:         llm,
		UI:          ui,
		Title:       titlePtr,
		Description: descriptionPtr,
		DueDate:     dueDatePtr,
		NoGithub:    noGithub,
	})
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🔓 Reopened goal: %s", res.ID))
	return ToolResult{Output: *output}
}

func ToolContributorAdd(github string) ToolResult {
	output := NewOutput()
	contributor, err := CreateContributor(github)
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n👤 Added contributor: %s", contributor.Github))
	output.Info(fmt.Sprintf("   Path: %s", GetContributorPath(github)))
	return ToolResult{Output: *output, Data: contributor}
}

func ToolContributorList() ToolResult {
	output := NewOutput()
	contributors, err := ListContributors()
	if err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Info(fmt.Sprintf("\n👥 Found %d contributors:\n", len(contributors)))
	for _, c := range contributors {
		name := c.Name
		if name == "" {
			name = c.Github
		}
		output.Plain(fmt.Sprintf("   %s (@%s)", name, c.Github))
		ticketCount := len(c.Contributions.Tickets)
		if ticketCount > 0 {
			output.Plain(fmt.Sprintf("      Tickets: %d", ticketCount))
		}
		projectCount := len(c.Contributions.Bundles)
		if projectCount > 0 {
			output.Plain(fmt.Sprintf("      Bundles: %d", projectCount))
		}
		fileCount := len(c.Contributions.Files)
		if fileCount > 0 {
			output.Plain(fmt.Sprintf("      Files: %d", fileCount))
		}
		commitCount := len(c.Contributions.Commits)
		if commitCount > 0 {
			output.Plain(fmt.Sprintf("      Commits: %d", commitCount))
		}
		if c.Contributions.Lines != nil {
			output.Plain(fmt.Sprintf("      Lines: +%d -%d", c.Contributions.Lines.Added, c.Contributions.Lines.Removed))
		}
	}
	return ToolResult{Output: *output, Data: contributors}
}

func ToolContributorRemove(github string) ToolResult {
	output := NewOutput()
	if err := RemoveContributor(github); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🗑️ Removed contributor: %s", github))
	return ToolResult{Output: *output}
}

func ToolProjectList() ToolResult {
	output := NewOutput()
	bundles := GetProjects()
	output.Info(fmt.Sprintf("\n📦 Found %d bundles:\n", len(bundles)))
	for _, p := range bundles {
		output.Plain(fmt.Sprintf("   %s", p.Name))
		output.Plain(fmt.Sprintf("      Root: %s", p.Root))
		if len(p.Tags) > 0 {
			output.Plain(fmt.Sprintf("      Tags: %s", strings.Join(p.Tags, ", ")))
		}
	}
	return ToolResult{Output: *output, Data: bundles}
}

func ToolProjectTree() ToolResult {
	output := NewOutput()
	bundles := GetProjects()
	output.Info("\n📦 Bundle tree:\n")
	for _, p := range bundles {
		output.Plain(fmt.Sprintf("   └── %s (%s)", p.Name, p.Root))
	}
	return ToolResult{Output: *output, Data: bundles}
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
	language := GetLanguage(path)
	content := generateFileHeader(path, language)
	if err := WriteTextFile(absPath, content); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n📄 Created file: %s", path))
	return ToolResult{Output: *output}
}

func generateFileHeader(path string, language LanguagePlugin) string {
	if language == nil || !language.SupportsHeaders() {
		return ""
	}
	gitAuthor := GetGitAuthor()
	year := strconv.Itoa(time.Now().Year())
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
	return language.FormatHeader(path, year, gitAuthor, formatLicenseLines(license, language.CommentPrefix()))
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
	bundles := GetProjects()
	files, err := ScopeToFiles(scope, bundles)
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
	language := GetLanguage(filePath)
	if language != nil && language.Name() == "json" {
		pathParts := NormalizeSectionPath(sectionPath)
		if len(pathParts) == 0 {
			output.Error("Error: Section path required")
			return ToolResult{Output: *output, Error: "section path required"}
		}
		sectionName = pathParts[len(pathParts)-1]
		parentPath := strings.Join(pathParts[:len(pathParts)-1], "/")
		_, locations, err := ParseJSONSectionsDetailed(content)
		if err != nil {
			output.Error(fmt.Sprintf("Error: %v", err))
			return ToolResult{Output: *output, Error: err.Error()}
		}
		targetPath := strings.Join(pathParts, "/")
		if _, exists := locations[targetPath]; exists {
			output.Error(fmt.Sprintf("Error: Section already exists: %s", targetPath))
			return ToolResult{Output: *output, Error: "section exists"}
		}
		objectStart, objectEnd, ok := jsonFindObjectRange(content, locations, parentPath)
		if !ok {
			output.Error("Error: Parent section is not a JSON object")
			return ToolResult{Output: *output, Error: "parent not object"}
		}
		entry := fmt.Sprintf("%s: {}", strconv.Quote(sectionName))
		updated, inserted := jsonInsertEntry(content, objectStart, objectEnd, entry)
		if !inserted {
			output.Error("Error: Failed to insert section")
			return ToolResult{Output: *output, Error: "insert failed"}
		}
		if err := WriteTextFile(absPath, updated); err != nil {
			output.Error(fmt.Sprintf("Error: %v", err))
			return ToolResult{Output: *output, Error: err.Error()}
		}
		output.Success(fmt.Sprintf("\n🏷️ Created section \"%s\" in %s", sectionName, filePath))
		return ToolResult{Output: *output}
	}
	if language == nil || !language.SupportsSections() {
		output.Error("Error: Unsupported file type")
		return ToolResult{Output: *output, Error: "unsupported file type"}
	}
	newSection := language.FormatSectionBoth(sectionName)
	if newSection == "" {
		output.Error("Error: Cannot create section for this file type")
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
	language := GetLanguage(filePath)
	if language != nil && language.Name() == "json" {
		oldParts = NormalizeSectionPath(oldPath)
		newParts = NormalizeSectionPath(newPath)
		if len(oldParts) == 0 || len(newParts) == 0 {
			output.Error("Error: Section path required")
			return ToolResult{Output: *output, Error: "section path required"}
		}
		oldPathNormalized := strings.Join(oldParts, "/")
		newPathNormalized := strings.Join(newParts, "/")
		_, locations, err := ParseJSONSectionsDetailed(content)
		if err != nil {
			output.Error(fmt.Sprintf("Error: %v", err))
			return ToolResult{Output: *output, Error: err.Error()}
		}
		source, ok := locations[oldPathNormalized]
		if !ok {
			output.Error(fmt.Sprintf("Error: Section not found: %s", oldPathNormalized))
			return ToolResult{Output: *output, Error: "section not found"}
		}
		entry, start, end := jsonExtractEntry(content, source.KeyStart, source.ValueEnd)
		updated := content[:start] + content[end:]
		_, updatedLocations, err := ParseJSONSectionsDetailed(updated)
		if err != nil {
			output.Error(fmt.Sprintf("Error: %v", err))
			return ToolResult{Output: *output, Error: err.Error()}
		}
		newName = newParts[len(newParts)-1]
		entry = jsonRenameEntryKey(entry, newName)
		parentPath := strings.Join(newParts[:len(newParts)-1], "/")
		objectStart, objectEnd, ok := jsonFindObjectRange(updated, updatedLocations, parentPath)
		if !ok {
			output.Error("Error: Target section is not a JSON object")
			return ToolResult{Output: *output, Error: "target not object"}
		}
		entry = jsonReindentEntry(entry, "")
		finalContent, inserted := jsonInsertEntry(updated, objectStart, objectEnd, entry)
		if !inserted {
			output.Error("Error: Failed to move section")
			return ToolResult{Output: *output, Error: "move failed"}
		}
		if err := WriteTextFile(absPath, finalContent); err != nil {
			output.Error(fmt.Sprintf("Error: %v", err))
			return ToolResult{Output: *output, Error: err.Error()}
		}
		output.Success(fmt.Sprintf("\n🏷️ Renamed section \"%s\" to \"%s\" in %s", oldPathNormalized, newPathNormalized, filePath))
		return ToolResult{Output: *output}
	}
	if language != nil && language.SupportsSections() {
		oldStart := language.FormatSectionStart(oldName)
		newStart := language.FormatSectionStart(newName)
		if oldStart != "" && newStart != "" {
			content = strings.ReplaceAll(content, oldStart, newStart)
		}
		oldEnd := language.FormatSectionEnd(oldName)
		newEnd := language.FormatSectionEnd(newName)
		if oldEnd != "" && newEnd != "" {
			content = strings.ReplaceAll(content, oldEnd, newEnd)
		}
		if language.Name() == "markdown" {
			content = strings.ReplaceAll(content, "# "+oldName, "# "+newName)
		}
	}
	if err := WriteTextFile(absPath, content); err != nil {
		output.Error(fmt.Sprintf("Error: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("\n🏷️ Renamed section \"%s\" to \"%s\" in %s", oldName, newName, filePath))
	return ToolResult{Output: *output}
}

func ToolIntegrate(sourcePath, targetSectionName, targetFilePath, targetParentSectionName string) ToolResult {
	output := NewOutput()

	// 1. Read source content
	absSourcePath := filepath.Join(rootDir, sourcePath)
	if !FileExists(absSourcePath) {
		output.Error(fmt.Sprintf("Error: Source file not found: %s", sourcePath))
		return ToolResult{Output: *output, Error: "source file not found"}
	}
	sourceContent, err := ReadTextFile(absSourcePath)
	if err != nil {
		output.Error(fmt.Sprintf("Error reading source file: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}

	// 2. Read target file
	absTargetFilePath := filepath.Join(rootDir, targetFilePath)
	if !FileExists(absTargetFilePath) {
		output.Error(fmt.Sprintf("Error: Target file not found: %s", targetFilePath))
		return ToolResult{Output: *output, Error: "target file not found"}
	}
	targetContent, err := ReadTextFile(absTargetFilePath)
	if err != nil {
		output.Error(fmt.Sprintf("Error reading target file: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}

	// 3. Get target language
	targetLanguage := GetLanguage(targetFilePath)
	if targetLanguage == nil || !targetLanguage.SupportsSections() {
		output.Error("Error: Target file type does not support sections")
		return ToolResult{Output: *output, Error: "unsupported target file type"}
	}

	output.Info("Splitting headers...")

	// 4. Split Headers
	sourceHeader, sourceBody := SplitHeader(sourceContent, targetLanguage)
	targetHeader, targetBody := SplitHeader(targetContent, targetLanguage)

	output.Info(fmt.Sprintf("Source Header len: %d, Source Body len: %d", len(sourceHeader), len(sourceBody)))

	// 5. Extract Imports
	_, sourceBodyNoPkg := targetLanguage.ExtractPackage(sourceBody)
	targetPkg, targetBodyNoPkg := targetLanguage.ExtractPackage(targetBody)

	sourceImports, sourceCode := targetLanguage.ExtractImports(sourceBodyNoPkg)
	targetImports, targetCode := targetLanguage.ExtractImports(targetBodyNoPkg)

	// 6. Merge Headers
	mergedHeader := MergeHeaders(targetHeader, sourceHeader, targetLanguage)

	// 7. Merge Imports
	mergedImports := UniqueStrings(append(targetImports, sourceImports...))

	// 8. Format the new section with source content
	startMarker := targetLanguage.FormatSectionStart(targetSectionName)
	endMarker := targetLanguage.FormatSectionEnd(targetSectionName)

	// Ensure content ends with newline if it doesn't
	if !strings.HasSuffix(sourceCode, "\n") && sourceCode != "" {
		sourceCode += "\n"
	}

	sectionContent := "\n" + startMarker + "\n" + sourceCode + endMarker + "\n"

	// 9. Handle insertion
	var updatedBody string
	if targetParentSectionName != "" {
		// Find parent section
		sections := targetLanguage.ParseSections(targetCode)
		parentSection := FindSection(sections, targetParentSectionName)
		if parentSection == nil {
			output.Error(fmt.Sprintf("Error: Parent section not found: %s", targetParentSectionName))
			return ToolResult{Output: *output, Error: "parent section not found"}
		}

		// Insert at the end of parent section (before parent's end marker)
		if parentSection.EndLine == -1 {
			output.Error(fmt.Sprintf("Error: Parent section %s is not properly closed", targetParentSectionName))
			return ToolResult{Output: *output, Error: "parent section not closed"}
		}

		// Insert before the end marker line of the parent section
		lines := strings.Split(targetCode, "\n")
		newLines := make([]string, 0, len(lines)+strings.Count(sectionContent, "\n"))
		newLines = append(newLines, lines[:parentSection.EndLine-1]...)
		newLines = append(newLines, strings.Split(strings.Trim(sectionContent, "\n"), "\n")...)
		newLines = append(newLines, lines[parentSection.EndLine-1:]...)
		updatedBody = strings.Join(newLines, "\n")
	} else {
		// Append to the end of the target file
		updatedBody = targetCode
		if !strings.HasSuffix(updatedBody, "\n") && updatedBody != "" {
			updatedBody += "\n"
		}
		updatedBody += sectionContent
	}

	// 10. Reassemble
	finalContent := mergedHeader
	if finalContent != "" {
		if !strings.HasSuffix(finalContent, "\n") {
			finalContent += "\n"
		}
		finalContent += "\n"
	}

	if targetPkg != "" {
		finalContent += targetPkg + "\n\n"
	}

	formattedImports := targetLanguage.FormatImports(mergedImports)
	if formattedImports != "" {
		finalContent += formattedImports + "\n\n"
	}

	finalContent += updatedBody

	// 12. Write target file
	if err := WriteTextFile(absTargetFilePath, finalContent); err != nil {
		output.Error(fmt.Sprintf("Error writing target file: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}

	output.Success(fmt.Sprintf("\n🧩 Integrated %s into %s section of %s", sourcePath, targetSectionName, targetFilePath))
	return ToolResult{Output: *output}
}

func ToolExtract(sourceFilePath, sourceSectionName, targetFilePath string) ToolResult {
	output := NewOutput()

	// 1. Read source content
	absSourcePath := filepath.Join(rootDir, sourceFilePath)
	if !FileExists(absSourcePath) {
		output.Error(fmt.Sprintf("Error: Source file not found: %s", sourceFilePath))
		return ToolResult{Output: *output, Error: "source file not found"}
	}
	sourceContent, err := ReadTextFile(absSourcePath)
	if err != nil {
		output.Error(fmt.Sprintf("Error reading source file: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}

	// 2. Language
	sourceLanguage := GetLanguage(sourceFilePath)
	if sourceLanguage == nil || !sourceLanguage.SupportsSections() {
		output.Error("Error: Source file type does not support sections")
		return ToolResult{Output: *output, Error: "unsupported source file type"}
	}

	// 3. Parse and Find Section
	sections := sourceLanguage.ParseSections(sourceContent)
	section := FindSection(sections, sourceSectionName)
	if section == nil {
		output.Error(fmt.Sprintf("Error: Section not found: %s", sourceSectionName))
		return ToolResult{Output: *output, Error: "section not found"}
	}

	// 4. Extract Content
	lines := strings.Split(sourceContent, "\n")
	if section.StartLine > len(lines) || section.EndLine > len(lines) {
		output.Error("Error: Section range invalid")
		return ToolResult{Output: *output, Error: "invalid section range"}
	}

	// Extract lines inside markers (StartLine+1 to EndLine-1 in 1-based logic)
	// Indices: StartLine (inclusive) to EndLine-1 (exclusive)
	var extractedLines []string
	if section.EndLine > section.StartLine {
		extractedLines = lines[section.StartLine : section.EndLine-1]
	}
	extractedBody := strings.Join(extractedLines, "\n")

	// 5. Build Target Content
	header, sourceBody := SplitHeader(sourceContent, sourceLanguage)
	_, sourceBodyNoPkg := sourceLanguage.ExtractPackage(sourceBody)
	imports, _ := sourceLanguage.ExtractImports(sourceBodyNoPkg)

	targetContent := ""
	if header != "" {
		targetContent += header + "\n\n"
	}
	// Try to get package from source (simple heuristic)
	pkgDecl, _ := sourceLanguage.ExtractPackage(sourceBody)
	if pkgDecl != "" {
		targetContent += pkgDecl + "\n\n"
	} else if len(imports) > 0 {
		// If no package but imports, ensure space? Header usually has space.
	}

	if len(imports) > 0 {
		formattedImports := sourceLanguage.FormatImports(imports)
		if formattedImports != "" {
			targetContent += formattedImports + "\n\n"
		}
	}

	targetContent += extractedBody
	if !strings.HasSuffix(targetContent, "\n") {
		targetContent += "\n"
	}

	// 6. Write Target File
	absTargetFilePath := filepath.Join(rootDir, targetFilePath)
	if err := os.MkdirAll(filepath.Dir(absTargetFilePath), 0755); err != nil {
		output.Error(fmt.Sprintf("Error creating target directory: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	if err := WriteTextFile(absTargetFilePath, targetContent); err != nil {
		output.Error(fmt.Sprintf("Error writing target file: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}

	// 7. Remove Section from Source
	// Lines to keep: 0 to StartLine-2 (index StartLine-1 is splice point) -> 0 to StartLine-1 exclusive
	// And: EndLine to end.
	// Index: StartLine-1 (inclusive) is first line to remove.
	// Index: EndLine (inclusive) is first line to keep after. (Because EndLine 1-based is end marker line).
	// Wait, EndLine index is EndLine-1. We remove that too. So we keep from EndLine index.
	// Go slice [low:high] -> low inclusive, high exclusive.
	// Keep lines[:StartLine-1] (indexes 0 to StartLine-2)
	// Keep lines[EndLine:] (indexes EndLine to ...)
	var newSourceLines []string
	if section.StartLine > 0 {
		newSourceLines = append(newSourceLines, lines[:section.StartLine-1]...)
	}
	if section.EndLine < len(lines) {
		newSourceLines = append(newSourceLines, lines[section.EndLine:]...)
	}
	newSourceContent := strings.Join(newSourceLines, "\n")

	if err := WriteTextFile(absSourcePath, newSourceContent); err != nil {
		output.Error(fmt.Sprintf("Error updating source file: %v", err))
		// Note: Target file was already written. Partial state?
		return ToolResult{Output: *output, Error: err.Error()}
	}

	output.Success(fmt.Sprintf("\n🧩 Extracted %s from %s to %s", sourceSectionName, sourceFilePath, targetFilePath))
	return ToolResult{Output: *output}
}

func SplitHeader(content string, lang LanguagePlugin) (string, string) {
	sections := lang.ParseSections(content)
	for _, s := range sections {
		if strings.EqualFold(s.Name, "Header") {
			header := content[:s.EndIndex]
			body := content[s.EndIndex:]
			return header, body
		}
	}
	return "", content
}

func MergeHeaders(targetHeader, sourceHeader string, lang LanguagePlugin) string {
	if targetHeader == "" {
		return sourceHeader
	}
	if sourceHeader == "" {
		return targetHeader
	}
	targetLines := strings.Split(targetHeader, "\n")
	sourceLines := strings.Split(sourceHeader, "\n")
	seen := make(map[string]bool)
	for _, line := range targetLines {
		seen[strings.TrimSpace(line)] = true
	}
	var insertIdx = -1
	for i, line := range targetLines {
		if matched, _ := lang.PolicySectionEndMatch(line); matched {
			insertIdx = i
		}
	}
	if insertIdx == -1 {
		return targetHeader
	}

	var newLines []string
	for _, line := range sourceLines {
		trimmed := strings.TrimSpace(line)
		if trimmed == "" {
			continue
		}
		if matched, _ := lang.PolicySectionStartMatch(line); matched {
			continue
		}
		if matched, _ := lang.PolicySectionEndMatch(line); matched {
			continue
		}
		if !seen[trimmed] {
			newLines = append(newLines, line)
		}
	}

	if len(newLines) == 0 {
		return targetHeader
	}

	res := make([]string, 0, len(targetLines)+len(newLines)+1)
	res = append(res, targetLines[:insertIdx]...)
	res = append(res, newLines...)
	res = append(res, targetLines[insertIdx:]...)
	return strings.Join(res, "\n")
}

func UniqueStrings(input []string) []string {
	keys := make(map[string]bool)
	list := []string{}
	for _, entry := range input {
		if _, value := keys[entry]; !value {
			keys[entry] = true
			list = append(list, entry)
		}
	}
	return list
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
	language := GetLanguage(filePath)
	if language != nil && language.Name() == "json" {
		pathParts := NormalizeSectionPath(sectionPath)
		if len(pathParts) == 0 {
			output.Error("Error: Section path required")
			return ToolResult{Output: *output, Error: "section path required"}
		}
		_, locations, err := ParseJSONSectionsDetailed(content)
		if err != nil {
			output.Error(fmt.Sprintf("Error: %v", err))
			return ToolResult{Output: *output, Error: err.Error()}
		}
		location, ok := locations[strings.Join(pathParts, "/")]
		if !ok {
			output.Error(fmt.Sprintf("Error: Section not found: %s", strings.Join(pathParts, "/")))
			return ToolResult{Output: *output, Error: "section not found"}
		}
		_, start, end := jsonExtractEntry(content, location.KeyStart, location.ValueEnd)
		updated := content[:start] + content[end:]
		if err := WriteTextFile(absPath, updated); err != nil {
			output.Error(fmt.Sprintf("Error: %v", err))
			return ToolResult{Output: *output, Error: err.Error()}
		}
		output.Success(fmt.Sprintf("\n🗑️ Deleted section \"%s\" from %s", strings.Join(pathParts, "/"), filePath))
		return ToolResult{Output: *output}
	}
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
	var printSection func(s Section, indent string)
	printSection = func(s Section, indent string) {
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
	var printSection func(s Section, prefix string)
	printSection = func(s Section, prefix string) {
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
	return ToolResult{Output: *output, Data: []Definition{}}
}

func ToolDefinitionTree(filePath string) ToolResult {
	return ToolDefinitionList(filePath)
}

func ToolUpdateMetabolism() ToolResult {
	output := NewOutput()
	output.Info("\n🔄 Running update-metabolism via npx tsx...")
	stdout, stderr, exitCode := ExecCommand("npx", []string{"tsx", "scripts/update-metabolism.tsx"}, "")
	if exitCode != 0 {
		output.Error(fmt.Sprintf("Error: %s%s", stdout, stderr))
		return ToolResult{Output: *output, Error: "update-metabolism failed"}
	}
	output.Success(stdout)
	return ToolResult{Output: *output}
}

// #region SQLite Export

type ExportResult struct {
	Path           string `json:"path"`
	Bundles        int    `json:"bundles"`
	Folders        int    `json:"folders"`
	Files          int    `json:"files"`
	Sections       int    `json:"sections"`
	Definitions    int    `json:"definitions"`
	Contributors   int    `json:"contributors"`
	Tickets        int    `json:"tickets"`
	Policies       int    `json:"policies"`
	ViolationKinds int    `json:"violationKinds"`
	Violations     int    `json:"violations"`
}

func ExportToSQLite(outputPath string, ctx RepoContext) (*ExportResult, error) {
	if outputPath == "" {
		outputPath = filepath.Join(ctx.GetRootDir(), "repo.db")
	}
	if err := os.Remove(outputPath); err != nil && !os.IsNotExist(err) {
		return nil, fmt.Errorf("failed to remove existing database: %w", err)
	}
	db, err := sql.Open("sqlite", outputPath)
	if err != nil {
		return nil, fmt.Errorf("failed to open database: %w", err)
	}
	defer db.Close()
	schemaPath := filepath.Join(ctx.GetRootDir(), "sql", "sqlite", "repo", "schema.sql")
	schemaBytes, err := os.ReadFile(schemaPath)
	if err != nil {
		return nil, fmt.Errorf("failed to read schema file: %w", err)
	}
	if _, err := db.Exec(string(schemaBytes)); err != nil {
		return nil, fmt.Errorf("failed to execute schema: %w", err)
	}
	tx, err := db.Begin()
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback()
	result := &ExportResult{Path: outputPath}
	if err := exportRepo(tx, ctx); err != nil {
		return nil, fmt.Errorf("failed to export repo: %w", err)
	}
	if result.Bundles, err = exportBundles(tx, ctx); err != nil {
		return nil, fmt.Errorf("failed to export bundles: %w", err)
	}
	if result.Folders, err = exportFolders(tx, ctx); err != nil {
		return nil, fmt.Errorf("failed to export folders: %w", err)
	}
	if result.Files, result.Sections, result.Definitions, err = exportFiles(tx, ctx); err != nil {
		return nil, fmt.Errorf("failed to export files: %w", err)
	}
	if result.Contributors, err = exportContributors(tx, ctx); err != nil {
		return nil, fmt.Errorf("failed to export contributors: %w", err)
	}
	if result.Tickets, err = exportTickets(tx, ctx); err != nil {
		return nil, fmt.Errorf("failed to export tickets: %w", err)
	}
	if result.Policies, result.ViolationKinds, err = exportPolicies(tx, ctx); err != nil {
		return nil, fmt.Errorf("failed to export policies: %w", err)
	}
	analyzeResult, err := ctx.Analyze(nil)
	if err != nil {
		return nil, fmt.Errorf("failed to analyze: %w", err)
	}
	if result.Violations, err = exportViolations(tx, analyzeResult.Violations); err != nil {
		return nil, fmt.Errorf("failed to export violations: %w", err)
	}
	if err := tx.Commit(); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}
	return result, nil
}

func exportRepo(tx *sql.Tx, ctx RepoContext) error {
	_, err := tx.Exec(`INSERT INTO repo (id, name, path, exported_at) VALUES (?, ?, ?, ?)`,
		"@semio-repo/repo",
		"semio",
		ctx.GetRootDir(),
		time.Now().UTC().Format(time.RFC3339))
	return err
}

func exportBundles(tx *sql.Tx, ctx RepoContext) (int, error) {
	bundles := ctx.GetBundles()
	stmt, err := tx.Prepare(`INSERT INTO bundle (id, name, root, source_root, project_type, uri) VALUES (?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, err
	}
	defer stmt.Close()
	tagStmt, err := tx.Prepare(`INSERT INTO bundle_tag (bundle_id, tag) VALUES (?, ?)`)
	if err != nil {
		return 0, err
	}
	defer tagStmt.Close()

	for _, b := range bundles {
		id := b.GetID()
		uri := "file://" + NormalizePath(filepath.Join(ctx.GetRootDir(), b.Root))
		var sourceRoot, projectType interface{}
		if b.SourceRoot != "" {
			sourceRoot = b.SourceRoot
		}
		// ProjectType was removed
		if _, err := stmt.Exec(id, b.Name, b.Root, sourceRoot, projectType, uri); err != nil {
			return 0, err
		}
		for _, tag := range b.Tags {
			if _, err := tagStmt.Exec(id, tag); err != nil {
				return 0, err
			}
		}
	}
	return len(bundles), nil
}

func exportFolders(tx *sql.Tx, ctx RepoContext) (int, error) {
	folders := ctx.GetFolders()
	stmt, err := tx.Prepare(`INSERT INTO folder (id, path, uri, name, parent_id, bundle_id) VALUES (?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, err
	}
	defer stmt.Close()
	for _, f := range folders {
		if _, err := stmt.Exec(f.ID, f.Path, f.URI, f.Name, f.ParentID, f.BundleID); err != nil {
			return 0, err
		}
	}
	return len(folders), nil
}

func exportFiles(tx *sql.Tx, ctx RepoContext) (int, int, int, error) {
	files := ctx.GetFiles()
	fileStmt, err := tx.Prepare(`INSERT INTO file (id, path, uri, name, extension, folder_id, bundle_id, lines) VALUES (?, ?, ?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, 0, 0, err
	}
	defer fileStmt.Close()
	sectionStmt, err := tx.Prepare(`INSERT INTO section (id, name, path, file_id, parent_id, start_line, end_line, start_column, end_column) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, 0, 0, err
	}
	defer sectionStmt.Close()
	defStmt, err := tx.Prepare(`INSERT INTO definition (id, name, kind, file_id, section_id, start_line, end_line, start_column, end_column) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, 0, 0, err
	}
	defer defStmt.Close()

	totalSections := 0
	totalDefs := 0
	for _, f := range files {
		absPath := filepath.Join(ctx.GetRootDir(), f.Path)
		lines := 0
		if content, err := ReadTextFile(absPath); err == nil {
			lines = strings.Count(content, "\n") + 1
		}
		if _, err := fileStmt.Exec(f.ID, f.Path, f.URI, f.Name, f.Extension, f.FolderID, f.BundleID, lines); err != nil {
			return 0, 0, 0, err
		}
		if content, err := ReadTextFile(absPath); err == nil {
			sections := ParseSections(content, f.Path)
			sectionCount, err := exportSectionsRecursive(sectionStmt, sections, f.ID, f.Path, nil)
			if err != nil {
				return 0, 0, 0, err
			}
			totalSections += sectionCount

			// Export definitions
			lang := GetLanguage(f.Path)
			if lang != nil && lang.SupportsDefinitions() {
				lines := strings.Split(content, "\n")
				defs := lang.ParseDefinitions(content, lines)
				for _, d := range defs {
					sectionPath := findSectionForDefinition(sections, d.Start, d.End, "")
					var sectionID interface{}
					id := f.ID + "§" + d.Name
					if sectionPath != "" {
						sid := f.ID + "#" + sectionPath
						sectionID = sid
						id = sid + "§" + d.Name
					}
					// TODO: Get actual kind from LanguagePlugin
					kind := "variable"
					if _, err := defStmt.Exec(id, d.Name, kind, f.ID, sectionID, d.Start, d.End, 0, 0); err != nil {
						return 0, 0, 0, err
					}
					totalDefs++
				}
			}
		}
	}
	return len(files), totalSections, totalDefs, nil
}

func exportSectionsRecursive(sectionStmt *sql.Stmt, sections []Section, fileID, filePath string, parentID *string) (int, error) {
	count := 0
	for _, s := range sections {
		sectionPath := s.Name
		if parentID != nil {
			// Extract section path from parentID
			parentPath := strings.SplitN(*parentID, "#", 2)[1]
			sectionPath = parentPath + "#" + s.Name
		}
		sectionID := fileID + "#" + sectionPath

		if _, err := sectionStmt.Exec(sectionID, s.Name, sectionPath, fileID, parentID, s.StartLine, s.EndLine, 0, 0); err != nil {
			return 0, err
		}
		count++
		if len(s.Children) > 0 {
			childCount, err := exportSectionsRecursive(sectionStmt, s.Children, fileID, filePath, &sectionID)
			if err != nil {
				return 0, err
			}
			count += childCount
		}
	}
	return count, nil
}

func exportContributors(tx *sql.Tx, ctx RepoContext) (int, error) {
	contributors, err := ctx.GetContributors()
	if err != nil {
		return 0, err
	}
	stmt, err := tx.Prepare(`INSERT INTO contributor (id, github, name, avatar_url, avatar_round_url, github_icon_url) VALUES (?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, err
	}
	defer stmt.Close()
	emailStmt, err := tx.Prepare(`INSERT INTO contributor_email (contributor_id, email) VALUES (?, ?)`)
	if err != nil {
		return 0, err
	}
	defer emailStmt.Close()
	linkStmt, err := tx.Prepare(`INSERT INTO contributor_link (contributor_id, name, url) VALUES (?, ?, ?)`)
	if err != nil {
		return 0, err
	}
	defer linkStmt.Close()
	for _, c := range contributors {
		id := c.GetID()
		var name interface{}
		if c.Name != "" {
			name = c.Name
		}
		if _, err := stmt.Exec(id, c.Github, name, nil, nil, nil); err != nil {
			return 0, err
		}
		for _, email := range c.Emails {
			if _, err := emailStmt.Exec(id, email); err != nil {
				return 0, err
			}
		}
		for linkName, url := range c.Links {
			if _, err := linkStmt.Exec(id, linkName, url); err != nil {
				return 0, err
			}
		}
	}
	return len(contributors), nil
}

func exportTickets(tx *sql.Tx, ctx RepoContext) (int, error) {
	tickets, err := ctx.GetTickets(nil, nil, nil, nil)
	if err != nil {
		return 0, err
	}
	ticketStmt, err := tx.Prepare(`INSERT INTO ticket (id, year, month, day, slug, title, path, uri, prompt, summary, status, author_id, llm, ui, commit_sha, created_at, finished_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, err
	}
	defer ticketStmt.Close()
	ticketFileStmt, err := tx.Prepare(`INSERT INTO ticket_file (ticket_id, file_path) VALUES (?, ?)`)
	if err != nil {
		return 0, err
	}
	defer ticketFileStmt.Close()
	for _, t := range tickets {
		ticketID := t.GetID()
		uri := "file://" + NormalizePath(t.FolderPath)
		status := string(t.GetStatus())
		if status == "" {
			status = "open"
		}
		var authorID, llm, ui, summary, commit, finishedAt interface{}
		if author := t.GetAuthor(); author != "" {
			authorID = "@semio-repo/contributor/" + author
		}
		if val := t.GetLLM(); val != "" {
			llm = val
		}
		if val := t.GetUI(); val != "" {
			ui = val
		}
		if val := t.GetCommit(); val != "" {
			commit = val
		}
		if s := t.GetSummary(); s != "" {
			summary = s
		}
		createdAtTime := t.GetDateStarted()
		var createdAt string
		if createdAtTime.IsZero() {
			createdAt = time.Now().UTC().Format(time.RFC3339)
		} else {
			createdAt = createdAtTime.Format(time.RFC3339)
		}
		if f := t.GetDateFinished(); f != nil {
			finishedAt = f.Format(time.RFC3339)
		}
		if _, err := ticketStmt.Exec(ticketID, t.Year, t.Month, t.Day, t.Slug, t.GetTitle(), t.FolderPath, uri, t.GetPrompt(), summary, status, authorID, llm, ui, commit, createdAt, finishedAt); err != nil {
			return 0, err
		}
		fileDiffs := t.GetFiles().Files
		for _, entry := range fileDiffs.Modified {
			if _, err := ticketFileStmt.Exec(ticketID, entry.Path); err != nil {
				return 0, err
			}
		}
		for _, entry := range fileDiffs.Added {
			if _, err := ticketFileStmt.Exec(ticketID, entry.Path); err != nil {
				return 0, err
			}
		}
		for _, entry := range fileDiffs.Deleted {
			if _, err := ticketFileStmt.Exec(ticketID, entry.Path); err != nil {
				return 0, err
			}
		}
		for _, entry := range fileDiffs.Renamed {
			if _, err := ticketFileStmt.Exec(ticketID, entry.To); err != nil {
				return 0, err
			}
		}
	}
	return len(tickets), nil
}

func exportPolicies(tx *sql.Tx, ctx RepoContext) (int, int, error) {
	policies := ctx.GetPolicies()
	policyStmt, err := tx.Prepare(`INSERT INTO policy (id, name, description) VALUES (?, ?, ?)`)
	if err != nil {
		return 0, 0, err
	}
	defer policyStmt.Close()
	scopeStmt, err := tx.Prepare(`INSERT INTO policy_scope (policy_id, scope) VALUES (?, ?)`)
	if err != nil {
		return 0, 0, err
	}
	defer scopeStmt.Close()
	kindStmt, err := tx.Prepare(`INSERT INTO violation_kind (id, policy_id, priority, autofixable, reason, solution) VALUES (?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, 0, err
	}
	defer kindStmt.Close()
	totalKinds := 0
	for _, p := range policies {
		policyID := p.GetID()
		var desc interface{}
		if p.Description != nil {
			desc = *p.Description
		}
		if _, err := policyStmt.Exec(policyID, p.Name, desc); err != nil {
			return 0, 0, err
		}
		for _, scope := range p.Scopes {
			if _, err := scopeStmt.Exec(policyID, scope); err != nil {
				return 0, 0, err
			}
		}
		for _, vk := range p.ViolationKinds {
			kindID := vk.GetID()
			autofixable := 0
			if vk.Autofixable {
				autofixable = 1
			}
			if _, err := kindStmt.Exec(kindID, policyID, string(vk.Priority), autofixable, vk.Reason, vk.Solution); err != nil {
				return 0, 0, err
			}
			totalKinds++
		}
	}
	return len(policies), totalKinds, nil
}

func exportViolations(tx *sql.Tx, violations []*Violation) (int, error) {
	stmt, err := tx.Prepare(`INSERT INTO violation (id, kind_id, scope, file_id, folder_id, line, column_num, excerpt, summary) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`)
	if err != nil {
		return 0, err
	}
	defer stmt.Close()
	ctx := NewCodebaseContext()
	ctx.LoadBundles()
	for _, v := range violations {
		kindID := "@semio-repo/violation-kind/" + string(v.Kind)
		var fileID, folderID, line, column, excerpt interface{}
		if v.Line > 0 {
			line = v.Line
		}
		if v.Column > 0 {
			column = v.Column
		}
		if v.Excerpt != "" {
			excerpt = v.Excerpt
		}
		filePath := extractFileFromScope(v.Scope)
		if filePath != "" {
			fileID = ctx.GetFileID(filePath)
			dir := filepath.Dir(filePath)
			if dir != "." && dir != "" {
				folderID = ctx.GetFolderID(dir)
			}
		}
		if _, err := stmt.Exec(v.ID, kindID, v.Scope, fileID, folderID, line, column, excerpt, v.Summary); err != nil {
			return 0, err
		}
	}
	return len(violations), nil
}

func ToolExport(outputPath string) ToolResult {
	output := NewOutput()
	output.Info("\n📦 Exporting repo to SQLite...")
	ctx := NewRepoContext(rootDir)
	result, err := ExportToSQLite(outputPath, ctx)
	if err != nil {
		output.Error(fmt.Sprintf("Export failed: %v", err))
		return ToolResult{Output: *output, Error: err.Error()}
	}
	output.Success(fmt.Sprintf("Exported to: %s", result.Path))
	output.Plain(fmt.Sprintf("  Bundles: %d", result.Bundles))
	output.Plain(fmt.Sprintf("  Folders: %d", result.Folders))
	output.Plain(fmt.Sprintf("  Files: %d", result.Files))
	output.Plain(fmt.Sprintf("  Sections: %d", result.Sections))
	output.Plain(fmt.Sprintf("  Definitions: %d", result.Definitions))
	output.Plain(fmt.Sprintf("  Contributors: %d", result.Contributors))
	output.Plain(fmt.Sprintf("  Tickets: %d", result.Tickets))
	output.Plain(fmt.Sprintf("  Policies: %d", result.Policies))
	output.Plain(fmt.Sprintf("  Violation Kinds: %d", result.ViolationKinds))
	output.Plain(fmt.Sprintf("  Violations: %d", result.Violations))
	return ToolResult{Output: *output, Data: result}
}

// #endregion SQLite Export

// #endregion Tickets

// #region GraphQL Context Port

type RepoContext interface {
	GetRootDir() string
	GetBundles() []*Bundle
	GetProjects() []*Project
	GetCommits(limit *int) ([]*Commit, error)
	GetFolders() []*Folder
	GetFiles() []*File
	GetSections() []*Section
	GetDefinitions() []*Definition
	GetContributors() ([]*Contributor, error)
	GetGoals() ([]*Goal, error)
	GetTickets(year, month, day *int, status *TicketStatus) ([]*Ticket, error)
	GetPolicies() []*Policy
	GetDrafts() ([]*Draft, error)
	GetTodos(filter *FilterInput) ([]*Todo, error)
	GetViolationKinds() []*ViolationKindMeta
	Analyze(scope *string) (*AnalyzeResult, error)
	Fix(scope *string) (*FixResult, error)
	GoalCreate(input GoalCreateInput) (*Goal, error)
	GoalChange(input GoalChangeInput) (*Goal, error)
	GoalClose(input GoalCloseInput) (*Goal, error)
	GoalReopen(input GoalReopenInput) (*Goal, error)
	GoalDelete(input GoalDeleteInput) (bool, error)
	TodoCreate(input TodoCreateInput) (*Todo, error)
	TodoChange(input TodoChangeInput) (*Todo, error)
	TodoDelete(id string) (bool, error)
	DraftCreate(input DraftCreateInput) (*Draft, error)
	DraftDelete(id string) (bool, error)
	TicketOpen(input TicketOpenInput) (*Ticket, error)
	TicketProgress(input TicketProgressInput) (string, error)
	TicketClose(input TicketCloseInput) (*Ticket, error)
	TicketReopen(input TicketReopenInput) (*Ticket, error)
	TicketChange(input TicketChangeInput) (*Ticket, error)
	TicketDelete(input TicketDeleteInput) (bool, error)
	FolderCreate(path string) (*Folder, error)
	FolderMove(src, dst string) (*Folder, error)
	FolderDelete(path string) error
	FileCreate(path string) (*File, error)
	FileMove(src, dst string) (*File, error)
	FileDelete(path string) error
	SectionCreate(file, name string, parent *string) (*Section, error)
	SectionMove(file, oldName, newName string) (*Section, error)
	SectionDelete(file, name string) error
	Integrate(source, targetSection, targetFile, targetParent *string) (*File, error)
	Extract(sourceFile, sourceSection, targetFile *string) (*File, error)
	ContributorAdd(input ContributorAddInput) (*Contributor, error)
	ContributorRemove(github string) error
}

// #endregion GraphQL Context Port

// #region GraphQL Resolver

type Resolver struct {
	RootDir string
	Ctx     RepoContext
}

func NewResolver(rootDir string) *Resolver {
	return &Resolver{RootDir: rootDir, Ctx: NewRepoContext(rootDir)}
}

func NewResolverWithContext(rootDir string, ctx RepoContext) *Resolver {
	return &Resolver{RootDir: rootDir, Ctx: ctx}
}

func (r *Resolver) context() RepoContext {
	return r.Ctx
}

// #endregion GraphQL Resolver

// #region Default Context

type defaultContext struct {
	rootDir string
}

func NewDefaultContext(rootDir string) RepoContext {
	return &defaultContext{rootDir: rootDir}
}

// repoContext is the full implementation of RepoContext
type repoContext struct {
	rootDir string
	bundles []Bundle
}

func NewRepoContext(rootDir string) RepoContext {
	ctx := &repoContext{rootDir: rootDir}
	ctx.bundles = LoadBundles()
	return ctx
}

func (c *repoContext) GetRootDir() string { return c.rootDir }

func (c *repoContext) GetFileID(path string) string {
	ctx := &CodebaseContext{RootDir: c.rootDir, Bundles: c.bundles}
	return ctx.GetFileID(path)
}

func (c *repoContext) GetFolderID(path string) string {
	ctx := &CodebaseContext{RootDir: c.rootDir, Bundles: c.bundles}
	return ctx.GetFolderID(path)
}

func (c *repoContext) GetBundles() []*Bundle {
	result := make([]*Bundle, len(c.bundles))
	for i := range c.bundles {
		result[i] = &c.bundles[i]
	}
	return result
}

func (c *repoContext) GetProjects() []*Project {
	projects := LoadProjects()
	res := make([]*Project, len(projects))
	for i := range projects {
		res[i] = &projects[i]
	}
	return res
}

func (c *repoContext) GetCommits(limit *int) ([]*Commit, error) {
	commits := LoadCommits(limit)
	res := make([]*Commit, len(commits))
	for i := range commits {
		res[i] = &commits[i]
	}
	return res, nil
}

func (c *repoContext) GetFolders() []*Folder {
	ctx := NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return []*Folder{}
	}
	folders := BuildCodebaseFolders(ctx)
	results := make([]*Folder, 0, len(folders))
	for _, entry := range folders {
		parent := filepath.Dir(entry.Path)
		var parentID *string
		if parent != "." {
			pid := ctx.GetFolderID(parent)
			parentID = &pid
		}
		bundleID := ctx.GetBundleForFile(entry.Path)
		results = append(results, &Folder{
			ID:       entry.ID,
			Path:     entry.Path,
			URI:      entry.URI,
			Name:     entry.Name,
			ParentID: parentID,
			BundleID: &bundleID,
		})
	}
	return results
}

func (c *repoContext) GetFiles() []*File {
	ctx := NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return []*File{}
	}
	files := BuildCodebaseFiles(ctx)
	results := make([]*File, 0, len(files))
	for _, entry := range files {
		bundleID := ctx.GetBundleForFile(entry.Path)
		folder := filepath.Dir(entry.Path)
		var folderID *string
		if folder != "." {
			fid := ctx.GetFolderID(folder)
			folderID = &fid
		}
		ext := strings.TrimPrefix(filepath.Ext(entry.Path), ".")
		results = append(results, &File{
			ID:        entry.ID,
			Path:      entry.Path,
			URI:       entry.URI,
			Name:      filepath.Base(entry.Path),
			Extension: ext,
			FolderID:  folderID,
			BundleID:  &bundleID,
		})
	}
	return results
}

func (c *repoContext) GetDefinitions() []*Definition {
	ctx := NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return []*Definition{}
	}
	var results []*Definition
	for _, file := range ctx.Files {
		tool := ToolDefinitionList(file)
		definitions, ok := tool.Data.([]Definition)
		if !ok {
			continue
		}
		fileID := ctx.GetFileID(file)
		for i := range definitions {
			def := definitions[i]
			// Update ID to hierarcData.hical
			var sectionSegments []string
			if def.SectionPath != "" {
				sectionSegments = strings.Split(def.SectionPath, "/")
			}
			id := buildDefinitionID(fileID, sectionSegments, def.Name)
			results = append(results, &Definition{
				Name:        def.Name,
				Kind:        def.Kind,
				FilePath:    def.FilePath,
				SectionPath: def.SectionPath,
				StartLine:   def.StartLine,
				EndLine:     def.EndLine,
				ID:          id, // Explicit ID for GQL
			})
		}
	}
	return results
}

func (c *repoContext) GetSections() []*Section {
	ctx := NewCodebaseContext()
	ctx.LoadBundles()
	if err := ctx.LoadFiles(); err != nil {
		return []*Section{}
	}
	var results []*Section
	for _, file := range ctx.Files {
		tool := ToolSectionList(file)
		sections, ok := tool.Data.([]Section)
		if !ok {
			continue
		}
		fileID := ctx.GetFileID(file)
		for i := range sections {
			sec := sections[i]
			// Update ID to hierData.archical
			id := buildSectionID(fileID, strings.Split(sec.Path, "/"))
			results = append(results, &Section{
				Name:       sec.Name,
				Path:       sec.Path,
				FilePath:   sec.FilePath,
				StartLine:  sec.StartLine,
				EndLine:    sec.EndLine,
				StartIndex: sec.StartIndex,
				EndIndex:   sec.EndIndex,
				ID:         id, // Explicit ID for GQL
			})
		}
	}
	return results
}

func (c *repoContext) GetContributors() ([]*Contributor, error) {
	contributors, err := ListContributors()
	if err != nil {
		return nil, err
	}
	result := make([]*Contributor, len(contributors))
	for i := range contributors {
		result[i] = &contributors[i]
	}
	return result, nil
}

func (c *repoContext) GetTickets(year, month, day *int, status *TicketStatus) ([]*Ticket, error) {
	tickets, err := ListTickets(year, month, day)
	if err != nil {
		return nil, err
	}
	var result []*Ticket
	for i := range tickets {
		if status == nil || tickets[i].GetStatus() == *status {
			result = append(result, &tickets[i])
		}
	}
	return result, nil
}

func (c *repoContext) GetGoals() ([]*Goal, error) {
	return ListGoals()
}

func (c *repoContext) GoalCreate(input GoalCreateInput) (*Goal, error) {
	// Validate required fields
	if input.Title == "" {
		return nil, fmt.Errorf("missing title")
	}
	if input.Description == "" {
		return nil, fmt.Errorf("missing description")
	}
	if input.Prompt == "" {
		return nil, fmt.Errorf("missing prompt")
	}
	if input.DueDate == "" {
		return nil, fmt.Errorf("missing due date")
	}
	if input.LLM == "" {
		return nil, fmt.Errorf("missing llm")
	}
	if input.UI == "" {
		return nil, fmt.Errorf("missing ui")
	}

	// Validate LLM and UI
	llmSlug, err := ResolveAllowedLLM(input.LLM)
	if err != nil {
		return nil, err
	}
	uiSlug, err := ResolveAllowedUI(input.UI)
	if err != nil {
		return nil, err
	}

	slug := Slugify(input.Title)
	id := slug
	if input.Parent != "" {
		id = input.Parent + "/" + slug
	}

	dir := GetRepoGoalsDir()
	path := filepath.Join(dir, filepath.FromSlash(id), "goal.json")
	if FileExists(path) {
		return nil, fmt.Errorf("goal with id %s already exists", id)
	}

	var milestoneUrl string
	if !input.NoGithub {
		if input.Milestone != "" {
			// Use existing milestone URL
			milestoneUrl = input.Milestone
		} else {
			// Create new milestone
			milestoneNumber, err := ghCreateMilestone(input.Title, input.Description)
			if err != nil {
				return nil, err
			}
			repoUrl, _ := getGhRepoUrl()
			milestoneUrl = fmt.Sprintf("%s/milestone/%d", repoUrl, milestoneNumber)
		}
	}

	gitAuthor := GetGitAuthorGithub()

	goal := Goal{
		ID:          id,
		Title:       input.Title,
		Parent:      input.Parent,
		Description: input.Description,
		Prompt:      input.Prompt,
		Status:      "open",
		Dates:       GoalDates{Due: input.DueDate},
		UI:          uiSlug,
		LLM:         llmSlug,
		Iterations: []TicketIteration{{
			Prompt:  input.Prompt,
			LLM:     llmSlug,
			UI:      uiSlug,
			Author:  parseGitAuthor(gitAuthor),
			Started: time.Now(),
		}},
	}
	if !input.NoGithub {
		goal.GitHub = &GoalGithubData{
			Milestone: milestoneUrl,
		}
	}

	if err := SaveGoal(goal); err != nil {
		return nil, err
	}
	return &goal, nil
}

func getGhRepoUrl() (string, error) {
	out, err := exec.Command("gh", "repo", "view", "--json", "url", "--jq", ".url").Output()
	if err != nil {
		return "", err
	}
	return strings.TrimSpace(string(out)), nil
}

func parseMilestoneNumber(milestone string) (int, error) {
	if n, err := strconv.Atoi(milestone); err == nil {
		return n, nil
	}
	parts := strings.Split(milestone, "/")
	if len(parts) > 0 {
		if n, err := strconv.Atoi(parts[len(parts)-1]); err == nil {
			return n, nil
		}
	}
	return 0, fmt.Errorf("could not parse milestone number from %s", milestone)
}

func UpdateGoalTitle(goal *Goal, title string) error {
	title = strings.TrimSpace(title)
	if title == "" {
		return fmt.Errorf("goal title is required")
	}
	newSlug := Slugify(title)
	if title == newSlug {
		return fmt.Errorf("goal title must be titleized (e.g. \"Some Title on Something\") and NOT an all-caps slug")
	}
	if title == strings.ToLower(newSlug) {
		return fmt.Errorf("goal title must be titleized (e.g. \"Some Title on Something\") and NOT a slug")
	}

	var newID string
	if goal.Parent != "" {
		newID = goal.Parent + "/" + newSlug
	} else {
		newID = newSlug
	}

	if newID != goal.ID {
		dir := GetRepoGoalsDir()
		oldPath := filepath.Join(dir, filepath.FromSlash(goal.ID))
		newPath := filepath.Join(dir, filepath.FromSlash(newID))
		if FileExists(newPath) {
			return fmt.Errorf("goal folder already exists: %s", newPath)
		}
		if err := os.Rename(oldPath, newPath); err != nil {
			return err
		}
		goal.ID = newID
	}
	goal.Title = title
	return nil
}

func (c *repoContext) GoalChange(input GoalChangeInput) (*Goal, error) {
	dir := GetRepoGoalsDir()
	path := filepath.Join(dir, input.ID, "goal.json")
	content, err := ReadTextFile(path)
	if err != nil {
		return nil, err
	}
	var goal Goal
	if err := json.Unmarshal([]byte(content), &goal); err != nil {
		return nil, err
	}
	goal.ID = input.ID // Ensure ID is set

	if input.Title != nil {
		if err := UpdateGoalTitle(&goal, *input.Title); err != nil {
			return nil, err
		}
	}
	if input.Description != nil {
		goal.Description = *input.Description
	}
	if input.DueDate != nil {
		goal.Dates.Due = *input.DueDate
	}
	if input.Parent != nil {
		goal.Parent = *input.Parent

		// Logic to move goal to new parent
		slug := goal.ID
		if idx := strings.LastIndex(goal.ID, "/"); idx != -1 {
			slug = goal.ID[idx+1:]
		}

		var newID string
		if goal.Parent != "" {
			newID = goal.Parent + "/" + slug
		} else {
			newID = slug
		}

		if newID != goal.ID {
			oldPath := filepath.Join(dir, filepath.FromSlash(goal.ID))
			newPath := filepath.Join(dir, filepath.FromSlash(newID))

			if FileExists(newPath) {
				return nil, fmt.Errorf("goal folder already exists: %s", newPath)
			}
			// Ensure parent dir exists
			if err := os.MkdirAll(filepath.Dir(newPath), 0755); err != nil {
				return nil, err
			}

			if err := os.Rename(oldPath, newPath); err != nil {
				return nil, err
			}
			goal.ID = newID
		}
	}

	if goal.GitHub != nil && !input.NoGithub {
		number, err := parseMilestoneNumber(goal.GitHub.Milestone)
		if err == nil {
			status := goal.Status
			// Update milestone with current values
			if err := ghUpdateMilestone(number, goal.Title, goal.Description, status, goal.Dates.Due); err != nil {
				return nil, err
			}
		}
	}

	if err := SaveGoal(goal); err != nil {
		return nil, err
	}
	return &goal, nil
}

func (c *repoContext) GoalClose(input GoalCloseInput) (*Goal, error) {
	dir := GetRepoGoalsDir()
	path := filepath.Join(dir, input.ID, "goal.json")
	content, err := ReadTextFile(path)
	if err != nil {
		return nil, err
	}
	var goal Goal
	if err := json.Unmarshal([]byte(content), &goal); err != nil {
		return nil, err
	}
	goal.ID = input.ID

	goal.Status = "closed"
	goal.Summary = input.Summary
	now := time.Now()
	goal.Dates.Closed = &now

	if goal.GitHub != nil && !input.NoGithub {
		number, err := parseMilestoneNumber(goal.GitHub.Milestone)
		if err == nil {
			if err := ghUpdateMilestone(number, goal.Title, goal.Description, "closed", ""); err != nil {
				return nil, err
			}
		}
	}

	if err := SaveGoal(goal); err != nil {
		return nil, err
	}
	return &goal, nil
}

func (c *repoContext) GoalReopen(input GoalReopenInput) (*Goal, error) {
	dir := GetRepoGoalsDir()
	path := filepath.Join(dir, input.ID, "goal.json")
	content, err := ReadTextFile(path)
	if err != nil {
		return nil, err
	}
	var goal Goal
	if err := json.Unmarshal([]byte(content), &goal); err != nil {
		return nil, err
	}
	goal.ID = input.ID

	goal.Status = "open"

	if input.Title != nil {
		if err := UpdateGoalTitle(&goal, *input.Title); err != nil {
			return nil, err
		}
	}
	if input.Description != nil {
		goal.Description = *input.Description
	}
	if input.DueDate != nil {
		goal.Dates.Due = *input.DueDate
	}
	if input.Parent != nil {
		goal.Parent = *input.Parent
	}

	gitAuthor := GetGitAuthorGithub()
	goal.Iterations = append(goal.Iterations, TicketIteration{
		Prompt:  input.Prompt,
		LLM:     input.LLM,
		UI:      input.UI,
		Author:  parseGitAuthor(gitAuthor),
		Started: time.Now(),
	})
	goal.Prompt = input.Prompt
	goal.LLM = input.LLM
	goal.UI = input.UI

	if goal.GitHub != nil && !input.NoGithub {
		number, err := parseMilestoneNumber(goal.GitHub.Milestone)
		if err == nil {
			if err := ghUpdateMilestone(number, goal.Title, goal.Description, "open", goal.Dates.Due); err != nil {
				return nil, err
			}
		}
	}

	if err := SaveGoal(goal); err != nil {
		return nil, err
	}
	return &goal, nil
}

func (c *repoContext) TicketChange(input TicketChangeInput) (*Ticket, error) {
	ticket, err := ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}

	// Handle title update if provided
	if input.Title != nil && *input.Title != "" {
		if err := UpdateTicketTitle(ticket, *input.Title); err != nil {
			return nil, err
		}
		// Update GitHub issue title if exists
		if ticket.GitHub != nil && ticket.GitHub.Issue != "" && !input.NoGithub {
			if err := ghUpdateIssueTitle(ticket.GitHub.Issue, *input.Title); err != nil {
				fmt.Printf("Warning: Failed to update GitHub issue title: %v\n", err)
			}
		}
	}

	// Handle other updates
	changed := false
	if input.Prompt != nil {
		ticket.Prompt = *input.Prompt
		if len(ticket.Iterations) > 0 {
			ticket.Iterations[len(ticket.Iterations)-1].Prompt = *input.Prompt
		}
		changed = true
	}
	// Update iterations for LLM/UI
	if len(ticket.Iterations) > 0 {
		if input.LLM != nil {
			llmSlug, err := ResolveAllowedLLM(*input.LLM)
			if err != nil {
				return nil, err
			}
			ticket.Iterations[len(ticket.Iterations)-1].LLM = llmSlug
			changed = true
		}
		if input.UI != nil {
			uiSlug, err := ResolveAllowedUI(*input.UI)
			if err != nil {
				return nil, err
			}
			ticket.Iterations[len(ticket.Iterations)-1].UI = uiSlug
			changed = true
		}
	}
	if input.Goal != nil {
		ticket.Goal = *input.Goal
		changed = true
	}
	if input.Parent != nil {
		ticket.Parent = *input.Parent
		changed = true
	}

	if changed {
		if err := SaveTicket(ticket); err != nil {
			return nil, err
		}
	}

	return ticket, nil
}

func (c *repoContext) GoalDelete(input GoalDeleteInput) (bool, error) {
	dir := GetRepoGoalsDir()
	path := filepath.Join(dir, input.ID, "goal.json")
	content, err := ReadTextFile(path)
	if err != nil {
		return false, err
	}
	var goal Goal
	if err := json.Unmarshal([]byte(content), &goal); err != nil {
		return false, err
	}

	if !input.NoGithub && goal.GitHub != nil {
		number, err := parseMilestoneNumber(goal.GitHub.Milestone)
		if err == nil {
			if err := ghDeleteMilestone(number); err != nil {
				return false, err
			}
		}
	}

	if err := os.RemoveAll(filepath.Dir(path)); err != nil {
		return false, err
	}
	return true, nil
}

func (c *repoContext) TicketDelete(input TicketDeleteInput) (bool, error) {
	ticket, err := ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return false, err
	}

	if !input.NoGithub && ticket.GitHub != nil {
		if ticket.GitHub.Issue != "" {
			parts := strings.Split(ticket.GitHub.Issue, "/")
			number := parts[len(parts)-1]
			if err := ghDeleteIssue(number); err != nil {
				return false, err
			}
		}
	}

	path := ticket.FolderPath
	if path == "" {
		base := filepath.Join(GetRepoMetaDir(), "tickets", fmt.Sprintf("%d", input.Year), fmt.Sprintf("%02d", input.Month), fmt.Sprintf("%02d", input.Day), input.Slug)
		path = base
	}

	if err := os.RemoveAll(path); err != nil {
		return false, err
	}

	legacyPath := filepath.Join("tickets", fmt.Sprintf("%d", input.Year), fmt.Sprintf("%02d", input.Month), fmt.Sprintf("%02d", input.Day), input.Slug)
	if FileExists(legacyPath) {
		_ = os.RemoveAll(legacyPath)
	}

	return true, nil
}

func (c *repoContext) GetDrafts() ([]*Draft, error) {
	return ListDrafts()
}

func (c *repoContext) DraftCreate(input DraftCreateInput) (*Draft, error) {
	return CreateDraft(input.Slug, input.Files)
}

func (c *repoContext) DraftDelete(id string) (bool, error) {
	return true, DeleteDraft(id)
}

func (c *repoContext) GetPolicies() []*Policy {
	policies := GetRegisteredPolicies()
	result := make([]*Policy, len(policies))
	for i := range policies {
		var descPtr *string
		if policies[i].Description != "" {
			d := policies[i].Description
			descPtr = &d
		}
		var violationKinds []*ViolationKindMeta
		for _, kind := range policies[i].Kinds {
			meta := kind.Info()
			meta.PolicyID = policies[i].ID
			violationKinds = append(violationKinds, &meta)
		}
		result[i] = &Policy{
			ID:             policies[i].ID,
			Name:           policies[i].Name,
			Description:    descPtr,
			Scopes:         policies[i].Scopes,
			ViolationKinds: violationKinds,
		}
	}
	return result
}

func (c *repoContext) GetViolationKinds() []*ViolationKindMeta {
	var result []*ViolationKindMeta
	for _, meta := range violationKindInfoTable {
		m := meta
		result = append(result, &m)
	}
	return result
}

func (c *repoContext) Analyze(scope *string) (*AnalyzeResult, error) {
	scopeStr := "@semio"
	if scope != nil {
		scopeStr = *scope
	}
	violations, err := CheckPolicies(ParseScope(scopeStr), c.bundles, nil)
	if err != nil {
		return nil, err
	}
	result := make([]*Violation, len(violations))
	for i := range violations {
		result[i] = &violations[i]
	}
	return &AnalyzeResult{Violations: result, Metrics: &AnalyzeMetrics{Total: len(violations)}}, nil
}

func (c *repoContext) Fix(scope *string) (*FixResult, error) {
	return &FixResult{Violations: []*Violation{}}, nil
}

func (c *repoContext) TicketOpen(input TicketOpenInput) (*Ticket, error) {
	return OpenTicket(input.Title, input.Prompt, input.LLM, input.UI, input.Draft, input.NoIssue, input.Goal, input.Parent, input.NoGithub, input.Issue)
}

func (c *repoContext) TicketProgress(input TicketProgressInput) (string, error) {
	ticket, err := ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return "", err
	}
	return ProgressTicket(ticket, input.Summary)
}

func (c *repoContext) TicketClose(input TicketCloseInput) (*Ticket, error) {
	if input.All {
		tickets, err := ListTickets(nil, nil, nil)
		if err != nil {
			return nil, err
		}
		var lastTicket *Ticket
		for _, t := range tickets {
			if t.Status == TicketStatusOpen {
				ticket := t
				fmt.Printf("Closing ticket %s...\n", ticket.Slug)
				if err := FinishTicket(&ticket, "Bulk close", []string{}, input.NoGithub, true); err != nil {
					fmt.Printf("Warning: Failed to close ticket %s: %v\n", ticket.Slug, err)
					continue
				}
				lastTicket = &ticket
			}
		}
		// Also close all GitHub issues with the "ticket" label
		if !input.NoGithub {
			issueURLs, err := ghListOpenIssuesWithLabel("ticket")
			if err != nil {
				fmt.Printf("Warning: Failed to list GitHub issues with 'ticket' label: %v\n", err)
			} else {
				for _, issueURL := range issueURLs {
					fmt.Printf("Closing GitHub issue %s...\n", issueURL)
					if err := ghCloseIssue(issueURL); err != nil {
						fmt.Printf("Warning: Failed to close GitHub issue %s: %v\n", issueURL, err)
					}
				}
			}
		}
		return lastTicket, nil
	}
	ticket, err := ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}
	// Handle title update if provided
	if input.Title != nil && *input.Title != "" {
		if err := UpdateTicketTitle(ticket, *input.Title); err != nil {
			return nil, err
		}
		// Update GitHub issue title if exists
		if ticket.GitHub != nil && ticket.GitHub.Issue != "" && !input.NoGithub {
			if err := ghUpdateIssueTitle(ticket.GitHub.Issue, *input.Title); err != nil {
				fmt.Printf("Warning: Failed to update GitHub issue title: %v\n", err)
			}
		}
	}
	if err := FinishTicket(ticket, input.Summary, input.Files, input.NoGithub, false); err != nil {
		return nil, err
	}
	return ticket, nil
}

func (c *repoContext) TicketReopen(input TicketReopenInput) (*Ticket, error) {
	ticket, err := ReadTicket(input.Year, input.Month, input.Day, input.Slug)
	if err != nil {
		return nil, err
	}
	// Handle title update if provided
	if input.Title != nil && *input.Title != "" {
		if err := UpdateTicketTitle(ticket, *input.Title); err != nil {
			return nil, err
		}
		// Update GitHub issue title if exists
		if ticket.GitHub != nil && ticket.GitHub.Issue != "" && !input.NoGithub {
			if err := ghUpdateIssueTitle(ticket.GitHub.Issue, *input.Title); err != nil {
				fmt.Printf("Warning: Failed to update GitHub issue title: %v\n", err)
			}
		}
	}
	if err := ReopenTicket(ticket, input.Prompt, input.LLM, input.UI, input.Draft, input.Goal, input.Parent, input.NoGithub); err != nil {
		return nil, err
	}
	return ticket, nil
}

func (c *repoContext) FolderCreate(path string) (*Folder, error) { return nil, nil }

func (c *repoContext) FolderMove(src, dst string) (*Folder, error) {
	result := ToolFolderMove(src, dst)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	ctx := &CodebaseContext{Bundles: c.bundles}
	return &Folder{ID: ctx.GetFolderID(dst), Path: dst, Name: filepath.Base(dst)}, nil
}

func (c *repoContext) FolderDelete(path string) error {
	result := ToolFolderDelete(path)
	if result.Error != "" {
		return errors.New(result.Error)
	}
	return nil
}

func (c *repoContext) FileCreate(path string) (*File, error) {
	result := ToolFileCreate(path)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	ctx := &CodebaseContext{Bundles: c.bundles}
	return &File{ID: ctx.GetFileID(path), Path: path, Name: filepath.Base(path), Extension: strings.TrimPrefix(filepath.Ext(path), ".")}, nil
}

func (c *repoContext) FileMove(src, dst string) (*File, error) {
	result := ToolFileMove(src, dst)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	ctx := &CodebaseContext{Bundles: c.bundles}
	return &File{ID: ctx.GetFileID(dst), Path: dst, Name: filepath.Base(dst), Extension: strings.TrimPrefix(filepath.Ext(dst), ".")}, nil
}

func (c *repoContext) FileDelete(path string) error {
	result := ToolFileDelete(path)
	if result.Error != "" {
		return errors.New(result.Error)
	}
	return nil
}

func (c *repoContext) SectionCreate(file, name string, parent *string) (*Section, error) {
	sectionPath := name
	if parent != nil && *parent != "" {
		sectionPath = *parent + "/" + name
	}
	result := ToolSectionCreate(file, sectionPath)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	fileID := c.GetFileID(file)
	id := buildSectionID(fileID, strings.Split(sectionPath, "/"))
	return &Section{ID: id, Name: name, Path: sectionPath, FilePath: file}, nil
}

func (c *repoContext) SectionMove(file, oldName, newName string) (*Section, error) {
	result := ToolSectionMove(file, oldName, newName)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	fileID := c.GetFileID(file)
	id := buildSectionID(fileID, strings.Split(newName, "/"))
	return &Section{ID: id, Name: newName, Path: newName, FilePath: file}, nil
}

func (c *repoContext) SectionDelete(file, name string) error {
	result := ToolSectionDelete(file, name)
	if result.Error != "" {
		return errors.New(result.Error)
	}
	return nil
}

func (c *repoContext) Integrate(source, targetSection, targetFile, targetParent *string) (*File, error) {
	s := ""
	if source != nil {
		s = *source
	}
	ts := ""
	if targetSection != nil {
		ts = *targetSection
	}
	tf := ""
	if targetFile != nil {
		tf = *targetFile
	}
	tp := ""
	if targetParent != nil {
		tp = *targetParent
	}
	result := ToolIntegrate(s, ts, tf, tp)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	return &File{ID: c.GetFileID(tf), Path: tf, Name: filepath.Base(tf)}, nil
}

func (c *repoContext) Extract(sourceFile, sourceSection, targetFile *string) (*File, error) {
	s := ""
	if sourceFile != nil {
		s = *sourceFile
	}
	ss := ""
	if sourceSection != nil {
		ss = *sourceSection
	}
	tf := ""
	if targetFile != nil {
		tf = *targetFile
	}

	result := ToolExtract(s, ss, tf)
	if result.Error != "" {
		return nil, errors.New(result.Error)
	}
	return &File{ID: c.GetFileID(tf), Path: tf, Name: filepath.Base(tf)}, nil
}

func (c *repoContext) ContributorAdd(input ContributorAddInput) (*Contributor, error) {
	return nil, nil
}

func (c *repoContext) ContributorRemove(github string) error { return nil }

var _ RepoContext = (*repoContext)(nil)

func (c *defaultContext) GetRootDir() string { return c.rootDir }

func (c *defaultContext) GetBundles() []*Bundle   { return []*Bundle{} }
func (c *defaultContext) GetProjects() []*Project { return []*Project{} }
func (c *defaultContext) GetCommits(limit *int) ([]*Commit, error) {
	return []*Commit{}, nil
}

func (c *defaultContext) GetFolders() []*Folder { return []*Folder{} }

func (c *defaultContext) GetFiles() []*File { return []*File{} }

func (c *defaultContext) GetDefinitions() []*Definition { return []*Definition{} }

func (c *defaultContext) GetSections() []*Section { return []*Section{} }

func (c *defaultContext) GetContributors() ([]*Contributor, error) { return []*Contributor{}, nil }

func (c *defaultContext) GetTickets(year, month, day *int, status *TicketStatus) ([]*Ticket, error) {
	return []*Ticket{}, nil
}

func (c *defaultContext) GetPolicies() []*Policy { return []*Policy{} }

func (c *defaultContext) GetViolationKinds() []*ViolationKindMeta { return []*ViolationKindMeta{} }

func (c *defaultContext) Analyze(scope *string) (*AnalyzeResult, error) {
	return &AnalyzeResult{Violations: []*Violation{}, Metrics: &AnalyzeMetrics{}}, nil
}

func (c *defaultContext) Fix(scope *string) (*FixResult, error) {
	return &FixResult{Violations: []*Violation{}}, nil
}

func (c *defaultContext) TicketOpen(input TicketOpenInput) (*Ticket, error) {
	return nil, nil
}

func (c *defaultContext) TicketProgress(input TicketProgressInput) (string, error) {
	return "", nil
}

func (c *defaultContext) TicketClose(input TicketCloseInput) (*Ticket, error) {
	return nil, nil
}

func (c *defaultContext) TicketReopen(input TicketReopenInput) (*Ticket, error) {
	return nil, nil
}

func (c *defaultContext) TicketChange(input TicketChangeInput) (*Ticket, error) {
	return nil, nil
}

func (c *defaultContext) FolderCreate(path string) (*Folder, error) { return nil, nil }

func (c *defaultContext) FolderMove(src, dst string) (*Folder, error) { return nil, nil }

func (c *defaultContext) FolderDelete(path string) error { return nil }

func (c *defaultContext) FileCreate(path string) (*File, error) { return nil, nil }

func (c *defaultContext) FileMove(src, dst string) (*File, error) { return nil, nil }

func (c *defaultContext) FileDelete(path string) error { return nil }

func (c *defaultContext) SectionCreate(file, name string, parent *string) (*Section, error) {
	return nil, nil
}

func (c *defaultContext) SectionMove(file, oldName, newName string) (*Section, error) {
	return nil, nil
}

func (c *defaultContext) SectionDelete(file, name string) error { return nil }

func (c *defaultContext) Integrate(source, targetSection, targetFile, targetParent *string) (*File, error) {
	return nil, nil
}

func (c *defaultContext) Extract(sourceFile, sourceSection, targetFile *string) (*File, error) {
	return nil, nil
}

func (c *defaultContext) ContributorAdd(input ContributorAddInput) (*Contributor, error) {
	return nil, nil
}

func (c *defaultContext) ContributorRemove(github string) error { return nil }

func (c *defaultContext) GetGoals() ([]*Goal, error) { return []*Goal{}, nil }

func (c *defaultContext) GoalCreate(input GoalCreateInput) (*Goal, error) { return nil, nil }

func (c *defaultContext) GoalChange(input GoalChangeInput) (*Goal, error) { return nil, nil }

func (c *defaultContext) GoalClose(input GoalCloseInput) (*Goal, error) { return nil, nil }

func (c *defaultContext) GoalReopen(input GoalReopenInput) (*Goal, error) { return nil, nil }

func (c *defaultContext) GoalDelete(input GoalDeleteInput) (bool, error) { return false, nil }

func (c *defaultContext) TicketDelete(input TicketDeleteInput) (bool, error) { return false, nil }

func (c *defaultContext) GetDrafts() ([]*Draft, error)                       { return []*Draft{}, nil }
func (c *defaultContext) DraftCreate(input DraftCreateInput) (*Draft, error) { return nil, nil }
func (c *defaultContext) DraftDelete(id string) (bool, error)                { return false, nil }

func (c *defaultContext) GetTodos(filter *FilterInput) ([]*Todo, error)   { return []*Todo{}, nil }
func (c *defaultContext) TodoCreate(input TodoCreateInput) (*Todo, error) { return nil, nil }
func (c *defaultContext) TodoChange(input TodoChangeInput) (*Todo, error) { return nil, nil }
func (c *defaultContext) TodoDelete(id string) (bool, error)              { return false, nil }

var _ RepoContext = (*defaultContext)(nil)

// #endregion Default Context

// #region GraphQL Executor

func parseFileListInput(f map[string]interface{}) *FileListInput {
	files := &FileListInput{}
	if updated, ok := f["updated"].([]interface{}); ok {
		for _, u := range updated {
			if s, ok := u.(string); ok {
				files.Updated = append(files.Updated, s)
			}
		}
	}
	if created, ok := f["created"].([]interface{}); ok {
		for _, c := range created {
			if s, ok := c.(string); ok {
				files.Created = append(files.Created, s)
			}
		}
	}
	if removed, ok := f["removed"].([]interface{}); ok {
		for _, r := range removed {
			if s, ok := r.(string); ok {
				files.Removed = append(files.Removed, s)
			}
		}
	}
	return files
}

type Executor struct {
	resolver *Resolver
	schema   graphql.Schema
}

func NewExecutor(rootDir string) (*Executor, error) {
	resolver := NewResolver(rootDir)
	schema, err := buildSchema(resolver)
	if err != nil {
		return nil, err
	}
	return &Executor{
		resolver: resolver,
		schema:   schema,
	}, nil
}

func NewExecutorWithContext(rootDir string, ctx RepoContext) (*Executor, error) {
	resolver := NewResolverWithContext(rootDir, ctx)
	schema, err := buildSchema(resolver)
	if err != nil {
		return nil, err
	}
	return &Executor{
		resolver: resolver,
		schema:   schema,
	}, nil
}

func (e *Executor) Execute(ctx context.Context, query string, variables map[string]interface{}) (interface{}, error) {
	result := graphql.Do(graphql.Params{
		Context:        ctx,
		Schema:         e.schema,
		RequestString:  query,
		VariableValues: variables,
	})
	if len(result.Errors) > 0 {
		return nil, fmt.Errorf("graphql errors: %v", result.Errors)
	}
	return result.Data, nil
}

func (e *Executor) ExecuteJSON(ctx context.Context, query string, variables map[string]interface{}) (string, error) {
	data, err := e.Execute(ctx, query, variables)
	if err != nil {
		return "", err
	}
	jsonBytes, err := json.MarshalIndent(data, "", "  ")
	if err != nil {
		return "", err
	}
	return string(jsonBytes), nil
}

func (e *Executor) ValidateQuery(query string) error {
	_, err := parser.Parse(parser.ParseParams{
		Source: query,
		Options: parser.ParseOptions{
			NoLocation: true,
		},
	})
	return err
}

func (e *Executor) GetOperationType(query string) (string, error) {
	doc, err := parser.Parse(parser.ParseParams{
		Source: query,
		Options: parser.ParseOptions{
			NoLocation: true,
		},
	})
	if err != nil {
		return "", err
	}
	for _, def := range doc.Definitions {
		if opDef, ok := def.(*ast.OperationDefinition); ok {
			return string(opDef.Operation), nil
		}
	}
	return "query", nil
}

// #endregion GraphQL Executor

// #region Schema Builder

func buildSchema(resolver *Resolver) (graphql.Schema, error) {
	repoResolverInstance = resolver
	positionType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Position",
		Fields: graphql.Fields{
			"line":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"column": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	rangeType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Range",
		Fields: graphql.Fields{
			"start": &graphql.Field{Type: graphql.NewNonNull(positionType)},
			"end":   &graphql.Field{Type: graphql.NewNonNull(positionType)},
		},
	})

	countMetricsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "CountMetrics",
		Fields: graphql.Fields{
			"added":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"updated": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"removed": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	priorityCountType := graphql.NewObject(graphql.ObjectConfig{
		Name: "PriorityCount",
		Fields: graphql.Fields{
			"high":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"medium": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"low":    &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	analyzeMetricsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "AnalyzeMetrics",
		Fields: graphql.Fields{
			"total":       &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"byPriority":  &graphql.Field{Type: priorityCountType},
			"autofixable": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
		},
	})

	definitionKindEnum := graphql.NewEnum(graphql.EnumConfig{
		Name: "DefinitionKind",
		Values: graphql.EnumValueConfigMap{
			"IMPLEMENTATION": &graphql.EnumValueConfig{Value: DefinitionKindImplementation},
			"INTERFACE":      &graphql.EnumValueConfig{Value: DefinitionKindInterface},
			"CONSTANT":       &graphql.EnumValueConfig{Value: DefinitionKindConstant},
		},
	})

	bundleKindEnum := graphql.NewEnum(graphql.EnumConfig{
		Name: "BundleKind",
		Values: graphql.EnumValueConfigMap{
			"LIBRARY": &graphql.EnumValueConfig{Value: BundleKindLibrary},
			"SCHEMA":  &graphql.EnumValueConfig{Value: BundleKindSchema},
			"BINARY":  &graphql.EnumValueConfig{Value: BundleKindBinary},
			"UI":      &graphql.EnumValueConfig{Value: BundleKindUI},
			"SITE":    &graphql.EnumValueConfig{Value: BundleKindSite},
			"ASSETS":  &graphql.EnumValueConfig{Value: BundleKindAssets},
		},
	})

	folderKindEnum := graphql.NewEnum(graphql.EnumConfig{
		Name: "FolderKind",
		Values: graphql.EnumValueConfigMap{
			"ORGANIZATION": &graphql.EnumValueConfig{Value: FolderKindOrganization},
			"REQUIRED":     &graphql.EnumValueConfig{Value: FolderKindRequired},
		},
	})

	ticketStatusEnum := graphql.NewEnum(graphql.EnumConfig{
		Name: "TicketStatus",
		Values: graphql.EnumValueConfigMap{
			"OPEN":   &graphql.EnumValueConfig{Value: TicketStatusOpen},
			"CLOSED": &graphql.EnumValueConfig{Value: TicketStatusClosed},
		},
	})

	ticketUIEnum := graphql.NewEnum(graphql.EnumConfig{
		Name: "TicketUI",
		Values: graphql.EnumValueConfigMap{
			"COPILOT_CHAT":     &graphql.EnumValueConfig{Value: "copilot_chat"},
			"ANTIGRAVITY":      &graphql.EnumValueConfig{Value: "antigravity"},
			"ANTIGRAVITY_CHAT": &graphql.EnumValueConfig{Value: "antigravity-chat"},
			"CURSOR":           &graphql.EnumValueConfig{Value: "cursor"},
			"CURSOR_CHAT":      &graphql.EnumValueConfig{Value: "cursor-chat"},
			"VSCODE":           &graphql.EnumValueConfig{Value: "vscode"},
			"CLAUDE_CODE":      &graphql.EnumValueConfig{Value: "claude_code"},
			"CODEX":            &graphql.EnumValueConfig{Value: "codex"},
			"DROID":            &graphql.EnumValueConfig{Value: "droid"},
		},
	})

	violationPriorityEnum := graphql.NewEnum(graphql.EnumConfig{
		Name: "ViolationPriority",
		Values: graphql.EnumValueConfigMap{
			"HIGH":   &graphql.EnumValueConfig{Value: ViolationPriorityHigh},
			"MEDIUM": &graphql.EnumValueConfig{Value: ViolationPriorityMedium},
			"LOW":    &graphql.EnumValueConfig{Value: ViolationPriorityLow},
		},
	})

	var bundleType *graphql.Object
	var packageType *graphql.Object
	var projectType *graphql.Object
	var folderType *graphql.Object
	var fileType *graphql.Object
	var sectionType *graphql.Object
	var sectionItemInterface *graphql.Interface
	var definitionType *graphql.Object
	var violationType *graphql.Object
	var violationKindType *graphql.Object
	var policyType *graphql.Object
	var ticketType *graphql.Object
	var todoType *graphql.Object
	var locationType *graphql.Object
	var goalType *graphql.Object
	var draftType *graphql.Object
	var contributorType *graphql.Object
	var repoType *graphql.Object

	var contributorContributionsType *graphql.Object
	var ticketYearType *graphql.Object
	var ticketMonthType *graphql.Object
	var ticketDayType *graphql.Object
	var contributorBundleType *graphql.Object
	var contributorFolderType *graphql.Object
	var contributorFileType *graphql.Object
	var contributorSectionType *graphql.Object
	var contributorDefinitionType *graphql.Object
	var commitType *graphql.Object

	packageType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Package",
		Fields: graphql.Fields{
			"name":    &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"version": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"path":    &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"kind":    &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
		},
	})

	projectType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Project",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return p.Source.(*Project).GetID(), nil
					},
				},
				"name": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"root": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"kind": &graphql.Field{
					Type: graphql.NewNonNull(graphql.String),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return string(p.Source.(*Project).Kind), nil
					},
				},
				"bundles": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(bundleType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						bun := p.Source.(*Project).Bundles
						res := make([]*Bundle, len(bun))
						for i := range bun {
							res[i] = &bun[i]
						}
						return res, nil
					},
				},
				"uri": &graphql.Field{
					Type: graphql.NewNonNull(graphql.String),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return p.Source.(*Project).GetURI(), nil
					},
				},
			}
		}),
	})

	bundleType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Bundle",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						bundle := p.Source.(*Bundle)
						return bundle.GetID(), nil
					},
				},
				"name":        &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"root":        &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"sourceRoot":  &graphql.Field{Type: graphql.String},
				"projectType": &graphql.Field{Type: graphql.String},
				"tags":        &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(graphql.String)))},
				"packages": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(packageType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return p.Source.(*Bundle).Packages, nil
					},
				},
				"kind": &graphql.Field{
					Type: graphql.NewNonNull(graphql.String),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						bundle := p.Source.(*Bundle)
						return string(bundle.Kind), nil
					},
				},
				"uri": &graphql.Field{
					Type: graphql.NewNonNull(graphql.String),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						bundle := p.Source.(*Bundle)
						return "file://" + filepath.ToSlash(filepath.Join(rootDir, bundle.Root)), nil
					},
				},
				"folders": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(folderType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Folder{}, nil
					},
				},
				"files": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*File{}, nil
					},
				},
				"violations": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Violation{}, nil
					},
				},
			}
		}),
	})

	folderType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Folder",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":   &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"path": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"uri":  &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"name": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"kind": &graphql.Field{
					Type: graphql.NewNonNull(graphql.String),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						folder := p.Source.(*Folder)
						return string(folder.Kind), nil
					},
				},
				"parent": &graphql.Field{Type: folderType},
				"children": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(folderType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						folder, ok := p.Source.(*Folder)
						if !ok {
							return []*Folder{}, nil
						}
						return GetFolderChildren(folder.Path, folder.BundleID)
					},
				},
				"files": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						folder, ok := p.Source.(*Folder)
						if !ok {
							return []*File{}, nil
						}
						return GetFolderFiles(folder.Path, folder.BundleID)
					},
				},
				"ignored":   &graphql.Field{Type: graphql.NewNonNull(graphql.Boolean)},
				"generated": &graphql.Field{Type: graphql.NewNonNull(graphql.Boolean)},
				"bundle":    &graphql.Field{Type: bundleType},
				"violations": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Violation{}, nil
					},
				},
			}
		}),
	})

	fileType = graphql.NewObject(graphql.ObjectConfig{
		Name: "File",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":        &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"path":      &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"uri":       &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"name":      &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"extension": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"folder":    &graphql.Field{Type: folderType},
				"kind":      &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"ignored":   &graphql.Field{Type: graphql.NewNonNull(graphql.Boolean)},
				"generated": &graphql.Field{Type: graphql.NewNonNull(graphql.Boolean)},
				"bundle":    &graphql.Field{Type: bundleType},
				"sections": &graphql.Field{
					Type: graphql.NewList(sectionType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						file := p.Source.(*File)
						absPath := filepath.Join(rootDir, file.Path)
						if !FileExists(absPath) {
							return []*Section{}, nil
						}
						content, err := ReadTextFile(absPath)
						if err != nil {
							return nil, err
						}
						sections := ParseSections(content, file.Path)
						definitions := ParseDefinitions(content, file.Path)
						sections = HydrateSectionsWithDefinitions(sections, definitions)
						result := make([]*Section, len(sections))
						stack := make([]*Section, 0, len(sections))
						for i := range sections {
							sections[i].FilePath = file.Path
							sections[i].Path = sections[i].Name
							for j := range sections[i].Definitions {
								sections[i].Definitions[j].SectionPath = sections[i].Path
							}
							result[i] = &sections[i]
							if len(sections[i].Children) > 0 {
								stack = append(stack, result[i])
							}
						}
						for len(stack) > 0 {
							section := stack[len(stack)-1]
							stack = stack[:len(stack)-1]
							for i := range section.Children {
								child := &section.Children[i]
								child.FilePath = file.Path
								if section.Path == "" {
									child.Path = child.Name
								} else {
									child.Path = section.Path + "#" + child.Name
								}
								for j := range child.Definitions {
									child.Definitions[j].SectionPath = child.Path
								}
								if len(child.Children) > 0 {
									stack = append(stack, child)
								}
							}
						}
						return result, nil
					},
				},
				"definitions": &graphql.Field{
					Type: graphql.NewList(definitionType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Definition{}, nil
					},
				},
				"violations": &graphql.Field{
					Type: graphql.NewList(violationType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Violation{}, nil
					},
				},
				"content": &graphql.Field{Type: graphql.String},
				"contributors": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Contributor{}, nil
					},
				},
			}
		}),
	})

	sectionItemInterface = graphql.NewInterface(graphql.InterfaceConfig{
		Name: "SectionItem",
		Fields: graphql.Fields{
			"id":    &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
			"name":  &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"range": &graphql.Field{Type: rangeType},
		},
		ResolveType: func(p graphql.ResolveTypeParams) *graphql.Object {
			if _, ok := p.Value.(*Section); ok {
				return sectionType
			}
			if _, ok := p.Value.(*Definition); ok {
				return definitionType
			}
			return nil
		},
	})

	sectionType = graphql.NewObject(graphql.ObjectConfig{
		Name:       "Section",
		Interfaces: []*graphql.Interface{sectionItemInterface},
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						section := p.Source.(*Section)
						return section.GetID(), nil
					},
				},
				"name": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"path": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"file": &graphql.Field{
					Type: fileType,
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						section := p.Source.(*Section)
						if section.FilePath == "" {
							return nil, nil
						}
						normalizedPath := strings.ReplaceAll(section.FilePath, "\\", "/")
						name := filepath.Base(normalizedPath)
						ext := filepath.Ext(name)
						folderPath := filepath.Dir(normalizedPath)
						var folderID *string
						if folderPath != "." {
							// Use buildFolderID to get consistent ID
							id := buildFolderID(folderPath, nil)
							folderID = &id
						}
						return &File{
							ID:        buildFileID(normalizedPath, nil),
							Path:      normalizedPath,
							URI:       fmt.Sprintf("file://%s/%s", rootDir, normalizedPath),
							Name:      name,
							Extension: ext,
							FolderID:  folderID,
						}, nil
					},
				},
				"parent": &graphql.Field{Type: sectionType},
				"children": &graphql.Field{
					Type: graphql.NewList(sectionItemInterface),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						section := p.Source.(*Section)
						count := len(section.Children) + len(section.Definitions)
						if count == 0 {
							return []interface{}{}, nil
						}

						items := make([]interface{}, 0, count)
						for i := range section.Children {
							items = append(items, &section.Children[i])
						}
						for i := range section.Definitions {
							items = append(items, &section.Definitions[i])
						}

						sort.Slice(items, func(i, j int) bool {
							var startI, indexI int
							switch v := items[i].(type) {
							case *Section:
								startI = v.StartLine
								indexI = v.StartIndex
							case *Definition:
								startI = v.StartLine
								indexI = v.StartIndex
							}

							var startJ, indexJ int
							switch v := items[j].(type) {
							case *Section:
								startJ = v.StartLine
								indexJ = v.StartIndex
							case *Definition:
								startJ = v.StartLine
								indexJ = v.StartIndex
							}

							if startI != startJ {
								return startI < startJ
							}
							return indexI < indexJ
						})

						return items, nil
					},
				},
				"definitions": &graphql.Field{
					Type: graphql.NewList(definitionType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						section := p.Source.(*Section)
						if len(section.Definitions) == 0 {
							return []*Definition{}, nil
						}
						definitions := make([]*Definition, len(section.Definitions))
						for i := range section.Definitions {
							definitions[i] = &section.Definitions[i]
						}
						return definitions, nil
					},
				},
				"violations": &graphql.Field{
					Type: graphql.NewList(violationType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Violation{}, nil
					},
				},
				"range": &graphql.Field{
					Type: graphql.NewNonNull(rangeType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						section := p.Source.(*Section)
						return &Range{
							Start: Position{Line: section.StartLine, Column: 1},
							End:   Position{Line: section.EndLine, Column: 1},
						}, nil
					},
				},
			}
		}),
	})

	definitionType = graphql.NewObject(graphql.ObjectConfig{
		Name:       "Definition",
		Interfaces: []*graphql.Interface{sectionItemInterface},
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						definition := p.Source.(*Definition)
						return definition.GetID(), nil
					},
				},
				"name":    &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"kind":    &graphql.Field{Type: graphql.NewNonNull(definitionKindEnum)},
				"file":    &graphql.Field{Type: graphql.NewNonNull(fileType)},
				"section": &graphql.Field{Type: sectionType},
				"violations": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return []*Violation{}, nil
					},
				},
				"range": &graphql.Field{
					Type: graphql.NewNonNull(rangeType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						definition := p.Source.(*Definition)
						return &Range{
							Start: Position{Line: definition.StartLine, Column: 1},
							End:   Position{Line: definition.EndLine, Column: 1},
						}, nil
					},
				},
			}
		}),
	})

	violationType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Violation",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"kindId": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						violation := p.Source.(*Violation)
						return string(violation.Kind), nil
					},
				},
				"kind": &graphql.Field{
					Type: graphql.NewNonNull(violationKindType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						violation := p.Source.(*Violation)
						info := violation.Kind.Info()
						return &info, nil
					},
				},
				"scope":   &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"file":    &graphql.Field{Type: fileType},
				"folder":  &graphql.Field{Type: folderType},
				"line":    &graphql.Field{Type: graphql.Int},
				"column":  &graphql.Field{Type: graphql.Int},
				"excerpt": &graphql.Field{Type: graphql.String},
				"summary": &graphql.Field{
					Type: graphql.NewNonNull(graphql.String),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						violation := p.Source.(*Violation)
						return violation.Summary, nil
					},
				},
				"priority": &graphql.Field{
					Type: graphql.NewNonNull(violationPriorityEnum),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						violation := p.Source.(*Violation)
						return violation.Priority(), nil
					},
				},
				"autofixable": &graphql.Field{
					Type: graphql.NewNonNull(graphql.Boolean),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						violation := p.Source.(*Violation)
						return violation.Autofixable(), nil
					},
				},
			}
		}),
	})

	violationKindType = graphql.NewObject(graphql.ObjectConfig{
		Name: "ViolationKind",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						kind := p.Source.(*ViolationKindMeta)
						return kind.GetID(), nil
					},
				},
				"policy":      &graphql.Field{Type: graphql.NewNonNull(policyType)},
				"priority":    &graphql.Field{Type: graphql.NewNonNull(violationPriorityEnum)},
				"autofixable": &graphql.Field{Type: graphql.NewNonNull(graphql.Boolean)},
				"reason":      &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"solution":    &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"violations": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType))),
					Args: graphql.FieldConfigArgument{
						"scope": &graphql.ArgumentConfig{Type: graphql.String},
					},
				},
			}
		}),
	})

	policyType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Policy",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":             &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"name":           &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"description":    &graphql.Field{Type: graphql.String},
				"scopes":         &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(graphql.String)))},
				"violationKinds": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationKindType)))},
			}
		}),
	})

	ticketDateType := graphql.NewObject(graphql.ObjectConfig{
		Name: "TicketDate",
		Fields: graphql.Fields{
			"started":  &graphql.Field{Type: graphql.NewNonNull(graphql.DateTime)},
			"finished": &graphql.Field{Type: graphql.DateTime},
		},
	})

	iterationType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Iteration",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"prompt":   &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"commit":   &graphql.Field{Type: graphql.String},
				"started":  &graphql.Field{Type: graphql.NewNonNull(graphql.DateTime)},
				"finished": &graphql.Field{Type: graphql.DateTime},
				"plan":     &graphql.Field{Type: graphql.String},
				"llm":      &graphql.Field{Type: graphql.String},
				"contributor": &graphql.Field{
					Type: graphql.NewNonNull(contributorType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						iteration := p.Source.(TicketIteration)
						author := iteration.Author.Email
						if author == "" {
							author = iteration.Author.Name
						}
						contributors, err := ListContributors()
						if err != nil {
							return &Contributor{Github: author, Name: author, Emails: []string{author}}, nil
						}
						for i := range contributors {
							for _, email := range contributors[i].Emails {
								if email == author || strings.Contains(author, email) {
									return &contributors[i], nil
								}
							}
							if contributors[i].Name == author {
								return &contributors[i], nil
							}
						}
						return &Contributor{Github: author, Name: author, Emails: []string{author}}, nil
					},
				},
			}
		}),
	})

	goalType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Goal",
		Fields: graphql.Fields{
			"id":          &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
			"title":       &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"description": &graphql.Field{Type: graphql.String},
			"prompt":      &graphql.Field{Type: graphql.String},
			"dueDate": &graphql.Field{
				Type: graphql.String,
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					goal := p.Source.(*Goal)
					return goal.Dates.Due, nil
				},
			},
			"createdAt": &graphql.Field{
				Type: graphql.String,
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					goal := p.Source.(*Goal)
					if len(goal.Iterations) > 0 {
						return goal.Iterations[0].Started, nil
					}
					return nil, nil
				},
			},
			"ui": &graphql.Field{
				Type: graphql.String,
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					goal := p.Source.(*Goal)
					return goal.UI, nil
				},
			},
			"llm":    &graphql.Field{Type: graphql.String},
			"status": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"milestone": &graphql.Field{
				Type: graphql.Int,
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					goal := p.Source.(*Goal)
					if goal.GitHub == nil {
						return nil, nil
					}
					return parseMilestoneNumber(goal.GitHub.Milestone)
				},
			},
			"parent": &graphql.Field{Type: graphql.String},
		},
	})

	draftType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Draft",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						draft := p.Source.(*Draft)
						return draft.ID, nil
					},
				},
			}
		}),
	})

	ticketType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Ticket",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						return ticket.GetID(), nil
					},
				},
				"year":  &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"month": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"day":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"slug":  &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"path": &graphql.Field{
					Type: graphql.NewNonNull(graphql.String),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						if ticket.TicketPath != "" {
							return ticket.TicketPath, nil
						}
						return ticket.JsonPath, nil
					},
				},
				"llm": &graphql.Field{
					Type: graphql.String,
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						llm := ticket.GetLLM()
						if llm == "" {
							return nil, nil
						}
						return llm, nil
					},
				},
				"client": &graphql.Field{
					Type: ticketUIEnum,
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						ui := ticket.GetUI()
						if ui == "" {
							return nil, nil
						}
						return ui, nil
					},
				},
				"commit": &graphql.Field{
					Type: graphql.String,
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						commit := ticket.GetCommit()
						if commit == "" {
							return nil, nil
						}
						return commit, nil
					},
				},
				"uri": &graphql.Field{
					Type: graphql.NewNonNull(graphql.String),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						path := ticket.JsonPath
						if ticket.TicketPath != "" {
							path = ticket.TicketPath
						}
						absPath := filepath.Join(rootDir, path)
						return "file://" + strings.ReplaceAll(absPath, "\\", "/"), nil
					},
				},
				"title": &graphql.Field{
					Type: graphql.NewNonNull(graphql.String),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						return ticket.GetTitle(), nil
					},
				},
				"prompt": &graphql.Field{
					Type: graphql.NewNonNull(graphql.String),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						return ticket.GetPrompt(), nil
					},
				},
				"summary": &graphql.Field{
					Type: graphql.String,
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return nil, nil
					},
				},
				"status": &graphql.Field{
					Type: graphql.NewNonNull(ticketStatusEnum),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						status := ticket.GetStatus()
						if status == "" || status == "open" {
							return TicketStatusOpen, nil
						}
						if status == "finished" || status == "closed" {
							return TicketStatusClosed, nil
						}
						return TicketStatusOpen, nil
					},
				},
				"iterations": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(iterationType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						return ticket.Iterations, nil
					},
				},
				"author": &graphql.Field{
					Type: contributorType,
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						if len(ticket.Iterations) == 0 {
							return nil, nil
						}
						iteration := ticket.Iterations[len(ticket.Iterations)-1]
						author := iteration.Author.Email
						if author == "" {
							author = iteration.Author.Name
						}
						contributors, err := ListContributors()
						if err != nil {
							return &Contributor{Github: author, Name: author, Emails: []string{author}}, nil
						}
						for i := range contributors {
							for _, email := range contributors[i].Emails {
								if email == author || strings.Contains(author, email) {
									return &contributors[i], nil
								}
							}
							if contributors[i].Name == author {
								return &contributors[i], nil
							}
						}
						return &Contributor{Github: author, Name: author, Emails: []string{author}}, nil
					},
				},
				"dates": &graphql.Field{
					Type: graphql.NewNonNull(ticketDateType),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						started := ticket.GetDateStarted()
						if started.IsZero() {
							started = time.Date(ticket.Year, time.Month(ticket.Month), ticket.Day, 0, 0, 0, 0, time.UTC)
						}
						finished := ticket.GetDateFinished()
						return map[string]interface{}{
							"started":  started,
							"finished": finished,
						}, nil
					},
				},
				"goal": &graphql.Field{
					Type: graphql.String,
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						if ticket.Goal == "" {
							return nil, nil
						}
						return ticket.Goal, nil
					},
				},
				"parent": &graphql.Field{
					Type: graphql.String,
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						ticket := p.Source.(*Ticket)
						if ticket.Parent == "" {
							return nil, nil
						}
						return ticket.Parent, nil
					},
				},
				"bundles": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(bundleType)))},
				"files":   &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType)))},
			}
		}),
	})

	contributorIconsType := graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorIcons",
		Fields: graphql.Fields{
			"avatar":      &graphql.Field{Type: graphql.String},
			"avatarRound": &graphql.Field{Type: graphql.String},
			"github":      &graphql.Field{Type: graphql.String},
		},
	})

	locationType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Location",
		Fields: graphql.Fields{
			"filePath": &graphql.Field{Type: graphql.String},
			"line":     &graphql.Field{Type: graphql.Int},
			"column":   &graphql.Field{Type: graphql.Int},
		},
	})

	todoType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Todo",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return p.Source.(*Todo).GetID(), nil
					},
				},
				"name":        &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"description": &graphql.Field{Type: graphql.String},
				"parentId": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						return p.Source.(*Todo).ParentID, nil
					},
				},
				"location": &graphql.Field{Type: locationType},
			}
		}),
	})

	_ = todoType
	contributorDefinitionType = graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorDefinition",
		Fields: graphql.Fields{
			"name": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
		},
	})

	contributorSectionType = graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorSection",
		Fields: graphql.Fields{
			"name":        &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"definitions": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorDefinitionType)))},
		},
	})

	contributorFileType = graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorFile",
		Fields: graphql.Fields{
			"name":     &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"sections": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorSectionType)))},
		},
	})

	contributorFolderType = graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorFolder",
		Fields: graphql.Fields{
			"name":  &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"files": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorFileType)))},
		},
	})

	contributorBundleType = graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorBundle",
		Fields: graphql.Fields{
			"name":    &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"folders": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorFolderType)))},
		},
	})

	ticketDayType = graphql.NewObject(graphql.ObjectConfig{
		Name: "TicketDay",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"day":     &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"tickets": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(ticketType)))},
			}
		}),
	})

	ticketMonthType = graphql.NewObject(graphql.ObjectConfig{
		Name: "TicketMonth",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"month": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"days":  &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(ticketDayType)))},
			}
		}),
	})

	ticketYearType = graphql.NewObject(graphql.ObjectConfig{
		Name: "TicketYear",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"year":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"months": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(ticketMonthType)))},
			}
		}),
	})

	contributorContributionsType = graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorContributions",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"commits": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(commitType)))},
				"tickets": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(ticketYearType)))},
				"bundles": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorBundleType)))},
			}
		}),
	})

	commitType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Commit",
		Fields: graphql.Fields{
			"id":    &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
			"sha":   &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"title": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"date":  &graphql.Field{Type: graphql.NewNonNull(graphql.DateTime)},
		},
	})

	contributorDefinitionType = graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorDefinition",
		Fields: graphql.Fields{
			"name": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
		},
	})

	contributorSectionType = graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorSection",
		Fields: graphql.Fields{
			"name":        &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"definitions": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorDefinitionType)))},
		},
	})

	contributorFileType = graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorFile",
		Fields: graphql.Fields{
			"name":     &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"sections": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorSectionType)))},
		},
	})

	contributorFolderType = graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorFolder",
		Fields: graphql.Fields{
			"name":  &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"files": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorFileType)))},
		},
	})

	contributorBundleType = graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorBundle",
		Fields: graphql.Fields{
			"name":    &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"folders": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorFolderType)))},
		},
	})

	ticketDayType = graphql.NewObject(graphql.ObjectConfig{
		Name: "TicketDay",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"day":     &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"tickets": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(ticketType)))},
			}
		}),
	})

	ticketMonthType = graphql.NewObject(graphql.ObjectConfig{
		Name: "TicketMonth",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"month": &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"days":  &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(ticketDayType)))},
			}
		}),
	})

	ticketYearType = graphql.NewObject(graphql.ObjectConfig{
		Name: "TicketYear",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"year":   &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
				"months": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(ticketMonthType)))},
			}
		}),
	})

	contributorContributionsType = graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorContributions",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"commits": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(commitType)))},
				"tickets": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(ticketYearType)))},
				"bundles": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorBundleType)))},
			}
		}),
	})

	contributorLinkType := graphql.NewObject(graphql.ObjectConfig{
		Name: "ContributorLink",
		Fields: graphql.Fields{
			"name": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
			"url":  &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
		},
	})

	contributorType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Contributor",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id": &graphql.Field{
					Type: graphql.NewNonNull(graphql.ID),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						contributor := p.Source.(*Contributor)
						return contributor.GetID(), nil
					},
				},
				"github": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"name":   &graphql.Field{Type: graphql.String},
				"emails": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(graphql.String)))},
				"links": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorLinkType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						contributor := p.Source.(*Contributor)
						links := []ContributorLink{}
						for name, url := range contributor.Links {
							links = append(links, ContributorLink{Name: name, URL: url})
						}
						// Sort links by name for consistency
						sort.Slice(links, func(i, j int) bool {
							return links[i].Name < links[j].Name
						})
						return links, nil
					},
				},
				"contributions": &graphql.Field{
					Type: contributorContributionsType,
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						contributor := p.Source.(*Contributor)
						// Fetch all tickets
						tickets, err := repoResolverInstance.Ctx.GetTickets(nil, nil, nil, nil)
						if err != nil {
							return nil, err
						}

						// Filter tickets by author
						userTickets := []*Ticket{}
						for _, t := range tickets {
							if strings.EqualFold(t.GetAuthor(), contributor.Github) {
								userTickets = append(userTickets, t)
							}
						}

						// Sort filtered tickets by date (descending)
						sort.Slice(userTickets, func(i, j int) bool {
							return userTickets[i].GetDateStarted().After(userTickets[j].GetDateStarted())
						})

						return ResolveContributorContributions(userTickets), nil
					},
				},
				"icons":   &graphql.Field{Type: contributorIconsType},
				"bundles": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(bundleType)))},
				"files":   &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType)))},
				"tickets": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(ticketType)))},
			}
		}),
	})

	repoType = graphql.NewObject(graphql.ObjectConfig{
		Name: "Repo",
		Fields: (graphql.FieldsThunk)(func() graphql.Fields {
			return graphql.Fields{
				"id":   &graphql.Field{Type: graphql.NewNonNull(graphql.ID)},
				"name": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"path": &graphql.Field{Type: graphql.NewNonNull(graphql.String)},
				"projects": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(projectType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						projects := LoadProjects()
						res := make([]*Project, len(projects))
						for i := range projects {
							res[i] = &projects[i]
						}
						return res, nil
					},
				},
				"commits": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(commitType))),
					Args: graphql.FieldConfigArgument{
						"limit": &graphql.ArgumentConfig{Type: graphql.Int},
					},
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						var limit *int
						if v, ok := p.Args["limit"].(int); ok {
							limit = &v
						}
						return repoResolverInstance.Ctx.GetCommits(limit)
					},
				},
				"bundles": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(bundleType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						return repoResolverInstance.Ctx.GetBundles(), nil
					},
				},
				"folders": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(folderType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						return repoResolverInstance.Ctx.GetFolders(), nil
					},
				},
				"files": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						return repoResolverInstance.Ctx.GetFiles(), nil
					},
				},
				"sections": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(sectionType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						return repoResolverInstance.Ctx.GetSections(), nil
					},
				},
				"definitions": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(definitionType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						return repoResolverInstance.Ctx.GetDefinitions(), nil
					},
				},
				"contributors": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						return repoResolverInstance.Ctx.GetContributors()
					},
				},
				"goals": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(goalType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						return repoResolverInstance.Ctx.GetGoals()
					},
				},
				"tickets": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(ticketType))),
					Args: graphql.FieldConfigArgument{
						"year":   &graphql.ArgumentConfig{Type: graphql.Int},
						"month":  &graphql.ArgumentConfig{Type: graphql.Int},
						"day":    &graphql.ArgumentConfig{Type: graphql.Int},
						"status": &graphql.ArgumentConfig{Type: ticketStatusEnum},
					},
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						var year, month, day *int
						var status *TicketStatus
						if v, ok := p.Args["year"].(int); ok {
							year = &v
						}
						if v, ok := p.Args["month"].(int); ok {
							month = &v
						}
						if v, ok := p.Args["day"].(int); ok {
							day = &v
						}
						if v, ok := p.Args["status"].(TicketStatus); ok {
							status = &v
						}
						return repoResolverInstance.Ctx.GetTickets(year, month, day, status)
					},
				},
				"policies": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(policyType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						return repoResolverInstance.Ctx.GetPolicies(), nil
					},
				},
				"violationKinds": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationKindType))),
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						_ = p.Source.(*Repo)
						return repoResolverInstance.Ctx.GetViolationKinds(), nil
					},
				},
				"violations": &graphql.Field{
					Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType))),
					Args: graphql.FieldConfigArgument{
						"scope": &graphql.ArgumentConfig{Type: graphql.String},
					},
					Resolve: func(p graphql.ResolveParams) (interface{}, error) {
						repo := p.Source.(*Repo)
						var scope *string
						if v, ok := p.Args["scope"].(string); ok {
							scope = &v
						}
						return repoResolverInstance.Violations(p.Context, repo, scope)
					},
				},
			}
		}),
	})

	analyzeResultType := graphql.NewObject(graphql.ObjectConfig{
		Name: "AnalyzeResult",
		Fields: graphql.Fields{
			"violations": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType)))},
			"metrics":    &graphql.Field{Type: graphql.NewNonNull(analyzeMetricsType)},
		},
	})

	fixResultType := graphql.NewObject(graphql.ObjectConfig{
		Name: "FixResult",
		Fields: graphql.Fields{
			"fixed":      &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"remaining":  &graphql.Field{Type: graphql.NewNonNull(graphql.Int)},
			"violations": &graphql.Field{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType)))},
		},
	})

	queryResolverInstance := &queryResolver{resolver}

	fileKindEnum := graphql.NewEnum(graphql.EnumConfig{
		Name: "FileKind",
		Values: graphql.EnumValueConfigMap{
			"CODE":     &graphql.EnumValueConfig{Value: FileKindCode},
			"SCRIPT":   &graphql.EnumValueConfig{Value: FileKindScript},
			"CONFIG":   &graphql.EnumValueConfig{Value: FileKindConfig},
			"TEST":     &graphql.EnumValueConfig{Value: FileKindTest},
			"DOCS":     &graphql.EnumValueConfig{Value: FileKindDocs},
			"RESOURCE": &graphql.EnumValueConfig{Value: FileKindResource},
			"LICENSE":  &graphql.EnumValueConfig{Value: FileKindLicense},
		},
	})

	filterInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "FilterInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"filter":         &graphql.InputObjectFieldConfig{Type: graphql.String},
			"regex":          &graphql.InputObjectFieldConfig{Type: graphql.Boolean},
			"matchCase":      &graphql.InputObjectFieldConfig{Type: graphql.Boolean},
			"matchWholeWord": &graphql.InputObjectFieldConfig{Type: graphql.Boolean},
			"showIgnored":    &graphql.InputObjectFieldConfig{Type: graphql.Boolean},
			"showGenerated":  &graphql.InputObjectFieldConfig{Type: graphql.Boolean},
			"excludeKinds":   &graphql.InputObjectFieldConfig{Type: graphql.NewList(fileKindEnum)},
			"includeKinds":   &graphql.InputObjectFieldConfig{Type: graphql.NewList(fileKindEnum)},
		},
	})

	parseFilterInput := func(args map[string]interface{}) *FilterInput {
		filterArg, ok := args["filter"]
		if !ok || filterArg == nil {
			return nil
		}
		filterMap, ok := filterArg.(map[string]interface{})
		if !ok {
			return nil
		}
		input := &FilterInput{}
		if v, ok := filterMap["filter"].(string); ok {
			input.Filter = &v
		}
		if v, ok := filterMap["regex"].(bool); ok {
			input.Regex = &v
		}
		if v, ok := filterMap["matchCase"].(bool); ok {
			input.MatchCase = &v
		}
		if v, ok := filterMap["matchWholeWord"].(bool); ok {
			input.MatchWholeWord = &v
		}
		if v, ok := filterMap["showIgnored"].(bool); ok {
			input.ShowIgnored = &v
		}
		if v, ok := filterMap["showGenerated"].(bool); ok {
			input.ShowGenerated = &v
		}
		if v, ok := filterMap["excludeKinds"].([]interface{}); ok {
			for _, k := range v {
				if s, ok := k.(string); ok {
					input.ExcludeKinds = append(input.ExcludeKinds, s)
				}
			}
		}
		if v, ok := filterMap["includeKinds"].([]interface{}); ok {
			for _, k := range v {
				if s, ok := k.(string); ok {
					input.IncludeKinds = append(input.IncludeKinds, s)
				}
			}
		}
		return input
	}
	_ = filterInputType
	_ = parseFilterInput

	queryType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Query",
		Fields: graphql.Fields{
			"node": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewUnion(graphql.UnionConfig{
					Name:  "Node",
					Types: []*graphql.Object{repoType, bundleType, folderType, fileType, sectionType, definitionType, contributorType, ticketType, policyType, violationKindType, violationType, draftType},
					ResolveType: func(p graphql.ResolveTypeParams) *graphql.Object {
						switch p.Value.(type) {
						case *Draft:
							return draftType
						case *Repo:
							return repoType
						case *Bundle:
							return bundleType
						case *Folder:
							return folderType
						case *File:
							return fileType
						case *Section:
							return sectionType
						case *Definition:
							return definitionType
						case *Contributor:
							return contributorType
						case *Ticket:
							return ticketType
						case *Policy:
							return policyType
						case *ViolationKindMeta:
							return violationKindType
						case *Violation:
							return violationType
						default:
							return nil
						}
					},
				})),
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.ID)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					id := p.Args["id"].(string)
					return queryResolverInstance.Node(p.Context, id)
				},
			},
			"repo": &graphql.Field{
				Type: graphql.NewNonNull(repoType),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return queryResolverInstance.Repo(p.Context)
				},
			},
			"projects": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(projectType))),
				Args: graphql.FieldConfigArgument{
					"filter": &graphql.ArgumentConfig{Type: filterInputType},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					filter := parseFilterInput(p.Args)
					return queryResolverInstance.Projects(p.Context, filter)
				},
			},
			"bundles": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(bundleType))),
				Args: graphql.FieldConfigArgument{
					"filter": &graphql.ArgumentConfig{Type: filterInputType},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					filter := parseFilterInput(p.Args)
					return queryResolverInstance.Bundles(p.Context, filter)
				},
			},
			"folders": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(folderType))),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return queryResolverInstance.Folders(p.Context)
				},
			},
			"files": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(fileType))),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return queryResolverInstance.Files(p.Context)
				},
			},
			"sections": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(sectionType))),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return queryResolverInstance.Sections(p.Context)
				},
			},
			"definitions": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(definitionType))),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return queryResolverInstance.Definitions(p.Context)
				},
			},
			"contributors": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(contributorType))),
				Args: graphql.FieldConfigArgument{
					"filter": &graphql.ArgumentConfig{Type: filterInputType},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					filter := parseFilterInput(p.Args)
					return queryResolverInstance.Contributors(p.Context, filter)
				},
			},
			"todos": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(todoType))),
				Args: graphql.FieldConfigArgument{
					"filter": &graphql.ArgumentConfig{Type: filterInputType},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					filter := parseFilterInput(p.Args)
					return queryResolverInstance.Todos(p.Context, filter)
				},
			},
			"tickets": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(ticketType))),
				Args: graphql.FieldConfigArgument{
					"year":   &graphql.ArgumentConfig{Type: graphql.Int},
					"month":  &graphql.ArgumentConfig{Type: graphql.Int},
					"day":    &graphql.ArgumentConfig{Type: graphql.Int},
					"status": &graphql.ArgumentConfig{Type: ticketStatusEnum},
					"filter": &graphql.ArgumentConfig{Type: filterInputType},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					var year, month, day *int
					var status *TicketStatus
					if y, ok := p.Args["year"].(int); ok {
						year = &y
					}
					if m, ok := p.Args["month"].(int); ok {
						month = &m
					}
					if d, ok := p.Args["day"].(int); ok {
						day = &d
					}
					if s, ok := p.Args["status"].(TicketStatus); ok {
						status = &s
					}
					filter := parseFilterInput(p.Args)
					return queryResolverInstance.Tickets(p.Context, year, month, day, status, filter)
				},
			},
			"drafts": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(draftType))),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return queryResolverInstance.Drafts(p.Context)
				},
			},
			"policies": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(policyType))),
				Args: graphql.FieldConfigArgument{
					"filter": &graphql.ArgumentConfig{Type: filterInputType},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					filter := parseFilterInput(p.Args)
					return queryResolverInstance.Policies(p.Context, filter)
				},
			},
			"violationKinds": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationKindType))),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return queryResolverInstance.ViolationKinds(p.Context)
				},
			},
			"violations": &graphql.Field{
				Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(violationType))),
				Args: graphql.FieldConfigArgument{
					"scope": &graphql.ArgumentConfig{Type: graphql.String},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					var scope *string
					if s, ok := p.Args["scope"].(string); ok {
						scope = &s
					}
					return queryResolverInstance.Violations(p.Context, scope)
				},
			},
			"bundle": &graphql.Field{
				Type: bundleType,
				Args: graphql.FieldConfigArgument{
					"name": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					name := p.Args["name"].(string)
					return queryResolverInstance.Bundle(p.Context, name)
				},
			},
			"folder": &graphql.Field{
				Type: folderType,
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return queryResolverInstance.Folder(p.Context, path)
				},
			},
			"file": &graphql.Field{
				Type: fileType,
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return queryResolverInstance.File(p.Context, path)
				},
			},
			"section": &graphql.Field{
				Type: sectionType,
				Args: graphql.FieldConfigArgument{
					"path":        &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"sectionPath": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.NewList(graphql.NewNonNull(graphql.String)))},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					sectionPathRaw := p.Args["sectionPath"].([]interface{})
					sectionPath := make([]string, len(sectionPathRaw))
					for i, v := range sectionPathRaw {
						sectionPath[i] = v.(string)
					}
					return queryResolverInstance.Section(p.Context, path, sectionPath)
				},
			},
			"definition": &graphql.Field{
				Type: definitionType,
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"name": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					name := p.Args["name"].(string)
					return queryResolverInstance.Definition(p.Context, path, name)
				},
			},
			"contributor": &graphql.Field{
				Type: contributorType,
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					id := p.Args["id"].(string)
					return queryResolverInstance.Contributor(p.Context, id)
				},
			},
			"ticket": &graphql.Field{
				Type: ticketType,
				Args: graphql.FieldConfigArgument{
					"year":  &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.Int)},
					"month": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.Int)},
					"day":   &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.Int)},
					"slug":  &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					year := p.Args["year"].(int)
					month := p.Args["month"].(int)
					day := p.Args["day"].(int)
					slug := p.Args["slug"].(string)
					return queryResolverInstance.Ticket(p.Context, year, month, day, slug)
				},
			},
			"policy": &graphql.Field{
				Type: policyType,
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					id := p.Args["id"].(string)
					return queryResolverInstance.Policy(p.Context, id)
				},
			},
			"violationKind": &graphql.Field{
				Type: violationKindType,
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					id := p.Args["id"].(string)
					return queryResolverInstance.ViolationKind(p.Context, id)
				},
			},
			"analyze": &graphql.Field{
				Type: graphql.NewNonNull(analyzeResultType),
				Args: graphql.FieldConfigArgument{
					"scope": &graphql.ArgumentConfig{Type: graphql.String},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					var scope *string
					if s, ok := p.Args["scope"].(string); ok {
						scope = &s
					}
					return queryResolverInstance.Analyze(p.Context, scope)
				},
			},
		},
	})

	mutationResolverInstance := &mutationResolver{resolver}

	draftCreateInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "DraftCreateInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"slug":  &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"files": &graphql.InputObjectFieldConfig{Type: graphql.NewList(graphql.NewNonNull(graphql.String))},
		},
	})

	ticketOpenInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "TicketOpenInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"title":    &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"prompt":   &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"llm":      &graphql.InputObjectFieldConfig{Type: graphql.String},
			"client":   &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(ticketUIEnum)},
			"noIssue":  &graphql.InputObjectFieldConfig{Type: graphql.Boolean},
			"draft":    &graphql.InputObjectFieldConfig{Type: graphql.String},
			"goal":     &graphql.InputObjectFieldConfig{Type: graphql.String},
			"parent":   &graphql.InputObjectFieldConfig{Type: graphql.String},
			"noGithub": &graphql.InputObjectFieldConfig{Type: graphql.Boolean},
			"issue":    &graphql.InputObjectFieldConfig{Type: graphql.String},
		},
	})

	ticketCloseInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "TicketCloseInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"year":     &graphql.InputObjectFieldConfig{Type: graphql.Int},
			"month":    &graphql.InputObjectFieldConfig{Type: graphql.Int},
			"day":      &graphql.InputObjectFieldConfig{Type: graphql.Int},
			"slug":     &graphql.InputObjectFieldConfig{Type: graphql.String},
			"summary":  &graphql.InputObjectFieldConfig{Type: graphql.String},
			"files":    &graphql.InputObjectFieldConfig{Type: graphql.NewList(graphql.NewNonNull(graphql.String))},
			"title":    &graphql.InputObjectFieldConfig{Type: graphql.String},
			"noGithub": &graphql.InputObjectFieldConfig{Type: graphql.Boolean},
			"all":      &graphql.InputObjectFieldConfig{Type: graphql.Boolean},
		},
	})

	ticketReopenInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "TicketReopenInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"year":     &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"month":    &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"day":      &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"slug":     &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"prompt":   &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"client":   &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(ticketUIEnum)},
			"llm":      &graphql.InputObjectFieldConfig{Type: graphql.String},
			"title":    &graphql.InputObjectFieldConfig{Type: graphql.String},
			"draft":    &graphql.InputObjectFieldConfig{Type: graphql.String},
			"goal":     &graphql.InputObjectFieldConfig{Type: graphql.String},
			"parent":   &graphql.InputObjectFieldConfig{Type: graphql.String},
			"noGithub": &graphql.InputObjectFieldConfig{Type: graphql.Boolean},
		},
	})

	ticketChangeInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "TicketChangeInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"year":     &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"month":    &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"day":      &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.Int)},
			"slug":     &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"title":    &graphql.InputObjectFieldConfig{Type: graphql.String},
			"prompt":   &graphql.InputObjectFieldConfig{Type: graphql.String},
			"llm":      &graphql.InputObjectFieldConfig{Type: graphql.String},
			"client":   &graphql.InputObjectFieldConfig{Type: ticketUIEnum},
			"goal":     &graphql.InputObjectFieldConfig{Type: graphql.String},
			"parent":   &graphql.InputObjectFieldConfig{Type: graphql.String},
			"noGithub": &graphql.InputObjectFieldConfig{Type: graphql.Boolean},
		},
	})

	todoCreateInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "TodoCreateInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"name":        &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"description": &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"parentID":    &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
		},
	})

	todoChangeInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "TodoChangeInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"id":          &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"name":        &graphql.InputObjectFieldConfig{Type: graphql.String},
			"description": &graphql.InputObjectFieldConfig{Type: graphql.String},
		},
	})

	goalCreateInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "GoalCreateInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"title":       &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"description": &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"prompt":      &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"dueDate":     &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"llm":         &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"client":      &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"parent":      &graphql.InputObjectFieldConfig{Type: graphql.String},
			"noGithub":    &graphql.InputObjectFieldConfig{Type: graphql.Boolean},
			"milestone":   &graphql.InputObjectFieldConfig{Type: graphql.String},
		},
	})

	goalChangeInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "GoalChangeInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"id":          &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"title":       &graphql.InputObjectFieldConfig{Type: graphql.String},
			"description": &graphql.InputObjectFieldConfig{Type: graphql.String},
			"dueDate":     &graphql.InputObjectFieldConfig{Type: graphql.String},
			"parent":      &graphql.InputObjectFieldConfig{Type: graphql.String},
			"noGithub":    &graphql.InputObjectFieldConfig{Type: graphql.Boolean},
		},
	})

	goalCloseInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "GoalCloseInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"id":       &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"summary":  &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"noGithub": &graphql.InputObjectFieldConfig{Type: graphql.Boolean},
		},
	})

	goalReopenInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "GoalReopenInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"id":          &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"prompt":      &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"client":      &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"llm":         &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"title":       &graphql.InputObjectFieldConfig{Type: graphql.String},
			"description": &graphql.InputObjectFieldConfig{Type: graphql.String},
			"dueDate":     &graphql.InputObjectFieldConfig{Type: graphql.String},
			"parent":      &graphql.InputObjectFieldConfig{Type: graphql.String},
			"noGithub":    &graphql.InputObjectFieldConfig{Type: graphql.Boolean},
		},
	})

	contributorAddInputType := graphql.NewInputObject(graphql.InputObjectConfig{
		Name: "ContributorAddInput",
		Fields: graphql.InputObjectConfigFieldMap{
			"github": &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"name":   &graphql.InputObjectFieldConfig{Type: graphql.NewNonNull(graphql.String)},
			"emails": &graphql.InputObjectFieldConfig{Type: graphql.NewList(graphql.NewNonNull(graphql.String))},
		},
	})

	mutationType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Mutation",
		Fields: graphql.Fields{
			"goalCreate": &graphql.Field{
				Type: goalType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(goalCreateInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := GoalCreateInput{
						Title:       inputMap["title"].(string),
						Description: inputMap["description"].(string),
					}
					if s, ok := inputMap["prompt"].(string); ok {
						input.Prompt = s
					}
					if s, ok := inputMap["dueDate"].(string); ok {
						input.DueDate = s
					}
					if s, ok := inputMap["client"].(string); ok {
						input.UI = s
					}
					if s, ok := inputMap["llm"].(string); ok {
						input.LLM = s
					}
					if s, ok := inputMap["parent"].(string); ok {
						input.Parent = s
					}
					if inputMap["noGithub"] != nil {
						input.NoGithub = inputMap["noGithub"].(bool)
					}
					return mutationResolverInstance.GoalCreate(p.Context, input)
				},
			},
			"goalChange": &graphql.Field{
				Type: goalType,
				Args: graphql.FieldConfigArgument{
					"id":    &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.ID)},
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(goalChangeInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					id := p.Args["id"].(string)
					inputMap := p.Args["input"].(map[string]interface{})
					input := GoalChangeInput{
						ID: id,
					}
					if s, ok := inputMap["title"].(string); ok {
						input.Title = &s
					}
					if s, ok := inputMap["description"].(string); ok {
						input.Description = &s
					}
					if s, ok := inputMap["dueDate"].(string); ok {
						input.DueDate = &s
					}
					if s, ok := inputMap["parent"].(string); ok {
						input.Parent = &s
					}
					if b, ok := inputMap["noGithub"].(bool); ok {
						input.NoGithub = b
					}
					return mutationResolverInstance.GoalChange(p.Context, id, input)
				},
			},
			"goalClose": &graphql.Field{
				Type: goalType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(goalCloseInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := GoalCloseInput{
						ID:      inputMap["id"].(string),
						Summary: inputMap["summary"].(string),
					}
					if inputMap["noGithub"] != nil {
						input.NoGithub = inputMap["noGithub"].(bool)
					}
					return mutationResolverInstance.GoalClose(p.Context, input)
				},
			},
			"goalReopen": &graphql.Field{
				Type: goalType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(goalReopenInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := GoalReopenInput{
						ID:     inputMap["id"].(string),
						Prompt: inputMap["prompt"].(string),
						UI:     inputMap["client"].(string),
						LLM:    inputMap["llm"].(string),
					}
					if s, ok := inputMap["title"].(string); ok {
						input.Title = &s
					}
					if s, ok := inputMap["description"].(string); ok {
						input.Description = &s
					}
					if s, ok := inputMap["dueDate"].(string); ok {
						input.DueDate = &s
					}
					if s, ok := inputMap["parent"].(string); ok {
						input.Parent = &s
					}
					if inputMap["noGithub"] != nil {
						input.NoGithub = inputMap["noGithub"].(bool)
					}
					return mutationResolverInstance.GoalReopen(p.Context, input)
				},
			},
			"fix": &graphql.Field{
				Type: graphql.NewNonNull(fixResultType),
				Args: graphql.FieldConfigArgument{
					"scope": &graphql.ArgumentConfig{Type: graphql.String},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					var scope *string
					if s, ok := p.Args["scope"].(string); ok {
						scope = &s
					}
					return mutationResolverInstance.Fix(p.Context, scope)
				},
			},
			"draftCreate": &graphql.Field{
				Type: draftType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(draftCreateInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := DraftCreateInput{
						Slug: inputMap["slug"].(string),
					}
					if files, ok := inputMap["files"].([]interface{}); ok {
						for _, f := range files {
							if s, ok := f.(string); ok {
								input.Files = append(input.Files, s)
							}
						}
					}
					return mutationResolverInstance.DraftCreate(p.Context, input)
				},
			},
			"draftDelete": &graphql.Field{
				Type: graphql.NewNonNull(graphql.Boolean),
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					id := p.Args["id"].(string)
					return mutationResolverInstance.DraftDelete(p.Context, id)
				},
			},
			"todoCreate": &graphql.Field{
				Type: todoType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(todoCreateInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := TodoCreateInput{
						ParentID: inputMap["parentId"].(string),
						Name:     inputMap["name"].(string),
					}
					if s, ok := inputMap["description"].(string); ok {
						input.Description = s
					}
					return mutationResolverInstance.TodoCreate(p.Context, input)
				},
			},
			"todoChange": &graphql.Field{
				Type: todoType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(todoChangeInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := TodoChangeInput{
						ID: inputMap["id"].(string),
					}
					if s, ok := inputMap["name"].(string); ok {
						input.Name = &s
					}
					if s, ok := inputMap["description"].(string); ok {
						input.Description = &s
					}
					return mutationResolverInstance.TodoChange(p.Context, input)
				},
			},
			"todoDelete": &graphql.Field{
				Type: graphql.Boolean,
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.ID)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					id := p.Args["id"].(string)
					return mutationResolverInstance.TodoDelete(p.Context, id)
				},
			},
			"ticketOpen": &graphql.Field{
				Type: ticketType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(ticketOpenInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := TicketOpenInput{
						Title:  inputMap["title"].(string),
						Prompt: inputMap["prompt"].(string),
						UI:     inputMap["client"].(string),
					}
					if inputMap["llm"] != nil {
						input.LLM = inputMap["llm"].(string)
					}
					if inputMap["noIssue"] != nil {
						input.NoIssue = inputMap["noIssue"].(bool)
					}
					if inputMap["noGithub"] != nil {
						input.NoGithub = inputMap["noGithub"].(bool)
					}
					if inputMap["draft"] != nil {
						input.Draft = inputMap["draft"].(string)
					}
					if inputMap["goal"] != nil {
						input.Goal = inputMap["goal"].(string)
					}
					if inputMap["parent"] != nil {
						input.Parent = inputMap["parent"].(string)
					}
					return mutationResolverInstance.TicketOpen(p.Context, input)
				},
			},
			"ticketClose": &graphql.Field{
				Type: ticketType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(ticketCloseInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					var files []string
					if filesRaw, ok := inputMap["files"].([]interface{}); ok {
						for _, f := range filesRaw {
							if s, ok := f.(string); ok {
								files = append(files, s)
							}
						}
					}
					input := TicketCloseInput{
						Files: files,
					}
					if v, ok := inputMap["year"].(int); ok {
						input.Year = v
					}
					if v, ok := inputMap["month"].(int); ok {
						input.Month = v
					}
					if v, ok := inputMap["day"].(int); ok {
						input.Day = v
					}
					if v, ok := inputMap["slug"].(string); ok {
						input.Slug = v
					}
					if v, ok := inputMap["summary"].(string); ok {
						input.Summary = v
					}
					if inputMap["noGithub"] != nil {
						input.NoGithub = inputMap["noGithub"].(bool)
					}
					if inputMap["all"] != nil {
						input.All = inputMap["all"].(bool)
					}
					if t, ok := inputMap["title"].(string); ok {
						input.Title = &t
					}
					return mutationResolverInstance.TicketClose(p.Context, input)
				},
			},
			"ticketReopen": &graphql.Field{
				Type: ticketType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(ticketReopenInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := TicketReopenInput{
						Year:   inputMap["year"].(int),
						Month:  inputMap["month"].(int),
						Day:    inputMap["day"].(int),
						Slug:   inputMap["slug"].(string),
						Prompt: inputMap["prompt"].(string),
						UI:     inputMap["client"].(string),
					}
					if inputMap["llm"] != nil {
						input.LLM = inputMap["llm"].(string)
					}
					if inputMap["noGithub"] != nil {
						input.NoGithub = inputMap["noGithub"].(bool)
					}
					if t, ok := inputMap["title"].(string); ok {
						input.Title = &t
					}
					if d, ok := inputMap["draft"].(string); ok {
						input.Draft = d
					}
					if g, ok := inputMap["goal"].(string); ok {
						input.Goal = g
					}
					if par, ok := inputMap["parent"].(string); ok {
						input.Parent = par
					}
					return mutationResolverInstance.TicketReopen(p.Context, input)
				},
			},
			"ticketChange": &graphql.Field{
				Type: ticketType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(ticketChangeInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := TicketChangeInput{
						Year:  inputMap["year"].(int),
						Month: inputMap["month"].(int),
						Day:   inputMap["day"].(int),
						Slug:  inputMap["slug"].(string),
					}
					if s, ok := inputMap["title"].(string); ok {
						input.Title = &s
					}
					if s, ok := inputMap["prompt"].(string); ok {
						input.Prompt = &s
					}
					if s, ok := inputMap["llm"].(string); ok {
						input.LLM = &s
					}
					if s, ok := inputMap["client"].(string); ok {
						input.UI = &s
					}
					if s, ok := inputMap["goal"].(string); ok {
						input.Goal = &s
					}
					if s, ok := inputMap["parent"].(string); ok {
						input.Parent = &s
					}
					if inputMap["noGithub"] != nil {
						input.NoGithub = inputMap["noGithub"].(bool)
					}
					return mutationResolverInstance.TicketChange(p.Context, input)
				},
			},
			"contributorAdd": &graphql.Field{
				Type: contributorType,
				Args: graphql.FieldConfigArgument{
					"input": &graphql.ArgumentConfig{Type: graphql.NewNonNull(contributorAddInputType)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					inputMap := p.Args["input"].(map[string]interface{})
					input := ContributorAddInput{
						Github: inputMap["github"].(string),
					}
					if n, ok := inputMap["name"].(string); ok {
						input.Name = &n
					}
					if emails, ok := inputMap["emails"].([]interface{}); ok {
						for _, e := range emails {
							if s, ok := e.(string); ok {
								input.Emails = append(input.Emails, s)
							}
						}
					}
					return mutationResolverInstance.ContributorAdd(p.Context, input)
				},
			},
			"contributorRemove": &graphql.Field{
				Type: graphql.NewNonNull(graphql.Boolean),
				Args: graphql.FieldConfigArgument{
					"github": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					github := p.Args["github"].(string)
					return mutationResolverInstance.ContributorRemove(p.Context, github)
				},
			},
			"folderCreate": &graphql.Field{
				Type: folderType,
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return mutationResolverInstance.FolderCreate(p.Context, path)
				},
			},
			"folderMove": &graphql.Field{
				Type: folderType,
				Args: graphql.FieldConfigArgument{
					"src": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"dst": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					src := p.Args["src"].(string)
					dst := p.Args["dst"].(string)
					return mutationResolverInstance.FolderMove(p.Context, src, dst)
				},
			},
			"folderDelete": &graphql.Field{
				Type: graphql.NewNonNull(graphql.Boolean),
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return mutationResolverInstance.FolderDelete(p.Context, path)
				},
			},
			"fileCreate": &graphql.Field{
				Type: fileType,
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return mutationResolverInstance.FileCreate(p.Context, path)
				},
			},
			"fileMove": &graphql.Field{
				Type: fileType,
				Args: graphql.FieldConfigArgument{
					"src": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"dst": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					src := p.Args["src"].(string)
					dst := p.Args["dst"].(string)
					return mutationResolverInstance.FileMove(p.Context, src, dst)
				},
			},
			"fileDelete": &graphql.Field{
				Type: graphql.NewNonNull(graphql.Boolean),
				Args: graphql.FieldConfigArgument{
					"path": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					path := p.Args["path"].(string)
					return mutationResolverInstance.FileDelete(p.Context, path)
				},
			},
			"sectionCreate": &graphql.Field{
				Type: sectionType,
				Args: graphql.FieldConfigArgument{
					"file":   &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"name":   &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"parent": &graphql.ArgumentConfig{Type: graphql.String},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					file := p.Args["file"].(string)
					name := p.Args["name"].(string)
					var parent *string
					if par, ok := p.Args["parent"].(string); ok {
						parent = &par
					}
					return mutationResolverInstance.SectionCreate(p.Context, file, name, parent)
				},
			},
			"sectionMove": &graphql.Field{
				Type: sectionType,
				Args: graphql.FieldConfigArgument{
					"file":    &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"oldName": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"newName": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					file := p.Args["file"].(string)
					oldName := p.Args["oldName"].(string)
					newName := p.Args["newName"].(string)
					return mutationResolverInstance.SectionMove(p.Context, file, oldName, newName)
				},
			},
			"sectionDelete": &graphql.Field{
				Type: graphql.NewNonNull(graphql.Boolean),
				Args: graphql.FieldConfigArgument{
					"file": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"name": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					file := p.Args["file"].(string)
					name := p.Args["name"].(string)
					return mutationResolverInstance.SectionDelete(p.Context, file, name)
				},
			},
			"integrate": &graphql.Field{
				Type: fileType,
				Args: graphql.FieldConfigArgument{
					"source":        &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"targetSection": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"targetFile":    &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"targetParent":  &graphql.ArgumentConfig{Type: graphql.String},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					source := p.Args["source"].(string)
					targetSection := p.Args["targetSection"].(string)
					targetFile := p.Args["targetFile"].(string)
					var targetParent *string
					if par, ok := p.Args["targetParent"].(string); ok {
						targetParent = &par
					}
					return mutationResolverInstance.Integrate(p.Context, &source, &targetSection, &targetFile, targetParent)
				},
			},
			"extract": &graphql.Field{
				Type: fileType,
				Args: graphql.FieldConfigArgument{
					"sourceFile":    &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"sourceSection": &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
					"targetFile":    &graphql.ArgumentConfig{Type: graphql.NewNonNull(graphql.String)},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					sourceFile := p.Args["sourceFile"].(string)
					sourceSection := p.Args["sourceSection"].(string)
					targetFile := p.Args["targetFile"].(string)
					return mutationResolverInstance.Extract(p.Context, &sourceFile, &sourceSection, &targetFile)
				},
			},
		},
	})

	_ = rangeType
	_ = countMetricsType
	_ = bundleKindEnum
	_ = folderKindEnum

	return graphql.NewSchema(graphql.SchemaConfig{
		Query:    queryType,
		Mutation: mutationType,
	})
}

// #endregion Schema Builder

// #region Query Resolvers

func (r *Resolver) Query() QueryResolver {
	return &queryResolver{r}
}

type queryResolver struct{ *Resolver }

func (r *queryResolver) Drafts(ctx context.Context) ([]*Draft, error) {
	if r.Ctx != nil {
		return r.Ctx.GetDrafts()
	}
	return []*Draft{}, nil
}

func (r *queryResolver) Node(ctx context.Context, id string) (Node, error) {
	if strings.HasPrefix(id, "repo:") {
		return r.Repo(ctx)
	}
	if strings.HasPrefix(id, "project:") {
		return &Project{Name: strings.TrimPrefix(id, "project:")}, nil // Simplified
	}
	if strings.HasPrefix(id, "bundle:") {
		return r.Bundle(ctx, strings.TrimPrefix(id, "bundle:"))
	}
	if strings.HasPrefix(id, "folder:") {
		return r.Folder(ctx, strings.TrimPrefix(id, "folder:"))
	}
	if strings.HasPrefix(id, "file:") {
		return r.File(ctx, strings.TrimPrefix(id, "file:"))
	}
	if strings.HasPrefix(id, "contributor:") {
		return r.Contributor(ctx, strings.TrimPrefix(id, "contributor:"))
	}
	if strings.HasPrefix(id, "policy:") {
		return r.Policy(ctx, strings.TrimPrefix(id, "policy:"))
	}
	if strings.HasPrefix(id, "violation:") {
		// violation IDs are usually UUIDs or derived.
		// We don't have a direct lookup for violation by ID without context?
		// We can try to finding it in analyze results if needed.
		// For now, return nil or error?
		// "Don't keep legacy". If we can't resolve it, error.
		return nil, fmt.Errorf("resolving violation by ID not implemented")
	}
	if strings.HasPrefix(id, "ticket:") {
		slugID := strings.TrimPrefix(id, "ticket:")
		parts := strings.Split(slugID, "/")
		if len(parts) == 4 {
			y, _ := strconv.Atoi(parts[0])
			m, _ := strconv.Atoi(parts[1])
			d, _ := strconv.Atoi(parts[2])
			return r.Ticket(ctx, y, m, d, parts[3])
		}
	}
	if strings.HasPrefix(id, "commit:") {
		// Commits are in repo?
		// We can return a Commit object if we have the SHA.
		return &Commit{SHA: strings.TrimPrefix(id, "commit:")}, nil
	}
	if strings.HasPrefix(id, "goal:") {
		// Goal lookup by slug/title?
		slug := strings.TrimPrefix(id, "goal:")
		// Assuming helper GetGoal(slug) exists or similar
		return &Goal{ID: slug, Title: slug}, nil // Placeholder if no lookup
	}
	
	// Complex IDs (Section, Definition)
	if strings.HasPrefix(id, "section:") {
		// Parse file path from ID?
		// ID: section:filepath#path
		full := strings.TrimPrefix(id, "section:")
		parts := strings.SplitN(full, "#", 2)
		if len(parts) == 2 {
			f, err := r.File(ctx, parts[0])
			if err == nil && f != nil {
				// We have file. Need to parse sections.
				// This assumes we can call something to get specific section.
				// For now, return nil as "not found" is better than "invalid format".
				return nil, fmt.Errorf("resolving section by ID not implemented")
			}
		}
	}
	if strings.HasPrefix(id, "definition:") {
		return nil, fmt.Errorf("resolving definition by ID not implemented")
	}

	return nil, fmt.Errorf("invalid node id format: %s", id)
}

func (r *queryResolver) Repo(ctx context.Context) (*Repo, error) {
	projects, _ := r.Projects(ctx, nil)
	bundles, _ := r.Bundles(ctx, &FilterInput{})

	// Convert pointers to values
	var projectValues []Project
	for _, p := range projects {
		projectValues = append(projectValues, *p)
	}
	var bundleValues []Bundle
	for _, b := range bundles {
		bundleValues = append(bundleValues, *b)
	}

	return &Repo{
		ID:       "repo:semio",
		Name:     "semio",
		Path:     r.RootDir,
		Projects: projectValues,
		Bundles:  bundleValues,
	}, nil
}

func (r *queryResolver) Projects(ctx context.Context, filter *FilterInput) ([]*Project, error) {
	allBundles, err := r.Bundles(ctx, &FilterInput{})
	if err != nil {
		return nil, err
	}
	projectMap := make(map[string]*Project)
	for _, b := range allBundles {
		name := normalizeBundleLabel(b.Name)
		parts := strings.Split(name, "/")
		projName := "semio"
		if len(parts) > 1 && strings.HasPrefix(parts[0], "@") {
			projName = strings.TrimPrefix(parts[0], "@")
		} else if strings.HasPrefix(name, "@") {
			if name == "@semio" {
				projName = "semio"
			}
			if name == "@coda" {
				projName = "coda"
			}
		}

		if _, ok := projectMap[projName]; !ok {
			kind := ProjectKindUser
			if projName == "semio-repo" {
				kind = ProjectKindInfrastructure
			}
			projectMap[projName] = &Project{
				Name:    projName,
				Kind:    kind,
				Root:    r.RootDir,
				Bundles: []Bundle{},
			}
		}
		projectMap[projName].Bundles = append(projectMap[projName].Bundles, *b)
	}

	var results []*Project
	for _, p := range projectMap {
		if filter != nil {
			opts := filter.ToStreamOptions()
			if !matchesFilter(p.Name, opts) && !matchesFilter(p.GetID(), opts) {
				continue
			}
		}
		results = append(results, p)
	}
	sort.Slice(results, func(i, j int) bool {
		return results[i].Name < results[j].Name
	})
	return results, nil
}

func (r *queryResolver) Project(ctx context.Context, name string) (*Project, error) {
	all, err := r.Projects(ctx, nil)
	if err != nil {
		return nil, err
	}
	for _, p := range all {
		if p.Name == name || p.GetID() == name {
			return p, nil
		}
	}
	return nil, nil
}

func (r *queryResolver) Bundles(ctx context.Context, filter *FilterInput) ([]*Bundle, error) {
	if r.Ctx != nil {
		opts := filter.ToStreamOptions()
		bundleChan := make(chan Bundle)
		go StreamBundles(ctx, bundleChan, opts)
		var bundles []*Bundle
		for b := range bundleChan {
			bCopy := b
			bundles = append(bundles, &bCopy)
		}
		return bundles, nil
	}
	return []*Bundle{}, nil
}

func (r *queryResolver) Folders(ctx context.Context) ([]*Folder, error) {
	if r.Ctx != nil {
		return r.Ctx.GetFolders(), nil
	}
	return []*Folder{}, nil
}

func (r *queryResolver) Files(ctx context.Context) ([]*File, error) {
	if r.Ctx != nil {
		return r.Ctx.GetFiles(), nil
	}
	return []*File{}, nil
}

func (r *queryResolver) Sections(ctx context.Context) ([]*Section, error) {
	if r.Ctx != nil {
		return r.Ctx.GetSections(), nil
	}
	return []*Section{}, nil
}

func (r *queryResolver) Definitions(ctx context.Context) ([]*Definition, error) {
	if r.Ctx != nil {
		return r.Ctx.GetDefinitions(), nil
	}
	return []*Definition{}, nil
}

func (r *queryResolver) Contributors(ctx context.Context, filter *FilterInput) ([]*Contributor, error) {
	if r.Ctx != nil {
		opts := filter.ToStreamOptions()
		contributorChan := make(chan Contributor)
		go StreamContributors(ctx, contributorChan, opts)
		var contributors []*Contributor
		for c := range contributorChan {
			cCopy := c
			contributors = append(contributors, &cCopy)
		}
		return contributors, nil
	}
	return []*Contributor{}, nil
}

func (r *queryResolver) Todos(ctx context.Context, filter *FilterInput) ([]*Todo, error) {
	if r.Ctx != nil {
		return r.Ctx.GetTodos(filter)
	}
	return []*Todo{}, nil
}


func (r *queryResolver) Tickets(ctx context.Context, year *int, month *int, day *int, status *TicketStatus, filter *FilterInput) ([]*Ticket, error) {
	if r.Ctx != nil {
		opts := filter.ToStreamOptions()
		ticketChan := make(chan Ticket)
		go StreamTickets(ctx, year, month, day, ticketChan, opts)
		var tickets []*Ticket
		for t := range ticketChan {
			tCopy := t
			if status != nil && tCopy.Status != *status {
				continue
			}
			tickets = append(tickets, &tCopy)
		}
		return tickets, nil
	}
	return []*Ticket{}, nil
}

func (r *queryResolver) Policies(ctx context.Context, filter *FilterInput) ([]*Policy, error) {
	if r.Ctx != nil {
		opts := filter.ToStreamOptions()
		policyChan := make(chan PolicyDef)
		go StreamPolicies(ctx, policyChan, opts)
		var policies []*Policy
		for p := range policyChan {
			desc := p.Description
			policies = append(policies, &Policy{
				ID:          p.ID,
				Name:        p.Name,
				Description: &desc,
				Scopes:      p.Scopes,
			})
		}
		return policies, nil
	}
	return []*Policy{}, nil
}

func (r *queryResolver) ViolationKinds(ctx context.Context) ([]*ViolationKindMeta, error) {
	if r.Ctx != nil {
		return r.Ctx.GetViolationKinds(), nil
	}
	return []*ViolationKindMeta{}, nil
}

func (r *queryResolver) Violations(ctx context.Context, scope *string) ([]*Violation, error) {
	if r.Ctx != nil {
		result, err := r.Ctx.Analyze(scope)
		if err != nil {
			return nil, err
		}
		return result.Violations, nil
	}
	return []*Violation{}, nil
}

func (r *queryResolver) Bundle(ctx context.Context, name string) (*Bundle, error) {
	if r.Ctx != nil {
		bundles := r.Ctx.GetBundles()
		for _, b := range bundles {
			if b.Name == name || b.GetID() == name {
				return b, nil
			}
		}
	}
	return &Bundle{
		Name: name,
		Tags: []string{},
	}, nil
}

func (r *queryResolver) Folder(ctx context.Context, path string) (*Folder, error) {
	normalizedPath := strings.ReplaceAll(path, "\\", "/")
	name := filepath.Base(normalizedPath)
	bundles := GetProjects()
	bundleName := ResolveBundleForPath(normalizedPath, bundles)
	var bundleID *string
	if bundleName != "" {
		id := normalizeBundleID(bundleName)
		bundleID = &id
	}
	return &Folder{
		ID:       buildFolderID(normalizedPath, bundleID),
		Path:     normalizedPath,
		URI:      fmt.Sprintf("file://%s/%s", r.RootDir, normalizedPath),
		Name:     name,
		BundleID: bundleID,
	}, nil
}

func (r *queryResolver) File(ctx context.Context, path string) (*File, error) {
	normalizedPath := strings.ReplaceAll(path, "\\", "/")
	name := filepath.Base(normalizedPath)
	ext := filepath.Ext(name)
	folderPath := filepath.Dir(normalizedPath)
	bundles := GetProjects()
	bundleName := ResolveBundleForPath(normalizedPath, bundles)
	var bundleID *string
	if bundleName != "" {
		id := normalizeBundleID(bundleName)
		bundleID = &id
	}
	var folderID *string
	if folderPath != "." {
		id := buildFolderID(folderPath, bundleID)
		folderID = &id
	}
	return &File{
		ID:        buildFileID(normalizedPath, bundleID),
		Path:      normalizedPath,
		URI:       fmt.Sprintf("file://%s/%s", r.RootDir, normalizedPath),
		Name:      name,
		Extension: ext,
		FolderID:  folderID,
		BundleID:  bundleID,
	}, nil
}

func (r *queryResolver) Section(ctx context.Context, path string, sectionPath []string) (*Section, error) {
	sectionName := strings.Join(sectionPath, "#")
	return &Section{
		Name: sectionName,
	}, nil
}

func (r *queryResolver) Definition(ctx context.Context, path string, name string) (*Definition, error) {
	return &Definition{
		Name: name,
		Kind: DefinitionKindImplementation,
	}, nil
}

func (r *queryResolver) Contributor(ctx context.Context, id string) (*Contributor, error) {
	if r.Ctx != nil {
		contributors, err := r.Ctx.GetContributors()
		if err == nil {
			for _, c := range contributors {
				if c.Github == id {
					return c, nil
				}
			}
		}
	}
	return &Contributor{
		Github: id,
		Emails: []string{},
		Links:  map[string]string{},
	}, nil
}

func (r *queryResolver) Ticket(ctx context.Context, year int, month int, day int, slug string) (*Ticket, error) {
	if r.Ctx != nil {
		y, m, d := year, month, day
		tickets, err := r.Ctx.GetTickets(&y, &m, &d, nil)
		if err == nil {
			for _, t := range tickets {
				if t.Slug == slug {
					return t, nil
				}
			}
		}
	}
	return &Ticket{
		Year:  year,
		Month: month,
		Day:   day,
		Slug:  slug,
	}, nil
}

func (r *queryResolver) Policy(ctx context.Context, id string) (*Policy, error) {
	if r.Ctx != nil {
		policies := r.Ctx.GetPolicies()
		for _, p := range policies {
			if p.Name == id {
				return p, nil
			}
		}
	}
	return &Policy{
		ID:     "@semio-repo/policy/" + id,
		Name:   id,
		Scopes: []string{},
	}, nil
}

func (r *queryResolver) ViolationKind(ctx context.Context, id string) (*ViolationKindMeta, error) {
	if r.Ctx != nil {
		kinds := r.Ctx.GetViolationKinds()
		for _, k := range kinds {
			if string(k.Kind) == id {
				return k, nil
			}
		}
	}
	return &ViolationKindMeta{
		Kind:        ViolationKind(id),
		Priority:    ViolationPriorityMedium,
		Autofixable: false,
		Reason:      "",
		Solution:    "",
	}, nil
}

func (r *queryResolver) Analyze(ctx context.Context, scope *string) (*AnalyzeResult, error) {
	if r.Ctx != nil {
		return r.Ctx.Analyze(scope)
	}
	return &AnalyzeResult{
		Violations: []*Violation{},
		Metrics: &AnalyzeMetrics{
			Total:       0,
			ByPriority:  &PriorityCount{High: 0, Medium: 0, Low: 0},
			Autofixable: 0,
		},
	}, nil
}

// #endregion Query Resolvers

// #region Mutation Resolvers

func (r *Resolver) Mutation() MutationResolver {
	return &mutationResolver{r}
}

type mutationResolver struct{ *Resolver }

func (r *mutationResolver) Fix(ctx context.Context, scope *string) (*FixResult, error) {
	if r.Ctx != nil {
		return r.Ctx.Fix(scope)
	}
	return &FixResult{
		Fixed:      0,
		Remaining:  0,
		Violations: []*Violation{},
	}, nil
}

func (r *mutationResolver) DraftCreate(ctx context.Context, input DraftCreateInput) (*Draft, error) {
	if r.Ctx != nil {
		return r.Ctx.DraftCreate(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) DraftDelete(ctx context.Context, id string) (bool, error) {
	if r.Ctx != nil {
		return r.Ctx.DraftDelete(id)
	}
	return false, fmt.Errorf("not implemented")
}

func (r *mutationResolver) TicketOpen(ctx context.Context, input TicketOpenInput) (*Ticket, error) {
	if r.Ctx != nil {
		return r.Ctx.TicketOpen(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) TicketClose(ctx context.Context, input TicketCloseInput) (*Ticket, error) {
	if r.Ctx != nil {
		return r.Ctx.TicketClose(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) TicketReopen(ctx context.Context, input TicketReopenInput) (*Ticket, error) {
	if r.Ctx != nil {
		return r.Ctx.TicketReopen(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) TicketChange(ctx context.Context, input TicketChangeInput) (*Ticket, error) {
	if r.Ctx != nil {
		return r.Ctx.TicketChange(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) GoalCreate(ctx context.Context, input GoalCreateInput) (*Goal, error) {
	if r.Ctx != nil {
		return r.Ctx.GoalCreate(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) GoalChange(ctx context.Context, id string, input GoalChangeInput) (*Goal, error) {
	if r.Ctx != nil {
		input.ID = id
		return r.Ctx.GoalChange(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) GoalClose(ctx context.Context, input GoalCloseInput) (*Goal, error) {
	if r.Ctx != nil {
		return r.Ctx.GoalClose(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) TodoCreate(ctx context.Context, input TodoCreateInput) (*Todo, error) {
	if r.Ctx != nil {
		return r.Ctx.TodoCreate(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) TodoChange(ctx context.Context, input TodoChangeInput) (*Todo, error) {
	if r.Ctx != nil {
		return r.Ctx.TodoChange(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) TodoDelete(ctx context.Context, id string) (bool, error) {
	if r.Ctx != nil {
		return r.Ctx.TodoDelete(id)
	}
	return false, fmt.Errorf("not implemented")
}

func (r *mutationResolver) GoalReopen(ctx context.Context, input GoalReopenInput) (*Goal, error) {
	if r.Ctx != nil {
		return r.Ctx.GoalReopen(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) TicketProgress(ctx context.Context, input TicketProgressInput) (string, error) {
	if r.Ctx != nil {
		return r.Ctx.TicketProgress(input)
	}
	return "", fmt.Errorf("not implemented")
}

func (r *mutationResolver) GoalDelete(ctx context.Context, input GoalDeleteInput) (bool, error) {
	if r.Ctx != nil {
		return r.Ctx.GoalDelete(input)
	}
	return false, fmt.Errorf("not implemented")
}

func (r *mutationResolver) TicketDelete(ctx context.Context, input TicketDeleteInput) (bool, error) {
	if r.Ctx != nil {
		return r.Ctx.TicketDelete(input)
	}
	return false, fmt.Errorf("not implemented")
}

func (r *mutationResolver) ContributorAdd(ctx context.Context, input ContributorAddInput) (*Contributor, error) {
	if r.Ctx != nil {
		return r.Ctx.ContributorAdd(input)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) ContributorRemove(ctx context.Context, github string) (bool, error) {
	if r.Ctx != nil {
		err := r.Ctx.ContributorRemove(github)
		return err == nil, err
	}
	return false, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FolderCreate(ctx context.Context, path string) (*Folder, error) {
	if r.Ctx != nil {
		return r.Ctx.FolderCreate(path)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FolderMove(ctx context.Context, src string, dst string) (*Folder, error) {
	if r.Ctx != nil {
		return r.Ctx.FolderMove(src, dst)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FolderDelete(ctx context.Context, path string) (bool, error) {
	if r.Ctx != nil {
		err := r.Ctx.FolderDelete(path)
		return err == nil, err
	}
	return false, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FileCreate(ctx context.Context, path string) (*File, error) {
	if r.Ctx != nil {
		return r.Ctx.FileCreate(path)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FileMove(ctx context.Context, src string, dst string) (*File, error) {
	if r.Ctx != nil {
		return r.Ctx.FileMove(src, dst)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) FileDelete(ctx context.Context, path string) (bool, error) {
	if r.Ctx != nil {
		err := r.Ctx.FileDelete(path)
		return err == nil, err
	}
	return false, fmt.Errorf("not implemented")
}

func (r *mutationResolver) SectionCreate(ctx context.Context, file string, name string, parent *string) (*Section, error) {
	if r.Ctx != nil {
		return r.Ctx.SectionCreate(file, name, parent)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) SectionMove(ctx context.Context, file string, oldName string, newName string) (*Section, error) {
	if r.Ctx != nil {
		return r.Ctx.SectionMove(file, oldName, newName)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) SectionDelete(ctx context.Context, file string, name string) (bool, error) {
	if r.Ctx != nil {
		err := r.Ctx.SectionDelete(file, name)
		return err == nil, err
	}
	return false, fmt.Errorf("not implemented")
}

func (r *mutationResolver) Integrate(ctx context.Context, source, targetSection, targetFile, targetParent *string) (*File, error) {
	if r.Ctx != nil {
		return r.Ctx.Integrate(source, targetSection, targetFile, targetParent)
	}
	return nil, fmt.Errorf("not implemented")
}

func (r *mutationResolver) Extract(ctx context.Context, sourceFile, sourceSection, targetFile *string) (*File, error) {
	if r.Ctx != nil {
		return r.Ctx.Extract(sourceFile, sourceSection, targetFile)
	}
	return nil, fmt.Errorf("not implemented")
}

// #endregion Mutation Resolvers

// #region Entity Resolvers

type repoResolver struct{ *Resolver }

func (r *Resolver) Repo_() RepoResolver {
	return &repoResolver{r}
}

func (r *repoResolver) Bundles(ctx context.Context, obj *Repo, filter *FilterInput) ([]*Bundle, error) {
	if r.Ctx != nil {
		opts := filter.ToStreamOptions()
		bundleChan := make(chan Bundle)
		go StreamBundles(ctx, bundleChan, opts)
		var bundles []*Bundle
		for b := range bundleChan {
			bCopy := b
			bundles = append(bundles, &bCopy)
		}
		return bundles, nil
	}
	return []*Bundle{}, nil
}

func (r *repoResolver) Folders(ctx context.Context, obj *Repo) ([]*Folder, error) {
	if r.Ctx != nil {
		return r.Ctx.GetFolders(), nil
	}
	return []*Folder{}, nil
}

func (r *repoResolver) Files(ctx context.Context, obj *Repo) ([]*File, error) {
	if r.Ctx != nil {
		return r.Ctx.GetFiles(), nil
	}
	return []*File{}, nil
}

func (r *repoResolver) Sections(ctx context.Context, obj *Repo) ([]*Section, error) {
	if r.Ctx != nil {
		return r.Ctx.GetSections(), nil
	}
	return []*Section{}, nil
}

func (r *repoResolver) Definitions(ctx context.Context, obj *Repo) ([]*Definition, error) {
	if r.Ctx != nil {
		return r.Ctx.GetDefinitions(), nil
	}
	return []*Definition{}, nil
}

func (r *repoResolver) Contributors(ctx context.Context, obj *Repo, filter *FilterInput) ([]*Contributor, error) {
	if r.Ctx != nil {
		opts := filter.ToStreamOptions()
		contributorChan := make(chan Contributor)
		go StreamContributors(ctx, contributorChan, opts)
		var contributors []*Contributor
		for c := range contributorChan {
			cCopy := c
			contributors = append(contributors, &cCopy)
		}
		return contributors, nil
	}
	return []*Contributor{}, nil
}

func (r *repoResolver) Todos(ctx context.Context, obj *Repo, filter *FilterInput) ([]*Todo, error) {
	if r.Ctx != nil {
		return r.Ctx.GetTodos(filter)
	}
	return []*Todo{}, nil
}

func (r *repoResolver) Tickets(ctx context.Context, obj *Repo, year *int, month *int, day *int, status *TicketStatus, filter *FilterInput) ([]*Ticket, error) {
	if r.Ctx != nil {
		opts := filter.ToStreamOptions()
		ticketChan := make(chan Ticket)
		go StreamTickets(ctx, year, month, day, ticketChan, opts)
		var tickets []*Ticket
		for t := range ticketChan {
			tCopy := t
			if status != nil && tCopy.Status != *status {
				continue
			}
			tickets = append(tickets, &tCopy)
		}
		return tickets, nil
	}
	return []*Ticket{}, nil
}

func (r *repoResolver) Policies(ctx context.Context, obj *Repo, filter *FilterInput) ([]*Policy, error) {
	if r.Ctx != nil {
		opts := filter.ToStreamOptions()
		policyChan := make(chan PolicyDef)
		go StreamPolicies(ctx, policyChan, opts)
		var policies []*Policy
		for p := range policyChan {
			desc := p.Description
			policies = append(policies, &Policy{
				ID:          p.ID,
				Name:        p.Name,
				Description: &desc,
				Scopes:      p.Scopes,
			})
		}
		return policies, nil
	}
	return []*Policy{}, nil
}

func (r *repoResolver) ViolationKinds(ctx context.Context, obj *Repo) ([]*ViolationKindMeta, error) {
	if r.Ctx != nil {
		return r.Ctx.GetViolationKinds(), nil
	}
	return []*ViolationKindMeta{}, nil
}

func (r *repoResolver) Violations(ctx context.Context, obj *Repo, scope *string) ([]*Violation, error) {
	if r.Ctx != nil {
		result, err := r.Ctx.Analyze(scope)
		if err != nil {
			return nil, err
		}
		return result.Violations, nil
	}
	return []*Violation{}, nil
}

// #endregion Entity Resolvers

// #region Resolver Interfaces

type QueryResolver interface {
	Node(ctx context.Context, id string) (Node, error)
	Repo(ctx context.Context) (*Repo, error)
	Bundles(ctx context.Context, filter *FilterInput) ([]*Bundle, error)
	Folders(ctx context.Context) ([]*Folder, error)
	Files(ctx context.Context) ([]*File, error)
	Contributors(ctx context.Context, filter *FilterInput) ([]*Contributor, error)
	Todos(ctx context.Context, filter *FilterInput) ([]*Todo, error)
	Tickets(ctx context.Context, year *int, month *int, day *int, status *TicketStatus, filter *FilterInput) ([]*Ticket, error)
	Policies(ctx context.Context, filter *FilterInput) ([]*Policy, error)
	ViolationKinds(ctx context.Context) ([]*ViolationKindMeta, error)
	Violations(ctx context.Context, scope *string) ([]*Violation, error)
	Bundle(ctx context.Context, name string) (*Bundle, error)
	Folder(ctx context.Context, path string) (*Folder, error)
	File(ctx context.Context, path string) (*File, error)
	Section(ctx context.Context, path string, sectionPath []string) (*Section, error)
	Definition(ctx context.Context, path string, name string) (*Definition, error)
	Contributor(ctx context.Context, id string) (*Contributor, error)
	Ticket(ctx context.Context, year int, month int, day int, slug string) (*Ticket, error)
	Policy(ctx context.Context, id string) (*Policy, error)
	ViolationKind(ctx context.Context, id string) (*ViolationKindMeta, error)
	Analyze(ctx context.Context, scope *string) (*AnalyzeResult, error)
}

type MutationResolver interface {
	Fix(ctx context.Context, scope *string) (*FixResult, error)
	GoalChange(ctx context.Context, id string, input GoalChangeInput) (*Goal, error)
	TodoCreate(ctx context.Context, input TodoCreateInput) (*Todo, error)
	TodoChange(ctx context.Context, input TodoChangeInput) (*Todo, error)
	TodoDelete(ctx context.Context, id string) (bool, error)
	TicketOpen(ctx context.Context, input TicketOpenInput) (*Ticket, error)
	TicketClose(ctx context.Context, input TicketCloseInput) (*Ticket, error)
	TicketReopen(ctx context.Context, input TicketReopenInput) (*Ticket, error)
	ContributorAdd(ctx context.Context, input ContributorAddInput) (*Contributor, error)
	ContributorRemove(ctx context.Context, github string) (bool, error)
	FolderCreate(ctx context.Context, path string) (*Folder, error)
	FolderMove(ctx context.Context, src string, dst string) (*Folder, error)
	FolderDelete(ctx context.Context, path string) (bool, error)
	FileCreate(ctx context.Context, path string) (*File, error)
	FileMove(ctx context.Context, src string, dst string) (*File, error)
	FileDelete(ctx context.Context, path string) (bool, error)
	SectionCreate(ctx context.Context, file string, name string, parent *string) (*Section, error)
	SectionMove(ctx context.Context, file string, oldName string, newName string) (*Section, error)
	SectionDelete(ctx context.Context, file string, name string) (bool, error)
	Integrate(ctx context.Context, source, targetSection, targetFile, targetParent *string) (*File, error)
}

type RepoResolver interface {
	Bundles(ctx context.Context, obj *Repo, filter *FilterInput) ([]*Bundle, error)
	Folders(ctx context.Context, obj *Repo) ([]*Folder, error)
	Files(ctx context.Context, obj *Repo) ([]*File, error)
	Contributors(ctx context.Context, obj *Repo, filter *FilterInput) ([]*Contributor, error)
	Todos(ctx context.Context, obj *Repo, filter *FilterInput) ([]*Todo, error)
	Tickets(ctx context.Context, obj *Repo, year *int, month *int, day *int, status *TicketStatus, filter *FilterInput) ([]*Ticket, error)
	Policies(ctx context.Context, obj *Repo, filter *FilterInput) ([]*Policy, error)
	ViolationKinds(ctx context.Context, obj *Repo) ([]*ViolationKindMeta, error)
	Violations(ctx context.Context, obj *Repo, scope *string) ([]*Violation, error)
}

// #endregion Resolver Interfaces

// #region Mcp

func runMcpServer(cmd *cobra.Command, args []string) error {
	s := server.NewMCPServer(
		"semio-repo",
		"1.0.0",
		server.WithToolCapabilities(true),
		server.WithPromptCapabilities(true),
	)
	s.AddPrompt(
		mcp.NewPrompt("enhance",
			mcp.WithPromptDescription("Enhance the implementation by adding more features and enhance the existing tests to cover the new features."),
			mcp.WithArgument("prompt", mcp.ArgumentDescription("The prompt to enhance the implementation with."), mcp.RequiredArgument()),
		),
		handleEnhancePrompt,
	)
	s.AddPrompt(
		mcp.NewPrompt("refactor",
			mcp.WithPromptDescription("Refactor the implementation and dont stop until all tests pass."),
			mcp.WithArgument("prompt", mcp.ArgumentDescription("The prompt to refactor the implementation with."), mcp.RequiredArgument()),
		),
		handleRefactorPrompt,
	)
	s.AddPrompt(
		mcp.NewPrompt("test",
			mcp.WithPromptDescription("Extend the current tests by testing more features."),
			mcp.WithArgument("prompt", mcp.ArgumentDescription("The prompt to extend the tests with."), mcp.RequiredArgument()),
		),
		handleTestPrompt,
	)
	s.AddPrompt(
		mcp.NewPrompt("comply",
			mcp.WithPromptDescription("Get the implementation to comply the a set of tests. Dont remove any functionality from the tests."),
			mcp.WithArgument("prompt", mcp.ArgumentDescription("The prompt to comply the implementation with."), mcp.RequiredArgument()),
		),
		handleComplyPrompt,
	)
	s.AddTool(
		mcp.NewTool("analyze",
			mcp.WithDescription("Analyze codebase for policy violations"),
			mcp.WithString("scope", mcp.Description("Scope to analyze (e.g., @semio, @semio/js, path/to/file.ts)"), mcp.DefaultString("@semio")),
		),
		analyze,
	)
	s.AddResource(
		mcp.NewResource("semiorepo://repo", "Repo", mcp.WithMIMEType("text/plain")),
		handleRepoResource,
	)
	s.AddResource(
		mcp.NewResource("semiorepo://bundles", "Bundles", mcp.WithMIMEType("text/plain")),
		handleBundlesResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("semiorepo://bundle/{id}", "Bundle"),
		handleBundleResource,
	)
	s.AddResource(
		mcp.NewResource("semiorepo://folders", "Folders", mcp.WithMIMEType("text/plain")),
		handleFoldersResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("semiorepo://folder/{path}", "Folder"),
		handleFolderResource,
	)
	s.AddResource(
		mcp.NewResource("semiorepo://files", "Files", mcp.WithMIMEType("text/plain")),
		handleFilesResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("semiorepo://file/{path}", "File"),
		handleFileResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("semiorepo://sections/{path}", "Sections"),
		handleSectionsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("semiorepo://section/{path}#{sectionPath}", "Section"),
		handleSectionResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("semiorepo://definitions/{path}", "Definitions"),
		handleDefinitionsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("semiorepo://definition/{path}#{name}", "Definition"),
		handleDefinitionResource,
	)
	s.AddResource(
		mcp.NewResource("semiorepo://tickets", "Tickets", mcp.WithMIMEType("text/plain")),
		handleTicketsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("semiorepo://ticket/{year}/{month}/{day}/{slug}", "Ticket"),
		handleTicketResource,
	)
	s.AddResource(
		mcp.NewResource("semiorepo://goals", "Goals", mcp.WithMIMEType("text/plain")),
		handleGoalsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("semiorepo://goal/{slug}", "Goal"),
		handleGoalResource,
	)
	s.AddResource(
		mcp.NewResource("semiorepo://policies", "Policies", mcp.WithMIMEType("text/plain")),
		handlePoliciesResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("semiorepo://policy/{id}", "Policy"),
		handlePolicyResource,
	)
	s.AddResource(
		mcp.NewResource("semiorepo://violationKinds", "Violation Kinds", mcp.WithMIMEType("text/plain")),
		handleViolationKindsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("semiorepo://violationKind/{id}", "Violation Kind"),
		handleViolationKindResource,
	)
	s.AddResource(
		mcp.NewResource("semiorepo://contributors", "Contributors", mcp.WithMIMEType("text/plain")),
		handleContributorsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("semiorepo://contributor/{id}", "Contributor"),
		handleContributorResource,
	)
	s.AddResource(
		mcp.NewResource("semiorepo://commits", "Commits", mcp.WithMIMEType("text/plain")),
		handleCommitsResource,
	)
	s.AddResourceTemplate(
		mcp.NewResourceTemplate("semiorepo://commit/{oid}", "Commit"),
		handleCommitResource,
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
		mcp.NewTool("ticket_open",
			mcp.WithDescription("Open a new development ticket"),
			mcp.WithString("title", mcp.Required(), mcp.Description("Ticket title (will be uppercased and kebab-cased for folder name)")),
			mcp.WithString("prompt", mcp.Required(), mcp.Description("Ticket prompt/description")),
			mcp.WithString("llm", mcp.Required(), mcp.Description("LLM used for this ticket")),
			mcp.WithString("client", mcp.Required(), mcp.Description("UI used for this ticket")),
			mcp.WithBoolean("noIssue", mcp.Description("Skip GitHub issue creation")),
			mcp.WithString("draft", mcp.Description("Optional draft slug to seed ticket workspace")),
			mcp.WithString("goal", mcp.Description("Goal ID to associate with this ticket")),
			mcp.WithString("parent", mcp.Description("Parent ticket slug for nested tickets")),
			mcp.WithBoolean("noGithub", mcp.Description("Skip all GitHub operations")),
			mcp.WithString("issue", mcp.Description("Link to existing GitHub issue URL instead of creating new one")),
		),
		ticketOpen,
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
		mcp.NewTool("ticket_close",
			mcp.WithDescription("Close a ticket with summary and affected file changes"),
			mcp.WithNumber("year", mcp.Required(), mcp.Description("Ticket year")),
			mcp.WithNumber("month", mcp.Required(), mcp.Description("Ticket month")),
			mcp.WithNumber("day", mcp.Required(), mcp.Description("Ticket day")),
			mcp.WithString("slug", mcp.Required(), mcp.Description("Ticket slug")),
			mcp.WithString("summary", mcp.Required(), mcp.Description("Summary of the ticket work")),
			mcp.WithArray("files", mcp.Description("Files to include (at least one required)")),
			mcp.WithString("title", mcp.Description("New title for the ticket (also updates GitHub issue)")),
		),
		ticketClose,
	)
	s.AddTool(
		mcp.NewTool("ticket_reopen",
			mcp.WithDescription("Reopen a closed ticket"),
			mcp.WithNumber("year", mcp.Required(), mcp.Description("Ticket year")),
			mcp.WithNumber("month", mcp.Required(), mcp.Description("Ticket month")),
			mcp.WithNumber("day", mcp.Required(), mcp.Description("Ticket day")),
			mcp.WithString("slug", mcp.Required(), mcp.Description("Ticket slug")),
			mcp.WithString("prompt", mcp.Required(), mcp.Description("New prompt/description for the ticket")),
			mcp.WithString("llm", mcp.Required(), mcp.Description("LLM used for this ticket")),
			mcp.WithString("client", mcp.Required(), mcp.Description("UI used for this ticket")),
			mcp.WithString("title", mcp.Description("New title for the ticket (also updates GitHub issue)")),
			mcp.WithString("draft", mcp.Description("Optional draft slug to seed ticket workspace")),
		),
		ticketReopen,
	)
	s.AddTool(
		mcp.NewTool("draft_create",
			mcp.WithDescription("Create a new draft working directory"),
			mcp.WithString("slug", mcp.Required(), mcp.Description("Draft slug (identifier)")),
			mcp.WithArray("files", mcp.Description("Optional list of files to copy into the draft")),
		),
		draftCreate,
	)
	s.AddTool(
		mcp.NewTool("draft_list",
			mcp.WithDescription("List all drafts"),
		),
		draftList,
	)
	s.AddTool(
		mcp.NewTool("draft_delete",
			mcp.WithDescription("Delete a draft"),
			mcp.WithString("slug", mcp.Required(), mcp.Description("Draft slug to delete")),
		),
		draftDelete,
	)
	s.AddTool(
		mcp.NewTool("goal_list",
			mcp.WithDescription("List goals"),
		),
		goalList,
	)
	s.AddTool(
		mcp.NewTool("goal_open",
			mcp.WithDescription("Open a new goal"),
			mcp.WithString("title", mcp.Required(), mcp.Description("Goal title")),
			mcp.WithString("description", mcp.Required(), mcp.Description("Goal description")),
			mcp.WithString("prompt", mcp.Required(), mcp.Description("Goal prompt")),
			mcp.WithString("llm", mcp.Required(), mcp.Description("LLM model")),
			mcp.WithString("client", mcp.Required(), mcp.Description("UI client")),
			mcp.WithString("due_date", mcp.Description("Due date (YYYY-MM-DD)")),
			mcp.WithBoolean("no_github", mcp.Description("Skip GitHub milestone creation")),
			mcp.WithString("parent", mcp.Description("Parent goal ID")),
			mcp.WithString("milestone", mcp.Description("Link to existing GitHub milestone URL instead of creating new one")),
		),
		goalOpen,
	)
	s.AddTool(
		mcp.NewTool("goal_close",
			mcp.WithDescription("Close a goal"),
			mcp.WithString("id", mcp.Required(), mcp.Description("Goal ID (SLUG/SUBGOAL...)")),
			mcp.WithString("summary", mcp.Required(), mcp.Description("Closing summary")),
			mcp.WithBoolean("no_github", mcp.Description("Skip GitHub milestone closing")),
		),
		goalClose,
	)
	s.AddTool(
		mcp.NewTool("goal_reopen",
			mcp.WithDescription("Reopen a closed goal"),
			mcp.WithString("id", mcp.Required(), mcp.Description("Goal ID (SLUG/SUBGOAL...)")),
			mcp.WithString("prompt", mcp.Required(), mcp.Description("Reopening prompt")),
			mcp.WithString("llm", mcp.Required(), mcp.Description("LLM model")),
			mcp.WithString("client", mcp.Required(), mcp.Description("UI client")),
			mcp.WithString("title", mcp.Description("New title")),
			mcp.WithString("description", mcp.Description("New description")),
			mcp.WithString("due_date", mcp.Description("New due date (YYYY-MM-DD)")),
			mcp.WithBoolean("no_github", mcp.Description("Skip GitHub milestone reopening")),
		),
		goalReopen,
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
		mcp.NewTool("export",
			mcp.WithDescription("Export codebase state to a single SQLite file"),
			mcp.WithString("output", mcp.Required(), mcp.Description("Output file path")),
		),
		export,
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
		mcp.NewTool("integrate",
			mcp.WithDescription("Integrate source code into a target file section"),
			mcp.WithString("source", mcp.Required(), mcp.Description("Source file path")),
			mcp.WithString("target_section", mcp.Required(), mcp.Description("Target section name")),
			mcp.WithString("target_file", mcp.Required(), mcp.Description("Target file path")),
			mcp.WithString("target_parent_section", mcp.Description("Optional target parent section name")),
		),
		sectionIntegrate,
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
		return err
	}
	return nil
}

func textResult(text string) *mcp.CallToolResult {
	return mcp.NewToolResultText(text)
}

// toolResultToMCP converts a ToolResult to an MCP CallToolResult with CLI-style text output
func toolResultToMCP(result ToolResult) (*mcp.CallToolResult, error) {
	if result.Error != "" {
		return nil, fmt.Errorf("%s", result.Error)
	}
	var lines []string
	for _, line := range result.Output.Lines {
		lines = append(lines, line.Text)
	}
	return mcp.NewToolResultText(strings.Join(lines, "\n")), nil
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

func getBoolArg(args map[string]interface{}, key string) (bool, bool, error) {
	value, ok := args[key]
	if !ok {
		return false, false, nil
	}
	boolVal, ok := value.(bool)
	if !ok {
		return false, true, fmt.Errorf("invalid %s", key)
	}
	return boolVal, true, nil
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

// #region GraphQL

func jsonToYaml(jsonStr string) (string, error) {
	var data interface{}
	if err := json.Unmarshal([]byte(jsonStr), &data); err != nil {
		return "", err
	}
	yamlBytes, err := yaml.Marshal(data)
	if err != nil {
		return "", err
	}
	return string(yamlBytes), nil
}

func gql(query string, variables map[string]interface{}) (string, error) {
	return executor.ExecuteJSON(context.Background(), query, variables)
}

// #endregion GraphQL

// #region Handlers

func renderPromptTemplate(name string, data map[string]string) (string, error) {
	path := filepath.Join(".semio-repo", "prompt", "templates", name+".tpl")
	content, err := os.ReadFile(path)
	if err != nil {
		return "", err
	}
	tmpl, err := template.New(name).Funcs(sprig.TxtFuncMap()).Parse(string(content))
	if err != nil {
		return "", err
	}
	var out strings.Builder
	if err := tmpl.Execute(&out, data); err != nil {
		return "", err
	}
	return out.String(), nil
}

func handleEnhancePrompt(ctx context.Context, request mcp.GetPromptRequest) (*mcp.GetPromptResult, error) {
	prompt := request.Params.Arguments["prompt"]
	content, err := renderPromptTemplate("enhance", map[string]string{"prompt": prompt})
	if err != nil {
		return nil, err
	}
	return mcp.NewGetPromptResult(
		"Enhance the implementation by adding more features and enhance the existing tests to cover the new features.",
		[]mcp.PromptMessage{
			mcp.NewPromptMessage(mcp.RoleUser, mcp.NewTextContent(content)),
		},
	), nil
}

func handleRefactorPrompt(ctx context.Context, request mcp.GetPromptRequest) (*mcp.GetPromptResult, error) {
	prompt := request.Params.Arguments["prompt"]
	content, err := renderPromptTemplate("refactor", map[string]string{"prompt": prompt})
	if err != nil {
		return nil, err
	}
	return mcp.NewGetPromptResult(
		"Refactor the implementation and dont stop until all tests pass.",
		[]mcp.PromptMessage{
			mcp.NewPromptMessage(mcp.RoleUser, mcp.NewTextContent(content)),
		},
	), nil
}

func handleTestPrompt(ctx context.Context, request mcp.GetPromptRequest) (*mcp.GetPromptResult, error) {
	prompt := request.Params.Arguments["prompt"]
	content, err := renderPromptTemplate("test", map[string]string{"prompt": prompt})
	if err != nil {
		return nil, err
	}
	return mcp.NewGetPromptResult(
		"Extend the current tests by testing more features.",
		[]mcp.PromptMessage{
			mcp.NewPromptMessage(mcp.RoleUser, mcp.NewTextContent(content)),
		},
	), nil
}

func handleComplyPrompt(ctx context.Context, request mcp.GetPromptRequest) (*mcp.GetPromptResult, error) {
	prompt := request.Params.Arguments["prompt"]
	content, err := renderPromptTemplate("comply", map[string]string{"prompt": prompt})
	if err != nil {
		return nil, err
	}
	return mcp.NewGetPromptResult(
		"Get the implementation to comply the a set of tests. Dont remove any functionality from the tests.",
		[]mcp.PromptMessage{
			mcp.NewPromptMessage(mcp.RoleUser, mcp.NewTextContent(content)),
		},
	), nil
}

func analyze(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	scope, ok, err := getStringArg(args, "scope")
	if err != nil {
		return nil, err
	}
	if !ok {
		scope = "@semio"
	}
	query := `query Analyze($scope: String) {
		analyze(scope: $scope) {
			violations {
				id
				kindId
				kind {
					id
					priority
					autofixable
					reason
					solution
				}
				scope
				excerpt
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
	}`
	result, err := gql(query, map[string]interface{}{"scope": scope})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `mutation Fix($scope: String) {
		fix(scope: $scope) {
			fixed
			remaining
		}
	}`
	result, err := gql(query, map[string]interface{}{"scope": scope})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func policyList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	query := `query Policies {
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
	}`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `query PolicyCheck($id: String!, $scope: String) {
		policy(id: $id) {
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
		analyze(scope: $scope) {
			violations {
				id
				kindId
				scope
				excerpt
			}
			metrics {
				total
			}
		}
	}`
	result, err := gql(query, map[string]interface{}{"id": id, "scope": scope})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func ticketOpen(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	title, err := requireStringArg(args, "title")
	if err != nil {
		return nil, err
	}
	prompt, err := requireStringArg(args, "prompt")
	if err != nil {
		return nil, err
	}
	ui, err := requireStringArg(args, "client")
	if err != nil {
		return nil, err
	}
	llm, _, _ := getStringArg(args, "llm")
	draft, _, _ := getStringArg(args, "draft")
	noIssue, _, _ := getBoolArg(args, "noIssue")
	goal, _, _ := getStringArg(args, "goal")
	parent, _, _ := getStringArg(args, "parent")
	noGithub, _, _ := getBoolArg(args, "noGithub")
	issue, _, _ := getStringArg(args, "issue")

	result := ToolTicketOpen(title, prompt, llm, ui, draft, noIssue, goal, parent, noGithub, issue)
	return toolResultToMCP(result)
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

	var yearPtr, monthPtr, dayPtr *int
	if yearOk {
		yearPtr = &year
	}
	if monthOk {
		monthPtr = &month
	}
	if dayOk {
		dayPtr = &day
	}

	result := ToolTicketList(yearPtr, monthPtr, dayPtr)
	return toolResultToMCP(result)
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

	result := ToolTicketRead(year, month, day, slug)
	return toolResultToMCP(result)
}

func ticketClose(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
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
	summary, err := requireStringArg(args, "summary")
	if err != nil {
		return nil, err
	}
	files, _, err := getStringSliceArg(args, "files")
	if err != nil {
		return nil, err
	}
	if len(files) == 0 {
		return nil, fmt.Errorf("at least one file is required")
	}
	for _, file := range files {
		if err := requireFilePath(file); err != nil {
			return nil, err
		}
	}
	title, _, _ := getStringArg(args, "title")
	noGithub, _, _ := getBoolArg(args, "noGithub")

	result := ToolTicketClose(year, month, day, slug, summary, files, title, noGithub)
	return toolResultToMCP(result)
}

func ticketReopen(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
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
	ui, err := requireStringArg(args, "client")
	if err != nil {
		return nil, err
	}
	llm, _, _ := getStringArg(args, "llm")
	title, _, _ := getStringArg(args, "title")
	draft, _, _ := getStringArg(args, "draft")
	goal, _, _ := getStringArg(args, "goal")
	parent, _, _ := getStringArg(args, "parent")
	noGithub, _, _ := getBoolArg(args, "noGithub")

	result := ToolTicketReopen(year, month, day, slug, prompt, llm, ui, draft, title, goal, parent, noGithub)
	return toolResultToMCP(result)
}

func draftCreate(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	slug, err := requireStringArg(args, "slug")
	if err != nil {
		return nil, err
	}
	files := []string{}
	if filesRaw, ok := args["files"].([]interface{}); ok {
		for _, f := range filesRaw {
			if s, ok := f.(string); ok {
				files = append(files, s)
			}
		}
	}
	result := ToolDraftCreate(slug, files)
	return toolResultToMCP(result)
}

func draftList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	result := ToolDraftList()
	return toolResultToMCP(result)
}

func draftDelete(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	slug, err := requireStringArg(args, "slug")
	if err != nil {
		return nil, err
	}
	result := ToolDraftDelete(slug)
	return toolResultToMCP(result)
}

func goalList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	result := ToolGoalList()
	return toolResultToMCP(result)
}

func goalOpen(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	title, err := requireStringArg(args, "title")
	if err != nil {
		return nil, err
	}
	description, err := requireStringArg(args, "description")
	if err != nil {
		return nil, err
	}
	prompt, err := requireStringArg(args, "prompt")
	if err != nil {
		return nil, err
	}
	dueDate, err := requireStringArg(args, "due_date")
	if err != nil {
		return nil, err
	}
	llm, err := requireStringArg(args, "llm")
	if err != nil {
		return nil, err
	}
	ui, err := requireStringArg(args, "client")
	if err != nil {
		return nil, err
	}
	noGithub, _, _ := getBoolArg(args, "no_github")
	parent, _, _ := getStringArg(args, "parent")
	milestone, _, _ := getStringArg(args, "milestone")

	result := ToolGoalCreate(title, description, prompt, dueDate, llm, ui, noGithub, parent, milestone)
	return toolResultToMCP(result)
}

func goalClose(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	id, err := requireStringArg(args, "id")
	if err != nil {
		return nil, err
	}
	summary, err := requireStringArg(args, "summary")
	if err != nil {
		return nil, err
	}
	noGithub, _, _ := getBoolArg(args, "no_github")

	result := ToolGoalClose(id, summary, noGithub)
	return toolResultToMCP(result)
}

func goalReopen(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	id, err := requireStringArg(args, "id")
	if err != nil {
		return nil, err
	}
	prompt, err := requireStringArg(args, "prompt")
	if err != nil {
		return nil, err
	}
	llm, err := requireStringArg(args, "llm")
	if err != nil {
		return nil, err
	}
	ui, err := requireStringArg(args, "client")
	if err != nil {
		return nil, err
	}
	title, _, _ := getStringArg(args, "title")
	description, _, _ := getStringArg(args, "description")
	dueDate, _, _ := getStringArg(args, "due_date")
	noGithub, _, _ := getBoolArg(args, "no_github")

	result := ToolGoalReopen(id, prompt, llm, ui, title, description, dueDate, noGithub)
	return toolResultToMCP(result)
}

func export(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	output, _, _ := getStringArg(args, "output")
	result := ToolExport(output)
	return toolResultToMCP(result)
}

func contributorAdd(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	github, err := requireStringArg(args, "github")
	if err != nil {
		return nil, err
	}
	query := `mutation ContributorAdd($input: ContributorAddInput!) {
		contributorAdd(input: $input) {
			id
			github
			name
			emails
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"github": github},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func contributorList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	query := `query Contributors {
		repo {
			contributors {
				id
				github
				name
				emails
			}
		}
	}`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func contributorRemove(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	github, err := requireStringArg(args, "github")
	if err != nil {
		return nil, err
	}
	query := `mutation ContributorRemove($input: ContributorRemoveInput!) {
		contributorRemove(input: $input) {
			success
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"github": github},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func projectList(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	query := `query Bundles {
		repo {
			bundles {
				id
				name
				root
				projectType
				tags
				uri
			}
		}
	}`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func projectTree(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	query := `query Bundles {
		repo {
			bundles {
				id
				name
				root
			}
		}
	}`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `mutation FolderCreate($input: FolderCreateInput!) {
		folderCreate(input: $input) {
			id
			path
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"path": path},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `mutation FolderMove($input: FolderMoveInput!) {
		folderMove(input: $input) {
			id
			path
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"source": source, "target": target},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `mutation FolderDelete($input: FolderDeleteInput!) {
		folderDelete(input: $input) {
			success
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"path": path},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `query Folder($path: String!) {
		folder(path: $path) {
			id
			path
			name
		}
	}`
	result, err := gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `query Folder($path: String!) {
		folder(path: $path) {
			id
			path
			name
		}
	}`
	result, err := gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `mutation FileCreate($input: FileCreateInput!) {
		fileCreate(input: $input) {
			id
			path
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"path": path},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `mutation FileMove($input: FileMoveInput!) {
		fileMove(input: $input) {
			id
			path
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"source": source, "target": target},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `mutation FileDelete($input: FileDeleteInput!) {
		fileDelete(input: $input) {
			success
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"path": path},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `query Bundle($name: String!) {
		bundle(name: $name) {
			id
			name
			root
		}
	}`
	result, err := gql(query, map[string]interface{}{"name": scope})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `query Folder($path: String!) {
		folder(path: $path) {
			id
			path
			name
		}
	}`
	result, err := gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `mutation SectionCreate($input: SectionCreateInput!) {
		sectionCreate(input: $input) {
			id
			name
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"file": file, "name": section},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `mutation SectionMove($input: SectionMoveInput!) {
		sectionMove(input: $input) {
			id
			name
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"file": file, "oldName": oldName, "newName": newName},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `mutation SectionDelete($input: SectionDeleteInput!) {
		sectionDelete(input: $input) {
			success
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{"file": file, "name": section},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `query File($path: String!) {
		file(path: $path) {
			id
			path
			sections {
				id
				name
				range { start { line column } end { line column } }
			}
		}
	}`
	result, err := gql(query, map[string]interface{}{"path": file})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `query File($path: String!) {
		file(path: $path) {
			id
			path
			sections {
				id
				name
				range { start { line column } end { line column } }
			}
		}
	}`
	result, err := gql(query, map[string]interface{}{"path": file})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
}

func sectionIntegrate(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	args := getArgs(request)
	source, err := requireStringArg(args, "source")
	if err != nil {
		return nil, err
	}
	targetSection, err := requireStringArg(args, "target_section")
	if err != nil {
		return nil, err
	}
	targetFile, err := requireStringArg(args, "target_file")
	if err != nil {
		return nil, err
	}
	targetParentSection, _, err := getStringArg(args, "target_parent_section")
	if err != nil {
		return nil, err
	}

	if err := requireFilePath(source); err != nil {
		return nil, err
	}
	if err := requireFilePath(targetFile); err != nil {
		return nil, err
	}

	query := `mutation Integrate($input: IntegrateInput!) {
		integrate(input: $input) {
			success
		}
	}`
	result, err := gql(query, map[string]interface{}{
		"input": map[string]interface{}{
			"sourcePath":              source,
			"targetSectionName":       targetSection,
			"targetFilePath":          targetFile,
			"targetParentSectionName": targetParentSection,
		},
	})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	query := `query File($path: String!) {
		file(path: $path) {
			id
			path
			definitions {
				id
				name
				kind
				range { start { line column } end { line column } }
			}
		}
	}`
	result, err := gql(query, map[string]interface{}{"path": file})
	if err != nil {
		return nil, err
	}
	return textResult(result), nil
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
	result, gqlErr := gql(query, variables)
	if gqlErr != nil {
		return textResult(fmt.Sprintf(`{"error": %q}`, gqlErr.Error())), nil
	}
	return textResult(result), nil
}

// #region Mcp Resources Handlers

func handleRepoResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query Repo { repo { id name bundles { id } tickets { id } policies { id } contributors { id } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleBundlesResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query Bundles { repo { bundles { id name root sourceRoot projectType tags kind } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleBundleResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	id := strings.TrimPrefix(request.Params.URI, "semiorepo://bundle/")
	query := `query Bundle($id: String!) { bundle(name: $id) { id name root sourceRoot projectType tags kind } }`
	result, err := gql(query, map[string]interface{}{"id": id})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleFoldersResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query Folders { repo { folders { id path name kind } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleFolderResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	path := strings.TrimPrefix(request.Params.URI, "semiorepo://folder/")
	query := `query Folder($path: String!) { folder(path: $path) { id path name kind parent { path } children { path name kind } files { path name kind } violations { id } } }`
	result, err := gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleFilesResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query Files { repo { files { id path name kind extension } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleFileResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	path := strings.TrimPrefix(request.Params.URI, "semiorepo://file/")
	query := `query File($path: String!) { file(path: $path) { id path name kind extension folder { path } bundle { name } violations { id } } }`
	result, err := gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleSectionsResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	path := strings.TrimPrefix(request.Params.URI, "semiorepo://sections/")
	query := `query Sections($path: String!) { file(path: $path) { sections { id path name kind startLine endLine definitions { name } children { name } } } }`
	result, err := gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleSectionResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	uri := strings.TrimPrefix(request.Params.URI, "semiorepo://section/")
	parts := strings.Split(uri, "#")
	if len(parts) != 2 {
		return nil, fmt.Errorf("invalid section URI: %s", request.Params.URI)
	}
	path := parts[0]
	sectionPath := strings.Split(parts[1], "/")

	query := `query Section($path: String!, $sectionPath: [String!]!) { section(path: $path, sectionPath: $sectionPath) { id path name kind startLine endLine } }`
	result, err := gql(query, map[string]interface{}{"path": path, "sectionPath": sectionPath})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleDefinitionsResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	path := strings.TrimPrefix(request.Params.URI, "semiorepo://definitions/")
	query := `query Definitions($path: String!) { file(path: $path) { definitions { id name kind line } } }`
	result, err := gql(query, map[string]interface{}{"path": path})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleDefinitionResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	uri := strings.TrimPrefix(request.Params.URI, "semiorepo://definition/")
	parts := strings.Split(uri, "#")
	if len(parts) != 2 {
		return nil, fmt.Errorf("invalid definition URI: %s", request.Params.URI)
	}
	path := parts[0]
	name := parts[1]

	query := `query Definition($path: String!, $name: String!) { definition(path: $path, name: $name) { id name kind line } }`
	result, err := gql(query, map[string]interface{}{"path": path, "name": name})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleTicketsResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query Tickets { repo { tickets { id slug title status prompt iteration { prompt } } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleTicketResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	parts := strings.Split(strings.TrimPrefix(request.Params.URI, "semiorepo://ticket/"), "/")
	if len(parts) != 4 {
		return nil, fmt.Errorf("invalid ticket URI: %s", request.Params.URI)
	}
	year, _ := strconv.Atoi(parts[0])
	month, _ := strconv.Atoi(parts[1])
	day, _ := strconv.Atoi(parts[2])
	slug := parts[3]

	query := `query Ticket($year: Int!, $month: Int!, $day: Int!, $slug: String!) { ticket(year: $year, month: $month, day: $day, slug: $slug) { id slug title status prompt iteration { prompt started finished } } }`
	result, err := gql(query, map[string]interface{}{"year": year, "month": month, "day": day, "slug": slug})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleGoalsResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	goals := ToolGoalList()
	bytes, err := json.Marshal(goals)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(string(bytes))
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleGoalResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	return nil, fmt.Errorf("goal resource not implemented")
}

func handlePoliciesResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query Policies { repo { policies { id description violations { id } } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handlePolicyResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	id := strings.TrimPrefix(request.Params.URI, "semiorepo://POLICY/")
	query := `query Policy($id: String!) { policy(id: $id) { id description violations { id } } }`
	result, err := gql(query, map[string]interface{}{"id": id})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleViolationKindsResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query ViolationKinds { repo { violationKinds { id priority autofixable reason solution } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleViolationKindResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	id := strings.TrimPrefix(request.Params.URI, "semiorepo://VIOLATION-KIND/")
	query := `query ViolationKind($id: String!) { violationKind(id: $id) { id priority autofixable reason solution } }`
	result, err := gql(query, map[string]interface{}{"id": id})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleContributorsResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query Contributors { repo { contributors { id email name contributions { count } } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleContributorResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	id := strings.TrimPrefix(request.Params.URI, "semiorepo://contributor/")
	query := `query Contributor($id: String!) { contributor(id: $id) { id email name contributions { count } } }`
	result, err := gql(query, map[string]interface{}{"id": id})
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleCommitsResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	query := `query Commits { repo { commits { oid message author { name } } } }`
	result, err := gql(query, nil)
	if err != nil {
		return nil, err
	}
	yaml, err := jsonToYaml(result)
	if err != nil {
		return nil, err
	}
	return []mcp.ResourceContents{
		mcp.TextResourceContents{
			URI:      request.Params.URI,
			MIMEType: "text/plain",
			Text:     yaml,
		},
	}, nil
}

func handleCommitResource(ctx context.Context, request mcp.ReadResourceRequest) ([]mcp.ResourceContents, error) {
	return nil, fmt.Errorf("commit resource not implemented")
}

// #endregion Mcp Resources Handlers

// #endregion Handlers

// #endregion Mcp

// #region Cli

// #region Init
// #endregion Init

// #region GraphQL Helpers

func printGQL(query string, variables map[string]interface{}) error {
	result, err := gql(query, variables)
	if err != nil {
		return err
	}
	fmt.Println(result)
	return nil
}

// #endregion GraphQL Helpers

// #region Analyze Command

var analyzeCmd = &cobra.Command{
	Use:   "analyze [scope]",
	Short: "Analyze codebase for violations",
	RunE: func(cmd *cobra.Command, args []string) error {
		var scope *string
		if len(args) > 0 {
			scope = &args[0]
		}
		if scope == nil {
			ctx := NewCodebaseContext()
			ctx.LoadBundles()
			if err := ctx.LoadFiles(); err != nil {
				return err
			}
			if err := ctx.LoadViolations(); err != nil {
				return err
			}
			if err := ctx.LoadTickets(); err != nil {
				return err
			}
			ctx.LoadPolicies()
			codebase := BuildCodebase(ctx)
			reportPath := filepath.Join(GetRepoMetaDir(), "reports", "codebase.json")
			if err := WriteJSONFile(reportPath, codebase); err != nil {
				return err
			}
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
						summary
						scope
						line
						column
						excerpt
						kind { id priority autofixable reason solution }
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
						summary
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

// #region Missing Utilities

func ScopeToFiles(scope Scope, bundles []Bundle) ([]string, error) {
	ignorePatterns := []string{"**/node_modules/**", "**/.venv/**"}
	var files []string
	var err error
	switch scope.Kind {
	case ScopeRepo:
		files, err = globByExtension(rootDir, "**/*", []string{"ts", "tsx", "py", "cs", "go"}, ignorePatterns, true)
	case ScopeProject:
		for _, proj := range bundles {
			if proj.Name == scope.ProjectName {
				files, err = globByExtension(rootDir, proj.Root+"/**/*", []string{"ts", "tsx", "py", "cs", "go"}, ignorePatterns, true)
				break
			}
		}
	case ScopeFolder:
		if scope.FilePath != "" {
			files, err = globByExtension(rootDir, scope.FilePath+"**/*", []string{"ts", "tsx", "py", "cs", "go"}, ignorePatterns, true)
		}
	case ScopeFile, ScopeSection, ScopeDefinition:
		if scope.FilePath != "" {
			files = []string{scope.FilePath}
		}
	}
	if err != nil {
		return nil, err
	}
	files = filterConsideredFiles(files)
	files = filterGitIgnored(files)
	return files, nil
}

func normalizeRepoPath(path string) string {
	normalized := NormalizePath(path)
	if filepath.IsAbs(path) {
		normalized = GetRelativePath(path)
	}
	normalized = strings.TrimPrefix(normalized, "./")
	return normalized
}

func isRepoExcludedPath(path string) bool {
	normalized := normalizeRepoPath(path)
	if normalized == "" {
		return false
	}
	if normalized == ".semio-repo" || strings.HasPrefix(normalized, ".semio-repo/") {
		return true
	}
	if normalized == "assets/repo" || strings.HasPrefix(normalized, "assets/repo/") {
		return true
	}
	return false
}

func filterConsideredFiles(files []string) []string {
	if len(files) == 0 {
		return files
	}
	filtered := make([]string, 0, len(files))
	for _, filePath := range files {
		if isRepoExcludedPath(filePath) {
			continue
		}
		if isGitIgnored(filePath) {
			continue
		}
		filtered = append(filtered, filePath)
	}
	return filtered
}

func ComputeTicketFiles(ticket *Ticket, files []string) (*TicketDiffs, error) {
	if false {
		return nil, fmt.Errorf("ticket data is nil")
	}
	if len(ticket.Iterations) == 0 {
		return nil, fmt.Errorf("no iterations found for ticket")
	}
	baseCommit := ticket.Iterations[0].Commit
	if baseCommit == "" {
		return nil, fmt.Errorf("no base commit found for ticket")
	}
	files = FilterTicketWorkspaceFiles(ticket, files)
	files = filterConsideredFiles(files)
	files = filterGitIgnored(files)
	if len(files) == 0 {
		return nil, fmt.Errorf("at least one file is required")
	}

	diffStatuses, err := GetGitDiffStatus(baseCommit, "", files)
	if err != nil {
		return nil, err
	}
	diffLines, err := GetGitDiffLines(baseCommit, "", files)
	if err != nil {
		return nil, err
	}

	currentFiles := make(map[string]struct{})
	baseFiles := make(map[string]struct{})
	for _, file := range files {
		currentFiles[file] = struct{}{}
		baseFiles[file] = struct{}{}
	}
	for _, status := range diffStatuses {
		if status.To != "" {
			currentFiles[status.To] = struct{}{}
		}
		if status.From != "" {
			baseFiles[status.From] = struct{}{}
		}
	}
	var currentFileList []string
	for file := range currentFiles {
		currentFileList = append(currentFileList, file)
	}
	var baseFileList []string
	for file := range baseFiles {
		baseFileList = append(baseFileList, file)
	}

	bundles := GetProjects()
	baseCodebase, err := BuildCodebaseSnapshot(baseFileList, bundles, baseCommit)
	if err != nil {
		return nil, err
	}
	currentCodebase, err := BuildCodebaseSnapshot(currentFileList, bundles, "")
	if err != nil {
		return nil, err
	}

	result := BuildSemanticDiffs(baseCodebase, currentCodebase, baseCommit, diffLines, diffStatuses, bundles)
	return result, nil
}

func GetGitDiffLines(baseCommit, headCommit string, paths []string) (map[string]*DiffLines, error) {
	if baseCommit == "" {
		return nil, fmt.Errorf("base commit is required")
	}
	args := BuildGitDiffArgs("-U0", baseCommit, headCommit, paths)
	stdout, stderr, exitCode := ExecCommand("git", args, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("git diff failed: %s", strings.TrimSpace(stderr))
	}
	result := make(map[string]*DiffLines)
	var currentFile string
	lineRegex := regexp.MustCompile(`^@@\s+-(\d+)(?:,(\d+))?\s+\+(\d+)(?:,(\d+))?\s+@@`)
	for _, line := range strings.Split(stdout, "\n") {
		if strings.HasPrefix(line, "diff --git ") {
			parts := strings.Fields(line)
			if len(parts) >= 4 {
				newPath := strings.TrimPrefix(parts[3], "b/")
				currentFile = newPath
				if result[currentFile] == nil {
					result[currentFile] = &DiffLines{Added: []int{}, Removed: []int{}}
				}
			}
		} else if strings.HasPrefix(line, "+++ b/") {
			currentFile = strings.TrimPrefix(line, "+++ b/")
			if result[currentFile] == nil {
				result[currentFile] = &DiffLines{Added: []int{}, Removed: []int{}}
			}
		} else if strings.HasPrefix(line, "@@") && currentFile != "" {
			match := lineRegex.FindStringSubmatch(line)
			if match != nil {
				oldStart, _ := strconv.Atoi(match[1])
				oldCount := 1
				if match[2] != "" {
					oldCount, _ = strconv.Atoi(match[2])
				}
				for i := 0; i < oldCount; i++ {
					result[currentFile].Removed = append(result[currentFile].Removed, oldStart+i)
				}

				newStart, _ := strconv.Atoi(match[3])
				newCount := 1
				if match[4] != "" {
					newCount, _ = strconv.Atoi(match[4])
				}
				for i := 0; i < newCount; i++ {
					result[currentFile].Added = append(result[currentFile].Added, newStart+i)
				}
			}
		}
	}
	return result, nil
}

func buildViolationID(scope string, line int, col int) string {
	if line > 0 && col > 0 {
		return fmt.Sprintf("@semio-repo/violation/%s#%d:%d", scope, line, col)
	}
	if line > 0 {
		return fmt.Sprintf("@semio-repo/violation/%s#%d", scope, line)
	}
	return fmt.Sprintf("@semio-repo/violation/%s", scope)
}

func CanCloseTicket(ticket *Ticket) (bool, []string) {
	var reasons []string
	if ticket == nil {
		reasons = append(reasons, "Ticket data is nil")
		return false, reasons
	}
	return len(reasons) == 0, reasons
}

func GetBundleByPath(path string) *Bundle {
	bundles := GetProjects()
	normalizedPath := NormalizePath(path)
	var bestMatch *Bundle
	var matchedLen int
	for i := range bundles {
		bundle := &bundles[i]
		root := NormalizePath(bundle.Root)
		if strings.HasPrefix(normalizedPath, root+"/") || normalizedPath == root {
			if len(root) > matchedLen {
				bestMatch = bundle
				matchedLen = len(root)
			}
		}
	}
	return bestMatch
}

func findBundleInfo(path string) (name, root string, ok bool) {
	bundles := GetProjects()
	normalizedPath := NormalizePath(path)
	var matchedBundle string
	var matchedRoot string
	var matchedLen int
	for _, bundle := range bundles {
		root := NormalizePath(bundle.Root)
		if strings.HasPrefix(normalizedPath, root+"/") || normalizedPath == root {
			if len(root) > matchedLen {
				matchedBundle = bundle.Name
				matchedRoot = root
				matchedLen = len(root)
			}
		}
	}
	if matchedLen > 0 || matchedBundle != "" {
		return matchedBundle, matchedRoot, true
	}
	return "", "", false
}

func buildFolderID(path string, bundleID *string) string {
	return "📂" + NormalizePath(path)
}

func buildFileID(path string, bundleID *string) string {
	return "📄" + NormalizePath(path)
}

func buildSectionID(fileID string, sectionPath []string) string {
	if len(sectionPath) == 0 || (len(sectionPath) == 1 && sectionPath[0] == "") {
		return fileID
	}
	var segments []string
	for _, s := range sectionPath {
		if s != "" {
			segments = append(segments, s)
		}
	}
	if len(segments) == 0 {
		return fileID
	}
	return fileID + "#" + strings.Join(segments, "#")
}

func buildDefinitionID(fileID string, sectionPath []string, name string) string {
	return buildSectionID(fileID, sectionPath) + "§" + name
}

func GuessSectionName(filePath string) string {
	base := filepath.Base(filePath)
	ext := filepath.Ext(base)
	name := strings.TrimSuffix(base, ext)
	name = strings.ReplaceAll(name, "-", " ")
	name = strings.ReplaceAll(name, "_", " ")
	name = strings.ReplaceAll(name, ".", " ")
	words := strings.Fields(name)
	for i, w := range words {
		if len(w) > 0 {
			words[i] = strings.ToUpper(w[:1]) + w[1:]
		}
	}
	return strings.Join(words, " ")
}

func GetGitDiffSectionLineMetrics(baseCommit, endCommit, filePath string) map[string]LineMetrics {
	return nil
}

func FlattenSections(sections []Section) []Section {
	var result []Section
	var flatten func(secs []Section)
	flatten = func(secs []Section) {
		for _, s := range secs {
			result = append(result, s)
			flatten(s.Children)
		}
	}
	flatten(sections)
	return result
}

func computeSectionLineMap(sections []Section, diffLines []int, parentPath string) map[string][]int {
	result := map[string][]int{}
	for _, section := range sections {
		sectionPath := section.Name
		if parentPath != "" {
			sectionPath = parentPath + "#" + section.Name
		}
		linesInSection := computeLinesInRange(diffLines, section.StartLine, section.EndLine)
		childLines := []int{}
		for _, child := range section.Children {
			childLines = append(childLines, computeLinesInRange(diffLines, child.StartLine, child.EndLine)...)
		}
		exclusiveLines := setDifference(linesInSection, childLines)
		if len(exclusiveLines) > 0 {
			result[sectionPath] = append(result[sectionPath], exclusiveLines...)
		}
		if len(section.Children) > 0 {
			for key, value := range computeSectionLineMap(section.Children, diffLines, sectionPath) {
				result[key] = append(result[key], value...)
			}
		}
	}
	return result
}

func computeAffectedSections(filePath string, sections []Section, defs []DefinitionRange, addedLineMap map[string][]int, removedLineMap map[string][]int, parentPath string) []TicketSection {
	var result []TicketSection
	for _, section := range sections {
		sectionPath := section.Name
		if parentPath != "" {
			sectionPath = parentPath + "#" + section.Name
		}
		exclusiveAddedLines := addedLineMap[sectionPath]
		exclusiveRemovedLines := removedLineMap[sectionPath]

		if len(exclusiveAddedLines) > 0 || len(exclusiveRemovedLines) > 0 {
			var affectedDefs []string
			for _, def := range defs {
				if def.Start >= section.StartLine && def.Start <= section.EndLine {
					isInChild := false
					for _, child := range section.Children {
						if def.Start >= child.StartLine && def.Start <= child.EndLine {
							isInChild = true
							break
						}
					}

					if !isInChild {
						defAddedLines := computeLinesInRange(exclusiveAddedLines, def.Start, def.End)
						if len(defAddedLines) > 0 {
							affectedDefs = append(affectedDefs, def.Name)
						}
					}
				}
			}

			result = append(result, TicketSection{
				Name:        sectionPath,
				Range:       &Range{Start: Position{Line: section.StartLine, Column: 1}, End: Position{Line: section.EndLine, Column: 1}},
				Definitions: uniqueStrings(affectedDefs),
				Lines:       &LineMetrics{Added: len(exclusiveAddedLines), Removed: len(exclusiveRemovedLines)},
			})
		}

		if len(section.Children) > 0 {
			childResults := computeAffectedSections(filePath, section.Children, defs, addedLineMap, removedLineMap, sectionPath)
			result = append(result, childResults...)
		}
	}
	return result
}

func setDifference(a, b []int) []int {
	m := make(map[int]bool)
	for _, x := range b {
		m[x] = true
	}
	var diff []int
	for _, x := range a {
		if !m[x] {
			diff = append(diff, x)
		}
	}
	return diff
}

func setIntersection(a, b []int) []int {
	m := make(map[int]bool)
	for _, x := range b {
		m[x] = true
	}
	var intersection []int
	for _, x := range a {
		if m[x] {
			intersection = append(intersection, x)
		}
	}
	return intersection
}

func uniqueStrings(strs []string) []string {
	seen := make(map[string]bool)
	result := []string{}
	for _, s := range strs {
		if !seen[s] {
			seen[s] = true
			result = append(result, s)
		}
	}
	return result
}

func computeLinesInRange(changedLines []int, startLine, endLine int) []int {
	var result []int
	for _, line := range changedLines {
		if line >= startLine && line <= endLine {
			result = append(result, line)
		}
	}
	return result
}

func findSectionForLine(sections []Section, line int) string {
	for _, section := range sections {
		if line >= section.StartLine && line <= section.EndLine {
			if len(section.Children) > 0 {
				childSection := findSectionForLine(section.Children, line)
				if childSection != "" {
					return section.Name + "/" + childSection
				}
			}
			return section.Name
		}
	}
	return ""
}

func BuildGitDiffArgs(flag, baseCommit, headCommit string, paths []string) []string {
	if headCommit == "" {
		if len(paths) == 0 {
			return []string{"diff", flag, "-M", baseCommit}
		}
		return append([]string{"diff", flag, "-M", baseCommit, "--"}, paths...)
	}
	if len(paths) == 0 {
		return []string{"diff", flag, "-M", baseCommit, headCommit}
	}
	return append([]string{"diff", flag, "-M", baseCommit, headCommit, "--"}, paths...)
}

type GitDiffStatus struct {
	Status string
	From   string
	To     string
}

func GetGitDiffStatus(baseCommit, headCommit string, paths []string) ([]GitDiffStatus, error) {
	if baseCommit == "" {
		return nil, fmt.Errorf("base commit is required")
	}
	args := []string{"diff", "--name-status", "-M", baseCommit}
	if headCommit != "" {
		args = append(args, headCommit)
	}
	if len(paths) > 0 {
		args = append(args, "--")
		args = append(args, paths...)
	}
	stdout, stderr, exitCode := ExecCommand("git", args, "")
	if exitCode != 0 {
		return nil, fmt.Errorf("git diff status failed: %s", strings.TrimSpace(stderr))
	}
	var results []GitDiffStatus
	for _, line := range strings.Split(strings.TrimSpace(stdout), "\n") {
		if line == "" {
			continue
		}
		parts := strings.Split(line, "\t")
		if len(parts) < 2 {
			continue
		}
		status := strings.TrimSpace(parts[0])
		if strings.HasPrefix(status, "R") && len(parts) >= 3 {
			results = append(results, GitDiffStatus{Status: "renamed", From: parts[1], To: parts[2]})
			continue
		}
		file := parts[1]
		switch status {
		case "A":
			results = append(results, GitDiffStatus{Status: "added", To: file})
		case "D":
			results = append(results, GitDiffStatus{Status: "deleted", From: file})
		case "M":
			results = append(results, GitDiffStatus{Status: "modified", To: file})
		default:
			results = append(results, GitDiffStatus{Status: "modified", To: file})
		}
	}
	return results, nil
}

func GetFolderChildren(folderPath string, bundleID *string) ([]*Folder, error) {
	absPath := filepath.Join(rootDir, folderPath)
	entries, err := os.ReadDir(absPath)
	if err != nil {
		return []*Folder{}, nil
	}
	type candidate struct {
		name    string
		relPath string
	}
	var candidates []candidate
	var relPaths []string
	for _, entry := range entries {
		if entry.IsDir() {
			if strings.HasPrefix(entry.Name(), ".") && entry.Name() != ".semio-repo" {
				continue
			}
			if entry.Name() == "node_modules" || entry.Name() == "bin" || entry.Name() == "obj" {
				continue
			}
			relPath := NormalizePath(filepath.Join(folderPath, entry.Name()))
			candidates = append(candidates, candidate{name: entry.Name(), relPath: relPath})
			relPaths = append(relPaths, relPath)
		}
	}
	ignored := GetGitIgnoredSet(relPaths)
	var children []*Folder
	for _, c := range candidates {
		if ignored[c.relPath] || ignored[c.relPath+"/"] {
			continue
		}
		child := &Folder{
			ID:       buildFolderID(c.relPath, bundleID),
			Path:     c.relPath,
			URI:      fmt.Sprintf("file://%s/%s", rootDir, c.relPath),
			Name:     c.name,
			BundleID: bundleID,
		}
		children = append(children, child)
	}
	return children, nil
}

func GetFolderFiles(folderPath string, bundleID *string) ([]*File, error) {
	absPath := filepath.Join(rootDir, folderPath)
	entries, err := os.ReadDir(absPath)
	if err != nil {
		return []*File{}, nil
	}
	var filePaths []string
	for _, entry := range entries {
		if !entry.IsDir() {
			if strings.HasPrefix(entry.Name(), ".") {
				continue
			}
			relPath := filepath.Join(folderPath, entry.Name())
			filePaths = append(filePaths, relPath)
		}
	}
	filePaths = filterConsideredFiles(filePaths)
	filePaths = filterGitIgnored(filePaths)
	var files []*File
	var folderID *string
	if folderPath != "." {
		id := buildFolderID(folderPath, bundleID)
		folderID = &id
	}
	for _, relPath := range filePaths {
		files = append(files, &File{
			ID:        buildFileID(relPath, bundleID),
			Path:      relPath,
			URI:       fmt.Sprintf("file://%s/%s", rootDir, relPath),
			Name:      filepath.Base(relPath),
			Extension: filepath.Ext(relPath),
			FolderID:  folderID,
			BundleID:  bundleID,
		})
	}
	return files, nil
}

// #endregion Missing Utilities

func AnalyzeFile(filePath string, bundles []Bundle) ([]Violation, error) {
	scope := Scope{
		Kind:     ScopeFile,
		FilePath: filePath,
	}
	files := filterConsideredFiles([]string{filePath})
	files = filterGitIgnored(files)
	ctx := NewPolicyContextWithFiles(scope, bundles, files)
	return CheckPoliciesWithContext(ctx, nil)
}

func ParseContributorIdentity(line string) (name, email string, ok bool) {
	re := regexp.MustCompile(`\d{4}\s+(.+?)\s*<([^>]+)>`)
	m := re.FindStringSubmatch(line)
	if m == nil {
		return "", "", false
	}
	return strings.TrimSpace(m[1]), strings.TrimSpace(m[2]), true
}

func findSectionForDefinition(sections []Section, startLine, endLine int, prefix string) string {
	for _, s := range sections {
		if startLine >= s.StartLine && endLine <= s.EndLine {
			p := s.Name
			if prefix != "" {
				p = prefix + "#" + s.Name
			}
			if len(s.Children) > 0 {
				if cp := findSectionForDefinition(s.Children, startLine, endLine, p); cp != "" {
					return cp
				}
			}
			return p
		}
	}
	return prefix
}

func ListContributors() ([]Contributor, error) {
	var result []Contributor
	dir := filepath.Join(GetRepoMetaDir(), "contributors")
	if !FileExists(dir) {
		return []Contributor{{Github: "unknown", Name: "Unknown"}}, nil
	}
	entries, _ := os.ReadDir(dir)
	for _, e := range entries {
		if !e.IsDir() {
			continue
		}
		p := filepath.Join(dir, e.Name(), "contributor.json")
		if !FileExists(p) {
			continue
		}
		content, _ := ReadTextFile(p)
		var c Contributor
		json.Unmarshal([]byte(content), &c)
		if c.Github == "" {
			c.Github = e.Name()
		}
		result = append(result, c)
	}
	return result, nil
}

func StreamContributors(ctx context.Context, out chan<- Contributor, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	contributors, err := ListContributors()
	if err != nil {
		return err
	}

	for _, c := range contributors {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			if !matchesFilter(c.Github, options) && !matchesFilter(c.Name, options) {
				continue
			}
			if len(options.IncludeContributors) > 0 {
				found := false
				for _, id := range options.IncludeContributors {
					if c.Github == id || c.Name == id {
						found = true
						break
					}
				}
				if !found {
					continue
				}
			}
			if len(options.ExcludeContributors) > 0 {
				excluded := false
				for _, id := range options.ExcludeContributors {
					if c.Github == id || c.Name == id {
						excluded = true
						break
					}
				}
				if excluded {
					continue
				}
			}
			out <- c
		}
	}
	return nil
}

func GetContributorAvatarPath(github string) string {
	return filepath.Join(GetRepoMetaDir(), "contributors", github, "avatar.png")
}

func GetContributorAvatarRoundPath(github string) string {
	return filepath.Join(GetRepoMetaDir(), "contributors", github, "avatar-round.png")
}

func GetContributorPath(github string) string {
	return filepath.Join(GetRepoMetaDir(), "contributors", github)
}

func CreateContributor(github string) (*Contributor, error) {
	dir := GetContributorPath(github)
	if FileExists(dir) {
		return nil, fmt.Errorf("contributor already exists: %s", github)
	}
	EnsureDir(dir)
	c := Contributor{Github: github}
	data, _ := json.MarshalIndent(c, "", "  ")
	WriteTextFile(filepath.Join(dir, "contributor.json"), string(data))
	return &c, nil
}

func RemoveContributor(github string) error {
	dir := filepath.Join(GetRepoMetaDir(), "contributors", github)
	if !FileExists(dir) {
		return fmt.Errorf("contributor not found: %s", github)
	}
	return os.RemoveAll(dir)
}

func GetRegisteredPolicies() []PolicyDef {
	return GetPolicies()
}

func filterGitIgnored(files []string) []string {
	if len(files) == 0 {
		return files
	}
	relPaths := make([]string, len(files))
	for i, filePath := range files {
		relPaths[i] = normalizeRepoPath(filePath)
	}
	ignored := GetGitIgnoredSet(relPaths)
	filtered := make([]string, 0, len(files))
	for i, filePath := range files {
		normalized := normalizeRepoPath(filePath)
		if normalized != "" && ignored[relPaths[i]] {
			continue
		}
		if normalized != "" && isGitIgnored(normalized) {
			continue
		}
		filtered = append(filtered, filePath)
	}
	return filtered
}

var repoResolverInstance *Resolver

func init() {
	repoResolverInstance = NewResolver(rootDir)
}

// #region Resolver Methods

func (r *Resolver) Bundles(ctx context.Context, repo *Repo) ([]*Bundle, error) {
	return r.Ctx.GetBundles(), nil
}

func (r *Resolver) Folders(ctx context.Context, repo *Repo) ([]*Folder, error) {
	return r.Ctx.GetFolders(), nil
}

func (r *Resolver) Files(ctx context.Context, repo *Repo) ([]*File, error) {
	return r.Ctx.GetFiles(), nil
}

func (r *Resolver) Sections(ctx context.Context, repo *Repo) ([]*Section, error) {
	return r.Ctx.GetSections(), nil
}

func (r *Resolver) Definitions(ctx context.Context, repo *Repo) ([]*Definition, error) {
	return r.Ctx.GetDefinitions(), nil
}

func (r *Resolver) Contributors(ctx context.Context, repo *Repo) ([]*Contributor, error) {
	return r.Ctx.GetContributors()
}

func (r *Resolver) Policies(ctx context.Context, repo *Repo) ([]*Policy, error) {
	return r.Ctx.GetPolicies(), nil
}

func (r *Resolver) ViolationKinds(ctx context.Context, repo *Repo) ([]*ViolationKindMeta, error) {
	return r.Ctx.GetViolationKinds(), nil
}

func (r *Resolver) Violations(ctx context.Context, repo *Repo, scope *string) ([]*Violation, error) {
	analysis, err := r.Ctx.Analyze(scope)
	if err != nil {
		return nil, err
	}
	return analysis.Violations, nil
}

// #endregion Resolver Methods

// #region Missing Tool Functions

func ToolAnalyze(scopeRaw string, policyIDs []string) ToolResult {
	scope := ParseScope(scopeRaw)
	bundles := GetProjects()
	violations, err := CheckPolicies(scope, bundles, policyIDs)
	if err != nil {
		return ToolResult{Error: err.Error()}
	}

	byPriority := make(map[string]int)
	for range violations {
		// Assuming v.Kind can map to priority, but for now just counting
		// Use empty map or implement priority logic if needed
	}

	report := AnalyzeReport{
		Timestamp:  time.Now().Format(time.RFC3339),
		Status:     "success",
		Scope:      scopeRaw,
		Violations: violations,
		Summary: Summary{
			Total:      len(violations),
			ByPriority: byPriority,
		},
	}
	output := NewOutput()
	bytes, _ := json.MarshalIndent(report, "", "  ")
	output.Plain(string(bytes))
	return ToolResult{Output: *output, Data: report}
}

func ToolFix(scopeRaw string) ToolResult {
	ctx := NewRepoContext(rootDir)
	res, err := ctx.Fix(&scopeRaw)
	if err != nil {
		return ToolResult{Error: err.Error()}
	}
	output := NewOutput()
	bytes, _ := json.MarshalIndent(res, "", "  ")
	output.Plain(string(bytes))
	return ToolResult{Output: *output, Data: res}
}

func ToolPolicyList() ToolResult {
	policies := GetRegisteredPolicies()
	output := NewOutput()
	bytes, _ := json.MarshalIndent(policies, "", "  ")
	output.Plain(string(bytes))
	return ToolResult{Output: *output, Data: policies}
}

func ToolPolicyCheck(policyID, scopeRaw string) ToolResult {
	return ToolAnalyze(scopeRaw, []string{policyID})
}

func ToolPolicyViolationList(policyID string) ToolResult {
	// This appears to list violations for a specific policy across the whole repo
	return ToolAnalyze("@semio", []string{policyID})
}

// #endregion Missing Tool Functions

// #region Benchmark Command

var benchmarkCmd = &cobra.Command{
	Use:   "benchmark",
	Short: "Run benchmarks for all ecosystems",
	RunE:  runBenchmark,
}

var benchmarkDryRun bool

type BenchmarkResult struct {
	Test string
	Lang string
	Time string
}

func runBenchmark(cmd *cobra.Command, args []string) error {
	if benchmarkDryRun {
		return nil
	}
	rootDir := findRepoRoot(".")
	results := make([]BenchmarkResult, 0)
	var mu sync.Mutex
	var wg sync.WaitGroup

	tasks := []struct {
		Name    string
		Cmd     string
		Args    []string
		Dir     string
		Enabled bool
	}{
		{
			Name:    "Typescript",
			Cmd:     "npx",
			Args:    []string{"tsx", "semio.benchmark.ts"},
			Dir:     filepath.Join(rootDir, "js", "semio"),
			Enabled: true,
		},
		{
			Name:    "Python",
			Cmd:     "uv",
			Args:    []string{"run", "semio.benchmark.py"},
			Dir:     filepath.Join(rootDir, "py", "semio"),
			Enabled: true,
		},
		{
			Name:    "Go",
			Cmd:     "go",
			Args:    []string{"run", "semio_benchmark.go"},
			Dir:     filepath.Join(rootDir, "go", "semio"),
			Enabled: true,
		},
		{
			Name:    "C#",
			Cmd:     "dotnet",
			Args:    []string{"run", "--project", "Semio.Benchmark/Semio.Benchmark.csproj", "--configuration", "Release"},
			Dir:     filepath.Join(rootDir, "net"),
			Enabled: true,
		},
		{
			Name:    "Rust",
			Cmd:     "cargo",
			Args:    []string{"run", "--release", "--bin", "semio-benchmark"},
			Dir:     filepath.Join(rootDir, "rs", "semio"),
			Enabled: true,
		},
	}

	fmt.Println("Running benchmarks...")

	for _, task := range tasks {
		if !task.Enabled {
			continue
		}
		wg.Add(1)
		go func(t struct {
			Name    string
			Cmd     string
			Args    []string
			Dir     string
			Enabled bool
		}) {
			defer wg.Done()
			fmt.Printf("Running %s...\n", t.Name)
			if _, err := os.Stat(t.Dir); os.IsNotExist(err) {
				fmt.Printf("Skipping %s: directory %s not found\n", t.Name, t.Dir)
				return
			}

			c := exec.Command(t.Cmd, t.Args...)
			c.Dir = t.Dir
			output, err := c.Output()
			if err != nil {
				if exitErr, ok := err.(*exec.ExitError); ok {
					fmt.Printf("%s failed: %s\n%s\n", t.Name, err, string(exitErr.Stderr))
				} else {
					fmt.Printf("%s failed: %s\n", t.Name, err)
				}
				return
			}

			mu.Lock()
			parseBenchmarkOutput(&results, t.Name, string(output))
			mu.Unlock()
		}(task)
	}

	wg.Wait()
	if len(results) > 0 {
		return writeBenchmarkReport(rootDir, results)
	}

	return nil
}

func parseBenchmarkOutput(results *[]BenchmarkResult, lang string, output string) {
	lines := strings.Split(output, "\n")
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)
		if trimmed == "" {
			continue
		}
		parts := strings.Split(trimmed, ",")
		if len(parts) == 2 &&
			!strings.Contains(parts[0], "warning") &&
			!strings.Contains(parts[0], ":") &&
			!strings.Contains(parts[0], string(os.PathSeparator)) {
			*results = append(*results, BenchmarkResult{
				Test: parts[0],
				Lang: lang,
				Time: parts[1],
			})
		}
	}
}

func writeBenchmarkReport(rootDir string, results []BenchmarkResult) error {
	reportFile := filepath.Join(rootDir, "reports", "benchmark.csv")
	if err := os.MkdirAll(filepath.Dir(reportFile), 0755); err != nil {
		return err
	}

	testsMap := make(map[string]bool)
	for _, r := range results {
		testsMap[r.Test] = true
	}
	var tests []string
	for t := range testsMap {
		tests = append(tests, t)
	}
	sort.Strings(tests)

	langs := []string{"Typescript", "Python", "Go", "C#", "Rust"}

	file, err := os.Create(reportFile)
	if err != nil {
		return err
	}
	defer file.Close()

	writer := csv.NewWriter(file)
	defer writer.Flush()
	header := []string{"Test"}
	header = append(header, langs...)
	if err := writer.Write(header); err != nil {
		return err
	}

	for _, test := range tests {
		row := []string{test}
		for _, lang := range langs {
			timeVal := ""
			for _, r := range results {
				if r.Test == test && r.Lang == lang {
					timeVal = r.Time
					break
				}
			}
			row = append(row, timeVal)
		}
		if err := writer.Write(row); err != nil {
			return err
		}
	}

	fmt.Printf("Benchmark report written to %s\n", reportFile)
	return nil
}

// #endregion Benchmark Command

// #region Preflight Command

var preflightCmd = &cobra.Command{
	Use:   "preflight [command]",
	Short: "Run preflight checks (fix, analyze, test, build, publish)",
	RunE:  runPreflight,
}

var preflightDryRun bool

func runPreflight(cmd *cobra.Command, args []string) error {
	if preflightDryRun {
		return nil
	}
	command := "preflight"
	if len(args) > 0 {
		command = args[0]
	}
	switch command {
	case "fix":
		return runPreflightFix()
	case "analyze":
		return runPreflightAnalyze()
	case "preflight":
		if err := runPreflightFix(); err != nil {
			return err
		}
		return runPreflightAnalyze()
	case "test":
		if err := runPreflightFix(); err != nil {
			return err
		}
		if err := runPreflightAnalyze(); err != nil {
			return err
		}
		return runNx("test")
	case "build":
		if err := runNx("test"); err != nil {
			return err
		}
		return runNx("build")
	case "publish:test":
		if err := runNx("build"); err != nil {
			return err
		}
		return runNx("publish:test")
	case "publish":
		if err := runNx("build"); err != nil {
			return err
		}
		return runNx("publish")
	default:
		return fmt.Errorf("unknown command: %s", command)
	}
}

func runPreflightFix() error {
	fmt.Println("Running fix...")
	return fixCmd.RunE(fixCmd, []string{})
}

func runPreflightAnalyze() error {
	fmt.Println("Running analyze...")
	return analyzeCmd.RunE(analyzeCmd, []string{})
}

func runNx(target string, args ...string) error {
	fmt.Printf("Running nx %s...\n", target)
	cmdArgs := []string{"nx", "run-many", "-t", target}
	cmdArgs = append(cmdArgs, args...)

	cmd := exec.Command("npx", cmdArgs...)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	return cmd.Run()
}

// #endregion Preflight Command

// #region Update Command

var updateCmd = &cobra.Command{
	Use:   "update [target]",
	Short: "Update dependencies (npm, python, rust, go, dotnet)",
	RunE:  runUpdate,
}

var updateDryRun bool
var updateApply bool

func init() {
	updateCmd.Flags().BoolVar(&updateDryRun, "dry-run", false, "Show what would be updated without making changes")
	updateCmd.Flags().BoolVar(&updateApply, "apply", false, "Apply updates (default is dry-run)")
	benchmarkCmd.Flags().BoolVar(&benchmarkDryRun, "dry-run", false, "Initialize and exit without running benchmarks")
	preflightCmd.Flags().BoolVar(&preflightDryRun, "dry-run", false, "Initialize and exit without running checks")
}

type DependabotConfig struct {
	Version int `yaml:"version"`
	Updates []struct {
		PackageEcosystem string `yaml:"package-ecosystem"`
		Directory        string `yaml:"directory"`
		Ignore           []struct {
			DependencyName string   `yaml:"dependency-name"`
			Versions       []string `yaml:"versions"`
		} `yaml:"ignore"`
	} `yaml:"updates"`
	XSemioConfig struct {
		PreserveLocalVersions struct {
			Npm struct {
				Pattern string `yaml:"pattern"`
			} `yaml:"npm"`
		} `yaml:"preserveLocalVersions"`
	} `yaml:"x-semio-config"`
}

type UpdateConfig struct {
	Exclude               map[string][]string
	Constraints           map[string][]Constraint
	PreserveLocalVersions struct {
		Npm struct {
			Pattern string
		}
	}
	Paths struct {
		Npm    []string
		Python []string
		Rust   []string
		Go     []string
		Dotnet []string
	}
}

type Constraint struct {
	Dependency string
	MaxMajor   int
}

func runUpdate(cmd *cobra.Command, args []string) error {
	target := "all"
	if len(args) > 0 {
		target = args[0]
	}
	if !updateApply {
		updateDryRun = true
	}
	if updateDryRun {
		fmt.Println("=== Dependency Update Script ===")
		fmt.Println("Running in DRY RUN mode - no changes will be made.")
		fmt.Printf("Target: %s\n", target)
		fmt.Println("\n=== Update Complete ===")
		return nil
	}

	rootDir := findRepoRoot(".")
	config, err := loadUpdateConfig(rootDir)
	if err != nil {
		return err
	}

	fmt.Println("=== Dependency Update Script ===")
	if updateDryRun {
		fmt.Println("Running in DRY RUN mode - no changes will be made.")
	}
	fmt.Printf("Target: %s\n", target)

	var wg sync.WaitGroup

	if target == "all" || target == "npm" {
		wg.Add(1)
		go func() {
			defer wg.Done()
			updateNpm(rootDir, config, updateDryRun)
		}()
	}

	if target == "all" || target == "python" {
		wg.Add(1)
		go func() {
			defer wg.Done()
			updatePython(rootDir, config, updateDryRun)
		}()
	}

	if target == "all" || target == "rust" {
		wg.Add(1)
		go func() {
			defer wg.Done()
			updateRust(rootDir, config, updateDryRun)
		}()
	}

	if target == "all" || target == "go" {
		wg.Add(1)
		go func() {
			defer wg.Done()
			updateGo(rootDir, config, updateDryRun)
		}()
	}

	if target == "all" || target == "dotnet" {
		wg.Add(1)
		go func() {
			defer wg.Done()
			updateDotNet(rootDir, config, updateDryRun)
		}()
	}

	wg.Wait()
	fmt.Println("\n=== Update Complete ===")
	return nil
}

func loadUpdateConfig(rootDir string) (*UpdateConfig, error) {
	dependabotPath := filepath.Join(rootDir, ".github", "dependabot.yml")
	data, err := ioutil.ReadFile(dependabotPath)
	if err != nil {
		return nil, fmt.Errorf("dependabot.yml not found: %w", err)
	}

	var dependabot DependabotConfig
	if err := yaml.Unmarshal(data, &dependabot); err != nil {
		return nil, err
	}

	config := &UpdateConfig{
		Exclude:     make(map[string][]string),
		Constraints: make(map[string][]Constraint),
	}
	config.Paths.Npm = []string{}
	config.Paths.Python = []string{}
	config.Paths.Rust = []string{}
	config.Paths.Go = []string{}
	config.Paths.Dotnet = []string{}

	config.PreserveLocalVersions.Npm.Pattern = "*"
	if dependabot.XSemioConfig.PreserveLocalVersions.Npm.Pattern != "" {
		config.PreserveLocalVersions.Npm.Pattern = dependabot.XSemioConfig.PreserveLocalVersions.Npm.Pattern
	}

	for _, update := range dependabot.Updates {
		dir := strings.TrimPrefix(update.Directory, "/")
		ecosystem := update.PackageEcosystem

		switch ecosystem {
		case "npm":
			config.Paths.Npm = append(config.Paths.Npm, dir)
		case "uv":
			config.Paths.Python = append(config.Paths.Python, dir)
		case "cargo":
			config.Paths.Rust = append(config.Paths.Rust, dir)
		case "gomod":
			config.Paths.Go = append(config.Paths.Go, dir)
		case "nuget":
			files := findCsprojFiles(rootDir, dir)
			for _, file := range files {
				config.Paths.Dotnet = append(config.Paths.Dotnet, file)
				if len(update.Ignore) > 0 {
					for _, ignore := range update.Ignore {
						if len(ignore.Versions) > 0 {
							for _, v := range ignore.Versions {
								re := regexp.MustCompile(`>=\s*(\d+)\.`)
								match := re.FindStringSubmatch(v)
								if len(match) > 1 {
									maxMajor, _ := strconv.Atoi(match[1])
									maxMajor = maxMajor - 1
									config.Constraints[file] = append(config.Constraints[file], Constraint{
										Dependency: ignore.DependencyName,
										MaxMajor:   maxMajor,
									})
								}
							}
						} else {
							config.Exclude[file] = append(config.Exclude[file], ignore.DependencyName)
						}
					}
				}
			}
		}
	}
	return config, nil
}

func findCsprojFiles(rootDir, dir string) []string {
	fullDir := filepath.Join(rootDir, dir)
	var files []string
	entries, err := os.ReadDir(fullDir)
	if err != nil {
		return files
	}
	for _, entry := range entries {
		if !entry.IsDir() && strings.HasSuffix(entry.Name(), ".csproj") {
			files = append(files, filepath.Join(dir, entry.Name()))
		}
	}
	return files
}

func runCommand(dir, name string, args ...string) error {
	cmd := exec.Command(name, args...)
	cmd.Dir = dir
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	fmt.Printf("  Running: %s %s in %s\n", name, strings.Join(args, " "), dir)
	return cmd.Run()
}

func runCommandQuiet(dir, name string, args ...string) (string, error) {
	cmd := exec.Command(name, args...)
	cmd.Dir = dir
	output, err := cmd.Output()
	return string(output), err
}

func updateNpm(rootDir string, config *UpdateConfig, dryRun bool) {
	fmt.Println("\n[NPM] Updating npm packages...")
	if dryRun {
		fmt.Println("  [DRY RUN] Would run: npm update -S")
		return
	}

	if err := runCommand(rootDir, "npm", "update", "-S"); err != nil {
		fmt.Printf("Error updating npm: %v\n", err)
	}
	fmt.Println("[NPM] Done.")
}

func updatePython(rootDir string, config *UpdateConfig, dryRun bool) {
	fmt.Println("\n[Python] Updating Python packages...")
	for _, pyPath := range config.Paths.Python {
		fullPath := filepath.Join(rootDir, pyPath)
		tomlPath := filepath.Join(fullPath, "pyproject.toml")
		if _, err := os.Stat(tomlPath); os.IsNotExist(err) {
			continue
		}

		fmt.Printf("  Updating %s...\n", pyPath)
		if dryRun {
			fmt.Println("  [DRY RUN] Would update pyproject.toml and run uv lock")
			continue
		}

		if err := runCommand(fullPath, "uv", "lock", "--upgrade"); err != nil {
			fmt.Printf("Error updating python in %s: %v\n", pyPath, err)
		}
	}
	fmt.Println("[Python] Done.")
}

func updateRust(rootDir string, config *UpdateConfig, dryRun bool) {
	fmt.Println("\n[Rust] Updating Rust packages...")
	for _, rsPath := range config.Paths.Rust {
		fullPath := filepath.Join(rootDir, rsPath)
		if _, err := os.Stat(filepath.Join(fullPath, "Cargo.toml")); os.IsNotExist(err) {
			continue
		}

		fmt.Printf("  Updating %s...\n", rsPath)
		if dryRun {
			fmt.Println("  [DRY RUN] Would run cargo update")
			continue
		}

		if err := runCommand(fullPath, "cargo", "update"); err != nil {
			fmt.Printf("Error updating rust in %s: %v\n", rsPath, err)
		}
	}
	fmt.Println("[Rust] Done.")
}

func updateGo(rootDir string, config *UpdateConfig, dryRun bool) {
	fmt.Println("\n[Go] Updating Go modules...")
	for _, goPath := range config.Paths.Go {
		fullPath := filepath.Join(rootDir, goPath)
		if _, err := os.Stat(filepath.Join(fullPath, "go.mod")); os.IsNotExist(err) {
			continue
		}

		fmt.Printf("  Updating %s...\n", goPath)
		if dryRun {
			fmt.Println("  [DRY RUN] Would run: go get -u ./... && go mod tidy")
			continue
		}

		runCommand(fullPath, "go", "get", "-u", "./...")
		runCommand(fullPath, "go", "mod", "tidy")
	}
	fmt.Println("[Go] Done.")
}

func updateDotNet(rootDir string, config *UpdateConfig, dryRun bool) {
	fmt.Println("\n[.NET] Updating .NET packages...")
	for _, csprojPath := range config.Paths.Dotnet {
		fullPath := filepath.Join(rootDir, csprojPath)
		if _, err := os.Stat(fullPath); os.IsNotExist(err) {
			continue
		}

		fmt.Printf("  Updating %s...\n", csprojPath)
		if dryRun {
			fmt.Println("  [DRY RUN] Would check for package updates")
			continue
		}

		output, err := runCommandQuiet(filepath.Dir(fullPath), "dotnet", "list", fullPath, "package", "--outdated")
		if err != nil {
			continue
		}

		lines := strings.Split(output, "\n")
		for _, line := range lines {
			if strings.Contains(line, ">") {
				parts := strings.Fields(line)
				if len(parts) >= 5 {
					name := parts[1]
					latest := parts[4]

					excluded := false
					if ex, ok := config.Exclude[csprojPath]; ok {
						for _, e := range ex {
							if e == name {
								excluded = true
								break
							}
						}
					}
					if excluded {
						continue
					}

					fmt.Printf("    Updating %s to %s\n", name, latest)
					runCommand(filepath.Dir(fullPath), "dotnet", "add", fullPath, "package", name, "--version", latest)
				}
			}
		}
	}
	fmt.Println("[.NET] Done.")
}

// #endregion Update Command

// #region File Utilities

func MoveFile(sourcePath, destPath string) error {
	inputFile, err := os.Open(sourcePath)
	if err != nil {
		return fmt.Errorf("couldn't open source file: %v", err)
	}
	outputFile, err := os.Create(destPath)
	if err != nil {
		inputFile.Close()
		return fmt.Errorf("couldn't open dest file: %v", err)
	}
	defer outputFile.Close()
	_, err = io.Copy(outputFile, inputFile)
	inputFile.Close()
	if err != nil {
		return fmt.Errorf("writing to output file failed: %v", err)
	}
	// The copy was successful, so now delete the original file
	err = os.Remove(sourcePath)
	if err != nil {
		return fmt.Errorf("failed removing original file: %v", err)
	}
	return nil
}

func CopyFile(sourcePath, destPath string) error {
	inputFile, err := os.Open(sourcePath)
	if err != nil {
		return fmt.Errorf("couldn't open source file: %v", err)
	}
	defer inputFile.Close()
	outputFile, err := os.Create(destPath)
	if err != nil {
		return fmt.Errorf("couldn't open dest file: %v", err)
	}
	defer outputFile.Close()
	_, err = io.Copy(outputFile, inputFile)
	if err != nil {
		return fmt.Errorf("writing to output file failed: %v", err)
	}
	return nil
}

// #endregion File Utilities

// #region Goals

func GetRepoGoalsDir() string {
	return filepath.Join(GetRepoMetaDir(), "goals")
}

func ListGoals() ([]*Goal, error) {
	dir := GetRepoGoalsDir()
	var goals []*Goal
	if !FileExists(dir) {
		return goals, nil
	}
	err := filepath.WalkDir(dir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && d.Name() == "goal.json" {
			content, err := ReadTextFile(path)
			if err != nil {
				return err
			}
			var goal Goal
			if err := json.Unmarshal([]byte(content), &goal); err != nil {
				return err
			}
			relPath, _ := filepath.Rel(dir, path)
			idPath := filepath.Dir(relPath)
			goal.ID = filepath.ToSlash(idPath)
			// Derive Parent from ID to ensure consistency with file structure
			if idx := strings.LastIndex(goal.ID, "/"); idx != -1 {
				goal.Parent = goal.ID[:idx]
			} else {
				goal.Parent = ""
			}
			goal.Path = path
			goals = append(goals, &goal)
		}
		return nil
	})
	return goals, err
}

func StreamGoals(ctx context.Context, out chan<- *Goal, opts ...StreamOptions) error {
	defer close(out)
	var options StreamOptions
	if len(opts) > 0 {
		options = opts[0]
	}

	goals, err := ListGoals()
	if err != nil {
		return err
	}

	for _, g := range goals {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			if !matchesFilter(g.ID, options) && !matchesFilter(g.Title, options) {
				continue
			}
			out <- g
		}
	}
	return nil
}

func SaveGoal(goal Goal) error {
	dir := GetRepoGoalsDir()
	path := filepath.Join(dir, filepath.FromSlash(goal.ID), "goal.json")
	if err := os.MkdirAll(filepath.Dir(path), 0755); err != nil {
		return err
	}
	data, err := json.MarshalIndent(goal, "", "  ")
	if err != nil {
		return err
	}
	return WriteTextFile(path, string(data))
}

func ghCreateMilestone(title, description string) (int, error) {
	args := []string{"api", "repos/:owner/:repo/milestones", "-f", fmt.Sprintf("title=%s", title), "-f", fmt.Sprintf("description=%s", description), "--jq", ".number"}
	stdout, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return 0, fmt.Errorf("gh api milestone create failed: %s", stderr)
	}
	num, _ := strconv.Atoi(strings.TrimSpace(stdout))
	return num, nil
}

func ghUpdateMilestone(number int, title, description, state, dueOn string) error {
	args := []string{"api", fmt.Sprintf("repos/:owner/:repo/milestones/%d", number), "-X", "PATCH"}
	if title != "" {
		args = append(args, "-f", fmt.Sprintf("title=%s", title))
	}
	if description != "" {
		args = append(args, "-f", fmt.Sprintf("description=%s", description))
	}
	if state != "" {
		args = append(args, "-f", fmt.Sprintf("state=%s", state))
	}
	if dueOn != "" {
		// Append time if missing
		if !strings.Contains(dueOn, "T") {
			dueOn = dueOn + "T00:00:00Z"
		}
		args = append(args, "-f", fmt.Sprintf("due_on=%s", dueOn))
	}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh api milestone update failed: %s", stderr)
	}
	return nil
}

func ghDeleteMilestone(number int) error {
	args := []string{"api", fmt.Sprintf("repos/:owner/:repo/milestones/%d", number), "-X", "DELETE"}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh api milestone delete failed: %s", stderr)
	}
	return nil
}

func ghLinkParent(childIssueURL, parentIssueURL string) error {
	// Not implemented yet
	return nil
}

// #endregion Goals

func ghDeleteIssue(issueURLOrNumber string) error {
	args := []string{"issue", "delete", issueURLOrNumber, "--yes"}
	_, stderr, exitCode := ExecCommand("gh", args, "")
	if exitCode != 0 {
		return fmt.Errorf("gh issue delete failed: %s", stderr)
	}
	return nil
}
func ResolveContributorContributions(tickets []*Ticket) *ContributorContributionsTree {
	commitsMap := map[string]*Commit{}

	var allBundles []*Bundle
	if repoResolverInstance != nil && repoResolverInstance.Ctx != nil {
		allBundles = repoResolverInstance.Ctx.GetBundles()
	}

	GetBundleNameForPath := func(path string) string {
		for _, b := range allBundles {
			if strings.HasPrefix(path, b.Root) {
				return b.Name
			}
		}
		return "other"
	}

	for _, t := range tickets {
		sha := t.GetCommit()
		if sha != "" {
			if _, exists := commitsMap[sha]; !exists {
				commitsMap[sha] = &Commit{
					ID:    "@semio-repo/commit/" + sha,
					SHA:   sha,
					Title: t.GetTitle(),
					Date:  t.GetDateStarted(),
				}
			}
		}
	}

	commits := []*Commit{}
	for _, c := range commitsMap {
		commits = append(commits, c)
	}
	sort.Slice(commits, func(i, j int) bool {
		return commits[i].Date.After(commits[j].Date)
	})

	// Maps for Tree Construction
	type DefNode struct {
		Name  string
		Lines LineMetrics
	}
	type SecNode struct {
		Name  string
		Defs  map[string]*DefNode
		Lines LineMetrics
	}
	type FileNode struct {
		Name  string
		Secs  map[string]*SecNode
		Lines LineMetrics
	}
	type FolderNode struct {
		Name  string
		Files map[string]*FileNode
		Lines LineMetrics
	}
	type BundleNode struct {
		Name    string
		Folders map[string]*FolderNode
		Lines   LineMetrics
	}

	bundlesMap := map[string]*BundleNode{}

	getBundle := func(name string) *BundleNode {
		if _, ok := bundlesMap[name]; !ok {
			bundlesMap[name] = &BundleNode{Name: name, Folders: make(map[string]*FolderNode)}
		}
		return bundlesMap[name]
	}
	getFolder := func(b *BundleNode, path string) *FolderNode {
		if _, ok := b.Folders[path]; !ok {
			b.Folders[path] = &FolderNode{Name: filepath.Base(path), Files: make(map[string]*FileNode)}
		}
		return b.Folders[path]
	}
	getFile := func(f *FolderNode, name string) *FileNode {
		if _, ok := f.Files[name]; !ok {
			f.Files[name] = &FileNode{Name: name, Secs: make(map[string]*SecNode)}
		}
		return f.Files[name]
	}
	getSec := func(f *FileNode, name string) *SecNode {
		if _, ok := f.Secs[name]; !ok {
			f.Secs[name] = &SecNode{Name: name, Defs: make(map[string]*DefNode)}
		}
		return f.Secs[name]
	}
	getDef := func(s *SecNode, name string) *DefNode {
		if _, ok := s.Defs[name]; !ok {
			s.Defs[name] = &DefNode{Name: name}
		}
		return s.Defs[name]
	}

	processPath := func(fullPath string, bundleName string, lines *LineMetrics) {
		if bundleName == "" {
			bundleName = "other"
		}

		parts := strings.Split(fullPath, "#")
		filePath := parts[0]
		regionName := ""
		defName := ""

		if len(parts) > 1 {
			regionAndDef := parts[1]
			subParts := strings.Split(regionAndDef, "§")
			regionName = subParts[0]
			if len(subParts) > 1 {
				defName = subParts[1]
			}
		} else {
			subParts := strings.Split(fullPath, "§")
			if len(subParts) > 1 {
				filePath = subParts[0]
				defName = subParts[1]
			}
		}

		dir := filepath.Dir(filePath)
		file := filepath.Base(filePath)

		b := getBundle(bundleName)
		if lines != nil {
			b.Lines.Added += lines.Added
			b.Lines.Removed += lines.Removed
		}

		f := getFolder(b, dir)
		if lines != nil {
			f.Lines.Added += lines.Added
			f.Lines.Removed += lines.Removed
		}

		fi := getFile(f, file)
		if lines != nil {
			fi.Lines.Added += lines.Added
			fi.Lines.Removed += lines.Removed
		}

		if regionName != "" {
			s := getSec(fi, regionName)
			if lines != nil {
				s.Lines.Added += lines.Added
				s.Lines.Removed += lines.Removed
			}
			if defName != "" {
				d := getDef(s, defName)
				if lines != nil {
					d.Lines.Added += lines.Added
					d.Lines.Removed += lines.Removed
				}
			}
		} else if defName != "" {
			s := getSec(fi, "Global")
			if lines != nil {
				s.Lines.Added += lines.Added
				s.Lines.Removed += lines.Removed
			}
			d := getDef(s, defName)
			if lines != nil {
				d.Lines.Added += lines.Added
				d.Lines.Removed += lines.Removed
			}
		}
	}

	for _, t := range tickets {
		diffs := t.GetFiles()
		if diffs == nil {
			continue
		}
		iter := func(set TicketDiffSet) {
			for _, f := range set.Added {
				bn := GetBundleNameForPath(f.Path)
				processPath(f.Path, bn, f.Lines)
			}
			for _, f := range set.Modified {
				bn := GetBundleNameForPath(f.Path)
				processPath(f.Path, bn, f.Lines)
			}
			for _, f := range set.Renamed {
				bn := GetBundleNameForPath(f.To)
				processPath(f.To, bn, f.Lines)
			}
		}
		iter(diffs.Files)
		iter(diffs.Sections)
		iter(diffs.Definitions)
	}

	resBundles := []*ContributorBundle{}
	for _, b := range bundlesMap {
		cb := &ContributorBundle{Name: b.Name, Lines: b.Lines, Folders: []*ContributorFolder{}}
		for _, f := range b.Folders {
			cf := &ContributorFolder{Name: f.Name, Lines: f.Lines, Files: []*ContributorFile{}}
			for _, fi := range f.Files {
				cfi := &ContributorFile{Name: fi.Name, Lines: fi.Lines, Sections: []*ContributorSection{}}
				for _, s := range fi.Secs {
					cs := &ContributorSection{Name: s.Name, Lines: s.Lines, Definitions: []*ContributorDefinition{}}
					for _, d := range s.Defs {
						cs.Definitions = append(cs.Definitions, &ContributorDefinition{Name: d.Name, Lines: d.Lines})
					}
					sort.Slice(cs.Definitions, func(i, j int) bool { return cs.Definitions[i].Name < cs.Definitions[j].Name })
					cfi.Sections = append(cfi.Sections, cs)
				}
				sort.Slice(cfi.Sections, func(i, j int) bool { return cfi.Sections[i].Name < cfi.Sections[j].Name })
				cf.Files = append(cf.Files, cfi)
			}
			sort.Slice(cf.Files, func(i, j int) bool { return cf.Files[i].Name < cf.Files[j].Name })
			cb.Folders = append(cb.Folders, cf)
		}
		sort.Slice(cb.Folders, func(i, j int) bool { return cb.Folders[i].Name < cb.Folders[j].Name })
		resBundles = append(resBundles, cb)
	}
	sort.Slice(resBundles, func(i, j int) bool { return resBundles[i].Name < resBundles[j].Name })

	return &ContributorContributionsTree{
		Commits: commits,
		Tickets: tickets,
		Bundles: resBundles,
	}
}

// #endregion Cli

// #region Todos

func (c *repoContext) GetTodos(filter *FilterInput) ([]*Todo, error) {
	allTodos, err := ScanTodos(c.rootDir)
	if err != nil {
		return nil, err
	}
	if filter != nil && filter.Filter != nil {
		search := strings.ToLower(*filter.Filter)
		var match []*Todo
		for _, t := range allTodos {
			if strings.Contains(strings.ToLower(t.Name), search) || strings.Contains(strings.ToLower(t.Description), search) {
				match = append(match, t)
			}
		}
		return match, nil
	}
	return allTodos, nil
}

func ScanTodos(rootDir string) ([]*Todo, error) {
	var todos []*Todo
	err := filepath.WalkDir(rootDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() {
			if strings.HasPrefix(d.Name(), ".") && d.Name() != "." && d.Name() != ".semio-repo" {
				return fs.SkipDir
			}
			if d.Name() == "node_modules" || d.Name() == "dist" || d.Name() == "build" {
				return fs.SkipDir
			}
			// Parse .todos.md
			todoPath := filepath.Join(path, ".todos.md")
			if _, err := os.Stat(todoPath); err == nil {
				content, _ := os.ReadFile(todoPath)
				todos = append(todos, ParseTodoMarkdown(string(content), path)...)
			}
		} else {
			ext := filepath.Ext(path)
			// Supported extensions
			switch ext {
			case ".ts", ".js", ".tsx", ".jsx", ".go", ".cs", ".py", ".md", ".json":
				content, _ := os.ReadFile(path)
				todos = append(todos, ParseTodoComments(string(content), path)...)
			}
		}
		return nil
	})
	return todos, err
}

func ParseTodoMarkdown(content string, parentPath string) []*Todo {
	var todos []*Todo
	lines := strings.Split(content, "\n")
	for _, line := range lines {
		trimmed := strings.TrimSpace(line)
		if strings.HasPrefix(trimmed, "- TODO ") {
			rem := strings.TrimPrefix(trimmed, "- TODO ")
			parts := strings.SplitN(rem, ":", 2)
			name := strings.TrimSpace(parts[0])
			desc := ""
			if len(parts) > 1 {
				desc = strings.TrimSpace(parts[1])
			}
			id := Slugify(name)
			todos = append(todos, &Todo{
				ID:          id,
				Name:        name,
				Description: desc,
				ParentID:    parentPath,
				Location:    &Location{FilePath: filepath.Join(parentPath, ".todos.md")},
			})
		}
	}
	return todos
}

func ParseTodoComments(content string, filePath string) []*Todo {
	var todos []*Todo
	lines := strings.Split(content, "\n")
	// Matches "// TODO name: desc" or "# TODO name: desc"
	re := regexp.MustCompile(`^\s*(//|#|--)\s*TODO\s+([^:]+):\s*(.*)$`)
	for i, line := range lines {
		matches := re.FindStringSubmatch(line)
		if len(matches) > 3 {
			name := strings.TrimSpace(matches[2])
			desc := strings.TrimSpace(matches[3])
			id := Slugify(name)
			todos = append(todos, &Todo{
				ID:          id,
				Name:        name,
				Description: desc,
				ParentID:    filePath,
				Location:    &Location{FilePath: filePath, Line: i + 1, Column: 1},
			})
		}
	}
	return todos
}

func (c *repoContext) TodoCreate(input TodoCreateInput) (*Todo, error) {
	// Check if parent is folder
	info, err := os.Stat(input.ParentID)
	if err == nil && info.IsDir() {
		todoPath := filepath.Join(input.ParentID, ".todos.md")
		line := fmt.Sprintf("- TODO %s: %s\n", input.Name, input.Description)
		f, err := os.OpenFile(todoPath, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
		if err != nil {
			return nil, err
		}
		defer f.Close()
		if _, err := f.WriteString(line); err != nil {
			return nil, err
		}
		return &Todo{
			ID:          Slugify(input.Name),
			Name:        input.Name,
			Description: input.Description,
			ParentID:    input.ParentID,
		}, nil
	}
	// Check if parent is file
	if err == nil && !info.IsDir() {
		ext := filepath.Ext(input.ParentID)
		prefix := "//"
		if ext == ".py" || ext == ".sh" || ext == ".yaml" || ext == ".yml" {
			prefix = "#"
		} else if ext == ".sql" || ext == ".lua" {
			prefix = "--"
		}
		line := fmt.Sprintf("\n%s TODO %s: %s\n", prefix, input.Name, input.Description)
		f, err := os.OpenFile(input.ParentID, os.O_APPEND|os.O_WRONLY, 0644)
		if err != nil {
			return nil, err
		}
		defer f.Close()
		if _, err := f.WriteString(line); err != nil {
			return nil, err
		}
		return &Todo{
			ID:          Slugify(input.Name),
			Name:        input.Name,
			Description: input.Description,
			ParentID:    input.ParentID,
			Location:    &Location{FilePath: input.ParentID},
		}, nil
	}
	return nil, fmt.Errorf("invalid parent id (must be path to folder or file)")
}

func (c *repoContext) TodoChange(input TodoChangeInput) (*Todo, error) {
	return nil, fmt.Errorf("not implemented")
}

func (c *repoContext) TodoDelete(id string) (bool, error) {
	todos, err := ScanTodos(c.rootDir)
	if err != nil {
		return false, err
	}
	for _, t := range todos {
		if t.ID == id {
			if strings.HasSuffix(t.Location.FilePath, ".todos.md") {
				removeLineFromMarkdown(t.Location.FilePath, t.Name)
				// Re-read file to verify?
				return true, nil
			}
			if t.Location.Line > 0 {
				removeLineFromFile(t.Location.FilePath, t.Location.Line)
				return true, nil
			}
		}
	}
	return false, fmt.Errorf("todo not found")
}

func (c *repoContext) TodoToTicket(id string, input TicketOpenInput) (*Ticket, error) {
	todos, _ := ScanTodos(c.rootDir)
	var todo *Todo
	for _, t := range todos {
		if t.ID == id {
			todo = t
			break
		}
	}
	if todo == nil {
		return nil, fmt.Errorf("todo not found")
	}

	if input.Title == "" {
		input.Title = todo.Name
	}
	input.Prompt = fmt.Sprintf("%s\n\n%s", todo.Description, input.Prompt)

	ticket, err := c.TicketOpen(input)
	if err != nil {
		return nil, err
	}
	c.TodoDelete(id)
	return ticket, nil
}

func removeLineFromMarkdown(path, name string) {
	content, _ := os.ReadFile(path)
	lines := strings.Split(string(content), "\n")
	var newLines []string
	prefix := "- TODO " + name + ":"
	for _, line := range lines {
		if strings.HasPrefix(strings.TrimSpace(line), prefix) {
			continue
		}
		newLines = append(newLines, line)
	}
	os.WriteFile(path, []byte(strings.Join(newLines, "\n")), 0644)
}



func removeLineFromFile(path string, lineNum int) {
content, _ := os.ReadFile(path)
lines := strings.Split(string(content), "\n")
if lineNum > 0 && lineNum <= len(lines) {
newLines := append(lines[:lineNum-1], lines[lineNum:]...)
os.WriteFile(path, []byte(strings.Join(newLines, "\n")), 0644)
}
}
