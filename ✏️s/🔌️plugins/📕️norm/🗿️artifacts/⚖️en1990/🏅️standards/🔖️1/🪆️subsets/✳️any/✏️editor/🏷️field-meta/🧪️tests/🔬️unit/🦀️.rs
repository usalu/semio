use super::*;
use crate::En1990Snapshot;

#[test]
fn every_editable_leaf_has_en_de_field_meta() {
    let wildcards = [
        "annex",
        "projectId",
        "structureKind",
        "altitudeM",
        "consequenceClass",
        "reliabilityClass",
        "designWorkingLifeCategory",
        "designWorkingLifeYears",
        "referencePeriodYears",
        "supervisionLevel",
        "inspectionLevel",
        "kFiDeclared",
        "betaComputed",
        "permanents",
        "permanents[].id",
        "permanents[].kind",
        "permanents[].gk",
        "variables",
        "variables[].id",
        "variables[].category",
        "variables[].qk",
        "accidentals",
        "accidentals[].id",
        "accidentals[].ad",
        "seismics",
        "seismics[].id",
        "seismics[].aEk",
        "seismics[].importanceClass",
        "members",
        "members[].id",
        "members[].labelEn",
        "members[].labelDe",
        "members[].rdStr",
        "members[].rdGeo",
        "members[].rdEquStab",
        "members[].rdEquDestab",
        "members[].rdFat",
        "members[].span",
        "members[].deflectionW",
        "members[].deflectionLimitRatio",
        "members[].vibrationFrequency",
        "members[].vibrationFrequencyMin",
        "bridgeSls",
        "bridgeSls[].id",
        "bridgeSls[].memberId",
        "bridgeSls[].deckAcceleration",
        "bridgeSls[].deckAccelerationLimit",
        "bridgeSls[].deckTwist",
        "bridgeSls[].deckTwistLimit",
        "bridgeSls[].bridgeDeflection",
        "bridgeSls[].bridgeDeflectionLimit",
        "effects",
        "effects[].memberId",
        "effects[].actionId",
        "effects[].influence",
    ];
    for path in wildcards {
        let meta = en1990_field_meta(path).unwrap_or_else(|| panic!("missing field meta for {path}"));
        assert!(!meta.label_en.is_empty(), "empty en for {path}");
        assert!(!meta.label_de.is_empty(), "empty de for {path}");
        if let Some(choices) = meta.choices {
            for c in choices {
                assert!(!c.label_en.is_empty(), "empty choice en for {path}");
                assert!(!c.label_de.is_empty(), "empty choice de for {path}");
                assert!(
                    c.label_en != c.value || c.value.len() <= 5,
                    "choice label must be human text for {path} value={}",
                    c.value
                );
            }
        }
    }
    let doc = En1990Snapshot::default();
    let concrete = [
        "projectId",
        "structureKind",
        "designWorkingLifeCategory",
        "kFiDeclared",
        "permanents[id=G-sup].gk",
        "variables[id=Q-office].category",
        "variables[id=Q-office].qk",
        "members[id=beam-B1].rdStr",
        "members[id=beam-B1].rdGeo",
        "bridgeSls[].deckAcceleration",
        "effects[].influence",
    ];
    for path in concrete {
        let meta = en1990_field_meta(path).unwrap_or_else(|| panic!("missing field meta for concrete {path}"));
        assert!(!meta.label_en.is_empty(), "empty en for {path}");
        assert!(!meta.label_de.is_empty(), "empty de for {path}");
    }
    let _ = doc;
}
