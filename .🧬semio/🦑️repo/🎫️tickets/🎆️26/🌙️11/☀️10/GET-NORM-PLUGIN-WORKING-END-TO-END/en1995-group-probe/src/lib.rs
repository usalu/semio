//! 🧭 Root selection and publication have one concrete typed case per owner.

use paid::{Budget, Projection, Refusal, Slot};

mod routes;

#[derive(Clone, Copy)]
enum Group {
    Deck(usize),
    Taper(usize),
}

pub struct Descriptor {
    pub tag: &'static str,
    pub number: u16,
    route: Group,
}

pub fn descriptors() -> &'static [Descriptor] {
    routes::DESCRIPTORS
}

pub enum Original<'a> {
    Direct(&'a str, u16),
    Carrier(&'a [(&'a str, &'a str)]),
}

pub struct Selected<'a, 'budget> {
    route: Group,
    projection: Projection<'a>,
    budget: &'budget Budget,
}

pub fn select<'a, 'budget>(
    original: Original<'a>,
    budget: &'budget Budget,
) -> Result<Selected<'a, 'budget>, Refusal> {
    budget.settle_selection()?;
    let (tag, projection) = match original {
        Original::Direct(tag, number) => (tag, Projection::Direct(number)),
        Original::Carrier([(tag, payload)]) => (*tag, Projection::Carrier(payload)),
        Original::Carrier(_) => return Err(Refusal::Shape),
    };
    let route = descriptors().iter().find(|row| row.tag == tag).ok_or(Refusal::UnknownTag)?.route;
    Ok(Selected { route, projection, budget })
}

pub enum Pending<'a> {
    Deck(Slot<'a, deck::Leaf>),
    Taper(Slot<'a, taper::Leaf>),
}

impl<'budget> Selected<'_, 'budget> {
    pub fn receive(self) -> Result<Pending<'budget>, Refusal> {
        match self.route {
            Group::Deck(local) => deck::receive(local, self.projection, self.budget).map(Pending::Deck),
            Group::Taper(local) => taper::receive(local, self.projection, self.budget).map(Pending::Taper),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mutation {
    Deck(deck::Leaf),
    Taper(taper::Leaf),
}

impl<'a> Pending<'a> {
    pub fn publish(self) -> Result<Slot<'a, Mutation>, Refusal> {
        match self {
            Self::Deck(slot) => slot.publish(Mutation::Deck),
            Self::Taper(slot) => slot.publish(Mutation::Taper),
        }
    }
}

impl Mutation {
    pub fn write_json(self) -> String {
        match self {
            Self::Deck(value) => value.write_json(),
            Self::Taper(value) => value.write_json(),
        }
    }

    pub fn inverse(self, previous: u16) -> Self {
        match self {
            Self::Deck(value) => Self::Deck(value.inverse(previous)),
            Self::Taper(value) => Self::Taper(value.inverse(previous)),
        }
    }
}
