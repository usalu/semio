# Framework root shape correction

User directive: `🧰️framework` may only have `📦️packages`, `🔨️modules`, `🛍️products`.
`📦️packages` is glue only (bundle entry) — no domain implementation at framework root.

Plan:
1. Create `🔨️modules/🧩core/`
2. Move `🎯️action-bus`, `🔺️mesh`, `🖥️platform`, `🧩️ui`, `🤖️generated` there
3. Retarget `📦️packages/🦀️rust/📦️lib.rs` #[path]s
4. Verify framework root has only the three folders
