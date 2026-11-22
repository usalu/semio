//! ⛳️ Runs the actual Terrain ASCII codec against the independent picker fixture on stdin.
mod schema {
    pub mod snapshot {
        pub const ELEVATION_MIN_SIDE: usize = 2;
        pub const ELEVATION_MAX_SIDE: usize = 65;

        #[derive(Debug, PartialEq)]
        pub struct ElevationSource {
            pub bounds: [f64; 4],
            pub rows_millimetres: Vec<Vec<i32>>,
        }

        impl ElevationSource {
            pub fn dimensions(&self) -> Option<(usize, usize)> {
                Some((self.rows_millimetres.first()?.len(), self.rows_millimetres.len()))
            }

            pub fn is_valid(&self) -> bool {
                let Some((columns, rows)) = self.dimensions() else { return false };
                let [west, south, east, north] = self.bounds;
                (2..=65).contains(&columns) && (2..=65).contains(&rows)
                    && self.rows_millimetres.iter().all(|row| row.len() == columns)
                    && (-180.0..east).contains(&west) && east <= 180.0
                    && (-90.0..north).contains(&south) && north <= 90.0
            }
        }
    }
}

#[path = "../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/🏔️ascii-grid/🦀️.rs"]
mod ascii_grid;

use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let size = input.len();
    assert_eq!(size, 64 * 1024);
    let decoded = ascii_grid::decode(&input, size).unwrap();
    assert_eq!(decoded.bounds, [6.625, 44.625, 6.75, 44.75]);
    assert_eq!(decoded.rows_millimetres, std::iter::repeat_n(vec![i32::MIN; 65], 64)
        .chain([vec![i32::MAX; 65]]).collect::<Vec<_>>());
    assert_eq!(ascii_grid::decode(&input, size - 1), Err(ascii_grid::Error::FileSize));
    let exported = ascii_grid::encode(&decoded).unwrap();
    assert_eq!(ascii_grid::decode(&exported, size).unwrap(), decoded);
    println!("Terrain ASCII codec: {size} bytes, i32 endpoints and exact canonical round trip");
}
