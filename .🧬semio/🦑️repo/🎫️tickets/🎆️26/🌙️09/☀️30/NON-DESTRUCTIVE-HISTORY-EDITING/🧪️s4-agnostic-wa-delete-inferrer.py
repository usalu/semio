#!/usr/bin/env python3
"""🧹️ D2 W-a step 3 (S4-AGNOSTIC): deletes every `ArtifactInferrer` marker impl (the trait has no runtime caller; inference runs through
`ArtifactInferenceService` and `protocol::Inference`). Per file: the whole `//#region 🔖️ArtifactInferrer` region when it holds only
doc/comment lines, `pub struct <X>Inferrer;` markers (+ attributes) and exactly one `impl … ArtifactInferrer for …` block; else a bare
two-line impl block (`type Snapshot`/`type Inference` only); then the `use semio_framework_plugin::ArtifactInferrer;` import. Files whose
shape is anything else are reported and left untouched. Idempotent, every file re-read immediately before its write; the four hand-edited
files (gltf root, drawing/animate/wires schema roots) and the trait owner are excluded. Usage: [--apply]."""
import re
import subprocess
import sys

ROOT = "/Users/ueli/Documents/semio"
EXCLUDED = ("🔌️plugin/🦀️.rs", "🗿️artifacts/🧊️gltf/🦀️.rs", "🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs", "🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs", "🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs")
IMPL = re.compile(r"^\s*impl\s+(?:[a-z_]+::)*ArtifactInferrer\s+for\s+[^{]+\{", re.M)
BARE = re.compile(r"\n?^impl\s+(?:[a-z_]+::)*ArtifactInferrer\s+for\s+[^{\n]+\{\n(?:    type (?:Snapshot|Inference) = [^;\n]+;\n){2}\}\n", re.M)
IMPORT = re.compile(r"^use semio_framework_plugin::ArtifactInferrer;\n", re.M)
REGION = re.compile(r"^//#region 🔖️ArtifactInferrer\n(.*?)^//#endregion 🔖️ArtifactInferrer\n", re.M | re.S)


def impl_end(text: str, start: int) -> int:
    depth, index = 1, text.index("{", start) + 1
    while depth:
        depth += {"{": 1, "}": -1}.get(text[index], 0)
        index += 1
    return index


def region_ok(body: str) -> bool:
    impls = list(IMPL.finditer(body))
    if len(impls) != 1:
        return False
    end = impl_end(body, impls[0].start())
    rest = body[: impls[0].start()] + body[end:]
    return all(not line.strip() or line.lstrip().startswith(("//", "#[")) or re.fullmatch(r"pub struct [A-Za-z0-9_]+Inferrer;", line.strip()) for line in rest.split("\n"))


def collapse(text: str, at: int) -> str:
    head, tail = text[:at], text[at:]
    while head.endswith("\n\n") and tail.startswith("\n"):
        tail = tail[1:]
    return head + tail


def rewrite(text: str) -> tuple[str, str]:
    match = REGION.search(text)
    if match:
        if not region_ok(match.group(1)):
            return text, "region holds more than the marker"
        text = collapse(text[: match.start()] + text[match.end() :], match.start())
    else:
        bare = BARE.search(text)
        if bare is None:
            return text, "no deletable impl" if IMPL.search(text) else "no impl"
        text = text[: bare.start()] + text[bare.end() :]
    text = IMPORT.sub("", text)
    leftover = [line for line in text.split("\n") if "ArtifactInferrer" in line and not line.lstrip().startswith("//")]
    return text, "; ".join(leftover)


def main() -> None:
    apply = "--apply" in sys.argv
    paths = subprocess.run(["git", "grep", "-l", "ArtifactInferrer", "--", "✏️s/*.rs", "🧰️framework/*.rs", "🌎️hub/*.rs"], cwd=ROOT, capture_output=True, text=True, check=True).stdout.split()
    written = problems = 0
    for path in paths:
        if path.endswith(EXCLUDED):
            continue
        with open(f"{ROOT}/{path}", encoding="utf-8") as handle:
            before = handle.read()
        after, note = rewrite(before)
        if after == before:
            if note != "no impl":
                problems += 1
                print(f"SKIP {path}: {note}")
            continue
        if note:
            problems += 1
            print(f"LEFTOVER {path}: {note}")
            continue
        if apply:
            with open(f"{ROOT}/{path}", encoding="utf-8") as handle:
                if handle.read() != before:
                    problems += 1
                    print(f"RACE {path}")
                    continue
            with open(f"{ROOT}/{path}", "w", encoding="utf-8") as handle:
                handle.write(after)
        written += 1
    print(f"files {'written' if apply else 'to write'} {written}, problems {problems}")
    sys.exit(1 if problems else 0)


if __name__ == "__main__":
    main()
