SPEC = {
 "orig": "5.rs.txt",
 "file": "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs",
 "nest": False,
 "children": [
  {
   "folder": "🏠️model-and-window-labels",
   "mod": "model_and_window_labels",
   "ranges": [
    [
     17,
     304
    ]
   ],
   "doc": ""
  },
  {
   "folder": "🎮️command-and-argument-labels",
   "mod": "command_and_argument_labels",
   "ranges": [
    [
     306,
     533
    ]
   ],
   "doc": ""
  },
  {
   "folder": "⚠️fault-and-utility-labels",
   "mod": "fault_and_utility_labels",
   "ranges": [
    [
     535,
     828
    ]
   ],
   "doc": ""
  },
  {
   "folder": "🔍️review-and-schedule-labels",
   "mod": "review_and_schedule_labels",
   "ranges": [
    [
     830,
     1276
    ]
   ],
   "doc": ""
  }
 ],
 "external": [],
 "expect_missing": [
  "bim_labels! {"
 ],
 "expect_extra": [
  "}",
  "}",
  "}",
  "}",
  "}",
  "}",
  "}",
  "}",
  "};",
  "};",
  "};",
  "};",
  "};",
  "macro_rules! bim_labels_emit {",
  "(; $($rows:tt)*) => {",
  "bim_labels! { $($rows)* }",
  "#[macro_use]",
  "#[macro_use]",
  "#[macro_use]",
  "#[macro_use]",
  "#[path = \"🏠️model-and-window-labels/🦀️.rs\"]",
  "mod model_and_window_labels;",
  "#[path = \"🎮️command-and-argument-labels/🦀️.rs\"]",
  "mod command_and_argument_labels;",
  "#[path = \"⚠️fault-and-utility-labels/🦀️.rs\"]",
  "mod fault_and_utility_labels;",
  "#[path = \"🔍️review-and-schedule-labels/🦀️.rs\"]",
  "mod review_and_schedule_labels;",
  "bim_label_rows_model! { bim_label_rows_commands, bim_label_rows_faults, bim_label_rows_review, bim_labels_emit; }",
  "//! 🏠️ Model and window labels of the BIM terminology: the utilities, windows, panels, entity kinds, groups, fields, actions, measures and status rows. See [`super`].",
  "macro_rules! bim_label_rows_model {",
  "($next:ident $(, $more:ident)*; $($rows:tt)*) => {",
  "($next:ident $(, $more:ident)*; $($rows:tt)*) => {",
  "($next:ident $(, $more:ident)*; $($rows:tt)*) => {",
  "($next:ident $(, $more:ident)*; $($rows:tt)*) => {",
  "$next! { $($more),*; $($rows)*",
  "$next! { $($more),*; $($rows)*",
  "$next! { $($more),*; $($rows)*",
  "$next! { $($more),*; $($rows)*",
  "//! 🎮️ Command and argument labels of the BIM terminology: the outliner, status and progress rows and the command, argument and measure rows. See [`super`].",
  "macro_rules! bim_label_rows_commands {",
  "//! ⚠️ Fault and utility labels of the BIM terminology: the fault rows, the utility rows, the sweep kinds and the schedule filter rows. See [`super`].",
  "macro_rules! bim_label_rows_faults {",
  "//! 🔍️ Review and schedule labels of the BIM terminology: the schedule, family, diagnostic, clash, rule, issue and envelope rows. See [`super`].",
  "macro_rules! bim_label_rows_review {"
 ]
}
