"""🧪️ Compiles the native relocate arithmetic in isolation against the shared fixture."""

from argparse import ArgumentParser
import json
from pathlib import Path
import re
import subprocess


def rust_point(values):
    return "[" + ", ".join(repr(float(value)) for value in values) + "]"


parser = ArgumentParser(description=__doc__)
parser.add_argument("--source", type=Path, required=True)
parser.add_argument("--fixture", type=Path, required=True)
parser.add_argument("--out", type=Path, required=True)
parser.add_argument("--rustc-arg", action="append", default=[])
arguments = parser.parse_args()
source = arguments.source.read_text(encoding="utf-8")
fixture = json.loads(arguments.fixture.read_text(encoding="utf-8"))
grid = fixture["grid"]
constants = re.search(r"^const WORLD_RELOCATE_EPSILON: .+;$", source, re.MULTILINE).group()
position = re.search(r"^pub fn world3d_relocate_dispatch_args\([\s\S]+?^}", source, re.MULTILINE).group()
snap = re.search(r"^fn snap_world_point_to_grid\([\s\S]+?^}", source, re.MULTILINE).group()
tests = []
for index, case in enumerate(grid["cases"]):
    expected = "None" if case["expected"] is None else f"Some({rust_point(case['expected'])})"
    tests.append(
        f"#[test] fn grid_case_{index}() {{\n"
        f"    assert_eq!(world3d_relocate_dispatch_args({rust_point(grid['origin'])}, {rust_point(grid['grab'])}, {rust_point(case['target'])}, World3dRelocatePlane::Z, true, {float(grid['factor'])}), {expected});\n"
        "}"
    )
for case in fixture["cardinal"]:
    plane = "XYZ"[case["axis"]]
    snap_enabled = str(case["snap"]).lower()
    for sample, expected in [("preview", "expectedPreview"), ("release", "expectedRelease")]:
        tests.append(
            f"#[test] fn {case['view']}_{sample}() {{\n"
            f"    assert_eq!(world3d_relocate_dispatch_args({rust_point(case['origin'])}, {rust_point(case['grab'])}, {rust_point(case[sample])}, World3dRelocatePlane::{plane}, {snap_enabled}, 1.0), Some({rust_point(case[expected])}));\n"
            "}"
        )
arguments.out.mkdir(parents=True, exist_ok=True)
unit = arguments.out / "native_relocate.rs"
unit.write_text(
    "\n".join(
        [
            "#[derive(Clone, Copy)] pub enum World3dRelocatePlane { X, Y, Z }",
            "impl World3dRelocatePlane { fn axis(self) -> usize { self as usize } }",
            constants,
            position,
            snap,
            *tests,
        ]
    ),
    encoding="utf-8",
)
binary = arguments.out / "native_relocate"
subprocess.run(["rustc", *arguments.rustc_arg, "--edition", "2021", "--test", str(unit), "-o", str(binary)], check=True)
subprocess.run([str(binary)], check=True)
