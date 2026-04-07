// #region 🧲Header
//
// Summary: Tests for blnbo (Berlin Building Code) validator.
//
// #endregion

package main

import (
	"bytes"
	"encoding/json"
	"strings"
	"testing"
)

// #region 🦉StaircaseLocatedTests

func TestStaircaseLocated_NoStaircases(t *testing.T) {
	trans := Translation{
		Properties: BuildingProperties{Height: 10, BuildingClass: "3"},
	}
	report := validate(trans)
	rule := findRule(t, report, "staircase-located")
	if rule.Status != "not-applicable" {
		t.Errorf("expected not-applicable, got %s", rule.Status)
	}
}

func TestStaircaseLocated_ExternalStaircaseExempt(t *testing.T) {
	trans := Translation{
		Properties: BuildingProperties{Height: 10, BuildingClass: "3"},
		Staircases: []Staircase{{
			ID:       "sc1",
			Kind:     "necessary",
			External: true,
			InSeparateStairwell: true,
			Connects: []string{"s0", "s1"},
		}},
	}
	report := validate(trans)
	rule := findRule(t, report, "staircase-located")
	if rule.Status != "compliant" {
		t.Errorf("expected compliant (external exempt), got %s", rule.Status)
	}
	assertHasClauseWithStatus(t, rule.Clauses, "exempt")
}

func TestStaircaseLocated_BuildingClass1Exempt(t *testing.T) {
	trans := Translation{
		Properties: BuildingProperties{Height: 6, BuildingClass: "1"},
		Staircases: []Staircase{{
			ID:       "sc1",
			Kind:     "necessary",
			External: false,
			InSeparateStairwell: false,
			Connects: []string{"s0", "s1"},
		}},
	}
	report := validate(trans)
	rule := findRule(t, report, "staircase-located")
	if rule.Status != "compliant" {
		t.Errorf("expected compliant (building class 1 exempt), got %s", rule.Status)
	}
}

func TestStaircaseLocated_BuildingClass2Exempt(t *testing.T) {
	trans := Translation{
		Properties: BuildingProperties{Height: 6, BuildingClass: "2"},
		Staircases: []Staircase{{
			ID:       "sc1",
			Kind:     "necessary",
			External: false,
			InSeparateStairwell: false,
			Connects: []string{"s0", "s1"},
		}},
	}
	report := validate(trans)
	rule := findRule(t, report, "staircase-located")
	if rule.Status != "compliant" {
		t.Errorf("expected compliant (building class 2 exempt), got %s", rule.Status)
	}
}

func TestStaircaseLocated_TwoStoreySmallUnitExempt(t *testing.T) {
	trans := Translation{
		Properties: BuildingProperties{Height: 10, BuildingClass: "3"},
		UsageUnits: []UsageUnit{{ID: "uu0", TotalGrossFloorArea: 180.0}},
		Storeys: []Storey{
			{ID: "s0", UsageUnitID: "uu0", EscapeRoutes: []string{"er1"}},
			{ID: "s1", UsageUnitID: "uu0", EscapeRoutes: []string{"er2"}},
		},
		Staircases: []Staircase{{
			ID:                  "sc1",
			Kind:                "necessary",
			External:            false,
			InSeparateStairwell: false,
			Connects:            []string{"s0", "s1"},
		}},
	}
	report := validate(trans)
	rule := findRule(t, report, "staircase-located")
	if rule.Status != "compliant" {
		t.Errorf("expected compliant (two-storey small usage unit exempt), got %s", rule.Status)
	}
}

func TestStaircaseLocated_TwoStoreyLargeUnit_NotExempt(t *testing.T) {
	trans := Translation{
		Properties: BuildingProperties{Height: 10, BuildingClass: "3"},
		UsageUnits: []UsageUnit{{ID: "uu0", TotalGrossFloorArea: 306.6}},
		Storeys: []Storey{
			{ID: "s0", UsageUnitID: "uu0", EscapeRoutes: []string{"er1"}},
			{ID: "s1", UsageUnitID: "uu0", EscapeRoutes: []string{"er2"}},
		},
		Staircases: []Staircase{{
			ID:                  "sc1",
			Kind:                "necessary",
			External:            false,
			InSeparateStairwell: false,
			Connects:            []string{"s0", "s1"},
		}},
	}
	report := validate(trans)
	rule := findRule(t, report, "staircase-located")
	if rule.Status != "violated" {
		t.Errorf("expected violated (large usage unit, no separate stairwell), got %s", rule.Status)
	}
}

func TestStaircaseLocated_ThreeStoreys_NotExempt(t *testing.T) {
	trans := Translation{
		Properties: BuildingProperties{Height: 10, BuildingClass: "3"},
		UsageUnits: []UsageUnit{{ID: "uu0", TotalGrossFloorArea: 150.0}},
		Storeys: []Storey{
			{ID: "s0", UsageUnitID: "uu0", EscapeRoutes: []string{"er1"}},
			{ID: "s1", UsageUnitID: "uu0", EscapeRoutes: []string{"er2"}},
			{ID: "s2", UsageUnitID: "uu0", EscapeRoutes: []string{"er3"}},
		},
		Staircases: []Staircase{{
			ID:                  "sc1",
			Kind:                "necessary",
			External:            false,
			InSeparateStairwell: false,
			Connects:            []string{"s0", "s1", "s2"},
		}},
	}
	report := validate(trans)
	rule := findRule(t, report, "staircase-located")
	if rule.Status != "violated" {
		t.Errorf("expected violated (3 storeys, no separate stairwell), got %s", rule.Status)
	}
}

func TestStaircaseLocated_RequiresStairwell_HasStairwell(t *testing.T) {
	trans := Translation{
		Properties: BuildingProperties{Height: 10, BuildingClass: "4"},
		UsageUnits: []UsageUnit{{ID: "uu0", TotalGrossFloorArea: 306.6}},
		Storeys: []Storey{
			{ID: "s0", UsageUnitID: "uu0", EscapeRoutes: []string{"er1"}},
			{ID: "s1", UsageUnitID: "uu0", EscapeRoutes: []string{"er2"}},
			{ID: "s2", UsageUnitID: "uu0", EscapeRoutes: []string{"er3"}},
			{ID: "s3", UsageUnitID: "uu0", EscapeRoutes: []string{"er4"}},
		},
		Staircases: []Staircase{{
			ID:                  "sc1",
			Kind:                "necessary",
			External:            false,
			InSeparateStairwell: true,
			SeparateStairwellID: "sw1",
			Connects:            []string{"s0", "s1", "s2", "s3"},
		}},
	}
	report := validate(trans)
	rule := findRule(t, report, "staircase-located")
	if rule.Status != "compliant" {
		t.Errorf("expected compliant (in separate stairwell), got %s", rule.Status)
	}
}

func TestStaircaseLocated_MixedStaircases(t *testing.T) {
	trans := Translation{
		Properties: BuildingProperties{Height: 12, BuildingClass: "4"},
		UsageUnits: []UsageUnit{
			{ID: "uu0", TotalGrossFloorArea: 306.6},
			{ID: "uu1", TotalGrossFloorArea: 306.6},
		},
		Storeys: []Storey{
			{ID: "s0_0", UsageUnitID: "uu0", EscapeRoutes: []string{"er1"}},
			{ID: "s0_1", UsageUnitID: "uu0", EscapeRoutes: []string{"er1"}},
			{ID: "s0_2", UsageUnitID: "uu0", EscapeRoutes: []string{"er1"}},
			{ID: "s0_3", UsageUnitID: "uu0", EscapeRoutes: []string{"er1"}},
			{ID: "s1_0", UsageUnitID: "uu1", EscapeRoutes: []string{"er2"}},
			{ID: "s1_1", UsageUnitID: "uu1", EscapeRoutes: []string{"er2"}},
			{ID: "s1_2", UsageUnitID: "uu1", EscapeRoutes: []string{"er2"}},
			{ID: "s1_3", UsageUnitID: "uu1", EscapeRoutes: []string{"er2"}},
		},
		Staircases: []Staircase{
			{
				ID:                  "necessary_staircase_0",
				Kind:                "necessary",
				External:            false,
				InSeparateStairwell: false,
				Connects:            []string{"s0_0", "s0_1", "s0_2", "s0_3"},
			},
			{
				ID:                  "necessary_staircase_1",
				Kind:                "necessary",
				External:            false,
				InSeparateStairwell: false,
				Connects:            []string{"s1_0", "s1_1", "s1_2", "s1_3"},
			},
			{
				ID:                  "external_staircase_1",
				Kind:                "necessary",
				External:            true,
				InSeparateStairwell: true,
				SeparateStairwellID: "sw1",
				Connects:            []string{"s0_0", "s1_0", "s0_1", "s1_1", "s0_2", "s1_2", "s0_3", "s1_3"},
			},
		},
	}
	report := validate(trans)
	rule := findRule(t, report, "staircase-located")
	if rule.Status != "violated" {
		t.Errorf("expected violated (necessary_staircase_0 and _1 not in separate stairwells), got %s", rule.Status)
	}
}

func TestStaircaseLocated_OnlyNonNecessaryStaircases(t *testing.T) {
	trans := Translation{
		Properties: BuildingProperties{Height: 10, BuildingClass: "3"},
		Staircases: []Staircase{{
			ID:       "sc1",
			Kind:     "convenience",
			External: false,
			Connects: []string{"s0", "s1"},
		}},
	}
	report := validate(trans)
	rule := findRule(t, report, "staircase-located")
	if rule.Status != "not-applicable" {
		t.Errorf("expected not-applicable (no necessary staircases), got %s", rule.Status)
	}
}

// #endregion

// #region 🎵BuildingHeightLimitTests

func TestBuildingHeightLimit_Compliant(t *testing.T) {
	trans := Translation{
		Properties: BuildingProperties{Height: 20.5},
	}
	report := validate(trans)
	rule := findRule(t, report, "building-height-limit")
	if rule.Status != "compliant" {
		t.Errorf("expected compliant, got %s", rule.Status)
	}
	if rule.Clauses[0].Actual != "20.5m" {
		t.Errorf("expected actual 20.5m, got %s", rule.Clauses[0].Actual)
	}
}

func TestBuildingHeightLimit_Violated(t *testing.T) {
	trans := Translation{
		Properties: BuildingProperties{Height: 22.5},
	}
	report := validate(trans)
	rule := findRule(t, report, "building-height-limit")
	if rule.Status != "violated" {
		t.Errorf("expected violated, got %s", rule.Status)
	}
	if rule.Clauses[0].Actual != "22.5m" {
		t.Errorf("expected actual 22.5m, got %s", rule.Clauses[0].Actual)
	}
	if rule.Clauses[0].Should != "<21m" {
		t.Errorf("expected should <21m, got %s", rule.Clauses[0].Should)
	}
}

func TestBuildingHeightLimit_ExactlyAtLimit(t *testing.T) {
	trans := Translation{
		Properties: BuildingProperties{Height: 21.0},
	}
	report := validate(trans)
	rule := findRule(t, report, "building-height-limit")
	if rule.Status != "compliant" {
		t.Errorf("expected compliant at exactly 21m, got %s", rule.Status)
	}
}

func TestBuildingHeightLimit_NoHeightData(t *testing.T) {
	trans := Translation{
		Properties: BuildingProperties{Height: 0},
	}
	report := validate(trans)
	rule := findRule(t, report, "building-height-limit")
	if rule.Status != "not-applicable" {
		t.Errorf("expected not-applicable, got %s", rule.Status)
	}
}

// #endregion

// #region 🏪EndToEndTests

func TestEndToEnd_FullTranslation(t *testing.T) {
	input := `{
		"target_id":"ldrbmrtv.blnbo",
		"design_id":"test-design",
		"properties":{"height":12.0,"building_class":"4","gross_floor_area":613.2},
		"usage_units":[
			{"id":"uu0","total_gross_floor_area":306.6},
			{"id":"uu1","total_gross_floor_area":306.6}
		],
		"storeys":[
			{"id":"s0_0","usage_unit_id":"uu0","escape_routes":["er1"]},
			{"id":"s0_1","usage_unit_id":"uu0","escape_routes":["er1"]},
			{"id":"s1_0","usage_unit_id":"uu1","escape_routes":["er2"]},
			{"id":"s1_1","usage_unit_id":"uu1","escape_routes":["er2"]}
		],
		"staircases":[
			{"id":"sc0","kind":"necessary","external":false,"in_separate_stairwell":true,"separate_stairwell_id":"sw0","connects":["s0_0","s0_1"]},
			{"id":"sc1","kind":"necessary","external":false,"in_separate_stairwell":true,"separate_stairwell_id":"sw1","connects":["s1_0","s1_1"]}
		]
	}`

	var trans Translation
	if err := json.NewDecoder(strings.NewReader(input)).Decode(&trans); err != nil {
		t.Fatal(err)
	}
	report := validate(trans)

	var buf bytes.Buffer
	if err := json.NewEncoder(&buf).Encode(report); err != nil {
		t.Fatal(err)
	}
	var decoded Report
	if err := json.NewDecoder(&buf).Decode(&decoded); err != nil {
		t.Fatal(err)
	}
	if len(decoded.Rules) != 2 {
		t.Fatalf("expected 2 rules, got %d", len(decoded.Rules))
	}

	for _, r := range decoded.Rules {
		if r.Status == "violated" {
			t.Errorf("expected all rules compliant, but %s is %s", r.ID, r.Status)
		}
	}
}

func TestEndToEnd_ViolatedStaircaseAndCompliantHeight(t *testing.T) {
	input := `{
		"target_id":"ldrbmrtv.blnbo",
		"design_id":"test-design",
		"properties":{"height":15.0,"building_class":"4","gross_floor_area":613.2},
		"usage_units":[{"id":"uu0","total_gross_floor_area":306.6}],
		"storeys":[
			{"id":"s0","usage_unit_id":"uu0","escape_routes":["er1"]},
			{"id":"s1","usage_unit_id":"uu0","escape_routes":["er2"]},
			{"id":"s2","usage_unit_id":"uu0","escape_routes":["er3"]},
			{"id":"s3","usage_unit_id":"uu0","escape_routes":["er4"]}
		],
		"staircases":[
			{"id":"sc0","kind":"necessary","external":false,"in_separate_stairwell":false,"connects":["s0","s1","s2","s3"]}
		]
	}`

	var trans Translation
	if err := json.NewDecoder(strings.NewReader(input)).Decode(&trans); err != nil {
		t.Fatal(err)
	}
	report := validate(trans)

	scRule := findRule(t, report, "staircase-located")
	if scRule.Status != "violated" {
		t.Errorf("expected staircase rule violated, got %s", scRule.Status)
	}

	htRule := findRule(t, report, "building-height-limit")
	if htRule.Status != "compliant" {
		t.Errorf("expected height rule compliant, got %s", htRule.Status)
	}
}

// #endregion

// #region 🪩TestHelpers

func findRule(t *testing.T, report Report, ruleID string) *Rule {
	t.Helper()
	for i := range report.Rules {
		if report.Rules[i].ID == ruleID {
			return &report.Rules[i]
		}
	}
	t.Fatalf("rule %q not found in report", ruleID)
	return nil
}

func assertHasClauseWithStatus(t *testing.T, clauses []Clause, status string) {
	t.Helper()
	for _, c := range clauses {
		if c.Status == status {
			return
		}
	}
	t.Errorf("no clause with status %q found", status)
}

// #endregion
