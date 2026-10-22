// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Agent sessions: whose call a build operation is. The repository's own `repo session open` issues the session (identity, lane, slot preference, namespaces, budget, capability) and this server only asks for it, once per agent, and hands the child its address.

// #endregion 🧲️Header

package main

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"strconv"
	"strings"
	"sync"
	"time"
)

// #region 🪪️AgentSession

// 🪪️AgentIdentity is the agent session of one call as `repo session open --json` reports it: the agent name, the session identifier, the cargo lane and the address children attach to.
type AgentIdentity struct {
	ID      string
	Session string
	Lane    string
	Address string
}

// 🪪️SessionRequest names what identifies the agent of a call: an explicit agent name, or the client of the connection with the process ids that make the connection unique.
type SessionRequest struct {
	Agent         string
	Client        string
	ClientProcess int
	Process       int
}

// 🪪️Argv is the `repo` argument vector that opens or attaches the session of a request.
func (request SessionRequest) Argv() []string {
	argv := []string{"session", "open", "--json"}
	switch {
	case request.Agent != "":
		argv = append(argv, "--agent="+request.Agent)
	case request.Client != "":
		argv = append(argv, "--client="+request.Client)
		if request.ClientProcess > 0 {
			argv = append(argv, "--client-pid="+strconv.Itoa(request.ClientProcess))
		}
		if request.Process > 0 {
			argv = append(argv, "--owner-pid="+strconv.Itoa(request.Process))
		}
	}
	return argv
}

// 🪪️SessionOpener opens or attaches the session of a request.
type SessionOpener interface {
	Open(ctx context.Context, request SessionRequest) (AgentIdentity, error)
}

// 🪪️ParseSession reads the JSON `repo session open --json` prints.
func ParseSession(data []byte) (AgentIdentity, error) {
	var reply struct {
		Address string `json:"address"`
		Session struct {
			ID    string `json:"id"`
			Agent struct {
				ID string `json:"id"`
			} `json:"agent"`
			Resources struct {
				Cargo struct {
					Lane string `json:"lane"`
				} `json:"cargo"`
			} `json:"resources"`
		} `json:"session"`
	}
	if err := json.Unmarshal(data, &reply); err != nil {
		return AgentIdentity{}, fmt.Errorf("agent session reply is not JSON: %w", err)
	}
	if reply.Address == "" || reply.Session.ID == "" || reply.Session.Agent.ID == "" {
		return AgentIdentity{}, errors.New("agent session reply lacks the address, the identifier or the agent")
	}
	return AgentIdentity{ID: reply.Session.Agent.ID, Session: reply.Session.ID, Lane: reply.Session.Resources.Cargo.Lane, Address: reply.Address}, nil
}

// 🪪️surfaceSessions asks the `repo` executable of a build surface, remembering every answered request.
type surfaceSessions struct {
	surface *BuildSurface
	mu      sync.Mutex
	known   map[SessionRequest]AgentIdentity
}

const sessionTimeout = 60 * time.Second

// 🪪️Open runs `repo session open --json` once per distinct request; a failure is not remembered.
func (sessions *surfaceSessions) Open(ctx context.Context, request SessionRequest) (AgentIdentity, error) {
	sessions.mu.Lock()
	if identity, found := sessions.known[request]; found {
		sessions.mu.Unlock()
		return identity, nil
	}
	sessions.mu.Unlock()
	surface := sessions.surface
	limits := surface.Policy.Limits
	spec := ProcessSpec{Binary: surface.Binary, Prefix: surface.Prefix, Argv: request.Argv(), Dir: surface.Root, Env: surface.Environment(Invocation{}, AgentIdentity{}), Grace: time.Duration(limits.CancelGraceMs) * time.Millisecond, HeadMax: limits.CaptureBytes, StdoutMax: limits.StdoutBytes, StderrMax: limits.StderrBytes}
	opening, cancel := context.WithTimeout(ctx, sessionTimeout)
	defer cancel()
	outcome := surface.Runner.Run(opening, spec, func(ProcessEvent) {})
	if !outcome.Started {
		return AgentIdentity{}, errors.New("repo did not start: " + outcome.StartError)
	}
	if outcome.ExitCode != 0 {
		return AgentIdentity{}, fmt.Errorf("repo session open failed with exit %d: %s", outcome.ExitCode, strings.TrimSpace(string(outcome.Stderr.Tail)))
	}
	identity, err := ParseSession(outcome.Stdout.Head)
	if err != nil {
		return AgentIdentity{}, err
	}
	sessions.mu.Lock()
	sessions.known[request] = identity
	sessions.mu.Unlock()
	return identity, nil
}

// 🌿️Environment projects the session onto the one variable the repository tools read; a call without a session carries nothing and its child resolves its own.
func (agent AgentIdentity) Environment(names AgentEnvironment) map[string]string {
	if agent.Address == "" {
		return map[string]string{}
	}
	return map[string]string{names.Session: agent.Address}
}

// 🌿️AgentEnvironment names the variable of the authored policy.
type AgentEnvironment struct {
	Session string `json:"session"`
}

// #endregion 🪪️AgentSession
