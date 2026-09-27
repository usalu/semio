"""🪪️ G12 window-3 set (guest-linked, lands AFTER T14's F9 `store::content_id`): ids a command mints are scoped by the
admitting author, never by a per-instance counter a fresh session restarts.

Root (measured 2026-09-27, `📓️wp-g12.md` item 1): every retained command builder scopes its ids by
`AppOperationContext { app_instance_id, parent_document_id, operation_id, generation }` — all four repeat across writers
and across fresh sessions at the same base (note: `text-<fnv(scope)>-<n>`), so two writers acting on one base mint the
same block id and the hub takes both as distinct ops.

The set:
  1. SDK `🔌️plugin/🦀️.rs`: `AppOperationContext.authoring_seed` + `ArtifactOwnedToolJobRequest.authoring_seed`, minted once
     per admission by `VcsArtifactApp::authoring_seed(actor)` = `store::content_id("authoring-seed", actor ␟ HLC ticked for
     the admission)` on BOTH admission lanes (shell typed operation, agent preview).
  2. Every `AppOperationContext { … }` literal (retained builders + tests) carries it: builders
     `authoring_seed: request.authoring_seed.clone()`, test literals `authoring_seed: "authoring-seed-test".into()`.
  3. note: `NoteCommandWork` scopes its `NoteIdOwner` by the seed.

usage: python3 g12-authoring-seed.py [--dry-run|--write] [--root <tree>]   (default --dry-run, root = the repo)
Idempotent: an already-applied site is reported as applied and left alone."""
import os
import re
import sys

args = sys.argv[1:]
WRITE = "--write" in args
ROOT = args[args.index("--root") + 1] if "--root" in args else "/Users/ueli/Documents/semio"
SDK = os.path.join(ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs")
NOTE = os.path.join(ROOT, "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧵️retained/🦀️.rs")
SCAN = [os.path.join(ROOT, "✏️s"), os.path.join(ROOT, "🧰️framework")]

SDK_EDITS = [
    (
        "AppOperationContext.authoring_seed",
        """    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct AppOperationContext {
        pub app_instance_id: u32,
        pub parent_document_id: String,
        pub operation_id: u64,
        pub generation: u64,
        pub canonical_base_revision: [u8; 32],
    }
""",
        """    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct AppOperationContext {
        pub app_instance_id: u32,
        pub parent_document_id: String,
        pub operation_id: u64,
        pub generation: u64,
        pub canonical_base_revision: [u8; 32],
        /// 🪪️ Who admitted this command, and when: content-addressed over the admitting actor and the document
        /// store's hybrid logical clock ticked for the admission (`VcsArtifactApp::authoring_seed`). Every id a
        /// command mints is scoped by it, so two writers — or two sessions of one — never mint the same id at one base.
        pub authoring_seed: String,
    }
""",
    ),
    (
        "ArtifactOwnedToolJobRequest.authoring_seed",
        """    pub struct ArtifactOwnedToolJobRequest<A: ArtifactApp> {
        pub command: Box<A::Command>,
        pub raw_wire: ArtifactToolRawInput,
        pub operation: semio_framework_job::Operation,
        pub controller_id: String,
        pub tool_id: String,
        pub payload_schema_id: String,
        pub contract: semio_framework::ToolExecutionContract,
        pub decoded_items: usize,
        pub app_instance_id: u32,
        pub parent_document_id: String,
        pub canonical_base_revision: [u8; 32],
""",
        """    pub struct ArtifactOwnedToolJobRequest<A: ArtifactApp> {
        pub command: Box<A::Command>,
        pub raw_wire: ArtifactToolRawInput,
        pub operation: semio_framework_job::Operation,
        pub controller_id: String,
        pub tool_id: String,
        pub payload_schema_id: String,
        pub contract: semio_framework::ToolExecutionContract,
        pub decoded_items: usize,
        pub app_instance_id: u32,
        pub parent_document_id: String,
        pub canonical_base_revision: [u8; 32],
        pub authoring_seed: String,
""",
    ),
    (
        "agent preview admission",
        """                parent_document_id: self.store.envelope().id.clone(),
                canonical_base_revision: roots.canonical_base_revision,
                snapshot: roots.snapshot,
""",
        """                parent_document_id: self.store.envelope().id.clone(),
                canonical_base_revision: roots.canonical_base_revision,
                authoring_seed: self.authoring_seed(&meta.actor),
                snapshot: roots.snapshot,
""",
    ),
    (
        "shell typed-operation admission",
        """                    parent_document_id,
                    canonical_base_revision,
                    snapshot,
                    config,
""",
        """                    parent_document_id,
                    canonical_base_revision,
                    authoring_seed: self.authoring_seed(&meta.actor),
                    snapshot,
                    config,
""",
    ),
    (
        "VcsArtifactApp::authoring_seed",
        """        async fn preview_retained_command(&mut self, command: Box<A::Command>, proof: &QualifiedToolProof, meta: &ActionMeta) -> Result<Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, Fault> {
""",
        """        async fn preview_retained_command(&mut self, command: Box<A::Command>, proof: &QualifiedToolProof, meta: &ActionMeta) -> Result<Emit<A::Mutation, A::ConfigMutation, A::DraftMutation>, Fault> {
""",
    ),
]

SEED_METHOD_ANCHOR = """        /// 👁️ Builds the SAME retained job the shell lane builds for this command (`A::build_tool_job` over"""
SEED_METHOD = """        /// 🪪️ The identity every id one admitted command mints is scoped by ([`AppOperationContext::authoring_seed`]):
        /// content-addressed with [`store::content_id`] over the admitting actor and this document store's hybrid
        /// logical clock ticked for the admission — per author and moment, never a per-instance counter a fresh
        /// session restarts.
        fn authoring_seed(&self, actor: &str) -> String {
            let mut clock = self.store.clock_now();
            clock.tick(semio_framework_job::default_now_ms().unwrap_or(clock.physical_ms));
            store::content_id("authoring-seed", format!("{actor}\\u{1f}{}:{}:{}", clock.actor, clock.physical_ms, clock.logical).as_bytes())
        }

"""

NOTE_OLD = """        let scope = format!("{}:{}:{}:{}", operation.app_instance_id, operation.parent_document_id, operation.operation_id, operation.generation);"""
NOTE_NEW = """        let scope = format!("{}:{}:{}:{}:{}", operation.app_instance_id, operation.parent_document_id, operation.operation_id, operation.generation, operation.authoring_seed);"""

LITERAL = re.compile(r"AppOperationContext \{")


def literal_span(text, start):
    depth = 0
    for index in range(start, len(text)):
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return index
    return -1


def patch_literals(text):
    out, cursor, report = [], 0, []
    for match in LITERAL.finditer(text):
        if match.start() < cursor:
            continue
        prefix = text[max(0, match.start() - 12):match.start()]
        if "struct " in prefix or "impl " in prefix:
            continue
        end = literal_span(text, match.end() - 1)
        body = text[match.end():end]
        if "authoring_seed" in body or re.search(r"\.\.[A-Za-z_(]", body):
            report.append("applied")
            continue
        field = re.search(r"canonical_base_revision: (.+?)(?=\s*[,}\n])", body + "}")
        if field is None:
            report.append(f"problem: literal without canonical_base_revision: {body[:80]!r}")
            continue
        seed = "request.authoring_seed.clone()" if field.group(1).startswith("request.") else '"authoring-seed-test".into()'
        insert_at = match.end() + field.end()
        trailing_comma = text[insert_at] == ","
        if trailing_comma:
            insert_at += 1
        indent = re.search(r"\n([ \t]*)canonical_base_revision", body)
        if indent is not None:
            indent = indent.group(1)
            piece = f"\n{indent}authoring_seed: {seed}," if trailing_comma else f",\n{indent}authoring_seed: {seed}"
        else:
            piece = f" authoring_seed: {seed}," if trailing_comma else f", authoring_seed: {seed}"
        out.append(text[cursor:insert_at])
        out.append(piece)
        cursor = insert_at
        report.append(f"insert {seed}")
    out.append(text[cursor:])
    return "".join(out), report


def main():
    problems, changed = [], {}
    sdk = open(SDK, encoding="utf-8").read()
    patched = sdk
    for name, old, new in SDK_EDITS:
        if name == "VcsArtifactApp::authoring_seed":
            if "fn authoring_seed(&self, actor: &str)" in patched:
                print(f"  sdk {name}: applied")
            elif patched.count(SEED_METHOD_ANCHOR) == 1:
                patched = patched.replace(SEED_METHOD_ANCHOR, SEED_METHOD + SEED_METHOD_ANCHOR)
                print(f"  sdk {name}: insert")
            else:
                problems.append(f"sdk {name}: anchor count {patched.count(SEED_METHOD_ANCHOR)}")
            continue
        if patched.count(new) == 1:
            print(f"  sdk {name}: applied")
        elif patched.count(old) == 1:
            patched = patched.replace(old, new)
            print(f"  sdk {name}: insert")
        else:
            problems.append(f"sdk {name}: anchor count {patched.count(old)}")
    if patched != sdk:
        changed[SDK] = patched
    store_root = os.path.join(ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store")
    if not any("pub fn content_id(" in open(os.path.join(d, f), encoding="utf-8").read() for d, _, fs in os.walk(store_root) for f in fs if f.endswith(".rs")):
        problems.append("store::content_id (T14 F9) is not in this tree yet — apply F9 first")
    note = open(NOTE, encoding="utf-8").read()
    if NOTE_NEW in note:
        print("  note scope: applied")
    elif note.count(NOTE_OLD) == 1:
        changed[NOTE] = note.replace(NOTE_OLD, NOTE_NEW)
        print("  note scope: insert")
    else:
        problems.append("note scope anchor missing")
    literal_files = 0
    for base in SCAN:
        for directory, _, files in os.walk(base):
            if "/target" in directory or "/node_modules" in directory or "/.🧬semio" in directory:
                continue
            for name in files:
                if not name.endswith(".rs"):
                    continue
                path = os.path.join(directory, name)
                text = changed.get(path) or open(path, encoding="utf-8").read()
                if "AppOperationContext {" not in text:
                    continue
                new_text, report = patch_literals(text)
                for entry in report:
                    if entry.startswith("problem"):
                        problems.append(f"{os.path.relpath(path, ROOT)}: {entry}")
                if new_text != text:
                    changed[path] = new_text
                    literal_files += 1
    print(f"root {ROOT}: {len(changed)} file(s) to change ({literal_files} with literals), {len(problems)} problem(s)")
    for problem in problems:
        print(f"  PROBLEM {problem}")
    if problems:
        sys.exit(1)
    if WRITE:
        for path, text in changed.items():
            open(path, "w", encoding="utf-8").write(text)
        print("written; gate: cargo check -p semio-framework-plugin -p semio-s-plugin-note + every builder crate (--lib --tests) native + wasm32-wasip2")
    else:
        print("dry run clean" if changed else "nothing to do (applied)")


main()
