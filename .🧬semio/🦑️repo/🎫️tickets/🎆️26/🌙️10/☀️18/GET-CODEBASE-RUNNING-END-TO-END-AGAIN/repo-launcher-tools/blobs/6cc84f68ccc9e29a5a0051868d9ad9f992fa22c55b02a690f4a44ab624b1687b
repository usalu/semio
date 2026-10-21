// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🧭️route-contract proves the process route against the shared native entrypoint vectors and the script-argument fixture.

// #endregion 🧲️Header

package launcher

import (
	"reflect"
	"sort"
	"strings"
	"testing"
)

// 🧭️The route and its arguments agree with the vectors that every implementation shares.
func TestRouteVectors(t *testing.T) {
	var fixture struct {
		BuildVerbs         []string
		DashboardArguments []string
		RootScriptPrefixes [][]string
		Vectors            []struct {
			ID        string
			Argv      []string
			Kind      string
			Arguments []string
		}
	}
	readFixture(t, "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/🧫️fixtures/🚪️entrypoint/🔣️.json", &fixture)
	verbs, err := BuildVerbsAt(testRoot(t), loadRegistry(t))
	if err != nil {
		t.Fatal(err)
	}
	sorted := func(values []string) []string {
		copied := append([]string{}, values...)
		sort.Strings(copied)
		return copied
	}
	if !reflect.DeepEqual(sorted(fixture.BuildVerbs), sorted(verbs)) || !reflect.DeepEqual(fixture.DashboardArguments, DashboardArguments) {
		t.Fatalf("the shared fixture and the operations catalogue declare different verbs: %q vs %q", fixture.BuildVerbs, verbs)
	}
	for _, row := range fixture.Vectors {
		plan := PlanRouteWith(verbs, row.Argv)
		if plan.Kind != row.Kind || strings.Join(plan.Arguments, "\x00") != strings.Join(row.Arguments, "\x00") {
			t.Fatalf("%s: got %+v want %s %q", row.ID, plan, row.Kind, row.Arguments)
		}
	}
}

// 🧭️Planning never mutates the caller's arguments.
func TestRouteLeavesArgumentsAlone(t *testing.T) {
	arguments := []string{"cargo", "--store=shared", "--", "check"}
	plan := PlanRoute(arguments)
	plan.Arguments[0] = "other"
	if arguments[0] != "cargo" {
		t.Fatal("planning mutated the caller arguments")
	}
}

// 🧭️Script options map to runtime flags and the script is split from its arguments.
func TestScriptArgumentsFixture(t *testing.T) {
	var fixture struct {
		ScriptArguments []struct {
			Name   string
			Args   []string
			Flags  []string
			Script *string
			Rest   []string
		}
		Launch []struct {
			Name       string
			Executable string
			Before     []string
			Flags      []string
			Script     string
			Prefix     []string
			Args       []string
			Argv       []string
		}
	}
	readFixture(t, launcherFixture, &fixture)
	bun := loadRegistry(t).Runtimes["bun"]
	for _, row := range fixture.ScriptArguments {
		flags, script, rest, ok := ScriptArguments(bun, row.Args)
		if row.Script == nil {
			if ok {
				t.Fatalf("%s: not a script", row.Name)
			}
			continue
		}
		if !ok || script != *row.Script || strings.Join(flags, "\x00") != strings.Join(row.Flags, "\x00") || strings.Join(rest, "\x00") != strings.Join(row.Rest, "\x00") {
			t.Fatalf("%s: got %q %q %q", row.Name, flags, script, rest)
		}
	}
	for _, row := range fixture.Launch {
		var rt Runtime
		rt.Launch.Before = row.Before
		got := LaunchArguments(row.Executable, rt, row.Flags, row.Script, row.Prefix, row.Args)
		if strings.Join(got, "\x00") != strings.Join(row.Argv, "\x00") {
			t.Fatalf("%s: %q", row.Name, got)
		}
	}
}

// #endregion 🧭️Route
