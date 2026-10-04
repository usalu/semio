"""🔌️ Wires `history_edit_acceptance_law!` (S3-AGNOSTIC, design §16.3) into every editor subset of every plugin crate that does not run
it yet: one line appended to the subset's `✏️editor/🧪️tests/🔬️unit/🦀️.rs` (a child module of the editor module, so `super::` names the
editor and its `create_*` definition), fixtures rooted at the subset; the crate's dev-dependency on `semio-framework-plugin` gains the
`artifact-app-testing` feature when it lacks it. Dry run by default (prints the plan); `--apply` writes, re-reading each file right before
writing. Usage: python3 🧪️s3-agnostic-wire-acceptance-all.py [--apply] [crate-name…]"""
import glob, os, re, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
APPLY = "--apply" in sys.argv
ONLY = {arg for arg in sys.argv[1:] if not arg.startswith("--")}
os.chdir(ROOT)


def plugin_name(artifact):
    return re.sub(r"^[^a-z0-9]+", "", artifact.split("/")[2])


def wired(artifact):
    found = subprocess.run(["/usr/bin/grep", "-rl", "--include=*.rs", "history_edit_acceptance_law!", artifact], capture_output=True, text=True).stdout
    return found.strip() != ""


def subsets(artifact):
    for editor in sorted(glob.glob(f"{artifact}/🏅️standards/*/🪆️subsets/*/✏️editor/🦀️.rs")):
        source = open(editor, encoding="utf-8").read()
        implementor = re.search(r"impl (?:semio_framework_plugin::)?ArtifactEditor for (\w+)", source)
        definition = re.search(r"pub (async )?fn (create_\w+?_(?:editor|app))\(\) -> (?:semio_framework_plugin::)?AppDefinition", source)
        unit = editor.replace("✏️editor/🦀️.rs", "✏️editor/🧪️tests/🔬️unit/🦀️.rs")
        mounted = '#[path = "🧪️tests/🔬️unit/🦀️.rs"]' in source and os.path.isfile(unit)
        yield editor, implementor and implementor.group(1), definition and (definition.group(2), bool(definition.group(1))), unit if mounted else None


def with_feature(spec):
    if "features" in spec:
        return re.sub(r'features\s*=\s*\[', 'features = ["artifact-app-testing", ', spec, count=1)
    return spec.rstrip()[:-1].rstrip().rstrip(",") + ', features = ["artifact-app-testing"] }'


def dev_feature(cargo):
    """The crate manifest with the `artifact-app-testing` dev feature, or `None` when it already has it."""
    text = open(cargo, encoding="utf-8").read()
    head, marker, dev = text.partition("[dev-dependencies]")
    table, rest = (dev.split("\n[", 1) + [""])[:2] if marker else ("", "")
    entry = re.search(r'^semio-framework-plugin\s*=\s*(\{.*\})\s*$', table, re.M)
    if entry and "artifact-app-testing" in entry.group(1):
        return None
    if entry:
        table = table.replace(entry.group(0), f"semio-framework-plugin = {with_feature(entry.group(1))}", 1)
        return head + marker + table + ("\n[" + rest if rest else "")
    regular = re.search(r'^semio-framework-plugin\s*=\s*(\{.*\})\s*$', head, re.M)
    line = f"semio-framework-plugin = {with_feature(regular.group(1) if regular else '{ workspace = true }')}\n"
    if marker:
        return head + marker + "\n" + line + table.lstrip("\n") + ("\n[" + rest if rest else "")
    return text.rstrip("\n") + "\n\n[dev-dependencies]\n" + line


plans = []
for cargo in sorted(glob.glob("✏️s/🔌️plugins/*/🗿️artifacts/*/📦️packages/🦀️rust/Cargo.toml")):
    artifact = cargo.split("/📦️packages")[0]
    crate = re.search(r'^name = "([^"]+)"', open(cargo, encoding="utf-8").read(), re.M).group(1)
    if (ONLY and crate not in ONLY) or wired(artifact):
        continue
    lines = []
    for editor, implementor, definition, unit in subsets(artifact):
        if not (implementor and definition and unit):
            print(f"skip {crate} {editor.split('/🏅️standards/')[1]}: editor={implementor} definition={definition} unit={unit is not None}")
            continue
        name, is_async = definition
        build = f"semio_framework_plugin::app::history_edit_acceptance::block_on_acceptance(super::{name}())" if is_async else f"super::{name}()"
        fixtures = "../../" + editor.split(artifact + "/")[1].replace("/✏️editor/🦀️.rs", "")
        lines.append((unit, f'semio_framework_plugin::history_edit_acceptance_law!("{plugin_name(artifact)}", super::{implementor}, || semio_framework_plugin::App {{ definition: {build}, examples: Vec::new() }}, "{fixtures}");'))
    if lines:
        plans.append((crate, cargo, dev_feature(cargo), lines))

for crate, cargo, feature, lines in plans:
    print(f"{crate}: dev-feature={'add' if feature else 'ok'} subsets={len(lines)}")
    for unit, line in lines:
        print(f"  {unit.split('/🏅️standards/')[1]}  {line}")
    if not APPLY:
        continue
    if feature is not None:
        updated = dev_feature(cargo)
        open(cargo, "w", encoding="utf-8").write(updated)
    for unit, line in lines:
        text = open(unit, encoding="utf-8").read()
        if "history_edit_acceptance_law!" not in text:
            open(unit, "w", encoding="utf-8").write(text.rstrip("\n") + "\n\n" + line + "\n")
    print(f"  wired {crate}")
