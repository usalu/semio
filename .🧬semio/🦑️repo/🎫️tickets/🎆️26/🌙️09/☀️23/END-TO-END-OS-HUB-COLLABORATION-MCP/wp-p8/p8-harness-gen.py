"""🏗️ Generates the ticket-local declared-verb probe harness (`wp-p8/probe-harness`): one standalone cargo crate (own
`[workspace]`, path deps only, nothing in the tree touched) that runs `artifact_app_laws::probe_declared_verbs` over every
editor and viewer surface `p8-surfaces.py` found and prints one JSON line per surface with its probes' findings and
agent-lane divergences. Usage: p8-harness-gen.py <surfaces.json> [--only crate,crate]"""
import json, os, re, sys
ROOT = "/Users/ueli/Documents/semio"
DEPS_ROOT = os.environ.get("P8_DEPS_ROOT", ROOT)
HARNESS = os.path.join(ROOT, ".tmp-ticket/wp-p8/probe-harness")
rows = json.load(open(sys.argv[1], encoding="utf-8"))
only = None
if "--only" in sys.argv:
    only = set(sys.argv[sys.argv.index("--only") + 1].split(","))

def crate_dir_of(file):
    d = os.path.join(ROOT, file)
    while not os.path.isdir(os.path.join(d, "📦️packages")):
        d = os.path.dirname(d)
    return d

def module_path(row):
    d = crate_dir_of(row["file"])
    root_src = open(os.path.join(d, "🦀️.rs"), encoding="utf-8").read()
    rel = os.path.relpath(os.path.join(ROOT, row["file"]), d)
    needle = f'#[path = "{rel}"]'
    i = root_src.find(needle)
    if i < 0:
        return None
    before = root_src[:i]
    inner = re.findall(r"pub mod (\w+) \{", before)
    stack = []
    for m in re.finditer(r"pub mod (\w+) \{|\{|\}", before):
        tok = m.group(0)
        if tok.startswith("pub mod"):
            stack.append(m.group(1))
        elif tok == "{":
            stack.append(None)
        else:
            if stack:
                stack.pop()
    names = [n for n in stack if n]
    return names

entries, deps, skipped = [], {}, []
for row in rows:
    if only and row["crate"] not in only:
        continue
    names = module_path(row)
    if not names or not row["create"]:
        skipped.append((row["type"], row["file"]))
        continue
    ident = row["crate"].replace("-", "_")
    path = "::".join([ident] + names)
    trait = "ArtifactEditor" if row["role"] == "Editor" else "ArtifactViewer"
    wrapper = "EditorApp" if row["role"] == "Editor" else "ViewerApp"
    examples = os.path.join(os.path.dirname(os.path.join(DEPS_ROOT, row["file"])), "🧫️fixtures", "⚖️declared-verb-examples.json")
    entries.append((row["crate"], row["role"], f"{path}::{row['type']}", f"{path}::{row['create'][0]}", trait, wrapper, examples if os.path.exists(examples) else None))
    d = crate_dir_of(row["file"])
    cargo = open(os.path.join(d, "📦️packages/🦀️rust/Cargo.toml"), encoding="utf-8").read()
    feats = ["component-app-assembly"] if re.search(r"^component-app-assembly\s*=", cargo, re.M) else []
    deps[row["crate"]] = (os.path.join(d, "📦️packages/🦀️rust").replace(ROOT, DEPS_ROOT, 1), feats)

os.makedirs(HARNESS, exist_ok=True)
lines = ["[workspace]", "", "[package]", 'name = "p8-probe-harness"', 'version = "0.0.0"', 'edition = "2021"', "publish = false", "", "[[bin]]", 'name = "p8-probe-harness"', 'path = "main.rs"', "", "[dependencies]",
         'semio-framework-plugin = { path = "' + DEPS_ROOT + '/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust", features = ["component-guest", "artifact-app-testing"] }',
         'semio-framework-async = { path = "' + DEPS_ROOT + '/🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust" }']
for crate, (path, feats) in sorted(deps.items()):
    f = f', features = {json.dumps(feats)}' if feats else ""
    lines.append(f'{crate} = {{ path = "{path}"{f} }}')
open(os.path.join(HARNESS, "Cargo.toml"), "w", encoding="utf-8").write("\n".join(lines) + "\n")

body = ['//! 🧪️ Ticket-local declared-verb probe harness (P8): runs the framework declared-verb law over every surface and prints',
        '//! one JSON line per surface — findings and agent-lane divergences — so the whole fleet is measured without touching the tree.',
        'use semio_framework_plugin::artifact_app_laws::{declared_verb_agent_divergences, declared_verb_findings, probe_declared_verbs};',
        '',
        'fn report(label: &str, probes: Vec<semio_framework_plugin::artifact_app_laws::DeclaredVerbProbe>) {',
        '    let findings: Vec<String> = probes.iter().flat_map(declared_verb_findings).map(|finding| finding.to_string()).collect();',
        '    let agent = declared_verb_agent_divergences(&probes);',
        '    if std::env::var_os("P8_DUMP").is_some() {',
        '        for probe in probes.iter().filter(|probe| !declared_verb_findings(probe).is_empty()) {',
        '            eprintln!("{label} {probe:#?}");',
        '        }',
        '    }',
        '    let quote = |text: &str| format!("\\"{}\\"", text.replace(\'\\\\\', "\\\\\\\\").replace(\'"\', "\\\\\\"").replace(\'\\n\', " "));',
        '    println!("{{\\"surface\\":{},\\"verbs\\":{},\\"findings\\":[{}],\\"agentDivergences\\":[{}]}}", quote(label), probes.len(), findings.iter().map(|f| quote(f)).collect::<Vec<_>>().join(","), agent.iter().map(|a| quote(a)).collect::<Vec<_>>().join(","));',
        '}',
        '',
        'fn run(filter: Option<String>) {']
for crate, role, ty, create, trait, wrapper, examples in entries:
    label = f"{crate}::{ty.split('::')[-1]}"
    examples_arg = f'Some(include_str!({json.dumps(examples, ensure_ascii=False)}))' if examples else "None"
    body.append(f'    if filter.as_deref().is_none_or(|filter| "{label}".contains(filter)) {{')
    body.append(f'        eprintln!("probing {label}");')
    body.append(f'        let run = std::thread::Builder::new().stack_size(512 * 1024 * 1024).spawn(|| semio_framework_async::block_on(Box::pin(probe_declared_verbs::<semio_framework_plugin::{wrapper}<{ty}>, <{ty} as semio_framework_plugin::{trait}>::Members>({create}, {examples_arg})))).expect("probe thread").join();')
    body.append(f'        match run {{ Ok(probes) => report("{label}", probes), Err(panic) => println!("{{{{\\"surface\\":\\"{label}\\",\\"panic\\":{{:?}}}}}}", panic.downcast_ref::<String>().cloned().or_else(|| panic.downcast_ref::<&str>().map(|text| text.to_string())).unwrap_or_default()) }}')
    body.append('    }')
body += ['}', '', 'fn main() {',
         '    let filter = std::env::args().nth(1);',
         '    run(filter);',
         '}']
open(os.path.join(HARNESS, "main.rs"), "w", encoding="utf-8").write("\n".join(body) + "\n")
print(len(entries), "entries,", len(deps), "crates, skipped", len(skipped))
for s in skipped[:10]: print(" skip", s)
