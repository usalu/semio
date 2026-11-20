"""Compile current solid transaction sources against the repository's native unit artifacts."""
import json
from pathlib import Path
import shutil
import subprocess
import sys

ticket = Path(__file__).resolve().parent
build = Path(sys.argv[1]).resolve()
repo_root = Path.cwd()
source_root = repo_root / "🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🧠️native-registry/🏗️solid-transaction"
fixture = repo_root / "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️certified-patterns.json"
output = ticket / "🗑️generated" / "pattern-native-check"


def run(*args, **options):
    result = subprocess.run(["repo", "tools", "exec", "--light", *args], **options)
    if result.returncode:
        raise SystemExit(result.returncode)
    return result


def artifact_index(root):
    return {
        lib.name.removeprefix("lib").split(".")[0]: lib
        for extension in ("rlib", "dylib", "so", "rmeta")
        for lib in root.glob(f"*/*/out/lib*.{extension}")
        if extension != "rmeta" or not any(lib.with_suffix(f".{kind}").exists() for kind in ("rlib", "dylib", "so"))
    }


def external_arguments(root, names):
    artifacts = artifact_index(root)
    anchor = next((lib for key, lib in artifacts.items() if key.startswith("semio_framework_3d_brep_solid_transaction-")), None)
    if anchor is None:
        anchor = next(lib for key, lib in artifacts.items() if key.startswith("semio_framework_3d_brep_construction-"))
    receipt = json.loads(anchor.with_suffix(".rmeta.semio-deps").read_text())
    preferred = {target for target, _ in receipt["dependencies"]}
    closure = set()
    direct = []
    for name in names:
        libraries = [(key, lib) for key, lib in artifacts.items() if key.rsplit("-", 1)[0] == name]
        _, library = max(libraries, key=lambda artifact: (artifact[0] in preferred, artifact[1].stat().st_mtime_ns))
        direct.append(library)
        record = library.with_suffix(".rmeta.semio-deps")
        if record.exists():
            closure.update(target for target, _ in json.loads(record.read_text())["dependencies"])
    options = [f"-Ldependency={folder}" for folder in sorted({artifacts[key].parent for key in closure if key in artifacts})]
    for name, lib in zip(names, direct):
        metadata = lib.with_suffix(".rmeta")
        if metadata.is_file() and metadata != lib:
            options.append(f"--extern={name}={metadata}")
        options.append(f"--extern={name}={lib}")
    return options


def quoted(path):
    return json.dumps(str(path), ensure_ascii=False)


rust = f'#[path = {quoted(source_root / "🦀️.rs")}] mod actual;\n'
rust += f'const FIXTURE: &str = include_str!({quoted(fixture)});\n'
rust += r'''
use actual::{SolidBoundary, SolidPatternRecipe, SolidPublication, UnpublishedSolidPattern};
use semio_framework_3d_brep_construction::primitives::SolidPrimitive;
use semio_framework_3d_brep_representation::topology::{Body, EntityRef};
use semio_framework_3d_brep_vector::{Pnt3, matrix::Frame3};
use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, ValueError};
use serde_json::Value;

const GRANT: Grant = Grant { maximum_items: 1, maximum_copy_bytes: 512, maximum_capacity_bytes: 1<<20,
    maximum_release_bytes: 1<<20, maximum_depth: 4096 };
fn axes(value: &Value) -> [f64; 3] { std::array::from_fn(|index| value[index].as_f64().unwrap()) }
fn num(value: &Value, key: &str) -> f64 { value[key].as_f64().unwrap() }
fn key(_: u64) -> [u8; 64] { [b'0'; 64] }

fn turn(caller: &mut OriginalJobFixtureCaller, op: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError>) -> Result<Progress, ValueError> {
    let mut outcome = None;
    let _ = caller.turn(GRANT, |cx| cx.with_original_retirement(|cx| cx.with_original_work_loan(1, 1500, |cx| {
        let (result, performed) = cx.with_original_effect(GRANT, |_, effect| op(effect))?.unwrap();
        cx.consume_work(performed);
        let progress = match &result { Ok(receipt) => *receipt, Err(error) => error.retained_progress() };
        assert!(performed <= 1 && progress.copied_items <= 1 && progress.fits(GRANT));
        cx.consume_retained(progress)?;
        outcome = Some(result);
        Ok(())
    }))).unwrap();
    outcome.unwrap()
}

#[test]
fn exact_pattern_transaction() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let stock = Grant { maximum_items: 65536, maximum_copy_bytes: 128<<20, maximum_capacity_bytes: 64<<20, maximum_release_bytes: 64<<20, ..GRANT };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(97744), Generation(17), stock, || Some(1000), 1500, 32768);
    for row in fixture["cases"].as_array().unwrap() {
        let input = &row["source"];
        let origin = Pnt3::from_array(axes(&input["origin"]));
        let primitive = if input["kind"].as_str() == Some("box") {
            let [width, depth, height] = axes(&input["size"]);
            SolidPrimitive::Box { origin, width, depth, height }
        } else {
            SolidPrimitive::Sphere { frame: Frame3 { origin, ..Frame3::WORLD }, radius: num(input, "radius") }
        };
        let params = &row["parameters"];
        let recipe = match row["kind"].as_str().unwrap() {
            "linear" => SolidPatternRecipe::linear(axes(&params["direction"]), num(params, "spacing"), num(params, "count") as usize),
            "circular" => SolidPatternRecipe::circular(axes(&params["axis"]), num(params, "count") as usize),
            "grid" => SolidPatternRecipe::grid(axes(&params["dirX"]), axes(&params["dirY"]), num(params, "spacingX"),
                num(params, "spacingY"), num(params, "countX") as usize, num(params, "countY") as usize),
            _ => unreachable!(),
        }.unwrap();
        let mut body = Body::new();
        let mut seed = SolidPublication::begin(1, SolidBoundary::Analytic(primitive), &mut body.labels, key).unwrap();
        while !seed.topology_ready() { turn(&mut caller, |effect| seed.advance(&mut body, effect)).unwrap(); }
        let (source, _) = seed.product(1).unwrap();
        seed.mark_published();
        while !seed.terminal_is_empty() { turn(&mut caller, |effect| seed.close(&mut body, effect).map(|step| step.progress())).unwrap(); }
        let before = [body.vertices.len(), body.faces.len(), body.solids.len()];
        if row["kind"].as_str() == Some("linear") {
            let overlap = &fixture["overlap"];
            let layout = SolidPatternRecipe::linear(axes(&overlap["direction"]), num(overlap, "spacing"), num(overlap, "count") as usize).unwrap();
            let mut rejected = UnpublishedSolidPattern::new(layout, primitive);
            let error = (0..16).find_map(|_| {
                let demand = rejected.demand(&body).unwrap();
                turn(&mut caller, |effect| rejected.advance(&mut body, 1, source, demand, key, effect)).err()
            }).expect("overlap refuses before topology");
            assert!(format!("{error:?}").contains("UnsupportedOwner"));
            assert!(rejected.terminal_is_empty());
            assert_eq!([body.vertices.len(), body.faces.len(), body.solids.len()], before);
        }
        for commit in [false, true] {
            let mut draft = UnpublishedSolidPattern::new(recipe, primitive);
            for _ in 0..2000 {
                if draft.ready() { break; }
                let demand = draft.demand(&body).unwrap();
                turn(&mut caller, |effect| draft.advance(&mut body, 1, source, demand, key, effect)).unwrap();
            }
            assert!(draft.ready());
            let roots = draft.take_roots();
            assert_eq!(roots[0], source);
            let centers = row["centers"].as_array().unwrap();
            assert_eq!(roots.len(), centers.len());
            for (&id, center) in roots.iter().zip(centers) {
                let points = body.reachable_from(&[EntityRef::Solid(id)]);
                let mut lo = [f64::INFINITY; 3];
                let mut hi = [f64::NEG_INFINITY; 3];
                for id in points.vertices.iter() {
                    let p = body.vertices.get(*id).unwrap().position.to_array();
                    for axis in 0..3 { lo[axis] = lo[axis].min(p[axis]); hi[axis] = hi[axis].max(p[axis]); }
                }
                let expected = axes(center);
                assert!((0..3).all(|axis| (0.5*(lo[axis]+hi[axis])-expected[axis]).abs() < 1e-10));
            }
            let expected_solids = if commit { before[2] + roots.len() - 1 } else { before[2] };
            if commit { draft.commit(); } else { draft.restore_roots(roots); }
            for _ in 0..2000 {
                if draft.terminal_is_empty() { break; }
                turn(&mut caller, |effect| draft.close(&mut body, effect).map(|step| step.progress())).unwrap();
            }
            assert!(draft.terminal_is_empty());
            assert_eq!(body.solids.len(), expected_solids);
            if !commit { assert_eq!([body.vertices.len(), body.faces.len(), body.solids.len()], before); }
        }
    }
    caller.retire_exact();
}
'''

names = [
    "semio_framework_3d_brep_boolean", "semio_framework_3d_brep_construction",
    "semio_framework_3d_brep_representation", "semio_framework_3d_brep_surface",
    "semio_framework_3d_brep_tolerance", "semio_framework_3d_brep_vector",
    "semio_framework_value", "semio_framework_job", "serde_json",
]
output.mkdir(parents=True, exist_ok=True)
try:
    source = output / "solid-pattern-check.rs"
    source.write_text(rust, encoding="utf-8")
    executable = output / "solid-pattern-check"
    channel = run("rustup", "--", "show", "active-toolchain", capture_output=True, text=True).stdout.split()[0]
    run("rustup", "--", "run", channel, "rustc", "-A", "dead_code", "--edition=2024", "--test", str(source), "-o", str(executable),
        *external_arguments(build, names))
    run("rustup", "--", "run", channel, str(executable))
finally:
    shutil.rmtree(output)
