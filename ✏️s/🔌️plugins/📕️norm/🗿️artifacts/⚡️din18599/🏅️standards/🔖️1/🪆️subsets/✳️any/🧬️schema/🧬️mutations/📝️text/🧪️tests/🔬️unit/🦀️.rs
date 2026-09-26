use super::*;

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_update_climate() {
    store::os_store::test_support::assert_op_line_round_trip(&Din18599Mutation::UpdateClimate(update_climate::UpdateClimate {
        new_climate: MonthlyClimate::potsdam_reference(),
    }));
}

#[semio_framework_async_macros::async_test]
async fn op_text_round_trips_change_net_floor_area() {
    store::os_store::test_support::assert_op_line_round_trip(&Din18599Mutation::ChangeNetFloorAreaM2(
        change_net_floor_area_m2::ChangeNetFloorAreaM2 { new_net_floor_area_m2: 160.0 },
    ));
}
