// #region Header
// SPDX-License-Identifier: AGPL-3.0-or-later
// #endregion Header

// #region Package
package core

// #endregion Package

// #region Imports
import "encoding/json"

// #endregion Imports

// #region Types
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

// #endregion Types
