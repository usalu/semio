//! 🔬️ Does a wasm guest's `dlmalloc` reuse a freed block, or does linear memory ratchet?
//!
//! One allocate/free cycle repeated at a fixed size, reading `memory.size` before and after. A
//! reused block leaves the reading flat; a block the allocator cannot put back on a free list
//! grows it by the request every cycle. Rust's `wasm32-*` `std::alloc::System` is `dlmalloc-rs`
//! with the `wasm` `Allocator`, whose `free` answers `false` — so every chunk `dlmalloc` decided to
//! serve out of a direct system allocation is lost the moment it is dropped.
//! Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.

/// 📏️ Linear memory this guest owns, in bytes.
fn linear_memory_bytes() -> u32 {
    (core::arch::wasm32::memory_size(0) * 65_536) as u32
}

/// 🔁️ Growth in bytes over `cycles` allocate-then-free rounds of `size` bytes, after one warm-up
/// round so the first round's own growth is never counted.
#[unsafe(no_mangle)]
pub extern "C" fn cycle_growth(size: u32, cycles: u32) -> u32 {
    let round = |size: usize| {
        let mut block = vec![0_u8; size];
        block[0] = 1;
        block[size - 1] = 1;
        core::hint::black_box(&block);
        drop(block);
    };
    round(size as usize);
    let before = linear_memory_bytes();
    for _ in 0..cycles {
        round(size as usize);
    }
    linear_memory_bytes() - before
}

/// 🌀️ Growth over `cycles` rounds shaped like a reactor turn: one large transient (the boxed turn
/// future) allocated first, a fringe of small owners allocated on top of it and outliving it by one
/// round, then the transient freed. If the allocator cannot place the next transient in the hole the
/// last one left, linear memory ratchets by the transient every round while nothing is retained.
#[unsafe(no_mangle)]
pub extern "C" fn turn_growth(transient: u32, fringe: u32, fringe_size: u32, cycles: u32) -> u32 {
    let round = |carried: &mut Vec<Vec<u8>>| {
        let mut box_like = vec![0_u8; transient as usize];
        box_like[0] = 1;
        core::hint::black_box(&box_like);
        let mut fresh = Vec::with_capacity(fringe as usize);
        for index in 0..fringe as usize {
            let mut small = vec![0_u8; fringe_size as usize + index * 8];
            small[0] = 1;
            fresh.push(small);
        }
        drop(box_like);
        core::hint::black_box(&fresh);
        *carried = fresh;
    };
    let mut carried = Vec::new();
    for _ in 0..4 {
        round(&mut carried);
    }
    let before = linear_memory_bytes();
    for _ in 0..cycles {
        round(&mut carried);
    }
    core::hint::black_box(&carried);
    linear_memory_bytes() - before
}

/// 🪜️ Growth over `cycles` rounds that each allocate a buffer, grow it by doubling to `size`, then
/// free it — the shape a `String` assembled page by page takes, where every intermediate capacity is
/// freed on the way up.
#[unsafe(no_mangle)]
pub extern "C" fn doubling_growth(size: u32, cycles: u32) -> u32 {
    let round = |size: usize| {
        let mut block: Vec<u8> = Vec::new();
        while block.len() < size {
            let step = (size - block.len()).min(4_096);
            block.extend(std::iter::repeat_n(1_u8, step));
        }
        core::hint::black_box(&block);
        drop(block);
    };
    round(size as usize);
    let before = linear_memory_bytes();
    for _ in 0..cycles {
        round(size as usize);
    }
    linear_memory_bytes() - before
}
