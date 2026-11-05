#![crate_type = "lib"]
#[unsafe(no_mangle)]
pub unsafe fn initialize_large_option(target: *mut Option<[u8; 56000]>) {
    unsafe { std::ptr::addr_of_mut!(*target).write(None); }
}
#[unsafe(no_mangle)]
pub unsafe fn initialize_large_option_assigned(target: *mut Option<[u8; 56000]>) {
    unsafe { *target = None; }
}

fn absent_template<T: 'static>() -> &'static Option<T> { &None }
#[unsafe(no_mangle)]
pub unsafe fn initialize_absent_page(target: *mut Option<[u8; 56000]>, offset: usize, count: usize) {
    unsafe {
        std::ptr::copy_nonoverlapping(
            std::ptr::from_ref(absent_template::<[u8; 56000]>()).cast::<std::mem::MaybeUninit<u8>>().add(offset),
            target.cast::<std::mem::MaybeUninit<u8>>().add(offset),
            count,
        );
    }
}

fn absent_array_template<T: 'static, const N: usize>() -> &'static [Option<T>; N] { &[const { None }; N] }
#[unsafe(no_mangle)]
pub unsafe fn initialize_absent_drop_array(target: *mut [Option<Box<[u8; 512]>>; 64], offset: usize, count: usize) {
    unsafe {
        std::ptr::copy_nonoverlapping(
            std::ptr::from_ref(absent_array_template::<Box<[u8; 512]>, 64>()).cast::<std::mem::MaybeUninit<u8>>().add(offset),
            target.cast::<std::mem::MaybeUninit<u8>>().add(offset),
            count,
        );
    }
}
