// repo/tools/sections.go

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
	"regexp"
	"strings"
)

type sectionPatterns struct {
	Start *regexp.Regexp
	End   *regexp.Regexp
}

var patterns = map[string]sectionPatterns{
	"typescript": {
		Start: regexp.MustCompile(`(?i)^\s*//\s*#region\s+(.+?)\s*$`),
		End:   regexp.MustCompile(`(?i)^\s*//\s*#endregion(?:\s+(.+?))?\s*$`),
	},
	"python": {
		Start: regexp.MustCompile(`(?i)^\s*#\s*region\s+(.+?)\s*$`),
		End:   regexp.MustCompile(`(?i)^\s*#\s*endregion(?:\s+(.+?))?\s*$`),
	},
	"csharp": {
		Start: regexp.MustCompile(`(?i)^\s*#region\s+(.+?)\s*$`),
		End:   regexp.MustCompile(`(?i)^\s*#endregion(?:\s+(.+?))?\s*$`),
	},
}

func ParseCodeSections(content string, language string) []SectionInfo {
	lines := strings.Split(content, "\n")
	var stack []*SectionInfo
	var roots []SectionInfo
	pat, ok := patterns[language]
	if !ok {
		return roots
	}
	charIndex := 0
	for i, line := range lines {
		lineStart := charIndex
		lineNum := i + 1
		if match := pat.Start.FindStringSubmatch(line); match != nil {
			name := strings.TrimSpace(match[1])
			section := &SectionInfo{
				Name:       name,
				StartLine:  lineNum,
				EndLine:    -1,
				StartIndex: lineStart,
				EndIndex:   -1,
				Children:   []SectionInfo{},
			}
			if len(stack) > 0 {
				parent := stack[len(stack)-1]
				parent.Children = append(parent.Children, *section)
				section = &parent.Children[len(parent.Children)-1]
			}
			stack = append(stack, section)
		} else if pat.End.MatchString(line) {
			if len(stack) > 0 {
				section := stack[len(stack)-1]
				section.EndLine = lineNum
				section.EndIndex = charIndex + len(line)
				stack = stack[:len(stack)-1]
				if len(stack) == 0 {
					roots = append(roots, *section)
				}
			}
		}
		charIndex += len(line) + 1
	}
	return roots
}

func ParseMarkdownSections(content string) []SectionInfo {
	lines := strings.Split(content, "\n")
	var sections []SectionInfo
	type stackItem struct {
		level   int
		section *SectionInfo
	}
	var stack []stackItem
	headerRe := regexp.MustCompile(`^(#{1,6})\s+(.+?)\s*$`)
	frontmatterLines := 0
	if strings.HasPrefix(content, "---") {
		endIndex := strings.Index(content[3:], "---")
		if endIndex != -1 {
			frontmatterContent := content[:endIndex+6]
			frontmatterLines = strings.Count(frontmatterContent, "\n")
		}
	}
	charIndex := 0
	for i, line := range lines {
		lineStart := charIndex
		if match := headerRe.FindStringSubmatch(line); match != nil {
			level := len(match[1])
			name := strings.TrimSpace(match[2])
			for len(stack) > 0 && stack[len(stack)-1].level >= level {
				popped := stack[len(stack)-1]
				popped.section.EndLine = frontmatterLines + i
				popped.section.EndIndex = lineStart - 1
				stack = stack[:len(stack)-1]
			}
			section := &SectionInfo{
				Name:       name,
				StartLine:  frontmatterLines + i + 1,
				EndLine:    -1,
				StartIndex: lineStart,
				EndIndex:   -1,
				Children:   []SectionInfo{},
			}
			if len(stack) > 0 {
				parent := stack[len(stack)-1]
				parent.section.Children = append(parent.section.Children, *section)
				section = &parent.section.Children[len(parent.section.Children)-1]
			} else {
				sections = append(sections, *section)
				section = &sections[len(sections)-1]
			}
			stack = append(stack, stackItem{level: level, section: section})
		}
		charIndex += len(line) + 1
	}
	for len(stack) > 0 {
		popped := stack[len(stack)-1]
		popped.section.EndLine = frontmatterLines + len(lines)
		popped.section.EndIndex = len(content)
		stack = stack[:len(stack)-1]
	}
	return sections
}

func ParseSections(content string, filePath string) []SectionInfo {
	lang := GetLanguageFromPath(filePath)
	if lang == "markdown" {
		return ParseMarkdownSections(content)
	}
	if lang != "" {
		return ParseCodeSections(content, lang)
	}
	return nil
}

func FindSection(sections []SectionInfo, name string) *SectionInfo {
	for i := range sections {
		if sections[i].Name == name {
			return &sections[i]
		}
		if found := FindSection(sections[i].Children, name); found != nil {
			return found
		}
	}
	return nil
}

