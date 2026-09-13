//! 🧪️ Executes the actual Puzzle metabolism lookup against portable caller queries.

#[cfg(all(taxonomy_before, not(taxonomy_parent)))]
#[path = "../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔣️icons/🧩️metabolism.rs"]
mod catalog;
#[cfg(all(not(taxonomy_before), not(taxonomy_parent)))]
#[path = "../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔣️icons/🌱️metabolism/🦀️.rs"]
mod catalog;

#[cfg(taxonomy_parent)]
#[path = "../../../../../../../../✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔣️icons/🦀️.rs"]
mod catalog;

#[cfg(taxonomy_parent)]
use catalog::puzzle_themed_icon_lookup as lookup;
#[cfg(not(taxonomy_parent))]
use catalog::board_metabolism_icon_svg as lookup;

fn main() {
    for query in std::env::args().skip(1) {
        match lookup(&query) {
            None => println!("-"),
            Some(svg) => {
                let value: String = svg.as_bytes().iter().map(|byte| format!("{byte:02x}")).collect();
                println!("{value}");
            }
        }
    }
}
