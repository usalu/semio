// #region Header
// SPDX-License-Identifier: AGPL-3.0-or-later
// #endregion Header

// #region Package
package mcpadapter

// #endregion Package

// #region Imports
import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/mark3labs/mcp-go/mcp"
	"github.com/mark3labs/mcp-go/server"
	"github.com/usalu/semio/go/repo/internal/core"
	"github.com/usalu/semio/go/repo/internal/events"
)

// #endregion Imports

// #region Server
func Serve(ctx context.Context, engine *core.Engine) error {
	s := server.NewMCPServer(
		"semio-repo",
		"1.0.0",
		server.WithToolCapabilities(true),
	)
	s.AddTool(
		mcp.NewTool("graphql_query",
			mcp.WithDescription("Execute a GraphQL query"),
			mcp.WithString("query", mcp.Required(), mcp.Description("GraphQL query")),
			mcp.WithString("variables", mcp.Description("GraphQL variables JSON")),
		),
		graphqlQuery(engine),
	)
	s.AddTool(
		mcp.NewTool("graphql",
			mcp.WithDescription("Execute a GraphQL query"),
			mcp.WithString("query", mcp.Required(), mcp.Description("GraphQL query")),
			mcp.WithString("variables", mcp.Description("GraphQL variables JSON")),
		),
		graphqlQuery(engine),
	)
	return server.ServeStdio(s)
}

// #endregion Server

// #region Tools
func graphqlQuery(engine *core.Engine) func(context.Context, mcp.CallToolRequest) (*mcp.CallToolResult, error) {
	return func(ctx context.Context, request mcp.CallToolRequest) (*mcp.CallToolResult, error) {
		args := getArgs(request)
		query, err := requireStringArg(args, "query")
		if err != nil {
			return nil, err
		}
		var variables map[string]interface{}
		if variablesRaw, ok := args["variables"]; ok {
			if variablesMap, mapOk := variablesRaw.(map[string]interface{}); mapOk {
				variables = variablesMap
			} else if variablesString, strOk := variablesRaw.(string); strOk && variablesString != "" {
				if err := json.Unmarshal([]byte(variablesString), &variables); err != nil {
					return nil, fmt.Errorf("invalid variables JSON: %w", err)
				}
			}
		}
		payload, err := json.Marshal(core.GraphQLArgs{Query: query, Variables: variables})
		if err != nil {
			return nil, err
		}
		stream := engine.Run(ctx, core.Request{Command: core.CmdGraphQL, Args: payload})
		var lastResult []byte
		for event := range stream {
			if event.Kind == events.KindResult && len(event.Data) > 0 {
				lastResult = event.Data
			}
			if event.Kind == events.KindError && event.Error != nil && event.Error.Fatal {
				return nil, fmt.Errorf("%s", event.Error.Message)
			}
		}
		if len(lastResult) == 0 {
			lastResult = []byte("{}")
		}
		return mcp.NewToolResultText(string(lastResult)), nil
	}
}

// #endregion Tools
