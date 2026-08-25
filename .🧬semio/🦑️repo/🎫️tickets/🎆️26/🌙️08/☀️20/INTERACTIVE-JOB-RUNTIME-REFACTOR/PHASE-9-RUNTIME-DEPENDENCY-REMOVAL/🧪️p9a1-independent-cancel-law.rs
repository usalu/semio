#[path = "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌉️abi/🦀️component.rs"]
mod abi;

use abi::*;

#[test]
fn cancel_after_admission_must_not_copy_the_retained_page() {
    let handle = AbiHandle::try_new(1, 1).unwrap();
    let mut writer = AbiPageWriter::new(handle);
    writer.offer(AbiPage::try_new(handle, 0, b"page".to_vec()).unwrap()).unwrap();
    writer.cancel();
    assert_eq!(writer.write_step(AbiWorkBudget::credits(4)), Err(AbiErrorCode::Cancelled));
    assert!(writer.bytes().is_empty());
}
