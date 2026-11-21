    use crate::{controlled_graph::{PdfByteGraph, PdfByteGraphCursor, PdfByteGraphEvent as Event}, model::PdfStreamFilter};
    const COPY: usize = 32_768;
    const RELEASE: usize = 1 << 20;
    const BODY: usize = 128;
    fn graph(pdf: &[u8], cancel_after: Option<usize>) -> Option<PdfByteGraph> {
        let mut cursor = PdfByteGraphCursor::new(64, COPY - BODY);
        let mut work = 0;
        let mut live = 0;
        let mut copied = 0;
        for _ in 0..350_000 {
            let grant = CosGrant { work: 1, allocation_bytes: COPY - BODY, release_bytes: RELEASE };
            let cancel = cancel_after.is_some_and(|checkpoint| work >= checkpoint);
            let step = cursor.advance(pdf, grant, cancel);
            assert!(step.work <= 1 && step.allocated_bytes <= grant.allocation_bytes && step.released_bytes <= grant.release_bytes);
            let copy = step.work * BODY + step.allocated_bytes;
            assert!(copy <= COPY);
            work += step.work;
            copied += copy;
            live += step.allocated_bytes;
            live -= step.released_bytes;
            match step.event {
                Ok(Event::More | Event::Object) => {}
                Ok(Event::Complete) => {
                    let (file, released) = cursor.take(grant).unwrap();
                    println!("[DEBUG] graph work={work}, modeled copy={copied}, retained={}", live - released);
                    return Some(file);
                }
                Ok(Event::Cancelled) => {
                    let mut allocation = 0;
                    for _ in 0..2048 {
                        let turn = cursor.close_step(CosGrant { work: 1, allocation_bytes: 0, release_bytes: allocation });
                        assert!(turn.work <= 1 && turn.released_bytes <= allocation);
                        live -= turn.released_bytes;
                        allocation = match turn.event {
                            Ok(Event::More) => 0,
                            Ok(Event::NeedsRelease { bytes }) => bytes,
                            Ok(Event::Closed) => {
                                assert_eq!(live, 0);
                                println!("[DEBUG] cancelled graph at work {work}, all retained bytes released");
                                return None;
                            }
                            other => panic!("graph close: {other:?}"),
                        };
                    }
                    panic!("graph did not close");
                }
                other => panic!("graph step {work}: {other:?}"),
            }
        }
        panic!("graph did not finish");
    }
    let inputs = [
        (include_bytes!("@ASSETS@/large.pdf").as_slice(), None),
        (include_bytes!("@ASSETS@/UnknownXX.pdf").as_slice(), Some(5)),
        (include_bytes!("@ASSETS@/JPXDecode.pdf").as_slice(), Some(8)),
    ];
    for (pdf, opaque) in inputs {
        assert!(pdf.len() > COPY);
        let file = graph(pdf, None).unwrap();
        for (owner, jpeg, transform) in [
            (5, include_bytes!("@ASSETS@/5.jpg").as_slice(), Some(1)),
            (8, include_bytes!("@ASSETS@/8.jpg").as_slice(), None),
        ] {
            let object = &file.graph.objects.iter().find(|entry| entry.id.num == owner).unwrap().value;
            let PdfObject::Stream { data, filters, .. } = object else { unreachable!() };
            assert_eq!(data, jpeg);
            if opaque == Some(owner) {
                assert!(filters.is_empty() && object.dict_get("Filter").is_some());
            } else {
                assert_eq!(*filters, vec![PdfStreamFilter::Dct { color_transform: transform }]);
                assert!(object.dict_get("Filter").is_none() && object.dict_get("DecodeParms").is_none());
            }
        }
    }
    for cancel in [128, 4096, 22_000] {
        assert!(graph(inputs[0].0, Some(cancel)).is_none());
    }
}
