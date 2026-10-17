//! 🔤️ Font descriptors, programs, ordered encodings, CID metrics and CMaps.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1_7::subsets::base::io::sqlite::snapshot::*;

fn write_encoding(out: &mut Projection<'_, '_>, encoding: &PdfSimpleEncoding) -> Result<i64,ValueError> {
    let base = encoding.base.as_ref().map(|base| match base { PdfBaseEncoding::Standard => "standard", PdfBaseEncoding::WinAnsi => "winAnsi", PdfBaseEncoding::MacRoman => "macRoman", PdfBaseEncoding::MacExpert => "macExpert" });
    let key = out.insert("pdf_font_encoding", &[base.map_or(C::Null, C::Text)])?;
    for (ordinal, difference) in encoding.differences.iter().enumerate() { out.insert("pdf_encoding_difference", &[C::Integer(key), C::Integer(ordinal as i64), C::Integer(i64::from(difference.code)), C::Text(&difference.glyph)])?; }
    Ok(key)
}

fn read_encoding(reader: &mut Reader<'_, '_, '_>, key: i64) -> Result<PdfSimpleEncoding,ValueError> {
    let row = reader.take("pdf_font_encoding", key, 2)?;
    let base = row.optional_text(1)?.map(|base| match base { "standard" => Ok(PdfBaseEncoding::Standard), "winAnsi" => Ok(PdfBaseEncoding::WinAnsi), "macRoman" => Ok(PdfBaseEncoding::MacRoman), "macExpert" => Ok(PdfBaseEncoding::MacExpert), _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown PDF font base encoding")) }).transpose()?;
    let mut differences = Vec::new();
    for row in reader.children("pdf_encoding_difference", 1, 2, key)? { let row = reader.take("pdf_encoding_difference", row.rowid, 5)?; differences.push(PdfEncodingDifference { code: integer(row, 3)?, glyph: reader.text(row,4)? }); }
    Ok(PdfSimpleEncoding { base, differences })
}

fn write_descriptor(out: &mut Projection<'_, '_>, value: &PdfFontDescriptor) -> Result<i64,ValueError> {
    let extra = cos::write_dictionary(out, &value.extra)?;
    let mut cells = vec![C::Text(&value.font_name), C::Integer(i64::from(value.flags))]; cells.extend(value.font_bbox.iter().copied().map(C::Real));
    cells.extend([C::Real(value.italic_angle), C::Real(value.ascent), C::Real(value.descent), C::Real(value.cap_height), C::Real(value.stem_v), real_cell(value.stem_h), real_cell(value.x_height), real_cell(value.leading), real_cell(value.avg_width), real_cell(value.max_width), real_cell(value.missing_width), text_cell(&value.font_family), text_cell(&value.font_stretch), real_cell(value.font_weight), text_cell(&value.char_set), C::Integer(extra)]);
    out.insert("pdf_font_descriptor", &cells)
}

fn read_descriptor(reader: &mut Reader<'_, '_, '_>, key: i64) -> Result<PdfFontDescriptor,ValueError> {
    let row = reader.take("pdf_font_descriptor", key, 23)?;
    Ok(PdfFontDescriptor { font_name: reader.text(row,1)?, flags: integer(row, 2)?, font_bbox: [row.real(3)?, row.real(4)?, row.real(5)?, row.real(6)?], italic_angle: row.real(7)?, ascent: row.real(8)?, descent: row.real(9)?, cap_height: row.real(10)?, stem_v: row.real(11)?, stem_h: optional_real(row, 12)?, x_height: optional_real(row, 13)?, leading: optional_real(row, 14)?, avg_width: optional_real(row, 15)?, max_width: optional_real(row, 16)?, missing_width: optional_real(row, 17)?, font_family: reader.optional_text(row,18)?, font_stretch: reader.optional_text(row,19)?, font_weight: optional_real(row, 20)?, char_set: reader.optional_text(row,21)?, extra: cos::read_dictionary(reader, row.integer(22)?)? })
}

pub(super) fn write_program(out:&mut Projection<'_, '_>,program:&PdfFontProgram)->Result<i64,ValueError>{let kind=match program {PdfFontProgram::Type1 {..}=>"type1",PdfFontProgram::TrueType {..}=>"trueType",PdfFontProgram::Cff {..}=>"cff",PdfFontProgram::CidCff {..}=>"cidCff",PdfFontProgram::OpenType {..}=>"openType"};let reference=artifact_reference::write(out,program.reference())?;out.insert("pdf_font_program",&[C::Text(kind),C::Integer(reference)])}
pub(super) fn read_program(reader:&mut Reader<'_, '_, '_>,key:i64)->Result<PdfFontProgram,ValueError>{let row=reader.take("pdf_font_program",key,3)?;let reference=artifact_reference::read(reader,row.integer(2)?)?;Ok(match row.text(1)? {"type1"=>PdfFontProgram::Type1 {reference},"trueType"=>PdfFontProgram::TrueType {reference},"cff"=>PdfFontProgram::Cff {reference},"cidCff"=>PdfFontProgram::CidCff {reference},"openType"=>PdfFontProgram::OpenType {reference},_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown PDF font program kind"))})}

pub(super) fn write_unicode(out: &mut Projection<'_, '_>, cmap: &PdfToUnicode) -> Result<i64,ValueError> {
    let key = out.insert("pdf_to_unicode", &[C::Integer(i64::from(cmap.byte_width))])?;
    for (ordinal, mapping) in cmap.mappings.iter().enumerate() {
        let (kind, code, low, high, text) = match mapping { PdfToUnicodeMapping::Char { code, text } => ("char", C::Integer(i64::from(*code)), C::Null, C::Null, text), PdfToUnicodeMapping::Range { low, high, text } => ("range", C::Null, C::Integer(i64::from(*low)), C::Integer(i64::from(*high)), text) };
        out.insert("pdf_unicode_mapping", &[C::Integer(key), C::Integer(ordinal as i64), C::Text(kind), code, low, high, C::Text(text)])?;
    }
    Ok(key)
}

pub(super) fn read_unicode(reader: &mut Reader<'_, '_, '_>, key: i64) -> Result<PdfToUnicode,ValueError> {
    let row = reader.take("pdf_to_unicode", key, 2)?; let mut mappings = Vec::new();
    for value in reader.children("pdf_unicode_mapping", 1, 2, key)? { let value = reader.take("pdf_unicode_mapping", value.rowid, 8)?; let text = reader.text(value,7)?; mappings.push(match value.text(3)? { "char" => { null_except(value, 4..7, &[4])?; PdfToUnicodeMapping::Char { code: integer(value, 4)?, text } }, "range" => { null_except(value, 4..7, &[5, 6])?; PdfToUnicodeMapping::Range { low: integer(value, 5)?, high: integer(value, 6)?, text } }, _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown PDF Unicode mapping kind")) }); }
    Ok(PdfToUnicode { byte_width: integer(row, 1)?, mappings })
}

pub(super) fn write_cmap(out: &mut Projection<'_, '_>, cmap: &PdfCMap) -> Result<i64,ValueError> {
    match cmap {
        PdfCMap::Predefined { name } => out.insert("pdf_cmap", &[C::Text("predefined"), C::Text(name), C::Null, C::Null]),
        PdfCMap::Embedded { cmap } => {
            let key = out.insert("pdf_cmap", &[C::Text("embedded"), C::Text(&cmap.name), C::Integer(i64::from(cmap.vertical)), text_cell(&cmap.use_cmap)])?;
            for (ordinal, range) in cmap.codespace.iter().enumerate() { out.insert("pdf_codespace_range", &[C::Integer(key), C::Integer(ordinal as i64), C::Integer(i64::from(range.byte_width)), C::Integer(i64::from(range.low)), C::Integer(i64::from(range.high))])?; }
            for (ordinal, mapping) in cmap.mappings.iter().enumerate() { let (kind, code, low, high, cid) = match mapping { PdfCidMapping::Char { code, cid } => ("char", C::Integer(i64::from(*code)), C::Null, C::Null, *cid), PdfCidMapping::Range { low, high, cid } => ("range", C::Null, C::Integer(i64::from(*low)), C::Integer(i64::from(*high)), *cid) }; out.insert("pdf_cid_mapping", &[C::Integer(key), C::Integer(ordinal as i64), C::Text(kind), code, low, high, C::Integer(i64::from(cid))])?; }
            Ok(key)
        },
    }
}

pub(super) fn read_cmap(reader: &mut Reader<'_, '_, '_>, key: i64) -> Result<PdfCMap,ValueError> {
    let row = reader.take("pdf_cmap", key, 5)?; let name = reader.text(row,2)?;
    match row.text(1)? {
        "predefined" => { null_except(row, 3..5, &[])?; Ok(PdfCMap::Predefined { name }) },
        "embedded" => {
            let mut codespace = Vec::new(); for range in reader.children("pdf_codespace_range", 1, 2, key)? { let range = reader.take("pdf_codespace_range", range.rowid, 6)?; codespace.push(PdfCodespaceRange { byte_width: integer(range, 3)?, low: integer(range, 4)?, high: integer(range, 5)? }); }
            let mut mappings = Vec::new(); for mapping in reader.children("pdf_cid_mapping", 1, 2, key)? { let mapping = reader.take("pdf_cid_mapping", mapping.rowid, 8)?; let cid = integer(mapping, 7)?; mappings.push(match mapping.text(3)? { "char" => { null_except(mapping, 4..7, &[4])?; PdfCidMapping::Char { code: integer(mapping, 4)?, cid } }, "range" => { null_except(mapping, 4..7, &[5, 6])?; PdfCidMapping::Range { low: integer(mapping, 5)?, high: integer(mapping, 6)?, cid } }, _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown PDF CID mapping kind")) }); }
            Ok(PdfCMap::Embedded { cmap: PdfEmbeddedCMap { name, vertical: boolean(row, 3)?, codespace, mappings, use_cmap: reader.optional_text(row,4)? } })
        }, _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown PDF CMap kind")),
    }
}

fn write_cid_font(out: &mut Projection<'_, '_>, font: &PdfCidFont) -> Result<i64,ValueError> {
    let descriptor = write_descriptor(out, &font.descriptor)?; let program = font.program.as_ref().map(|value| write_program(out, value)).transpose()?; let extra = cos::write_dictionary(out, &font.extra)?;
    let (gid_kind, gid_data) = match &font.cid_to_gid { None => (C::Null, C::Null), Some(PdfCidToGid::Identity) => (C::Text("identity"), C::Null), Some(PdfCidToGid::Map { glyphs }) => (C::Text("map"), C::Integer(glyphs.len() as i64)) };
    let key = out.insert("pdf_cid_font", &[C::Integer(i64::from(font.true_type)), C::Text(&font.base_font), C::Text(&font.system_info.registry), C::Text(&font.system_info.ordering), C::Integer(i64::from(font.system_info.supplement)), C::Integer(descriptor), C::Real(font.default_width), font.default_vertical.map_or(C::Null, |value| C::Real(value[0])), font.default_vertical.map_or(C::Null, |value| C::Real(value[1])), gid_kind, gid_data, program.map_or(C::Null, C::Integer), C::Integer(extra)])?;
    if let Some(PdfCidToGid::Map {glyphs})=&font.cid_to_gid {for (ordinal,glyph) in glyphs.iter().enumerate(){out.insert("pdf_cid_glyph",&[C::Integer(key),C::Integer(ordinal as i64),C::Integer(i64::from(*glyph))])?;}}
    for (ordinal, run) in font.widths.iter().enumerate() { let run_key = out.insert("pdf_cid_width_run", &[C::Integer(key), C::Integer(ordinal as i64), C::Integer(i64::from(run.start_cid))])?; for (ordinal, width) in run.widths.iter().enumerate() { out.insert("pdf_cid_width", &[C::Integer(run_key), C::Integer(ordinal as i64), C::Real(*width)])?; } }
    for (ordinal, run) in font.vertical_metrics.iter().enumerate() { let run_key = out.insert("pdf_cid_vertical_run", &[C::Integer(key), C::Integer(ordinal as i64), C::Integer(i64::from(run.start_cid))])?; for (ordinal, metric) in run.metrics.iter().enumerate() { out.insert("pdf_cid_vertical_metric", &[C::Integer(run_key), C::Integer(ordinal as i64), C::Real(metric[0]), C::Real(metric[1]), C::Real(metric[2])])?; } }
    Ok(key)
}

fn read_cid_font(reader: &mut Reader<'_, '_, '_>, key: i64) -> Result<PdfCidFont,ValueError> {
    let row = reader.take("pdf_cid_font", key, 14)?;
    let default_vertical = match (optional_real(row, 8)?, optional_real(row, 9)?) { (None, None) => None, (Some(y), Some(width)) => Some([y, width]), _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF vertical defaults require both metric components")) };
    let cid_to_gid = match row.optional_text(10)? { None => { null_except(row, 11..12, &[])?; None }, Some("identity") => { null_except(row, 11..12, &[])?; Some(PdfCidToGid::Identity) }, Some("map") => {let count=integer::<usize>(row,11)?;let mut glyphs=Vec::new();for child in reader.children("pdf_cid_glyph",1,2,key)? {let child=reader.take("pdf_cid_glyph",child.rowid,4)?;glyphs.push(integer(child,3)?);}if glyphs.len()!=count{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF CID glyph count mismatch"));}Some(PdfCidToGid::Map {glyphs})}, _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown PDF CID to glyph mapping kind")) };
    let mut widths = Vec::new(); for run in reader.children("pdf_cid_width_run", 1, 2, key)? { let run = reader.take("pdf_cid_width_run", run.rowid, 4)?; let mut values = Vec::new(); for width in reader.children("pdf_cid_width", 1, 2, run.rowid)? { let width = reader.take("pdf_cid_width", width.rowid, 4)?; values.push(width.real(3)?); } widths.push(PdfCidWidthRun { start_cid: integer(run, 3)?, widths: values }); }
    let mut vertical_metrics = Vec::new(); for run in reader.children("pdf_cid_vertical_run", 1, 2, key)? { let run = reader.take("pdf_cid_vertical_run", run.rowid, 4)?; let mut metrics = Vec::new(); for metric in reader.children("pdf_cid_vertical_metric", 1, 2, run.rowid)? { let metric = reader.take("pdf_cid_vertical_metric", metric.rowid, 6)?; metrics.push([metric.real(3)?, metric.real(4)?, metric.real(5)?]); } vertical_metrics.push(PdfCidVerticalRun { start_cid: integer(run, 3)?, metrics }); }
    Ok(PdfCidFont { true_type: boolean(row, 1)?, base_font: reader.text(row,2)?, system_info: PdfCidSystemInfo { registry: reader.text(row,3)?, ordering: reader.text(row,4)?, supplement: integer(row, 5)? }, descriptor: read_descriptor(reader, row.integer(6)?)?, default_width: row.real(7)?, widths, default_vertical, vertical_metrics, cid_to_gid, program: optional_integer(row, 12)?.map(|value| read_program(reader, value)).transpose()?, extra: cos::read_dictionary(reader, row.integer(13)?)? })
}

pub(super) fn write_font(out: &mut Projection<'_, '_>, font: &PdfFont) -> Result<i64,ValueError> {
    let mut fields = [C::Null; 17];
    let kind = match &font.kind {
        PdfFontKind::Type1 { base_font, encoding, first_char, descriptor, program, .. } | PdfFontKind::TrueType { base_font, encoding, first_char, descriptor, program, .. } => { fields[0] = C::Text(base_font); fields[1] = C::Integer(write_encoding(out, encoding)?); fields[2] = C::Integer(i64::from(*first_char)); if let Some(descriptor) = descriptor { fields[3] = C::Integer(write_descriptor(out, descriptor)?); } if let Some(program) = program { fields[4] = C::Integer(write_program(out, program)?); } if matches!(font.kind, PdfFontKind::Type1 { .. }) { "type1" } else { "trueType" } },
        PdfFontKind::Type3 { font_matrix, font_bbox, encoding, first_char, descriptor, .. } => { fields[1] = C::Integer(write_encoding(out, encoding)?); fields[2] = C::Integer(i64::from(*first_char)); if let Some(descriptor) = descriptor { fields[3] = C::Integer(write_descriptor(out, descriptor)?); } for (index, value) in font_matrix.iter().chain(font_bbox).enumerate() { fields[5 + index] = C::Real(*value); } "type3" },
        PdfFontKind::Type0 { base_font, cmap, descendant } => { fields[0] = C::Text(base_font); fields[15] = C::Integer(write_cmap(out, cmap)?); fields[16] = C::Integer(write_cid_font(out, descendant)?); "type0" },
    };
    let unicode = font.to_unicode.as_ref().map(|value| write_unicode(out, value)).transpose()?; let extra = cos::write_dictionary(out, &font.extra)?;
    let mut cells = vec![C::Text(&font.id), C::Text(kind)]; cells.extend(fields); cells.extend([unicode.map_or(C::Null, C::Integer), C::Integer(extra)]);
    let key = out.insert("pdf_font", &cells)?;
    match &font.kind {
        PdfFontKind::Type1 { widths, .. } | PdfFontKind::TrueType { widths, .. } | PdfFontKind::Type3 { widths, .. } => { for (ordinal, width) in widths.iter().enumerate() { out.insert("pdf_font_width", &[C::Integer(key), C::Integer(ordinal as i64), C::Real(*width)])?; } }, _ => {},
    }
    if let PdfFontKind::Type3 { char_procs, .. } = &font.kind { for (ordinal, procedure) in char_procs.iter().enumerate() { let content = content::write_ops(out, &procedure.content)?; out.insert("pdf_char_proc", &[C::Integer(key), C::Integer(ordinal as i64), C::Text(&procedure.name), C::Integer(content)])?; } }
    Ok(key)
}

pub(super) fn read_font(reader: &mut Reader<'_, '_, '_>, key: i64) -> Result<PdfFont,ValueError> {
    let row = reader.take("pdf_font", key, 22)?; let kind = row.text(2)?;
    let present: &[usize] = match kind { "type1" | "trueType" => &[3,4,5,6,7], "type3" => &[4,5,6,8,9,10,11,12,13,14,15,16,17], "type0" => &[3,18,19], _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue,"unknown PDF font kind")) };
    null_except(row, 3..20, present)?;
    let mut widths = Vec::new(); if kind != "type0" { for width in reader.children("pdf_font_width", 1, 2, key)? { let width = reader.take("pdf_font_width", width.rowid, 4)?; widths.push(width.real(3)?); } }
    let font_kind = match kind {
        "type1" | "trueType" => { let base_font = reader.text(row,3)?; let encoding = read_encoding(reader, row.integer(4)?)?; let first_char = integer(row, 5)?; let descriptor = optional_integer(row, 6)?.map(|value| read_descriptor(reader, value)).transpose()?; let program = optional_integer(row, 7)?.map(|value| read_program(reader, value)).transpose()?; if kind == "type1" { PdfFontKind::Type1 { base_font, encoding, first_char, widths, descriptor, program } } else { PdfFontKind::TrueType { base_font, encoding, first_char, widths, descriptor, program } } },
        "type3" => { let mut char_procs = Vec::new(); for procedure in reader.children("pdf_char_proc", 1, 2, key)? { let procedure = reader.take("pdf_char_proc", procedure.rowid, 5)?; char_procs.push(PdfCharProc { name: reader.text(procedure,3)?, content: content::read_ops(reader, procedure.integer(4)?)? }); } PdfFontKind::Type3 { font_matrix: [row.real(8)?,row.real(9)?,row.real(10)?,row.real(11)?,row.real(12)?,row.real(13)?], font_bbox: [row.real(14)?,row.real(15)?,row.real(16)?,row.real(17)?], encoding: read_encoding(reader, row.integer(4)?)?, first_char: integer(row,5)?, widths, char_procs, descriptor: optional_integer(row,6)?.map(|value|read_descriptor(reader,value)).transpose()? } },
        "type0" => PdfFontKind::Type0 { base_font: reader.text(row,3)?, cmap: read_cmap(reader,row.integer(18)?)?, descendant: read_cid_font(reader,row.integer(19)?)? }, _ => unreachable!(),
    };
    Ok(PdfFont { id: reader.text(row,1)?, kind: font_kind, to_unicode: optional_integer(row,20)?.map(|value|read_unicode(reader,value)).transpose()?, extra: cos::read_dictionary(reader,row.integer(21)?)? })
}
