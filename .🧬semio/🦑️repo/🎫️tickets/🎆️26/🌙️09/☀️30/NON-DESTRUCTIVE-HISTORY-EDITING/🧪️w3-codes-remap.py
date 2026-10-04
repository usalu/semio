"""🔁️ W3-CODES: remaps non-vocabulary mutation outcome codes onto the frozen vocabulary, one plugin lane at a time.

Usage: `python3 🧪️w3-codes-remap.py <lane> [<lane> ...]`. Every lane lists git-tracked roots and ordered regex
rules; a rule rewrites code AND level together, so a lane is idempotent (a second run matches nothing). Prints one
line per touched file with its substitution count; `--dry` prints without writing."""
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
WFC3D = "✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any"
FORMS = "✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🏅️standards/🔖️1/🪆️subsets/✳️any"
STDIO = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
REMODEL = "✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any"
MISMATCH = r"(?:content-gap|content-kind-mismatch|content-conflict|content-capacity|incomplete-mesh|invalid-reconstruction-(?:sparse|mesh|asset))"
INVALID = r"(?:invalid-asset-payload|invalid-content-chunk)"
RASTER = "✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations"
MISSING = r"(?:missing|not-found|unknown-tile|unknown-pinned-tile)"

LANES = {
    "wfc3d": ([WFC3D], [
        (r'MutationOutcome::(?:error|fatal)\(\s*"wfc3d\.(?:tile|slot|edge|rule)\.' + MISSING + '"', 'MutationOutcome::error("mutation.target-missing"'),
        (r'MutationOutcome::fatal\(\s*"wfc3d\.(?:tile|slot|edge|rule)\.duplicate-id"', 'MutationOutcome::fatal("mutation.duplicate-id"'),
        (r'MutationOutcome::fatal\(\s*"wfc3d\.(?:tile\.non-positive-weight|slot\.degenerate-box|edge\.self-loop)"', 'MutationOutcome::fatal("mutation.invariant"'),
        (r'\.warn\(\s*"wfc3d\.[a-z.-]+"', '.warning("mutation.no-op"'),
        (r'\.info\(\s*"wfc3d\.[a-z.-]+-cascaded"', '.info("mutation.cascade"'),
        (r'\("(?:error|fatal)", "wfc3d\.(?:tile|slot|edge|rule)\.' + MISSING + '"\)', '("error", "mutation.target-missing")'),
        (r'\("fatal", "wfc3d\.(?:tile|slot|edge|rule)\.duplicate-id"\)', '("fatal", "mutation.duplicate-id")'),
        (r'\("fatal", "wfc3d\.(?:tile\.non-positive-weight|slot\.degenerate-box|edge\.self-loop)"\)', '("fatal", "mutation.invariant")'),
        (r'\("warning", "wfc3d\.[a-z.-]+"\)', '("warning", "mutation.no-op")'),
        (r'\("info", "wfc3d\.[a-z.-]+-cascaded"\)', '("info", "mutation.cascade")'),
        (r'"level":"info","code":"wfc3d\.[a-z.-]+-cascaded"', '"level":"info","code":"mutation.cascade"'),
        (r'code\.0 == "wfc3d\.slot\.missing"', 'code.0 == "mutation.target-missing"'),
        (r'code\.0 == "wfc3d\.tile\.duplicate-id"', 'code.0 == "mutation.duplicate-id"'),
        (r'`info`-level `wfc3d\.slot\.edges-cascaded`', '`info`-level `mutation.cascade`'),
        (r'under `wfc3d\.tile\.references-cascaded`', 'under `mutation.cascade`'),
    ]),
    "forms": ([FORMS], [
        (r'MutationOutcome::error\(\s*"forms\.invalid-response"', 'MutationOutcome::fatal("mutation.invariant"'),
        (r'MutationOutcome::error\(\s*"forms\.duplicate-response"', 'MutationOutcome::fatal("mutation.duplicate-id"'),
        (r'MutationOutcome::error\(\s*"forms\.missing-response"', 'MutationOutcome::error("mutation.target-missing"'),
        (r'"code": "forms\.missing-response"', '"code": "mutation.target-missing"'),
    ]),
    "apply-codes": ([f"{STDIO}/{a}" for a in ("☁️las", "🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base", "🏗️ifc", "📐️step", "🔺️stl", "🖋️dxf", "🗽️obj", "🧱️ply")]
                    + ["✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs",
                       "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🧬️mutation-plan/🧪️tests",
                       "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🖥️test-app-mutations-document/🦀️.rs"], [
        (r'"(invalid-(?:remove|modify|add|upsert)-(?:index|target)|duplicate-base-target|invalid-instance-order)"', r'"mutation.apply.\1"'),
        (r'"stdio\.zip\.serialization\.invalid-state"', '"mutation.apply.invalid-state"'),
        (r'"example\.unparsable"', '"mutation.apply.unparsable-example"'),
        (r'"job-test\.value-overflow"', '"mutation.apply.value-overflow"'),
        (r'"test-snapshot\.declared-child-uri"', '"mutation.apply.declared-child-uri"'),
    ]),
    "remodel": ([REMODEL], [
        (r'MutationOutcome::error\(\s*"mutation\.' + INVALID + '"', 'MutationOutcome::fatal("mutation.invariant"'),
        (r'refuse\("error", "mutation\.' + INVALID + '"', 'refuse("fatal", "mutation.invariant"'),
        (r'"mutation\.' + INVALID + '"', '"mutation.invariant"'),
        (r'"mutation\.' + MISMATCH + '"', '"mutation.target-mismatch"'),
        (r'"mutation\.referenced"', '"mutation.target-referenced"'),
        (r'`mutation\.referenced`', '`mutation.target-referenced`'),
        (r'`mutation\.invalid-reconstruction-\*`', '`mutation.target-mismatch`'),
        (r'`mutation\.content-\*`', '`mutation.target-mismatch`'),
        (r'`mutation\.' + INVALID + '`', '`mutation.invariant`'),
        (r'`mutation\.' + MISMATCH + '`', '`mutation.target-mismatch`'),
        (r'(?<![\w.`"-])mutation\.' + MISMATCH + r'(?![\w-])', 'mutation.target-mismatch'),
        (r'(?<![\w.`"-])mutation\.' + INVALID + r'(?![\w-])', 'mutation.invariant'),
        (r'(?<![\w.`"-])mutation\.referenced(?![\w-])', 'mutation.target-referenced'),
    ]),
    "raster": ([RASTER], [
        (r'"mutation\.(?:image|mask|transform|lock|adjustment)-conflict"', '"mutation.target-mismatch"'),
        (r'"mutation\.adjustment-capacity"', '"mutation.target-mismatch"'),
        (r'"mutation\.asset-missing"', '"mutation.target-missing"'),
        (r'"mutation\.transform-invalid"', '"mutation.invariant"'),
        (r'if adjustment_kind != "brightnessContrast" \|\| (!matches!\(payload\.parameter\.as_str\(\), "brightness" \| "contrast"\) \|\| .*?) \{ return Err\("mutation\.adjustment-invalid"\); \}',
         r'if \1 { return Err("mutation.invariant"); }\n    if adjustment_kind != "brightnessContrast" { return Err("mutation.target-mismatch"); }'),
        (r'protocol::MutationOutcome::error\(code,', 'protocol::MutationOutcome::refuse(code,'),
    ]),
    "profiles": ([f"{STDIO}/{a}" for a in ("📐️step", "🏗️ifc", "🎨️svg", "📰️xml", "📕️xlsx", "📜️docx", "📖️pdf")], [
        (r'"stdio\.(?:step\.cc[1-6]\.mutation-rejected|ifc\.2x3\.[a-z0-9]+\.mutation-rejected|svg\.(?:tiny|basic)\.mutation-outside-profile|xml\.valid\.mutation-outside-subset|xlsx\.canonical-edit\.invalid|docx\.xml-address\.invalid)"', '"mutation.target-mismatch"'),
        (r'MutationOutcome::error\("stdio\.pdf\.remove-page\.invalid-target",', 'MutationOutcome::error(if self.index < base.pages.len() { "mutation.target-mismatch" } else { "mutation.target-missing" },'),
        (r'"stdio\.pdf\.[a-z-]+\.invalid-target"', '"mutation.target-missing"'),
    ]),
}


def tracked(roots):
    out = subprocess.run(["git", "ls-files", "-z", "--", *roots], cwd=ROOT, capture_output=True, check=True).stdout
    return [ROOT / p.decode() for p in out.split(b"\0") if p]


def run(lane, dry):
    roots, rules = LANES[lane]
    compiled = [(re.compile(pattern), replacement) for pattern, replacement in rules]
    for path in tracked(roots):
        if path.suffix not in {".rs", ".ts", ".tsx", ".py", ".json", ".feature", ".md"} or not path.exists():
            continue
        text = path.read_text(encoding="utf-8")
        total = 0
        for pattern, replacement in compiled:
            text, count = pattern.subn(replacement, text)
            total += count
        if total:
            print(f"[w3-codes] {lane} {total:3d} {path.relative_to(ROOT)}")
            if not dry:
                path.write_text(text, encoding="utf-8")


if __name__ == "__main__":
    dry = "--dry" in sys.argv
    for lane in [arg for arg in sys.argv[1:] if arg != "--dry"]:
        run(lane, dry)
