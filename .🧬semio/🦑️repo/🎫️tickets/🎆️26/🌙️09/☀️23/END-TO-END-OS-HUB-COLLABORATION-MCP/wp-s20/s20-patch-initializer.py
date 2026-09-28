"""🏗️ S20 window-3 prepared patch: the SDK's default whole-document initializer pairs with its default bounded owners.

`ArtifactApp`, `ArtifactEditor` and `ArtifactViewer` default `build_document_store_owners` to
`bounded_document_store_owners`, but their `build_document_store_initialization_job` defaulted to `Err(envelope)`: every
app without a hand-written override refuses every whole-document load with `artifact-store.persisted-initializer-refused`
(measured live 28 12:4x: Import Document of puzzle3d; also every example switch — cad's override docstring). Step A makes
the three defaults `Ok(bounded_document_store_initialization_job(envelope, Self::DOCUMENT_SCHEMA, operation, generation))`.
Step B removes every app override that now repeats that default exactly (same schema expression as the impl's own
`const DOCUMENT_SCHEMA`), so one definition remains.

Usage: python3 s20-patch-initializer.py [--dry-run]   (idempotent; prints every hunk it would write or wrote)
"""
import re, subprocess, sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
SDK = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"
PLUGINS = ROOT / "✏️s/🔌️plugins"
DRY = "--dry-run" in sys.argv

DEFAULT_OLD = re.compile(
    r"(?P<head>        fn build_document_store_initialization_job\(\n"
    r"            envelope: ArtifactEnvelope<Self::Snapshot, Self::Mutation>,\n)"
    r"            _operation: semio_framework_job::OperationId,\n"
    r"            _generation: semio_framework_job::Generation,\n"
    r"(?P<ret>        \) -> ArtifactInitializationAdmission<Self::Snapshot, Self::Mutation> \{\n)"
    r"            Err\(envelope\)\n"
    r"        \}\n"
)
DEFAULT_NEW = (
    r"\g<head>"
    "            operation: semio_framework_job::OperationId,\n"
    "            generation: semio_framework_job::Generation,\n"
    r"\g<ret>"
    "            Ok(bounded_document_store_initialization_job(envelope, Self::DOCUMENT_SCHEMA, operation, generation))\n"
    "        }\n"
)
DOC_OLD = (
    "        /// 🏗️ Begins the app's retained semantic-validation/replay/store-construction job.\n"
    "        /// Rejection returns the exact completed envelope and grants no replacement authority.\n"
)
DOC_NEW = (
    "        /// 🏗️ Begins the app's retained semantic-validation/replay/store-construction job — framework-owned by\n"
    "        /// default, paired with [`Self::build_document_store_owners`]' bounded default, so every whole-document load\n"
    "        /// (Import Document, example switch, archive restore) can replace the store; an app with its own owner catalog\n"
    "        /// supplies its own job. Rejection returns the exact completed envelope and grants no replacement authority.\n"
)
VIEWER_DOC_OLD = "        /// 🏗️ Restores a saved document through the viewer's retained validation and replay authority.\n"
VIEWER_DOC_NEW = (
    "        /// 🏗️ Restores a saved document through the viewer's retained validation and replay authority — the\n"
    "        /// bounded job by default, paired with the bounded default owners.\n"
)

OVERRIDE = re.compile(
    r"\n(?P<doc>(?:[ \t]*///[^\n]*\n)*)(?P<attr>(?:[ \t]*#\[expect\(clippy::result_large_err[^\n]*\n)?)"
    r"(?P<indent>[ \t]*)fn build_document_store_initialization_job\(\s*envelope: [^\n]+\n\s*operation: [^\n]+\n\s*generation: [^\n]+\n\s*\)\s*->[^{\n]+\{\s*"
    r"Ok\(semio_framework_plugin::bounded_document_store_initialization_job\(envelope, (?P<schema>[A-Za-z0-9_:]+), operation, generation\)\)\s*\}\n"
)
SCHEMA_CONST = re.compile(r"const DOCUMENT_SCHEMA: &'static str = (?P<value>[A-Za-z0-9_:]+);")


def sdk_step() -> list[str]:
    text = SDK.read_text()
    notes = []
    applied = text.count("Ok(bounded_document_store_initialization_job(envelope, Self::DOCUMENT_SCHEMA, operation, generation))")
    new, count = DEFAULT_OLD.subn(DEFAULT_NEW, text)
    notes.append(f"A sdk defaults: {count} to rewrite, {applied} already applied (expect 3 total)")
    if count + applied != 3:
        raise SystemExit(f"A: expected 3 trait defaults, found {count} refusing + {applied} applied — re-read the SDK")
    if DOC_OLD in new:
        new = new.replace(DOC_OLD, DOC_NEW, 1)
        notes.append("A ArtifactApp doc rewritten")
    if VIEWER_DOC_OLD in new:
        new = new.replace(VIEWER_DOC_OLD, VIEWER_DOC_NEW, 1)
        notes.append("A ArtifactViewer doc rewritten")
    if new != text and not DRY:
        SDK.write_text(new)
    return notes


def impl_block_bounds(text: str, at: int) -> tuple[int, int]:
    start = text.rfind("\nimpl", 0, at)
    end = text.find("\nimpl", at)
    return (0 if start < 0 else start, len(text) if end < 0 else end)


def dedupe_step() -> list[str]:
    notes = []
    files = subprocess.run(["/usr/bin/grep", "-rl", "--include=🦀️.rs", "fn build_document_store_initialization_job", str(PLUGINS)], capture_output=True, text=True).stdout.split("\n")
    for name in sorted(filter(None, files)):
        if "🧪️tests" in name:
            continue
        path = Path(name)
        text = path.read_text()
        removals = []
        for match in OVERRIDE.finditer(text):
            lo, hi = impl_block_bounds(text, match.start())
            consts = SCHEMA_CONST.findall(text[lo:hi])
            if len(consts) != 1:
                notes.append(f"B keep {path.relative_to(PLUGINS)}: impl has {len(consts)} DOCUMENT_SCHEMA consts")
                continue
            if consts[0].split("::")[-1] != match.group("schema").split("::")[-1]:
                notes.append(f"B keep {path.relative_to(PLUGINS)}: schema {match.group('schema')} != DOCUMENT_SCHEMA {consts[0]}")
                continue
            removals.append(match)
        if not removals:
            continue
        new = text
        for match in reversed(removals):
            new = new[: match.start()] + ("" if new[match.start() - 1] == "\n" else "\n") + new[match.end():]
        orphaned = sorted({match.group("schema").split("::")[-1] for match in removals if not re.search(rf"\b{re.escape(match.group('schema').split('::')[-1])}\b", re.sub(r"(?m)^\s*use [^;]*;", "", new))})
        notes.append(f"B dedupe {path.relative_to(PLUGINS)}: {len(removals)} override(s) removed{f'; now only imported, never used: {orphaned}' if orphaned else ''}")
        if not DRY:
            path.write_text(new)
    return notes


def main() -> None:
    notes = sdk_step() + dedupe_step()
    print("\n".join(notes))
    print(f"{'dry-run' if DRY else 'applied'}: {sum(1 for line in notes if line.startswith('B dedupe'))} plugin files deduped")


if __name__ == "__main__":
    main()
