// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Agent identity: whose call a build operation is, passed to the child so its leases, capacity grants, cache lanes and compiler sessions attribute to that agent.

// #endregion 🧲️Header

package main

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"math/big"
	"strings"
)

// #region 🪪️AgentIdentity

const maximumAgentIdentifier = 64

// 🪪️AgentIdentity names one agent in the three spellings the repository tools accept.
type AgentIdentity struct {
	ID      string
	Session string
	Lane    string
}

// 🧼️SanitizeAgent keeps `[A-Za-z0-9._-]`, replaces every other character and bounds the length; the result is empty only for empty input.
func SanitizeAgent(raw string) string {
	var out strings.Builder
	for _, character := range strings.TrimSpace(raw) {
		switch {
		case character >= 'a' && character <= 'z', character >= 'A' && character <= 'Z', character >= '0' && character <= '9', character == '.', character == '_', character == '-':
			out.WriteRune(character)
		default:
			out.WriteByte('_')
		}
		if out.Len() >= maximumAgentIdentifier {
			break
		}
	}
	return out.String()
}

// 🏷️AgentLane derives the Cargo lane of an agent: `a` and ten base-36 characters of its digest, which is the shape of the lane the repository derives from an agent client process, so an agent keeps one lane across server restarts.
func AgentLane(agent string) string {
	digest := sha256.Sum256([]byte("mcp\x00" + agent))
	number, _ := new(big.Int).SetString(hex.EncodeToString(digest[:6]), 16)
	return "a" + number.Text(36)[:min(10, len(number.Text(36)))]
}

// 🪪️NewAgent completes an identifier into an identity.
func NewAgent(id string) AgentIdentity {
	id = SanitizeAgent(id)
	return AgentIdentity{ID: id, Session: id, Lane: AgentLane(id)}
}

// 🔌️ConnectionAgent names the agent of one MCP connection: the first non-empty source variable, else the client name with a digest of the client process and this process, so two connections never share an agent by accident.
func ConnectionAgent(lookup func(string) (string, bool), sources []string, clientName string, clientProcess, process int) AgentIdentity {
	for _, name := range sources {
		if value, found := lookup(name); found && SanitizeAgent(value) != "" {
			return NewAgent(SanitizeAgent(value))
		}
	}
	name := strings.ToLower(SanitizeAgent(clientName))
	if name == "" {
		name = "agent"
	}
	digest := sha256.Sum256([]byte(fmt.Sprintf("%d:%d", clientProcess, process)))
	return NewAgent(name[:min(len(name), 40)] + "-" + hex.EncodeToString(digest[:4]))
}

// 🌿️Environment projects the identity onto the variables the repository tools read.
func (agent AgentIdentity) Environment(names AgentEnvironment) map[string]string {
	return map[string]string{names.Agent: agent.ID, names.Session: agent.Session, names.Lane: agent.Lane}
}

// 🌿️AgentEnvironment names the three variables of the authored policy.
type AgentEnvironment struct {
	Agent   string `json:"agent"`
	Session string `json:"session"`
	Lane    string `json:"lane"`
}

// #endregion 🪪️AgentIdentity
