"""W1G-2: every plugin-owned retained store initializer validates and finds edits through the framework's
`store::ArtifactStoreInitializationEditIndex` (linear) instead of the quadratic `ValidateEditPair` and the per-position scans.
Usage: python3 <this> [--apply]   (without --apply it prints a unified diff per file)."""
import difflib, pathlib, re, subprocess, sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
APPLY = "--apply" in sys.argv
ADMISSION = "store::ArtifactStoreInitializationEditAdmission"
files = subprocess.run(["/usr/bin/grep", "-rl", "ValidateEditPair", "✏️s/", "--include=*.rs"], cwd=ROOT, capture_output=True, text=True).stdout.split()


def one(text, old, new, path, count=1):
    found = text.count(old)
    if found != count:
        sys.exit(f"{path}: expected {count} of {old!r}, found {found}")
    return text.replace(old, new)


for relative in sorted(files):
    path = ROOT / relative
    text = path.read_text(encoding="utf-8")
    original = text
    prefix = re.search(r"enum (\w+)StoreInitializationPhase \{", text).group(1)
    phase = f"{prefix}StoreInitializationPhase"
    text = one(text, "    ValidateEditPair { left: usize, right: usize },\n", "    ValidateEdit { index: usize },\n", relative)
    text = one(text, "    FindApplied { position: usize, scan: usize },\n", "    FindApplied { position: usize },\n", relative)
    text = one(text, "    FindRedo { position: usize, scan: usize },\n", "    FindRedo { position: usize },\n", relative)
    text = one(text, "    edit_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,\n", "    edit_digest: std::mem::ManuallyDrop<Option<store::ArtifactStoreInitializationDigest>>,\n    edit_index: store::ArtifactStoreInitializationEditIndex,\n", relative)
    text = re.sub(r"(\n(\s+)edit_digest: std::mem::ManuallyDrop::new\(None\),\n)", lambda m: f"{m.group(1)}{m.group(2)}edit_index: store::ArtifactStoreInitializationEditIndex::default(),\n", text, count=1)
    text = one(text, f"{phase}::ValidateEditPair {{ left: 0, right: 1 }}", f"{phase}::ValidateEdit {{ index: 0 }}", relative)
    chain = re.compile(
        r"(?P<indent>[ \t]+)if left >= envelope\.vcs\.edits\.len\(\) \{\n"
        r"\s+self\.phase = (?P<next>[^;]+);\n"
        r"\s+\} else if right >= envelope\.vcs\.edits\.len\(\) \{\n"
        r"\s+self\.phase = [\w:]+::ValidateEditPair \{ left: left \+ 1, right: left \+ 2 \};\n"
        r"\s+\} else if (?P<cond>[^\n]+) \{\n"
        r"\s+(?P<fail>[^\n]+);\n"
        r"\s+\} else \{\n"
        r"\s+self\.phase = [\w:]+::ValidateEditPair \{ left, right: right \+ 1 \};\n"
        r"\s+\}\n")

    def admission(match):
        indent = match.group("indent")
        limit = re.search(r"\.id\.len\(\) > (\w+)", match.group("cond"))
        bound = limit.group(1) if limit else "usize::MAX"
        return (f"{indent}match self.edit_index.admit(&envelope.vcs.edits, index, {bound}) {{\n"
                f"{indent}    {ADMISSION}::Complete => self.phase = {match.group('next')},\n"
                f"{indent}    {ADMISSION}::Admitted => self.phase = {phase}::ValidateEdit {{ index: index + 1 }},\n"
                f"{indent}    {ADMISSION}::Oversized | {ADMISSION}::Duplicate => {match.group('fail')},\n"
                f"{indent}}}\n")

    split = re.compile(
        r"(?P<indent>[ \t]+)if left >= envelope\.vcs\.edits\.len\(\) \{\n"
        r"\s+self\.phase = (?P<next>[^;]+);\n"
        r"\s+\} else if envelope\.vcs\.edits\[left\]\.id\.len\(\) > (?P<bound>\w+) \{\n"
        r"\s+(?P<oversized>[^\n]+);\n"
        r"\s+\} else if right >= envelope\.vcs\.edits\.len\(\) \{\n"
        r"\s+self\.phase = [\w:]+::ValidateEditPair \{ left: left \+ 1, right: left \+ 2 \};\n"
        r"\s+\} else if [^\n]+ \{\n"
        r"\s+(?P<duplicate>[^\n]+);\n"
        r"\s+\} else \{\n"
        r"\s+self\.phase = [\w:]+::ValidateEditPair \{ left, right: right \+ 1 \};\n"
        r"\s+\}\n")

    def split_admission(match):
        indent = match.group("indent")
        return (f"{indent}match self.edit_index.admit(&envelope.vcs.edits, index, {match.group('bound')}) {{\n"
                f"{indent}    {ADMISSION}::Complete => self.phase = {match.group('next')},\n"
                f"{indent}    {ADMISSION}::Admitted => self.phase = {phase}::ValidateEdit {{ index: index + 1 }},\n"
                f"{indent}    {ADMISSION}::Oversized => {match.group('oversized')},\n"
                f"{indent}    {ADMISSION}::Duplicate => {match.group('duplicate')},\n"
                f"{indent}}}\n")

    text, chains = chain.subn(admission, text)
    if chains == 0:
        text, chains = split.subn(split_admission, text)
    if chains != 1:
        sys.exit(f"{relative}: ValidateEditPair chain matched {chains} times")
    text = one(text, f"{phase}::ValidateEditPair {{ left, right }} =>", f"{phase}::ValidateEdit {{ index }} =>", relative)
    for lane, getter in (("FindApplied", "applied_id"), ("FindRedo", "redo_id")):
        text = one(text, f"{phase}::{lane} {{ position, scan }} =>", f"{phase}::{lane} {{ position }} =>", relative)
        text = text.replace(f"{phase}::{lane} {{ position: 0, scan: 0 }}", f"{phase}::{lane} {{ position: 0 }}")
        text = text.replace(f"{phase}::{lane} {{ position: position + 1, scan: 0 }}", f"{phase}::{lane} {{ position: position + 1 }}")
        start = text.index(f"{phase}::{lane} {{ position }} =>")
        head = re.compile(rf"(let Some\(id\) = self\.{getter}\(position\) else \{{\n(?:.*\n)*?(?P<indent>[ \t]+)\}};\n)")
        match = head.search(text, start)
        arm_end = text.index(f"{phase}::", match.end())
        if lane == "FindRedo":
            block = match.group(1)
            cleared = re.sub(rf"(\n(\s+))(self\.phase = {phase}::BuildCandidate;)", r"\1self.edit_index.clear();\1\3", block, count=1)
            if cleared == block:
                sys.exit(f"{relative}: FindRedo end did not reach BuildCandidate")
            text = text[:match.start(1)] + cleared + text[match.end(1):]
            match = head.search(text, start)
        insert = f"{match.group('indent')}let scan = self.edit_index.position(&id).unwrap_or(usize::MAX);\n"
        text = text[:match.end(1)] + insert + text[match.end(1):]
        arm_end = text.index(f"\n            {phase}::", match.end())
        arm = text[match.end(1):arm_end]
        missing = re.search(r"(self\.fail\(b\"[^\"]+\"\))", arm)
        if not missing:
            sys.exit(f"{relative}: {lane} has no missing-edit failure")
        rescan = f"self.phase = {phase}::{lane} {{ position, scan: scan + 1 }}"
        if arm.count(rescan) != 1:
            sys.exit(f"{relative}: {lane} rescan found {arm.count(rescan)} times")
        arm = arm.replace(rescan, missing.group(1))
        text = text[:match.end(1)] + arm + text[arm_end:]
    if "ValidateEditPair" in text or re.search(r"scan: scan \+ 1|scan: 0\b", text):
        sys.exit(f"{relative}: leftovers")
    if APPLY:
        path.write_text(text, encoding="utf-8")
        print(f"applied {relative}")
    else:
        sys.stdout.writelines(difflib.unified_diff(original.splitlines(True), text.splitlines(True), relative, relative, n=1))
