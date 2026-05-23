func applyAutofixes(file string, breachs []Breach) (int, error) {
	absPath := filepath.Join(rootDir, file)
	content, err := ReadTextFile(absPath)
	if err != nil {
		return 0, err
	}
	language := GetLanguage(file)
	fixed := 0
	lines := strings.Split(content, "\n")
	sort.Slice(breachs, func(i, j int) bool {
		return breachs[i].Line > breachs[j].Line
	})
	linesToRemove := map[int]bool{}
	for _, v := range breachs {
		switch v.Kind {
		case BreachCodeFileMissingHeaderRegion:
			if language != nil && language.SupportsHeaders() {
				headerContent := generateFileHeader(file, language)
				if headerContent != "" {
					content = headerContent + "\n" + content
					lines = strings.Split(content, "\n")
					fixed++
				}
			}
		case BreachCodeSectionEmpty:
			sectionStartLine := 0
			sectionEndLine := 0
			for i := v.Line - 1; i >= 0; i-- {
				if language != nil {
					if matched, _ := language.PolicySectionStartMatch(lines[i]); matched {
						sectionStartLine = i + 1
						break
					}
				}
			}
			for i := v.Line - 1; i < len(lines); i++ {
				if language != nil {
					if matched, _ := language.PolicySectionEndMatch(lines[i]); matched {
						sectionEndLine = i + 1
						break
					}
				}
			}
			if sectionStartLine > 0 && sectionEndLine > 0 {
				for i := sectionStartLine; i <= sectionEndLine; i++ {
					linesToRemove[i] = true
				}
				hasPrecedingBlank := sectionStartLine > 1 && strings.TrimSpace(lines[sectionStartLine-2]) == ""
				hasFollowingBlank := sectionEndLine < len(lines) && strings.TrimSpace(lines[sectionEndLine]) == ""
				if hasPrecedingBlank {
					linesToRemove[sectionStartLine-1] = true
				}
				if hasFollowingBlank && !hasPrecedingBlank {
					linesToRemove[sectionEndLine+1] = true
				}
				fixed++
			}
		case BreachCodeSectionWrongFormatNewlineAfterRegion:
			if v.Line > 0 && v.Line <= len(lines) {
				if strings.TrimSpace(lines[v.Line-1]) == "" {
					linesToRemove[v.Line] = true
					fixed++
				}
			}
		case BreachCodeSectionMissingEndName:
			if v.Line > 0 && v.Line <= len(lines) && language != nil {
				line := lines[v.Line-1]
				if matched, _ := language.PolicySectionEndMatch(line); matched {
					startName := findMatchingSectionStartName(lines, v.Line-1, language)
					if startName != "" {
						lines[v.Line-1] = language.FormatSectionEnd(startName)
						fixed++
					}
				}
			}
		case BreachCodeSectionNameMismatch:
			if v.Line > 0 && v.Line <= len(lines) && language != nil {
				startName := findMatchingSectionStartName(lines, v.Line-1, language)
				if startName != "" {
					lines[v.Line-1] = language.FormatSectionEnd(startName)
					fixed++
				}
			}
		case BreachCodeDefNotNativeDocstring:
			if v.Line > 0 && v.Line <= len(lines) && language != nil {
				langName := language.Name()
				defLineNum := v.Line
				prefix := language.CommentPrefix()
				switch langName {
				case "typescript":
					var commentTexts []string
					commentStartIdx := defLineNum - 1
					for lineIndex := defLineNum - 2; lineIndex >= 0; lineIndex-- {
						line := strings.TrimSpace(lines[lineIndex])
						if line == "" {
							break
						}
						if !strings.HasPrefix(line, prefix) {
							break
						}
						commentStartIdx = lineIndex
						text := strings.TrimSpace(strings.TrimPrefix(line, prefix))
						commentTexts = append([]string{text}, commentTexts...)
					}
					if len(commentTexts) > 0 {
						var summaryLines, specLines, todoLines []string
						var identificationLine string
						inTodo := false
						for _, cl := range commentTexts {
							if strings.HasPrefix(cl, "[") && strings.Contains(cl, "](repo://definition/") {
								identificationLine = cl
								inTodo = false
								continue
							}
							if strings.HasPrefix(cl, "TODO:") || strings.HasPrefix(cl, "TODO ") {
								inTodo = true
								todoLines = append(todoLines, cl)
								continue
							}
							if inTodo {
								todoLines = append(todoLines, cl)
								continue
							}
							if isSpecText(cl) {
								specLines = append(specLines, cl)
							} else {
								summaryLines = append(summaryLines, cl)
							}
						}
						indent := ""
						if defLineNum-1 < len(lines) {
							raw := lines[defLineNum-1]
							for _, ch := range raw {
								if ch == ' ' || ch == '\t' {
									indent += string(ch)
								} else {
									break
								}
							}
						}
						var jsdocLines []string
						jsdocLines = append(jsdocLines, indent+"/**")
						for _, sl := range summaryLines {
							jsdocLines = append(jsdocLines, indent+" * "+sl)
						}
						if len(specLines) > 0 {
							if len(summaryLines) > 0 {
								jsdocLines = append(jsdocLines, indent+" *")
							}
							for _, sp := range specLines {
								jsdocLines = append(jsdocLines, indent+" * "+sp)
							}
						}
						if len(todoLines) > 0 {
							jsdocLines = append(jsdocLines, indent+" *")
							for _, td := range todoLines {
								jsdocLines = append(jsdocLines, indent+" * "+td)
							}
						}
						if identificationLine != "" {
							jsdocLines = append(jsdocLines, indent+" * "+identificationLine)
						}
						jsdocLines = append(jsdocLines, indent+" **/")
						newLines := make([]string, 0, len(lines)-len(commentTexts)+len(jsdocLines))
						newLines = append(newLines, lines[:commentStartIdx]...)
						newLines = append(newLines, jsdocLines...)
						newLines = append(newLines, lines[defLineNum-1:]...)
						lines = newLines
						fixed++
					}
				case "csharp", "rust":
					for lineIndex := defLineNum - 2; lineIndex >= 0; lineIndex-- {
						line := strings.TrimSpace(lines[lineIndex])
						if line == "" {
							break
						}
						if strings.HasPrefix(line, "///") {
							break
						}
						if strings.HasPrefix(line, "//") {
							lines[lineIndex] = strings.Replace(lines[lineIndex], "// ", "/// ", 1)
							fixed++
						} else {
							break
						}
					}
				case "python":
					var commentTexts []string
					commentStartIdx := defLineNum - 1
					for lineIndex := defLineNum - 2; lineIndex >= 0; lineIndex-- {
						line := strings.TrimSpace(lines[lineIndex])
						if line == "" {
							break
						}
						if !strings.HasPrefix(line, prefix) {
							break
						}
						commentStartIdx = lineIndex
						text := strings.TrimSpace(strings.TrimPrefix(line, prefix))
						commentTexts = append([]string{text}, commentTexts...)
					}
					if len(commentTexts) > 0 {
						var cSummary, cRequirements, cTodos []string
						var cId string
						inTodo := false
						for _, cl := range commentTexts {
							if strings.HasPrefix(cl, "[") && strings.Contains(cl, "](repo://definition/") {
								cId = cl
								inTodo = false
								continue
							}
							if strings.HasPrefix(cl, "TODO:") || strings.HasPrefix(cl, "TODO ") {
								inTodo = true
								cTodos = append(cTodos, cl)
								continue
							}
							if inTodo {
								cTodos = append(cTodos, cl)
								continue
							}
							if isSpecText(cl) {
								cRequirements = append(cRequirements, cl)
							} else {
								cSummary = append(cSummary, cl)
							}
						}
						bodyIndent := "    "
						parenDepth := 0
						for _, ch := range lines[defLineNum-1] {
							if ch == '(' {
								parenDepth++
							}
							if ch == ')' {
								parenDepth--
							}
						}
						bodyStart := defLineNum
						if parenDepth > 0 {
							for scanIdx := defLineNum; scanIdx < len(lines) && scanIdx < defLineNum+15; scanIdx++ {
								for _, ch := range lines[scanIdx] {
									if ch == '(' {
										parenDepth++
									}
									if ch == ')' {
										parenDepth--
									}
								}
								if parenDepth <= 0 {
									bodyStart = scanIdx + 1
									break
								}
							}
						}
						if bodyStart < len(lines) {
							raw := lines[bodyStart]
							detected := ""
							for _, ch := range raw {
								if ch == ' ' || ch == '\t' {
									detected += string(ch)
								} else {
									break
								}
							}
							if detected != "" {
								bodyIndent = detected
							}
						}
						existingDocStart := -1
						existingDocEnd := -1
						existingQuote := `"""`
						for bodyIdx := bodyStart; bodyIdx < len(lines) && bodyIdx < bodyStart+5; bodyIdx++ {
							trimmed := strings.TrimSpace(lines[bodyIdx])
							if trimmed == "" {
								continue
							}
							if strings.HasPrefix(trimmed, `"""`) || strings.HasPrefix(trimmed, `'''`) {
								existingDocStart = bodyIdx
								if strings.HasPrefix(trimmed, `'''`) {
									existingQuote = `'''`
								}
								afterOpen := strings.TrimPrefix(trimmed, existingQuote)
								closeIdx := strings.Index(afterOpen, existingQuote)
								if closeIdx >= 0 {
									existingDocEnd = bodyIdx
								} else {
									for scanIdx := bodyIdx + 1; scanIdx < len(lines); scanIdx++ {
										sline := strings.TrimSpace(lines[scanIdx])
										if sline == existingQuote || strings.HasSuffix(sline, existingQuote) {
											existingDocEnd = scanIdx
											break
										}
									}
								}
							}
							break
						}
						var eSummary, eRequirements, eTodos []string
						var eId string
						if existingDocStart >= 0 && existingDocEnd >= 0 {
							for lineIdx := existingDocStart; lineIdx <= existingDocEnd; lineIdx++ {
								trimmed := strings.TrimSpace(lines[lineIdx])
								trimmed = strings.TrimPrefix(trimmed, existingQuote)
								trimmed = strings.TrimSuffix(trimmed, existingQuote)
								trimmed = strings.TrimSpace(trimmed)
								if trimmed == "" {
									continue
								}
								if strings.HasPrefix(trimmed, "[") && strings.Contains(trimmed, "](repo://definition/") {
									eId = trimmed
								} else if isSpecText(trimmed) {
									eRequirements = append(eRequirements, trimmed)
								} else if strings.HasPrefix(trimmed, "TODO:") || strings.HasPrefix(trimmed, "TODO ") {
									eTodos = append(eTodos, trimmed)
								} else {
									eSummary = append(eSummary, trimmed)
								}
							}
						}
						mergedSummary := eSummary
						if len(mergedSummary) == 0 {
							mergedSummary = cSummary
						}
						mergedRequirements := cRequirements
						if len(mergedRequirements) == 0 {
							mergedRequirements = eRequirements
						}
						mergedTodos := cTodos
						if len(mergedTodos) == 0 {
							mergedTodos = eTodos
						}
						mergedId := cId
						if mergedId == "" {
							mergedId = eId
						}
						var docLines []string
						for _, sl := range mergedSummary {
							docLines = append(docLines, sl)
						}
						for _, sp := range mergedRequirements {
							docLines = append(docLines, sp)
						}
						for _, td := range mergedTodos {
							docLines = append(docLines, td)
						}
						if mergedId != "" {
							docLines = append(docLines, mergedId)
						}
						var tripleQuoteLines []string
						if len(docLines) == 1 {
							tripleQuoteLines = append(tripleQuoteLines, bodyIndent+`"""`+docLines[0]+`"""`)
						} else if len(docLines) > 1 {
							tripleQuoteLines = append(tripleQuoteLines, bodyIndent+`"""`+docLines[0])
							for i := 1; i < len(docLines); i++ {
								tripleQuoteLines = append(tripleQuoteLines, bodyIndent+docLines[i])
							}
							tripleQuoteLines = append(tripleQuoteLines, bodyIndent+`"""`)
						}
						if len(tripleQuoteLines) > 0 {
							afterDoc := bodyStart
							if existingDocEnd >= 0 {
								afterDoc = existingDocEnd + 1
							}
							newLines := make([]string, 0, len(lines))
							newLines = append(newLines, lines[:commentStartIdx]...)
							newLines = append(newLines, lines[defLineNum-1:bodyStart]...)
							newLines = append(newLines, tripleQuoteLines...)
							newLines = append(newLines, lines[afterDoc:]...)
							lines = newLines
							fixed++
						}
					}
				}
			}
		case BreachCodeDefMissingSummary:
			if v.Line > 0 && v.Line <= len(lines) && language != nil {
				defName := ""
				if idx := strings.Index(v.Scope, "::"); idx >= 0 {
					defName = v.Scope[idx+2:]
				}
				if defName != "" {
					langName := language.Name()
					prefix := language.CommentPrefix()
					summaryText := defName + " holds the data fields for a " + defName + " record."
					defLine := lines[v.Line-1]
					trimmedDef := strings.TrimSpace(defLine)
					noPub := strings.TrimPrefix(trimmedDef, "pub ")
					if strings.HasPrefix(noPub, "(") {
						if cidx := strings.Index(noPub, ") "); cidx >= 0 {
							noPub = strings.TrimSpace(noPub[cidx+2:])
						}
					}
					noExport := strings.TrimPrefix(trimmedDef, "export ")
					noExport = strings.TrimLeft(noExport, "async abstract declare default ")
					if strings.HasPrefix(noPub, "fn ") || strings.HasPrefix(noExport, "function ") || strings.HasPrefix(trimmedDef, "def ") || strings.HasPrefix(trimmedDef, "async def ") || strings.HasPrefix(trimmedDef, "func ") {
						summaryText = defName + " performs the " + defName + " operation."
					}
					if langName == "python" {
						parenDepth := 0
						for _, ch := range defLine {
							if ch == '(' {
								parenDepth++
							}
							if ch == ')' {
								parenDepth--
							}
						}
						bodyStart := v.Line
						if parenDepth > 0 {
							for scanIdx := v.Line; scanIdx < len(lines) && scanIdx < v.Line+15; scanIdx++ {
								for _, ch := range lines[scanIdx] {
									if ch == '(' {
										parenDepth++
									}
									if ch == ')' {
										parenDepth--
									}
								}
								if parenDepth <= 0 {
									bodyStart = scanIdx + 1
									break
								}
							}
						}
						docstringFound := false
						for bodyIdx := bodyStart; bodyIdx < len(lines) && bodyIdx < bodyStart+5; bodyIdx++ {
							trimmed := strings.TrimSpace(lines[bodyIdx])
							if trimmed == "" {
								continue
							}
							if strings.HasPrefix(trimmed, `"""`) || strings.HasPrefix(trimmed, `'''`) {
								docstringFound = true
								quote := `"""`
								if strings.HasPrefix(trimmed, `'''`) {
									quote = `'''`
								}
								afterOpen := strings.TrimPrefix(trimmed, quote)
								closeIdx := strings.Index(afterOpen, quote)
								bodyIndent := ""
								for _, ch := range lines[bodyIdx] {
									if ch == ' ' || ch == '\t' {
										bodyIndent += string(ch)
									} else {
										break
									}
								}
								if closeIdx >= 0 {
									existingContent := strings.TrimSpace(afterOpen[:closeIdx])
									if existingContent == "" || strings.HasPrefix(existingContent, "[") {
										if existingContent != "" {
											lines[bodyIdx] = bodyIndent + quote + summaryText
											newLines := make([]string, 0, len(lines)+2)
											newLines = append(newLines, lines[:bodyIdx+1]...)
											newLines = append(newLines, bodyIndent+existingContent)
											newLines = append(newLines, bodyIndent+quote)
											newLines = append(newLines, lines[bodyIdx+1:]...)
											lines = newLines
										} else {
											lines[bodyIdx] = bodyIndent + quote + summaryText + quote
										}
									}
								} else {
									firstContent := strings.TrimSpace(afterOpen)
									if firstContent == "" || strings.HasPrefix(firstContent, "[") {
										lines[bodyIdx] = bodyIndent + quote + summaryText
										if firstContent != "" {
											newLines := make([]string, 0, len(lines)+1)
											newLines = append(newLines, lines[:bodyIdx+1]...)
											newLines = append(newLines, bodyIndent+firstContent)
											newLines = append(newLines, lines[bodyIdx+1:]...)
											lines = newLines
										}
									}
								}
								fixed++
							}
							break
						}
						if !docstringFound {
							bodyIndent := "    "
							if bodyStart < len(lines) {
								raw := lines[bodyStart]
								detected := ""
								for _, ch := range raw {
									if ch == ' ' || ch == '\t' {
										detected += string(ch)
									} else {
										break
									}
								}
								if detected != "" {
									bodyIndent = detected
								}
							}
							newLines := make([]string, 0, len(lines)+1)
							newLines = append(newLines, lines[:bodyStart]...)
							newLines = append(newLines, bodyIndent+`"""`+summaryText+`"""`)
							newLines = append(newLines, lines[bodyStart:]...)
							lines = newLines
							fixed++
						}
					} else if langName == "typescript" {
						prevIdx := v.Line - 2
						if prevIdx >= 0 {
							prevLine := strings.TrimSpace(lines[prevIdx])
							if strings.HasSuffix(prevLine, "**/") || strings.HasSuffix(prevLine, "*/") {
								indent := ""
								for _, ch := range lines[v.Line-1] {
									if ch == ' ' || ch == '\t' {
										indent += string(ch)
									} else {
										break
									}
								}
								for scanIdx := prevIdx; scanIdx >= 0; scanIdx-- {
									sline := strings.TrimSpace(lines[scanIdx])
									if strings.HasPrefix(sline, "/**") {
										openContent := strings.TrimPrefix(sline, "/**")
										openContent = strings.TrimSpace(openContent)
										if openContent == "" || openContent == "**/" || openContent == "*/" {
											lines[scanIdx] = indent + "/** " + summaryText
										}
										fixed++
										break
									}
								}
								break
							}
						}
						newLine := prefix + " " + summaryText
						insertAt := v.Line - 1
						for insertAt > 0 {
							prev := strings.TrimSpace(lines[insertAt-1])
							if prev == "" || !strings.HasPrefix(prev, prefix) {
								break
							}
							insertAt--
						}
						newLines := make([]string, 0, len(lines)+1)
						newLines = append(newLines, lines[:insertAt]...)
						newLines = append(newLines, newLine)
						newLines = append(newLines, lines[insertAt:]...)
						lines = newLines
						fixed++
					} else if langName == "csharp" || langName == "rust" {
						prevIdx := v.Line - 2
						if prevIdx >= 0 && strings.HasPrefix(strings.TrimSpace(lines[prevIdx]), "///") {
							hasSummaryTag := false
							docStartIdx := prevIdx
							for scanIdx := prevIdx; scanIdx >= 0; scanIdx-- {
								sline := strings.TrimSpace(lines[scanIdx])
								if !strings.HasPrefix(sline, "///") {
									break
								}
								docStartIdx = scanIdx
								if strings.Contains(sline, "<summary>") {
									hasSummaryTag = true
									summaryContent := strings.TrimPrefix(sline, "///")
									summaryContent = strings.TrimSpace(summaryContent)
									summaryContent = strings.TrimPrefix(summaryContent, "<summary>")
									summaryContent = strings.TrimSuffix(summaryContent, "</summary>")
									summaryContent = strings.TrimSpace(summaryContent)
									if summaryContent == "" {
										lines[scanIdx] = "/// <summary>" + summaryText + "</summary>"
										fixed++
									}
									break
								}
							}
							if !hasSummaryTag {
								summaryLine := "/// <summary>" + summaryText + "</summary>"
								newLines := make([]string, 0, len(lines)+1)
								newLines = append(newLines, lines[:docStartIdx]...)
								newLines = append(newLines, summaryLine)
								newLines = append(newLines, lines[docStartIdx:]...)
								lines = newLines
								fixed++
							}
							break
						}
						summaryLine := "/// <summary>" + summaryText + "</summary>"
						newLines := make([]string, 0, len(lines)+1)
						newLines = append(newLines, lines[:v.Line-1]...)
						newLines = append(newLines, summaryLine)
						newLines = append(newLines, lines[v.Line-1:]...)
						lines = newLines
						fixed++
					} else {
						newLine := prefix + " " + summaryText
						insertAt := v.Line - 1
						for insertAt > 0 {
							prev := strings.TrimSpace(lines[insertAt-1])
							if prev == "" || !strings.HasPrefix(prev, prefix) {
								break
							}
							insertAt--
						}
						newLines := make([]string, 0, len(lines)+1)
						newLines = append(newLines, lines[:insertAt]...)
						newLines = append(newLines, newLine)
						newLines = append(newLines, lines[insertAt:]...)
						lines = newLines
						fixed++
					}
				}
			}
		case BreachCodeDefMissingRequirements:
			if v.Line > 0 && v.Line <= len(lines) && language != nil {
				defName := ""
				if idx := strings.Index(v.Scope, "::"); idx >= 0 {
					defName = v.Scope[idx+2:]
				}
				if defName != "" {
					langName := language.Name()
					prefix := language.CommentPrefix()
					specText := v.Excerpt
					if langName == "python" {
						parenDepth := 0
						for _, ch := range lines[v.Line-1] {
							if ch == '(' {
								parenDepth++
							}
							if ch == ')' {
								parenDepth--
							}
						}
						bodyStart := v.Line
						if parenDepth > 0 {
							for scanIdx := v.Line; scanIdx < len(lines) && scanIdx < v.Line+15; scanIdx++ {
								for _, ch := range lines[scanIdx] {
									if ch == '(' {
										parenDepth++
									}
									if ch == ')' {
										parenDepth--
									}
								}
								if parenDepth <= 0 {
									bodyStart = scanIdx + 1
									break
								}
							}
						}
						for bodyIdx := bodyStart; bodyIdx < len(lines) && bodyIdx < bodyStart+5; bodyIdx++ {
							trimmed := strings.TrimSpace(lines[bodyIdx])
							if trimmed == "" {
								continue
							}
							if strings.HasPrefix(trimmed, `"""`) || strings.HasPrefix(trimmed, `'''`) {
								quote := `"""`
								if strings.HasPrefix(trimmed, `'''`) {
									quote = `'''`
								}
								bodyIndent := ""
								for _, ch := range lines[bodyIdx] {
									if ch == ' ' || ch == '\t' {
										bodyIndent += string(ch)
									} else {
										break
									}
								}
								afterOpen := strings.TrimPrefix(trimmed, quote)
								closeIdx := strings.Index(afterOpen, quote)
								if closeIdx >= 0 {
									existingContent := strings.TrimSpace(afterOpen[:closeIdx])
									lines[bodyIdx] = bodyIndent + quote + existingContent
									newLines := make([]string, 0, len(lines)+2)
									newLines = append(newLines, lines[:bodyIdx+1]...)
									newLines = append(newLines, bodyIndent+specText)
									newLines = append(newLines, bodyIndent+quote)
									newLines = append(newLines, lines[bodyIdx+1:]...)
									lines = newLines
								} else {
									for scanIdx := bodyIdx + 1; scanIdx < len(lines); scanIdx++ {
										sline := strings.TrimSpace(lines[scanIdx])
										if sline == quote || strings.HasSuffix(sline, quote) {
											insertIdx := scanIdx
											for backIdx := scanIdx - 1; backIdx > bodyIdx; backIdx-- {
												bline := strings.TrimSpace(lines[backIdx])
												if strings.HasPrefix(bline, "[") && strings.Contains(bline, "](repo://definition/") {
													insertIdx = backIdx
													break
												}
											}
											newLines := make([]string, 0, len(lines)+1)
											newLines = append(newLines, lines[:insertIdx]...)
											newLines = append(newLines, bodyIndent+specText)
											newLines = append(newLines, lines[insertIdx:]...)
											lines = newLines
											break
										}
									}
								}
								fixed++
							}
							break
						}
					} else if langName == "typescript" {
						prevIdx := v.Line - 2
						if prevIdx >= 0 {
							prevLine := strings.TrimSpace(lines[prevIdx])
							if strings.HasSuffix(prevLine, "**/") || strings.HasSuffix(prevLine, "*/") {
								indent := ""
								for _, ch := range lines[v.Line-1] {
									if ch == ' ' || ch == '\t' {
										indent += string(ch)
									} else {
										break
									}
								}
								for scanIdx := prevIdx; scanIdx >= 0; scanIdx-- {
									sline := strings.TrimSpace(lines[scanIdx])
									if strings.HasPrefix(sline, "/**") {
										newLines := make([]string, 0, len(lines)+1)
										newLines = append(newLines, lines[:scanIdx+1]...)
										newLines = append(newLines, indent+" * "+specText)
										newLines = append(newLines, lines[scanIdx+1:]...)
										lines = newLines
										fixed++
										break
									}
								}
								break
							}
						}
						newLine := prefix + " " + specText
						newLines := make([]string, 0, len(lines)+1)
						newLines = append(newLines, lines[:v.Line-1]...)
						newLines = append(newLines, newLine)
						newLines = append(newLines, lines[v.Line-1:]...)
						lines = newLines
						fixed++
					} else if langName == "csharp" || langName == "rust" {
						prevIdx := v.Line - 2
						if prevIdx >= 0 && strings.HasPrefix(strings.TrimSpace(lines[prevIdx]), "///") {
							hasRemarks := false
							remarksEnd := -1
							for scanIdx := prevIdx; scanIdx >= 0; scanIdx-- {
								sline := strings.TrimSpace(lines[scanIdx])
								if !strings.HasPrefix(sline, "///") {
									break
								}
								if strings.Contains(sline, "</remarks>") {
									remarksEnd = scanIdx
								}
								if strings.Contains(sline, "<remarks>") {
									hasRemarks = true
									break
								}
							}
							if hasRemarks && remarksEnd >= 0 {
								newLines := make([]string, 0, len(lines)+1)
								newLines = append(newLines, lines[:remarksEnd]...)
								newLines = append(newLines, "/// "+specText)
								newLines = append(newLines, lines[remarksEnd:]...)
								lines = newLines
								fixed++
							} else {
								newLines := make([]string, 0, len(lines)+3)
								newLines = append(newLines, lines[:v.Line-1]...)
								newLines = append(newLines, "/// <remarks>")
								newLines = append(newLines, "/// "+specText)
								newLines = append(newLines, "/// </remarks>")
								newLines = append(newLines, lines[v.Line-1:]...)
								lines = newLines
								fixed++
							}
							break
						}
						newLine := "/// " + specText
						newLines := make([]string, 0, len(lines)+1)
						newLines = append(newLines, lines[:v.Line-1]...)
						newLines = append(newLines, newLine)
						newLines = append(newLines, lines[v.Line-1:]...)
						lines = newLines
						fixed++
					} else {
						newLine := prefix + " " + specText
						newLines := make([]string, 0, len(lines)+1)
						newLines = append(newLines, lines[:v.Line-1]...)
						newLines = append(newLines, newLine)
						newLines = append(newLines, lines[v.Line-1:]...)
						lines = newLines
						fixed++
					}
				}
			}
		case BreachCodeSectionMissingSummary:
			if v.Line > 0 && v.Line <= len(lines) && language != nil {
				sectionName := ""
				if idx := strings.Index(v.Scope, "#"); idx >= 0 {
					sectionName = v.Scope[idx+1:]
				}
				if sectionName != "" {
					prefix := language.CommentPrefix()
					summaryLine := prefix + " " + sectionName + " MUST provide the " + strings.ToLower(sectionName) + " functionality."
					insertAt := v.Line
					for i := v.Line; i < len(lines); i++ {
						line := strings.TrimSpace(lines[i])
						if line == "" {
							continue
						}
						if strings.HasPrefix(line, prefix) {
							commentText := strings.TrimSpace(strings.TrimPrefix(line, prefix))
							if strings.HasPrefix(commentText, "[") && strings.Contains(commentText, "](repo://section/") {
								insertAt = i + 1
								break
							}
						}
						break
					}
					newLines := make([]string, 0, len(lines)+1)
					newLines = append(newLines, lines[:insertAt]...)
					newLines = append(newLines, summaryLine)
					newLines = append(newLines, lines[insertAt:]...)
					lines = newLines
					fixed++
				}
			}
		case BreachCodeCommentInline:
			if v.Line > 0 && v.Line <= len(lines) && language != nil {
				startLine := v.Line
				prefix := language.CommentPrefix()
				var pendingBlanks []int
				for i := startLine; i <= len(lines); i++ {
					line := lines[i-1]
					trimmed := strings.TrimSpace(line)
					if trimmed == "" {
						pendingBlanks = append(pendingBlanks, i)
						continue
					}
					if i == startLine && v.Column > 1 {
						if v.Column <= len(line) {
							lines[i-1] = strings.TrimRight(line[:v.Column-1], " \t")
							pendingBlanks = nil
							continue
						}
					}
					if !strings.HasPrefix(trimmed, prefix) {
						break
					}
					if matched, _ := language.PolicySectionStartMatch(line); matched {
						break
					}
					if matched, _ := language.PolicySectionEndMatch(line); matched {
						break
					}
					isSkipDirective := false
					for _, d := range language.SkipDirectives() {
						if strings.HasPrefix(trimmed, prefix+" "+d) {
							isSkipDirective = true
							break
						}
					}
					if isSkipDirective {
						break
					}
					if strings.Contains(lines[i-1], "[DEBUG]") {
						break
					}
					for _, bl := range pendingBlanks {
						linesToRemove[bl] = true
					}
					pendingBlanks = nil
					linesToRemove[i] = true
				}
				fixed++
			}
		case BreachCodeCommentBlock, BreachCodeCommentJSDoc:
			if v.Line > 0 && v.Line <= len(lines) && language != nil {
				startLine := v.Line
				endPrefix := language.BlockCommentEnd()
				startPrefix := language.BlockCommentStart()
				for i := startLine; i <= len(lines); i++ {
					line := lines[i-1]
					if i == startLine && v.Column > 1 {
						idx := strings.Index(line[v.Column-1:], startPrefix)
						if idx >= 0 {
							idx += v.Column - 1
							left := strings.TrimRight(line[:idx], " \t")
							if strings.Contains(line[idx:], endPrefix) {

								endIdx := strings.Index(line[idx:], endPrefix) + idx
								right := ""
								if endIdx+len(endPrefix) < len(line) {
									right = line[endIdx+len(endPrefix):]
								}
								if left != "" && strings.TrimSpace(right) != "" {
									lines[i-1] = left + " " + strings.TrimLeft(right, " \t")
								} else {
									lines[i-1] = left + right
								}
								if strings.TrimSpace(lines[i-1]) == "" {
									linesToRemove[i] = true
								}
								break
							}
							if left != "" {
								lines[i-1] = left
							} else {
								linesToRemove[i] = true
							}
							continue
						}
					}
					if strings.Contains(line, endPrefix) {
						idx := strings.Index(line, endPrefix)
						if idx+len(endPrefix) < len(line) {
							lines[i-1] = strings.TrimLeft(line[idx+len(endPrefix):], " \t")
							if strings.TrimSpace(lines[i-1]) == "" {
								linesToRemove[i] = true
							}
						} else {
							linesToRemove[i] = true
						}
						break
					}
					linesToRemove[i] = true
				}
				fixed++
			}
		case BreachCodeUnicodeEmojiVariation:
			if v.Line > 0 && v.Line <= len(lines) {
				line := lines[v.Line-1]

				line = strings.ReplaceAll(line, "\uFE0E", "\uFE0F")

				textDefaultEmojis := []string{
					"\U0001F3D7",
					"\u2328",
					"\U0001F5B1",
					"\U0001F5C3",
					"\u2699",
					"\u2696",
					"\U0001F3F7",
					"\U0001F6E0",
					"\u2702",
					"\U0001F6E1",
				}
				for _, emoji := range textDefaultEmojis {

					line = strings.ReplaceAll(line, emoji+"\uFE0F", emoji)
					line = strings.ReplaceAll(line, emoji, emoji+"\uFE0F")
				}
				lines[v.Line-1] = line
				fixed++
			}
		case BreachCodeFileMissingLicense, BreachCodeFileWrongLicense:
			if language != nil {
				sections := language.ParseSections(strings.Join(lines, "\n"))
				var headerSec *Section
				for i := range sections {
					if strings.ToLower(sections[i].Name) == "header" {
						headerSec = &sections[i]
						break
					}
				}
				if headerSec != nil {
					var licenseSec *Section
					for i := range headerSec.Children {
						if strings.ToLower(headerSec.Children[i].Name) == "license" {
							licenseSec = &headerSec.Children[i]
							break
						}
					}
					prefix := language.CommentPrefix()
					licenseText := AGPLLicenseText()
					var licenseLines []string
					licenseLines = append(licenseLines, "")
					for _, ll := range strings.Split(licenseText, "\n") {
						if ll == "" {
							licenseLines = append(licenseLines, prefix)
						} else {
							licenseLines = append(licenseLines, prefix+" "+ll)
						}
					}
					licenseLines = append(licenseLines, "")
					if licenseSec != nil {
						newLines := make([]string, 0, len(lines)+len(licenseLines))
						newLines = append(newLines, lines[:licenseSec.StartLine]...)
						newLines = append(newLines, licenseLines...)
						newLines = append(newLines, lines[licenseSec.EndLine-1:]...)
						lines = newLines
					} else {
						requirementsSec := (*Section)(nil)
						for i := range headerSec.Children {
							if strings.ToLower(headerSec.Children[i].Name) == "requirements" {
								requirementsSec = &headerSec.Children[i]
								break
							}
						}
						insertBefore := headerSec.EndLine - 1
						if requirementsSec != nil {
							insertBefore = requirementsSec.StartLine - 1
						}
						regionStart := language.FormatSectionStart("License")
						regionEnd := language.FormatSectionEnd("License")
						var block []string
						block = append(block, regionStart)
						block = append(block, licenseLines...)
						block = append(block, regionEnd)
						block = append(block, "")
						newLines := make([]string, 0, len(lines)+len(block))
						newLines = append(newLines, lines[:insertBefore]...)
						newLines = append(newLines, block...)
						newLines = append(newLines, lines[insertBefore:]...)
						lines = newLines
					}
					fixed++
				}
			}
		}
	}
	systemFixed, systemErr := applySystemAutofixes(breachs)
	if systemErr != nil {
		return fixed, systemErr
	}
	fixed += systemFixed
	if len(linesToRemove) > 0 {
		var newLines []string
		for i, line := range lines {
			if !linesToRemove[i+1] {
				newLines = append(newLines, line)
			}
		}
		var collapsed []string
		for i, line := range newLines {
			if strings.TrimSpace(line) == "" && i > 0 && strings.TrimSpace(newLines[i-1]) == "" {
				continue
			}
			collapsed = append(collapsed, line)
		}
		content = strings.Join(collapsed, "\n")
	} else {
		content = strings.Join(lines, "\n")
	}
	if fixed > 0 {
		if err := WriteTextFile(absPath, content); err != nil {
			return 0, err
		}
		if err := runFormatterAfterAutofix(file, language); err != nil {
			return 0, err
		}
	}
	return fixed, nil
}

// ­ƒƒ®applySystemAutofixes holds the data fields for a applySystemAutofixes record.
func applySystemAutofixes(breachs []Breach) (int, error) {
	fixed := 0
	for _, v := range breachs {
		switch v.Kind {
		case BreachSystemDevcontainerVscodeSettingsOutside:
			settingsPath := filepath.Join(rootDir, ".vscode", "settings.json")
			settingsData, err := os.ReadFile(settingsPath)
			if err != nil {
				continue
			}
			var settings map[string]interface{}
			if err := json.Unmarshal(settingsData, &settings); err != nil {
				continue
			}
			devcontainerPath := filepath.Join(rootDir, ".devcontainer", "devcontainer.json")
			var devcontainer map[string]interface{}
			if dcData, err := os.ReadFile(devcontainerPath); err == nil {
				_ = json.Unmarshal(dcData, &devcontainer)
			}
			if devcontainer == nil {
				devcontainer = map[string]interface{}{}
			}
			customizations, _ := devcontainer["customizations"].(map[string]interface{})
			if customizations == nil {
				customizations = map[string]interface{}{}
			}
			vscodeCustom, _ := customizations["vscode"].(map[string]interface{})
			if vscodeCustom == nil {
				vscodeCustom = map[string]interface{}{}
			}
			vscodeCustom["settings"] = settings
			customizations["vscode"] = vscodeCustom
			devcontainer["customizations"] = customizations
			dcOut, err := json.MarshalIndent(devcontainer, "", "  ")
			if err != nil {
				continue
			}
			if err := os.MkdirAll(filepath.Join(rootDir, ".devcontainer"), 0755); err != nil {
				continue
			}
			if err := os.WriteFile(devcontainerPath, append(dcOut, '\n'), 0644); err != nil {
				continue
			}
			_ = os.Remove(settingsPath)
			vscodeDir := filepath.Join(rootDir, ".vscode")
			if entries, err := os.ReadDir(vscodeDir); err == nil && len(entries) == 0 {
				_ = os.Remove(vscodeDir)
			}
			fixed++
		case BreachFolderIllegalEmpty:
			folderPath := filepath.Join(rootDir, v.Excerpt)
			entries, readErr := os.ReadDir(folderPath)
			if readErr == nil && len(entries) == 0 {
				if err := os.Remove(folderPath); err == nil {
					fixed++
				}
			}
		case BreachSystemDevcontainerVscodeExtensionsOutside:
			extensionsPath := filepath.Join(rootDir, ".vscode", "extensions.json")
			extData, err := os.ReadFile(extensionsPath)
			if err != nil {
				continue
			}
			var extFile map[string]interface{}
			if err := json.Unmarshal(extData, &extFile); err != nil {
				continue
			}
			recommendations, _ := extFile["recommendations"].([]interface{})
			if recommendations == nil {
				recommendations = []interface{}{}
			}
			devcontainerPath := filepath.Join(rootDir, ".devcontainer", "devcontainer.json")
			var devcontainer map[string]interface{}
			if dcData, err := os.ReadFile(devcontainerPath); err == nil {
				_ = json.Unmarshal(dcData, &devcontainer)
			}
			if devcontainer == nil {
				devcontainer = map[string]interface{}{}
			}
			customizations, _ := devcontainer["customizations"].(map[string]interface{})
			if customizations == nil {
				customizations = map[string]interface{}{}
			}
			vscodeCustom, _ := customizations["vscode"].(map[string]interface{})
			if vscodeCustom == nil {
				vscodeCustom = map[string]interface{}{}
			}
			vscodeCustom["extensions"] = recommendations
			customizations["vscode"] = vscodeCustom
			devcontainer["customizations"] = customizations
			dcOut, err := json.MarshalIndent(devcontainer, "", "  ")
			if err != nil {
				continue
			}
			if err := os.MkdirAll(filepath.Join(rootDir, ".devcontainer"), 0755); err != nil {
				continue
			}
			if err := os.WriteFile(devcontainerPath, append(dcOut, '\n'), 0644); err != nil {
				continue
			}
			_ = os.Remove(extensionsPath)
			vscodeDir := filepath.Join(rootDir, ".vscode")
			if entries, err := os.ReadDir(vscodeDir); err == nil && len(entries) == 0 {
				_ = os.Remove(vscodeDir)
			}
			fixed++
		}
	}
	return fixed, nil
}

// ÔûÂ´©ÅfindMatchingSectionStartName holds the data fields for a findMatchingSectionStartName record.
func findMatchingSectionStartName(lines []string, endLineIdx int, language LanguagePlugin) string {
	depth := 0
	for i := endLineIdx - 1; i >= 0; i-- {
		if matched, _ := language.PolicySectionEndMatch(lines[i]); matched {
			depth++
			continue
		}
		if matched, name := language.PolicySectionStartMatch(lines[i]); matched {
			if depth > 0 {
				depth--
				continue
			}
			return name
		}
	}
	return ""
}

