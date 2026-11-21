use super::*;

#[test]
fn beam_fire_tables_match_the_published_rows_and_axis_interpolation() {
    for (index, line) in include_str!("table-4-1.tsv").lines().skip(1).enumerate() {
        let values: Vec<_> = line.split('\t').collect();
        let row = BEAM_FIRE_ALTERNATIVES[index];
        assert_eq!(row.load_level, values[0].parse::<f64>().unwrap());
        assert_eq!(row.depth_factor, values[1].parse::<f64>().unwrap());
        for (actual, expected) in row.rating_cells.into_iter().zip(&values[2..]) {
            let pair = expected.split_once('/').map(|(width, ratio)| (width.parse().unwrap(), ratio.parse().unwrap()));
            assert_eq!(actual, pair);
        }
    }
    for (index, line) in include_str!("table-4-2.tsv").lines().skip(1).enumerate() {
        let values: Vec<_> = line.split('\t').collect();
        let (width, cells) = BEAM_FIRE_AXES_MM[index];
        assert_eq!(width, values[0].parse::<u16>().unwrap());
        for (actual, expected) in cells.into_iter().zip(&values[1..]) {
            let pair = expected.split_once('/').map(|(side, bottom)| (side.parse().unwrap(), bottom.parse().unwrap()));
            assert_eq!(actual, pair);
        }
    }
    assert_eq!(beam_load_level(0.51), Some(0.70));
    assert_eq!(beam_load_level(0.71), None);
    assert!(beam_reinforcement_axes(120, 0.199).is_none());
    assert_eq!(beam_reinforcement_axes(180, 0.250), Some((0.120, 0.060)));
    let (u1, u2) = beam_reinforcement_axes(90, 0.225).unwrap();
    assert!((u1 - 0.0875).abs() < 1e-12);
    assert!((u2 - 0.0525).abs() < 1e-12);
}
