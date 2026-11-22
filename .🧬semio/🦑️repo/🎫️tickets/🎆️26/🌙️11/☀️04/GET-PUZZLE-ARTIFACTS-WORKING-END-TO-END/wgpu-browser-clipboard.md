# WGPU browser clipboard bridge

The actor wire record carries `clipboard-write` as a pack of the selection fragment. The WGPU friendly-effect projection now decodes that pack and returns `clipboardWrite` in the invocation result that the Rust shell consumes.

Copy and cut reserve clipboard effects for their instance before admission. Both the admitting turn and the standing operation drain deliver writes to the waiting command. If the guest starts a typed operation, the bridge waits for that invocation's exact operation ID, including when the completion races the admitting reply. Uncorrelated progress cannot replace that ID, and unrelated dispatches cannot consume the reserved write. Inline commands still return immediately. The existing operation wait watchdog bounds a lost completion.

Paste uses the existing WGPU completion publication. The shell drains its final UI dirty scope into a refresh and applies its history patch, so paste needs no special browser-side wait or publication fields.

The baseline integration run failed all three original clipboard cases and reported an unmapped `clipboard-write` effect. The patched fixture adds early completion and verifies immediate copy, late copy, late cut, exact 64-bit completion IDs, and isolation from unrelated activity. All six focused browser-program integration tests passed. All four existing turn-driver tests passed with 59 assertions.

The repository CLI could not create its capacity queue under the read-only machine cache, and the repository MCP call timed out. Test execution used the installed runtimes and the renderer's existing configuration. The unfiltered package integration run was stopped after several minutes without results. No browser run was attempted in this slice because the workspace sandbox had already refused Chromium startup and local listening.
