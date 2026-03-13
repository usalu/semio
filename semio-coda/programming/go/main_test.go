// #region 🔖Header
// [🔬coda📦programming🥼maintest](semiorepo://p/r/coda/b/l/programming/f/main_test.go)
//
// Summary: Tests for programming validator.
//
// #endregion

package main

import (
	"bytes"
	"encoding/json"
	"strings"
	"testing"
)

func TestValidateAreaRuleViolated(t *testing.T) {
	trans := Translation{
		Rooms: []Room{},
		Totals: map[string]float64{"area.gfa.net.usbl.ofc-work.ofc-rm": 270},
	}
	req := &Requirements{
		ByKind: []KindConstraint{{
			Kind: "area.gfa.net.usbl.ofc-work.ofc-rm",
			Constraints: AreaConstraints{Min: ptr(300.0)},
		}},
	}
	report := validate(trans, req)
	if len(report.Rules) == 0 {
		t.Fatal("expected at least one rule")
	}
	var violated *Rule
	for i := range report.Rules {
		if report.Rules[i].Status == "violated" {
			violated = &report.Rules[i]
			break
		}
	}
	if violated == nil {
		t.Fatal("expected violated rule for area < min")
	}
	if violated.Clauses[0].Actual != "270m²" || violated.Clauses[0].Should != "300m²" {
		t.Errorf("clause: actual=%q should=%q", violated.Clauses[0].Actual, violated.Clauses[0].Should)
	}
}

func TestValidateAreaRuleCompliant(t *testing.T) {
	trans := Translation{
		Totals: map[string]float64{"area.gfa.net.usbl.ofc-work.ofc-rm": 350},
	}
	req := &Requirements{
		ByKind: []KindConstraint{{
			Kind: "area.gfa.net.usbl.ofc-work.ofc-rm",
			Constraints: AreaConstraints{Min: ptr(300.0)},
		}},
	}
	report := validate(trans, req)
	var areaRule *Rule
	for i := range report.Rules {
		if strings.Contains(report.Rules[i].ID, "program-area") {
			areaRule = &report.Rules[i]
			break
		}
	}
	if areaRule == nil {
		t.Fatal("expected program-area rule")
	}
	if areaRule.Status != "compliant" {
		t.Errorf("expected compliant, got %s", areaRule.Status)
	}
}

func TestValidateAdjacencyMandatoryViolated(t *testing.T) {
	trans := Translation{
		Rooms: []Room{
			{ID: "a", Program: "office", Adjacencies: []string{}},
			{ID: "b", Program: "lobby", Adjacencies: []string{}},
		},
		Totals: map[string]float64{},
	}
	req := &Requirements{
		Adjacency: []AdjacencyRule{{
			From: "office", To: "lobby", Type: "mandatory",
		}},
	}
	report := validate(trans, req)
	var adjRule *Rule
	for i := range report.Rules {
		if strings.Contains(report.Rules[i].ID, "adjacency") {
			adjRule = &report.Rules[i]
			break
		}
	}
	if adjRule == nil {
		t.Fatal("expected adjacency rule")
	}
	if adjRule.Status != "violated" {
		t.Errorf("expected violated (office not adjacent to lobby), got %s", adjRule.Status)
	}
}

func TestValidateAdjacencyMandatoryCompliant(t *testing.T) {
	trans := Translation{
		Rooms: []Room{
			{ID: "a", Program: "office", Adjacencies: []string{"b"}},
			{ID: "b", Program: "lobby", Adjacencies: []string{"a"}},
		},
		Totals: map[string]float64{},
	}
	req := &Requirements{
		Adjacency: []AdjacencyRule{{
			From: "office", To: "lobby", Type: "mandatory",
		}},
	}
	report := validate(trans, req)
	var adjRule *Rule
	for i := range report.Rules {
		if strings.Contains(report.Rules[i].ID, "adjacency") {
			adjRule = &report.Rules[i]
			break
		}
	}
	if adjRule == nil {
		t.Fatal("expected adjacency rule")
	}
	if adjRule.Status != "compliant" {
		t.Errorf("expected compliant, got %s", adjRule.Status)
	}
}

func TestRunEndToEnd(t *testing.T) {
	input := `{"target_id":"programming","design_id":"x","rooms":[{"id":"r1","program":"office","area":100,"adjacencies":["r2"]},{"id":"r2","program":"office","area":250,"adjacencies":["r1"]}],"totals":{"office":350}}`
	req := &Requirements{
		ByKind: []KindConstraint{{
			Kind: "office",
			Constraints: AreaConstraints{Min: ptr(300.0)},
		}},
	}
	var trans Translation
	if err := json.NewDecoder(strings.NewReader(input)).Decode(&trans); err != nil {
		t.Fatal(err)
	}
	report := validate(trans, req)
	var buf bytes.Buffer
	if err := json.NewEncoder(&buf).Encode(report); err != nil {
		t.Fatal(err)
	}
	var decoded Report
	if err := json.NewDecoder(&buf).Decode(&decoded); err != nil {
		t.Fatal(err)
	}
	if len(decoded.Rules) == 0 {
		t.Fatal("expected rules in report")
	}
}

func ptr(f float64) *float64 { return &f }
