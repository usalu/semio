# Bearbeiten model import registration

The current-bridge wmJDEkF probe traps during startup. The generated importModelFile declaration identifies Process3dModelImportJobFactory. The previous registration used a Rust type alias, whose runtime identity is Process3dBoundedCommandJobFactory<true>. Owner, controller and document schema match. The exact factory registration check rejects the alias.

The import and bounded routes now register distinct concrete factories. They preserve their existing tool lists, payload schema, retained command implementation, execution contracts and publication lanes. The ordinary route still registers 24 tools and model import registers its one host-only tool. The Rust compiler identity probe failed on the old alias and now passes for both rows. The editor’s native route test also checks every proof name against its compiler witness. A corrected browser build is pending.
