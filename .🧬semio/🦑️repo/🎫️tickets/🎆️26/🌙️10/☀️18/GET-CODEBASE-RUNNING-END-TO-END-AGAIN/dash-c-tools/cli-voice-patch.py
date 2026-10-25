D="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧭️cli/"
p=D+"🦀️.rs"; s=open(p,encoding="utf8").read()
def sub(old,new,count=1):
    global s
    assert s.count(old)==count,(s.count(old),old)
    s=s.replace(old,new)
say=lambda label,args="": f'voice.say(|text| text.{label}.{"fill(&["+args+"]).into_string()" if args else "as_str().to_string()"})'
sub("use dashboard_daemon::control::{self as daemon_control, Action, Followed, RunError, Wait};",
    "use dashboard_daemon::control::{self as daemon_control, Action, Followed, RunError, Wait};\nuse dashboard_labels::DashboardLabels;")
sub("#[path = \"🌐️open/🦀️.rs\"]\npub mod browser;\n",
'''#[path = "🌐️open/🦀️.rs"]
pub mod browser;

/// 🗣️ The languages the verbs speak to the developer: the explicitly chosen one, or every declared one while none is chosen (their texts are joined with ` / `).
#[derive(Clone)]
pub struct Voice { texts: Vec<&'static DashboardLabels> }

impl Voice {
    pub fn new(texts: Vec<&'static DashboardLabels>) -> Self { Self { texts } }

    /// 💬 One catalogue text in each language of the voice.
    pub fn say(&self, pick: impl Fn(&DashboardLabels) -> String) -> String { self.texts.iter().map(|text| pick(text)).collect::<Vec<_>>().join(" / ") }
}
''')
sub("    pub fn parse(argv: &[String], valued: &[&str]) -> Result<Self, String> {","    pub fn parse(argv: &[String], valued: &[&str], voice: &Voice) -> Result<Self, String> {")
sub('Some((name, _)) => return Err(format!("--{name} takes no value")),','Some((name, _)) => return Err(voice.say(|text| text.cli_flag_no_value.fill(&[("name", name)]).into_string())),')
sub('.ok_or_else(|| format!("--{flag} needs a value"))?.clone())),','.ok_or_else(|| voice.say(|text| text.cli_flag_needs_value.fill(&[("name", flag)]).into_string()))?.clone())),')
sub("    pub fn pairs(&self, name: &str) -> Result<Vec<(String, String)>, String> {","    pub fn pairs(&self, name: &str, voice: &Voice) -> Result<Vec<(String, String)>, String> {")
sub('.ok_or_else(|| format!("--{name} {text:?} is not `key=value`"))).collect()','.ok_or_else(|| voice.say(|label| label.cli_pair_malformed.fill(&[("name", name), ("text", text)]).into_string()))).collect()')
sub("    pub fn request(&self, kind_of: &dyn Fn(&str) -> Option<ParameterKind>) -> Result<Request, String> {","    pub fn request(&self, kind_of: &dyn Fn(&str) -> Option<ParameterKind>, voice: &Voice) -> Result<Request, String> {")
sub('Some(_) => Err(format!("--param {text:?} is not `id` or `id=value`")),','Some(_) => Err(voice.say(|label| label.cli_param_malformed.fill(&[("text", text)]).into_string())),')
sub('Some(kind) => Err(format!("--param {text}: a {} parameter needs a value, write `--param {text}=<value>`", match kind { ParameterKind::Choice => "choice", ParameterKind::Text => "text", ParameterKind::Flag => "flag" })),','Some(kind) => Err(voice.say(|label| label.cli_param_needs_value.fill(&[("text", text), ("kind", match kind { ParameterKind::Choice => "choice", ParameterKind::Text => "text", ParameterKind::Flag => "flag" })]).into_string())),')
sub('env: self.pairs("env")? })','env: self.pairs("env", voice)? })')
sub("fn parse(argv: &[String], valued: &[&str]) -> Result<Arguments, i32> { Arguments::parse(argv, valued).map_err(|error| refuse(&error)) }","fn parse(argv: &[String], valued: &[&str], voice: &Voice) -> Result<Arguments, i32> { Arguments::parse(argv, valued, voice).map_err(|error| refuse(&error)) }")
sub("pub fn commands(root: &Path, argv: &[String], locale: Option<Locale>) -> i32 {\n    let arguments = match parse(argv, &[\"root\", \"snapshot\"]) {","pub fn commands(root: &Path, argv: &[String], locale: Option<Locale>, voice: &Voice) -> i32 {\n    let arguments = match parse(argv, &[\"root\", \"snapshot\"], voice) {")
sub('''        if registry.graph_unpublished() { eprintln!("{}", match locale { Locale::En => "build graph publication absent: targets inferred by the graph are not checked", Locale::De => "Build-Graph-Veröffentlichung fehlt: vom Graphen abgeleitete Targets werden nicht geprüft" }); }
        println!("{} commands, {} problems", registry.entries().len(), problems.len());''','''        if registry.graph_unpublished() { eprintln!("{}", voice.say(|text| text.cli_graph_absent.as_str().to_string())); }
        println!("{}", voice.say(|text| text.cli_check_summary.fill(&[("commands", &registry.entries().len().to_string()), ("problems", &problems.len().to_string())]).into_string()));''')
sub('''fn connect(root: &Path) -> Result<Connection, String> {
    Connection::connect(&ipc::canonical_path(root)).map_err(|error| format!("no dashboard daemon answers for this workspace ({error}); `repo dashboard run` starts one"))
}

fn connect_or_start(root: &Path) -> Result<Connection, String> {
    let executable = std::env::current_exe().map_err(|error| error.to_string())?;
    dashboard_daemon::lifecycle::connect_or_start(&ipc::canonical_path(root), &executable).map_err(|error| format!("cannot reach the dashboard daemon: {error}"))
}''','''fn connect(root: &Path, voice: &Voice) -> Result<Connection, String> {
    Connection::connect(&ipc::canonical_path(root)).map_err(|error| voice.say(|text| text.cli_no_daemon.fill(&[("error", &error.to_string())]).into_string()))
}

fn connect_or_start(root: &Path, voice: &Voice) -> Result<Connection, String> {
    let executable = std::env::current_exe().map_err(|error| error.to_string())?;
    dashboard_daemon::lifecycle::connect_or_start(&ipc::canonical_path(root), &executable).map_err(|error| voice.say(|text| text.cli_daemon_unreachable.fill(&[("error", &error.to_string())]).into_string()))
}''')
sub('''fn sessions_of(root: &Path) -> Vec<SessionInfo> {
    match connect(root).and_then(''','''fn sessions_of(root: &Path, voice: &Voice) -> Vec<SessionInfo> {
    match connect(root, voice).and_then(''')
sub("pub fn pick<'a>(sessions: &'a [SessionInfo], needle: &str) -> Result<Picked<'a>, String> {\n    if needle.is_empty() { return Err(\"a task handle is empty; `repo dashboard tasks` lists them\".into()); }",
    "pub fn pick<'a>(sessions: &'a [SessionInfo], needle: &str, voice: &Voice) -> Result<Picked<'a>, String> {\n    if needle.is_empty() { return Err(voice.say(|text| text.cli_handle_empty.as_str().to_string())); }")
sub('map_or_else(|| Err(format!("no task matches {needle:?}; `repo dashboard tasks` lists them")), |latest|','map_or_else(|| Err(voice.say(|text| text.cli_no_task_matches.fill(&[("needle", needle)]).into_string())), |latest|')
sub('_ => Err(format!("{needle:?} has {} live tasks: {}; name one session id", live.len(), listed(&live))),','_ => Err(voice.say(|text| text.cli_live_tasks.fill(&[("needle", needle), ("count", &live.len().to_string()), ("listed", &listed(&live))]).into_string())),')
sub('_ => return Err(format!("{needle:?} matches {} tasks: {}; name one session id", by_prefix.len(), listed(&by_prefix))),','_ => return Err(voice.say(|text| text.cli_matches_tasks.fill(&[("needle", needle), ("count", &by_prefix.len().to_string()), ("listed", &listed(&by_prefix))]).into_string())),')
sub('        0 => Err(format!("no task matches {needle:?}; `repo dashboard tasks` lists them")),\n        1 => of_command(by_part),\n        _ => Err(format!("{needle:?} matches several commands: {}; name one", commands.join(", "))),','        0 => Err(voice.say(|text| text.cli_no_task_matches.fill(&[("needle", needle)]).into_string())),\n        1 => of_command(by_part),\n        _ => Err(voice.say(|text| text.cli_matches_commands.fill(&[("needle", needle), ("commands", &commands.join(", "))]).into_string())),')
sub("fn only<'a>(picked: &Picked<'a>, needle: &str) -> Result<&'a SessionInfo, String> {","fn only<'a>(picked: &Picked<'a>, needle: &str, voice: &Voice) -> Result<&'a SessionInfo, String> {")
sub('many => Err(format!("{needle:?} names {} tasks of one launch: {}; name one session id", many.len(), listed(many))),','many => Err(voice.say(|text| text.cli_names_tasks.fill(&[("needle", needle), ("count", &many.len().to_string()), ("listed", &listed(many))]).into_string())),')
sub('pub fn run(root: &Path, argv: &[String]) -> i32 {\n    let arguments = match parse(argv, &["param", "env", "root", "snapshot", "timeout"]) {','pub fn run(root: &Path, argv: &[String], voice: &Voice) -> i32 {\n    let arguments = match parse(argv, &["param", "env", "root", "snapshot", "timeout"], voice) {')
sub('else { return refuse("usage: repo dashboard run <id> [--param k=v]… [--env K=V]… [--detach] [--wait-ready] [--dry-run] [-- args…]") };','else { return refuse(&voice.say(|text| text.cli_usage_run.as_str().to_string())) };')
sub("let request = match arguments.request(&|name| accepted.iter().find(|parameter| parameter.id == name).map(|parameter| parameter.kind)) {","let request = match arguments.request(&|name| accepted.iter().find(|parameter| parameter.id == name).map(|parameter| parameter.kind), voice) {")
sub("let result = connect_or_start(&root).and_then(|mut connection| start(&mut connection, SpawnGroup::from_launch(&launch, &[]), &arguments));","let result = connect_or_start(&root, voice).and_then(|mut connection| start(&mut connection, SpawnGroup::from_launch(&launch, &[]), &arguments, voice));")
sub("fn start(connection: &mut Connection, group: SpawnGroup, arguments: &Arguments) -> Result<i32, String> {","fn start(connection: &mut Connection, group: SpawnGroup, arguments: &Arguments, voice: &Voice) -> Result<i32, String> {")
sub('eprintln!("[semio] the command declares no ready address; it is started");','eprintln!("[semio] {}", voice.say(|text| text.cli_no_ready_declared.as_str().to_string()));')
sub('pub fn tasks(root: &Path, argv: &[String]) -> i32 {\n    let arguments = match parse(argv, &["root"]) { Ok(arguments) => arguments, Err(code) => return code };\n    print_sessions(&sessions_of(&root_of(root, &arguments)), arguments.has("json"));','pub fn tasks(root: &Path, argv: &[String], voice: &Voice) -> i32 {\n    let arguments = match parse(argv, &["root"], voice) { Ok(arguments) => arguments, Err(code) => return code };\n    print_sessions(&sessions_of(&root_of(root, &arguments), voice), arguments.has("json"));')
sub('pub fn logs(root: &Path, argv: &[String]) -> i32 {\n    let arguments = match parse(argv, &["root"]) { Ok(arguments) => arguments, Err(code) => return code };\n    let [needle] = arguments.words.as_slice() else { return refuse("usage: repo dashboard logs <task> [--follow]") };','pub fn logs(root: &Path, argv: &[String], voice: &Voice) -> i32 {\n    let arguments = match parse(argv, &["root"], voice) { Ok(arguments) => arguments, Err(code) => return code };\n    let [needle] = arguments.words.as_slice() else { return refuse(&voice.say(|text| text.cli_usage_logs.as_str().to_string())) };')
sub("    let sessions = sessions_of(&root);\n    let session = match pick(&sessions, needle).and_then(|picked| only(&picked, needle)) { Ok(session) => session, Err(error) => return refuse(&error) };","    let sessions = sessions_of(&root, voice);\n    let session = match pick(&sessions, needle, voice).and_then(|picked| only(&picked, needle, voice)) { Ok(session) => session, Err(error) => return refuse(&error) };")
sub("    let result = match connect(&root) {\n        Ok(mut connection) => daemon_control::follow(","    let result = match connect(&root, voice) {\n        Ok(mut connection) => daemon_control::follow(")
sub('pub fn act(root: &Path, verb: &str, argv: &[String]) -> i32 {\n    let arguments = match parse(argv, &["root", "timeout"]) { Ok(arguments) => arguments, Err(code) => return code };\n    let [needle] = arguments.words.as_slice() else { return refuse(&format!("usage: semio {verb} <task> [--group]")) };','pub fn act(root: &Path, verb: &str, argv: &[String], voice: &Voice) -> i32 {\n    let arguments = match parse(argv, &["root", "timeout"], voice) { Ok(arguments) => arguments, Err(code) => return code };\n    let [needle] = arguments.words.as_slice() else { return refuse(&voice.say(|text| text.cli_usage_act.fill(&[("verb", verb)]).into_string())) };')
sub("        let mut connection = connect(&root)?;\n        let sessions = daemon_control::tasks(&mut connection, QUERY).map_err(|error| error.to_string())?;\n        let picked = pick(&sessions, needle)?;","        let mut connection = connect(&root, voice)?;\n        let sessions = daemon_control::tasks(&mut connection, QUERY).map_err(|error| error.to_string())?;\n        let picked = pick(&sessions, needle, voice)?;")
sub('.ok_or_else(|| format!("{needle:?} belongs to no compound"))?)','.ok_or_else(|| voice.say(|text| text.cli_no_compound.fill(&[("needle", needle)]).into_string()))?)')
sub('_ => return Err("--group stops or kills; it does not restart".into()) })','_ => return Err(voice.say(|text| text.cli_group_not_restart.as_str().to_string())) })')
sub('return Err(format!("{needle:?} has no live task to {verb}; {}", listed(&picked.sessions)));','return Err(voice.say(|text| text.cli_no_live_task.fill(&[("needle", needle), ("verb", verb), ("listed", &listed(&picked.sessions))]).into_string()));')
sub('pub fn open(root: &Path, argv: &[String]) -> i32 {\n    let arguments = match parse(argv, &["root"]) { Ok(arguments) => arguments, Err(code) => return code };\n    let [needle] = arguments.words.as_slice() else { return refuse("usage: repo dashboard open <task> [--print]") };\n    let sessions = sessions_of(&root_of(root, &arguments));\n    let session = match pick(&sessions, needle).and_then(','pub fn open(root: &Path, argv: &[String], voice: &Voice) -> i32 {\n    let arguments = match parse(argv, &["root"], voice) { Ok(arguments) => arguments, Err(code) => return code };\n    let [needle] = arguments.words.as_slice() else { return refuse(&voice.say(|text| text.cli_usage_open.as_str().to_string())) };\n    let sessions = sessions_of(&root_of(root, &arguments), voice);\n    let session = match pick(&sessions, needle, voice).and_then(')
sub("only(&Picked { sessions: if ready.len() == 1 { ready } else { picked.sessions }, group: None }, needle) }) {","only(&Picked { sessions: if ready.len() == 1 { ready } else { picked.sessions }, group: None }, needle, voice) }) {")
sub('else { return refuse(&format!("{} has no ready address", label_text(session))) };','else { return refuse(&voice.say(|text| text.cli_no_ready_address.fill(&[("task", &label_text(session))]).into_string())) };')
sub('Err(error) => refuse(&format!("cannot open a browser: {error}")) }','Err(error) => refuse(&voice.say(|text| text.cli_cannot_open_browser.fill(&[("error", &error.to_string())]).into_string())) }')
open(p,"w",encoding="utf8").write(s)
print("patched")
