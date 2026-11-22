//! ☀️ Potsdam reference irradiance and orientation and tilt selection, DIN V 18599-10 Annex E.

use semio_s_artifact_norm_din18599_model::POTSDAM_HORIZONTAL_IRRADIANCE_W_M2;

const TILTS_DEG: [f64; 5] = [0.0, 30.0, 45.0, 60.0, 90.0];

/// 🧭️ North, north-east, east, south-east, south, south-west, west and north-west, at 30°, 45°, 60° and 90°.
const TILTED_IRRADIANCE_W_M2: [[[u16; 12]; 4]; 8] = [
    [
        [16, 29, 56, 128, 172, 197, 175, 129, 77, 36, 21, 11],
        [15, 26, 43, 90, 136, 161, 145, 95, 56, 33, 19, 10],
        [13, 24, 39, 71, 101, 119, 113, 72, 50, 30, 17, 9],
        [10, 18, 31, 58, 75, 83, 81, 57, 41, 25, 13, 7],
    ],
    [
        [17, 34, 71, 151, 185, 209, 187, 144, 93, 50, 22, 12],
        [15, 29, 61, 131, 160, 181, 167, 123, 79, 42, 20, 11],
        [14, 26, 54, 114, 139, 157, 148, 107, 68, 36, 18, 9],
        [11, 19, 41, 87, 104, 116, 112, 81, 52, 29, 13, 7],
    ],
    [
        [31, 43, 95, 189, 211, 231, 205, 173, 122, 77, 30, 17],
        [31, 41, 91, 181, 198, 217, 194, 163, 115, 74, 28, 16],
        [30, 38, 85, 170, 180, 198, 179, 150, 106, 70, 26, 15],
        [25, 29, 68, 134, 137, 150, 138, 115, 83, 55, 20, 12],
    ],
    [
        [46, 52, 114, 214, 227, 242, 212, 194, 147, 102, 38, 23],
        [51, 53, 116, 212, 217, 229, 201, 188, 148, 107, 39, 25],
        [54, 51, 112, 201, 198, 207, 183, 175, 141, 107, 38, 26],
        [50, 42, 90, 156, 143, 146, 132, 130, 111, 91, 32, 23],
    ],
    [
        [50, 55, 121, 217, 230, 241, 208, 199, 157, 110, 41, 26],
        [57, 56, 124, 214, 218, 224, 194, 193, 160, 119, 44, 29],
        [61, 55, 121, 201, 196, 197, 172, 178, 155, 121, 44, 31],
        [59, 47, 98, 147, 132, 124, 113, 127, 123, 106, 39, 29],
    ],
    [
        [40, 49, 110, 201, 222, 234, 201, 188, 145, 96, 37, 23],
        [43, 48, 110, 195, 209, 218, 188, 181, 145, 99, 38, 24],
        [44, 46, 105, 181, 190, 195, 169, 167, 138, 97, 37, 25],
        [40, 36, 83, 136, 137, 135, 120, 123, 108, 80, 31, 22],
    ],
    [
        [25, 40, 90, 172, 202, 219, 188, 165, 120, 70, 29, 16],
        [24, 36, 84, 159, 187, 201, 174, 153, 112, 65, 27, 16],
        [22, 33, 78, 146, 169, 181, 157, 139, 103, 60, 25, 14],
        [17, 24, 60, 114, 127, 136, 117, 105, 79, 47, 19, 11],
    ],
    [
        [16, 32, 68, 139, 178, 199, 173, 138, 91, 47, 22, 12],
        [15, 28, 58, 116, 151, 169, 149, 116, 77, 40, 20, 11],
        [13, 25, 50, 101, 130, 144, 128, 99, 66, 35, 18, 9],
        [11, 18, 38, 78, 96, 108, 95, 74, 51, 28, 13, 7],
    ],
];

fn tilted_month(month: usize, direction: usize, tilt: f64, bracket: usize) -> f64 {
    let horizontal = POTSDAM_HORIZONTAL_IRRADIANCE_W_M2[month];
    let lower = if bracket == 0 { horizontal } else { f64::from(TILTED_IRRADIANCE_W_M2[direction][bracket - 1][month]) };
    let upper = f64::from(TILTED_IRRADIANCE_W_M2[direction][bracket][month]);
    let lower_angle = TILTS_DEG[bracket];
    let upper_angle = TILTS_DEG[bracket + 1];
    lower + (upper - lower) * (tilt - lower_angle) / (upper_angle - lower_angle)
}

/// 📐 Monthly mean irradiance for an azimuth measured clockwise from north. DIN's 2024 Annex E interpretation sets compass and tilt selection.
pub fn potsdam_irradiance_w_m2(month: usize, azimuth_deg: f64, tilt_deg: f64) -> Option<f64> {
    if month >= 12 || !azimuth_deg.is_finite() || !(0.0..=90.0).contains(&tilt_deg) {
        return None;
    }
    if tilt_deg == 0.0 {
        return Some(POTSDAM_HORIZONTAL_IRRADIANCE_W_M2[month]);
    }
    let bracket = TILTS_DEG[1..].partition_point(|&angle| angle < tilt_deg);
    let sector = azimuth_deg.rem_euclid(360.0) / 45.0;
    let lower_direction = sector.floor() as usize % 8;
    let upper_direction = (lower_direction + 1) % 8;
    let irradiance = match sector.fract().total_cmp(&0.5) {
        std::cmp::Ordering::Less => tilted_month(month, lower_direction, tilt_deg, bracket),
        std::cmp::Ordering::Greater => tilted_month(month, upper_direction, tilt_deg, bracket),
        std::cmp::Ordering::Equal => (
            tilted_month(month, lower_direction, tilt_deg, bracket) + tilted_month(month, upper_direction, tilt_deg, bracket)
        ) / 2.0,
    };
    Some(irradiance)
}
