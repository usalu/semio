# Validation Directory Loss

During native560, the complete ticket-generated directory disappeared externally. At approximately 21:17 local time, repeated filesystem checks confirmed that the ticket itself remained but its generated directory was absent. Worker PID 75515 and its Cargo child 75521 were still running after more than 51 minutes. No command in this task removed that directory.

Sent SIGTERM only to this ticket's Cargo child, PID 75521. The run cannot provide a retained final diagnostic receipt after loss of its output and cache. All observations from native560 were partial diagnostics, and no clean compilation was claimed. The retained reports and input runner remain in the ticket.

Pass574/575 was still starting under Nx when the directory loss occurred; its completion and outputs must be inspected. Native/WASI/browser compilation and exact runtime verification remain incomplete. Recreate the generated directory before restarting validation, and retain all new output under that directory.
