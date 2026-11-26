//! ⏱️ Compare sparse probing with the reactor's occupancy-word cursor on a single live actor.
use semio_slots::{ReactorFixedSlots, REACTOR_TASK_SLOTS};
use std::{hint::black_box, time::Instant};
fn main() {
    let mut slots=ReactorFixedSlots::new();
    slots.insert(1,1usize).unwrap();
    for bitmap in [false,true] {
        let begin=Instant::now();
        for turn in 0..25_000 {
            let start=black_box(turn%REACTOR_TASK_SLOTS);
            let missing=black_box(42);
            let result=if bitmap {
                slots.iter_from(start).find(|(_,value)| **value==missing).map(|(i,_)|i)
            } else {
                (0..REACTOR_TASK_SLOTS).map(|d|(start+d)%REACTOR_TASK_SLOTS).find(|i|slots.get(*i)==Some(&missing))
            };
            assert!(black_box(result).is_none());
        }
        println!("bitmap={bitmap} elapsed_ms={}",begin.elapsed().as_millis());
    }
}
