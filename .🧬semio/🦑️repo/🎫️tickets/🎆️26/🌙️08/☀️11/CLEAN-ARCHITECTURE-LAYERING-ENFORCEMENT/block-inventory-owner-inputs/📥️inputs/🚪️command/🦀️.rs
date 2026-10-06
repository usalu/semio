use super::*;
use std::{cell::Cell, time::{Duration, Instant}};
/// 🚪️ Admits exact owner commands and explicit limits before selecting a contribution.
pub fn inventory_for_command<F: FnMut(MutationInventoryProgress) -> Result<(), MutationInventoryError>>(produced_by: &str, contributions: &[MutationInventoryContribution<'_>], args: &[String], caller: &mut F) -> Result<Vec<u8>, MutationInventoryError> {
    if !(10..=11).contains(&args.len()) || args[0] != "list-mutations" { return Err(MutationInventoryError::Policy); }
    let end = args.len() - 6;
    if args[end] != "--maximum-units" || args[end + 2] != "--maximum-owned-bytes" || args[end + 4] != "--budget-ms" { return Err(MutationInventoryError::Policy); }
    let units = args[end + 1].parse::<u64>().map_err(|_| MutationInventoryError::Policy)?;
    let bytes = args[end + 3].parse::<usize>().map_err(|_| MutationInventoryError::Policy)?;
    let budget_ms = args[end + 5].parse::<u64>().map_err(|_| MutationInventoryError::Policy)?;
    if units == 0 || bytes == 0 || budget_ms == 0 || units > 9_007_199_254_740_991 || bytes as u128 > 9_007_199_254_740_991 || budget_ms > 9_007_199_254_740_991 { return Err(MutationInventoryError::Policy); }
    let deadline = Instant::now().checked_add(Duration::from_millis(budget_ms)).ok_or(MutationInventoryError::Policy)?;
    let surface = if end == 5 { Some(args[4].as_str()) } else { None };
    let base_units = Cell::new(0);
    let mut observe = |progress: MutationInventoryProgress| {
        if Instant::now() >= deadline { return Err(MutationInventoryError::Cancelled); }
        let completed = progress.completed_units + base_units.get();
        caller(MutationInventoryProgress { completed_units: completed, ..progress })?;
        std::thread::yield_now();
        Ok(())
    };
    let mut selection = Encoder { output: Vec::new(), budget: MutationInventoryBudget { maximum_units: units, maximum_owned_bytes: bytes }, units: 0, observe: &mut observe };
    selection.checkpoint()?;
    let mut selected = None;
    for contribution in contributions {
        selection.charge()?;
        let coordinate = &contribution.coordinate;
        let same_surface = match (coordinate.surface, surface) { (None, None) => true, (Some(left), Some(right)) => selection.equal(left, right)?, _ => false };
        if selection.equal(coordinate.artifact, &args[1])? && selection.equal(coordinate.standard, &args[2])? && selection.equal(coordinate.subset, &args[3])? && same_surface {
            if selected.is_some() { return Err(MutationInventoryError::Coordinate); }
            selected = Some(contribution);
        }
    }
    let selected = selected.ok_or(MutationInventoryError::Coordinate)?;
    let selection_units = selection.units;
    drop(selection);
    base_units.set(selection_units);
    let output = encode_mutation_inventory(&selected.coordinate, produced_by, selected.aggregates, selected.excluded_owner_segments, MutationInventoryBudget { maximum_units: units - selection_units, maximum_owned_bytes: bytes }, &mut observe)?;
    Ok(output)
}
