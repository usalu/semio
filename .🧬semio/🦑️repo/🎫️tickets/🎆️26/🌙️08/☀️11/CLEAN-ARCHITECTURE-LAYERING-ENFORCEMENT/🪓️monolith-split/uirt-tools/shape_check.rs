#![allow(dead_code)]
use std::mem::take;
#[derive(Default, Debug)] struct Id(u8);
#[derive(Default, Debug)] struct Shadow { surface: Id, document: Option<Box<u8>>, retire_scalar: u8 }
impl Shadow {
    pub fn retire_one<H>(&mut self, handback: &mut Option<H>, release: impl FnOnce(H)) -> bool {
        match self.retire_scalar {
            0 => self.surface = Id::default(),
            5 => { if let Some(handback) = handback.take() { release(handback); } }
            _ => return true,
        }
        self.retire_scalar += 1;
        self.retire_scalar >= 6
    }
    pub fn retire_one_detached(&mut self) -> bool { self.retire_one(&mut None::<()>, drop) }
    pub fn take_all(&mut self) -> Self { Self { surface: take(&mut self.surface), document: self.document.take(), retire_scalar: self.retire_scalar } }
    pub fn terminal_is_empty(&self) -> bool { self.document.is_none() }
}
#[derive(Debug)] struct Reservation { key: u64 }
fn release_surface_reconcile_handback(_r: Reservation) {}
#[derive(Debug)] struct Owner { shadow: Shadow, handback: Option<Reservation>, armed: bool }
impl Owner {
    fn from_shadow(shadow: Shadow) -> Self { Self { shadow, handback: None, armed: true } }
    fn retire_one(&mut self) -> bool { self.shadow.retire_one(&mut self.handback, release_surface_reconcile_handback) }
}
impl Drop for Owner {
    fn drop(&mut self) {
        if !self.armed { return; }
        self.armed = false;
        if self.shadow.terminal_is_empty() { if let Some(h) = self.handback.take() { release_surface_reconcile_handback(h); } return; }
        let generation = self.handback.as_ref().map_or(0, |r| r.key);
        let owner = Owner { shadow: self.shadow.take_all(), handback: None, armed: false };
        let _ = (generation, owner, self.handback.take());
    }
}
#[derive(Clone, Copy, Debug, Default)] struct Usage { items: usize }
struct Cursor { usage: Usage }
impl Cursor { pub fn usage(&self) -> Usage { self.usage } }
fn transaction_usage(c: Option<&Cursor>, fallback: Usage) -> Usage { c.map_or(fallback, Cursor::usage) }
fn main() {}
