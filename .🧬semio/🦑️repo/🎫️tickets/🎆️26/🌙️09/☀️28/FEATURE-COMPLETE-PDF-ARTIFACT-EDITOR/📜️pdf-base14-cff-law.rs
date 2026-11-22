use std::{collections::BTreeMap, path::Path};
use truetype::TrueTypeOutlineSegment::*;

fn main() {
    let directory = std::env::args_os().nth(1).unwrap();
    for &(font, program, probes) in FACES {
        let data = std::fs::read(Path::new(&directory).join(format!("{program}.cff"))).unwrap();
        let owner = cff::CffOutlineFont::new(&data).unwrap_or_else(|| panic!("{font} CFF source refused"));
        let mut glyphs = BTreeMap::new();
        owner.visit_glyph_names(|name, id| { assert!(glyphs.insert(name, id).is_none(), "{font} duplicate name {name}"); }).unwrap();
        for &(name, code) in probes {
            let glyph = *glyphs.get(name).unwrap_or_else(|| panic!("{font} missing {name}"));
            let outline = owner.raw_outline(glyph).unwrap_or_else(|| panic!("{font} /{name} outline refused"));
            assert!(!outline.is_empty());
            let (mut min, mut max) = ([f64::INFINITY; 2], [f64::NEG_INFINITY; 2]);
            let mut point = [0.0; 2];
            let mut include = |position: [f64; 2]| {
                for axis in 0..2 {
                    min[axis] = min[axis].min(position[axis]);
                    max[axis] = max[axis].max(position[axis]);
                }
            };
            for segment in outline {
                point = match segment {
                    MoveTo(to) | LineTo(to) => { include(to); to },
                    CubicTo { first, second, to } => {
                        for part in 1..=512 {
                            let t = f64::from(part) / 512.0;
                            let u = 1.0 - t;
                            include(std::array::from_fn(|axis|
                                u*u*u*point[axis] + 3.0*u*u*t*first[axis] + 3.0*u*t*t*second[axis] + t*t*t*to[axis]));
                        }
                        to
                    }
                    Close => point,
                    QuadraticTo { .. } => panic!("unexpected quadratic contour in CFF"),
                };
            }
            println!("{font}\t{name}\t{code}\t{glyph}\t{}\t{}\t{}\t{}", min[0], min[1], max[0], max[1]);
        }
    }
}
