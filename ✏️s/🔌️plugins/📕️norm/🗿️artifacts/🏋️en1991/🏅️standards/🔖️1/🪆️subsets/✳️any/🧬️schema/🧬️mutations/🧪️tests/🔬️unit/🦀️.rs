use super::*;
use protocol::{Mutation, MutationDiff, SemanticMutation};

fn round_trip(base: &En1991Snapshot, operation: &En1991Mutation) -> En1991Snapshot {
    let forward = operation.diff(base).diff().apply(base).expect("valid mutation diff");
    let backwards = operation.inverse(base);
    let mut restored = forward.clone();
    for back in &backwards {
        restored = back.diff(&restored).diff().apply(&restored).expect("valid mutation diff");
    }
    assert_eq!(&restored, base, "inverse must exactly restore the pre-operation fixture");
    forward
}

#[semio_framework_async_macros::async_test]
async fn change_annex_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.annex, crate::document::AnnexChoice::En);
}

#[semio_framework_async_macros::async_test]
async fn change_snow_zone_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeSnowZone(change_snow_zone::ChangeSnowZone { new_snow_zone: "3".into() });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.snow_zone, "3".to_string());
}

#[semio_framework_async_macros::async_test]
async fn change_altitude_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAltitude(change_altitude::ChangeAltitude { new_altitude: 321.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.altitude, 321.0);
}

#[semio_framework_async_macros::async_test]
async fn change_assumed_bridge_tandem_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAssumedBridgeTandem(change_assumed_bridge_tandem::ChangeAssumedBridgeTandem { new_assumed_bridge_tandem: 111000.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.assumed_bridge_tandem, 111000.0);
}

#[semio_framework_async_macros::async_test]
async fn change_assumed_bridge_udl_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAssumedBridgeUdl(change_assumed_bridge_udl::ChangeAssumedBridgeUdl { new_assumed_bridge_udl: 7777.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.assumed_bridge_udl, 7777.0);
}

#[semio_framework_async_macros::async_test]
async fn change_assumed_bridge_lm2_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAssumedBridgeLm2(change_assumed_bridge_lm2::ChangeAssumedBridgeLm2 { new_assumed_bridge_lm2: 222000.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.assumed_bridge_lm2, 222000.0);
}

#[semio_framework_async_macros::async_test]
async fn change_assumed_bridge_footway_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAssumedBridgeFootway(change_assumed_bridge_footway::ChangeAssumedBridgeFootway { new_assumed_bridge_footway: 4444.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.assumed_bridge_footway, 4444.0);
}

#[semio_framework_async_macros::async_test]
async fn change_assumed_crane_wheel_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAssumedCraneWheel(change_assumed_crane_wheel::ChangeAssumedCraneWheel { new_assumed_crane_wheel: 99000.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.assumed_crane_wheel, 99000.0);
}

#[semio_framework_async_macros::async_test]
async fn change_assumed_crane_horizontal_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAssumedCraneHorizontal(change_assumed_crane_horizontal::ChangeAssumedCraneHorizontal { new_assumed_crane_horizontal: 12000.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.assumed_crane_horizontal, 12000.0);
}

#[semio_framework_async_macros::async_test]
async fn change_assumed_silo_pressure_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAssumedSiloPressure(change_assumed_silo_pressure::ChangeAssumedSiloPressure { new_assumed_silo_pressure: 33000.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.assumed_silo_pressure, 33000.0);
}

#[semio_framework_async_macros::async_test]
async fn change_assumed_silo_patch_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAssumedSiloPatch(change_assumed_silo_patch::ChangeAssumedSiloPatch { new_assumed_silo_patch: 11000.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.assumed_silo_patch, 11000.0);
}

#[semio_framework_async_macros::async_test]
async fn change_assumed_silo_wall_friction_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeAssumedSiloWallFriction(change_assumed_silo_wall_friction::ChangeAssumedSiloWallFriction { new_assumed_silo_wall_friction: 8800.0 });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.assumed_silo_wall_friction, 8800.0);
}

#[semio_framework_async_macros::async_test]
async fn change_structure_kind_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeStructureKind(change_structure_kind::ChangeStructureKind { new_structure_kind: crate::StructureKind::Bridge });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.structure_kind, crate::StructureKind::Bridge);
}

#[semio_framework_async_macros::async_test]
async fn change_crane_claimed_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeCraneClaimed(change_crane_claimed::ChangeCraneClaimed { new_crane_claimed: true });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.crane_claimed, true);
}

#[semio_framework_async_macros::async_test]
async fn change_silo_claimed_round_trips() {
    let base = En1991Snapshot::default();
    let mutation = En1991Mutation::ChangeSiloClaimed(change_silo_claimed::ChangeSiloClaimed { new_silo_claimed: true });
    let after = round_trip(&base, &mutation);
    assert_eq!(after.silo_claimed, true);
}
