#[derive(Clone, Copy)]
enum PageValue { Integer(i64), Rectangle([i64; 4]) }

type PageChange = (&'static str, Option<PageValue>);

impl PageValue {
    fn into_object(self) -> PdfObject {
        match self {
            Self::Integer(value) => PdfObject::Int(value),
            Self::Rectangle(rect) => PdfObject::Array(rect.into_iter().map(PdfObject::Int).collect()),
        }
    }

    fn allocated(self) -> usize {
        match self {
            Self::Integer(_) => 0,
            Self::Rectangle(_) => 4 * size_of::<PdfObject>(),
        }
    }
}

fn patch_page(
    page: &PdfPage, original: &[PdfDictEntry], parent: &[PdfDictEntry], rotate: i32,
) -> Result<[Option<PageChange>; 7], GraftRefusal> {
    let box_fields = [
        ("MediaBox", Some(page.media_box), true), ("CropBox", page.crop_box, true),
        ("TrimBox", page.trim_box, false), ("BleedBox", page.bleed_box, false),
        ("ArtBox", page.art_box, false),
    ];
    let mut changes: [Option<PageChange>; 7] = [None; 7];
    for ((key, current, inherited), target) in box_fields.into_iter().zip(&mut changes) {
        let direct = dict_get(original, key);
        let parent_value = if inherited { dict_get(parent, key) } else { None };
        let old = direct.or(parent_value).map(page_rectangle).transpose()?;
        if old == current { continue; }
        if current.is_none() && parent_value.is_some() { return Err(GraftRefusal::Geometry); }
        let value = current.map(|rect| {
            let [left, bottom, right, top] = rect;
            Ok(PageValue::Rectangle([page_integer(left)?, page_integer(bottom)?, page_integer(right)?, page_integer(top)?]))
        }).transpose()?;
        *target = Some((key, value));
    }
    let old_unit = dict_get(original, "UserUnit").map(page_number).transpose()?;
    if old_unit != page.user_unit {
        let value = page.user_unit.map(page_integer).transpose()?;
        if value.is_some_and(|value| value <= 0) { return Err(GraftRefusal::Geometry); }
        changes[5] = Some(("UserUnit", value.map(PageValue::Integer)));
    }
    if rotate != page.rotate {
        if !matches!(page.rotate, 0 | 90 | 180 | 270) { return Err(GraftRefusal::Rotation); }
        changes[6] = Some(("Rotate", Some(PageValue::Integer(i64::from(page.rotate)))));
    }
    Ok(changes)
}

fn page_integer(value: f64) -> Result<i64, GraftRefusal> {
    if value.fract() != 0.0 || !(-1_099_511_627_776.0..=1_099_511_627_776.0).contains(&value) {
        return Err(GraftRefusal::Geometry);
    }
    Ok(value as i64)
}

fn page_rectangle(value: &PdfObject) -> Result<[f64; 4], GraftRefusal> {
    let PdfObject::Array(coordinates) = value else { return Err(GraftRefusal::Geometry); };
    let [left, bottom, right, top] = coordinates.as_slice() else { return Err(GraftRefusal::Geometry); };
    Ok([page_number(left)?, page_number(bottom)?, page_number(right)?, page_number(top)?])
}

fn page_number(value: &PdfObject) -> Result<f64, GraftRefusal> {
    use std::fmt::Write;
    match value {
        PdfObject::Int(value) => Ok(*value as f64),
        PdfObject::Real(value) if value.coefficient.len() <= 32 && value.scale <= 32 && value.coefficient.bytes().all(|byte| byte.is_ascii_digit()) => {
            let mut text = PageNumberText { bytes: [0; 64], len: 0 };
            write!(&mut text, "{value}").map_err(|_| GraftRefusal::Geometry)?;
            std::str::from_utf8(&text.bytes[..text.len]).ok()
                .and_then(|text| text.parse::<f64>().ok()).filter(|value| value.is_finite())
                .ok_or(GraftRefusal::Geometry)
        }
        _ => Err(GraftRefusal::Geometry),
    }
}

struct PageNumberText { bytes: [u8; 64], len: usize }

impl std::fmt::Write for PageNumberText {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        let end = self.len + text.len();
        self.bytes.get_mut(self.len..end).ok_or(std::fmt::Error)?.copy_from_slice(text.as_bytes());
        self.len = end;
        Ok(())
    }
}

fn replacement_page(
    original: &[PdfDictEntry], changes: &[Option<PageChange>; 7], count: usize,
) -> Vec<PdfDictEntry> {
    let mut entries = Vec::with_capacity(count);
    for entry in original {
        match changes.iter().flatten().find(|(key, _)| *key == entry.key) {
            Some((key, Some(value))) => entries.push(PdfDictEntry::new(*key, value.into_object())),
            Some((_, None)) => {}
            None => entries.push(entry.clone()),
        }
    }
    for (key, value) in changes.iter().flatten() {
        if let Some(value) = value.filter(|_| dict_get(original, key).is_none()) {
            entries.push(PdfDictEntry::new(*key, value.into_object()));
        }
    }
    entries
}
