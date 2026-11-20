#[path = "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🗜️filters/🪣️stream-data/✂️packet-filters/🦀️.rs"]
mod packet_filters;

#[test]
fn source_and_destination_both_resume_for_each_packet() {
    let cases = [
        (packet_filters::SimpleFilterCursor::ascii_hex(), b"303020f>".as_slice(), &[48,48,32,240][..]),
        (packet_filters::SimpleFilterCursor::run_length(), &[254,255,2,3,4,5,128][..], &[255,255,255,3,4,5][..]),
    ];
    for (mut cursor, source, expected) in cases {
        for budget in [1usize] {
            let mut out = Vec::new();
            let mut at = 0;
            loop {
                let mut slot = [0u8; 1];
                let turn = cursor.advance(source, &mut slot, budget);
                assert!(turn.work <= budget && turn.input >= at);
                at = turn.input;
                out.extend_from_slice(&slot[..turn.written]);
                if turn.complete { break; }
            }
            assert_eq!(out, expected);
            assert_eq!(at, source.len());
        }
    }
}

#[path = "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🗜️filters/🪣️stream-data/🔡️ascii85/🦀️.rs"]
mod ascii85;

#[test]
fn ascii85_zero_groups_and_partial_tails_span_multiple_turns() {
    for source in [b"<~z'HSY~>".as_slice(), b"z'HSY~>".as_slice()] {
        let mut cursor = ascii85::Ascii85Cursor::new();
        let mut decoded = Vec::new();
        loop {
            let mut slot = [0u8; 1];
            let turn = cursor.advance(source, &mut slot, 1);
            assert!(turn.work <= 1);
            decoded.extend_from_slice(&slot[..turn.written]);
            if turn.done.unwrap() { assert_eq!(turn.input, source.len()); break; }
        }
        assert_eq!(decoded, [0, 0, 0, 0, 20, 30, 40]);
    }
}
