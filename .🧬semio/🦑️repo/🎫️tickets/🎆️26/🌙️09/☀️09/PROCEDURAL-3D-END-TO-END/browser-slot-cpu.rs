#[path="../../../../../../../🧰️framework/🔨️modules/⏳️async/💤️lazy-slots/🦀️.rs"]
mod lazy_slots;

use lazy_slots::LazySlots;
use std::cell::RefCell;
use std::hint::black_box;

struct Payload([u8; 16384]);

thread_local! {
    static SLOTS: RefCell<LazySlots<Payload, 1024>> = const { RefCell::new(LazySlots::new()) };
}

#[no_mangle]
pub extern "C" fn initialize() {
    SLOTS.with(|slots| { slots.borrow_mut()[1023] = Some(Payload([7; 16384])); });
}

#[no_mangle]
pub extern "C" fn select_owner(occupancy_first: bool, start: usize) -> usize {
    SLOTS.with(|table| {
        let mut slots = table.borrow_mut();
        let selected = if occupancy_first {
            slots.take_next_occupied_slot(start)
        } else {
            (0..1024).map(|offset| (start + offset) % 1024)
                .find_map(|index| slots[index].take().map(|owner| (index, owner)))
        };
        if let Some((index, owner)) = selected {
            let result = index + black_box(owner.0[0]) as usize;
            slots[index] = Some(owner);
            result
        } else {
            usize::MAX
        }
    })
}
