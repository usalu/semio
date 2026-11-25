//! 🔁️ Apply and inverse of the EN 1994 diff: the central applier hook and the algebra that returns the negative diff. See [`super`].

use super::*;

impl protocol::MutationDiff<En1994Snapshot> for En1994Diff {}

impl protocol::ColdMutationDiff<En1994Snapshot> for En1994Diff {
    fn absorb(&mut self, other: Self) {
        if other.annex.is_some() {
            self.annex = other.annex;
        }
        if other.reliability_class.is_some() {
            self.reliability_class = other.reliability_class;
        }
        if other.structure_kind.is_some() {
            self.structure_kind = other.structure_kind;
        }
        if other.steel_f_y_pa.is_some() {
            self.steel_f_y_pa = other.steel_f_y_pa;
        }
        if let Some(theirs) = other.beams {
            match self.beams.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.beams = Some(theirs),
            }
            self.beams = self.beams.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.columns {
            match self.columns.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.columns = Some(theirs),
            }
            self.columns = self.columns.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.slabs {
            match self.slabs.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.slabs = Some(theirs),
            }
            self.slabs = self.slabs.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.placements {
            match self.placements.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.placements = Some(theirs),
            }
            self.placements = self.placements.take().filter(|rows| !rows.is_empty());
        }
        if let Some(theirs) = other.studs {
            match self.studs.as_mut() {
                Some(mine) => mine.absorb(theirs),
                None => self.studs = Some(theirs),
            }
            self.studs = self.studs.take().filter(|rows| !rows.is_empty());
        }
        if other.fire_rating.is_some() {
            self.fire_rating = other.fire_rating;
        }
        if other.insulation_thickness_m.is_some() {
            self.insulation_thickness_m = other.insulation_thickness_m;
        }
        if other.fatigue_detail.is_some() {
            self.fatigue_detail = other.fatigue_detail;
        }
        self.cases.absorb(other.cases);
    }

    fn apply(&self, unpublished: &mut En1994Snapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let base = &*unpublished;
        let mut next = base.clone();
        if let Some(value) = &self.annex {
            next.annex = value.clone();
        }
        if let Some(value) = &self.reliability_class {
            next.reliability_class = value.clone();
        }
        if let Some(value) = &self.structure_kind {
            next.structure_kind = value.clone();
        }
        if let Some(value) = &self.steel_f_y_pa {
            next.steel_f_y_pa = value.clone();
        }
        if let Some(rows) = &self.beams {
            next.beams = rows.commit_onto(&base.beams, capability).map_err(|error| error.under(["beams"]))?;
        }
        if let Some(rows) = &self.columns {
            next.columns = rows.commit_onto(&base.columns, capability).map_err(|error| error.under(["columns"]))?;
        }
        if let Some(rows) = &self.slabs {
            next.slabs = rows.commit_onto(&base.slabs, capability).map_err(|error| error.under(["slabs"]))?;
        }
        if let Some(rows) = &self.placements {
            next.placements = rows.commit_onto(&base.placements, capability).map_err(|error| error.under(["placements"]))?;
        }
        if let Some(rows) = &self.studs {
            next.studs = rows.commit_onto(&base.studs, capability).map_err(|error| error.under(["studs"]))?;
        }
        if let Some(value) = &self.fire_rating {
            next.fire_rating = value.clone();
        }
        if let Some(value) = &self.insulation_thickness_m {
            next.insulation_thickness_m = value.clone();
        }
        if let Some(value) = &self.fatigue_detail {
            next.fatigue_detail = value.clone();
        }
        next.cases = self.cases.commit_onto(&base.cases, capability).map_err(|error| error.under(["cases"]))?;
        *unpublished = next;
        Ok(())
    }
}

impl protocol::DiffAlgebra<En1994Snapshot> for En1994Diff {
    fn is_empty(&self) -> bool {
        self.annex.is_none() && self.reliability_class.is_none() && self.structure_kind.is_none() && self.steel_f_y_pa.is_none() && self.beams.as_ref().map_or(true, |rows| rows.is_empty()) && self.columns.as_ref().map_or(true, |rows| rows.is_empty()) && self.slabs.as_ref().map_or(true, |rows| rows.is_empty()) && self.placements.as_ref().map_or(true, |rows| rows.is_empty())  && self.studs.as_ref().map_or(true, |rows| rows.is_empty()) && self.fire_rating.is_none() && self.insulation_thickness_m.is_none() && self.fatigue_detail.is_none() && self.cases.is_empty()
    }
}

impl protocol::ColdDiffAlgebra<En1994Snapshot> for En1994Diff {
    fn inverse(&self, base: &En1994Snapshot) -> Self {
        Self {
            annex: self.annex.as_ref().map(|_| base.annex.clone()),
            reliability_class: self.reliability_class.as_ref().map(|_| base.reliability_class.clone()),
            structure_kind: self.structure_kind.as_ref().map(|_| base.structure_kind.clone()),
            steel_f_y_pa: self.steel_f_y_pa.as_ref().map(|_| base.steel_f_y_pa.clone()),
            beams: self.beams.as_ref().map(|rows| rows.inverse(&base.beams)).filter(|rows| !rows.is_empty()),
            columns: self.columns.as_ref().map(|rows| rows.inverse(&base.columns)).filter(|rows| !rows.is_empty()),
            slabs: self.slabs.as_ref().map(|rows| rows.inverse(&base.slabs)).filter(|rows| !rows.is_empty()),
            placements: self.placements.as_ref().map(|rows| rows.inverse(&base.placements)).filter(|rows| !rows.is_empty()),
            studs: self.studs.as_ref().map(|rows| rows.inverse(&base.studs)).filter(|rows| !rows.is_empty()),
            fire_rating: self.fire_rating.as_ref().map(|_| base.fire_rating.clone()),
            insulation_thickness_m: self.insulation_thickness_m.as_ref().map(|_| base.insulation_thickness_m.clone()),
            fatigue_detail: self.fatigue_detail.as_ref().map(|_| base.fatigue_detail.clone()),
            cases: self.cases.inverse(&base.cases),
        }
    }
}
