//! 🔪️ Folds at most two certified rectangular tools into one exact retained boundary.

use super::*;

type Bounds = [[f64; 3]; 2];

pub(super) struct CompoundCutDraft {
    stock: Bounds,
    tools: [Bounds; 2],
    count: usize,
    next: usize,
}

impl CompoundCutDraft {
    pub(super) fn new(session: &Session, input: &Dictionary) -> Result<Self, ValueError> {
        let list = tools(input)?;
        let count = list.len().saturating_sub(1);
        if !(1..=2).contains(&count) { return Err(invalid_tools()); }
        Ok(Self {
            stock: session.certified_box_bounds(geometry_handle(input, "target")?)?,
            tools: [[Default::default(); 2]; 2],
            count,
            next: 0,
        })
    }

    fn selection<'a>(&self, input: &'a Dictionary) -> Result<Option<&'a str>, ValueError> {
        if self.next == self.count { return Ok(None); }
        let tool = tools(input)?.get(["0", "1"][self.next]).ok_or_else(invalid_tools)?;
        geometry_atom_handle(Some(tool)).map(Some)
    }

    pub(super) fn demand(&self, session: &Session, input: &Dictionary) -> Result<RetirementDemand, ValueError> {
        let depth = match self.selection(input)? {
            Some(handle) => session.certified_solid_depth(handle)?.max(dictionary_depth(tools(input)?)),
            None => 1,
        }.max(dictionary_depth(input));
        Ok(RetirementDemand { copy_bytes: size_of::<Bounds>(), depth, ..Default::default() })
    }

    pub(super) fn advance(&mut self, session: &Session, input: &Dictionary, effect: &mut RetainedAllocationEffect<'_>) -> Result<(Option<SolidPublication>, Progress), ValueError> {
        let demand = self.demand(session, input)?;
        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? { return Ok((None, Progress::default())); }
        let result = match self.selection(input)? {
            Some(handle) => {
                self.tools[self.next] = session.certified_box_bounds(handle)?;
                self.next += 1;
                None
            }
            None => Some(session.plan_compound_box_cut(self.stock, &self.tools[..self.count])?),
        };
        Ok((result, Progress { copied_bytes: demand.copy_bytes, ..item() }))
    }
}

fn tools(input: &Dictionary) -> Result<&Dictionary, ValueError> {
    input.get("tools").and_then(Value::as_dictionary).filter(|list| list.schema() == Some("list")).ok_or_else(invalid_tools)
}

fn invalid_tools() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "original Compound Cut requires one or two certified boxes in a dense tool list")
}
