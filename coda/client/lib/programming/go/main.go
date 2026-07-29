// #region 🧲Header
//
// Summary: Go validator for __KEEP_pluginming__ target. Validates space programs (area constraints) and adjacency matrices.
//
// Specs: Reads translation JSON from stdin, program requirements from .progam/config.json or .coda/__KEEP_pluginming__-requirements.json, outputs report JSON to stdout.
//
// #endregion

package main

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
)

// #region 🗄️Translation
// Translation input from compose-to-__KEEP_pluginming__ translator.

type Translation struct {
	TargetID string  `json:"target_id"`
	DesignID string  `json:"design_id"`
	Rooms    []Room  `json:"rooms"`
	Totals   map[string]float64 `json:"totals"`
}

type Room struct {
	ID           string   `json:"id"`
	Program      string   `json:"program"`
	Area         float64  `json:"area"`
	Adjacencies  []string `json:"adjacencies"`
}

// #endregion

// #region 🎯Requirements
// Program requirements config (space program + adjacency matrix).

type Requirements struct {
	Programs   []ProgramDef     `json:"programs"`
	ByKind     []KindConstraint `json:"byKind"`
	Adjacency  []AdjacencyRule  `json:"adjacency"`
	Adjancency []AdjacencyRule  `json:"adjancency"` // typo in existing config
}

type ProgramDef struct {
	ID     string `json:"id"`
	Name   string `json:"name"`
	DIN276 string `json:"din276"`
}

type KindConstraint struct {
	Kind       string          `json:"kind"`
	Constraints AreaConstraints `json:"constraints"`
}

type AreaConstraints struct {
	Min *float64 `json:"min"`
	Max *float64 `json:"max"`
}

type AdjacencyRule struct {
	From string `json:"from"`
	To   string `json:"to"`
	Type string `json:"type"` // mandatory, desirable, neutral, negative
}

// #endregion

// #region 🎃Report
// Validation report output.

type Report struct {
	Rules []Rule `json:"rules"`
}

type Rule struct {
	ID      string   `json:"id"`
	Status  string   `json:"status"` // compliant, violated, not-applicable
	Clauses []Clause `json:"clauses"`
}

type Clause struct {
	ID     string `json:"id,omitempty"`
	Status string `json:"status,omitempty"`
	Actual string `json:"actual,omitempty"`
	Should string `json:"should,omitempty"`
}

// #endregion

func main() {
	if err := run(); err != nil {
		fmt.Fprintf(os.Stderr, "[DEBUG] __KEEP_pluginming__ validator error: %v\n", err)
		os.Exit(1)
	}
}

func run() error {
	var trans Translation
	if err := json.NewDecoder(os.Stdin).Decode(&trans); err != nil {
		return fmt.Errorf("decode translation: %w", err)
	}

	req, err := loadRequirements()
	if err != nil {
		return fmt.Errorf("load requirements: %w", err)
	}

	report := validate(trans, req)
	if err := json.NewEncoder(os.Stdout).Encode(report); err != nil {
		return fmt.Errorf("encode report: %w", err)
	}
	return nil
}

func loadRequirements() (*Requirements, error) {
	cwd, err := os.Getwd()
	if err != nil {
		return nil, err
	}
	for _, p := range []string{
		filepath.Join(cwd, ".progam", "config.json"),
		filepath.Join(cwd, ".coda", "__KEEP_pluginming__-requirements.json"),
	} {
		data, err := os.ReadFile(p)
		if err == nil {
			var req Requirements
			if err := json.Unmarshal(data, &req); err != nil {
				return nil, fmt.Errorf("parse %s: %w", p, err)
			}
			if len(req.Adjacency) == 0 && len(req.Adjancency) > 0 {
				req.Adjacency = req.Adjancency
			}
			return &req, nil
		}
	}
	return &Requirements{}, nil
}

func validate(trans Translation, req *Requirements) Report {
	var rules []Rule

	// Space program: area constraints per kind
	byKind := make(map[string]AreaConstraints)
	for _, k := range req.ByKind {
		byKind[k.Kind] = k.Constraints
	}
	for kind, c := range byKind {
		actual := trans.Totals[kind]
		if c.Min == nil && c.Max == nil {
			continue
		}
		rule := validateAreaRule(kind, actual, c)
		if rule != nil {
			rules = append(rules, *rule)
		}
	}

	// Adjacency matrix: mandatory adjacencies
	adjRules := req.Adjacency
	if len(adjRules) == 0 {
		adjRules = req.Adjancency
	}
	for _, ar := range adjRules {
		if ar.Type != "mandatory" {
			continue
		}
		rule := validateAdjacencyRule(trans.Rooms, ar)
		if rule != nil {
			rules = append(rules, *rule)
		}
	}

	if len(rules) == 0 {
		rules = append(rules, Rule{
			ID:     "__KEEP_pluginming__",
			Status: "compliant",
			Clauses: []Clause{{ID: "all", Status: "compliant"}},
		})
	}

	return Report{Rules: rules}
}

func validateAreaRule(kind string, actual float64, c AreaConstraints) *Rule {
	ruleID := "program-area-" + kind
	if c.Min != nil && actual < *c.Min {
		return &Rule{
			ID:     ruleID,
			Status: "violated",
			Clauses: []Clause{{
				ID:     fmt.Sprintf(">=%.0fm²", *c.Min),
				Actual: fmt.Sprintf("%.0fm²", actual),
				Should: fmt.Sprintf("%.0fm²", *c.Min),
			}},
		}
	}
	if c.Max != nil && actual > *c.Max {
		return &Rule{
			ID:     ruleID,
			Status: "violated",
			Clauses: []Clause{{
				ID:     fmt.Sprintf("<=%.0fm²", *c.Max),
				Actual: fmt.Sprintf("%.0fm²", actual),
				Should: fmt.Sprintf("%.0fm²", *c.Max),
			}},
		}
	}
	return &Rule{
		ID:     ruleID,
		Status: "compliant",
		Clauses: []Clause{{ID: "area", Status: "compliant"}},
	}
}

func validateAdjacencyRule(rooms []Room, ar AdjacencyRule) *Rule {
	adjSet := buildAdjacencySet(rooms)
	ruleID := "adjacency-" + ar.From + "-" + ar.To

	fromRooms := roomsWithProgram(rooms, ar.From)
	toRooms := roomsWithProgram(rooms, ar.To)
	if len(fromRooms) == 0 || len(toRooms) == 0 {
		return &Rule{
			ID:     ruleID,
			Status: "not-applicable",
			Clauses: []Clause{{ID: "no-rooms", Status: "not-applicable"}},
		}
	}

	for _, fr := range fromRooms {
		hasAdj := false
		for _, tr := range toRooms {
			if fr.ID != tr.ID && adjSet[adjKey(fr.ID, tr.ID)] {
				hasAdj = true
				break
			}
		}
		if !hasAdj {
			return &Rule{
				ID:     ruleID,
				Status: "violated",
				Clauses: []Clause{{
					ID:     "mandatory-adjacency",
					Actual: fmt.Sprintf("room %s (%s) not adjacent to %s", fr.ID, ar.From, ar.To),
					Should: fmt.Sprintf("room %s must be adjacent to at least one %s", fr.ID, ar.To),
				}},
			}
		}
	}
	return &Rule{
		ID:     ruleID,
		Status: "compliant",
		Clauses: []Clause{{ID: "adjacency", Status: "compliant"}},
	}
}

func buildAdjacencySet(rooms []Room) map[string]bool {
	set := make(map[string]bool)
	for _, r := range rooms {
		for _, a := range r.Adjacencies {
			set[adjKey(r.ID, a)] = true
		}
	}
	return set
}

func adjKey(a, b string) string {
	if a < b {
		return a + "|" + b
	}
	return b + "|" + a
}

func roomsWithProgram(rooms []Room, program string) []Room {
	var out []Room
	for _, r := range rooms {
		if r.Program == program {
			out = append(out, r)
		}
	}
	return out
}
