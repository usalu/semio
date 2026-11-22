from pathlib import Path
import subprocess

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
checks = root / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌬️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks"
kernel = checks / "🌙️unoccupied-ventilation/🦀️.rs"
report = checks / "🌙️unoccupied-ventilation/🧾️report/🦀️.rs"
directory = ticket / "🗑️generated/unoccupied-report"
directory.mkdir(parents=True, exist_ok=True)
source = directory / "check.rs"
binary = directory / "check"
source.write_text(r'''
extern crate self as semio_s_artifact_norm_din16798_model;
extern crate self as semio_s_plugin_norm_compliance;
pub enum UnoccupiedVentilationOperation { NonResidentialPurge, NonResidentialReduced, ResidentialReduced, ResidentialShutdown }
pub struct ZoneDocument { pub id: &'static str, pub unoccupied_ventilation: Option<UnoccupiedVentilation> }
pub struct UnoccupiedVentilation {
    pub operation: UnoccupiedVentilationOperation,
    pub assessed_area_m2: f64, pub zone_volume_m3: f64, pub minutes_before_occupancy: f64,
    pub residential_design_rate_l_per_s_m2: f64, pub outdoor_air_m3_h: f64,
}
#[derive(Clone, Copy, PartialEq)]
pub enum AnnexChoice { En, De }
pub enum OccupancyType { Residential, Other }
#[derive(Debug, PartialEq, Eq)]
pub enum CheckStatus { Pass, Fail, UnverifiedBasis }
pub struct ClauseId;
impl ClauseId { pub fn new(_: &str, _: &str, _: &str) -> Self { Self } }
pub struct Quantity(f64);
pub enum QuantityKind { Dimensionless }
impl Quantity { pub fn new(_: QuantityKind, value: f64) -> Self { Self(value) } }
pub struct CheckResult { status: CheckStatus, explanation: String }
impl CheckResult {
    pub fn assess(_: String, _: &str, _: ClauseId, _: String, _: String) -> Self {
        Self { status: CheckStatus::UnverifiedBasis, explanation: String::new() }
    }
    pub fn annex(self, _: AnnexChoice) -> Self { self }
    pub fn status(mut self, status: CheckStatus) -> Self { self.status = status; self }
    pub fn explanation(mut self, explanation: String) -> Self { self.explanation = explanation; self }
    pub fn minimum(mut self, computed: Quantity, minimum: Quantity) -> Self {
        self.status = if computed.0 >= minimum.0 { CheckStatus::Pass } else { CheckStatus::Fail };
        self
    }
    pub fn build(self) -> Self { self }
}
mod checks {
    use super::*;
    pub fn loc(en: impl Into<String>, _de: impl Into<String>) -> String { en.into() }
    pub fn zone_subject(_: &ZoneDocument, path: String) -> String { path }
    #[path = "KERNEL_PATH"] mod unoccupied_ventilation;
    #[path = "REPORT_PATH"] mod report;
    pub fn outcomes() {
        use AnnexChoice::*;
        use CheckStatus::*;
        use OccupancyType::*;
        use UnoccupiedVentilationOperation::*;
        let mut zone = ZoneDocument { id: "zone", unoccupied_ventilation: None };
        assert!(report::check(&zone, Residential, En).is_none());
        zone.unoccupied_ventilation = Some(UnoccupiedVentilation {
            operation: ResidentialReduced, assessed_area_m2: 80.0, zone_volume_m3: 0.0,
            minutes_before_occupancy: 0.0, residential_design_rate_l_per_s_m2: 0.12, outdoor_air_m3_h: 40.0,
        });
        let result = report::check(&zone, Residential, En).unwrap();
        assert_eq!(result.status, Pass);
        assert!(result.explanation.contains("34.6 m³/h"));
        assert_eq!(report::check(&zone, Residential, De).unwrap().status, UnverifiedBasis);
        assert_eq!(report::check(&zone, Other, En).unwrap().status, UnverifiedBasis);
        zone.unoccupied_ventilation.as_mut().unwrap().outdoor_air_m3_h = 30.0;
        assert_eq!(report::check(&zone, Residential, En).unwrap().status, Fail);
        let record = zone.unoccupied_ventilation.as_mut().unwrap();
        record.operation = NonResidentialPurge;
        record.zone_volume_m3 = 300.0;
        assert_eq!(report::check(&zone, Other, En).unwrap().status, Fail);
        let record = zone.unoccupied_ventilation.as_mut().unwrap();
        record.minutes_before_occupancy = 90.0;
        record.outdoor_air_m3_h = 205.0;
        let result = report::check(&zone, Other, En).unwrap();
        assert_eq!(result.status, Pass);
        assert!(result.explanation.contains("200.0 m³/h"));
    }
}
fn main() { checks::outcomes(); }
'''.replace("KERNEL_PATH", str(kernel)).replace("REPORT_PATH", str(report)))
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "stable", "rustc",
                "--edition", "2024", str(source), "-o", str(binary)], check=True, cwd=root)
subprocess.run([binary], check=True)
print("[DEBUG] DIN EN 16798 unoccupied report outcomes passed direct Rust verification.")
