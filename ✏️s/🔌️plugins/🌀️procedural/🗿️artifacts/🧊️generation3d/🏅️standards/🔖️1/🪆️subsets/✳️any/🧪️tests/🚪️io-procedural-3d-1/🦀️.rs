//! 🚪️ Generation3d 1 IO case — Rust subject adapter. Ticket
//! `26/09/09/PROCEDURAL-3D-END-TO-END`.
//!
//! **What this half asserts, and what it deliberately does not.** A generated test host may not gain
//! a Cargo dependency on a plugin crate, so — exactly as this subset's sibling `🧊️mutate-procedural-3d-1`
//! records for its own vocabulary — the subject half here does NOT run `s.procedural.generation3d`'s
//! codec. It replays the committed specification vector (`🧫️fixtures/🚪️io/🧊️unit-cube/🔣️.json`, the
//! cube every lane is driven from), computes the projection that vector implies with its own
//! arithmetic, and checks the file-level laws of each committed encoding that need no grammar: the
//! format's own magic, and a record count the encoding must expose in plain sight.
//!
//! The grammar itself is the ORACLE's job. `🐍️.py` beside this file PARSES each committed encoding
//! with an independent Python implementation of the ASCII STL / Wavefront OBJ / ASCII PLY grammars
//! and returns its own projection; the coordinator compares the two under `semantic-mesh-v1`. So a
//! file that carried the right magic but the wrong geometry passes this half and fails the
//! comparison — which is the whole point of running the two halves in different languages.
//!
//! **Where this artifact's real codec IS exercised.** In-crate, by `[[test]] io-round-trip`
//! (`🚪️io/🧪️tests/🔁️round-trip/🦀️.rs`): the same cube goes out through every export leaf and back
//! through every import leaf, with `parry3d` — a registered third-party test-lane library —
//! recomputing the enclosed volume and bounds of each recovered mesh. That lane additionally feeds
//! this case's Python-written committed files through the real import leaves, so the two languages
//! meet on both sides of the boundary.
//!
//! @see ../🧊️mutate-procedural-3d-1/🦀️.rs — the sibling case whose structure this one mirrors.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Vocabulary
/// 🗂️ Per format: the scenario suffix, the committed encoding, the byte prefix that identifies the
/// container, the token whose occurrence count the encoding must expose, how many of them the cube
/// implies, and the vertex count the format's own model can carry.
const FORMATS: &[(&str, &str, &str, &str, usize, usize)] = &[
    ("stl", "shared://🚪️io/🧊️unit-cube/🔺️second-implementation.stl", "solid", "facet normal", 12, 36),
    ("obj", "shared://🚪️io/🧊️unit-cube/🗿️second-implementation.obj", "o ", "\nf ", 12, 8),
    ("ply", "shared://🚪️io/🧊️unit-cube/🧱️second-implementation.ply", "ply", "\n3 ", 12, 8),
];

/// 🧊️ The committed specification vector every format is measured against.
const CUBE: &str = "shared://🚪️io/🧊️unit-cube/🔣️.json";
//#endregion 🔖️Vocabulary

//#region 🔖️Geometry
/// 🔢️ A `Json` array of numbers as `f64`s; a non-number member is a malformed vector, not a zero.
fn numbers(value: &Json, key: &str) -> Result<Vec<f64>, String> {
    value
        .array(key)
        .iter()
        .map(|entry| match entry {
            Json::Number(number) => Ok(*number),
            other => Err(format!("the committed cube's {key} carries {other:?}, which is not a number")),
        })
        .collect()
}

/// 📦️ The enclosed volume of a closed triangle soup, by the divergence theorem — one sixth of the
/// summed scalar triple product of each triangle's three corners. Positive for outward winding, so a
/// reversed face shows up here rather than hiding behind an unchanged triangle count.
fn signed_volume(vertices: &[[f64; 3]], triangles: &[[usize; 3]]) -> f64 {
    let mut total = 0.0;
    for triangle in triangles {
        let (a, b, c) = (vertices[triangle[0]], vertices[triangle[1]], vertices[triangle[2]]);
        total += a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0]) + a[2] * (b[0] * c[1] - b[1] * c[0]);
    }
    total / 6.0
}

/// 🧊️ The committed cube, read out of its own vector.
fn cube(ctx: &Context) -> Result<(Vec<[f64; 3]>, Vec<[usize; 3]>), String> {
    let document = ctx.fixture_json(CUBE)?;
    let flat = numbers(&document, "positions")?;
    let raw = numbers(&document, "indices")?;
    if flat.len() != 24 || raw.len() != 36 {
        return Err(format!("the committed cube declares {} position floats and {} indices; this case is driven from a unit cube (24 and 36)", flat.len(), raw.len()));
    }
    let vertices: Vec<[f64; 3]> = flat.chunks_exact(3).map(|p| [p[0], p[1], p[2]]).collect();
    let triangles: Vec<[usize; 3]> = raw.chunks_exact(3).map(|t| [t[0] as usize, t[1] as usize, t[2] as usize]).collect();
    Ok((vertices, triangles))
}

/// 🔁️ The shape both halves answer with.
fn projection(format: &str, vertex_count: usize, vertices: &[[f64; 3]], triangles: &[[usize; 3]]) -> Json {
    let axis = |pick: fn(f64, f64) -> f64, seed: f64, index: usize| vertices.iter().fold(seed, |carry, vertex| pick(carry, vertex[index]));
    Json::Object(vec![
        ("format".to_string(), Json::String(format.to_string())),
        ("triangleCount".to_string(), Json::Number(triangles.len() as f64)),
        ("vertexCount".to_string(), Json::Number(vertex_count as f64)),
        ("min".to_string(), Json::Array((0..3).map(|index| Json::Number(axis(f64::min, f64::INFINITY, index))).collect())),
        ("max".to_string(), Json::Array((0..3).map(|index| Json::Number(axis(f64::max, f64::NEG_INFINITY, index))).collect())),
        ("volume".to_string(), Json::Number(signed_volume(vertices, triangles))),
    ])
}
//#endregion 🔖️Geometry

//#region 🔖️Handlers
/// 📄️ The committed encoding really is a file of the named format, checked by the two things a
/// reader-free half can check: the container's own opening token, and the count of the record the
/// grammar repeats once per triangle.
fn read_handler(format: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| {
        let (_, uri, magic, token, records, vertex_count) = *FORMATS.iter().find(|entry| entry.0 == format).expect("registered format");
        let bytes = ctx.fixture_bytes(uri)?;
        let text = String::from_utf8(bytes.clone()).map_err(|error| format!("read-{format}: the committed encoding is not utf-8: {error}"))?;
        if !text.starts_with(magic) {
            return Err(format!("read-{format}: the committed encoding does not open with {magic:?}"));
        }
        let found = text.matches(token).count();
        if found != records {
            return Err(format!("read-{format}: the committed encoding exposes {found} {token:?} records, and the committed cube has {records} triangles"));
        }
        let (vertices, triangles) = cube(ctx)?;
        Ok(Outcome::with_raw(bytes, projection(format, vertex_count, &vertices, &triangles)))
    }
}

/// 🔁️ The round-trip law, from the vector's side: writing and re-reading the cube in a format must
/// not move the geometry the format is able to carry. The subject half has no writer, so what it
/// answers with is the vector's own projection — the reference the oracle's write/read cycle has to
/// land on.
fn round_trip_handler(format: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |ctx: &Context| {
        let (_, _, _, _, records, vertex_count) = *FORMATS.iter().find(|entry| entry.0 == format).expect("registered format");
        let (vertices, triangles) = cube(ctx)?;
        if triangles.len() != records {
            return Err(format!("round-trip-{format}: the committed cube has {} triangles, this case expects {records}", triangles.len()));
        }
        let volume = signed_volume(&vertices, &triangles);
        if (volume - 1.0).abs() > 1e-9 {
            return Err(format!("round-trip-{format}: the committed cube encloses {volume}, not the unit volume this case is built on — its winding or its coordinates moved"));
        }
        Ok(Outcome::projection(projection(format, vertex_count, &vertices, &triangles)))
    }
}
//#endregion 🔖️Handlers

//#region 🔖️Registration
/// 🧭️ Registration by FULL expanded scenario id, in the SUBJECT role only — the reference half is
/// registered as the oracle in `🐍️.py`, and registering either one in both roles would manufacture a
/// green self-comparison.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for (format, ..) in FORMATS {
        built = built.subject(&format!("read-{format}"), read_handler(format)).subject(&format!("round-trip-{format}"), round_trip_handler(format));
    }
    built
}
//#endregion 🔖️Registration
