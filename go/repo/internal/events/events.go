// #region Header
// SPDX-License-Identifier: AGPL-3.0-or-later
// #endregion Header

// #region Package
package events

// #endregion Package

// #region Imports
import "encoding/json"

// #endregion Imports

// #region Types
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

// #endregion Types
