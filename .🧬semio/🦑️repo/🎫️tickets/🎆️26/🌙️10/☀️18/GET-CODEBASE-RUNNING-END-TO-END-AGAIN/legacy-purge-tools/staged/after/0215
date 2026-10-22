// #region 🧲️Header

// 2025-2026 Ueli Saluz <ueli@semio-tech.com>

// Artifact URI parsing and subpath resolution.

// #endregion 🧲️Header

package client

import (
	"os"
	"path/filepath"
	"strings"
)

// 🟨️extractFileAndSectionsFromUri extracts the file path and section names from a URI.
// 🆕️Reads repo://section/{{id}} and repo://file/{{id}}, whose ID encodes file and sections as emojis; the `repo://` prefix may be stripped.
func extractFileAndSectionsFromUri(uri string) (filePath string, sectionSlugs []string) {
	if !strings.HasPrefix(uri, "repo://") {
		uri = "repo://" + uri
	}
	if !strings.HasPrefix(uri, "repo://section/") && !strings.HasPrefix(uri, "repo://file/") {
		return "", nil
	}
	id := UriToId(uri)
	if id == "" {
		return "", nil
	}
	return IdToPath(id), IdToSectionPath(id)
}

// 🟩️extractFileAndDefinitionFromUri extracts the file path and definition name from a URI.
// ▶️Reads repo://definition/{{id}}, whose ID encodes file, sections and definition as emojis; the `repo://` prefix may be stripped.
func extractFileAndDefinitionFromUri(uri string) (filePath string, defName string) {
	if !strings.HasPrefix(uri, "repo://") {
		uri = "repo://" + uri
	}
	if !strings.HasPrefix(uri, "repo://definition/") {
		return "", ""
	}
	id := UriToId(uri)
	if id == "" {
		return "", ""
	}
	return IdToPath(id), IdToDefinitionName(id)
}

// 🟦️ArtifactRef holds the data fields for a artifact ref record.
type ArtifactRef struct {
	Kind         string
	Path         string
	SectionParts []string
}

// ❌️ParseArtifactRef MUST return the parsed result or an error for invalid input.
// 💾️ParseArtifactRef parses and returns the artifact ref from the input.
func ParseArtifactRef(ref string) ArtifactRef {
	clean := strings.ReplaceAll(ref, "\uFE0E", "")
	clean = strings.ReplaceAll(clean, "\uFE0F", "")
	if uri := IdToUri(clean); uri != "" && strings.HasPrefix(uri, "repo://") {
		if containsUriSection(uri) && !containsUriDefinition(uri) {
			filePath, sectionSlugs := extractFileAndSectionsFromUri(uri)
			return ArtifactRef{Kind: "section", Path: NormalizePath(filePath), SectionParts: sectionSlugs}
		}
		if containsUriDefinition(uri) {
			filePath, defName := extractFileAndDefinitionFromUri(uri)
			if defName != "" {
				return ArtifactRef{Kind: "definition", Path: NormalizePath(filePath + "§" + defName)}
			}
			return ArtifactRef{Kind: "definition", Path: NormalizePath(filePath)}
		}
	}

	sectionEmojis := []string{"🔖️"}
	for _, e := range sectionEmojis {
		if strings.HasPrefix(clean, e) {
			rest := clean[len(e):]
			parts := strings.SplitN(rest, "#", 2)
			filePath := NormalizePath(parts[0])
			var sectionParts []string
			if len(parts) > 1 {
				sectionParts = strings.Split(parts[1], "#")
			}
			return ArtifactRef{Kind: "section", Path: filePath, SectionParts: sectionParts}
		}
	}

	defEmojis := []string{"🏷️", "✂️", "🪨️", "🛠️"}
	for _, e := range defEmojis {
		if strings.HasPrefix(clean, e) {
			rest := clean[len(e):]
			return ArtifactRef{Kind: "definition", Path: NormalizePath(rest)}
		}
	}

	folderEmojis := []string{"📁️", "🗃️", "📂️"}
	for _, e := range folderEmojis {
		if strings.HasPrefix(clean, e) {
			rest := clean[len(e):]
			return ArtifactRef{Kind: "folder", Path: NormalizePath(strings.TrimSuffix(rest, "/"))}
		}
	}

	fileEmojis := []string{"💻️", "🥼️", "📜️", "📃️", "⚙️", "💾️", "⚖️", "📄️"}
	for _, e := range fileEmojis {
		if strings.HasPrefix(clean, e) {
			rest := clean[len(e):]
			return ArtifactRef{Kind: "file", Path: NormalizePath(rest)}
		}
	}

	if strings.Contains(ref, "#") {
		parts := strings.SplitN(ref, "#", 2)
		filePath := NormalizePath(parts[0])
		var sectionParts []string
		if len(parts) > 1 {
			sectionParts = strings.Split(parts[1], "#")
		}
		return ArtifactRef{Kind: "section", Path: filePath, SectionParts: sectionParts}
	}
	if strings.HasSuffix(ref, "/") {
		return ArtifactRef{Kind: "folder", Path: NormalizePath(strings.TrimSuffix(ref, "/"))}
	}
	normalized := NormalizePath(ref)
	absPath := filepath.Join(rootDir, normalized)
	if info, err := os.Stat(absPath); err == nil && info.IsDir() {
		return ArtifactRef{Kind: "folder", Path: normalized}
	}
	return ArtifactRef{Kind: "file", Path: normalized}
}

// 🟪️UnSlugify MUST complete the operation successfully.
func UnSlugify(slug string) string {
	parts := strings.Split(slug, "-")
	for i, p := range parts {
		if len(p) > 0 {
			parts[i] = strings.ToUpper(p[:1]) + strings.ToLower(p[1:])
		}
	}
	return strings.Join(parts, " ")
}

// 🎯️FindSectionBySlug MUST return the matching result or an error if not found.
// 🧩️FindSectionBySlug locates and returns the matching section by slug.
func FindSectionBySlug(sections []Section, slug string) *Section {
	for i := range sections {
		if Slugify(sections[i].Name) == slug {
			return &sections[i]
		}
		if found := FindSectionBySlug(sections[i].Children, slug); found != nil {
			return found
		}
	}
	return nil
}

// 🟫️ResolveSectionName MUST return the resolved value or an error if unresolvable.
// 📩️ResolveSectionName resolves and returns the section name.
func ResolveSectionName(filePath string, slug string) string {
	absPath := filepath.Join(rootDir, filePath)
	content, err := ReadTextFile(absPath)
	if err != nil {
		return UnSlugify(slug)
	}
	lang := GetLanguage(filePath)
	if lang == nil || !lang.SupportsSections() {
		return UnSlugify(slug)
	}
	sections := lang.ParseSections(content)
	section := FindSectionBySlug(sections, slug)
	if section != nil {
		return section.Name
	}
	return UnSlugify(slug)
}

// 💠️SectionIdValueToUriPath MUST complete the operation successfully.
func SectionIdValueToUriPath(value string) string {
	hashIdx := strings.Index(value, "#")
	if hashIdx < 0 {
		return PathToUriPath(value)
	}
	filePath := value[:hashIdx]
	rest := value[hashIdx+1:]
	sectionParts := strings.Split(rest, "#")
	result := PathToUriPath(filePath)
	for _, p := range sectionParts {
		result += "/" + PathToUriPath(p)
	}
	return result
}

// 🔳️DefinitionIdValueToUriPath MUST complete the operation successfully.
func DefinitionIdValueToUriPath(value string) string {
	hashIdx := strings.Index(value, "#")
	paragraphIdx := strings.Index(value, "§")
	if hashIdx < 0 && paragraphIdx < 0 {
		return PathToUriPath(value)
	}
	if hashIdx < 0 && paragraphIdx >= 0 {
		filePath := value[:paragraphIdx]
		defName := value[paragraphIdx+len("§"):]
		return PathToUriPath(filePath) + "/" + PathToUriPath(defName)
	}
	filePath := value[:hashIdx]
	rest := value[hashIdx+1:]
	parts := strings.Split(rest, "§")
	result := PathToUriPath(filePath)
	for _, p := range parts {
		subParts := strings.Split(p, "#")
		for _, sp := range subParts {
			result += "/" + PathToUriPath(sp)
		}
	}
	return result
}

// 🔬️ParseSectionUriPath MUST return the parsed result or an error for invalid input.
// 📝️ParseSectionUriPath parses and returns the section uri path from the input.
func ParseSectionUriPath(uriPath string) (filePath string, sectionSlugs []string) {
	parts := strings.Split(uriPath, "/")
	fileEnd := -1
	for i, p := range parts {
		if strings.Contains(p, ".") {
			fileEnd = i
		}
	}
	if fileEnd < 0 {
		return uriPath, nil
	}
	filePath = strings.Join(parts[:fileEnd+1], "/")
	if fileEnd+1 < len(parts) {
		sectionSlugs = parts[fileEnd+1:]
	}
	return
}

// 🔲️StatuteIdToUriPath MUST complete the operation successfully.
func StatuteIdToUriPath(id string) string {
	parts := strings.Split(id, "/")
	for i, p := range parts {
		parts[i] = PathToUriPath(p)
	}
	return strings.Join(parts, "/")
}

// ▪️StatuteUriPathToId MUST complete the operation successfully.
func StatuteUriPathToId(uriPath string) string {
	parts := strings.Split(uriPath, "/")
	for i, p := range parts {
		parts[i] = PathFromUriPath(p)
	}
	return strings.Join(parts, "/")
}

// ⚪️technologyEmojiFromCode holds the data fields for a technologyEmojiFromCode record.
func technologyEmojiFromCode(code string) string {
	switch code {
	case "i":
		return EmojiTechnologyInfra
	case "r":
		return EmojiTechnologyResearch
	case "m":
		return EmojiTechnologyMono
	}
	return EmojiTechnologyUser
}

func bundleEmojiFromCode(code string) string {
	switch code {
	case "r":
		return EmojiBundleRepo
	case "s":
		return EmojiBundleSchema
	case "b":
		return EmojiBundleBinary
	case "u":
		return EmojiBundleUI
	case "e":
		return EmojiBundleExample
	case "w":
		return EmojiBundleSite
	case "a":
		return EmojiBundleAssets
	}
	return EmojiBundleLibrary
}

// ⚫️folderEmojiFromCode holds the data fields for a folderEmojiFromCode record.
func folderEmojiFromCode(code string) string {
	if code == "req" {
		return EmojiFolderRequired
	}
	return EmojiFolderOrg
}

// 🩵️definitionEmojiFromCode holds the data fields for a definitionEmojiFromCode record.
func definitionEmojiFromCode(code string) string {
	switch code {
	case "f":
		return EmojiDefinitionInterface
	case "c":
		return EmojiDefinitionConstant
	case "t":
		return EmojiDefinitionTest
	}
	return EmojiDefinitionImpl
}

// 🩶️uriSubPathToId holds the data fields for a uriSubPathToId record.
func uriSubPathToId(parentId string, subPath string) string {
	if subPath == "fds" {
		return emojiText(EmojiFolders)
	}
	if strings.HasPrefix(subPath, "fd/") {
		fdRest := strings.TrimPrefix(subPath, "fd/")
		fdParts := strings.SplitN(fdRest, "/", 3)
		if len(fdParts) < 2 {
			return ""
		}
		fEmoji := folderEmojiFromCode(fdParts[0])
		fName := fdParts[1]
		folderId := parentId
		_ = fEmoji
		_ = fName
		if len(fdParts) == 2 {
			return emojiText(fEmoji) + Flat(fName)
		}
		return uriSubPathToId(folderId, fdParts[2])
	}
	if subPath == "fis" {
		return emojiText(EmojiFiles)
	}
	if strings.HasPrefix(subPath, "f/") {
		fRest := strings.TrimPrefix(subPath, "f/")
		fParts := strings.SplitN(fRest, "/", 2)
		fileName := PathFromUriPath(fParts[0])
		ext := filepath.Ext(fileName)
		if ext != "" {
			fileName = strings.TrimSuffix(fileName, ext)
		}
		fileId := emojiText(EmojiFileCode) + Flat(fileName)
		if len(fParts) == 1 {
			return fileId
		}
		return uriSubPathToIdFromFile(fileId, fParts[1])
	}
	if subPath == "ss" {
		return emojiText(EmojiSections)
	}
	if strings.HasPrefix(subPath, "s/") {
		sRest := strings.TrimPrefix(subPath, "s/")
		sectionParts := splitSectionUri(sRest)
		result := parentId
		for _, sp := range sectionParts {
			result += emojiText(EmojiSection) + PathFromUriPath(sp)
		}
		return result
	}
	if subPath == "ds" {
		return emojiText(EmojiDefinitions)
	}
	if strings.HasPrefix(subPath, "d/") {
		dRest := strings.TrimPrefix(subPath, "d/")
		dParts := strings.SplitN(dRest, "/", 2)
		if len(dParts) < 2 {
			return ""
		}
		dEmoji := definitionEmojiFromCode(dParts[0])
		dName := PathFromUriPath(dParts[1])
		return parentId + emojiText(dEmoji) + Flat(dName)
	}
	return ""
}

// 🩷️uriSubPathToIdFromFile holds the data fields for a uriSubPathToIdFromFile record.
func uriSubPathToIdFromFile(fileId string, subPath string) string {
	if subPath == "ss" {
		return emojiText(EmojiSections)
	}
	if strings.HasPrefix(subPath, "s/") {
		sRest := strings.TrimPrefix(subPath, "s/")
		return parseSectionAndRest(fileId, sRest)
	}
	if subPath == "ds" {
		return emojiText(EmojiDefinitions)
	}
	if strings.HasPrefix(subPath, "d/") {
		dRest := strings.TrimPrefix(subPath, "d/")
		dParts := strings.SplitN(dRest, "/", 2)
		if len(dParts) < 2 {
			return ""
		}
		dEmoji := definitionEmojiFromCode(dParts[0])
		dName := PathFromUriPath(dParts[1])
		return fileId + emojiText(dEmoji) + Flat(dName)
	}
	return ""
}

func parseSectionAndRest(parentId string, sRest string) string {
	parts := strings.SplitN(sRest, "/s/", 2)
	sectionName := PathFromUriPath(parts[0])
	if dIdx := strings.Index(parts[0], "/d/"); dIdx >= 0 {
		sectionName = PathFromUriPath(parts[0][:dIdx])
		sectionId := parentId + emojiText(EmojiSection) + sectionName
		dRest := parts[0][dIdx+3:]
		dParts := strings.SplitN(dRest, "/", 2)
		if len(dParts) < 2 {
			return ""
		}
		dEmoji := definitionEmojiFromCode(dParts[0])
		dName := PathFromUriPath(dParts[1])
		return sectionId + emojiText(dEmoji) + Flat(dName)
	}
	sectionId := parentId + emojiText(EmojiSection) + sectionName
	if len(parts) == 1 {
		return sectionId
	}
	return parseSectionAndRest(sectionId, parts[1])
}

// 💜️splitSectionUri holds the data fields for a splitSectionUri record.
func splitSectionUri(path string) []string {
	return strings.Split(path, "/s/")
}

func parseGoalUri(rest string) string {
	parts := strings.SplitN(rest, "/", 2)
	goalName := parts[0]
	result := emojiText(EmojiGoal) + Flat(goalName)
	if len(parts) == 1 {
		return result
	}
	sub := parts[1]
	if strings.HasPrefix(sub, "g/") {
		return result + parseGoalUri(strings.TrimPrefix(sub, "g/"))
	}
	if strings.HasPrefix(sub, "tk/") {
		return emojiText(EmojiTicket) + Flat(strings.TrimPrefix(sub, "tk/"))
	}
	if sub == "tks" {
		return emojiText(EmojiTickets)
	}
	if sub == "gs" {
		return emojiText(EmojiGoals)
	}
	return result
}
