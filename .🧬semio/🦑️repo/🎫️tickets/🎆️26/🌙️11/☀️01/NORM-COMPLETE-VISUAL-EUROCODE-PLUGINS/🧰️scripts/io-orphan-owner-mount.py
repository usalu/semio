import os, re, shutil, sys
from pathlib import Path

ROOT = Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm")
ARTIFACTS = ROOT / "🗿️artifacts"
TICKET = Path("/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS")
BACKUP = TICKET / "🗑️generated" / "io-orphan" / "backup"
DRY = "--dry" in sys.argv
TEST_DIR = "🧪️tests"
ANY = "🏅️standards/🔖️1/🪆️subsets/✳️any"
NOTES = []


def note(message):
    NOTES.append(message)
    print(message)


def read(path):
    return path.read_text(encoding="utf-8")


def write(path, text):
    if DRY:
        return
    rel = path.relative_to("/")
    target = BACKUP / rel
    if path.exists() and not target.exists():
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(path, target)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def rs_files(directory, with_tests):
    for path in sorted(directory.rglob("*.rs")):
        if not with_tests and TEST_DIR in path.relative_to(directory).parts:
            continue
        yield path


def cargo_sections(text):
    sections, current = {}, None
    for line in text.splitlines():
        header = re.match(r"^\[(.+)\]\s*$", line)
        if header:
            current = header.group(1)
            sections.setdefault(current, [])
        elif current:
            sections[current].append(line)
    return sections


def dependency_names(text, section):
    names = set()
    for line in cargo_sections(text).get(section, []):
        match = re.match(r"^([A-Za-z0-9_-]+)\s*=", line)
        if match:
            names.add(match.group(1))
    return names


def dependency_line(name, sources, workspace_text, dest_dir):
    for source_text, source_dir in sources:
        for section in ("dependencies", "dev-dependencies"):
            for line in cargo_sections(source_text).get(section, []):
                if re.match(rf"^{re.escape(name)}\s*=", line):
                    path = re.search(r'path = "([^"]+)"', line)
                    if path:
                        absolute = os.path.normpath(os.path.join(source_dir, path.group(1)))
                        relative = os.path.relpath(absolute, dest_dir)
                        return line.replace(path.group(1), relative)
                    if "features" in line:
                        return f"{name} = {{ workspace = true }}"
                    return line
    if re.search(rf"^\[workspace\.dependencies\.{re.escape(name)}\]", workspace_text, re.M) or re.search(rf"^{re.escape(name)}\s*=", workspace_text, re.M):
        return f"{name} = {{ workspace = true }}"
    return None


def add_dependencies(cargo_path, wanted, dev_wanted, sources, workspace_text):
    text = read(cargo_path)
    deps = dependency_names(text, "dependencies")
    dev = dependency_names(text, "dev-dependencies")
    additions, dev_additions = [], []
    for name in sorted(wanted):
        if name not in deps:
            line = dependency_line(name, sources, workspace_text, cargo_path.parent)
            if line:
                additions.append((name, line))
            else:
                note(f"WARN no dependency spec for {name} in {cargo_path}")
    for name in sorted(dev_wanted - wanted):
        if name not in deps and name not in dev:
            line = dependency_line(name, sources, workspace_text, cargo_path.parent)
            if line:
                dev_additions.append((name, line))
            else:
                note(f"WARN no dev dependency spec for {name} in {cargo_path}")

    def insert(text, section, items):
        if not items:
            return text
        block = "\n".join(line for _, line in items) + "\n"
        if f"[{section}]" not in text:
            return text.rstrip("\n") + f"\n\n[{section}]\n" + block
        lines = text.splitlines(keepends=True)
        out, inside, done = [], False, False
        for index, line in enumerate(lines):
            header = re.match(r"^\[(.+)\]\s*$", line)
            if header and inside and not done:
                while out and out[-1].strip() == "":
                    out.pop()
                out.append(block + "\n")
                done = True
            if header:
                inside = header.group(1) == section
            out.append(line)
        if inside and not done:
            if out and not out[-1].endswith("\n"):
                out[-1] += "\n"
            out.append(block)
        return "".join(out)

    text = insert(text, "dependencies", additions)
    text = insert(text, "dev-dependencies", dev_additions)
    if additions or dev_additions:
        write(cargo_path, text)
    return [name for name, _ in additions + dev_additions]


def patch_lock(lock_path, package, names):
    if not names or not lock_path.exists():
        return
    text = read(lock_path)
    match = re.search(rf'(\[\[package\]\]\nname = "{re.escape(package)}"\nversion = "[^"]*"\n(?:source = "[^"]*"\n)?dependencies = \[\n)(.*?)(\]\n)', text, re.S)
    if not match:
        note(f"WARN lock entry missing for {package}")
        return
    entries = [item.strip().strip(",").strip('"') for item in match.group(2).splitlines() if item.strip()]
    lock_names = set(re.findall(r'^name = "([^"]+)"', text, re.M))
    for name in (("semio-framework-pack" if n == "pack" else n) for n in names):
        if name not in lock_names:
            note(f"WARN lock has no package {name} (cargo will resolve)")
        elif name not in entries:
            entries.append(name)
    body = "".join(f' "{entry}",\n' for entry in sorted(entries))
    write(lock_path, text[: match.start(2)] + body + text[match.end(2):])


def used_crates(files):
    crates, test_crates = set(), set()
    for path in files:
        if not path.exists():
            continue
        content = re.sub(r"//[^\n]*", "", read(path))
        found = set(re.findall(r"\b(semio_[a-z0-9_]+)\s*::", content))
        if re.search(r"\bpack\s*::", content):
            found.add("pack")
        (test_crates if TEST_DIR in path.parts else crates).update(found)
    return crates, test_crates


def package_name(ident):
    return ident.replace("_", "-")


def process(artifact):
    slug = re.search(r"(en\d{4})$", artifact.name).group(1)
    cap = slug.capitalize()
    base = artifact / ANY
    io = base / "🚪️io"
    text_dir, binary_dir, sqlite_dir = io / "📝️text", io / "💾️binary", io / "🪶️sqlite"
    schema = base / "🧬️schema"
    model_lib, mutations_lib = schema / "📸️snapshot" / "🦀️.rs", schema / "🧬️mutations" / "🦀️.rs"
    model_cargo = schema / "📸️snapshot/📦️packages/🦀️rust/Cargo.toml"
    mutations_cargo = schema / "🧬️mutations/📦️packages/🦀️rust/Cargo.toml"
    model_ident, mutations_ident = f"semio_s_artifact_norm_{slug}_model", f"semio_s_artifact_norm_{slug}_mutations"
    model_text, mutations_text = read(model_lib), read(mutations_lib)
    if "🚪️io/🪞️records/🦀️.rs" in model_text:
        note(f"{slug}: already mounted")
        return
    for alias in ("store", "protocol"):
        for label, lib_text in (("model", model_text), ("mutations", mutations_text)):
            if not re.search(rf"^extern crate [a-z_]+ as {alias};", lib_text, re.M):
                note(f"WARN {slug}: {label} lib lacks extern alias {alias}")
    if not re.search(rf"\b(struct|type) {cap}Snapshot\b|pub use .*{cap}Snapshot", model_text):
        note(f"WARN {slug}: {cap}Snapshot not visibly defined at the model root")

    text_snapshot, binary_snapshot = text_dir / "📸️snapshot/🦀️.rs", binary_dir / "📸️snapshot/🦀️.rs"
    text_source, binary_source = read(text_snapshot), read(binary_snapshot)
    text_macro = re.search(rf'^semio_s_plugin_norm_compliance::impl_norm_artifact_record!\(@text {cap}Snapshot, extension="([^"]+)", envelope_id="([^"]+)"\);[ \t]*\n?', text_source, re.M)
    binary_macro = re.search(rf'^semio_s_plugin_norm_compliance::impl_norm_artifact_record!\(@binary {cap}Snapshot, extension="([^"]+)", envelope_id="([^"]+)"(?:, sqlite=([A-Za-z0-9_:]+))?\);[ \t]*\n?', binary_source, re.M)
    if not text_macro or not binary_macro or text_macro.groups() != binary_macro.groups()[:2]:
        note(f"ERROR {slug}: record macros not found or inconsistent")
        return
    extension, envelope, sqlite = binary_macro.groups()
    sqlite_arg = f", sqlite={sqlite}" if sqlite else ""
    records = f'//! 🪞️ {cap} snapshot record impls, owned by the crate that owns `{cap}Snapshot` so the orphan rule holds.\n\nuse crate::{cap}Snapshot;\n\nsemio_s_plugin_norm_compliance::impl_norm_artifact_record!({cap}Snapshot, extension="{extension}", envelope_id="{envelope}"{sqlite_arg});\n'
    write(text_snapshot, text_source[: text_macro.start()].rstrip("\n") + "\n" + text_source[text_macro.end():])
    write(binary_snapshot, binary_source[: binary_macro.start()].rstrip("\n") + "\n" + binary_source[binary_macro.end():])
    records_file = io / "🪞️records/🦀️.rs"
    write(records_file, records)

    sqlite_files = list(rs_files(sqlite_dir, False))
    for path in sqlite_files:
        source = read(path)
        for other in ("mutations", "diff", "io", "inferences"):
            if re.search(rf"semio_s_artifact_norm_{slug}_{other}\b", source):
                note(f"WARN {slug}: {path.name} references {other}")
        for match in set(re.findall(r"crate::(?!sqlite::)[A-Za-z_]+", source)):
            note(f"WARN {slug}: sqlite file uses {match} in {path.relative_to(base)}")
        write(path, source.replace(f"{model_ident}::", "crate::"))
    write(model_lib, model_text.rstrip("\n") + '\n\n#[path = "../../🚪️io/🪶️sqlite/🦀️.rs"]\npub mod sqlite;\n\n#[path = "../../🚪️io/🪞️records/🦀️.rs"]\nmod records;\n')

    for tree, name, other in ((text_dir / "🧬️mutations", "text", "binary"), (binary_dir / "🧬️mutations", "binary", "text")):
        for path in rs_files(tree, True):
            source = read(path)
            rewritten = source.replace(f"{mutations_ident}::", "crate::")
            rewritten = rewritten.replace("crate::text::mutations", "crate::text").replace("crate::binary::mutations", "crate::binary")
            rewritten = re.sub(r"crate::mutations\b", f"crate::{name}", rewritten)
            if re.search(rf"semio_s_artifact_norm_{slug}_io\w*", rewritten):
                note(f"WARN {slug}: {path.relative_to(base)} references an io crate")
            write(path, rewritten)
    write(mutations_lib, mutations_text.rstrip("\n") + '\n\n#[path = "../../🚪️io/📝️text/🧬️mutations/🦀️.rs"]\npub mod text;\n\n#[path = "../../🚪️io/💾️binary/🧬️mutations/🦀️.rs"]\npub mod binary;\n')

    mount = re.compile(r'#\[path = "🧬️mutations/🦀️.rs"\]\npub mod mutations;\n')
    for facet_lib, facet in ((text_dir / "🦀️.rs", "text"), (binary_dir / "🦀️.rs", "binary")):
        source = read(facet_lib)
        if not mount.search(source):
            note(f"ERROR {slug}: {facet} mutations mount not found")
            continue
        write(facet_lib, mount.sub(f"pub use {mutations_ident}::{facet} as mutations;\n", source))
    if sqlite_dir.exists():
        io_lib = io / "🦀️.rs"
        source = read(io_lib)
        pattern = re.compile(r'#\[path = "🪶️sqlite/🦀️.rs"\]\npub mod sqlite;\n')
        if pattern.search(source):
            write(io_lib, pattern.sub(f"pub use {model_ident}::sqlite;\n", source))
        else:
            note(f"ERROR {slug}: io sqlite mount not found")

    workspace_text = read(artifact / "Cargo.toml")
    sources = []
    for cargo in (io / "📦️packages/🦀️rust/Cargo.toml", text_dir / "📦️packages/🦀️rust/Cargo.toml"):
        if cargo.exists():
            sources.append((read(cargo), str(cargo.parent)))
    model_used, model_test = used_crates(sqlite_files + [records_file])
    model_used |= {"pack", "semio_framework_dsl_record", "semio_framework_os_kernel", "semio_s_plugin_norm_compliance"}
    mutations_files = list(rs_files(text_dir / "🧬️mutations", True)) + list(rs_files(binary_dir / "🧬️mutations", True))
    mutations_used, mutations_test = used_crates(mutations_files)
    lock = artifact / "Cargo.lock"
    for cargo, used, tested, own, package in (
        (model_cargo, model_used, model_test, {model_ident}, f"semio-s-artifact-norm-{slug}-model"),
        (mutations_cargo, mutations_used, mutations_test, {mutations_ident}, f"semio-s-artifact-norm-{slug}-mutations"),
    ):
        wanted = {package_name(i) if i != "pack" else "pack" for i in used - own}
        dev_wanted = {package_name(i) if i != "pack" else "pack" for i in tested - own}
        added = add_dependencies(cargo, wanted, dev_wanted, sources, workspace_text)
        patch_lock(lock, package, added)
        note(f"{slug}: {package} deps added: {', '.join(added) or 'none'}")
    note(f"{slug}: mounted")


def main():
    for artifact in sorted(ARTIFACTS.iterdir()):
        if re.search(r"en199\d$", artifact.name):
            process(artifact)
    if not DRY:
        log = TICKET / "🗑️generated" / "io-orphan" / "script.log"
        log.parent.mkdir(parents=True, exist_ok=True)
        log.write_text("\n".join(NOTES) + "\n", encoding="utf-8")


main()
