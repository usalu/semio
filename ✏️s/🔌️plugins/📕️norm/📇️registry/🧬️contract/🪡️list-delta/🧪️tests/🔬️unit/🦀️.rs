//! 🧪️ Randomized sequence laws for the keyed list delta algebra: for every random edit sequence over a nested list,
//! `absorb` of the step deltas equals sequential application, the summed delta's `inverse` restores the base, every step's
//! `inverse` restores its pre-state, and `between` reproduces the final list.

use crate::list_delta::{Keyed, Parts, RowPatch};
use crate::{norm_list_delta, norm_row_patch};
use protocol::MutationApplyError;

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
struct Part {
    id: String,
    weight: i64,
}

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
struct Item {
    id: String,
    value: i64,
    label: String,
    parts: Vec<Part>,
}

norm_row_patch! { PartPatch of Part { set { weight: i64 } } }
norm_list_delta! { PartDelta { addition: PartAddition, modification: PartModification, row: Part, patch: PartPatch, key: id } }
norm_row_patch! { ItemPatch of Item { set { value: i64, label: String } nest { parts: PartDelta } } }
norm_list_delta! { ItemDelta { addition: ItemAddition, modification: ItemModification, row: Item, patch: ItemPatch, key: id } }

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound as u64) as usize
    }
}

fn item(id: &str, value: i64) -> Item {
    Item { id: id.into(), value, label: format!("label-{id}"), parts: Vec::new() }
}

fn random_step(rng: &mut Rng, state: &[Item], fresh: &mut usize, graveyard: &mut Vec<String>) -> Option<ItemDelta> {
    match rng.below(7) {
        0 | 1 => {
            let index = rng.below(state.len() + 1);
            let id = if !graveyard.is_empty() && rng.below(3) == 0 { graveyard.swap_remove(rng.below(graveyard.len())) } else { *fresh += 1; format!("k{fresh}") };
            if state.iter().any(|row| row.id == id) {
                return None;
            }
            Some(ItemDelta::insertion(state, index, item(&id, rng.below(50) as i64)))
        }
        2 | 3 => {
            let row = state.get(rng.below(state.len().max(1)))?;
            graveyard.push(row.id.clone());
            Some(ItemDelta::removal(&row.id))
        }
        4 => {
            let row = state.get(rng.below(state.len().max(1)))?;
            Some(ItemDelta::modification(&row.id, ItemPatch { value: Some(rng.below(50) as i64), ..ItemPatch::default() }))
        }
        5 => {
            let row = state.get(rng.below(state.len().max(1)))?;
            let index = rng.below(row.parts.len() + 1);
            *fresh += 1;
            let part = Part { id: format!("p{fresh}"), weight: rng.below(9) as i64 };
            Some(ItemDelta::modification(&row.id, ItemPatch { parts: PartDelta::insertion(&row.parts, index, part), ..ItemPatch::default() }))
        }
        _ => {
            let row = state.get(rng.below(state.len().max(1)))?;
            let part = row.parts.get(rng.below(row.parts.len().max(1)))?;
            if rng.below(2) == 0 {
                Some(ItemDelta::modification(&row.id, ItemPatch { parts: PartDelta::removal(&part.id), ..ItemPatch::default() }))
            } else {
                Some(ItemDelta::modification(&row.id, ItemPatch { parts: PartDelta::modification(&part.id, PartPatch { weight: Some(rng.below(9) as i64) }), ..ItemPatch::default() }))
            }
        }
    }
}

#[test]
fn random_edit_sequences_obey_the_delta_laws() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for case in 0..4000 {
        let mut fresh = 0usize;
        let mut graveyard = Vec::new();
        let base: Vec<Item> = (0..rng.below(5)).map(|index| item(&format!("b{index}"), index as i64)).collect();
        let mut state = base.clone();
        let mut absorbed = ItemDelta::default();
        let mut steps = Vec::new();
        for _ in 0..rng.below(9) + 1 {
            let Some(step) = random_step(&mut rng, &state, &mut fresh, &mut graveyard) else { continue };
            let next = step.commit_onto(&state).unwrap_or_else(|error| panic!("case {case}: step {step:?} must apply to {state:?}: {error}"));
            assert_eq!(step.inverse(&state).commit_onto(&next).as_ref(), Ok(&state), "case {case}: the step inverse restores its pre-state");
            absorbed.absorb(step.clone());
            steps.push((state.clone(), step));
            state = next;
        }
        assert_eq!(absorbed.commit_onto(&base).as_ref(), Ok(&state), "case {case}: absorb of {steps:?} equals sequential application");
        assert_eq!(absorbed.inverse(&base).commit_onto(&state).as_ref(), Ok(&base), "case {case}: the absorbed delta's inverse restores the base");
        assert_eq!(ItemDelta::between(&base, &state).commit_onto(&base).as_ref(), Ok(&state), "case {case}: between reproduces the final list");
        let mut reverse_sum = ItemDelta::default();
        for (pre, step) in steps.iter().rev() {
            reverse_sum.absorb(step.inverse(pre));
        }
        assert_eq!(reverse_sum.commit_onto(&state).as_ref(), Ok(&base), "case {case}: the reversed step inverses sum to the negative");
    }
    assert!(ItemDelta::between(&[item("a", 1)], &[item("a", 1)]).is_empty());
}

#[test]
fn absorb_coalesces_same_key_entries() {
    let base = vec![item("a", 1), item("b", 2)];
    let created = ItemDelta::insertion(&base, 1, item("n", 5));
    let after_create = created.commit_onto(&base).expect("create applies");
    let mut cancelled = created.clone();
    cancelled.absorb(ItemDelta::removal("n"));
    assert!(cancelled.is_empty(), "create∘delete is nothing: {cancelled:?}");
    let mut folded = created.clone();
    folded.absorb(ItemDelta::modification("n", ItemPatch { value: Some(9), ..ItemPatch::default() }));
    assert_eq!((folded.added.len(), folded.modified.len(), folded.added[0].row.value), (1, 0, 9), "create∘modify is one changed create");
    let mut patched = ItemDelta::modification("a", ItemPatch { value: Some(7), ..ItemPatch::default() });
    patched.absorb(ItemDelta::modification("a", ItemPatch { label: Some("x".into()), ..ItemPatch::default() }));
    assert_eq!(patched.modified.len(), 1, "patch∘patch is one patch");
    let mut replaced = ItemDelta::removal("b");
    replaced.absorb(ItemDelta::insertion(&[item("a", 1)], 1, item("b", 8)));
    assert_eq!((replaced.removed.len(), replaced.added.len()), (1, 1), "delete∘create is a replacement");
    assert_eq!(replaced.commit_onto(&base).expect("replacement applies"), vec![item("a", 1), item("b", 8)]);
    let _ = after_create;
}

#[test]
fn deltas_refuse_what_they_cannot_apply() {
    let base = vec![item("a", 1)];
    assert!(ItemDelta::removal("zz").commit_onto(&base).is_err());
    assert!(ItemDelta::modification("zz", ItemPatch::default()).commit_onto(&base).is_err());
    assert!(ItemDelta::insertion(&base, 0, item("a", 2)).commit_onto(&base).is_err());
    let dangling = ItemDelta::from_parts(Parts { added: vec![(Some("ghost".into()), item("n", 1))], ..Parts::default() });
    assert!(dangling.commit_onto(&base).is_err());
    let _: fn(&Item) -> &str = Keyed::key;
    let _: Option<MutationApplyError> = None;
    let _ = <ItemPatch as RowPatch<Item>>::is_empty(&ItemPatch::default());
}
