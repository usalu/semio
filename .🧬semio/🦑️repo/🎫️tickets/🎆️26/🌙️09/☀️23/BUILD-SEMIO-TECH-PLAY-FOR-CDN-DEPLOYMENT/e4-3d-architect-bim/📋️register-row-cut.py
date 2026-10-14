#!/usr/bin/env python3
"""📋️ E4 phase 2: replaces architect's 256 per-register create/delete/rename/replace leaves with the four generic
`📋️register-row` leaves typed by the generated register catalog. usage: 📋️register-row-cut.py <phase>
phases: tree (crate mounts, enum, kinds, leaf dirs), fixtures, callsites, registries, surfaces, hosts."""
import json, os, re, shutil, sys, hashlib

REPO = "/Users/ueli/Documents/semio"
PROGRAM = f"{REPO}/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program"
ANY_REL = "🏅️standards/🔖️1/🪆️subsets/✳️any"
ANY = f"{PROGRAM}/{ANY_REL}"
MUT = f"{ANY}/🧬️schema/🧬️mutations"
MUT_REPO = MUT[len(REPO) + 1:]
FIX = f"{ANY}/🧪️tests/🧫️fixtures/🧬️mutations"
ROOT_RS = f"{PROGRAM}/🦀️.rs"
CATALOG = json.load(open(f"{ANY}/🧬️schema/🗄️registers/🧬️schema/🔣️.json", encoding="utf-8"))["x-semio-collection-catalog"]["collections"]
BY_ENTITY = {entry["entity"].replace("-", "_"): entry for entry in CATALOG}
BY_VARIANT = {entry["variant"]: entry for entry in CATALOG}
OPS = {"create": "🌱️create", "delete": "🗑️delete", "rename": "🏷️rename", "replace": "♻️replace"}
REGISTERS_PATH = "crate::standards::v1::subsets::any::schema::registers"
BASE = "https://json.schemas.assets.semio-tech.com/s/architect/program"
GENERIC = ["create-register-row", "delete-register-row", "rename-register-row", "replace-register-row"]
REST = ["connect-adjacency", "disconnect-adjacency", "connect-trace", "disconnect-trace", "rename-meta", "replace-meta", "rename-project", "replace-project", "rename-governance", "replace-governance"]
KINDS = GENERIC + REST


def read(path):
    return open(path, encoding="utf-8").read()


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    open(path, "w", encoding="utf-8").write(text)


def pascal(kebab):
    return "".join(part[:1].upper() + part[1:] for part in kebab.split("-"))


def camel(kebab):
    value = pascal(kebab)
    return value[:1].lower() + value[1:]


def register_blocks(source):
    pattern = re.compile(r'                        #\[path = "\."\]\n                        pub mod ((create|delete|rename|replace)_(\w+)) \{\n(?:.*\n)*?                        \}\n')
    return [match for match in pattern.finditer(source) if match.group(3) in BY_ENTITY]


def domains():
    """🗺️ entity → old domain directory, read off the crate root mounts (or the fixture tree once the mounts are gone)."""
    source = read(ROOT_RS)
    found = {}
    for match in register_blocks(source):
        found[match.group(3)] = re.search(r"🧬️mutations/([^/]+)/[^/]+/🦀️.rs", match.group(0)).group(1)
    if found:
        return found
    return json.load(open(f"{os.path.dirname(__file__)}/📋️register-row-domains.json", encoding="utf-8"))


def mount_block(op):
    leaf = f"{ANY_REL}/🧬️schema/🧬️mutations/📋️register-row/{OPS[op]}"
    return (f'                        #[path = "."]\n                        pub mod {op}_register_row {{\n'
            f'                            #[path = "{leaf}/🦀️.rs"]\n                            mod component;\n'
            f'                            #[path = "{leaf}/🔺️diff/🦀️.rs"]\n                            pub mod diff;\n'
            f'                            #[path = "{leaf}/↩️inverse/🦀️.rs"]\n                            pub mod inverse;\n'
            f'                            pub use component::*;\n                        }}\n')


def phase_tree():
    found = domains()
    json.dump(found, open(f"{os.path.dirname(__file__)}/📋️register-row-domains.json", "w", encoding="utf-8"), ensure_ascii=False, indent=2)
    source = read(ROOT_RS)
    blocks = register_blocks(source)
    assert len(blocks) == 256, len(blocks)
    first = blocks[0].start()
    for match in reversed(blocks):
        source = source[:match.start()] + source[match.end():]
    source = source[:first] + "".join(mount_block(op) for op in OPS) + source[first:]
    old_registers = f'                    #[path = "{ANY_REL}/🧬️schema/🗄️registers/🦀️.rs"]\n                    pub mod registers;\n'
    new_registers = (f'                    #[path = "."]\n                    pub mod registers {{\n'
                     f'                        #[path = "{ANY_REL}/🧬️schema/🗄️registers/🦀️.rs"]\n                        mod component;\n'
                     f'                        pub use component::*;\n'
                     f'                        #[path = "{ANY_REL}/🧬️schema/🗄️registers/🧬️schema/🦀️.rs"]\n                        pub mod catalog;\n'
                     f'                        pub use catalog::{{RegisterKey, RegisterLabel, RegisterRow}};\n                    }}\n')
    assert old_registers in source
    source = source.replace(old_registers, new_registers)
    write(ROOT_RS, source)

    enum_rs = f"{MUT}/🦀️.rs"
    source = read(enum_rs)
    variants = re.compile(r"    (Create|Delete|Rename|Replace)(\w+)\(super::\w+::\w+\),\n")
    kept = [line for line in source.splitlines(keepends=True) if not (variants.fullmatch(line) and variants.fullmatch(line).group(2) in BY_VARIANT)]
    source = "".join(kept)
    generic_variants = "".join(f"    {pascal(kind)}(super::{kind.replace('-', '_')}::{pascal(kind)}),\n" for kind in GENERIC)
    source = source.replace("pub enum ProgramMutation {\n", "pub enum ProgramMutation {\n" + generic_variants, 1)
    kinds = re.search(r"pub const KINDS: &\[&str\] = &\[\n(?:    \"[a-z-]+\",\n)+\];\n", source)
    source = source[:kinds.start()] + "pub const KINDS: &[&str] = &[\n" + "".join(f'    "{kind}",\n' for kind in KINDS) + "];\n" + source[kinds.end():]
    header_end = source.index("use crate::standards")
    source = ('//! 🧬️ Architect program artifact — document mutation dispatch enum.\n//!\n'
              '//! Fourteen kinds. The 64 id-keyed registers share the four generic `📋️register-row` leaves\n'
              '//! (`create`/`delete`/`rename`/`replace`), typed per register by the catalog generated from\n'
              '//! `🗄️registers/🧬️schema/🔣️.json` (`RegisterKey`, `RegisterRow`); the two edge registers (`adjacencies`,\n'
              '//! `traces`) keep `connect`/`disconnect`, and the three document-level facets (`meta`/`project`/`governance`)\n'
              '//! keep `rename`/`replace`.\n//!\n'
              '//! `#[derive(dsl::Mutations)]` generates `impl protocol::Mutation<ProgramSnapshot>` and\n'
              '//! `impl protocol::SemanticMutation<ProgramSnapshot>` by delegating each variant to its payload\'s\n'
              '//! `protocol::MutationKind` impl; this file is dispatch-only. Physical ownership is declared by the exact\n'
              '//! `mutationDomainOwners` taxonomy registry: seven domains own the fourteen operation leaves.\n\n') + source[header_end:]
    source = source.replace('/// 🧩️ Every variant wraps exactly one `protocol::MutationKind<ProgramSnapshot, ProgramMutation>`\n/// payload struct declared in the corresponding triad leaf\'s `🦠️mutation/🦀️.rs`.',
                            '/// 🧩️ Every variant wraps exactly one `protocol::MutationKind<ProgramSnapshot, ProgramMutation>`\n/// payload struct declared in its leaf\'s `🦀️.rs`.')
    source = source.replace("exhaustive `mutate-*` case measures itself against (66 registers × create/delete/rename/replace, minus the four the two edge-shaped registers replace with connect/disconnect).",
                            "exhaustive `mutate-*` case measures itself against (the four generic register-row kinds, connect/disconnect over the two edge registers, rename/replace over the three facets).")
    fixture_mount = '#[cfg(test)]\n#[path = "🧪️tests/🔬️fixture/🦀️.rs"]\nmod fixture_tests;\n'
    corpus = '#[cfg(test)]\n#[path = "📋️register-row/🧪️tests/🔬️corpus/🦀️.rs"]\nmod register_row_corpus;\n'
    assert fixture_mount in source
    source = source.replace(fixture_mount, fixture_mount + corpus)
    source = source.replace("/// 🧫️ Handcrafted per-mutation fixture cases (contract D1, ticket\n/// `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`) — one `🧪️tests/<case>/🦀️.rs` per mutation\n/// leaf, mounted here rather than in `🦀️.rs` so this tree owns its own test wiring.",
                            "/// 🧫️ Handcrafted per-mutation fixture cases (contract D1, ticket `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`): one\n/// `🧪️tests/<case>/🦀️.rs` per edge or facet leaf, and the register-row corpus that walks every committed\n/// `📋️register-row` case at run time.")
    write(enum_rs, source)

    fixture_rs = f"{MUT}/🧪️tests/🔬️fixture/🦀️.rs"
    lines = read(fixture_rs).splitlines(keepends=True)
    out = []
    index = 0
    while index < len(lines):
        target = re.match(r"mod tests_(create|delete|rename|replace)_(\w+);", lines[index + 1]) if index + 1 < len(lines) else None
        if lines[index].startswith("#[path") and target and target.group(2) in BY_ENTITY:
            index += 2
            continue
        out.append(lines[index])
        index += 1
    write(fixture_rs, "".join(out))

    for entity, domain in found.items():
        shutil.rmtree(f"{MUT}/{domain}")
    leaf_registers = f"{ANY_REL}/🧬️schema/🗄️registers"
    for path in [f"{MUT}/📋️register-row/{op}/🦀️.rs" for op in OPS.values()]:
        write(path, read(path).replace("registers::catalog::", "registers::"))
    print("tree: 256 leaves unmounted and removed, 4 generic leaves mounted, catalog mounted under", leaf_registers)


def case_name(domain, case):
    return f"{domain}-{re.sub(r'^[^a-z0-9]+', '', case)}"


def generic_payload(op, entry, payload):
    payload = dict(payload)
    old = payload.pop("mutation")
    assert old == camel(f"{op}-{entry['entity']}"), (old, op, entry["entity"])
    if op in ("create", "replace"):
        index = payload.pop("index", None)
        (field, row), = payload.items()
        out = {"mutation": f"{op}RegisterRow", "row": {"register": entry["key"], "row": row}}
        if index is not None:
            out["index"] = index
        return out
    return {"mutation": f"{op}RegisterRow", "register": entry["key"], **payload}


def phase_fixtures():
    found = domains()
    moved = []
    for entity, domain in sorted(found.items()):
        entry = BY_ENTITY[entity]
        for op, op_dir in OPS.items():
            source = f"{FIX}/{domain}/{op_dir}"
            for case in sorted(os.listdir(source)):
                target = f"{FIX}/📋️register-row/{op_dir}/{case_name(domain, case)}"
                assert not os.path.exists(target), target
                os.makedirs(os.path.dirname(target), exist_ok=True)
                shutil.move(f"{source}/{case}", target)
                mutation = f"{target}/🦠️mutation/🔣️.json"
                write(mutation, json.dumps(generic_payload(op, entry, json.load(open(mutation, encoding="utf-8"))), ensure_ascii=False, indent=2) + "\n")
                moved.append(target)
            os.rmdir(source)
        os.rmdir(f"{FIX}/{domain}")
    longest = max(moved, key=lambda path: len(path[len(REPO) + 1:]) + len("/📸️snapshot/⬅️before/🔣️.json"))
    tail = longest[len(REPO) + 1:] + "/📸️snapshot/⬅️before/🔣️.json"
    print(f"fixtures: {len(moved)} cases moved; longest path {len(tail)} chars / {len(tail.encode('utf-16-le')) // 2} UTF-16 units")


def split_top(text, separator=","):
    parts, depth, current, quote = [], 0, "", None
    for index, character in enumerate(text):
        if quote:
            current += character
            if character == quote and text[index - 1] != "\\":
                quote = None
            continue
        if character == '"':
            quote = character
        elif character in "([{":
            depth += 1
        elif character in ")]}":
            depth -= 1
        if character == separator and depth == 0:
            parts.append(current)
            current = ""
        else:
            current += character
    if current.strip():
        parts.append(current)
    return [part.strip() for part in parts]


def balanced(text, start):
    depth, quote = 0, None
    for index in range(start, len(text)):
        character = text[index]
        if quote:
            if character == quote and text[index - 1] != "\\":
                quote = None
            continue
        if character == '"':
            quote = character
        elif character in "([{":
            depth += 1
        elif character in ")]}":
            depth -= 1
            if depth == 0:
                return index
    raise ValueError("unbalanced")


def rewrite_calls(source):
    pattern = re.compile(r"ProgramMutation::(Create|Delete|Rename|Replace)(%s)\(" % "|".join(sorted(BY_VARIANT, key=len, reverse=True)))
    used, manual, out, cursor = set(), [], [], 0
    for match in pattern.finditer(source):
        if match.start() < cursor:
            continue
        verb, variant = match.group(1), match.group(2)
        end = balanced(source, match.end() - 1)
        inner = source[match.end():end].strip()
        literal = re.fullmatch(r"(.*?)(create|delete|rename|replace)_\w+::(\w+)\s*\{(.*)\}", inner, re.S)
        if not literal:
            manual.append(source[match.start():end + 1])
            continue
        prefix, fields = literal.group(1), dict()
        for field in split_top(literal.group(4)):
            name, _, value = field.partition(":")
            fields[name.strip()] = value.strip() or name.strip()
        op = verb.lower()
        if op in ("create", "replace"):
            row = next(value for name, value in fields.items() if name != "index")
            body = f"row: RegisterRow::{variant}({row})" + (f", index: {fields['index']}" if op == "create" else "")
            used.add("RegisterRow")
        else:
            body = f"register: RegisterKey::{variant}, id: {fields['id']}" + (f", new_name: {fields['new_name']}" if op == "rename" else "")
            used.add("RegisterKey")
        out.append(source[cursor:match.start()])
        out.append(f"ProgramMutation::{verb}RegisterRow({prefix}{op}_register_row::{verb}RegisterRow {{ {body} }})")
        cursor = end + 1
    out.append(source[cursor:])
    return "".join(out), used, manual


def phase_callsites():
    roots = [f"{ANY}/✏️editor", f"{ANY}/🚪️io", f"{MUT}/🧪️tests"]
    for root in roots:
        for directory, _, files in os.walk(root):
            for name in files:
                if name != "🦀️.rs":
                    continue
                path = f"{directory}/{name}"
                source = read(path)
                rewritten, used, manual = rewrite_calls(source)
                for snippet in manual:
                    print(f"MANUAL {path[len(REPO) + 1:]}: {snippet[:160]}")
                if rewritten == source:
                    continue
                anchor = re.search(r"^( *)use [^\n]*(ProgramMutation|mutations as leaves)[^\n]*;\n", rewritten, re.M) or re.search(r"^( *)use [^\n]*;\n", rewritten, re.M)
                names = ", ".join(sorted(used))
                line = f"{anchor.group(1)}use {REGISTERS_PATH}::{names if len(used) == 1 else '{' + names + '}'};\n"
                rewritten = rewritten[:anchor.end()] + line + rewritten[anchor.end():]
                write(path, rewritten)
                print(f"rewrote {path[len(REPO) + 1:]} ({names})")


if __name__ == "__main__":
    {"tree": phase_tree, "fixtures": phase_fixtures, "callsites": phase_callsites}[sys.argv[1]]()
