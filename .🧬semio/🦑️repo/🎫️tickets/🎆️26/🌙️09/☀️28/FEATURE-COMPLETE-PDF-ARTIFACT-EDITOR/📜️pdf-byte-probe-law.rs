/// 🧬 Supplies the PDF model's two identifier fields to the focused lexical law.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ObjRef { num: u32, gen: u16 }

fn is_ws(value: u8) -> bool { matches!(value, 0 | b'\t' | b'\n' | b'\x0c' | b'\r' | b' ') }
fn is_delim(value: u8) -> bool { b"()<>[]{}/%".contains(&value) }

fn main() {
    use probe_0::{PdfHeaderCursor, PdfHeaderStep};
    use probe_1::{HeaderStep, IndirectHeaderCursor};
    use probe_2::{XrefSectionKind, XrefSectionProbe};
    use probe_3::{FooterCursor, FooterStep};
    use std::{cell::{Cell, RefCell}, fs::File, io::{Read, Seek, SeekFrom}};
    let source = std::env::args_os().nth(1).expect("Original PDF path");
    let source = RefCell::new(File::open(source).unwrap());
    let length = source.borrow().metadata().unwrap().len() as usize;
    for width in [1, 3, 13, 4096] {
        let mut footer = FooterCursor::new(length);
        let mut header = PdfHeaderCursor::default();
        let mut xref = XrefSectionProbe::new($XREF_OFFSET);
        let mut indirect = IndirectHeaderCursor::new($CATALOG_OFFSET);
        let reads = Cell::new(0);
        let byte_at = |position: usize| {
            if position >= length { return None; }
            reads.set(reads.get() + 1);
            let start = position / width * width;
            let mut page = [0; 4096];
            let mut source = source.borrow_mut();
            source.seek(SeekFrom::Start(start as u64)).unwrap();
            let extent = width.min(length - start);
            source.read_exact(&mut page[..extent]).unwrap();
            page.get(position - start).copied()
        };
        assert!(matches!(header.advance(&byte_at, 0), (0, PdfHeaderStep::More)));
        assert!(matches!(footer.advance(&byte_at, 0), FooterStep::More));
        assert!(matches!(xref.advance(&byte_at, 0), (0, None)));
        assert_eq!(indirect.advance(&byte_at, 0), Ok((0, HeaderStep::More)));
        assert_eq!(reads.get(), 0);
        let head = (0..2048).find_map(|_| {
            let prior = reads.get();
            let (work, step) = header.advance(&byte_at, 1);
            assert!(work <= 1 && reads.get() - prior <= work);
            if let PdfHeaderStep::Complete(value) = step { Some(value.unwrap()) } else { None }
        }).unwrap();
        assert_eq!(head.offset(), $HEADER_OFFSET);
        assert_eq!(head.version(), "$PDF_VERSION");
        let offset = (0..128).find_map(|_| {
            let prior = reads.get();
            let step = footer.advance(&byte_at, 1);
            assert!(reads.get() - prior <= 1);
            if let FooterStep::Complete(value) = step { Some(value.unwrap()) } else { None }
        }).unwrap();
        assert_eq!(offset, $XREF_OFFSET);
        let kind = (0..16).find_map(|_| xref.advance(&byte_at, 1).1).unwrap();
        assert_eq!(kind, XrefSectionKind::Table);
        let (id, offset) = (0..32).find_map(|_| {
            if let HeaderStep::Complete { id, body_offset } = indirect.advance(&byte_at, 1).unwrap().1 {
                Some((id, body_offset))
            } else { None }
        }).unwrap();
        assert_eq!(id, ObjRef { num: 1, gen: 0 });
        assert_eq!(offset, $CATALOG_OFFSET + 7);
    }
    println!("[DEBUG] Paged PDF probes matched four source page widths under one-byte grants");
}
