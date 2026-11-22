import json
import math
import pathlib
import subprocess

ticket = pathlib.Path(__file__).resolve().parent
generated = ticket / '🗑️generated'
generated.mkdir(exist_ok=True)
root = pathlib.Path('✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📐️formulas')
part = root / '🫙️part-4/🦀️.rs'
rustc = ['repo', 'tools', 'exec', '--light', 'rustup', '--', 'run', 'nightly-2026-07-20', 'rustc', '--edition=2024']
facade = next(line for line in (root / '🦀️.rs').read_text().splitlines() if 'schema_part4::{self' in line)
support = generated / 'en1998-part4-golden-compliance.rs'
support.write_text('pub fn ndp_common(_: &str) -> f64 { panic!("This fixture may only exercise pure formulas") }')
support_lib = generated / 'libsemio_s_plugin_norm_compliance.rlib'
subprocess.run([*rustc, '--crate-name=semio_s_plugin_norm_compliance', '--crate-type=rlib', str(support), '-o', str(support_lib)], check=True)
subprocess.run([*rustc, '--crate-name=semio_s_artifact_norm_en1998_schema_part4', '--crate-type=rlib', '--extern', f'semio_s_plugin_norm_compliance={support_lib}', str(part), '-o', str(generated / 'libsemio_s_artifact_norm_en1998_schema_part4.rlib')], check=True)

pipe_golden = generated / 'en1998-part4-pipe-tests'
subprocess.run([*rustc, '--test', '--crate-name=en1998_part4_pipe_goldens', '--extern', f'semio_s_plugin_norm_compliance={support_lib}', str(part), '-o', str(pipe_golden)], check=True)
subprocess.run([str(pipe_golden), 'buried_pipe_wave_response_matches_common_cases'], check=True)

runner = generated / 'en1998-part4-goldens.rs'
runner.write_text(facade + r'''
use part_4::{SiloPressure, SiloPressureInput, SiloWall, SiloWallSection, granular_silo_pressure};

fn main() {
    let same_export = std::ptr::fn_addr_eq(
        part_4::bonded_pipe_wave_response as fn(pipe_wave::PipeWaveInput) -> Option<pipe_wave::PipeWaveResponse>,
        pipe_wave::bonded_pipe_wave_response as fn(pipe_wave::PipeWaveInput) -> Option<pipe_wave::PipeWaveResponse>,
    );
    assert!(same_export);

    let input = SiloPressureInput {
        fill_height_m: 10.0, width_m: 8.0, height_m: 2.0, acceleration_ratio: 0.25,
        bulk_weight_n_m3: 8000.0, static_pressure_pa: 10000.0,
        wall: SiloWall::Circular { azimuth_deg: 180.0 }, section: SiloWallSection::VerticalFlatBottom,
    };
    assert_eq!(granular_silo_pressure(input), Some(SiloPressure::Applicable {
        reference_pa: 8000.0, additional_pa: -8000.0, total_pa: 2000.0,
    }));
    assert_eq!(granular_silo_pressure(SiloPressureInput { static_pressure_pa: 7999.0, ..input }),
        Some(SiloPressure::Redistribution { reference_pa: 8000.0 }));

    for ratio in [0.25, 1.0, 4.0] {
        println!("[{:.17},{:.17},{:.17},{:.17}]", part_4::impulsive_mass_ratio(ratio),
            part_4::convective_mass_ratio(ratio), part_4::convective_period_s(2.0 * ratio, 2.0),
            part_4::convective_damping_eta());
    }
}
''')
program = generated / 'en1998-part4-goldens'
subprocess.run([*rustc, '--crate-name=en1998_part4_goldens', str(runner), '-L', f'dependency={generated}', '--extern', f'semio_s_artifact_norm_en1998_schema_part4={generated / "libsemio_s_artifact_norm_en1998_schema_part4.rlib"}', '-o', str(program)], check=True)
actual = [json.loads(line) for line in subprocess.check_output([str(program)], text=True).splitlines()]
assert len(actual) == 3
for ratio, measured in zip((0.25, 1.0, 4.0), actual):
    impulsive_argument = 1.7 / ratio
    omega = math.sqrt(1.84 * 9.81 / 2.0 * math.tanh(1.84 * ratio))
    expected = (math.tanh(impulsive_argument) / impulsive_argument,
                0.460 / ratio * math.tanh(1.84 * ratio), math.tau / omega, math.sqrt(10.0 / 5.5))
    assert len(measured) == len(expected)
    assert all(math.isclose(rust, python, rel_tol=1e-14) for rust, python in zip(measured, expected))
print('EN 1998 Part 4: pipe golden, silo golden, two public export paths and twelve Python reference values passed')
