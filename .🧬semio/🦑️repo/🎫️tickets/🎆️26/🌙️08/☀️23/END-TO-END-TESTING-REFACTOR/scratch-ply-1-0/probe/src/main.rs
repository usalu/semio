use ply_rs::parser::Parser;
use ply_rs::ply::{DefaultElement, Property, PropertyAccess, PropertyType, ScalarType, Encoding};
use ply_rs::writer::Writer;
use std::io::Cursor;

fn write_binary_payload(ply: &ply_rs::ply::Ply<DefaultElement>, little: bool) -> Vec<u8> {
    let mut out = Vec::new();
    for (ename, edef) in &ply.header.elements {
        for row in &ply.payload[ename] {
            for (pname, pdef) in &edef.properties {
                match &pdef.data_type {
                    PropertyType::Scalar(st) => write_scalar(&mut out, row, pname, st, little),
                    PropertyType::List(index_ty, item_ty) => {
                        let items_len = list_len(row, pname, item_ty);
                        write_index(&mut out, index_ty, items_len, little);
                        write_list_items(&mut out, row, pname, item_ty, little);
                    }
                }
            }
        }
    }
    out
}

fn list_len(row: &DefaultElement, k: &str, item_ty: &ScalarType) -> usize {
    let key = k.to_string();
    match item_ty {
        ScalarType::Char => row.get_list_char(&key).map(|v| v.len()).unwrap_or(0),
        ScalarType::UChar => row.get_list_uchar(&key).map(|v| v.len()).unwrap_or(0),
        ScalarType::Short => row.get_list_short(&key).map(|v| v.len()).unwrap_or(0),
        ScalarType::UShort => row.get_list_ushort(&key).map(|v| v.len()).unwrap_or(0),
        ScalarType::Int => row.get_list_int(&key).map(|v| v.len()).unwrap_or(0),
        ScalarType::UInt => row.get_list_uint(&key).map(|v| v.len()).unwrap_or(0),
        ScalarType::Float => row.get_list_float(&key).map(|v| v.len()).unwrap_or(0),
        ScalarType::Double => row.get_list_double(&key).map(|v| v.len()).unwrap_or(0),
    }
}

fn write_index(out: &mut Vec<u8>, ty: &ScalarType, len: usize, little: bool) {
    match ty {
        ScalarType::Char => out.push(len as i8 as u8),
        ScalarType::UChar => out.push(len as u8),
        ScalarType::Short => out.extend(le_be((len as i16).to_le_bytes(), (len as i16).to_be_bytes(), little)),
        ScalarType::UShort => out.extend(le_be((len as u16).to_le_bytes(), (len as u16).to_be_bytes(), little)),
        ScalarType::Int => out.extend(le_be((len as i32).to_le_bytes(), (len as i32).to_be_bytes(), little)),
        ScalarType::UInt => out.extend(le_be((len as u32).to_le_bytes(), (len as u32).to_be_bytes(), little)),
        _ => unreachable!("list index must be integer"),
    }
}

fn le_be<const N: usize>(le: [u8; N], be: [u8; N], little: bool) -> [u8; N] {
    if little { le } else { be }
}

fn write_scalar(out: &mut Vec<u8>, row: &DefaultElement, k: &str, ty: &ScalarType, little: bool) {
    let key = k.to_string();
    match ty {
        ScalarType::Char => out.push(row.get_char(&key).unwrap() as u8),
        ScalarType::UChar => out.push(row.get_uchar(&key).unwrap()),
        ScalarType::Short => out.extend(le_be(row.get_short(&key).unwrap().to_le_bytes(), row.get_short(&key).unwrap().to_be_bytes(), little)),
        ScalarType::UShort => out.extend(le_be(row.get_ushort(&key).unwrap().to_le_bytes(), row.get_ushort(&key).unwrap().to_be_bytes(), little)),
        ScalarType::Int => out.extend(le_be(row.get_int(&key).unwrap().to_le_bytes(), row.get_int(&key).unwrap().to_be_bytes(), little)),
        ScalarType::UInt => out.extend(le_be(row.get_uint(&key).unwrap().to_le_bytes(), row.get_uint(&key).unwrap().to_be_bytes(), little)),
        ScalarType::Float => out.extend(le_be(row.get_float(&key).unwrap().to_le_bytes(), row.get_float(&key).unwrap().to_be_bytes(), little)),
        ScalarType::Double => out.extend(le_be(row.get_double(&key).unwrap().to_le_bytes(), row.get_double(&key).unwrap().to_be_bytes(), little)),
    }
}

fn write_list_items(out: &mut Vec<u8>, row: &DefaultElement, k: &str, item_ty: &ScalarType, little: bool) {
    let key = k.to_string();
    match item_ty {
        ScalarType::Int => { for v in row.get_list_int(&key).unwrap() { out.extend(le_be(v.to_le_bytes(), v.to_be_bytes(), little)); } }
        ScalarType::UInt => { for v in row.get_list_uint(&key).unwrap() { out.extend(le_be(v.to_le_bytes(), v.to_be_bytes(), little)); } }
        ScalarType::Float => { for v in row.get_list_float(&key).unwrap() { out.extend(le_be(v.to_le_bytes(), v.to_be_bytes(), little)); } }
        _ => unimplemented!(),
    }
}

fn main() {
    let path = "../pattern-sphere.ply";
    let bytes = std::fs::read(path).expect("read fixture");
    let parser = Parser::<DefaultElement>::new();
    let mut cur = Cursor::new(&bytes);
    let mut ply = parser.read_ply(&mut cur).expect("parse ply");

    ply.header.encoding = Encoding::BinaryLittleEndian;
    ply.payload.get_mut("vertex").unwrap()[0].set_property("x".to_string(), Property::Float(42.0));

    let w: Writer<DefaultElement> = Writer::new();
    let mut out = Vec::new();
    out.extend(w.write_header(&mut out.clone(), &ply.header).map(|_| Vec::<u8>::new()).unwrap_or_default()); // placeholder, replaced below
    let mut header_buf = Vec::new();
    w.write_header(&mut header_buf, &ply.header).expect("write header");
    let payload_buf = write_binary_payload(&ply, true);
    let mut full = header_buf;
    full.extend(payload_buf);
    println!("custom binary wrote {} bytes", full.len());

    let mut cur2 = Cursor::new(&full);
    let parser2 = Parser::<DefaultElement>::new();
    let ply2 = parser2.read_ply(&mut cur2).expect("parse custom binary output");
    println!("re-read OK encoding={:?} vertex len={} face len={} edge len={}", ply2.header.encoding, ply2.payload["vertex"].len(), ply2.payload["face"].len(), ply2.payload["edge"].len());
    let v0 = &ply2.payload["vertex"][0];
    println!("re-read v0.x={:?}", v0.get_float(&"x".to_string()));
    let f100 = &ply2.payload["face"][100];
    let orig_f100 = &ply.payload["face"][100];
    println!("face[100] custom-binary={:?} original={:?}", f100.get_list_int(&"vertex_indices".to_string()), orig_f100.get_list_int(&"vertex_indices".to_string()));
    assert_eq!(f100.get_list_int(&"vertex_indices".to_string()), orig_f100.get_list_int(&"vertex_indices".to_string()));
    let e10 = &ply2.payload["edge"][10];
    println!("edge[10] v1={:?} v2={:?}", e10.get_int(&"v1".to_string()), e10.get_int(&"v2".to_string()));
    println!("ALL GOOD");
}
