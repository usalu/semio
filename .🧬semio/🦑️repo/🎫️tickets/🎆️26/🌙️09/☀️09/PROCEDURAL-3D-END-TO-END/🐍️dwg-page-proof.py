"""🧱 Exercise the authored DWG page encoding and directory with deterministic section bytes."""
from pathlib import Path
import json
import subprocess
import sys

ticket = Path(__file__).parent
out = ticket / "🗑️generated"
exchange = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io")
write = exchange / "📤️export"
fixture = json.loads((write / "🧪️tests/🧫️fixtures/multiple-object-pages.json").read_text())
record_case = fixture["recordCrc"]
owner = (write / "🦀️.rs").read_text().split("const R2004_PAGE_SIZE", 1)[1].split("fn materialize_r2004_sections", 1)[0]
directory = "🧱️records/🗜️container/🦀️.rs"
writer = (write / directory).read_text().split("//#region 🔖️Lz77Variant", 1)[1]
reader = (exchange / "📥️import" / directory).read_text()
container = reader[reader.index("//#region 🔖️R2004FileHeaderDecrypt"):]
object_source = (write / "📦️objects/🦀️.rs").read_text()
object_records = object_source[object_source.index("pub enum ObjectRecordStep"):object_source.index("//#endregion DwgRead")]
handle_case = json.loads((write / "🧪️tests/🧫️fixtures/handle-blocks.json").read_text())
read_fields = (exchange / "📥️import/🧱️records/🔩️fields/🦀️.rs").read_text()
handle_read = read_fields[read_fields.index("fn read_r2004_modular_char"):read_fields.index("pub fn decode_r2010_eed")].rstrip()
handle_read = handle_read[:handle_read.rfind("// 🚫️async:")]
write_fields = (write / "🧱️records/🔩️fields/🦀️.rs").read_text()
handle_write = write_fields[write_fields.index("pub fn write_r2004_unsigned_modular_char"):write_fields.index("pub fn encode_complex_color_value")].rstrip()
handle_write = handle_write[:handle_write.rfind("// 🚫️async:")]
handle_blocks = object_source[object_source.index("fn finish_r2004_handle_block"):object_source.index("/// 🧩️ Reports one object")]
crc_source = (exchange / "💾️binary/📸️snapshot/🦀️.rs").read_text()
crc = crc_source[crc_source.index("pub fn dwg_crc16"):]
crc = crc[:crc.index("\n}")+2]
header = crc_source[crc_source.index("pub fn decrypt_r2004_header"):crc_source.index("pub const R2004_HEADER_LEN") + len("pub const R2004_HEADER_LEN: usize = 0x6c;")]
fixture_path = exchange.parent / "🖼️assets/🏛️architectural/🏛️architectural.dwg"
source = out / "dwg-page-proof.rs"
source.write_text(
    """
fn push_u32(bytes: &mut Vec<u8>, value: u32) { bytes.extend(value.to_le_bytes()); }
fn push_u64(bytes: &mut Vec<u8>, value: u64) { bytes.extend(value.to_le_bytes()); }
""" + writer + header + container + "const R2004_PAGE_SIZE" + owner + """
#[test]
fn compressed_pages_restore_section_data_and_offsets() {
    let data: Vec<u8> = (0..FACE_COUNT).flat_map(|row| {
        (0..89).map(move |column| ((row * 257 + column * 233) % 251) as u8)
    }).collect();
    let section = R2004Section::compressed(7, "AcDb:AcDbObjects", data.clone());
    let mut pages = Vec::new();
    section.append_pages(&mut pages).unwrap();
    assert!(pages.len() >= MINIMUM_PAGES);
    let mut physical = vec![0; 0x100];
    let mut restored = Vec::new();
    for (index, page) in pages.iter().enumerate() {
        assert_eq!(page.page_number, index as i32 + 1);
        assert_eq!(page.start_offset, (index * R2004_PAGE_SIZE) as u64);
        assert_eq!(page.address, physical.len() as u64);
        write_data_page(&mut physical, page).unwrap();
        restored.extend(decompress_r2004_section(&page.payload, R2004_PAGE_SIZE).unwrap());
    }
    assert_eq!(restored[..data.len()], data);
    assert!(restored[data.len()..].iter().all(|&byte| byte == 0));
    let info = encode_r2004_section_info(&[section.descriptor()], &pages, &[0; 64], &[]).unwrap();
    let directory = r2004_descriptor(&info, 20).unwrap();
    assert_eq!(directory.section.name, "AcDb:AcDbObjects");
    assert_eq!(directory.section.declared_size, data.len() as u64);
    let roster: Vec<_> = info[directory.rows].as_chunks::<16>().0.iter().map(r2004_section_page_row).collect();
    assert_eq!(roster, pages.iter().map(|page| (page.page_number, page.payload.len() as u32, page.start_offset)).collect::<Vec<_>>());
}

#[test]
fn stored_page_keeps_existing_padding_and_checks_capacity() {
    let stored = R2004Section::stored(9, "AcDb:SummaryInfo", vec![1, 2, 3], 128, 0);
    let mut pages = Vec::new();
    stored.append_pages(&mut pages).unwrap();
    assert_eq!(pages.len(), 1);
    assert_eq!(pages[0].payload.len(), 128);
    assert_eq!(pages[0].allocation_size, 160);
    assert_eq!(&pages[0].payload[..3], &[1, 2, 3]);
    assert!(pages[0].payload[3..].iter().all(|&byte| byte == 0));
    assert_eq!(stored.descriptor().semantic_size, 3);
    assert_eq!(stored.descriptor().compression, 1);
    let oversized = R2004Section::stored(9, "AcDb:SummaryInfo", vec![0; 129], 128, 0);
    assert!(oversized.append_pages(&mut pages).is_err());
    assert_eq!(pages.len(), 1);
}
""".replace("FACE_COUNT", str(fixture["faceCount"])).replace("MINIMUM_PAGES", str(fixture["minimumObjectPages"])) + object_records + r"""
use std::collections::{BTreeMap, BTreeSet};
pub struct DwgLogicalObject { handle: u64, body: Option<DwgLogicalObjectBody> }
enum DwgLogicalObjectBody { TableRecord(DwgTableRecordBody), Insert(u64) }
enum DwgTableRecordBody { BlockHeader(Header) }
struct Header { block_entity_handle: u64, common: TableCommon }
struct TableCommon { name: String }
type DwgObjectFrame = (Vec<u8>, Vec<(u64,usize)>);

fn encode_r2010_object_frame(object: &DwgLogicalObject, names: &BTreeMap<u64,String>) -> Result<Vec<u8>,String> {
    match &object.body {
        Some(DwgLogicalObjectBody::Insert(block)) => Ok(names.get(block).ok_or("unindexed block")?.as_bytes().to_vec()),
        _ => Ok(object.handle.to_le_bytes().to_vec()),
    }
}

#[test]
fn framing_keeps_source_order_and_resolves_forward_names() {
    let objects = [
        DwgLogicalObject { handle:9, body:Some(DwgLogicalObjectBody::Insert(5)) },
        DwgLogicalObject { handle:5, body:Some(DwgLogicalObjectBody::TableRecord(
            DwgTableRecordBody::BlockHeader(Header { block_entity_handle:5, common:TableCommon { name:"Bore".into() } }))) },
    ];
    let mut cursor = ObjectRecords::new(&objects);
    for _ in &objects {
        assert!(matches!(cursor.step(&objects).unwrap(), ObjectRecordStep::Indexed));
    }
    assert!(matches!(cursor.step(&objects).unwrap(), ObjectRecordStep::Frame { handle:9, bytes } if bytes == b"Bore"));
    assert!(matches!(cursor.step(&objects).unwrap(), ObjectRecordStep::Frame { handle:5, .. }));
    assert!(matches!(cursor.step(&objects).unwrap(), ObjectRecordStep::Complete));
    let (bytes, offsets) = materialize_r2010_objects(&objects).unwrap();
    assert_eq!(offsets, [(9,4), (5,8)]);
    assert_eq!(&bytes[4..8], b"Bore");
}

#[test]
fn invalid_roster_refuses_before_any_frame() {
    for invalid in [0,8] {
        let objects = [DwgLogicalObject { handle:8, body:None }, DwgLogicalObject { handle:invalid, body:None }];
        let mut cursor = ObjectRecords::new(&objects);
        assert!(cursor.step(&objects[1..]).is_err());
        assert!(matches!(cursor.step(&objects).unwrap(), ObjectRecordStep::Indexed));
        assert!(cursor.step(&objects).is_err());
        assert!(cursor.step(&objects).is_err());
    }
}
"""
 + handle_read + handle_write + handle_blocks + crc + r"""

#[test]
fn read_all_authored_handle_locations() {
    let places = [FIXTURE_LOCATIONS];
    assert_eq!(decode_r2004_handle_map(&materialize_r2004_handles(&places).unwrap()).unwrap(), places);
}

#[test]
fn a_modular_char_cannot_read_across_the_handle_block() {
    assert!(decode_r2004_handle_map(&SHORT_BLOCK).is_err());
}
""".replace("FIXTURE_LOCATIONS", ", ".join(f"(0x{handle}u64,{address}usize)"
    for handle, address in handle_case["locations"])).replace("SHORT_BLOCK", repr(handle_case["shortBlock"]))

)
if "--acadrust" in sys.argv:
    dependency = next(Path(".cache/b/r/m/cargo/home/registry/src").glob("*/acadrust-0.5.5"), None)
    if dependency is None:
        raise RuntimeError("Provision acadrust through the authored DWG Cargo test first")
    module = (dependency / "src/io/dwg/dwg_stream_readers/handle_reader.rs").read_text()
    start = module.index("use std::collections::HashMap;")
    end = module.index("// ═", start)
    oracle = module[start:end].replace("use crate::error::Result;", "type Result<T> = std::result::Result<T, String>;")
    with source.open("a") as proof:
        proof.write("mod acadrust_handle_reader {\n" + oracle + "}\n")
        proof.write(r"""
#[test]
fn acadrust_reads_the_same_unsigned_handle_locations() {
    let expected = [FIXTURE_LOCATIONS];
    let bytes = materialize_r2004_handles(&expected).unwrap();
    let independent = acadrust_handle_reader::read_handles(&bytes).unwrap();
    assert_eq!(independent, expected.iter().map(|&(handle, address)| (handle, address as i64)).collect());
}
""".replace("FIXTURE_LOCATIONS", ", ".join(f"(0x{handle}u64,{address}usize)"
    for handle, address in handle_case["locations"])))
        absolute_fixture = str(fixture_path.resolve())
        with source.open("a") as proof:
            proof.write(r"""
#[test]
fn selected_pages_match_native_objects_and_acadrust_handle_locations() {
    let mut bytes = include_bytes!("FIXTURE_DWG").to_vec();
    let all = decode_r2004_sections(&bytes).unwrap();
    let extra = all.iter().find(|section| section.name == "AcDb:SummaryInfo").unwrap();
    bytes[extra.pages[0].file_address as usize] ^= 0xff;
    let decoded = decode_r2004_sections(&bytes).unwrap();
    assert!(decoded.iter().find(|section| section.name == "AcDb:SummaryInfo").unwrap().pages[0].error.is_some());
    let expected_handles = r2004_section_data(all.iter().find(|section| section.name == "AcDb:Handles").unwrap()).unwrap();
    let expected_objects = r2004_section_data(all.iter().find(|section| section.name == "AcDb:AcDbObjects").unwrap()).unwrap();
    let mut read = DwgDrawingSections::open(&bytes, false).unwrap();
    let mut pages = 0usize;
    let mut directories = 0;
    let mut page_data = 0;
    let (handles, objects) = loop {
        match read.step().unwrap() {
            DwgDrawingSectionStep::Page => pages += 1,
            DwgDrawingSectionStep::Padding => {},
            DwgDrawingSectionStep::PageData => page_data += 1,
            DwgDrawingSectionStep::Directory => { assert_eq!(pages, 0); directories += 1; },
            DwgDrawingSectionStep::Complete { handles, objects } => break (handles, objects),
        }
    };
    assert_eq!(pages, all.iter().filter(|section| matches!(section.name.as_str(), "AcDb:Handles" | "AcDb:AcDbObjects")).map(|section| section.pages.len()).sum());
    assert!(directories > all.len() && page_data > pages);
    assert_eq!(handles, expected_handles);
    assert_eq!(objects, expected_objects);
    let oracle = acadrust_handle_reader::read_handles(&handles).unwrap();
    let decoded = decode_r2004_handle_map(&handles).unwrap();
    assert_eq!(oracle, decoded.iter().map(|&(handle, address)| (handle, address as i64)).collect());
    assert!(read.step().is_err());
}
""".replace("FIXTURE_DWG", absolute_fixture.replace("\\", "\\\\")))
    boundary = handle_case["boundary"]
    with source.open("a") as proof:
        proof.write(r"""
#[test]
fn acadrust_reads_wide_records_across_a_handle_block_boundary() {
    let mut expected: Vec<_> = (1..=BLOCK_COUNT).map(|handle| (handle, 0)).collect();
    expected.push((LAST_HANDLE, usize::try_from(LAST_ADDRESS).unwrap_or(usize::MAX)));
    let bytes = materialize_r2004_handles(&expected).unwrap();
    assert!(u16::from_be_bytes(bytes[..2].try_into().unwrap()) <= MAXIMUM_BLOCK);
    assert_eq!(decode_r2004_handle_map(&bytes).unwrap(), expected);
    let independent = acadrust_handle_reader::read_handles(&bytes).unwrap();
    assert_eq!(independent, expected.iter().map(|&(handle, address)| (handle, address as i64)).collect());
}
""".replace("BLOCK_COUNT", str(boundary["count"])).replace("LAST_HANDLE", f"0x{boundary['lastHandle']}u64")
    .replace("LAST_ADDRESS", f"0x{boundary['lastAddress']}u64").replace("MAXIMUM_BLOCK", str(boundary["maximumBlock"])))

fixture = json.loads((write / "🧪️tests/🧫️fixtures/drawing-section-pages.json").read_text())
with source.open("a") as proof:
    proof.write(r"""
fn encode_independent_rs(data: &[u8], width: usize) -> Vec<u8> {
    let factor = data.len().div_ceil(width);
    let mut encoded = vec![0; factor * 255];
    for (i, &byte) in data.iter().enumerate() {
        encoded[i / width + (i % width) * factor] = byte;
    }
    encoded
}

fn section_row(map: &mut Vec<u8>, name: &str, size: usize, encoding: u64, pages: &[(usize, u64, usize)]) {
    let name = name.encode_utf16().chain([0]).flat_map(u16::to_le_bytes).collect::<Vec<_>>();
    for field in [size as u64, 32768, 0, 0, name.len() as u64, 0, encoding, pages.len() as u64] {
        map.extend(field.to_le_bytes());
    }
    map.extend(name);
    for &(start, id, size) in pages {
        for field in [start as u64, 0, id, size as u64, size as u64, 0, 0] {
            map.extend(field.to_le_bytes());
        }
    }
}

#[test]
fn r2007_skips_unrelated_page_references_and_restores_sparse_object_pages() {
    let handles = EXPECTED_HANDLES;
    let first = EXPECTED_FIRST;
    let second = EXPECTED_SECOND;
    let offset = PAGE_OFFSET;
    let declared = offset + second.len() + TAIL_PADDING;
    let mut map = Vec::new();
    section_row(&mut map, "AcDb:Handles", handles.len(), 1, &[(0, 3, handles.len())]);
    section_row(&mut map, "AcDb:AcDbObjects", declared, 4, &[(0, 4, first.len()), (offset, 5, second.len())]);
    section_row(&mut map, "UNRELATED_SECTION", 1, 1, &[(0, 999, 1)]);
    let section_map = encode_independent_rs(&map, 239);
    let first_page = encode_independent_rs(&first, 251);
    let second_page = encode_independent_rs(&second, 251);
    let mut directory = Vec::new();
    for (index, span) in [255, section_map.len(), handles.len(), first_page.len(), second_page.len()].into_iter().enumerate() {
        directory.extend((span as u64).to_le_bytes());
        directory.extend(((index + 1) as u64).to_le_bytes());
    }
    let mut fields = [0u64; 34];
    for (index, value) in [(3, 1), (10, directory.len()), (11, directory.len()), (22, map.len()), (24, 2), (25, map.len()), (27, 1)] {
        fields[index] = value as u64;
    }
    let mut decoded_header = vec![0; 32];
    decoded_header[24..28].copy_from_slice(&(-1i32).to_le_bytes());
    decoded_header.extend(fields.into_iter().flat_map(u64::to_le_bytes));
    decoded_header.resize(3 * 239, 0);
    let mut bytes = vec![0; 0x80];
    bytes[..6].copy_from_slice(b"DWG_VERSION");
    let header = encode_independent_rs(&decoded_header, 239);
    bytes.extend(header);
    bytes.resize(0x480, 0);
    bytes.extend(encode_independent_rs(&directory, 239));
    bytes.extend(section_map);
    bytes.extend(handles);
    bytes.extend(first_page);
    bytes.extend(second_page);
    assert!(decode_r2007_sections(&bytes).unwrap_err().contains("page 999"));
    let mut read = DwgDrawingSections::open(bytes, true).unwrap();
    let mut pages = 0;
    let mut padding = 0;
    let mut progress = (0, 0);
    let mut fraction = 0.0;
    let mut directory_steps = 0;
    loop {
        let step = read.step().unwrap();
        let current = read.progress();
        assert!(matches!(current.1, 0 | 3));
        assert!(current.0 >= progress.0);
        assert!(read.fraction() >= fraction);
        fraction = read.fraction();
        progress = current;
        match step {
            DwgDrawingSectionStep::Page => pages += 1,
            DwgDrawingSectionStep::Padding => padding += 1,
            DwgDrawingSectionStep::PageData => {},
            DwgDrawingSectionStep::Directory => { assert_eq!(pages, 0); directory_steps += 1; },
            DwgDrawingSectionStep::Complete { handles: actual, objects } => {
                assert_eq!(actual, handles);
                assert_eq!(objects.len(), declared);
                assert_eq!(objects[..first.len()], first);
                assert!(objects[first.len()..offset].iter().all(|&byte| byte == 0));
                assert_eq!(objects[offset..offset + second.len()], second);
                assert!(objects[offset + second.len()..].iter().all(|&byte| byte == 0));
                break;
            }
        }
    }
    assert_eq!(pages, 3);
    assert_eq!(padding, 3);
    assert!(directory_steps > 8);
}

#[test]
fn a_zero_r2007_back_reference_returns_a_codec_error() {
    assert!(decompress_r2007(&ZERO_BACK_REFERENCE, 2).is_err());
    assert!(decompress_r2007(&ZERO_BACK_REFERENCE, 1).is_err());
}

#[test]
fn an_unrepresentable_physical_page_address_has_no_wrapped_read() {
    let page = DwgRawPage { file_address: u64::MAX, ..Default::default() };
    assert!(read_section_page(&[0; 80], &page, false, 0).is_err());
}
""".replace("EXPECTED_HANDLES", repr(fixture["handles"]))
    .replace("EXPECTED_FIRST", repr(fixture["objectPages"][0]))
    .replace("EXPECTED_SECOND", repr(fixture["objectPages"][1]))
    .replace("PAGE_OFFSET", str(fixture["secondObjectPageOffset"]))
    .replace("TAIL_PADDING", str(fixture["declaredTrailingPadding"]))
    .replace("UNRELATED_SECTION", fixture["unrelatedSection"])
    .replace("DWG_VERSION", fixture["version"])
    .replace("ZERO_BACK_REFERENCE", repr(fixture["zeroBackReference"])))

if "--acadrust" in sys.argv:
    frames = (exchange / "📥️import/🧱️records/📐️entities/🦀️.rs").read_text()
    drawing = (exchange / "📥️import/🦀️.rs").read_text()
    actual_reader = drawing[drawing.index("pub fn dwg_from_bytes"):drawing.index("fn project_drawing_record")]
    with source.open("a") as proof:
        proof.write(crc_source[crc_source.index("//#region DwgBits"):crc_source.index("pub fn dwg_crc16")].rsplit("// 🚫️async:", 1)[0])
        string_stream = read_fields[read_fields.index("pub fn r2010_string_stream"):]
        proof.write(string_stream[:string_stream.index("\n}")+2])
        proof.write(frames[frames.index("#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]"):frames.index("/// 🧬️ The entity common data")])
        proof.write(actual_reader)
        crc_path = next(Path(".cache/b/r/m/cargo/home/registry/src").glob("*/acadrust-0.5.5")) / "src/io/dwg/crc.rs"
        crc_oracle = crc_path.read_text()
        table_start = crc_oracle.index("pub const CRC16_TABLE")
        table_end = crc_oracle.index("\n];", table_start) + len("\n];")
        crc_start = crc_oracle.index("pub fn crc16(")
        crc_end = crc_oracle.index("\n}", crc_start) + len("\n}")
        proof.write("mod independent_crc {\n" + crc_oracle[table_start:table_end] + "\n" + crc_oracle[crc_start:crc_end] + "\n}\n")
        writer_bits = (write / "🧱️records/🔢️bits/🦀️.rs").read_text()
        proof.write(writer_bits[writer_bits.index("pub struct DwgBitWriter"):writer_bits.index("//#endregion DwgBits")])
        proof.write(r"""
#[test]
fn object_crc_opens_only_after_all_independent_crc_chunks() {
    let mut payload = DwgBitWriter::new();
    payload.write_bot(TYPE_CODE);
    payload.write_handle(0, OBJECT_HANDLE);
    payload.bytes.resize(PAYLOAD_BYTES, 0);
    let mut header = DwgBitWriter::new();
    header.write_ms(PAYLOAD_BYTES as u32);
    header.write_umc(0);
    let mut record = header.bytes;
    record.extend(payload.bytes);
    let turns = record.len().div_ceil(FRAME_BUDGET);
    let crc = independent_crc::crc16(0xc0c1, &record);
    let mut section = vec![0xff; OFFSET_BYTES];
    section.extend(record);
    section.extend(crc.to_le_bytes());
    let mut reader = DwgObjectFrames::new(section.clone(), DwgObjectLayout::R2010);
    reader.start(OBJECT_HANDLE, OFFSET_BYTES).unwrap();
    for _ in 1..turns { assert!(reader.step().unwrap().is_none()); }
    let (id, frame) = reader.step().unwrap().unwrap();
    assert_eq!(id, OBJECT_HANDLE);
    assert_eq!(frame.type_code, TYPE_CODE);
    assert_eq!(frame.end_bit, PAYLOAD_BYTES * 8);
    assert!(!reader.is_pending());

    let corrupt = section.len() - 3;
    section[corrupt] ^= 1;
    let mut reader = DwgObjectFrames::new(section, DwgObjectLayout::R2010);
    reader.start(OBJECT_HANDLE, OFFSET_BYTES).unwrap();
    for _ in 1..turns { assert!(reader.step().unwrap().is_none()); }
    let error = reader.step().err().unwrap();
    assert!(error.contains("CRC mismatch"));
    assert_eq!(reader.step().err(), Some(error));
}
""".replace("TYPE_CODE", str(record_case["typeCode"]))
    .replace("OBJECT_HANDLE", str(record_case["objectHandle"]))
    .replace("PAYLOAD_BYTES", str(record_case["payloadBytes"]))
    .replace("OFFSET_BYTES", str(record_case["offsetBytes"]))
    .replace("FRAME_BUDGET", str(record_case["maximumBytesPerTurn"])))
        proof.write(r"""
struct DwgLogicalLayer { name: String, color: u8 }
impl DwgLogicalLayer { fn to_native(&self) -> (String, u8) { (self.name.clone(), self.color) } }
struct DwgLogicalDrawing { layers: Vec<DwgLogicalLayer>, objects: Vec<(u64, u16)>, extmin: Vec<f64>, extmax: Vec<f64> }
#[derive(PartialEq, Debug)]
struct DwgEntity { record: (u64, u16), geometry: () }
#[derive(Default)]
pub struct DwgDrawing { layers: Vec<(String, u8)>, entities: Vec<DwgEntity> }
#[derive(Default)]
struct DwgDrawingExtents;
impl DwgDrawingExtents {
    fn apply(&self, _: &mut DwgDrawing) {}
}
struct DwgEntityRead((u64, u16));
impl DwgEntityRead {
    fn progress(&self) -> (usize, usize) { (0, 0) }
    fn step(&mut self, _: &DwgEntityProjection, _: &[(u64, u16)], _: &mut DwgDrawingExtents) -> Result<std::task::Poll<DwgEntity>, String> {
        Ok(std::task::Poll::Ready(DwgEntity { record: self.0, geometry: () }))
    }
}
#[derive(Default)]
struct DwgEntityProjection;
impl DwgEntityProjection {
    fn index(&mut self, _: usize, _: &(u64, u16)) {}
    fn begin(&self, objects: &[(u64, u16)], row: usize) -> Option<DwgEntityRead> {
        objects.get(row).copied().map(DwgEntityRead)
    }
}
fn project_drawing_record(logical: &mut DwgLogicalDrawing, frame: &mut DwgObjectRecordFrame<'_>, handle: u64) -> Result<(), String> {
    logical.objects.push((handle, frame.type_code));
    Ok(())
}

#[test]
fn drawing_cursor_resumes_real_object_frames_and_a_failed_record_stays_terminal() {
    let bytes = include_bytes!("FRAME_FIXTURE").as_slice();
    let drawing = dwg_from_bytes(bytes).unwrap();
    let mut pages = DwgDrawingSections::open(bytes, false).unwrap();
    let (handles, mut objects) = loop {
        if let DwgDrawingSectionStep::Complete { handles, objects } = pages.step().unwrap() { break (handles, objects); }
    };
    let oracle = acadrust_handle_reader::read_handles(&handles).unwrap();
    assert_eq!(drawing.entities.len(), oracle.len());
    assert!(drawing.entities.iter().all(|entity| oracle.contains_key(&entity.record.0)));
    let mut reader = DwgDrawingRead::open(bytes).unwrap();
    let mut steps = 0;
    let mut geometry = 0;
    let mut progress = 0.0;
    loop {
        let result = reader.step().unwrap();
        assert!(reader.progress() >= progress);
        progress = reader.progress();
        match result {
            DwgDrawingReadStep::Record => steps += 1,
            DwgDrawingReadStep::Geometry => geometry += 1,
            DwgDrawingReadStep::Complete(result) => {
                assert_eq!(result.entities, drawing.entities);
                assert_eq!(result.layers, [("0".into(), 7)]);
                break;
            }
            _ => {}
        }
    }
    assert_eq!(steps, oracle.len());
    assert_eq!(geometry, oracle.len() + 1);
    let broken_address = oracle[&drawing.entities[1].record.0] as usize;
    objects[broken_address] ^= 0xff;
    let mut broken = DwgDrawingRead::open(bytes).unwrap();
    broken.phase = DrawingReadPhase::Records { handles: HandleMapRows::new(handles), frames: DwgObjectFrames::new(objects, DwgObjectLayout::R2010) };
    assert!(matches!(broken.step().unwrap(), DwgDrawingReadStep::Record));
    let error = broken.step().err().unwrap();
    assert_eq!(broken.step().err().unwrap(), error);
    assert_eq!(broken.logical.objects.len(), 1);
}
""".replace("FRAME_FIXTURE", str(fixture_path.resolve()).replace("\\", "\\\\")))

if "--acadrust" in sys.argv:
    fixture = json.loads((write / "🧪️tests/🧫️fixtures/bounded-r2004-page.json").read_text())
    oracle = (dependency / "src/io/dwg/decompressor_ac18.rs").read_text().split("#[cfg(test)]", 1)[0]
    with source.open("a") as proof:
        proof.write("mod acadrust_lz77 {\n" + oracle + "}\n")
        proof.write(r"""
#[test]
fn r2004_extended_overlap_resumes_with_bounded_copy_and_matches_acadrust() {
    let seed = SEED_BYTES;
    let match_length = EXT_ZEROES * 255 + EXT_FINAL + 33;
    let expected = seed.into_iter().cycle().take(seed.len() + match_length).collect::<Vec<_>>();
    let mut payload = vec![1];
    payload.extend(seed);
    payload.push(0x20);
    payload.extend(std::iter::repeat_n(0, EXT_ZEROES));
    payload.extend([EXT_FINAL as u8, 12, 0, 0x11]);
    for budget in [1, COPY_BUDGET] {
        let mut decoder = R2004Decoder::new(expected.len());
        assert!(!decoder.step(&payload, 0).unwrap());
        assert_eq!(decoder.input_offset(), 0);
        let mut steps = 0;
        loop {
            let before = (decoder.input_offset(), decoder.output().len());
            let done = decoder.step(&payload, budget).unwrap();
            assert!(decoder.input_offset() - before.0 <= budget);
            assert!(decoder.output().len() - before.1 <= budget);
            steps += 1;
            if done { break; }
        }
        assert!(steps >= expected.len().div_ceil(budget));
        assert_eq!(decoder.output(), expected);
    }
    assert_eq!(acadrust_lz77::decompress_ac18(&payload, expected.len()), expected);
    assert_eq!(decompress_r2004_section(&payload, expected.len()).unwrap(), expected);
}

#[test]
fn real_r2004_compressed_pages_match_the_independent_lz77_decoder() {
    let bytes = include_bytes!("R2004_FIXTURE").as_slice();
    let sections = locate_r2004_sections(bytes).unwrap();
    let mut checked = 0;
    for section in sections.iter().filter(|section| section.compressed) {
        for page in &section.pages {
            let (payload, bound) = r2004_data_payload(&bytes[page.file_address as usize..], page.file_address, section.max_decomp_size).unwrap();
            let restored = decompress_r2004_section(payload, bound).unwrap();
            let oracle = acadrust_lz77::decompress_ac18(payload, bound);
            assert_eq!(restored.len(), oracle.len(), "{} page {}", section.name, page.page_number);
            assert_eq!(restored, oracle, "{} page {}", section.name, page.page_number);
            checked += 1;
        }
    }
    assert!(checked >= 5);
}

#[test]
fn r2004_truncated_match_offset_cannot_publish_a_partial_decode() {
    let input = TRUNCATED_OFFSET;
    let mut decoder = R2004Decoder::new(32768);
    let error = loop {
        match decoder.step(&input, COPY_BUDGET) {
            Ok(false) => {},
            Ok(true) => panic!("truncated match was accepted"),
            Err(error) => break error,
        }
    };
    assert_eq!(decoder.step(&input, COPY_BUDGET).unwrap_err(), error);
    assert_eq!(decoder.output(), SEED_BYTES);
}
""".replace("SEED_BYTES", repr(fixture["seed"]))
    .replace("EXT_ZEROES", str(fixture["extendedLengthZeroes"]))
    .replace("EXT_FINAL", str(fixture["extendedLengthTerminator"]))
    .replace("COPY_BUDGET", str(fixture["copyBudget"]))
    .replace("TRUNCATED_OFFSET", repr(fixture["truncatedOffset"]))
    .replace("R2004_FIXTURE", str(fixture_path.resolve()).replace("\\", "\\\\")))

if "--acadrust" in sys.argv:
    fixture = json.loads((write / "🧪️tests/🧫️fixtures/bounded-r2007-page.json").read_text())
    with source.open("a") as proof:
        for module in ("compressor_ac21", "decompressor_ac21"):
            dependent = (dependency / "src/io/dwg" / (module + ".rs")).read_text().rsplit("#[cfg(test)]", 1)[0]
            proof.write(f"mod {module} {{\n{dependent}}}\n")
        proof.write(r"""
fn ac21_original() -> Vec<u8> {
    let mut data: Vec<u8> = (0..PREFIX_BYTES).map(|i: usize| (i.wrapping_mul(STRIDE).wrapping_add(i >> 3).wrapping_add(i >> 7) % 251) as u8).collect();
    data.resize(OUTPUT_BYTES, 5);
    data
}

#[test]
fn ac21_literal_reordering_and_long_matches_resume_and_match_acadrust() {
    let original = ac21_original();
    let source = compressor_ac21::compress_ac21(&original);
    let mut oracle = vec![0; original.len()];
    decompressor_ac21::decompress_ac21(&source, 0, source.len() as u32, &mut oracle);
    assert_eq!(oracle, original);
    let mut decoder = R2007Decoder::new(original.len());
    let mut steps = 0;
    loop {
        let before = (decoder.input, decoder.output.len());
        let exhausted = decoder.step(&source).unwrap();
        assert!(decoder.input - before.0 <= COPY_WINDOW);
        assert!(decoder.output.len() - before.1 <= COPY_WINDOW);
        steps += 1;
        if exhausted { break; }
    }
    assert!(steps >= original.len().div_ceil(COPY_WINDOW));
    assert_eq!(decoder.output, original);
}

#[test]
fn ac21_output_keeps_padding_and_checks_complete_oversize_streams() {
    let source = compressor_ac21::compress_ac21(&ac21_original());
    let prefix = decompress_r2007(&source, PREFIX_BYTES).unwrap();
    assert_eq!(prefix, ac21_original()[..PREFIX_BYTES]);
    let padded = decompress_r2007(&source, OUTPUT_BYTES + EXTRA_BYTES).unwrap();
    assert_eq!(padded[..OUTPUT_BYTES], ac21_original());
    assert!(padded[OUTPUT_BYTES..].iter().all(|&byte| byte == 0));
}
#[test]
fn r2007_physical_interleaving_validates_all_blocks_and_decodes_both_storage_modes() {
    let original = ac21_original();
    for payload in [original.clone(), compressor_ac21::compress_ac21(&original)] {
        let padded_length = payload.len().next_multiple_of(8) * 3;
        let mut padded = payload.clone();
        padded.resize(padded_length, 0);
        let encoded = encode_independent_rs(&padded, RS_BLOCK_BYTES);
        let mut file = vec![0; 81];
        file.extend(encoded);
        let mut read = R2007PageReader::new(81, payload.len(), original.len(), 3, RS_BLOCK_BYTES).unwrap();
        let mut deinterleave_steps = 0;
        let mut progress = 0.0;
        loop {
            if matches!(read.stage, R2007PageStage::Deinterleave(_)) { deinterleave_steps += 1; }
            let done = read.step(&file).unwrap();
            assert!(read.fraction() >= progress);
            progress = read.fraction();
            if done { break; }
        }
        assert_eq!(deinterleave_steps, padded_length.div_ceil(COPY_WINDOW));
        assert!(matches!(&read.stage, R2007PageStage::Complete(bytes) if bytes == &original));
        assert_eq!(r2007_page(&file, 81, payload.len(), original.len(), 3, RS_BLOCK_BYTES).unwrap(), original);
        let length = file.len();
        file.truncate(length / 2);
        assert!(r2007_page(&file, 81, payload.len(), original.len(), 3, RS_BLOCK_BYTES).is_err());
    }
}

#[test]
fn r2007_directory_utf16_name_does_not_split_a_surrogate_pair_or_trim_early() {
    let first = "x".repeat(COPY_WINDOW / 3 - 1);
    let name = format!("{first}🌈\0layer");
    let padded = format!("{name}{}", "\0".repeat(COPY_WINDOW));
    let mut data = Vec::new();
    section_row(&mut data, &padded, 0, 1, &[]);
    let mut rows = R2007SectionRows::open(&data, 0).unwrap();
    let mut steps = 0;
    loop {
        let before = rows.name.start;
        let done = rows.name_step(&data).unwrap();
        assert!(rows.name.start - before <= COPY_WINDOW);
        steps += 1;
        if done { break; }
    }
    assert!(steps >= 2);
    assert_eq!(rows.section.name, name);
}
""".replace("PREFIX_BYTES", str(fixture["prefixBytes"]))
    .replace("OUTPUT_BYTES", str(fixture["outputBytes"]))
    .replace("STRIDE", str(fixture["alphabetStride"]))
    .replace("COPY_WINDOW", str(fixture["boundaryBytes"]))
    .replace("EXTRA_BYTES", str(fixture["validZeroPaddingBytes"]))
    .replace("RS_BLOCK_BYTES", str(fixture["reedSolomonBlockBytes"])))

binary = out / "dwgpg"
subprocess.run(["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20",
                "rustc", "--edition=2024", "--test", str(source), "-o", str(binary)], check=True)
subprocess.run(["repo", "tools", "exec", "--light", "python", "--", "-c",
                "import os,sys; os.execv(sys.argv[1], sys.argv[1:])", str(binary)], check=True)
