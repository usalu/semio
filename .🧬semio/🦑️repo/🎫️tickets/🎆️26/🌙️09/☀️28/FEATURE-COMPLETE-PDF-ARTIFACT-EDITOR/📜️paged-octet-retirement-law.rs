use semio_framework_value_control_contract::{AuthoringWallet, RetainedCloneGrant, RetainedCloneProgress};
use list::{PagedList, PagedListReleaseOutcome};

const EXTENT: usize = 512 * 1024 * 1024;
const PAGE: usize = 4096;

fn main() {
    let path=std::env::args_os().nth(1).unwrap();
    let original=std::fs::read(path).unwrap();
    let extent=original.len();
    let mut octets=PagedList::<u8,EXTENT>::try_from_iter(original.into_iter()).unwrap();
    let stock=RetainedCloneGrant {
        maximum_items:1<<20, maximum_copy_bytes:1<<28, maximum_capacity_bytes:0,
        maximum_release_bytes:1<<26, maximum_depth:4096,
    };
    let mut wallet=AuthoringWallet::new(stock,1<<24);
    let denied=RetainedCloneGrant { maximum_items:0, ..stock };
    let step=wallet.run(denied,1,|effect|octets.discard_plain_tail_page(effect)).unwrap().unwrap();
    assert_eq!(step.output.result.unwrap(),RetainedCloneProgress::default());
    assert_eq!(octets.len(),extent);
    let grant=RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:PAGE+32, maximum_release_bytes:0, ..stock };
    let pages=extent.div_ceil(PAGE);
    for remaining in (0..pages).rev() {
        let before=octets.len();
        let step=wallet.run(grant,1,|effect|octets.discard_plain_tail_page(effect)).unwrap().unwrap();
        let result=step.output.result.unwrap();
        assert_eq!(step.performed,1);
        assert_eq!(result.copied_items,1);
        assert_eq!(result.copied_bytes,before-remaining*PAGE+2*std::mem::size_of::<usize>());
        assert_eq!(result.retained_capacity_bytes,0);
        assert_eq!(result.released_bytes,0);
        assert_eq!(octets.len(),remaining*PAGE);
        wallet.post(result).unwrap();
    }
    let mut released=0;
    let mut steps=0;
    while !octets.terminal_is_empty() {
        let demand=octets.next_release_demand().unwrap();
        let grant=RetainedCloneGrant { maximum_items:1, maximum_copy_bytes:demand.copy_bytes, maximum_release_bytes:demand.release_bytes, ..stock };
        let step=wallet.run(grant,1,|effect|octets.release_empty_page(effect)).unwrap().unwrap();
        let PagedListReleaseOutcome::Ready(result)=step.output.result.unwrap() else {panic!("exact page release denied")};
        assert_eq!(step.performed,1);
        assert!(result.progressed);
        released+=result.released_allocation_bytes;
        steps+=1;
        wallet.post(RetainedCloneProgress { copied_items:1, copied_bytes:result.placed_bytes,
            released_bytes:result.released_allocation_bytes, ..Default::default() }).unwrap();
        assert!(steps<=pages*3+16);
    }
    assert!(released>=extent);
    let mut typed=PagedList::<u8,16>::try_from_iter([1,2,3,4]).unwrap();
    let step=wallet.run(grant,1,|effect|typed.discard_plain_tail(effect)).unwrap().unwrap();
    assert_eq!(step.output.result.unwrap().copied_items,1);
    assert_eq!(typed.len(),3);
    println!("[DEBUG] Retired {extent} octets in {pages} paid leaf operations and {steps} release turns; {released} physical bytes.");
}
