//! 🌾️ Horizontal seismic pressure of granular material on a silo wall.

/// 🧱️ Identifies the assessed wall relative to one horizontal seismic component.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SiloWall {
    Circular { azimuth_deg: f64 },
    Leeward,
    Windward,
    Parallel,
}

/// 🔻️ The section sets the datum of the wall station and the inclination correction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SiloWallSection {
    VerticalFlatBottom,
    Hopper { inclination_deg: f64 },
}

/// 📐️ A filled horizontal plane with uniform static pressure and a documented wall section.
#[derive(Clone, Copy, Debug)]
pub struct SiloPressureInput {
    pub fill_height_m: f64,
    pub width_m: f64,
    pub height_m: f64,
    pub acceleration_ratio: f64,
    pub bulk_weight_n_m3: f64,
    pub static_pressure_pa: f64,
    pub wall: SiloWall,
    pub section: SiloWallSection,
}

/// 🏗️ Distinguishes an applicable pressure field from one requiring redistribution.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SiloPressure {
    Applicable { reference_pa: f64, additional_pa: f64, total_pa: f64 },
    Redistribution { reference_pa: f64 },
}

/// 📏️ EN 1998-4:2006 §3.3(5)–(9) and (11)–(12) for uniform static pressure at the assessed plane.
pub fn granular_silo_pressure(input: SiloPressureInput) -> Option<SiloPressure> {
    let SiloPressureInput {
        fill_height_m, width_m, height_m, acceleration_ratio, bulk_weight_n_m3, static_pressure_pa, wall, section,
    } = input;
    if ![fill_height_m, width_m, height_m, acceleration_ratio, bulk_weight_n_m3, static_pressure_pa]
        .into_iter().all(f64::is_finite)
        || fill_height_m <= 0.0 || width_m <= 0.0 || bulk_weight_n_m3 <= 0.0
        || height_m < 0.0 || acceleration_ratio < 0.0 || static_pressure_pa < 0.0
    {
        return None;
    }
    let inclination_factor = match section {
        SiloWallSection::VerticalFlatBottom if height_m <= fill_height_m => 1.0,
        SiloWallSection::Hopper { inclination_deg } if inclination_deg > 0.0 && inclination_deg < 90.0 =>
            inclination_deg.to_radians().cos(),
        _ => return None,
    };
    let station_factor = match wall {
        SiloWall::Circular { azimuth_deg } if (0.0..360.0).contains(&azimuth_deg) => azimuth_deg.to_radians().cos(),
        SiloWall::Circular { .. } => return None,
        SiloWall::Leeward => 1.0,
        SiloWall::Windward => -1.0,
        SiloWall::Parallel => 0.0,
    };
    let effective_depth = (width_m / 2.0).min(fill_height_m).min(3.0 * height_m);
    let reference_pa = acceleration_ratio * (bulk_weight_n_m3 * effective_depth) / inclination_factor;
    if !reference_pa.is_finite() {
        return None;
    }
    if static_pressure_pa < reference_pa {
        return Some(SiloPressure::Redistribution { reference_pa });
    }
    let additional_pa = station_factor * reference_pa;
    let total_pa = static_pressure_pa + additional_pa;
    total_pa.is_finite().then_some(SiloPressure::Applicable { reference_pa, additional_pa, total_pa })
}
