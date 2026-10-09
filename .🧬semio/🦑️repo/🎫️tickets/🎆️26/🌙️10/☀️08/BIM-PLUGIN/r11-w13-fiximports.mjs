import { readFileSync, writeFileSync } from "node:fs";
const [rows, table, graph] = process.argv.slice(2);
const swap = (file, from, to) => { let t = readFileSync(file, "utf8"); if (!t.includes(from)) throw new Error(file.slice(-30) + " missing " + from.slice(0, 50)); writeFileSync(file, t.replace(from, to)); };
swap(rows, "use crate::{Phase, PropertyValue, ScheduleCategory, ScheduleField};", "use crate::{ModelInference, Phase, PropertyValue, ScheduleCategory, ScheduleColumn, ScheduleField};");
swap(table, "use crate::{ScheduleCategory, ScheduleField, ScheduleFilter, ScheduleGroup, ScheduleOp, ScheduleSort};", "use crate::{ModelInference, ScheduleCategory, ScheduleColumn, ScheduleField, ScheduleFilter, ScheduleGroup, ScheduleOp, ScheduleSort};");
swap(graph, "use crate::{Entry, ModelDiff, OpeningPatch, SchedulePatch, TopConstraint, WallPatch};", "use crate::{Entry, ModelDiff, ModelInference, OpeningPatch, SchedulePatch, TopConstraint, WallPatch};");
swap(graph, "TopConstraint::Unconnected { height: 2.0 }", "TopConstraint::Unconnected { height: 2.5 }");
swap(graph, "assert_eq!(height_of(&after.schedules[\"sch-wall\"], \"South\"), 2.0,", "assert_eq!(height_of(&after.schedules[\"sch-wall\"], \"South\"), 2.5,");
swap(graph, "    assert_eq!(after.schedules[\"sch-room\"], before.schedules[\"sch-room\"], \"the room schedule stays what it was\");", "    assert_eq!(after.schedules[\"sch-door\"], before.schedules[\"sch-door\"], \"the door schedule stays what it was\");");
