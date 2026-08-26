use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};
use std::{env, fs};

#[derive(Deserialize)]
struct Fixture {
    schema: String,
    cases: Vec<Case>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Case {
    id: String,
    capacity: usize,
    maximum_bytes: usize,
    steps: Vec<Step>,
    expected: Vec<String>,
}

#[derive(Deserialize)]
#[serde(tag = "action", rename_all = "camelCase", rename_all_fields = "camelCase")]
enum Step {
    Admit { owner: String, operation: u64, generation: u64, bytes: usize },
    Take { operation: u64, generation: u64 },
    Cancel { operation: u64, generation: u64 },
    CancelStaleStep { operation: u64, live_generation: u64 },
    Close { maximum_items: usize, maximum_bytes: usize },
    Inspect,
}

struct Entry {
    owner: String,
    operation: u64,
    generation: u64,
    admitted_bytes: usize,
    remaining_bytes: usize,
    closing: bool,
}

#[derive(Serialize)]
struct Oracle {
    schema: String,
    results: Vec<ResultRow>,
}

#[derive(Serialize)]
struct ResultRow {
    id: String,
    output: Vec<String>,
}

fn index(operation: u64, generation: u64, capacity: usize) -> usize {
    ((operation ^ generation.rotate_left(17)) as usize) % capacity
}

fn main() {
    let path = env::args().nth(1).expect("fixture path");
    let fixture: Fixture = serde_json::from_str(&fs::read_to_string(path).expect("fixture read")).expect("fixture parse");
    let mut results = Vec::new();
    for case in fixture.cases {
        assert!((1..=64).contains(&case.capacity));
        let mut slots: ArrayVec<Option<Entry>, 64> = ArrayVec::new();
        for _ in 0..case.capacity {
            slots.push(None);
        }
        let mut retained_bytes = 0_usize;
        let mut occupied = 0_usize;
        let mut close_cursor = 0_usize;
        let mut output = Vec::new();
        for step in case.steps {
            match step {
                Step::Admit { owner, operation, generation, bytes } => {
                    let slot = index(operation, generation, case.capacity);
                    let next_bytes = retained_bytes.checked_add(bytes);
                    if occupied == case.capacity || next_bytes.is_none_or(|next| next > case.maximum_bytes) || slots[slot].is_some() {
                        output.push(format!("admit:rejected:{owner}"));
                    } else {
                        slots[slot] = Some(Entry { owner: owner.clone(), operation, generation, admitted_bytes: bytes, remaining_bytes: bytes, closing: false });
                        retained_bytes = next_bytes.expect("checked byte credit");
                        occupied += 1;
                        output.push(format!("admit:accepted:{owner}"));
                    }
                }
                Step::Take { operation, generation } => {
                    let slot = index(operation, generation, case.capacity);
                    if slots[slot].as_ref().is_some_and(|entry| entry.operation == operation && entry.generation == generation && !entry.closing) {
                        let entry = slots[slot].take().expect("exact oracle owner");
                        retained_bytes -= entry.admitted_bytes;
                        occupied -= 1;
                        output.push(format!("take:{}", entry.owner));
                    } else {
                        output.push("take:none".into());
                    }
                }
                Step::Cancel { operation, generation } => {
                    let slot = index(operation, generation, case.capacity);
                    let exact = slots[slot].as_mut().filter(|entry| entry.operation == operation && entry.generation == generation);
                    if let Some(entry) = exact {
                        entry.closing = true;
                        output.push("cancel:true".into());
                    } else {
                        output.push("cancel:false".into());
                    }
                }
                Step::CancelStaleStep { operation, live_generation } => {
                    let entry = slots[close_cursor].as_mut();
                    close_cursor = (close_cursor + 1) % case.capacity;
                    let stale = entry.filter(|entry| entry.operation == operation && entry.generation != live_generation);
                    if let Some(entry) = stale {
                        entry.closing = true;
                        output.push("stale:true".into());
                    } else {
                        output.push("stale:false".into());
                    }
                }
                Step::Close { maximum_items, maximum_bytes } => {
                    if maximum_items == 0 {
                        output.push("close:blocked".into());
                        continue;
                    }
                    let slot = close_cursor;
                    close_cursor = (close_cursor + 1) % case.capacity;
                    let Some(entry) = slots[slot].as_mut() else {
                        output.push(if occupied == 0 { "close:complete" } else { "close:pending" }.into());
                        continue;
                    };
                    if !entry.closing {
                        output.push("close:pending".into());
                        continue;
                    }
                    if maximum_bytes == 0 {
                        output.push("close:blocked".into());
                        continue;
                    }
                    entry.remaining_bytes = entry.remaining_bytes.checked_sub(1).unwrap_or(0);
                    if entry.remaining_bytes == 0 {
                        let entry = slots[slot].take().expect("terminal oracle owner");
                        retained_bytes -= entry.admitted_bytes;
                        occupied -= 1;
                    }
                    output.push(if occupied == 0 { "close:complete" } else { "close:pending" }.into());
                }
                Step::Inspect => {
                    let remaining = slots.iter().filter_map(Option::as_ref).map(|entry| entry.remaining_bytes).sum::<usize>();
                    output.push(format!("state:{occupied}:{retained_bytes}:{remaining}"));
                }
            }
        }
        assert_eq!(output, case.expected, "oracle case {}", case.id);
        results.push(ResultRow { id: case.id, output });
    }
    println!("{}", serde_json::to_string(&Oracle { schema: fixture.schema, results }).expect("oracle output"));
}
