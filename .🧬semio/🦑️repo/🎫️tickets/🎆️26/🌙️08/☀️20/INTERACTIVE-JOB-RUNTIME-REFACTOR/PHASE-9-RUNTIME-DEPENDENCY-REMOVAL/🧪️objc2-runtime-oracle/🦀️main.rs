#[cfg(target_os = "macos")]
fn main() {
    use objc2::rc::{Retained, autoreleasepool};
    use objc2::runtime::{NSObject, NSObjectProtocol};
    use std::mem::{align_of, size_of};

    autoreleasepool(|_| {
        let object = NSObject::new();
        let before = object.retainCount();
        let clone = object.clone();
        assert_eq!(clone.retainCount(), before + 1);
        drop(clone);
        assert_eq!(object.retainCount(), before);
        assert!(unsafe { Retained::<NSObject>::from_raw(std::ptr::null_mut()) }.is_none());
        print!(
            "{{\n  \"$schema\": \"../🧬️schema/🔣️objc2-runtime-abi.schema.json\",\n  \"schemaVersion\": 1,\n  \"contract\": \"owned-objective-c-runtime\",\n  \"oracle\": {{ \"package\": \"objc2\", \"version\": \"0.6.4\" }},\n  \"layout\": {{ \"ownedBytes\": {}, \"ownedAlign\": {}, \"optionalOwnedBytes\": {} }},\n  \"ownership\": {{ \"cloneRetainDelta\": 1, \"dropRestores\": true, \"nullOwnedAccepted\": false, \"autoreleasePoolDrained\": true }},\n  \"boundaries\": {{ \"empty\": \"accepted\", \"single\": \"accepted\", \"maximum\": 4096, \"maximumPlusOne\": \"rejected\", \"hostileNull\": \"rejected\" }}\n}}\n",
            size_of::<Retained<NSObject>>(),
            align_of::<Retained<NSObject>>(),
            size_of::<Option<Retained<NSObject>>>()
        );
    });
}

#[cfg(not(target_os = "macos"))]
fn main() {
    panic!("the objc2 runtime oracle runs only on macOS");
}
