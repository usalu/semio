#[derive(Clone, Copy)]
struct Composite { owner: ObjRef, default_width: f64, identity: bool }

struct FontDependencies { owners: [ObjRef; 4], count: usize }

impl FontDependencies {
    fn owners(&self) -> &[ObjRef] { &self.owners[..self.count] }
}

fn string_bytes(value: &PdfObject) -> Option<&[u8]> {
    match value {
        PdfObject::Text(text) => Some(text.as_bytes()),
        _ => value.as_str_bytes(),
    }
}

fn cid_font(value: &PdfObject, base_font: &str) -> Option<(ObjRef, f64, bool, u32, usize)> {
    let dict = value.as_dict()?;
    let keys = ["Type", "Subtype", "BaseFont", "CIDSystemInfo", "FontDescriptor", "CIDToGIDMap", "W", "DW"];
    if dict.len() > keys.len() || dict.iter().enumerate().any(|(index, entry)| {
        !keys.contains(&entry.key.as_str()) || dict[..index].iter().any(|before| before.key == entry.key)
    }) { return None; }
    if name(dict, "Type")? != "Font" || name(dict, "Subtype")? != "CIDFontType2" || name(dict, "BaseFont")? != base_font { return None; }
    let info = qualified(get(dict, "CIDSystemInfo")?, &["Registry", "Ordering", "Supplement"])?;
    if string_bytes(get(info, "Registry")?)? != b"Adobe" || string_bytes(get(info, "Ordering")?)? != b"Identity"
        || integer(info, "Supplement")? != 0 { return None; }
    let descriptor = reference(dict, "FontDescriptor")?;
    let identity = match get(dict, "CIDToGIDMap") {
        None => false,
        Some(owner) if owner.as_name() == Some("Identity") => true,
        _ => return None,
    };
    let default_width = match get(dict, "DW") {
        Some(value) => value.as_f64().filter(|value| value.is_finite() && *value >= 0.)?,
        None => 1000.,
    };
    let Some(widths) = get(dict, "W") else { return Some((descriptor, default_width, identity, 0, 0)); };
    let [first, array] = widths.as_array()? else { return None; };
    let first = u16::try_from(first.as_i64()?).ok()?;
    let count = array.as_array()?.len();
    if count == 0 || count > MAPPINGS || usize::from(first) + count > u16::MAX as usize + 1 { return None; }
    Some((descriptor, default_width, identity, u32::from(first), count))
}
