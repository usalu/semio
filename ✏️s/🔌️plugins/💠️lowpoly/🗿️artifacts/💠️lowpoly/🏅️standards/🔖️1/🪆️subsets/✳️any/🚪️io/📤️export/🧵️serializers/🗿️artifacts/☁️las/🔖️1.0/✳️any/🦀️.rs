//! lowpoly -> las
//!
//! Real LAS 1.x export through `io::encode_las`: every world-space mesh vertex becomes one point
//! record.
//!
//! 🔖 `IoFidelity::Lossy`: a point cloud of the vertices — faces, objects and paint do not survive,
//! so there is no las import.
use crate::io::mesh_geometry::world_parts;
use crate::schema::snapshot::LowpolySnapshot;
use semio_s_artifact_stdio_las::io::encode_las;
use semio_s_artifact_stdio_las::schema::snapshot::{LasHeader, LasPoint};
use semio_s_artifact_stdio_las::LasSnapshot;

pub fn register() {}

pub fn serialize(snapshot: &LowpolySnapshot) -> Result<LasSnapshot, store::TextError> {
    let mut points = Vec::new();
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for part in world_parts("las", snapshot)? {
        for p in &part.positions {
            for i in 0..3 {
                min[i] = min[i].min(p[i]);
                max[i] = max[i].max(p[i]);
            }
            points.push(LasPoint {
                x: p[0],
                y: p[1],
                z: p[2],
                intensity: 0,
                return_number: 1,
                number_of_returns: 1,
                scan_direction_flag: false,
                edge_of_flight_line: false,
                classification: 0,
                scan_angle_rank: 0,
                user_data: 0,
                point_source_id: 0,
                gps_time: None,
                rgb: None,
            });
        }
    }
    if min[0].is_infinite() {
        min = [0.0; 3];
        max = [0.0; 3];
    }
    let mut header = LasHeader::default();
    header.system_identifier = "semio.lowpoly".into();
    header.generating_software = "semio.lowpoly".into();
    header.min_x = min[0];
    header.min_y = min[1];
    header.min_z = min[2];
    header.max_x = max[0];
    header.max_y = max[1];
    header.max_z = max[2];
    header.number_of_point_records = points.len() as u32;
    Ok(LasSnapshot {
        header,
        points,
        ..Default::default()
    })
}

pub fn serialize_bytes(snapshot: &LowpolySnapshot) -> Result<Vec<u8>, store::TextError> {
    encode_las(&serialize(snapshot)?).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
}
