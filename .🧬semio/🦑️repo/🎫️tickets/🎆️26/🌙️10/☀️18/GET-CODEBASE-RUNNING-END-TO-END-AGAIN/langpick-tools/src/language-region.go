// 📖️LanguageProtocol is the one language protocol of the repository (`⚙️runtime/🗣️language/🔣️.json`); every rule the launcher applies is read from it.
type LanguageProtocol struct {
	Flag                    string   `json:"flag"`
	ArgumentsEnd            string   `json:"argumentsEnd"`
	ExplicitVariable        string   `json:"explicitVariable"`
	PreferenceListVariable  string   `json:"preferenceListVariable"`
	PreferenceListSeparator string   `json:"preferenceListSeparator"`
	EnvironmentOrder        []string `json:"environmentOrder"`
	SubtagSeparators        string   `json:"subtagSeparators"`
	NeutralTags             []string `json:"neutralTags"`
	OsLocale                OsLocale `json:"osLocale"`
	Refusal                 map[string]Text `json:"refusal"`
	separators              *regexp.Regexp
}

// 🖥️OsLocale says how the operating system language list is read.
type OsLocale struct {
	Darwin struct {
		Command   string   `json:"command"`
		Args      []string `json:"args"`
		TimeoutMs int      `json:"timeoutMs"`
		Pattern   string   `json:"pattern"`
	} `json:"darwin"`
}

// 🏳️Resolution says which language was chosen and where it came from.
type Resolution struct{ Language, Source string }

// 🚫️LanguageRefusal says why no language could be chosen: Kind is none, flag or variable.
type LanguageRefusal struct{ Kind, Name, Value string }

// 🔤️Error names the refusal kind.
func (r LanguageRefusal) Error() string { return "language " + r.Kind }

// 📥️LoadLanguageProtocol reads the shared protocol file named by the registry.
func LoadLanguageProtocol(root string, registry *Registry) (LanguageProtocol, error) {
	var protocol LanguageProtocol
	raw, err := os.ReadFile(filepath.Join(root, registry.Language.Protocol))
	if err == nil {
		err = json.Unmarshal(raw, &protocol)
	}
	if err == nil {
		protocol.separators, err = regexp.Compile("[" + regexp.QuoteMeta(protocol.SubtagSeparators) + "]")
	}
	if err == nil && (protocol.Flag == "" || protocol.ArgumentsEnd == "" || protocol.ExplicitVariable == "" || protocol.PreferenceListSeparator == "" || protocol.SubtagSeparators == "" || len(protocol.EnvironmentOrder) == 0 || len(protocol.Refusal) == 0) {
		err = RegistryError{Path: registry.Language.Protocol, Kind: "protocol"}
	}
	return protocol, err
}

// 🔤️LanguageSubtag reads the language subtag of a locale or tag; neutral locales carry none.
func (p LanguageProtocol) LanguageSubtag(value string) string {
	tag := p.separators.Split(strings.ToLower(strings.TrimSpace(value)), 2)[0]
	for _, neutral := range p.NeutralTags {
		if tag == neutral {
			return ""
		}
	}
	return tag
}

// 🏳️LanguageFlag reads the last language flag before the arguments end and says whether it was given.
func (p LanguageProtocol) LanguageFlag(args []string) (string, bool) {
	end := len(args)
	for index, argument := range args {
		if argument == p.ArgumentsEnd {
			end = index
			break
		}
	}
	value, given := "", false
	for index := 0; index < end; index++ {
		if args[index] == p.Flag {
			given, value = true, ""
			if index+1 < end {
				index++
				value = args[index]
			}
		} else if strings.HasPrefix(args[index], p.Flag+"=") {
			given, value = true, strings.TrimPrefix(args[index], p.Flag+"=")
		}
	}
	return value, given
}

func offers(languages []string, tag string) bool {
	for _, language := range languages {
		if language == tag {
			return tag != ""
		}
	}
	return false
}

// 🗣️ResolveLanguage chooses the language of a command by the protocol: the flag, the explicit variable, the preference list, the first set locale variable, then the operating system list; unoffered sources are skipped except the two explicit ones.
func (p LanguageProtocol) ResolveLanguage(languages, args []string, env Environment, system func(OsLocale) []string) (Resolution, *LanguageRefusal) {
	if flag, given := p.LanguageFlag(args); given {
		if !offers(languages, flag) {
			return Resolution{}, &LanguageRefusal{Kind: "flag", Value: flag}
		}
		return Resolution{flag, p.Flag}, nil
	}
	if explicit := env(p.ExplicitVariable); explicit != "" {
		if tag := p.LanguageSubtag(explicit); offers(languages, tag) {
			return Resolution{tag, p.ExplicitVariable}, nil
		}
		return Resolution{}, &LanguageRefusal{Kind: "variable", Name: p.ExplicitVariable, Value: explicit}
	}
	for _, entry := range strings.Split(env(p.PreferenceListVariable), p.PreferenceListSeparator) {
		if tag := p.LanguageSubtag(entry); offers(languages, tag) {
			return Resolution{tag, p.PreferenceListVariable}, nil
		}
	}
	for _, name := range p.EnvironmentOrder {
		if env(name) == "" {
			continue
		}
		if tag := p.LanguageSubtag(env(name)); offers(languages, tag) {
			return Resolution{tag, name}, nil
		}
		break
	}
	for _, entry := range system(p.OsLocale) {
		if tag := p.LanguageSubtag(entry); offers(languages, tag) {
			return Resolution{tag, "os"}, nil
		}
	}
	return Resolution{}, &LanguageRefusal{Kind: "none"}
}

// 🌐️Refuse renders a refusal of the protocol in every offered language, one line each; a language without its own text reads the English one.
func (p LanguageProtocol) Refuse(languages []string, refusal *LanguageRefusal) string {
	lines := make([]string, 0, len(languages))
	for _, language := range languages {
		text, found := p.Refusal[refusal.Kind][language]
		if !found {
			text = p.Refusal[refusal.Kind]["en"]
		}
		lines = append(lines, Format(text, map[string]string{"choice": strings.Join(languages, "|"), "value": refusal.Value, "name": refusal.Name}))
	}
	return strings.Join(lines, "\n")
}

// ✍️Format fills {name} placeholders of one text.
func Format(text string, values map[string]string) string {
	for name, value := range values {
		text = strings.ReplaceAll(text, "{"+name+"}", value)
	}
	return text
}

// 💬️Say renders a registry text in one language.
func (r *Registry) Say(language, id string, values map[string]string) string {
	return Format(r.Messages[id][language], values)
}

