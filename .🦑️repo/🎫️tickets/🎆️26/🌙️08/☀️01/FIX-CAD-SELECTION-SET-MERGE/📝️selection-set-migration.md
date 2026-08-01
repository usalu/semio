# SelectionSet migration

CadPlayRuntime.selected_object_ids: Vec<String> -> SelectionSet to match merge_world_selection_ids.

- Import SelectionSet
- Default/init via SelectionSet::default() / SelectionSet::from(vec![...])
- merge sites assign SelectionSet
- mesh_selection_ids / world3d_selection_json / gumball_target_for use as_slice()
- first() -> Option<&str> via .map(str::to_string) where owned String needed
- push -> push_unique; retain -> remove_id; contains via SelectionSet::contains
