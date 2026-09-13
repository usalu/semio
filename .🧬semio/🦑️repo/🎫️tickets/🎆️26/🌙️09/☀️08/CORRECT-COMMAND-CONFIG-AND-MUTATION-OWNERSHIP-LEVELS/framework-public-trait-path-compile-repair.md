# Framework Public Trait Path Compile Repair

The Architect native rerun and both root FEM reruns reported E0603 in the new dirty_background_surfaces reactor helper: crate::plugin_runtime::PluginApp names a private import. The compiler exposes the trait through crate::PluginApp. Root changed only that generic bound, then ran scoped git diff --check successfully. The surrounding concurrent dirty-surface behavior is unchanged.

This is a compile correction with no new behavior or test suite. Architect, mesh, numerical-page and FEM3d registered native reruns validate it transitively; their actual results remain recorded in their own reports.
