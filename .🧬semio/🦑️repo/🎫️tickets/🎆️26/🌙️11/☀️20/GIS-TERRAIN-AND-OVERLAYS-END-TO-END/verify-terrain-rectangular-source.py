"""🏔️ Build a standalone probe from the current first-party codec and elevation edit sources."""

import json
from pathlib import Path
import sys


TICKET = Path(__file__).resolve().parent
terrain = Path(sys.argv[1])
CODEC = (terrain / "🚪️io/📝️text/🏔️ascii-grid/🦀️.rs").read_text(encoding="utf-8")
REDUCER = (terrain / "🧬️schema/💡️inferences/📐️elevation-edit/🦀️.rs").read_text(encoding="utf-8")
fixture = json.loads((TICKET / "terrain-fractional-grid.json").read_text(encoding="utf-8"))
expected = "vec![" + ",".join("vec!" + str(row) for row in fixture["elevation"]["millimetres"]) + "]"
stub = r'''
extern crate self as semio_framework_value;

#[derive(Debug)]
pub struct ValueError;
pub enum ValueRefusalKind { InvalidValue }

impl ValueError {
    pub fn literal(_: ValueRefusalKind, _: &str) -> Self { Self }
}

mod schema {
    pub mod snapshot {
        use crate::ValueError;
        pub const ELEVATION_MIN_SIDE: usize = 2;
        pub const ELEVATION_MAX_SIDE: usize = 65;

        #[derive(Clone, Debug, PartialEq)]
        pub struct ElevationSource {
            pub bounds: [f64; 4],
            pub rows_millimetres: Vec<Vec<i32>>,
        }

        impl ElevationSource {
            pub fn dimensions(&self) -> Option<(usize, usize)> {
                let width = self.rows_millimetres.first()?.len();
                let height = self.rows_millimetres.len();
                ((2..=65).contains(&width) && (2..=65).contains(&height)
                    && self.rows_millimetres.iter().all(|row| row.len() == width))
                    .then_some((width, height))
            }

            pub fn is_valid(&self) -> bool {
                let [west, south, east, north] = self.bounds;
                self.dimensions().is_some() && (-180.0..east).contains(&west)
                    && east <= 180.0 && (-90.0..north).contains(&south) && north <= 90.0
            }

            pub fn validate(&self) -> Result<(), crate::ValueError> {
                self.is_valid().then_some(()).ok_or(ValueError)
            }
        }
    }
}
'''
main = r'''
use elevation_edit::{ElevationEdit, ElevationEditMode, metres_to_millimetres};
use schema::snapshot::ElevationSource;

fn main() {
    let raster = r#"__FIXTURE__"#;
    let source = ascii_grid::decode(raster, 64 * 1024).unwrap();
    assert_eq!(source.rows_millimetres, __EXPECTED__);
    assert_eq!(source.dimensions(), Some((4, 3)));
    assert_eq!(ascii_grid::decode(&ascii_grid::encode(&source).unwrap(), 64 * 1024).unwrap(), source);

    assert_eq!(metres_to_millimetres(-2_147_483.648), Some(i32::MIN));
    assert_eq!(metres_to_millimetres(2_147_483.647), Some(i32::MAX));
    assert_eq!(metres_to_millimetres(0.0009), None);
    assert_eq!(metres_to_millimetres(f64::INFINITY), None);
    let edit = ElevationEdit {
        mode: ElevationEditMode::SetSample, column: 2, row: 1, radius: 0,
        value: metres_to_millimetres(12.125).unwrap(),
    };
    let changed = edit.apply(&source).unwrap();
    assert_eq!(changed.rows_millimetres[1][2], 12125);
    assert_eq!(source.rows_millimetres[1][2], 3333);
    assert!(ElevationEdit { row: 3, ..edit }.apply(&source).is_err());

    let mut large = ElevationSource {
        bounds: [0.0, 0.0, 0.5, 0.5],
        rows_millimetres: vec![vec![90_000; 65]; 65],
    };
    let smooth = ElevationEdit { mode: ElevationEditMode::Smooth, row: 32, column: 0, radius: 4, value: 100 };
    let anchors = [[0, 32], [64, 32]];
    assert!(smooth.stroke_work(&large, &anchors).unwrap() < 2_000_000);
    assert_eq!(smooth.apply_stroke(&large, &anchors).unwrap(), large);
    large.rows_millimetres[0][0] = i32::MAX;
    let raise = ElevationEdit { mode: ElevationEditMode::RaiseLower, row: 0, column: 0, radius: 0, value: 1 };
    assert!(raise.apply(&large).is_err());
    assert_eq!(large.rows_millimetres[0][0], i32::MAX);
    let extreme = ElevationSource {
        bounds: [0.0, 0.0, 0.5, 0.5],
        rows_millimetres: vec![vec![i32::MIN; 65]; 65],
    };
    let ascii = ascii_grid::encode(&extreme).unwrap();
    assert!(ascii.len() > 32 * 1024 && ascii.len() <= 64 * 1024);
    let mut reader = ascii_grid::GridReader::new(&ascii, 64 * 1024).unwrap();
    assert_eq!(reader.dimensions, [65, 65]);
    for _ in 0..65 {
        assert_eq!(reader.next_row(&ascii).unwrap().unwrap(), vec![i32::MIN; 65]);
    }
    assert_eq!(reader.next_row(&ascii).unwrap(), None);
    assert_eq!(ascii_grid::decode(&ascii, 64 * 1024), Ok(extreme));
    println!("[DEBUG] Exact rectangular ASCII and elevation edit source probes passed.");
}
'''.replace("__FIXTURE__", fixture["asciiGrid"]).replace("__EXPECTED__", expected)
path = TICKET / "🗑️generated" / "rectangular-terrain-probe.rs"
path.write_text(stub + "\nmod ascii_grid {\n" + CODEC + "\n}\nmod elevation_edit {\n" + REDUCER + "\n}\n" + main, encoding="utf-8")
