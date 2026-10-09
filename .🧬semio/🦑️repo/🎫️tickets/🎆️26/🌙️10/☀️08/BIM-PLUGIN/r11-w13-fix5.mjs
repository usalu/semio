import { readFileSync, writeFileSync } from "node:fs";
const f = process.argv[2];
let t = readFileSync(f, "utf8");
if (!t.includes("the_office_schedules_list")) {
  const anchor = "#[semio_framework_async_macros::async_test]\nasync fn the_office_round_trips_and_validates() {";
  if (!t.includes(anchor)) throw new Error("anchor");
  t = t.replace(anchor, `#[semio_framework_async_macros::async_test]
async fn the_office_schedules_list_every_door_window_room_and_finish() {
    let model = ASSET.model();
    let inferred = crate::examples::checks::infer(&model);
    let items = |id: &str| inferred.schedules[id].items as usize;
    assert_eq!((items("sch-doors"), items("sch-windows")), (17, 24), "a door and a window schedule list every one of them");
    let finished = model.spaces.keys().filter(|id| inferred.quantities.elements.get(*id).is_some_and(|quantity| !quantity.finishes.is_empty())).count();
    assert_eq!(items("sch-finishes"), 3 * finished, "a floor, a wall and a ceiling row for every resolved room");
    assert!(finished > 0 && model.spaces.values().any(|space| space.floor_finish.is_some()), "the finishes come from the authored rooms");
}

` + anchor);
}
writeFileSync(f, t);
