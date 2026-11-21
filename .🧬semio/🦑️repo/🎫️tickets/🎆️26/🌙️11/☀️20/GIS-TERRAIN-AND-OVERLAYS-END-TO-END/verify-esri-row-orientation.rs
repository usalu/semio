//! 🧭 Verifies the actual Terrain ASCII codec with the signed canonical grid shape.

mod schema {
    pub mod snapshot {
        #[derive(Debug, PartialEq)]
        pub struct ElevationSource {
            pub bounds: [f64; 4],
            pub rows: Vec<Vec<i16>>,
        }

        impl ElevationSource {
            pub fn is_valid(&self) -> bool {
                let [west, south, east, north] = self.bounds;
                (-180.0..east).contains(&west)
                    && east <= 180.0
                    && (-90.0..north).contains(&south)
                    && north <= 90.0
                    && self.rows.len() == 33
                    && self.rows.iter().all(|row| row.len() == 33)
            }
        }
    }
}

#[path = "../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🏔️ascii-grid/🦀️.rs"]
mod ascii_grid;

fn main() {
    let file = include_str!("../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️import-elevation/🧪️tests/🧫️fixtures/🔼️slope.asc");
    let source = ascii_grid::decode(file, 24 * 1024).expect("read the Esri survey");
    assert_eq!(source.bounds, [6.625, 44.625, 7.125, 45.125]);
    assert_eq!(source.rows[0][0], 32, "southwest");
    assert_eq!(source.rows[0][32], 128, "southeast");
    assert_eq!(source.rows[32][0], -32, "northwest");
    assert_eq!(source.rows[32][32], 64, "northeast");
    let output = ascii_grid::encode(&source).expect("write the Esri survey");
    assert!(output.lines().nth(6).unwrap().starts_with("-32 -29 -26 "));
    assert!(output.lines().last().unwrap().starts_with("32 35 38 "));
    assert_eq!(ascii_grid::decode(&output, 24 * 1024).unwrap(), source);
    println!("[DEBUG] Terrain ASCII orientation and round-trip passed.");
}
