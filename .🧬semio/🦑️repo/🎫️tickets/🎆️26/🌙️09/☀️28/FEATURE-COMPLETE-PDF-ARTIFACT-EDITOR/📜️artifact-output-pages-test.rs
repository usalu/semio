//! 🧪 Checks segmented slot funding and FIFO wraparound.
#[path = "📜️artifact-output-pages.rs"]
mod pages;
use pages::ArtifactOutputPages;
use std::collections::VecDeque;

#[test]
fn full_export_uses_bounded_backing_and_preserves_every_donated_vector() {
    let max = 32 * 1024 * 1024 / 4096;
    let mut queue = ArtifactOutputPages::try_new(max).unwrap();
    assert_eq!(queue.table_capacity_bytes(), ArtifactOutputPages::table_bytes(max));
    assert!(queue.table_capacity_bytes() < 4096);
    let mut expected = VecDeque::new();
    let mut pages = 0;
    for index in 0..max {
        let previously = queue.len();
        let demand = queue.tail_page_bytes();
        if demand != 0 {
            pages += 1;
            assert!(demand <= 64 * size_of::<Option<Vec<u8>>>());
            assert_eq!(queue.reserve_tail().unwrap(), demand);
            assert_eq!(queue.len(), previously);
        }
        let octets = index.to_le_bytes().to_vec();
        expected.push_back((octets.as_ptr(), octets.clone()));
        queue.push(octets).unwrap();
    }
    assert_eq!(pages, max / 64);
    assert!(!queue.can_push());
    let rejected = vec![255];
    let refused = queue.push(rejected.clone()).unwrap_err();
    assert_eq!(refused, rejected);
    while let Some((pointer, bytes)) = expected.pop_front() {
        let output = queue.pop().expect("original output page");
        assert_eq!(output.as_ptr(), pointer);
        assert_eq!(output, bytes);
    }
    assert_eq!(queue.len(), 0);
    assert!(queue.pop().is_none());
    assert_eq!(queue.pages().len(), pages);
}

#[test]
fn partial_tail_and_wrap_do_not_move_or_reallocate_prepaid_slots() {
    let mut queue = ArtifactOutputPages::try_new(129).unwrap();
    for value in 0..129u8 {
        let expected = queue.tail_page_bytes();
        if expected != 0 { assert_eq!(queue.reserve_tail().unwrap(), expected); }
        queue.push(vec![value]).unwrap();
    }
    assert_eq!(queue.pages().iter().map(Vec::capacity).collect::<Vec<_>>(), vec![64, 64, 1]);
    assert_eq!(queue.first(), Some(&vec![0]));
    assert_eq!(queue.last(), Some(&vec![128]));
    for value in 0..65u8 { assert_eq!(queue.pop(), Some(vec![value])); }
    for value in 0..65u8 {
        assert_eq!(queue.tail_page_bytes(), 0);
        queue.push(vec![value]).unwrap();
    }
    for value in (65..129u8).chain(0..65) { assert_eq!(queue.pop(), Some(vec![value])); }
    assert_eq!(queue.len(), 0);
    assert!(queue.into_pages().iter().flatten().all(Option::is_none));
}

#[test]
fn native_large_pdf_preserves_the_complete_backed_queue() {
    use std::{fs::File, io::{Read, Write}};
    let mut source = File::open(std::env::var_os("SEMIO_QUEUE_SOURCE_PDF").unwrap()).unwrap();
    let mut delivery = File::create(std::env::var_os("SEMIO_QUEUE_DESTINATION_PDF").unwrap()).unwrap();
    let mut queue = ArtifactOutputPages::try_new(32 * 1024 * 1024 / 4096).unwrap();
    let mut total = 0;
    loop {
        let mut chunk = vec![0; 4096];
        let mut used = 0;
        while used < chunk.len() {
            let n = source.read(&mut chunk[used..]).unwrap();
            if n == 0 { break; }
            used += n;
        }
        if used == 0 { break; }
        if queue.tail_page_bytes() != 0 { queue.reserve_tail().unwrap(); }
        chunk.truncate(used);
        queue.push(chunk).unwrap();
        total += used;
    }
    let mut emitted = 0;
    while let Some(chunk) = queue.pop() {
        delivery.write_all(&chunk).unwrap();
        emitted += chunk.len();
    }
    assert_eq!(emitted, total);
    assert!(emitted > 2 * 1024 * 1024);
}
