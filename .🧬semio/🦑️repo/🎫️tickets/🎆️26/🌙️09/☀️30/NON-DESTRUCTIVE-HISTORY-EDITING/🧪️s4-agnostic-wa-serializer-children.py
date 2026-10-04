#!/usr/bin/env python3
"""🪆️ D2 W-a step 1 (S4-AGNOSTIC, design §20.15): every `io_mechanism::Serializer<S>` impl takes the composed artifact's owned
children (`serialize(from: &S, _: &ArchiveChildren)`), its import gains `ArchiveChildren`. Idempotent, count-asserted per file
(signatures rewritten == `impl … Serializer<…> for` blocks), region-safe (only the `async fn serialize(from: &X) -> …IoResult<…IoPayload>`
line of a trait impl and the three known import spellings are touched; ArtifactSerializer impls and inherent `serialize` helpers keep
their bytes). Each file is re-read immediately before its write. Usage: [--apply]; without it a dry run."""
import re
import subprocess
import sys

ROOT = "/Users/ueli/Documents/semio"
IMPL = re.compile(r"impl(?:<[^>]*>)?\s+(?:[a-z_]+::)*Serializer<[^{]*?>\s+for\s+[A-Za-z0-9_]+")
SIGNATURE = re.compile(r"(async fn serialize\(from: &[^,()]+)\)( -> (?:[a-z_]+::)*IoResult<(?:[a-z_]+::)*IoPayload> \{)")
IMPORTS = {
    "use semio_framework::io::io_mechanism::Serializer;": "use semio_framework::io::io_mechanism::{ArchiveChildren, Serializer};",
    "use semio_framework::io::io_mechanism::{serialize_dsl_txt, Serializer};": "use semio_framework::io::io_mechanism::{serialize_dsl_txt, ArchiveChildren, Serializer};",
    "use semio_framework::io::io_mechanism::{Deserializer, Serializer};": "use semio_framework::io::io_mechanism::{ArchiveChildren, Deserializer, Serializer};",
}


def files() -> list[str]:
    out = subprocess.run(["git", "grep", "-l", "-E", r"impl(<[^>]*>)? *([a-z_:]*::)?Serializer<", "--", "✏️s/*.rs", "🧰️framework/*.rs", "🌎️hub/*.rs"], cwd=ROOT, capture_output=True, text=True, check=True).stdout
    return [line for line in out.splitlines() if line and not line.startswith("🧰️framework/🔨️modules/🚪️io/")]


def rewrite(text: str) -> tuple[str, int, int]:
    impls = len([m for m in IMPL.finditer(text) if not re.search(r"(Artifact|De)Serializer<", m.group(0))])
    imported = next((old for old in IMPORTS if old in text), None)
    param = "_: &ArchiveChildren" if imported or any(new in text for new in IMPORTS.values()) else "_: &semio_framework::io::io_mechanism::ArchiveChildren"
    done = len(re.findall(r"async fn serialize\(from: &[^,()]+, _: &(?:semio_framework::io::io_mechanism::)?ArchiveChildren\)", text))
    text, count = SIGNATURE.subn(lambda m: f"{m.group(1)}, {param}){m.group(2)}", text)
    if imported and count:
        text = text.replace(imported, IMPORTS[imported], 1)
    return text, impls, count + done


def main() -> None:
    apply = "--apply" in sys.argv
    problems = 0
    changed = 0
    for path in files():
        absolute = f"{ROOT}/{path}"
        with open(absolute, encoding="utf-8") as handle:
            before = handle.read()
        after, impls, signatures = rewrite(before)
        if impls != signatures:
            problems += 1
            print(f"MISMATCH {path}: {impls} impls, {signatures} signatures")
            continue
        if after == before:
            continue
        changed += 1
        if apply:
            with open(absolute, encoding="utf-8") as handle:
                if handle.read() != before:
                    problems += 1
                    print(f"RACE {path}: changed while rewriting, skipped")
                    continue
            with open(absolute, "w", encoding="utf-8") as handle:
                handle.write(after)
        print(f"{'WROTE' if apply else 'WOULD'} {path} ({signatures})")
    print(f"files changed {changed}, problems {problems}, {'applied' if apply else 'dry run'}")
    sys.exit(1 if problems else 0)


if __name__ == "__main__":
    main()
