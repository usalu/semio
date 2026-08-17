# 🔓 lease-request — P3-manifest-schema → MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME's sol

**Requesting agent:** terra (P3-manifest-schema, ticket `26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY`)
**Target file (registrar-only for us; contested H1/H3 territory for you):** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs`
**Reason:** D6 (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY/📓️design-decisions.md`) removes the `ActionArgDef.control: ActionArgControl` field and replaces it with a derived method `ActionArgDef::control() -> ActionArgControl` (in `🛂️manifest/🦀️component.rs`, our exclusive path). `ActionArgControl` itself is unchanged. This wgpu shell file has 6 real `.control` field reads on `ActionArgDef` that will fail to compile once our manifest change lands. We verified every hit in the file first — the two `engagement.control`/`.control.is_some()` sites (`WindowEngagementControl`, lines 5654 and 10460 at our read-time) are a DIFFERENT `control` field entirely and are **not** part of this lease.

Apply these six textual changes verbatim (line numbers are from our read of the file at commit `1eaf87e6f5`, before any of our edits — re-locate by the surrounding function name if drift has occurred):

## 1. `commit_staged_input` — control lookup (~L6334)
```diff
-        let control = self.session.as_ref().and_then(|session| window_action_definition(&session.app, &window_id, &action_id)).and_then(|action| action.args.iter().find(|arg| arg.id == arg_id)).map(|arg| arg.control.clone());
+        let control = self.session.as_ref().and_then(|session| window_action_definition(&session.app, &window_id, &action_id)).and_then(|action| action.args.iter().find(|arg| arg.id == arg_id)).map(|arg| arg.control());
```

## 2. Command-palette Select expansion (~L6522)
```diff
-                if let semio_framework::ActionArgControl::Select { options } = &arg.control {
+                if let semio_framework::ActionArgControl::Select { options } = &arg.control() {
```

## 3. `build_command_panel_row` (~L6749)
```diff
-            if let semio_framework::ActionArgControl::Select { options } = &arg.control {
+            if let semio_framework::ActionArgControl::Select { options } = &arg.control() {
```

## 4. Test `build_os_commands_terminology_options_include_app_terminologies` (~L6999)
```diff
-        let ActionArgControl::Select { options } = &terminology_command.args[0].control else {
+        let ActionArgControl::Select { options } = &terminology_command.args[0].control() else {
```

## 5. `staged_arg_height` (~L10695)
```diff
-        match arg.control {
+        match arg.control() {
             semio_framework::ActionArgControl::Toggle => theme.control_height + theme.gap_standard,
             _ => theme.control_height * 2.0 + theme.gap_standard,
         }
```

## 6. `render_staged_arg` (~L10768)
```diff
-        match &arg.control {
+        match &arg.control() {
             ActionArgControl::Toggle => {
```
(The `Toggle`/`Select`/`IconSelect`/`Vec3` match arms directly below are untouched — `ActionArgControl` itself did not change shape.)

## Status

Not blocking our acceptance run — none of the 6 sites are in a file we can build (`Shell/🧊️component.rs` lives in the `semio-framework-os-renderer`-family crate, outside `-p semio-framework`/`-p semio-framework-os-kernel`). We verified these are the only real `ActionArgDef.control` reads in the file (full grep + manual read of every `.control`/`ActionArgControl` hit). Our own `cargo check --workspace --all-targets` acceptance step WILL fail on this file until this lease is applied; we will report that failure attributed to you per §5 of our packet, not attempt to "fix" it ourselves.
