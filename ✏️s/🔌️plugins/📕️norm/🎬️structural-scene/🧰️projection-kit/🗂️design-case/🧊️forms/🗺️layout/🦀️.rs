//! 🗺️ Layout pass: the default places of the design cases of one document packed by bounding box, shelf by shelf along X, skipping every rectangle that is already taken (the rows of the typed entities of an artifact, other cases), so no two cases and no case and an obstacle overlap. See [`super`].

use super::*;

pub const PACK_WIDTH: f64 = 60.0;

/// 🔲️ Whether two plan rectangles `[x0, x1, y0, y1]` share area; touching edges do not overlap.
pub fn overlaps(a: &[f64; 4], b: &[f64; 4]) -> bool { a[0] < b[1] - 1e-9 && b[0] < a[1] - 1e-9 && a[2] < b[3] - 1e-9 && b[2] < a[3] - 1e-9 }

/// 🗺️ The default origins of the cases of `rows` that declare geometry, packed from `(x0, y0)` in shelves of at most [`PACK_WIDTH`] metres; the origin is the centre of the case.
pub fn origins<C: CaseCatalogue>(rows: &[CaseRow], x0: f64, y0: f64) -> Vec<(String, Vec3)> { origins_with::<C>(rows, x0, y0, &[], &no_extras) }

/// 🗺️ [`origins`] around the plan rectangles `reserved` and for catalogues that hand in `ext` extras: a case that would touch a reserved or placed rectangle moves behind it, and a shelf that would exceed the pack width starts a new one below the deepest case of the shelf.
pub fn origins_with<C: CaseCatalogue>(rows: &[CaseRow], x0: f64, y0: f64, reserved: &[[f64; 4]], ext: Extend<'_>) -> Vec<(String, Vec3)> {
    let drawn: Vec<(&CaseRow, [f64; 4])> = rows.iter().filter_map(|row| extent_with::<C>(row, ext).map(|e| (row, e))).collect();
    let mut taken: Vec<[f64; 4]> = reserved.to_vec();
    let mut out = Vec::with_capacity(drawn.len());
    let (mut x, mut y, mut shelf) = (x0, y0, 0.0_f64);
    for (row, e) in drawn {
        let (width, depth) = (e[1] - e[0], e[3] - e[2]);
        let cell = loop {
            if x > x0 && x + width > x0 + PACK_WIDTH {
                x = x0;
                y += shelf.max(depth);
                shelf = 0.0;
            }
            let cell = [x, x + width, y, y + depth];
            match taken.iter().find(|rect| overlaps(rect, &cell)) {
                Some(rect) => x = rect[1].max(x + 1e-9),
                None => break cell,
            }
        };
        out.push((row.id.clone(), v3(cell[0] - e[0], cell[2] - e[2], 0.0)));
        taken.push(cell);
        x = cell[1];
        shelf = shelf.max(depth);
    }
    out
}
