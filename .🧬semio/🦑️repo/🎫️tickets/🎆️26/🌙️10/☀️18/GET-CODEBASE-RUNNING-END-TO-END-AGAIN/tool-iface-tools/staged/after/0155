package statutes

import (
	json "encoding/json"
	os "os"
	filepath "path/filepath"
	sort "sort"
	strings "strings"
	unicode "unicode"

	model "github.com/usalu/semio/repo/model"
	workspace "github.com/usalu/semio/repo/workspace"
)

// #region 🔣️Path Emoji

// 🔏️pathEmojiEntry is one language-neutral file or directory identity fact.
type pathEmojiEntry struct {
	Path     string `json:"path"`
	NodeKind string `json:"nodeKind"`
	Reserved bool   `json:"reserved,omitempty"`
}

// ⚠️pathEmojiFinding is one deterministic naming statute result.
type pathEmojiFinding struct {
	Kind    string `json:"kind"`
	Path    string `json:"path"`
	Sibling string `json:"sibling,omitempty"`
	Emoji   string `json:"emoji,omitempty"`
}

// 🪞️foldPathEmojiIdentity removes presentation selectors from a logical emoji identity.
func foldPathEmojiIdentity(value string) string {
	return strings.ReplaceAll(strings.ReplaceAll(value, "\uFE0E", ""), "\uFE0F", "")
}

// 😀️leadingPathEmojiIdentity extracts every contiguous leading emoji grapheme.
func leadingPathEmojiIdentity(value string) (string, string, string) {
	first, rest := model.ExtractEntityEmoji(value)
	if first == "" {
		return "", value, ""
	}
	identity := first
	for {
		next, remaining := model.ExtractEntityEmoji(rest)
		if next == "" {
			break
		}
		identity += next
		rest = remaining
	}
	return identity, rest, first
}

// ⚖️pathEmojiStatuteFindings evaluates the shared file/folder sibling namespace.
func pathEmojiStatuteFindings(entries []pathEmojiEntry, genericEmojiIdentities []string) []pathEmojiFinding {
	generic := map[string]bool{}
	for _, emoji := range genericEmojiIdentities {
		generic[foldPathEmojiIdentity(emoji)] = true
	}
	sort.Slice(entries, func(i, j int) bool { return entries[i].Path < entries[j].Path })
	seen := map[string]pathEmojiEntry{}
	findings := []pathEmojiFinding{}
	for _, entry := range entries {
		name := entry.Path
		if index := strings.LastIndex(name, "/"); index >= 0 {
			name = name[index+1:]
		}
		identity, rest, first := leadingPathEmojiIdentity(name)
		reservedDocumentation := rest == "README" || strings.HasPrefix(rest, "README.") || rest == "LICENSE" || strings.HasPrefix(rest, "LICENSE.") || rest == "AGENTS.md"
		if entry.NodeKind == "file" && reservedDocumentation {
			if identity != "" {
				findings = append(findings, pathEmojiFinding{Kind: "reserved-emoji", Path: entry.Path, Emoji: identity})
			}
			continue
		}
		if entry.Reserved {
			continue
		}
		if identity == "" {
			findings = append(findings, pathEmojiFinding{Kind: "missing", Path: entry.Path})
			continue
		}
		multiple := identity != first
		for index := range rest {
			if emoji, _ := model.ExtractEntityEmoji(rest[index:]); emoji != "" {
				multiple = true
				break
			}
		}
		if multiple {
			findings = append(findings, pathEmojiFinding{Kind: "multiple", Path: entry.Path, Emoji: identity})
		}
		if generic[foldPathEmojiIdentity(first)] {
			findings = append(findings, pathEmojiFinding{Kind: "generic", Path: entry.Path, Emoji: first})
		}
		if (model.EmojiText(first) != strings.ReplaceAll(first, "\uFE0E", "") || strings.Contains(first, "\u20E3")) && !strings.Contains(first, "\uFE0F") {
			findings = append(findings, pathEmojiFinding{Kind: "presentation", Path: entry.Path, Emoji: first})
		}
		if rest != "" {
			firstRest, _ := utf8DecodeRuneInString(rest)
			if unicode.IsSpace(firstRest) {
				findings = append(findings, pathEmojiFinding{Kind: "spacing", Path: entry.Path, Emoji: identity})
			}
		}
		parent := ""
		if index := strings.LastIndex(entry.Path, "/"); index >= 0 {
			parent = entry.Path[:index]
		}
		key := parent + "\x00" + foldPathEmojiIdentity(first)
		if previous, ok := seen[key]; ok {
			findings = append(findings, pathEmojiFinding{Kind: "duplicate", Path: entry.Path, Sibling: previous.Path, Emoji: foldPathEmojiIdentity(first)})
		} else {
			seen[key] = entry
		}
	}
	return findings
}

// 🔡️utf8DecodeRuneInString isolates the standard-library decoding call for path spacing checks.
func utf8DecodeRuneInString(value string) (rune, int) {
	for _, character := range value {
		return character, len(string(character))
	}
	return 0, 0
}

type pathEmojiFixedContract struct {
	PathPattern string `json:"pathPattern"`
	Descendants string `json:"descendants"`
	Scope       struct {
		Kind                     string `json:"kind"`
		EcosystemID              string `json:"ecosystemId"`
		FixedFilenameContractID  string `json:"fixedFilenameContractId"`
		FixedDirectoryContractID string `json:"fixedDirectoryContractId"`
		Path                     string `json:"path"`
	} `json:"scope"`
}

type pathEmojiTaxonomy struct {
	PathEmojiPolicy struct {
		GenericEmojiIdentities        []string `json:"genericEmojiIdentities"`
		ReservedSubtreeDirectoryNames []string `json:"reservedSubtreeDirectoryNames"`
	} `json:"pathEmojiPolicy"`
	FixedFilenameContracts  map[string]pathEmojiFixedContract `json:"fixedFilenameContracts"`
	FixedDirectoryContracts map[string]pathEmojiFixedContract `json:"fixedDirectoryContracts"`
	PathExclusions          map[string]struct {
		Path string `json:"path"`
	} `json:"pathExclusions"`
}

func pathEmojiPackageContext(root, path string) (bool, string) {
	parts := strings.Split(path, "/")
	parent := parts[:len(parts)-1]
	packagesIndex := -1
	for index := len(parent) - 1; index >= 0; index-- {
		_, rest, first := leadingPathEmojiIdentity(parent[index])
		if rest == "packages" && foldPathEmojiIdentity(first) == foldPathEmojiIdentity("📦️") {
			packagesIndex = index
			break
		}
	}
	ecosystemID := ""
	if packagesIndex >= 0 && packagesIndex+1 < len(parent) {
		ecosystemID = parent[packagesIndex+1]
	}
	parentPath := filepath.Join(root, filepath.FromSlash(strings.Join(parent, "/")))
	adjacentEcosystem := ""
	for _, manifest := range []struct{ name, ecosystemID string }{{"Cargo.toml", "🦀️rust"}, {"go.mod", "🐹️go"}, {"package.json", "🟦️typescript"}} {
		_, manifestError := os.Lstat(filepath.Join(parentPath, manifest.name))
		if filepath.Base(path) == manifest.name || manifestError == nil {
			adjacentEcosystem = manifest.ecosystemID
			break
		}
	}
	if ecosystemID == "" {
		ecosystemID = adjacentEcosystem
	}
	packageRoot := packagesIndex == len(parent)-2 || adjacentEcosystem != ""
	return packageRoot, ecosystemID
}

func pathEmojiContractApplies(root, path string, contract pathEmojiFixedContract, taxonomy pathEmojiTaxonomy) bool {
	matched, err := workspace.Match(contract.PathPattern, path)
	if err != nil || !matched {
		return false
	}
	switch contract.Scope.Kind {
	case "exact-path":
		return path == contract.Scope.Path
	case "repository-root":
		return !strings.Contains(path, "/")
	case "package-root":
		packageRoot, ecosystemID := pathEmojiPackageContext(root, path)
		return packageRoot && ecosystemID == contract.Scope.EcosystemID
	case "sibling-fixed-filename-contract":
		sibling, ok := taxonomy.FixedFilenameContracts[contract.Scope.FixedFilenameContractID]
		if !ok {
			return false
		}
		siblingPath := filepath.Join(root, filepath.FromSlash(filepath.ToSlash(filepath.Dir(path))), filepath.Base(sibling.PathPattern))
		_, siblingError := os.Lstat(siblingPath)
		return siblingError == nil
	case "fixed-directory-contract":
		parent, ok := taxonomy.FixedDirectoryContracts[contract.Scope.FixedDirectoryContractID]
		return ok && pathEmojiContractApplies(root, filepath.ToSlash(filepath.Dir(path)), parent, taxonomy)
	default:
		return true
	}
}

func pathEmojiInventory(ctx *PolicyContext) ([]pathEmojiEntry, []string, error) {
	data, err := os.ReadFile(filepath.Join(ctx.RootDir, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "🔣️taxonomy.json"))
	if err != nil {
		return nil, nil, err
	}
	var taxonomy pathEmojiTaxonomy
	if err := json.Unmarshal(data, &taxonomy); err != nil {
		return nil, nil, err
	}
	output, err := workspace.ToolCommand("git", "-C", ctx.RootDir, "ls-files", "-co", "--exclude-standard", "-z").Output()
	if err != nil {
		return nil, nil, err
	}
	rawFiles := strings.Split(string(output), "\x00")
	files := []string{}
	directories := map[string]bool{}
	for _, rawFile := range rawFiles {
		if rawFile == "" {
			continue
		}
		file := strings.TrimSuffix(workspace.NormalizePath(rawFile), "/")
		if file == "" {
			continue
		}
		info, statError := os.Lstat(filepath.Join(ctx.RootDir, filepath.FromSlash(file)))
		if statError != nil {
			continue
		}
		if info.IsDir() {
			directories[file] = true
		} else {
			files = append(files, file)
		}
		for parent := filepath.ToSlash(filepath.Dir(file)); parent != "." && parent != ""; parent = filepath.ToSlash(filepath.Dir(parent)) {
			directories[parent] = true
		}
	}
	reservedSubtrees := map[string]bool{}
	for _, name := range taxonomy.PathEmojiPolicy.ReservedSubtreeDirectoryNames {
		reservedSubtrees[foldPathEmojiIdentity(name)] = true
	}
	isExcluded := func(path string) bool {
		for _, exclusion := range taxonomy.PathExclusions {
			if strings.HasPrefix(path+"/", exclusion.Path) {
				return true
			}
		}
		return false
	}
	underReserved := func(path string) bool {
		parts := strings.Split(path, "/")
		for _, part := range parts[:len(parts)-1] {
			_, rest, _ := leadingPathEmojiIdentity(part)
			if reservedSubtrees[foldPathEmojiIdentity(part)] || reservedSubtrees[foldPathEmojiIdentity(rest)] {
				return true
			}
		}
		return false
	}
	isReservedSubtreeName := func(path string) bool {
		name := path
		if index := strings.LastIndex(name, "/"); index >= 0 {
			name = name[index+1:]
		}
		_, rest, _ := leadingPathEmojiIdentity(name)
		return reservedSubtrees[foldPathEmojiIdentity(name)] || reservedSubtrees[foldPathEmojiIdentity(rest)]
	}
	contractReserved := func(path string, contracts map[string]pathEmojiFixedContract) bool {
		for _, contract := range contracts {
			if pathEmojiContractApplies(ctx.RootDir, path, contract, taxonomy) {
				return true
			}
		}
		return false
	}
	fixedReservedRoots := []string{}
	for directory := range directories {
		for _, contract := range taxonomy.FixedDirectoryContracts {
			if contract.Descendants == "reserved" && pathEmojiContractApplies(ctx.RootDir, directory, contract, taxonomy) {
				fixedReservedRoots = append(fixedReservedRoots, directory)
				break
			}
		}
	}
	underFixedReserved := func(path string) bool {
		for _, reservedRoot := range fixedReservedRoots {
			if strings.HasPrefix(path, reservedRoot+"/") {
				return true
			}
		}
		return false
	}
	entries := []pathEmojiEntry{}
	for directory := range directories {
		if !isExcluded(directory) {
			entries = append(entries, pathEmojiEntry{Path: directory, NodeKind: "directory", Reserved: underReserved(directory) || underFixedReserved(directory) || isReservedSubtreeName(directory) || contractReserved(directory, taxonomy.FixedDirectoryContracts)})
		}
	}
	for _, file := range files {
		file = workspace.NormalizePath(file)
		if file != "" && !isExcluded(file) {
			entries = append(entries, pathEmojiEntry{Path: file, NodeKind: "file", Reserved: underReserved(file) || underFixedReserved(file) || contractReserved(file, taxonomy.FixedFilenameContracts)})
		}
	}
	return entries, taxonomy.PathEmojiPolicy.GenericEmojiIdentities, nil
}

func pathEmojiBreaches(ctx *PolicyContext, nodeKind string) []model.Breach {
	entries, generic, err := pathEmojiInventory(ctx)
	if err != nil {
		return nil
	}
	byPath := map[string]string{}
	for _, entry := range entries {
		byPath[entry.Path] = entry.NodeKind
	}
	kinds := map[string]map[string]model.Statute{
		"directory": {"missing": model.BreachFolderNameMissingEmoji, "generic": model.BreachFolderNameGenericEmoji, "presentation": model.BreachFolderNameNoncanonicalEmojiPresentation, "spacing": model.BreachFolderNameWhitespaceAfterEmoji, "duplicate": model.BreachFolderNameEmojiNotUnique, "multiple": model.BreachFolderNameMultipleEmojis},
		"file":      {"missing": model.BreachFileNameMissingEmoji, "generic": model.BreachFileNameGenericEmoji, "presentation": model.BreachFileNameNoncanonicalEmojiPresentation, "spacing": model.BreachFileNameWhitespaceAfterEmoji, "duplicate": model.BreachFileNameEmojiNotUnique, "multiple": model.BreachFileNameMultipleEmojis, "reserved-emoji": model.BreachFileNameReservedEmoji},
	}
	breachs := []model.Breach{}
	for _, finding := range pathEmojiStatuteFindings(entries, generic) {
		if byPath[finding.Path] != nodeKind {
			continue
		}
		summary := "Path \"" + finding.Path + "\" breaches the " + finding.Kind + " emoji statute"
		breachs = append(breachs, ctx.CreateBreach(summary, kinds[nodeKind][finding.Kind], finding.Path, 0, 0, finding.Path))
	}
	return breachs
}

// #endregion 🔣️Path Emoji
