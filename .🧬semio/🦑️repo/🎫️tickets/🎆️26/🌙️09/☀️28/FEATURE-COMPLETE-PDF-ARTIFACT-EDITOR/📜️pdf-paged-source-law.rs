use semio_framework_value::{
    AuthoringWallet, ByteReadSource, OriginalRetirementFailure, OriginalRetirementPoll,
    RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneStep,
    RetainedWorkPoll, ValueError, paged::PagedBytes, retirement::controlled::ControlledRetirement,
};

type NaturalFilePagedSource = ControlledRetirement<PagedBytes<{ 512 * 1024 * 1024 }>>;

fn main() {
    let path = std::env::args_os().nth(1).unwrap();
    let original = std::fs::read(path).unwrap();
    let extent = original.len();
    assert!(extent > 8 * 1024 * 1024);
    let pages = PagedBytes::try_from_slice(&original).unwrap();
    let mut source = PdfNaturalSource::Paged(NaturalFilePagedSource::retain(pages));
    let reader = source.reader().unwrap();
    assert_eq!(reader.byte_len(), extent);
    let mut window = [0u8; 4096];
    let mut compared = 0;
    while compared < extent {
        let count = window.len().min(extent - compared);
        assert_eq!(reader.read_bytes(compared, &mut window[..count]), count);
        assert_eq!(window[..count], original[compared..compared + count]);
        compared += count;
    }
    assert_eq!(source.contiguous_capacity(), 0);
    assert!(!source.terminal_is_empty());
    let stock = RetainedCloneGrant {
        maximum_items: 1 << 20, maximum_copy_bytes: 1 << 28, maximum_capacity_bytes: 1 << 26,
        maximum_release_bytes: 1 << 26, maximum_depth: 4096,
    };
    let mut wallet = AuthoringWallet::new(stock, 1 << 24);
    let withheld = RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: 16 * 1024, maximum_release_bytes: 0, ..stock };
    let denied = wallet.run_invoked(withheld, 16, |effect| source.close_paged(effect)).unwrap().unwrap();
    let first = denied.output.unwrap().unwrap().progress();
    assert_eq!(first.released_bytes,0);
    wallet.post(first).unwrap();
    assert!(!source.terminal_is_empty());
    let mut released = 0;
    let mut allocated = first.retained_capacity_bytes;
    let mut turns = 0;
    while !source.terminal_is_empty() {
        let PdfNaturalSource::Paged(owner) = &source else { panic!("source custody changed before retirement"); };
        let body = 16 * 1024;
        let demand = owner.next_demand(body).unwrap();
        let work = owner.next_computational_work(body).unwrap();
        let grant = RetainedCloneGrant {
            maximum_items: 16,
            maximum_copy_bytes: 16 * 1024,
            maximum_capacity_bytes: demand.capacity_bytes,
            maximum_release_bytes: demand.release_bytes,
            maximum_depth: stock.maximum_depth,
        };
        let receipt = wallet.run_invoked(grant, work.max(1)+4096, |effect| source.close_paged(effect)).unwrap().unwrap();
        let progress = receipt.output.unwrap().unwrap().progress();
        if turns < 14 { eprintln!("[DEBUG] paid source turn {turns} quote {demand:?} work {work} progress {progress:?}"); }
        assert!(progress.fits(grant));
        wallet.post(progress).unwrap();
        released += progress.released_bytes;
        allocated += progress.retained_capacity_bytes;
        turns += 1;
        assert!(turns < 65_536);
    }
    assert!(released >= extent + allocated);
    assert!(turns > 1_000);
    assert!(source.terminal_is_empty() && source.contiguous_capacity() == 0);
    assert_eq!(source.reader().unwrap().byte_len(), 0);
    println!("[DEBUG] Compared {compared} exact original paged bytes and retired in {turns} paid turns; {released} released, {allocated} retirement capacity.");
}
