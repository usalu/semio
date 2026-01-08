// #region Header

// go/cli/main_test.go

// 2025 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// #endregion Header

package main

import (
	"bytes"
	"context"
	"encoding/json"
	"strings"
	"testing"

	"github.com/usalu/semio/go/repo"
)

// #region Helpers

func executeCommand(args ...string) (string, error) {
	buf := new(bytes.Buffer)
	rootCmd.SetOut(buf)
	rootCmd.SetErr(buf)
	rootCmd.SetArgs(args)
	err := rootCmd.Execute()
	return buf.String(), err
}

func parseJSONOutput(output string) (map[string]interface{}, error) {
	var result map[string]interface{}
	err := json.Unmarshal([]byte(output), &result)
	return result, err
}

func hasExitCode(output string, code int) bool {
	parsed, err := parseJSONOutput(output)
	if err != nil {
		return false
	}
	outputMap, ok := parsed["output"].(map[string]interface{})
	if !ok {
		return false
	}
	exitCode, ok := outputMap["exitCode"].(float64)
	if !ok {
		return false
	}
	return int(exitCode) == code
}

// #endregion Helpers

// #region Codebase Tests

func TestCodebaseCommand(t *testing.T) {
	result := repo.ToolCodebase()
	if result.Error != "" {
		t.Errorf("ToolCodebase returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolCodebase returned nil data")
	}
}

// #endregion Codebase Tests

// #region Analyze Tests

func TestAnalyzeCommand(t *testing.T) {
	result := repo.ToolAnalyze("@semio/js", nil)
	if result.Error != "" {
		t.Errorf("ToolAnalyze returned error: %s", result.Error)
	}
}

func TestAnalyzeFile(t *testing.T) {
	result := repo.ToolAnalyze("js/semio/semio.ts", nil)
	if result.Error != "" {
		t.Errorf("ToolAnalyze file returned error: %s", result.Error)
	}
}

// #endregion Analyze Tests

// #region Fix Tests

func TestFixCommand(t *testing.T) {
	result := repo.ToolFix("@semio/js")
	if result.Error != "" {
		t.Errorf("ToolFix returned error: %s", result.Error)
	}
}

// #endregion Fix Tests

// #region Policy Tests

func TestPolicyListCommand(t *testing.T) {
	result := repo.ToolPolicyList()
	if result.Error != "" {
		t.Errorf("ToolPolicyList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolPolicyList returned nil data")
	}
	policies, ok := result.Data.([]repo.PolicyDef)
	if !ok {
		t.Error("ToolPolicyList data is not []PolicyDef")
		return
	}
	if len(policies) == 0 {
		t.Error("ToolPolicyList returned no policies")
	}
	foundCode := false
	for _, p := range policies {
		if p.ID == "code" {
			foundCode = true
			break
		}
	}
	if !foundCode {
		t.Error("Expected to find 'code' policy")
	}
}

func TestPolicyCheckCommand(t *testing.T) {
	result := repo.ToolPolicyCheck("code", "@semio/js")
	if result.Error != "" {
		t.Errorf("ToolPolicyCheck returned error: %s", result.Error)
	}
}

func TestPolicyViolationListCommand(t *testing.T) {
	result := repo.ToolPolicyViolationList("code")
	if result.Error != "" {
		t.Errorf("ToolPolicyViolationList returned error: %s", result.Error)
	}
}

// #endregion Policy Tests

// #region Bundle Tests

func TestBundleListCommand(t *testing.T) {
	result := repo.ToolProjectList()
	if result.Error != "" {
		t.Errorf("ToolProjectList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolProjectList returned nil data")
	}
	bundles, ok := result.Data.([]repo.Bundle)
	if !ok {
		t.Error("ToolProjectList data is not []Bundle")
		return
	}
	if len(bundles) == 0 {
		t.Error("ToolProjectList returned no bundles")
	}
	foundJS := false
	for _, b := range bundles {
		if b.Name == "@semio/js" {
			foundJS = true
			break
		}
	}
	if !foundJS {
		t.Error("Expected to find '@semio/js' bundle")
	}
}

func TestBundleTreeCommand(t *testing.T) {
	result := repo.ToolProjectTree()
	if result.Error != "" {
		t.Errorf("ToolProjectTree returned error: %s", result.Error)
	}
}

// #endregion Bundle Tests

// #region Folder Tests

func TestFolderListCommand(t *testing.T) {
	result := repo.ToolFolderList("go")
	if result.Error != "" {
		t.Errorf("ToolFolderList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolFolderList returned nil data")
	}
}

func TestFolderTreeCommand(t *testing.T) {
	result := repo.ToolFolderTree("go")
	if result.Error != "" {
		t.Errorf("ToolFolderTree returned error: %s", result.Error)
	}
}

func TestFolderCreateMoveDelete(t *testing.T) {
	testFolder := "temp/test-folder-cli"
	createResult := repo.ToolFolderCreate(testFolder)
	if createResult.Error != "" {
		t.Errorf("ToolFolderCreate returned error: %s", createResult.Error)
	}
	moveResult := repo.ToolFolderMove(testFolder, testFolder+"-moved")
	if moveResult.Error != "" {
		t.Errorf("ToolFolderMove returned error: %s", moveResult.Error)
	}
	deleteResult := repo.ToolFolderDelete(testFolder + "-moved")
	if deleteResult.Error != "" {
		t.Errorf("ToolFolderDelete returned error: %s", deleteResult.Error)
	}
}

// #endregion Folder Tests

// #region File Tests

func TestFileListCommand(t *testing.T) {
	result := repo.ToolFileList("@semio/js")
	if result.Error != "" {
		t.Errorf("ToolFileList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolFileList returned nil data")
	}
}

func TestFileTreeCommand(t *testing.T) {
	result := repo.ToolFileTree("go")
	if result.Error != "" {
		t.Errorf("ToolFileTree returned error: %s", result.Error)
	}
}

func TestFileCreateMoveDelete(t *testing.T) {
	testFile := "temp/test-file-cli.txt"
	createResult := repo.ToolFileCreate(testFile)
	if createResult.Error != "" {
		t.Errorf("ToolFileCreate returned error: %s", createResult.Error)
	}
	moveResult := repo.ToolFileMove(testFile, "temp/test-file-cli-moved.txt")
	if moveResult.Error != "" {
		t.Errorf("ToolFileMove returned error: %s", moveResult.Error)
	}
	deleteResult := repo.ToolFileDelete("temp/test-file-cli-moved.txt")
	if deleteResult.Error != "" {
		t.Errorf("ToolFileDelete returned error: %s", deleteResult.Error)
	}
}

// #endregion File Tests

// #region Section Tests

func TestSectionListCommand(t *testing.T) {
	result := repo.ToolSectionList("js/semio/semio.ts")
	if result.Error != "" {
		t.Errorf("ToolSectionList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolSectionList returned nil data")
	}
	sections, ok := result.Data.([]repo.Section)
	if !ok {
		t.Error("ToolSectionList data is not []SectionInfo")
		return
	}
	if len(sections) == 0 {
		t.Error("ToolSectionList returned no sections")
	}
	foundHeader := false
	for _, s := range sections {
		if s.Name == "Header" {
			foundHeader = true
			break
		}
	}
	if !foundHeader {
		t.Error("Expected to find 'Header' section in js/semio/semio.ts")
	}
}

func TestSectionTreeCommand(t *testing.T) {
	result := repo.ToolSectionTree("js/semio/semio.ts")
	if result.Error != "" {
		t.Errorf("ToolSectionTree returned error: %s", result.Error)
	}
}

// #endregion Section Tests

// #region Definition Tests

func TestDefinitionListCommand(t *testing.T) {
	result := repo.ToolDefinitionList("js/semio/semio.ts")
	if result.Error != "" {
		t.Errorf("ToolDefinitionList returned error: %s", result.Error)
	}
}

// #endregion Definition Tests

// #region Ticket Tests

func TestTicketListCommand(t *testing.T) {
	year := 2025
	result := repo.ToolTicketList(&year, nil, nil)
	if result.Error != "" {
		t.Errorf("ToolTicketList returned error: %s", result.Error)
	}
}

// #endregion Ticket Tests

// #region Contributor Tests

func TestContributorListCommand(t *testing.T) {
	result := repo.ToolContributorList()
	if result.Error != "" {
		t.Errorf("ToolContributorList returned error: %s", result.Error)
	}
	if result.Data == nil {
		t.Error("ToolContributorList returned nil data")
	}
}

// #endregion Contributor Tests

// #region GraphQL Tests

func TestGraphQLRepoQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { id name } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL returned error: %v", err)
	}
	if !strings.Contains(result, "semio") {
		t.Errorf("Expected result to contain 'semio', got: %s", result)
	}
}

func TestGraphQLBundlesQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { bundles { id name root } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL bundles returned error: %v", err)
	}
	if !strings.Contains(result, "@semio/js") {
		t.Errorf("Expected result to contain '@semio/js', got: %s", result)
	}
}

func TestGraphQLPoliciesQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { policies { id name } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL policies returned error: %v", err)
	}
	if !strings.Contains(result, "code") {
		t.Errorf("Expected result to contain 'code', got: %s", result)
	}
}

func TestGraphQLTicketsQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { tickets(year: 2025) { id slug status } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL tickets returned error: %v", err)
	}
	if !strings.Contains(result, "ticket:") {
		t.Errorf("Expected result to contain 'ticket:', got: %s", result)
	}
}

func TestGraphQLAnalyzeQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ analyze { metrics { total } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL analyze returned error: %v", err)
	}
	if !strings.Contains(result, "total") {
		t.Errorf("Expected result to contain 'total', got: %s", result)
	}
}

func TestGraphQLContributorsQuery(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `{ repo { contributors { id github } } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL contributors returned error: %v", err)
	}
	if result == "" {
		t.Error("ExecuteGraphQL contributors returned empty result")
	}
}

func TestGraphQLFixMutation(t *testing.T) {
	result, err := executor.ExecuteJSON(context.Background(), `mutation { fix { fixed remaining } }`, nil)
	if err != nil {
		t.Errorf("ExecuteGraphQL fix mutation returned error: %v", err)
	}
	if !strings.Contains(result, "fixed") {
		t.Errorf("Expected result to contain 'fixed', got: %s", result)
	}
}

// #endregion GraphQL Tests
