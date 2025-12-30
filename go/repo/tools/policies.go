// repo/tools/policies.go

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

package tools

import (
	"fmt"
	"math/rand"
	"path/filepath"
	"regexp"
	"strings"
	"time"
)

type PolicyFunc func(ctx *PolicyContext) []Violation

type RegisteredPolicy struct {
	Meta PolicyMeta
	Run  PolicyFunc
}

var registeredPolicies []RegisteredPolicy

func RegisterPolicy(meta PolicyMeta, run PolicyFunc) {
	registeredPolicies = append(registeredPolicies, RegisteredPolicy{Meta: meta, Run: run})
}

func GetRegisteredPolicies() []PolicyMeta {
	var metas []PolicyMeta
	for _, p := range registeredPolicies {
		metas = append(metas, p.Meta)
	}
	return metas
}

type PolicyContext struct {
	Scope    Scope
	RootDir  string
	Projects []NxProject
	fileCache     map[string]string
	sectionCache  map[string][]SectionInfo
}

func NewPolicyContext(scope Scope, projects []NxProject) *PolicyContext {
	return &PolicyContext{
		Scope:        scope,
		RootDir:      rootDir,
		Projects:     projects,
		fileCache:    make(map[string]string),
		sectionCache: make(map[string][]SectionInfo),
	}
}

func (ctx *PolicyContext) Files(pattern string) ([]string, error) {
	if pattern != "" {
		return SimpleGlob(pattern, rootDir, []string{"**/node_modules/**", "**/.venv/**"}, true)
	}
	return ScopeToFiles(ctx.Scope, ctx.Projects)
}

func (ctx *PolicyContext) ReadText(filePath string) string {
	absPath := filepath.Join(rootDir, filePath)
	if content, ok := ctx.fileCache[absPath]; ok {
		return content
	}
	content, err := ReadTextFile(absPath)
	if err != nil {
		ctx.fileCache[absPath] = ""
		return ""
	}
	ctx.fileCache[absPath] = content
	return content
}

func (ctx *PolicyContext) Sections(filePath string) []SectionInfo {
	if sections, ok := ctx.sectionCache[filePath]; ok {
		return sections
	}
	content := ctx.ReadText(filePath)
	sections := ParseSections(content, filePath)
	ctx.sectionCache[filePath] = sections
	return sections
}

func (ctx *PolicyContext) CreateViolation(summary, kind, solution, reason, scope string, line int, excerpt string, autofix *Fix) Violation {
	priority := PriorityMedium
	for _, p := range registeredPolicies {
		if kind == p.Meta.ID || strings.HasPrefix(kind, p.Meta.ID+":") {
			priority = p.Meta.Priority
			break
		}
	}
	return Violation{
		ID:          fmt.Sprintf("%s-%d-%s", kind, time.Now().UnixNano(), randomString(6)),
		Summary:     summary,
		Kind:        kind,
		Priority:    priority,
		Autofixable: autofix != nil,
		Solution:    solution,
		Reason:      reason,
		Scope:       scope,
		Line:        line,
		Excerpt:     excerpt,
		Autofix:     autofix,
	}
}

func randomString(n int) string {
	const letters = "abcdefghijklmnopqrstuvwxyz0123456789"
	b := make([]byte, n)
	for i := range b {
		b[i] = letters[rand.Intn(len(letters))]
	}
	return string(b)
}

func RunPolicies(scope Scope, projects []NxProject, policyIDs []string) ([]Violation, error) {
	ctx := NewPolicyContext(scope, projects)
	var violations []Violation
	var policiesToRun []RegisteredPolicy
	if len(policyIDs) > 0 {
		for _, p := range registeredPolicies {
			for _, id := range policyIDs {
				if p.Meta.ID == id {
					policiesToRun = append(policiesToRun, p)
					break
				}
			}
		}
	} else {
		for _, p := range registeredPolicies {
			if matchesScope(p.Meta.Scopes, scope) {
				policiesToRun = append(policiesToRun, p)
			}
		}
	}
	for _, policy := range policiesToRun {
		policyViolations := policy.Run(ctx)
		violations = append(violations, policyViolations...)
	}
	return violations, nil
}

func matchesScope(policyScopes []string, targetScope Scope) bool {
	for _, pattern := range policyScopes {
		if pattern == "*" || pattern == "**/*" {
			return true
		}
		if strings.HasPrefix(pattern, "@semio") {
			if targetScope.Kind == ScopeRepo || (targetScope.Kind == ScopeProject && strings.HasPrefix(targetScope.ProjectName, pattern)) {
				return true
			}
		}
		if targetScope.Kind == ScopeRepo && strings.HasPrefix(pattern, "**/*.") {
			return true
		}
		if targetScope.FilePath != "" {
			normalizedTarget := NormalizePath(targetScope.FilePath)
			normalizedPattern := NormalizePath(pattern)
			if strings.Contains(normalizedTarget, normalizedPattern) {
				return true
			}
		}
	}
	return false
}

func init() {
	RegisterPolicy(PolicyMeta{
		ID:          "header",
		Name:        "Header",
		Description: "Validates source file header section with filename, contributors, and license",
		Scopes:      []string{"**/*.{ts,tsx,py,cs}"},
		Priority:    PriorityLow,
	}, headerPolicy)
	RegisterPolicy(PolicyMeta{
		ID:          "section",
		Name:        "Section",
		Description: "Validates section blocks for proper naming and content",
		Scopes:      []string{"**/*.{ts,tsx,py,cs}"},
		Priority:    PriorityLow,
	}, sectionPolicy)
	RegisterPolicy(PolicyMeta{
		ID:          "comment",
		Name:        "Comment",
		Description: "Detects forbidden comments (inline, block, JSDoc) - documentation belongs in README.md and AGENTS.md",
		Scopes:      []string{"**/*.{ts,tsx}"},
		Priority:    PriorityLow,
	}, commentPolicy)
}

func headerPolicy(ctx *PolicyContext) []Violation {
	var violations []Violation
	files, err := ctx.Files("")
	if err != nil {
		return violations
	}
	agplMarkers := []string{"GNU Affero General Public License", "AGPL", "https://www.gnu.org/licenses/"}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		lang := GetLanguageFromPath(file)
		if lang == "" || lang == "markdown" {
			continue
		}
		sections := ctx.Sections(file)
		var headerSection *SectionInfo
		for i := range sections {
			if strings.ToLower(sections[i].Name) == "header" {
				headerSection = &sections[i]
				break
			}
		}
		if headerSection == nil {
			violations = append(violations, ctx.CreateViolation(
				fmt.Sprintf("Missing header section in %s", file),
				"header:missing-section",
				"Add a #region Header with filename, contributors, and appropriate license",
				"Every source file must include a header section",
				file, 0, "", nil))
			continue
		}
		headerContent := content[headerSection.StartIndex:headerSection.EndIndex]
		headerLines := strings.Split(headerContent, "\n")
		filename := filepath.Base(file)
		hasFilename := false
		for _, line := range headerLines {
			if strings.Contains(line, filename) {
				hasFilename = true
				break
			}
		}
		if !hasFilename {
			violations = append(violations, ctx.CreateViolation(
				fmt.Sprintf("Missing filename in header of %s", file),
				"header:missing-filename",
				fmt.Sprintf("Add the filename \"%s\" to the header section", filename),
				"Header must include the source file name",
				fmt.Sprintf("%s#Header", file), headerSection.StartLine, "", nil))
		}
		contributorPattern := regexp.MustCompile(`\d{4}\s+[\w\s]+<[\w.@-]+>`)
		hasContributors := false
		for _, line := range headerLines {
			if contributorPattern.MatchString(line) {
				hasContributors = true
				break
			}
		}
		if !hasContributors {
			violations = append(violations, ctx.CreateViolation(
				fmt.Sprintf("Missing contributors in header of %s", file),
				"header:missing-contributors",
				"Add contributor line in format: YEAR Name <email>",
				"Header must include at least one contributor",
				fmt.Sprintf("%s#Header", file), headerSection.StartLine, "", nil))
		}
		hasLicense := false
		for _, marker := range agplMarkers {
			if strings.Contains(headerContent, marker) {
				hasLicense = true
				break
			}
		}
		if !hasLicense {
			violations = append(violations, ctx.CreateViolation(
				fmt.Sprintf("Missing license in header of %s", file),
				"header:missing-license",
				"Add AGPL-3.0 license text to the header section",
				"Header must include AGPL-3.0 license",
				fmt.Sprintf("%s#Header", file), headerSection.StartLine, "", nil))
		} else {
			wrongLicenses := []string{"MIT", "Apache", "BSD"}
			hasWrongLicense := false
			for _, wrong := range wrongLicenses {
				if strings.Contains(headerContent, wrong) {
					hasWrongLicense = true
					break
				}
			}
			if strings.Contains(headerContent, "GPL") && !strings.Contains(headerContent, "AGPL") {
				hasWrongLicense = true
			}
			if hasWrongLicense {
				violations = append(violations, ctx.CreateViolation(
					fmt.Sprintf("Wrong license in header of %s", file),
					"header:wrong-license",
					"Replace with AGPL-3.0 license text",
					"Project uses AGPL-3.0, not other licenses",
					fmt.Sprintf("%s#Header", file), headerSection.StartLine, "", nil))
			}
		}
	}
	return violations
}

func sectionPolicy(ctx *PolicyContext) []Violation {
	var violations []Violation
	files, err := ctx.Files("")
	if err != nil {
		return violations
	}
	sectionPatterns := map[string]struct{ start, end *regexp.Regexp }{
		"typescript": {
			start: regexp.MustCompile(`(?i)^\s*//\s*#region(?:\s+(\S.*?))?\s*$`),
			end:   regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
		"python": {
			start: regexp.MustCompile(`(?i)^\s*#\s*region(?:\s+(\S.*?))?\s*$`),
			end:   regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(\S.*?))?\s*$`),
		},
		"csharp": {
			start: regexp.MustCompile(`(?i)^\s*#region(?:\s+(\S.*?))?\s*$`),
			end:   regexp.MustCompile(`(?i)^\s*#endregion(?:\s+(\S.*?))?\s*$`),
		},
	}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		lang := GetLanguageFromPath(file)
		if lang == "" || lang == "markdown" {
			continue
		}
		pat, ok := sectionPatterns[lang]
		if !ok {
			continue
		}
		lines := strings.Split(content, "\n")
		type stackItem struct {
			name string
			line int
		}
		var stack []stackItem
		for i, line := range lines {
			lineNum := i + 1
			line = strings.TrimSuffix(line, "\r")
			if match := pat.start.FindStringSubmatch(line); match != nil {
				name := ""
				if len(match) > 1 {
					name = strings.TrimSpace(match[1])
				}
				if name == "" {
					violations = append(violations, ctx.CreateViolation(
						fmt.Sprintf("Missing section name at %s:%d", file, lineNum),
						"section:missing-start-name",
						"Add a name after #region",
						"Section blocks should have descriptive names",
						file, lineNum, strings.TrimSpace(line), nil))
				}
				stack = append(stack, stackItem{name: name, line: lineNum})
				continue
			}
			if match := pat.end.FindStringSubmatch(line); match != nil {
				endName := ""
				if len(match) > 1 {
					endName = strings.TrimSpace(match[1])
				}
				if len(stack) > 0 {
					open := stack[len(stack)-1]
					stack = stack[:len(stack)-1]
					if open.name != "" {
						if endName == "" {
							violations = append(violations, ctx.CreateViolation(
								fmt.Sprintf("Missing end section name at %s:%d", file, lineNum),
								"section:missing-end-name",
								fmt.Sprintf("Add the section name \"%s\" after #endregion", open.name),
								"End section should match start section name for clarity",
								file, lineNum, strings.TrimSpace(line), nil))
						} else if endName != open.name {
							violations = append(violations, ctx.CreateViolation(
								fmt.Sprintf("Section name mismatch at %s:%d", file, lineNum),
								"section:name-mismatch",
								fmt.Sprintf("Change end name from \"%s\" to \"%s\"", endName, open.name),
								"Start and end section names must match",
								file, lineNum, fmt.Sprintf("Start: \"%s\" at line %d, End: \"%s\"", open.name, open.line, endName), nil))
						}
					}
				}
			}
		}
		sections := ctx.Sections(file)
		var checkSection func(s SectionInfo)
		checkSection = func(s SectionInfo) {
			sectionContent := content[s.StartIndex:s.EndIndex]
			sectionLines := strings.Split(sectionContent, "\n")
			nonEmpty := 0
			for _, line := range sectionLines[1 : len(sectionLines)-1] {
				trimmed := strings.TrimSpace(line)
				if trimmed != "" && !strings.HasPrefix(trimmed, "//") && !strings.HasPrefix(trimmed, "#") {
					nonEmpty++
				}
			}
			if nonEmpty == 0 && len(s.Children) == 0 {
				violations = append(violations, ctx.CreateViolation(
					fmt.Sprintf("Empty section \"%s\" in %s", s.Name, file),
					"section:empty",
					"Remove the empty section or add content to it",
					"Empty sections add noise without providing value",
					fmt.Sprintf("%s#%s", file, s.Name), s.StartLine, "", nil))
			}
			for _, child := range s.Children {
				checkSection(child)
			}
		}
		for _, s := range sections {
			checkSection(s)
		}
	}
	return violations
}

func commentPolicy(ctx *PolicyContext) []Violation {
	var violations []Violation
	files, err := ctx.Files("")
	if err != nil {
		return violations
	}
	for _, file := range files {
		content := ctx.ReadText(file)
		if content == "" {
			continue
		}
		lang := GetLanguageFromPath(file)
		if lang != "typescript" {
			continue
		}
		lines := strings.Split(content, "\n")
		charIndex := 0
		inBlockComment := false
		blockCommentStartLine := 0
		blockCommentStartIndex := 0
		inJsDoc := false
		jsDocStartLine := 0
		jsDocStartIndex := 0
		for i, line := range lines {
			lineNum := i + 1
			lineStart := charIndex
			lineEnd := lineStart + len(line) + 1
			trimmed := strings.TrimSpace(line)
			if strings.HasPrefix(trimmed, "/**") && !strings.HasSuffix(trimmed, "*/") {
				inJsDoc = true
				jsDocStartLine = lineNum
				jsDocStartIndex = lineStart
				charIndex = lineEnd
				continue
			}
			if inJsDoc {
				if strings.HasSuffix(trimmed, "*/") {
					violations = append(violations, ctx.CreateViolation(
						fmt.Sprintf("JSDoc comment in %s:%d", file, jsDocStartLine),
						"comment:jsdoc",
						"Remove JSDoc and document in README.md or AGENTS.md",
						"Documentation is centralized, not inline",
						file, jsDocStartLine, "", nil))
					inJsDoc = false
				}
				charIndex = lineEnd
				continue
			}
			if strings.HasPrefix(trimmed, "/*") && !strings.HasPrefix(trimmed, "/**") && !strings.HasSuffix(trimmed, "*/") {
				inBlockComment = true
				blockCommentStartLine = lineNum
				blockCommentStartIndex = lineStart
				charIndex = lineEnd
				continue
			}
			if inBlockComment {
				if strings.HasSuffix(trimmed, "*/") {
					violations = append(violations, ctx.CreateViolation(
						fmt.Sprintf("Block comment in %s:%d", file, blockCommentStartLine),
						"comment:block",
						"Remove block comment and document in README.md or AGENTS.md",
						"Documentation is centralized, not inline",
						file, blockCommentStartLine, "", nil))
					inBlockComment = false
				}
				charIndex = lineEnd
				continue
			}
			if strings.HasPrefix(trimmed, "/*") && strings.HasSuffix(trimmed, "*/") {
				if strings.HasPrefix(trimmed, "/**") {
					violations = append(violations, ctx.CreateViolation(
						fmt.Sprintf("JSDoc comment in %s:%d", file, lineNum),
						"comment:jsdoc",
						"Remove JSDoc and document in README.md or AGENTS.md",
						"Documentation is centralized, not inline",
						file, lineNum, truncate(trimmed, 80), nil))
				} else {
					violations = append(violations, ctx.CreateViolation(
						fmt.Sprintf("Block comment in %s:%d", file, lineNum),
						"comment:block",
						"Remove block comment and document in README.md or AGENTS.md",
						"Documentation is centralized, not inline",
						file, lineNum, truncate(trimmed, 80), nil))
				}
				charIndex = lineEnd
				continue
			}
			if strings.HasPrefix(trimmed, "// #region") || strings.HasPrefix(trimmed, "// #endregion") {
				charIndex = lineEnd
				continue
			}
			if strings.Contains(trimmed, "[DEBUG]") {
				charIndex = lineEnd
				continue
			}
			isHeaderLine := strings.Contains(trimmed, "Copyright") ||
				strings.Contains(trimmed, "License") ||
				strings.Contains(trimmed, "SPDX") ||
				strings.Contains(trimmed, "GNU") ||
				strings.Contains(trimmed, "AGPL")
			if isHeaderLine {
				charIndex = lineEnd
				continue
			}
			charIndex = lineEnd
			_ = blockCommentStartIndex
			_ = jsDocStartIndex
		}
	}
	return violations
}

func truncate(s string, maxLen int) string {
	if len(s) <= maxLen {
		return s
	}
	return s[:maxLen]
}

