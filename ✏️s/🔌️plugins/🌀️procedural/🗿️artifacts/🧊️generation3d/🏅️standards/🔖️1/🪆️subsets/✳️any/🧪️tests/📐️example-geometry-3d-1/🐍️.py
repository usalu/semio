"""🧊️ Third-party oracle half of `procedural-3d-1-example-geometry`.

Re-derives every committed expected number in the eight bundled generation3d examples'
`📚️examples/<example>/🧪️tests/🧩️example/🔣️.json` fixtures from two committed, language-neutral
inputs only — the example's own `🗣️.dsl.semio` graph and the packaged brep extension descriptor
`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🔣️.json` (for the channel defaults an unwired port
falls back to) — using `numpy`/`scipy` for the two solids whose volume has no closed form.

**Why this is an oracle and not a transliteration.** It reads no Rust. It does not evaluate the
graph: it recognises the eight op chains as the classical solids they are (rectangular prism,
regular-polygon prism, hollow box, rounded box as a Minkowski sum, ball-cube union, ball-minus-torus
difference) and integrates those solids independently. The SUBJECT half —
`📚️examples/🧪️tests/🧩️geometry/🦀️.rs` — evaluates the same DSL through the real flow host, runs
the real brep kernel, tessellates the preview handle and holds the resulting triangle soup to the
same fixture, cross-checked in-lane by `parry3d`. Oracle and subject therefore meet on one committed
file each, never on each other's code.

**What this does NOT establish.** Nothing here touches the kernel, so a disagreement located here is
a fixture error and a disagreement located in the Rust lane is a kernel error. Both have happened:
this file's own `ball_cube_union` once modelled a CONCENTRIC cube while the kernel's
`brep.prim3d.box` grows from its corner (a fixture error, in the oracle and the fixture together),
and `➡️sweep`'s prism builder once emitted inward lateral faces (a kernel error, which no oracle
number moved).

Run directly: `.venv/bin/python3 "🐍️.py"` from this directory, or `uv run pytest` from the repo
root (collected by `pyproject.toml`'s `python_files = ["🐍️.py"]`).

@see 🥒️.feature
@see ../../🔮️oracle/🔣️.json — the `parry3d` + `scipy` registration this case's tags name.
@see ../../📚️examples/🧪️tests/🧩️geometry/🦀️.rs — the subject half.
"""

from __future__ import annotations

import json
import math
import re
import sys
from pathlib import Path

import numpy as np
from scipy import integrate

# region 🔖️Paths
SUBSET = Path(__file__).resolve().parents[2]
EXAMPLES = SUBSET / "📚️examples"
BREP_DESCRIPTOR = SUBSET.parents[6] / "🌊️flow" / "🧩️extensions" / "📐️brep" / "🔣️.json"
FIXTURE_SCHEMA = "s.procedural.generation3d.example-geometry/v1"
# endregion 🔖️Paths


# region 🔖️Readers
def example_directories() -> list[Path]:
    """📚️ Every bundled example that ships an expected-geometry fixture, in name order."""
    return sorted(path.parent.parent.parent for path in EXAMPLES.glob("*/🧪️tests/🧩️example/🔣️.json"))


def read_dsl(example_dir: Path) -> str:
    """🗣️ The example's single real asset — its flow-fixture DSL."""
    assets = list((example_dir / "🖼️assets").glob("*/🗣️.dsl.semio"))
    if len(assets) != 1:
        raise AssertionError(f"{example_dir.name}: expected exactly one dsl asset, found {len(assets)}")
    return assets[0].read_text(encoding="utf-8")


def read_fixture(example_dir: Path) -> dict:
    """📇️ The committed expected-geometry statement both lanes read."""
    return json.loads((example_dir / "🧪️tests" / "🧩️example" / "🔣️.json").read_text(encoding="utf-8"))


def sliders(dsl: str) -> dict[str, float]:
    """🎚️ Every `input-slider`'s committed value, keyed by widget id."""
    return {match.group(1): float(match.group(2)) for match in re.finditer(r'input-slider id="([^"]+)"[^\n]*?\svalue=(-?[\d.]+)', dsl)}


def neuron_kinds(dsl: str) -> list[str]:
    """🧠️ Every `neuron-kind` the graph declares, in file order."""
    return re.findall(r"neuron-kind=([\w.]+)", dsl)


def channel_defaults() -> dict[tuple[str, str], dict]:
    """🔌️ Declared per-operator channel defaults, read from the packaged brep extension descriptor."""
    descriptor = json.loads(BREP_DESCRIPTOR.read_text(encoding="utf-8"))
    manifest = json.loads(descriptor["contributions"]["topicContributions"][0]["payload"]["manifestJson"])
    return {(operator["id"], channel["name"]): channel.get("default") for operator in manifest["contributes"]["operators"] for channel in operator["inputs"]}
# endregion 🔖️Readers


# region 🔖️Solids
def regular_polygon_area(circumradius: float, sides: int) -> float:
    """📐️ Area of a regular `sides`-gon inscribed in a circle of `circumradius`."""
    return 0.5 * sides * circumradius * circumradius * math.sin(2.0 * math.pi / sides)


def rounded_box_volume(side: float, radius: float) -> float:
    """🧱️ A cube with every edge rolled to `radius` is the Minkowski sum of a shrunk cube with a ball."""
    core = side - 2.0 * radius
    return core**3 + 6.0 * radius * core**2 + 3.0 * math.pi * radius**2 * core + (4.0 / 3.0) * math.pi * radius**3


def ball_cube_union(ball_radius: float, cube_side: float) -> float:
    """🧲️ |ball ∪ cube| for `brep.prim3d.box`'s own placement: the cube spans `[0, side]³` from its
    CORNER and the ball is centred on that corner, so the two meet in the ball's POSITIVE OCTANT
    clipped to the box — not in a concentric intersection. Each `z` slice contributes a quarter disc
    of radius `√(r² − z²)` clipped to the square `[0, side]²`, integrated by quadrature."""

    def clipped_quarter_disc_area(z: float) -> float:
        squared = ball_radius * ball_radius - z * z
        if squared <= 0.0:
            return 0.0
        radius = math.sqrt(squared)
        if radius <= cube_side:
            return math.pi * squared / 4.0
        if radius >= cube_side * math.sqrt(2.0):
            return cube_side * cube_side
        chord = math.sqrt(squared - cube_side * cube_side)
        return cube_side * chord + squared / 2.0 * (math.asin(cube_side / radius) - math.asin(chord / radius))

    limit = min(ball_radius, cube_side)
    overlap, _ = integrate.quad(clipped_quarter_disc_area, 0.0, limit, limit=400, points=[0.0, limit])
    return (4.0 / 3.0) * math.pi * ball_radius**3 + cube_side**3 - overlap


def ball_minus_torus(ball_radius: float, major: float, minor: float) -> float:
    """🍩️ |ball \\ torus| for a z-axis torus concentric with the ball, by quadrature over the tube angle."""

    def revolved_shell(angle: float) -> float:
        cosine = math.cos(angle)
        discriminant = major * major * cosine * cosine - major * major + ball_radius * ball_radius
        if discriminant <= 0.0:
            return 0.0
        reach = min(minor, max(0.0, -major * cosine + math.sqrt(discriminant)))
        return 2.0 * math.pi * (major * reach * reach / 2.0 + cosine * reach**3 / 3.0)

    inside, _ = integrate.quad(revolved_shell, 0.0, 2.0 * math.pi, limit=500, epsabs=1e-12, epsrel=1e-12)
    return (4.0 / 3.0) * math.pi * ball_radius**3 - inside
# endregion 🔖️Solids


# region 🔖️Expectations
def expected(example: str, knob: dict[str, float], defaults: dict[tuple[str, str], dict]) -> dict:
    """🎯️ What each op chain's solid must measure, derived here and nowhere else."""
    if example == "rectangle-wire-preview":
        return {"volume": None, "perimeter": 2.0 * (knob["width"] + knob["height"]), "closed": False, "extent": (knob["width"], knob["height"], 0.0)}
    if example == "rectangle-extrude-volume":
        return {"volume": knob["width"] * knob["height"] * knob["distance"], "perimeter": None, "closed": True, "extent": (knob["width"], knob["height"], knob["distance"])}
    if example == "face-sweep-extrude":
        return {"volume": knob["width"] * knob["height"] * knob["distance"], "perimeter": None, "closed": True, "extent": (knob["width"], knob["height"], knob["distance"])}
    if example == "hexagonal-mushroom-column":
        area = regular_polygon_area(knob["radius"], int(round(knob["sides"])))
        return {"volume": area * knob["height"], "perimeter": None, "closed": True, "extent": None}
    if example == "box-shell-preview":
        return {"volume": knob["size"] ** 3 - (knob["size"] - 2.0 * knob["thickness"]) ** 3, "perimeter": None, "closed": True, "extent": (knob["size"], knob["size"], knob["size"])}
    if example == "box-fillet-preview":
        return {"volume": rounded_box_volume(knob["size"], knob["radius"]), "perimeter": None, "closed": True, "extent": (knob["size"], knob["size"], knob["size"])}
    if example == "sphere-box-fuse":
        return {"volume": ball_cube_union(knob["radius"], knob["size"]), "perimeter": None, "closed": True, "extent": (knob["radius"] + max(knob["radius"], knob["size"]),) * 3}
    if example == "sphere-cut-with-torus":
        major = defaults[("brep.prim3d.torus", "major")]["value"]
        minor = defaults[("brep.prim3d.torus", "minor")]["value"]
        ball = knob["slider_2"]
        reach = (ball * ball + major * major - minor * minor) / (2.0 * major)
        return {"volume": ball_minus_torus(ball, major, minor), "perimeter": None, "closed": True, "extent": (2.0 * reach, 2.0 * reach, 2.0 * ball)}
    raise AssertionError(f"no oracle derivation registered for example {example!r}")
# endregion 🔖️Expectations


# region 🔖️Checks
def check(example_dir: Path, defaults: dict[tuple[str, str], dict], failures: list[str]) -> int:
    """✅️ Holds one example's committed fixture to this file's own derivation."""
    dsl = read_dsl(example_dir)
    fixture = read_fixture(example_dir)
    example = fixture["example"]
    checked = 0
    if fixture["schema"] != FIXTURE_SCHEMA:
        failures.append(f"{example}: fixture schema {fixture['schema']!r}")
    for kind in neuron_kinds(dsl):
        if kind not in fixture["opChain"]:
            failures.append(f"{example}: dsl declares {kind!r}, absent from the fixture op chain")
    for kind in fixture["opChain"]:
        if kind not in dsl:
            failures.append(f"{example}: fixture op chain claims {kind!r}, absent from the dsl")
    want = expected(example, sliders(dsl), defaults)
    got = fixture["expect"]
    if got["closed"] != want["closed"]:
        failures.append(f"{example}: closed {got['closed']} want {want['closed']}")
    checked += 1
    if want["volume"] is None:
        if got["volume"] is not None:
            failures.append(f"{example}: fixture states a volume for an edge-only preview")
    else:
        if got["volume"] is None or abs(got["volume"] - want["volume"]) > got["volumeTolerance"]:
            failures.append(f"{example}: volume {got['volume']} want {want['volume']:.12f} (tolerance {got['volumeTolerance']})")
        checked += 1
    if want["perimeter"] is not None:
        if got["edgePerimeter"] is None or abs(got["edgePerimeter"] - want["perimeter"]) > got["edgePerimeterTolerance"]:
            failures.append(f"{example}: edge perimeter {got['edgePerimeter']} want {want['perimeter']:.12f}")
        checked += 1
    if want["extent"] is not None:
        span = np.array(got["boundingBoxMax"], dtype=float) - np.array(got["boundingBoxMin"], dtype=float)
        if not np.allclose(span, np.array(want["extent"], dtype=float), atol=max(got["boundingBoxTolerance"], 1e-9)):
            failures.append(f"{example}: bounding-box span {span.tolist()} want {list(want['extent'])}")
        checked += 1
    return checked


def main() -> int:
    """🚦 Runs every example and reports one line per disagreement."""
    defaults = channel_defaults()
    directories = example_directories()
    if len(directories) != 8:
        print(f"FAIL: expected 8 bundled examples with expected-geometry fixtures, found {len(directories)}", file=sys.stderr)
        return 1
    failures: list[str] = []
    checked = sum(check(directory, defaults, failures) for directory in directories)
    if failures:
        print(f"FAIL ({len(failures)} disagreement(s) between this oracle and the committed fixtures):", file=sys.stderr)
        for line in failures:
            print(f"  - {line}", file=sys.stderr)
        return 1
    print(f"PASS: numpy {np.__version__} + scipy agree with all {checked} committed expectations across {len(directories)} examples")
    return 0


def test_committed_example_geometry_fixtures_match_the_oracle() -> None:
    """🧪️ pytest entry point for the same run."""
    assert main() == 0
# endregion 🔖️Checks


# region 🔖️Registration
def adapter():
    """🧭️ Registration by scenario id, in the ORACLE role only — registering these derivations as
    subjects too would make the reference its own subject and manufacture a green self-comparison.
    One handler per bundled example, each returning the numbers THIS file derives from the example's
    own DSL and the packaged brep descriptor, never anything read back out of the fixture.

    The harness module is imported inside the function so this file stays runnable on its own
    (`python3 🐍️.py`) and collectable by `pytest`, neither of which provides `semio_repo_test`.

    ⚖️ The `parry3d` half of this capability is NOT served here and cannot be: a generated repository
    test host takes no Cargo dependency on a plugin crate or on a third-party one, so it runs in-crate
    as `[[test]] example-geometry` instead (see 🥒️.feature)."""
    from semio_repo_test import Adapter, Outcome

    defaults = channel_defaults()

    def derivation(example_dir: Path):
        def handler(context):
            del context
            failures: list[str] = []
            check(example_dir, defaults, failures)
            if failures:
                raise AssertionError("; ".join(failures))
            fixture = read_fixture(example_dir)
            want = expected(fixture["example"], sliders(read_dsl(example_dir)), defaults)
            return Outcome(
                {
                    "example": fixture["example"],
                    "closed": want["closed"],
                    "volume": want["volume"],
                    "perimeter": want["perimeter"],
                    "extent": None if want["extent"] is None else [float(axis) for axis in want["extent"]],
                }
            )

        return handler

    built = Adapter("python")
    for directory in example_directories():
        built = built.oracle(read_fixture(directory)["example"], derivation(directory))
    return built
# endregion 🔖️Registration


if __name__ == "__main__":
    raise SystemExit(main())
