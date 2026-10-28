//! ⌨️ The non-interactive dashboard usage reference.

// #region 🔖️Presentation
/// 📖️ The English usage reference: every verb with every flag it reads. `FLAGS` states the same flags as data; a test fails
/// when the two, the German text, the dispatch table, the argument readers and the README verb table drift apart.
pub const USAGE_EN: &str = "repo dashboard — the developer control plane of the monorepo\n\nUsage:\n  repo dashboard [--language en|de] [--appearance dark|light] [--terminology native|reuse]\n                  [--renderer react|wgpu-wasm|wgpu-native] [--layout tabs|columns|rows] [--prefix KEY]\n                  [--bindings JSON] [--root PATH] [--config JOURNAL] [--workspace] [--help]\n                                 the interactive dashboard; --help prints its keyboard help\n  repo dashboard commands [words…] [--json] [--all] [--check] [--refresh] [--root PATH] [--snapshot PATH]\n                                 list or search the command registry; --check proves every declaration\n  repo dashboard run <id> [--param k=v|flag]… [--env K=V]… [--detach] [--wait-ready] [--dry-run] [--json] [--raw]\n                 [--timeout S] [--refresh] [--root PATH] [--snapshot PATH] [-- args…]\n                                 run a registry command as a daemon task\n  repo dashboard tasks [--json] [--root PATH]\n                                 list the daemon tasks\n  repo dashboard logs <task> [--follow] [--raw] [--root PATH]\n                                 print the output of a task\n  repo dashboard stop|restart|kill <task> [--group] [--timeout S] [--root PATH]\n  repo dashboard open <task> [--print] [--root PATH]\n                                 open the ready address of a task in the browser\n  repo dashboard daemon start|status|stop|attach|serve [--root PATH]\n                                 manage the workspace daemon\n  repo dashboard preferences show|set [--language L] [--appearance A] [--terminology T] [--renderer R] [--layout L]\n                  [--prefix KEY] [--bindings JSON] [--root PATH] [--config JOURNAL] [--workspace]\n                                 read or record preferences\n  repo dashboard catalog [--json]         list the playground catalog\n  repo dashboard plugin registry generate|check\n  repo dashboard command-tree [--dump-tree] [--root PATH]\n                                 print the workspace command tree\n  repo dashboard --help                   this text\n\nThe dashboard has no other verbs: every task of the monorepo is a registry command. Find an id with\n`repo dashboard commands <words> --json`, start it with `repo dashboard run <id>`; the routes of the root script are the targets\nof the root project, for example `repo dashboard run workspace:test` or `repo dashboard run workspace:dev-storybook`.\nHuman developers start tasks through the dashboard command catalog and inspect their ready address in the task overview. A task is named by the session id every verb prints, or by its command id.";

/// 📘️ The German usage reference; it names the same verbs and flags in the same order as [`USAGE_EN`].
pub const USAGE_DE: &str = "repo dashboard — die Entwicklerzentrale des Monorepos\n\nVerwendung:\n  repo dashboard [--language en|de] [--appearance dark|light] [--terminology native|reuse]\n                  [--renderer react|wgpu-wasm|wgpu-native] [--layout tabs|columns|rows] [--prefix KEY]\n                  [--bindings JSON] [--root PATH] [--config JOURNAL] [--workspace] [--help]\n                                 das interaktive Dashboard; --help zeigt seine Tastaturhilfe\n  repo dashboard commands [words…] [--json] [--all] [--check] [--refresh] [--root PATH] [--snapshot PATH]\n                                 das Befehlsregister auflisten oder durchsuchen; --check prüft jede Deklaration\n  repo dashboard run <id> [--param k=v|flag]… [--env K=V]… [--detach] [--wait-ready] [--dry-run] [--json] [--raw]\n                 [--timeout S] [--refresh] [--root PATH] [--snapshot PATH] [-- args…]\n                                 einen Registerbefehl als Dienstaufgabe ausführen\n  repo dashboard tasks [--json] [--root PATH]\n                                 die Aufgaben des Dienstes auflisten\n  repo dashboard logs <task> [--follow] [--raw] [--root PATH]\n                                 die Ausgabe einer Aufgabe ausgeben\n  repo dashboard stop|restart|kill <task> [--group] [--timeout S] [--root PATH]\n  repo dashboard open <task> [--print] [--root PATH]\n                                 die Bereitschaftsadresse einer Aufgabe im Browser öffnen\n  repo dashboard daemon start|status|stop|attach|serve [--root PATH]\n                                 den Dienst des Arbeitsbereichs verwalten\n  repo dashboard preferences show|set [--language L] [--appearance A] [--terminology T] [--renderer R] [--layout L]\n                  [--prefix KEY] [--bindings JSON] [--root PATH] [--config JOURNAL] [--workspace]\n                                 Einstellungen lesen oder festhalten\n  repo dashboard catalog [--json]         den Playground-Katalog auflisten\n  repo dashboard plugin registry generate|check\n  repo dashboard command-tree [--dump-tree] [--root PATH]\n                                 den Befehlsbaum des Arbeitsbereichs ausgeben\n  repo dashboard --help                   dieser Text\n\nDas Dashboard kennt keine weiteren Verben: Jede Aufgabe des Monorepos ist ein Registerbefehl. Eine ID findest du mit\n`repo dashboard commands <Wörter> --json`, gestartet wird sie mit `repo dashboard run <id>`; die Routen des Root-Skripts sind die Targets\ndes Root-Projekts, zum Beispiel `repo dashboard run workspace:test` oder `repo dashboard run workspace:dev-storybook`.\nEntwickler starten Aufgaben über den Befehlskatalog des Dashboards und sehen ihre Bereitschaftsadresse in der Aufgabenübersicht. Eine Aufgabe wird durch die Sitzungs-ID benannt, die jedes Verb ausgibt, oder durch ihre Befehls-ID.";

/// 🌐️ The usage reference per declared dashboard language.
pub const TEXTS: &[(&str, &str)] = &[("en", USAGE_EN), ("de", USAGE_DE)];

/// 🗣️ The usage reference in one language, when the dashboard declares it.
pub fn text(language: &str) -> Option<&'static str> { TEXTS.iter().find(|(code, _)| *code == language).map(|(_, text)| *text) }

/// 🚩️ Every flag each verb reads, in the order `USAGE` lists them (`--help` of the dashboard itself excluded).
pub const FLAGS: &[(&str, &[&str])] = &[
    ("dashboard", &["language", "appearance", "terminology", "renderer", "layout", "prefix", "bindings", "root", "config", "workspace", "help"]),
    ("commands", &["json", "all", "check", "refresh", "root", "snapshot"]),
    ("run", &["param", "env", "detach", "wait-ready", "dry-run", "json", "raw", "timeout", "refresh", "root", "snapshot"]),
    ("tasks", &["json", "root"]),
    ("logs", &["follow", "raw", "root"]),
    ("stop", &["group", "timeout", "root"]),
    ("restart", &["group", "timeout", "root"]),
    ("kill", &["group", "timeout", "root"]),
    ("open", &["print", "root"]),
    ("daemon", &["root"]),
    ("preferences", &["language", "appearance", "terminology", "renderer", "layout", "prefix", "bindings", "root", "config", "workspace"]),
    ("catalog", &["json"]),
    ("command-tree", &["dump-tree", "root"]),
];

/// 🧭️ Presents one usage reference on standard error, where a refusal belongs.
pub fn print(text: &str) {
    eprintln!("{text}");
}

/// 📣️ Presents one usage reference on standard output, for `repo dashboard --help`.
pub fn help(text: &str) {
    println!("{text}");
}
// #endregion 🔖️Presentation

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
