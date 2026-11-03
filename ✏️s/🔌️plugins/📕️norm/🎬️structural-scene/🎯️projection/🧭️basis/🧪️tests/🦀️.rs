use super::*;
use crate::projection::empty_scene;
use crate::Validate;
use norm::{AnnexChoice, NormFamilyId};

fn diagram() -> Diagram2d {
    crate::Diagram2d {
        id: "basis-law".into(),
        title: LocalizedCopy::new("t", "t"),
        description: LocalizedCopy::new("d", "d"),
        axes: Vec::new(),
        series: Vec::new(),
        markers: Vec::new(),
        annotations: vec![DiagramNote { id: "note:a".into(), at: Vec2::new(1.0, 1.0), text: LocalizedCopy::new("a", "a"), tone: Tone::Neutral }],
        focus: None,
        legend: false,
        table_twin: crate::DataTable { columns: Vec::new() },
    }
}

#[test]
fn the_badge_names_the_annex_and_the_edition_in_both_languages() {
    let en = Basis::new(NormFamilyId::En1993, AnnexChoice::En).badge();
    let de = Basis::new(NormFamilyId::En1993, AnnexChoice::De).badge();
    assert_eq!(en.en, "Basis: annex EN · EN 1993-1-1:2005+A1:2014");
    assert_eq!(en.de, "Grundlage: Anhang EN · EN 1993-1-1:2005+A1:2014");
    assert!(de.en.contains("DE-NA") && de.de.contains("DE-NA") && de != en);
    assert!(!norm::looks_english(&en.de) && !norm::looks_english(&de.de));
}

#[test]
fn a_family_without_an_edition_shows_its_label() {
    assert_eq!(Basis::new(NormFamilyId::Din4108, AnnexChoice::En).standard(), "DIN 4108");
}

#[test]
fn the_scene_badge_follows_the_annex_and_never_doubles() {
    let mut scene = empty_scene();
    assert!(basis_title(&scene).is_none());
    Basis::new(NormFamilyId::En1995, AnnexChoice::En).apply(&mut scene);
    Basis::new(NormFamilyId::En1995, AnnexChoice::De).apply(&mut scene);
    assert_eq!(scene.legends.iter().filter(|l| l.id == BASIS_LEGEND_ID).count(), 1);
    assert!(basis_title(&scene).is_some_and(|t| t.en.contains("DE-NA") && t.de.contains("DE-NA")));
    assert_eq!(scene.validate(), Ok(()));
}

#[test]
fn every_diagram_header_carries_one_badge_and_the_plot_keeps_its_notes() {
    let shown = with_basis(with_basis(vec![diagram(), diagram()], &Basis::new(NormFamilyId::En1999, AnnexChoice::En)), &Basis::new(NormFamilyId::En1999, AnnexChoice::De));
    for d in &shown {
        assert_eq!(d.annotations.iter().filter(|n| n.id == BASIS_NOTE_ID).count(), 1);
        assert!(d.basis().is_some_and(|b| b.en.contains("DE-NA")));
        assert_eq!(d.plotted_notes().map(|n| n.id.as_str()).collect::<Vec<_>>(), vec!["note:a"]);
    }
}
