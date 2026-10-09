# ⚖️ Coordinator Decisions for r9 (Completeness Waves)

Answers to `r9-audit-completeness.md` §6. Binding for every wave-W agent.

1. **Geometric constraints check, they never drive.** Authored geometry stays the only source of truth. A constraint
   (e.g. dimension lock, alignment, parallel) is an authored record that inference *checks* (`⚠️diagnostics`/`⚠️rules`).
   Tools that edit a constrained element compute payloads that satisfy the constraint (editor-side, pure), and emit
   ordinary concrete mutations for every affected element. Nothing derived is ever written back. Parametric propagation
   remains reference-based (top constraints, hosting, attach), which inference already resolves.
2. **Curtain panel overrides are a keyed collection** `curtain_panel_overrides: BTreeMap<Id, Override { curtain, u, v,
   panel }>` with (curtain, u, v) unique (refuse duplicates). Each override is independently addressable, so it must not be
   a whole-list field.
3. **Sheets use the existing stdio dialects**: SVG (`s.stdio.svg@1.1/*`) and PDF (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf`).
   No new framework PDF writer.
4. **Parametric families**: run a spike (WP-23) that ends in a decision record. Until it is accepted, types stay typed records.
   A type may extend them with typed parameter tables and profile references.
5. **BCF and zip**: use the existing stdio `💬️bcf` and `🎒️zip` artifacts and the framework `🗜️deflate` module. No external
   library and no new primitive.

Additional rulings:
- **CSV export** (diagnostics, schedules) goes through `s.stdio.csv`.
- **Verbs**: only approved mutation verbs. If a needed verb is missing from the framework `APPROVED_VERBS`, use the closest
  approved one and keep the kind name semantic. Do not edit the framework verb list.
- **Ids**: every new collection joins the cross-kind id uniqueness predicate (`taken`). Every element delete cascades
  properties and classifications (r6 rules).
