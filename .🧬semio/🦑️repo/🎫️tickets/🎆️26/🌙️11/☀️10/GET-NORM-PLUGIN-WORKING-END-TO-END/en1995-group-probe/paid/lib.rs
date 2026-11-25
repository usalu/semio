//! 🎟️ Minimal model of a paid selection and a typed owner handoff.

use std::cell::Cell;

#[derive(Debug, PartialEq, Eq)]
pub enum Refusal {
    UnknownTag,
    Shape,
    Work,
    Capacity,
    Payload,
}

#[derive(Default)]
pub struct Budget {
    work: Cell<u16>,
    held: Cell<usize>,
    peak: Cell<usize>,
    owners: Cell<usize>,
    selections: Cell<u16>,
    allowance: usize,
}

impl Budget {
    pub fn new(work: u16, allowance: usize) -> Self {
        Self { work: Cell::new(work), allowance, ..Self::default() }
    }

    pub fn spend(&self) -> Result<(), Refusal> {
        self.work.set(self.work.get().checked_sub(1).ok_or(Refusal::Work)?);
        Ok(())
    }

    pub fn settle_selection(&self) -> Result<(), Refusal> {
        self.spend()?;
        self.selections.set(self.selections.get() + 1);
        Ok(())
    }

    pub fn observed(&self) -> (u16, usize, usize, usize, u16) {
        (self.work.get(), self.held.get(), self.peak.get(), self.owners.get(), self.selections.get())
    }

    fn reserve(&self, bytes: usize) -> Result<Lease<'_>, Refusal> {
        let held = self.held.get().checked_add(bytes).ok_or(Refusal::Capacity)?;
        if held > self.allowance {
            return Err(Refusal::Capacity);
        }
        self.held.set(held);
        self.peak.set(held.max(self.peak.get()));
        self.owners.set(self.owners.get() + 1);
        Ok(Lease { budget: self, bytes })
    }
}

struct Lease<'a> {
    budget: &'a Budget,
    bytes: usize,
}

impl Drop for Lease<'_> {
    fn drop(&mut self) {
        self.budget.held.set(self.budget.held.get() - self.bytes);
        self.budget.owners.set(self.budget.owners.get() - 1);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Projection<'a> {
    Direct(u16),
    Carrier(&'a str),
}

impl Projection<'_> {
    pub fn number(self) -> Result<u16, Refusal> {
        match self {
            Self::Direct(number) => Ok(number),
            Self::Carrier(number) => number.parse().map_err(|_| Refusal::Payload),
        }
    }
}

pub struct Slot<'a, T> {
    value: T,
    lease: Lease<'a>,
}

impl<'a, T> Slot<'a, T> {
    pub fn receive(
        budget: &'a Budget,
        projection: Projection<'_>,
        construct: fn(u16) -> T,
    ) -> Result<Self, Refusal> {
        budget.spend()?;
        let lease = budget.reserve(std::mem::size_of::<T>())?;
        let value = construct(projection.number()?);
        Ok(Self { value, lease })
    }

    pub fn publish<U>(self, wrap: impl FnOnce(T) -> U) -> Result<Slot<'a, U>, Refusal> {
        self.lease.budget.spend()?;
        let lease = self.lease.budget.reserve(std::mem::size_of::<U>())?;
        Ok(Slot { value: wrap(self.value), lease })
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}
