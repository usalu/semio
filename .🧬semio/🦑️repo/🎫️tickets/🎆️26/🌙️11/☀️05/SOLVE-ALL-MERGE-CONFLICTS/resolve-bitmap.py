import subprocess
from pathlib import Path

root = Path(r"c:\git\semio")
files = subprocess.check_output(["git", "diff", "--name-only", "--diff-filter=U"], cwd=root, text=True, encoding="utf-8").splitlines()
bitmap = [root / f for f in files if "bitmap" in f and f.endswith(".rs")]
A, B, C = "<<<<<<< Updated upstream\n", "=======\n", ">>>>>>> Stashed changes\n"

def split_hunks(text):
    out = []
    rest = text
    while True:
        i = rest.find(A)
        if i < 0:
            out.append(rest)
            return out
        j = rest.find(B, i)
        k = rest.find(C, j)
        if j < 0 or k < 0:
            raise SystemExit("broken markers")
        out.append(rest[:i])
        out.append((rest[i + len(A):j], rest[j + len(B):k]))
        rest = rest[k + len(C):]

def join(parts, choices):
    text = []
    choice = 0
    for part in parts:
        if isinstance(part, str):
            text.append(part)
        else:
            text.append(choices[choice])
            choice += 1
    if choice != len(choices):
        raise SystemExit(f"choice count {choice} != {len(choices)}")
    return "".join(text)

def output_tests(upstream):
    text = upstream
    text = text.replace(
        "render(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default(), None)",
        "render(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default(), None, None)",
    )
    text = text.replace(
        "render(&snapshot, &stale, &BitmapOutputWindowConfig::default(), None)",
        "render(&snapshot, &stale, &BitmapOutputWindowConfig::default(), None, None)",
    )
    text = text.replace(
        "render_layers_fingerprint(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default(), None)",
        "render_layers_fingerprint(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default(), None, None)",
    )
    text = text.replace(
        "render_layers_fingerprint(&snapshot, &contradiction, &BitmapOutputWindowConfig::default(), None)",
        "render_layers_fingerprint(&snapshot, &contradiction, &BitmapOutputWindowConfig::default(), None, None)",
    )
    text = text.replace(
        "render_layers_fingerprint(&snapshot, &finished, &BitmapOutputWindowConfig::default(), None)",
        "render_layers_fingerprint(&snapshot, &finished, &BitmapOutputWindowConfig::default(), None, None)",
    )
    text = text.replace(
        "render_layers_fingerprint(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default(), Some(&run))",
        "render_layers_fingerprint(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default(), Some(&run), None)",
    )
    return text

def picker(stash):
    body = stash.split("\n", 1)[1]
    return body.replace("semio_framework_plugin::HistoryView", "semio_plugin::HistoryView")

for path in bitmap:
    text = path.read_text(encoding="utf-8")
    parts = split_hunks(text)
    key = str(path)
    if "fill" in key and "tests" in key:
        choices = ["use semio_framework_job::Generation;\nuse semio_plugin::ToolRunJobPort;\n"]
    elif "output" in key and "tests" in key:
        choices = [output_tests(parts[1][0])]
    elif key.endswith("artifact-editor\\🦀️.rs") or key.endswith("artifact-editor/🦀️.rs"):
        choices = [
            "        use semio_ui_locale::LocalizedLabel;\n        static NOTICES: std::sync::LazyLock<[(&str, LocalizedLabel); 16]> = std::sync::LazyLock::new(|| {\n",
            "        semio_plugin::retained_command::producer::admit_retained_tool_producer(request, output, cx, bitmap_command_id, BITMAP_RETAINED_RAW_BYTES, BITMAP_RETAINED_WORK_ITEMS, false, |request| BitmapCommandWork::new(tool_id))\n",
            "    fn render(body_key: &str, doc: &ArtifactView<'_, Self::Snapshot>, cfg: &ConfigView<'_, Self::Config>, _view_state: &semio_plugin::ViewModel) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {\n        Self::render_bodies(body_key, doc, cfg, &BitmapTransient::default(), None)\n",
            "        _interaction: &semio_plugin::app::InteractionView<'_>,\n        cx:&mut semio_plugin::StepContext<'_>,\n    ) -> semio_plugin::OriginalRenderResult<semio_plugin::ComponentTree> {\n        let rendered:semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree>=(||{\n        Self::render_bodies(body_key, doc, cfg, transient.snapshot, doc.tool_run())\n",
            "    fn render_bodies(body_key: &str, doc: &ArtifactView<'_, BitmapSnapshot>, cfg: &ConfigView<'_, NoConfig>, transient: &BitmapTransient, tool_run: Option<&semio_plugin::ToolRunView>) -> semio_plugin::UiAssemblyResult<semio_plugin::ComponentTree> {\n        match body_key {\n            input::BODY_KEY => input::render(doc.snapshot, &input::config::current(cfg)).map(semio_plugin::built_to_component_tree),\n            output::BODY_KEY => output::render(doc.snapshot, transient, &output::config::current(cfg), tool_run, doc.render_operation_ref().map(|operation| operation.canonical_base_revision)).map(semio_plugin::built_to_component_tree),\n            _ => semio_plugin::built_text_to_component_tree(Label::data(format!(\"Unknown body: {body_key}\"))),\n",
        ]
    elif "brush-tool" in key:
        choices = ["    let command = <BitmapEditor as semio_plugin::ArtifactEditor>::command_from_action(action, &mut Some((semio_framework_value::DslValue::from(&args)).clone()), &mut None).expect(\"the canvas verb bridges\");\n"]
    elif "editor" in key and "tests" in key and "unit" in key:
        choices = [parts[1][0] + "\n" + picker(parts[1][1])]
    else:
        raise SystemExit("unclassified " + key)
    resolved = join(parts, choices)
    if "<<<<<<<" in resolved or "\n=======\n" in resolved or ">>>>>>> Stashed changes" in resolved:
        raise SystemExit("markers remain " + key)
    path.write_text(resolved, encoding="utf-8", newline="\n")
    print("RESOLVED", path.name, "choices", len(choices))
