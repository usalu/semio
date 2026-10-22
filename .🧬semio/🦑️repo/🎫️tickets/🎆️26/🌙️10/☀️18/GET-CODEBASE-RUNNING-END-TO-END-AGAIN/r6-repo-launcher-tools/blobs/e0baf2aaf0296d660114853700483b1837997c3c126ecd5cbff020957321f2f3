// #region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// 🗣️language chooses the language of the launcher's texts by the shared language protocol and formats the registry texts; no language is assumed.

// #endregion 🧲️Header

package launcher

import (
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"regexp"
	"strings"
)

// #region 🗣️Language

// 📖️LanguageProtocol is the part of the shared language protocol the launcher reads.
type LanguageProtocol struct {
	EnvironmentOrder       []string `json:"environmentOrder"`
	ExplicitVariable       string   `json:"explicitVariable"`
	PreferenceListVariable string   `json:"preferenceListVariable"`
	NeutralTags            []string `json:"neutralTags"`
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
	if err == nil && (protocol.ExplicitVariable == "" || len(protocol.EnvironmentOrder) == 0) {
		err = RegistryError{Path: registry.Language.Protocol, Kind: "protocol"}
	}
	return protocol, err
}

var languageSeparators = regexp.MustCompile(`[._@-]`)

// 🔤️LanguageSubtag reads the language subtag of a locale or tag; neutral locales carry none.
func (p LanguageProtocol) LanguageSubtag(value string) string {
	tag := languageSeparators.Split(strings.ToLower(strings.TrimSpace(value)), 2)[0]
	for _, neutral := range p.NeutralTags {
		if tag == neutral {
			return ""
		}
	}
	return tag
}

// 🏳️LanguageFlag reads the last --lang before the -- separator and says whether it was given.
func LanguageFlag(args []string) (string, bool) {
	end := len(args)
	for index, argument := range args {
		if argument == "--" {
			end = index
			break
		}
	}
	value, given := "", false
	for index := 0; index < end; index++ {
		if args[index] == "--lang" {
			given, value = true, ""
			if index+1 < end {
				index++
				value = args[index]
			}
		} else if strings.HasPrefix(args[index], "--lang=") {
			given, value = true, strings.TrimPrefix(args[index], "--lang=")
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

// 🗣️ResolveLanguage chooses a language: --lang, SEMIO_LANGUAGE, LANGUAGE, the first set of LC_ALL, LC_MESSAGES, LANG, then the system list; unoffered sources are skipped except the two explicit ones.
func (p LanguageProtocol) ResolveLanguage(languages, args []string, env Environment, system func() []string) (Resolution, *LanguageRefusal) {
	if flag, given := LanguageFlag(args); given {
		if !offers(languages, flag) {
			return Resolution{}, &LanguageRefusal{Kind: "flag", Value: flag}
		}
		return Resolution{flag, "--lang"}, nil
	}
	if explicit := env(p.ExplicitVariable); explicit != "" {
		if tag := p.LanguageSubtag(explicit); offers(languages, tag) {
			return Resolution{tag, p.ExplicitVariable}, nil
		}
		return Resolution{}, &LanguageRefusal{Kind: "variable", Name: p.ExplicitVariable, Value: explicit}
	}
	for _, entry := range strings.Split(env(p.PreferenceListVariable), ":") {
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
	for _, entry := range system() {
		if tag := p.LanguageSubtag(entry); offers(languages, tag) {
			return Resolution{tag, "os"}, nil
		}
	}
	return Resolution{}, &LanguageRefusal{Kind: "none"}
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

// 🌐️Refusal renders a language refusal in every offered language, one line each.
func (r *Registry) Refusal(refusal *LanguageRefusal) string {
	id := map[string]string{"none": "languageRefusal", "flag": "languageUnknownFlag", "variable": "languageUnknownVariable"}[refusal.Kind]
	lines := make([]string, 0, len(r.Languages))
	for _, language := range r.Languages {
		lines = append(lines, r.Say(language, id, map[string]string{"choice": strings.Join(r.Languages, "|"), "value": refusal.Value, "name": refusal.Name}))
	}
	return strings.Join(lines, "\n")
}

// 🌍️EverySay renders a registry text in every offered language, one line each.
func (r *Registry) EverySay(id string, values map[string]string) string {
	lines := make([]string, 0, len(r.Languages))
	for _, language := range r.Languages {
		lines = append(lines, r.Say(language, id, values))
	}
	return strings.Join(lines, "\n")
}

// 🧱️Builtin holds the launcher's own bilingual texts for the moments when no registry is usable, keyed by identifier; they are always printed in every language.
var Builtin = map[string]Text{
	"noRepository":          Text{"en": "No repository found: {start} and its parents lack a paired package.json (with workspaces) and 📋️project.json. Run repo inside a checkout, or set SEMIO_WORKSPACE_ROOT.", "de": "Kein Repository gefunden: {start} und seine übergeordneten Ordner haben kein gepaartes package.json (mit workspaces) und 📋️project.json. Starte repo in einem Checkout oder setze SEMIO_WORKSPACE_ROOT."},
	"rootUnreadable":        Text{"en": "The file {file} cannot be read.", "de": "Die Datei {file} ist nicht lesbar."},
	"rootMalformed":         Text{"en": "The file {file} is not valid JSON.", "de": "Die Datei {file} ist kein gültiges JSON."},
	"registry.unreadable":   Text{"en": "The runtime registry {path} cannot be read.", "de": "Die Laufzeitregistratur {path} ist nicht lesbar."},
	"registry.malformed":    Text{"en": "The runtime registry {path} is malformed (field: {subject}).", "de": "Die Laufzeitregistratur {path} ist fehlerhaft aufgebaut (Feld: {subject})."},
	"registry.contract":     Text{"en": "The runtime registry declares contract {found}, this launcher implements contract {expected}. Rebuild the launcher with the install script of the launcher module, or run repo run repo-mcp:install.", "de": "Die Laufzeitregistratur deklariert Vertrag {found}, dieser Launcher setzt Vertrag {expected} um. Baue den Launcher mit dem Installationsskript des Launcher-Moduls neu oder führe repo run repo-mcp:install aus."},
	"registry.incomplete":   Text{"en": "The runtime registry {path} lacks languages, a language protocol, routing, a cache layout or implementation entries.", "de": "Der Laufzeitregistratur {path} fehlen Sprachen, ein Sprachprotokoll, die Routenwahl, das Cache-Layout oder Implementierungs-Einstiege."},
	"registry.selected":     Text{"en": "The runtime registry {path} selects the undeclared runtime {subject}.", "de": "Die Laufzeitregistratur {path} wählt die nicht deklarierte Laufzeit {subject}."},
	"registry.runtime":      Text{"en": "The runtime registry {path} declares an incomplete runtime: {subject}.", "de": "Die Laufzeitregistratur {path} deklariert eine unvollständige Laufzeit: {subject}."},
	"registry.distribution": Text{"en": "The runtime registry {path} declares an invalid distribution: {subject}.", "de": "Die Laufzeitregistratur {path} deklariert eine ungültige Distribution: {subject}."},
	"registry.entry":        Text{"en": "The runtime registry {path}: the implementation entry {subject} names an undeclared runtime or no script.", "de": "Laufzeitregistratur {path}: Der Implementierungs-Einstieg {subject} nennt eine nicht deklarierte Laufzeit oder kein Skript."},
	"registry.message":      Text{"en": "The runtime registry {path}: the text {subject} is missing, lacks a language, or its placeholders differ between languages.", "de": "Laufzeitregistratur {path}: Der Text {subject} fehlt, es fehlt eine Sprache, oder die Platzhalter unterscheiden sich zwischen den Sprachen."},
	"languageProtocol":      Text{"en": "The language protocol {path} cannot be used.", "de": "Das Sprachprotokoll {path} ist nicht verwendbar."},
	"unexpected":            Text{"en": "Unexpected error: {reason}", "de": "Unerwarteter Fehler: {reason}"},
	"versionLauncher":       Text{"en": "repo launcher {version} (contract {contract})", "de": "repo Launcher {version} (Vertrag {contract})"},
	"versionNoRepository":   Text{"en": "no repository here: runtime and implementation versions need a checkout", "de": "kein Repository hier: Laufzeit- und Implementierungsversion brauchen einen Checkout"},
}

// 🌍️BuiltinLanguages are the languages of the built-in texts.
var BuiltinLanguages = []string{"en", "de"}

func builtin(text Text, values map[string]string) string {
	lines := make([]string, 0, len(BuiltinLanguages))
	for _, language := range BuiltinLanguages {
		lines = append(lines, Format(text[language], values))
	}
	return strings.Join(lines, "\n")
}

func asFailure(err error, target *Failure) bool      { return errors.As(err, target) }
func asUnknown(err error, target *UnknownError) bool { return errors.As(err, target) }
func asExit(err error, target *ExitError) bool       { return errors.As(err, target) }

// #endregion 🗣️Language
