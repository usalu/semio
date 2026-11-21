use crate::actual_fonts::cmap::{parse_to_unicode, to_unicode_lookup, PdfDirectToUnicodeCursor};
use crate::actual_lexer::{CosGrant, CosStatus};
use crate::font_model::PdfToUnicodeMapping;
use std::mem::size_of;

fn settle_retirement(cursor: &mut PdfDirectToUnicodeCursor, live: &mut usize) {
    cursor.begin_close();
    let mut release = 0;
    for _ in 0..8192 {
        let turn = cursor.close_step(CosGrant { work: 1, allocation_bytes: 0, release_bytes: release });
        assert!(turn.work <= 1 && turn.released_bytes <= release && turn.allocated_bytes == 0);
        *live -= turn.released_bytes;
        release = match turn.status {
            Ok(CosStatus::More) => 0,
            Ok(CosStatus::NeedsRelease { bytes }) => bytes,
            Ok(CosStatus::Closed) => return,
            other => panic!("{other:?}"),
        };
    }
    panic!("ToUnicode custody did not close");
}

#[test]
fn direct_tounicode_contract_matches_codes_and_independent_text() {
    let cases: &[(&[u8], u32, &[u32], &str)] = &[
@ORACLE@
    ];
    for (source, width, codes, expected) in cases {
        let mut cursor = PdfDirectToUnicodeCursor::new(32);
        let mut grant = CosGrant { work: 1, ..Default::default() };
        let mut live = 0;
        let mut work = 0;
        for _ in 0..16384 {
            let turn = cursor.advance(source, grant, false);
            assert!(turn.work <= 1 && turn.released_bytes <= grant.release_bytes && turn.allocated_bytes <= grant.allocation_bytes);
            live += turn.allocated_bytes;
            live -= turn.released_bytes;
            work += turn.work;
            grant = CosGrant { work: 1, ..Default::default() };
            match turn.status {
                Ok(CosStatus::More) => {}
                Ok(CosStatus::NeedsAllocation { bytes }) => grant.allocation_bytes = bytes,
                Ok(CosStatus::NeedsRelease { bytes }) => grant.release_bytes = bytes,
                Ok(CosStatus::Complete { end }) => {
                    assert_eq!(end, source.len());
                    let map = cursor.take_mapping().unwrap();
                    assert_eq!(map.byte_width, *width);
                    assert_eq!(map, parse_to_unicode(source));
                    let actual = codes.iter().map(|code| to_unicode_lookup(&map, *code).unwrap()).collect::<String>();
                    assert_eq!(actual, *expected);
                    let output = map.mappings.capacity() * size_of::<PdfToUnicodeMapping>() + map.mappings.iter().map(|mapping| match mapping {
                        PdfToUnicodeMapping::Char { text, .. } | PdfToUnicodeMapping::Range { text, .. } => text.capacity(),
                    }).sum::<usize>();
                    settle_retirement(&mut cursor, &mut live);
                    assert_eq!(live, output);
                    println!("[DEBUG] ToUnicode {} source bytes, {work} work, {output} transferred bytes, {actual:?}", source.len());
                    break;
                }
                other => panic!("unexpected ToUnicode event: {other:?}"),
            }
        }
        assert!(work < 16384);
    }
}

#[test]
fn direct_tounicode_contract_defers_outside_subset_and_retires_partial_maps() {
    const VALID: &[u8] = b"1 begincodespacerange <0000> <ffff> endcodespacerange 2 beginbfchar <0001> <0041> <0002> <0042> endbfchar";
    let unsupported = [
        b"1 begincodespacerange <00> <ff> endcodespacerange 1 beginbfrange <41> <42> [<0041> <0042>] endbfrange".as_slice(),
        b"/Identity-H usecmap",
        b"1 begincodespacerange <0000> <ffff> endcodespacerange 1 beginbfrange <41> <0042> <0041> endbfrange",
        b"1 begincodespacerange <0000> <ffff> endcodespacerange 1 beginbfrange <0041> <0042> <d7ff> endbfrange",
    ];
    let malformed = b"1 begincodespacerange <00> <ff> endcodespacerange 2 beginbfchar <41> <0041> endbfchar";
    let modes = unsupported.into_iter().map(|source| (source, "defer", 32))
        .chain([(VALID, "cancel", 32), (VALID, "defer", 1), (malformed.as_slice(), "invalid", 32)]);
    for (source, mode, capacity) in modes {
        let mut cursor = PdfDirectToUnicodeCursor::new(capacity);
        let mut grant = CosGrant { work: 1, ..Default::default() };
        let mut live = 0;
        let mut reserved_map = false;
        let mut wrote_mapping = false;
        let mut stopped = false;
        for _ in 0..8192 {
            let turn = cursor.advance(source, grant, mode == "cancel" && wrote_mapping);
            assert!(turn.work <= 1 && turn.released_bytes <= grant.release_bytes && turn.allocated_bytes <= grant.allocation_bytes);
            live += turn.allocated_bytes;
            live -= turn.released_bytes;
            if reserved_map && turn.allocated_bytes == 1 { wrote_mapping = true; }
            if turn.allocated_bytes == capacity * size_of::<PdfToUnicodeMapping>() { reserved_map = true; }
            grant = CosGrant { work: 1, ..Default::default() };
            match turn.status {
                Ok(CosStatus::More) => {}
                Ok(CosStatus::NeedsAllocation { bytes }) => grant.allocation_bytes = bytes,
                Ok(CosStatus::NeedsRelease { bytes }) => grant.release_bytes = bytes,
                Ok(CosStatus::Cancelled) => { assert_eq!(mode, "cancel"); stopped = true; break; }
                Err(_) => { assert_eq!(cursor.reference_required(), mode == "defer"); stopped = true; break; }
                other => panic!("{other:?}"),
            }
        }
        assert!(stopped && cursor.take_mapping().is_none());
        settle_retirement(&mut cursor, &mut live);
        assert_eq!(live, 0, "{mode} must retire lexical and mapping backing");
    }
}
