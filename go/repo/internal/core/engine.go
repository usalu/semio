// #region Header
// SPDX-License-Identifier: AGPL-3.0-or-later
// #endregion Header

// #region Package
package core

// #endregion Package

// #region Imports
import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/usalu/semio/go/repo/internal/events"
)

// #endregion Imports

// #region Types
type GraphQLExecutor interface {
	Execute(ctx context.Context, query string, variables map[string]interface{}) (interface{}, error)
}

type Engine struct {
	GraphQL GraphQLExecutor
}

// #endregion Types

// #region Constructors
func NewEngine(graphql GraphQLExecutor) *Engine {
	return &Engine{GraphQL: graphql}
}

// #endregion Constructors

// #region Engine
func (e *Engine) Run(ctx context.Context, req Request) <-chan events.Event {
	out := make(chan events.Event)
	go func() {
		defer func() {
			if recovered := recover(); recovered != nil {
				e.emitError(out, req, events.ErrPayload{Code: string(ErrInternal), Message: "internal error", Detail: fmt.Sprintf("%v", recovered), Fatal: true})
				e.emitDone(out, exitCodeError, "error")
			}
			close(out)
		}()

		e.emitStart(out, req)

		if ctx.Err() != nil {
			e.emitError(out, req, events.ErrPayload{Code: string(ErrCanceled), Message: ctx.Err().Error(), Fatal: true})
			e.emitDone(out, exitCodeCanceled, "canceled")
			return
		}

		switch req.Command {
		case CmdGraphQL, CmdAnalyze, CmdFix, CmdPolicy, CmdTicket, CmdBundle, CmdFolder, CmdFile, CmdSection, CmdDef:
			e.runGraphQL(ctx, req, out)
		default:
			e.emitError(out, req, events.ErrPayload{Code: string(ErrInternal), Message: "unsupported command", Fatal: true})
			e.emitDone(out, exitCodeError, "error")
		}
	}()
	return out
}

// #endregion Engine

// #region GraphQL
func (e *Engine) runGraphQL(ctx context.Context, req Request, out chan<- events.Event) {
	var args GraphQLArgs
	if err := json.Unmarshal(req.Args, &args); err != nil {
		e.emitError(out, req, events.ErrPayload{Code: string(ErrParse), Message: "invalid arguments", Detail: err.Error(), Fatal: true})
		e.emitDone(out, exitCodeUsage, "error")
		return
	}
	if e.GraphQL == nil {
		e.emitError(out, req, events.ErrPayload{Code: string(ErrInternal), Message: "graphql executor missing", Fatal: true})
		e.emitDone(out, exitCodeError, "error")
		return
	}
	result, err := e.GraphQL.Execute(ctx, args.Query, args.Variables)
	if err != nil {
		e.emitError(out, req, events.ErrPayload{Code: string(ErrInternal), Message: err.Error(), Fatal: true})
		e.emitDone(out, exitCodeError, "error")
		return
	}
	payload, err := json.Marshal(map[string]interface{}{"data": result})
	if err != nil {
		e.emitError(out, req, events.ErrPayload{Code: string(ErrInternal), Message: "failed to encode result", Detail: err.Error(), Fatal: true})
		e.emitDone(out, exitCodeError, "error")
		return
	}
	out <- events.Event{Kind: events.KindResult, Command: string(req.Command), Data: payload}
	e.emitDone(out, exitCodeOK, "ok")
}

// #endregion GraphQL

// #region Emitters
func (e *Engine) emitStart(out chan<- events.Event, req Request) {
	out <- events.Event{Kind: events.KindStart, Command: string(req.Command)}
}

func (e *Engine) emitError(out chan<- events.Event, req Request, payload events.ErrPayload) {
	out <- events.Event{Kind: events.KindError, Command: string(req.Command), Error: &payload}
}

func (e *Engine) emitDone(out chan<- events.Event, code int, status string) {
	out <- events.Event{Kind: events.KindDone, Done: &events.DonePayload{ExitCode: code, Status: status}}
}

// #endregion Emitters

// #region ExitCodes
const (
	exitCodeOK       = 0
	exitCodeError    = 1
	exitCodeUsage    = 2
	exitCodeCanceled = 130
)

// #endregion ExitCodes
