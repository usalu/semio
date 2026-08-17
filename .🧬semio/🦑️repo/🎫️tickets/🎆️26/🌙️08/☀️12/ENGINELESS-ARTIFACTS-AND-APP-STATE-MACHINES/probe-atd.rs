#![feature(associated_type_defaults)]
pub struct NoThing;
pub trait Lane { fn tag() -> &'static str; }
impl Lane for NoThing { fn tag() -> &'static str { "none" } }
pub trait App { type Machine: Lane = NoThing; }
pub struct Plain;
impl App for Plain {}
fn main() { let _ = <<Plain as App>::Machine as Lane>::tag(); }
