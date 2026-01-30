package main

import (
	"encoding/json"
	"strings"
	"testing"
)

func TestFormatResult_Additional(t *testing.T) {
	// 1. Goal
	t.Run("Goal", func(t *testing.T) {
		payload := map[string]interface{}{
			"goal": map[string]interface{}{
				"id":          "SKETCHPAD/MVP",
				"title":       "Sketchpad MVP",
				"status":      "open",
				"description": "Get sketchpad running at MVP level",
				"dates": map[string]interface{}{
					"due": "2026-02-15",
				},
			},
		}
		jsonBytes, _ := json.Marshal(payload)
		var p map[string]interface{}
		json.Unmarshal(jsonBytes, &p)

		output := formatResult("goal list", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"goal\":") {
			t.Errorf("expected formatted goal, got: %s", output)
		}
		if !strings.Contains(output, "Sketchpad MVP") {
			t.Error("output missing title")
		}
		if !strings.Contains(output, "2026-02-15") {
			t.Error("output missing due date")
		}
		if !strings.Contains(output, "SKETCHPAD/MVP") {
			t.Error("output missing id/slug")
		}
	})

	// 2. Contributor
	t.Run("Contributor", func(t *testing.T) {
		payload := map[string]interface{}{
			"contributor": map[string]interface{}{
				"github": "octocat",
				"name":   "The Octocat",
				"contributions": map[string]interface{}{
					"commits": 10,
				},
			},
		}
		jsonBytes, _ := json.Marshal(payload)

		output := formatResult("contributor list", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"contributor\":") {
			t.Errorf("expected formatted contributor, got: %s", output)
		}
		if !strings.Contains(output, "octocat") {
			t.Error("output missing github handle")
		}
		if !strings.Contains(output, "The Octocat") {
			t.Error("output missing name")
		}
	})

	// 3. Policy
	t.Run("Policy", func(t *testing.T) {
		payload := map[string]interface{}{
			"policy": map[string]interface{}{
				"id":          "code",
				"description": "Validates source file headers",
				"kinds":       []interface{}{"code:header"},
			},
		}
		jsonBytes, _ := json.Marshal(payload)

		output := formatResult("policy list", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"policy\":") {
			t.Errorf("expected formatted policy, got: %s", output)
		}
		if !strings.Contains(output, "code") {
			t.Error("output missing id")
		}
		if !strings.Contains(output, "Validates") {
			t.Error("output missing description")
		}
	})
	
	// 4. ViolationKind (from policy list sometimes?)
	// Actually policy list returns "policy" objects based on user input.

	// 5. Check File List formatting again (Case 12)
	t.Run("File", func(t *testing.T) {
		payload := map[string]interface{}{
			"file": map[string]interface{}{
				"id":        "path/to/file.md",
				"extension": ".md",
			},
		}
		jsonBytes, _ := json.Marshal(payload)

		output := formatResult("file list", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"file\":") {
			t.Errorf("expected formatted file, got: %s", output)
		}
		if !strings.Contains(output, "path/to/file.md") {
			t.Error("output missing path")
		}
	})

	// 6. Fix
	t.Run("Fix", func(t *testing.T) {
		payload := map[string]interface{}{
			"fix": map[string]interface{}{
				"fixed":     5,
				"remaining": 2,
			},
		}
		jsonBytes, _ := json.Marshal(payload)

		output := formatResult("fix", jsonBytes, true)
		if strings.TrimSpace(output) == "" || strings.Contains(output, "\"fix\":") {
			t.Errorf("expected formatted fix, got: %s", output)
		}
		if !strings.Contains(output, "fixed 5 violations") {
			t.Error("output missing fixed count")
		}
	})
}
