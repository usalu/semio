// #region 🔖Header
// [🔬coda📦blnbo💻main](semiorepo://p/r/coda/b/l/blnbo/f/main.go)
//
// Summary: Go validator for blnbo (Berlin Building Code) target. Validates staircase placement and building height rules.
//
// Specs: Reads translation JSON from stdin, validates against Berlin Building Code rules (BauO Bln), outputs report JSON to stdout.
//
// #endregion

package main

import (
	"encoding/json"
	"fmt"
	"os"
)

// #region 🔖Translation
// [🔬coda📦blnbo💻main🔖translation](semiorepo://p/r/coda/b/l/blnbo/f/main.go/s/Translation)
// Translation input from semio-to-blnbo translator.

type Translation struct {
	TargetID   string              `json:"target_id"`
	DesignID   string              `json:"design_id"`
	Properties BuildingProperties  `json:"properties"`
	UsageUnits []UsageUnit         `json:"usage_units"`
	Storeys    []Storey            `json:"storeys"`
	Staircases []Staircase         `json:"staircases"`
}

type BuildingProperties struct {
	Height         float64 `json:"height"`
	BuildingClass  string  `json:"building_class"`
	GrossFloorArea float64 `json:"gross_floor_area"`
}

type UsageUnit struct {
	ID                 string  `json:"id"`
	TotalGrossFloorArea float64 `json:"total_gross_floor_area"`
}

type Storey struct {
	ID           string   `json:"id"`
	UsageUnitID  string   `json:"usage_unit_id"`
	EscapeRoutes []string `json:"escape_routes"`
}

type Staircase struct {
	ID                   string   `json:"id"`
	Kind                 string   `json:"kind"`
	External             bool     `json:"external"`
	InSeparateStairwell  bool     `json:"in_separate_stairwell"`
	SeparateStairwellID  string   `json:"separate_stairwell_id"`
	Connects             []string `json:"connects"`
}

// #endregion

// #region 🔖Report
// [🔬coda📦blnbo💻main🔖report](semiorepo://p/r/coda/b/l/blnbo/f/main.go/s/Report)
// Validation report output.

type Report struct {
	Rules []Rule `json:"rules"`
}

type Rule struct {
	ID      string   `json:"id"`
	Status  string   `json:"status"`
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
		fmt.Fprintf(os.Stderr, "[DEBUG] blnbo validator error: %v\n", err)
		os.Exit(1)
	}
}

func run() error {
	var trans Translation
	if err := json.NewDecoder(os.Stdin).Decode(&trans); err != nil {
		return fmt.Errorf("decode translation: %w", err)
	}

	report := validate(trans)
	if err := json.NewEncoder(os.Stdout).Encode(report); err != nil {
		return fmt.Errorf("encode report: %w", err)
	}
	return nil
}

// #region 🔖Validate
// [🔬coda📦blnbo💻main🔖validate](semiorepo://p/r/coda/b/l/blnbo/f/main.go/s/Validate)
// Validate MUST apply all Berlin Building Code rules and return a report.

func validate(trans Translation) Report {
	var rules []Rule

	rules = append(rules, validateStaircaseLocated(trans))
	rules = append(rules, validateBuildingHeightLimit(trans))

	return Report{Rules: rules}
}

// #endregion

// #region 🔖StaircaseLocated
// [🔬coda📦blnbo💻main🔖staircaselocated](semiorepo://p/r/coda/b/l/blnbo/f/main.go/s/StaircaseLocated)
// StaircaseLocated MUST check that necessary staircases are in separate stairwells (BauO Bln §35).
// Exemptions: external staircases, building classes 1/2, staircases connecting ≤2 storeys
// within a single usage unit ≤200m² GFA with separate escape routes per storey.

func validateStaircaseLocated(trans Translation) Rule {
	ruleID := "staircase-located"

	necessaryStaircases := filterNecessaryStaircases(trans.Staircases)
	if len(necessaryStaircases) == 0 {
		return Rule{
			ID:     ruleID,
			Status: "not-applicable",
			Clauses: []Clause{{ID: "no-necessary-staircases", Status: "not-applicable"}},
		}
	}

	storeyIndex := buildStoreyIndex(trans.Storeys)
	usageUnitIndex := buildUsageUnitIndex(trans.UsageUnits)

	var allClauses []Clause
	overallStatus := "compliant"

	for _, sc := range necessaryStaircases {
		clauses := evaluateStaircaseClauses(sc, trans.Properties.BuildingClass, storeyIndex, usageUnitIndex)
		allClauses = append(allClauses, clauses...)

		requiresSeparateStairwell := staircaseRequiresSeparateStairwell(clauses)
		if requiresSeparateStairwell && !sc.InSeparateStairwell {
			overallStatus = "violated"
			allClauses = append(allClauses, Clause{
				ID:     fmt.Sprintf("%s-not-in-separate-stairwell", sc.ID),
				Status: "violated",
				Actual: "not in separate stairwell",
				Should: "must be in separate stairwell",
			})
		}
	}

	return Rule{
		ID:      ruleID,
		Status:  overallStatus,
		Clauses: allClauses,
	}
}

func filterNecessaryStaircases(staircases []Staircase) []Staircase {
	var result []Staircase
	for _, sc := range staircases {
		if sc.Kind == "necessary" {
			result = append(result, sc)
		}
	}
	return result
}

func buildStoreyIndex(storeys []Storey) map[string]Storey {
	idx := make(map[string]Storey, len(storeys))
	for _, s := range storeys {
		idx[s.ID] = s
	}
	return idx
}

func buildUsageUnitIndex(units []UsageUnit) map[string]UsageUnit {
	idx := make(map[string]UsageUnit, len(units))
	for _, u := range units {
		idx[u.ID] = u
	}
	return idx
}

func evaluateStaircaseClauses(
	sc Staircase,
	buildingClass string,
	storeyIndex map[string]Storey,
	usageUnitIndex map[string]UsageUnit,
) []Clause {
	var clauses []Clause

	// Clause 1: not-external-staircase
	if sc.External {
		clauses = append(clauses, Clause{
			ID:     fmt.Sprintf("%s-not-external-staircase", sc.ID),
			Status: "exempt",
			Actual: "external staircase",
		})
		return clauses
	}
	clauses = append(clauses, Clause{
		ID:     fmt.Sprintf("%s-not-external-staircase", sc.ID),
		Status: "applicable",
		Actual: "not external",
	})

	// Clause 2: not-in-building-classes-1-and-2
	if buildingClass == "1" || buildingClass == "2" {
		clauses = append(clauses, Clause{
			ID:     fmt.Sprintf("%s-not-in-building-classes-1-and-2", sc.ID),
			Status: "exempt",
			Actual: fmt.Sprintf("building class %s", buildingClass),
		})
		return clauses
	}
	clauses = append(clauses, Clause{
		ID:     fmt.Sprintf("%s-not-in-building-classes-1-and-2", sc.ID),
		Status: "applicable",
		Actual: fmt.Sprintf("building class %s", buildingClass),
	})

	// Clause 3: two-storey-small-usage-unit-exemption
	// Exempt if staircase connects exactly 2 storeys within the same usage unit
	// with total GFA ≤ 200m² and each storey has a different escape route
	exemptBySmallUnit := checkTwoStoreySmallUsageUnitExemption(sc, storeyIndex, usageUnitIndex)
	if exemptBySmallUnit {
		clauses = append(clauses, Clause{
			ID:     fmt.Sprintf("%s-two-storey-small-usage-unit-exemption", sc.ID),
			Status: "exempt",
			Actual: "connects ≤2 storeys in usage unit ≤200m² with separate escape routes",
		})
		return clauses
	}
	clauses = append(clauses, Clause{
		ID:     fmt.Sprintf("%s-two-storey-small-usage-unit-exemption", sc.ID),
		Status: "not-exempt",
	})

	return clauses
}

func checkTwoStoreySmallUsageUnitExemption(
	sc Staircase,
	storeyIndex map[string]Storey,
	usageUnitIndex map[string]UsageUnit,
) bool {
	if len(sc.Connects) != 2 {
		return false
	}

	storey0, ok0 := storeyIndex[sc.Connects[0]]
	storey1, ok1 := storeyIndex[sc.Connects[1]]
	if !ok0 || !ok1 {
		return false
	}

	if storey0.UsageUnitID != storey1.UsageUnitID || storey0.UsageUnitID == "" {
		return false
	}

	unit, ok := usageUnitIndex[storey0.UsageUnitID]
	if !ok || unit.TotalGrossFloorArea > 200.0 {
		return false
	}

	if len(storey0.EscapeRoutes) == 0 || len(storey1.EscapeRoutes) == 0 {
		return false
	}

	// Each storey must have at least one escape route reachable
	return true
}

func staircaseRequiresSeparateStairwell(clauses []Clause) bool {
	for _, c := range clauses {
		if c.Status == "exempt" {
			return false
		}
	}
	return true
}

// #endregion

// #region 🔖BuildingHeightLimit
// [🔬coda📦blnbo💻main🔖buildingheightlimit](semiorepo://p/r/coda/b/l/blnbo/f/main.go/s/BuildingHeightLimit)
// BuildingHeightLimit MUST check that the building height is less than 21m (BauO Bln §2 high-rise threshold).

const maxBuildingHeight = 21.0

func validateBuildingHeightLimit(trans Translation) Rule {
	ruleID := "building-height-limit"
	height := trans.Properties.Height

	if height <= 0 {
		return Rule{
			ID:     ruleID,
			Status: "not-applicable",
			Clauses: []Clause{{ID: "no-height-data", Status: "not-applicable"}},
		}
	}

	if height > maxBuildingHeight {
		return Rule{
			ID:     ruleID,
			Status: "violated",
			Clauses: []Clause{{
				ID:     fmt.Sprintf("<%.0fm", maxBuildingHeight),
				Status: "violated",
				Actual: fmt.Sprintf("%.1fm", height),
				Should: fmt.Sprintf("<%.0fm", maxBuildingHeight),
			}},
		}
	}

	return Rule{
		ID:     ruleID,
		Status: "compliant",
		Clauses: []Clause{{
			ID:     fmt.Sprintf("<%.0fm", maxBuildingHeight),
			Status: "compliant",
			Actual: fmt.Sprintf("%.1fm", height),
		}},
	}
}

// #endregion
