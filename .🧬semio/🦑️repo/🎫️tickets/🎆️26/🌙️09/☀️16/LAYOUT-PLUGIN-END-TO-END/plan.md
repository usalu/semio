# Layout End-to-End Plan

Use the existing end-to-end ticket associated with Running Layout. The connected MCP starts at /; invoke the same repo MCP with the workspace working directory. Its existing hand-written metadata used invalid status done; correct that field to closed so ticket_reopen can load it.

1. Audit current launch/build/test paths and gaps.
2. Repair TypeScript behavior and test discovery.
3. Repair native layout compilation and command/export behavior.
4. Stage the current plugin, launch the React playground, and exercise persistence, undo, selection and export where supported.
5. Run meaningful regression and cross-implementation checks, retain reports, remove generated output and close ticket.

Fleet: coordinator plus Medium Rust and TypeScript execution agents and Low read-only auditor.
